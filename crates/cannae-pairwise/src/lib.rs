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
    keys: Vec<(i64, u32)>,
    corr: Vec<Vec2>,
    pressure: Vec<i32>,
    weight: Vec<i32>,
    old: Vec<Vec2>,
}

fn cell_key(cx: i32, cy: i32) -> i64 {
    ((cy as i64) << 32) | cx as u32 as i64
}

impl Pairwise {
    pub fn new(p: Params) -> Self {
        Pairwise {
            p,
            keys: Vec::new(),
            corr: Vec::new(),
            pressure: Vec::new(),
            weight: Vec::new(),
            old: Vec::new(),
        }
    }

    /// Each man's summed overlap with his neighbours after the last tick, in mm. Private to
    /// this substrate: men read crowding from the View, never this.
    pub fn pressure(&self) -> &[i32] {
        &self.pressure
    }

    fn cell(&self, p: Vec2) -> (i32, i32) {
        let size = 2 * self.p.radius_mm;
        (p.x.div_euclid(size), p.y.div_euclid(size))
    }

    /// One relaxation pass. A sort rather than a counting sort for the cell list: the speed
    /// spike (S0.7) measures whether that matters.
    fn relax(&mut self, bodies: &mut Bodies) {
        let n = bodies.len();
        self.keys.clear();
        for i in 0..n {
            if bodies.posture[i] != Posture::Down {
                let (cx, cy) = self.cell(bodies.pos[i]);
                self.keys.push((cell_key(cx, cy), i as u32));
            }
        }
        self.keys.sort_unstable();
        self.corr.clear();
        self.corr.resize(n, Vec2::ZERO);
        self.pressure.clear();
        self.pressure.resize(n, 0);
        let reach = 2 * self.p.radius_mm as i64;
        for &(_, i) in &self.keys {
            let i = i as usize;
            let pi = bodies.pos[i];
            let (cx, cy) = self.cell(pi);
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let k = cell_key(cx + dx, cy + dy);
                    let lo = self.keys.partition_point(|c| c.0 < k);
                    for &(kk, j) in &self.keys[lo..] {
                        if kk != k {
                            break;
                        }
                        let j = j as usize;
                        if j <= i {
                            continue;
                        }
                        let mut d = bodies.pos[j] - pi;
                        let d2 = d.len_sq();
                        if d2 >= reach * reach {
                            continue;
                        }
                        let mut dist = (d2 as u64).isqrt() as i64;
                        if dist == 0 {
                            // Exactly coincident: separate along x, lower id to the left.
                            d = Vec2::new(1, 0);
                            dist = 1;
                        }
                        let overlap = reach - dist;
                        let (wi, wj) = (self.weight[i] as i64, self.weight[j] as i64);
                        let push = overlap * self.p.stiffness_q8 as i64;
                        let mi = push * wj / (wi + wj) / 256;
                        let mj = push * wi / (wi + wj) / 256;
                        self.corr[i] -= d.mul_div(mi, dist);
                        self.corr[j] += d.mul_div(mj, dist);
                        self.pressure[i] += overlap as i32;
                        self.pressure[j] += overlap as i32;
                    }
                }
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
