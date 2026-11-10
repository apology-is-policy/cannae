//! The vision of record's substrate. Every man is a soft disc that pushes his neighbours apart,
//! every base tick (20 Hz).
//!
//! Contacts are resolved by position, Jacobi style: each iteration sums integer corrections from
//! every overlapping pair and applies them together, so the order in which pairs are found
//! cannot change a bit. A man leaning into the press (`Target::press`) weighs more in each
//! contact and edges forward, so pressure passes from rank to rank through the bodies
//! themselves; that is what this substrate offers that a field does not.
//!
//! Limitations, for now: the fallen are not obstacles, and there are no walls. The passage
//! study (S0.8) adds walls.

use cannae_core::{
    Bodies, Cause, Event, EventKind, Key, Posture, Prob, Rng, SimMs, Stream, Substrate, Target,
    Vec2, Wound,
};

/// Every value here is a placeholder until the priors are fixed in S0.3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Params {
    /// A man with shield, side on: about half a metre across.
    pub radius_mm: i32,
    /// Relaxation passes per base tick.
    pub iterations: u32,
    /// The share of an overlap removed per pass, out of 256.
    pub stiffness_q8: i32,
    /// The most a man turns in one base tick, in turn units.
    pub turn_per_tick: u16,
    /// How fast a man at full press edges forward, mm/s.
    pub press_mm_s: i32,
    /// How fast a standing man regains his spot after a shove, mm/s. Standing still is not
    /// the same as offering no resistance.
    pub brace_mm_s: i32,
    /// Summed overlap, in mm, above which a man is being crushed.
    pub crush_mm: i32,
    /// Injury hazard per second for each mm of overlap above `crush_mm`, Q16.
    pub crush_rate_q16_per_mm: u32,
}

impl Params {
    pub const PLACEHOLDER: Params = Params {
        radius_mm: 280,
        iterations: 2,
        stiffness_q8: 128,
        turn_per_tick: 1_638,
        press_mm_s: 300,
        brace_mm_s: 600,
        crush_mm: 250,
        // 0.01 per second per mm.
        crush_rate_q16_per_mm: 655,
    };
}

/// The purpose word for a crush injury draw on the substrate stream.
pub const PURPOSE_CRUSH: u32 = 1;

pub struct Pairwise {
    pub p: Params,
    /// Threads for the contact passes. Results are identical for any count: corrections are
    /// integers summed per thread and then across threads, and integer addition does not care
    /// about order.
    pub threads: usize,
    grid: Grid,
    corr: Vec<Vec2>,
    pressure: Vec<i32>,
    weight: Vec<i32>,
    old: Vec<Vec2>,
    bufs: Vec<(Vec<Vec2>, Vec<i32>)>,
}

/// Men sorted by cell into a dense row-major grid over their bounding box, with their positions
/// and weights copied into that order, so a cell's neighbours are contiguous in memory. Cells
/// are at least one diameter wide, and wider when men are spread thin, which keeps the grid's
/// size proportional to their number.
#[derive(Default)]
struct Grid {
    size: i32,
    x0: i32,
    y0: i32,
    w: i32,
    h: i32,
    start: Vec<u32>,
    /// Man ids in cell order (ties by id).
    id: Vec<u32>,
    px: Vec<i32>,
    py: Vec<i32>,
    wt: Vec<i32>,
    slot: Vec<u32>,
}

impl Grid {
    fn build(
        &mut self,
        diameter: i32,
        pos: &[Vec2],
        weight: &[i32],
        active: impl Fn(usize) -> bool,
    ) {
        let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
        let mut m = 0usize;
        for (i, p) in pos.iter().enumerate() {
            if active(i) {
                x0 = x0.min(p.x);
                y0 = y0.min(p.y);
                x1 = x1.max(p.x);
                y1 = y1.max(p.y);
                m += 1;
            }
        }
        if m == 0 {
            (x0, y0, x1, y1) = (0, 0, 0, 0);
        }
        let (spanx, spany) = ((x1 - x0) as i64 + 1, (y1 - y0) as i64 + 1);
        let mut size = diameter as i64;
        // At most about eight cells per man.
        let cap = 8 * m as i64 + 4096;
        while (spanx / size + 1) * (spany / size + 1) > cap {
            size *= 2;
        }
        self.size = size as i32;
        (self.x0, self.y0) = (x0, y0);
        self.w = (spanx / size + 1) as i32;
        self.h = (spany / size + 1) as i32;
        let cells = (self.w * self.h) as usize;
        self.slot.clear();
        self.slot.resize(pos.len(), u32::MAX);
        self.start.clear();
        self.start.resize(cells + 1, 0);
        for (i, &p) in pos.iter().enumerate() {
            if active(i) {
                let c = self.cell_index(p);
                self.slot[i] = c as u32;
                self.start[c + 1] += 1;
            }
        }
        for c in 0..cells {
            self.start[c + 1] += self.start[c];
        }
        self.id.clear();
        self.id.resize(m, 0);
        let mut fill = self.start.clone();
        for i in 0..pos.len() {
            let c = self.slot[i];
            if c != u32::MAX {
                let k = fill[c as usize] as usize;
                self.id[k] = i as u32;
                fill[c as usize] += 1;
            }
        }
        self.px.clear();
        self.py.clear();
        self.wt.clear();
        for &i in &self.id {
            let i = i as usize;
            self.px.push(pos[i].x);
            self.py.push(pos[i].y);
            self.wt.push(weight[i]);
        }
    }

    fn cell_index(&self, p: Vec2) -> usize {
        let cx = (p.x - self.x0) / self.size;
        let cy = (p.y - self.y0) / self.size;
        (cy * self.w + cx) as usize
    }

    fn range(&self, cx: i32, cy: i32) -> core::ops::Range<usize> {
        if cx < 0 || cy < 0 || cx >= self.w || cy >= self.h {
            return 0..0;
        }
        let c = (cy * self.w + cx) as usize;
        self.start[c] as usize..self.start[c + 1] as usize
    }
}

/// The contact between sorted men `a` and `b`, if any, added to `corr` and `pressure` (indexed
/// by sorted position). Symmetric in `a` and `b` except for exact coincidence, which is broken by
/// man id, so the result does not depend on which of the two finds the other.
#[inline]
fn contact(g: &Grid, p: &Params, a: usize, b: usize, corr: &mut [Vec2], pressure: &mut [i32]) {
    let reach = 2 * p.radius_mm as i64;
    let mut d = Vec2::new(g.px[b] - g.px[a], g.py[b] - g.py[a]);
    let d2 = d.len_sq();
    if d2 >= reach * reach {
        return;
    }
    let mut dist = (d2 as u64).isqrt() as i64;
    if dist == 0 {
        // Exactly coincident: separate along x, lower id to the left.
        d = Vec2::new(if g.id[a] < g.id[b] { 1 } else { -1 }, 0);
        dist = 1;
    }
    let overlap = reach - dist;
    let (wa, wb) = (g.wt[a] as i64, g.wt[b] as i64);
    let push = overlap * p.stiffness_q8 as i64;
    let ma = push * wb / (wa + wb) / 256;
    let mb = push * wa / (wa + wb) / 256;
    corr[a] -= d.mul_div(ma, dist);
    corr[b] += d.mul_div(mb, dist);
    pressure[a] += overlap as i32;
    pressure[b] += overlap as i32;
}

/// All contacts of men in grid rows `rows`: within each cell, and with the half of the
/// neighbouring cells that come later (east, and the three cells of the next row), so each pair
/// is found exactly once.
fn pairs(
    g: &Grid,
    p: &Params,
    rows: core::ops::Range<i32>,
    corr: &mut [Vec2],
    pressure: &mut [i32],
) {
    for cy in rows {
        for cx in 0..g.w {
            let here = g.range(cx, cy);
            for a in here.clone() {
                for b in a + 1..here.end {
                    contact(g, p, a, b, corr, pressure);
                }
                for (nx, ny) in [
                    (cx + 1, cy),
                    (cx - 1, cy + 1),
                    (cx, cy + 1),
                    (cx + 1, cy + 1),
                ] {
                    for b in g.range(nx, ny) {
                        contact(g, p, a, b, corr, pressure);
                    }
                }
            }
        }
    }
}

impl Pairwise {
    pub fn new(p: Params) -> Self {
        Pairwise {
            p,
            threads: 1,
            grid: Grid::default(),
            corr: Vec::new(),
            pressure: Vec::new(),
            weight: Vec::new(),
            old: Vec::new(),
            bufs: Vec::new(),
        }
    }

    /// Each man's summed overlap with his neighbours after the last tick, in mm. Private to
    /// this substrate: men read crowding from the View, never this.
    pub fn pressure(&self) -> &[i32] {
        &self.pressure
    }

    /// One relaxation pass.
    fn relax(&mut self, bodies: &mut Bodies) {
        let n = bodies.len();
        let posture = &bodies.posture;
        self.grid
            .build(2 * self.p.radius_mm, &bodies.pos, &self.weight, |i| {
                posture[i] != Posture::Down
            });
        let m = self.grid.id.len();
        let threads = self.threads.max(1);
        self.bufs.resize_with(threads, Default::default);
        let h = self.grid.h;
        let per = (h as usize).div_ceil(threads).max(1) as i32;
        let (g, p) = (&self.grid, &self.p);
        let work = |k: usize, buf: &mut (Vec<Vec2>, Vec<i32>)| {
            buf.0.clear();
            buf.0.resize(m, Vec2::ZERO);
            buf.1.clear();
            buf.1.resize(m, 0);
            let lo = (k as i32 * per).min(h);
            pairs(g, p, lo..(lo + per).min(h), &mut buf.0, &mut buf.1);
        };
        if threads == 1 {
            work(0, &mut self.bufs[0]);
        } else {
            std::thread::scope(|sc| {
                for (k, buf) in self.bufs.iter_mut().enumerate() {
                    sc.spawn(move || work(k, buf));
                }
            });
        }
        self.corr.clear();
        self.corr.resize(n, Vec2::ZERO);
        self.pressure.clear();
        self.pressure.resize(n, 0);
        for buf in &self.bufs {
            for (k, &i) in self.grid.id.iter().enumerate() {
                self.corr[i as usize] += buf.0[k];
                self.pressure[i as usize] += buf.1[k];
            }
        }
        for i in 0..n {
            bodies.pos[i] += self.corr[i];
        }
    }
}

impl Substrate for Pairwise {
    fn advance(
        &mut self,
        bodies: &mut Bodies,
        targets: &[Target],
        dt: SimMs,
        rng: &Rng,
        tick: u32,
        out: &mut Vec<Event>,
    ) {
        let n = bodies.len();
        self.old.clear();
        self.old.extend_from_slice(&bodies.pos);
        self.weight.clear();
        self.weight
            .extend(targets.iter().map(|t| 256 + t.press as i32));
        for (i, t) in targets.iter().enumerate() {
            if bodies.posture[i] == Posture::Down {
                continue;
            }
            let to = t.point - bodies.pos[i];
            let speed = if t.speed == 0 {
                self.p.brace_mm_s
            } else {
                t.speed
            };
            let step = (speed as i64 * dt.0 as i64 / 1000) as i32;
            let mut mv = if to.len_sq() <= step as i64 * step as i64 {
                to
            } else {
                to.with_len(step)
            };
            if t.press > 0
                && let Some(f) = t.face
            {
                let lean = self.p.press_mm_s as i64 * t.press as i64 / 255 * dt.0 as i64 / 1000;
                mv += f.unit(lean as i32);
            }
            bodies.pos[i] += mv;
        }
        for _ in 0..self.p.iterations {
            self.relax(bodies);
        }
        for (i, t) in targets.iter().enumerate().take(n) {
            if bodies.posture[i] == Posture::Down {
                bodies.vel[i] = Vec2::ZERO;
                continue;
            }
            bodies.vel[i] = (bodies.pos[i] - self.old[i]).mul_div(1000, dt.0 as i64);
            if let Some(f) = t.face {
                bodies.facing[i] = bodies.facing[i].rotate_toward(f, self.p.turn_per_tick);
            }
            let excess = self.pressure[i] - self.p.crush_mm;
            if excess > 0 {
                let rate =
                    (excess as u64 * self.p.crush_rate_q16_per_mm as u64).min(u32::MAX as u64);
                let p = Prob::from_rate(rate as u32, dt);
                if rng.chance(
                    p,
                    Key::new(Stream::Substrate, i as u32, tick, PURPOSE_CRUSH),
                ) {
                    out.push(Event {
                        tick,
                        man: cannae_core::ManId(i as u32),
                        pos: bodies.pos[i],
                        kind: EventKind::Wound {
                            to: Wound::Disabled,
                        },
                        causes: [Cause::Crush, Cause::None, Cause::None],
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cannae_core::{Observed, Turn};

    fn man(x: i32, y: i32) -> Observed {
        Observed {
            pos: Vec2::new(x, y),
            ..Default::default()
        }
    }

    fn stay(b: &Bodies) -> Vec<Target> {
        b.pos
            .iter()
            .map(|&point| Target {
                point,
                ..Default::default()
            })
            .collect()
    }

    fn run(s: &mut Pairwise, b: &mut Bodies, t: &[Target], ticks: u32) -> Vec<Event> {
        let rng = Rng::new(1);
        let mut out = Vec::new();
        for tick in 0..ticks {
            s.advance(b, t, SimMs(50), &rng, tick, &mut out);
        }
        out
    }

    #[test]
    fn overlapping_men_are_pushed_apart() {
        let mut b = Bodies::default();
        b.push(man(0, 0));
        b.push(man(100, 0));
        let mut s = Pairwise::new(Params::PLACEHOLDER);
        // Each wants to stay where he started, so they settle where the pulls balance.
        let t = stay(&b);
        run(&mut s, &mut b, &t, 1);
        assert!(b.pos[0].x < 0 && b.pos[1].x > 100);
        assert_eq!(b.pos[0].x, -b.pos[1].x + 100);
        // Coincident men are separated too.
        let mut c = Bodies::default();
        c.push(man(0, 0));
        c.push(man(0, 0));
        let t = stay(&c);
        run(&mut s, &mut c, &t, 1);
        assert!(c.pos[0].x < c.pos[1].x);
    }

    #[test]
    fn a_walker_does_not_pass_through_a_wall() {
        let mut b = Bodies::default();
        for k in -5..=5 {
            b.push(man(2_000, k * 560));
        }
        let walker = b.len();
        b.push(man(0, 0));
        let mut t = stay(&b);
        t[walker] = Target {
            point: Vec2::new(4_000, 0),
            speed: 1_300,
            face: Some(Turn(0)),
            press: 0,
        };
        // The wall men hold their places; give them full press so they resist.
        for tt in t.iter_mut().take(walker) {
            tt.press = 255;
            tt.face = Some(Turn(Turn::HALF));
        }
        let mut s = Pairwise::new(Params::PLACEHOLDER);
        run(&mut s, &mut b, &t, 100);
        assert!(b.pos[walker].x < 2_000, "walker at {:?}", b.pos[walker]);
    }

    #[test]
    fn thread_count_cannot_change_a_bit() {
        let mut crowd = Bodies::default();
        for r in 0..30 {
            for f in 0..30 {
                crowd.push(man(f * 450 + (r % 2) * 200, r * 430));
            }
        }
        let centre = Vec2::new(6_500, 6_000);
        let t: Vec<Target> = crowd
            .pos
            .iter()
            .map(|_| Target {
                point: centre,
                speed: 1_300,
                face: None,
                press: 128,
            })
            .collect();
        let mut finals = Vec::new();
        for threads in [1, 3, 8] {
            let mut b = crowd.clone();
            let mut s = Pairwise::new(Params::PLACEHOLDER);
            s.threads = threads;
            let ev = run(&mut s, &mut b, &t, 60);
            finals.push((b.state_hash(), ev));
        }
        assert_eq!(finals[0], finals[1]);
        assert_eq!(finals[0], finals[2]);
    }

    #[test]
    fn a_pressed_crowd_is_crushed_and_a_loose_one_is_not() {
        let mut dense = Bodies::default();
        for r in 0..10 {
            for f in 0..10 {
                dense.push(man(f * 400, r * 400));
            }
        }
        let centre = Vec2::new(1_800, 1_800);
        let squeeze: Vec<Target> = dense
            .pos
            .iter()
            .map(|_| Target {
                point: centre,
                speed: 1_300,
                face: None,
                press: 255,
            })
            .collect();
        let mut s = Pairwise::new(Params::PLACEHOLDER);
        let hurt = run(&mut s, &mut dense, &squeeze, 200);
        assert!(!hurt.is_empty());
        assert!(hurt.iter().all(|e| e.causes[0] == Cause::Crush));

        let mut loose = Bodies::default();
        for r in 0..10 {
            for f in 0..10 {
                loose.push(man(f * 1_000, r * 1_000));
            }
        }
        let t = stay(&loose);
        assert!(run(&mut s, &mut loose, &t, 200).is_empty());
        assert!(s.pressure().iter().all(|&p| p == 0));
    }
}
