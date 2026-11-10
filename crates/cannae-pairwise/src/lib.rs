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

use cannae_core::schedule::{MAN_STEP, is_man_step};
use cannae_core::{
    Bodies, Cause, Event, EventKind, Key, Posture, Prob, Rng, SimMs, Stream, Substrate, Target,
    Vec2, Wall, Wound,
};
use core::ops::Range;
use rayon::prelude::*;

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
    /// Injury hazard per second for each mm of overlap above `crush_mm`, Q32: the prior's
    /// rates are a few parts in a hundred thousand per mm.
    pub crush_rate_q32_per_mm: u32,
    /// Overlap with one neighbour beyond which bodies give no more: the excess is removed in
    /// full at every pass, not by `stiffness_q8`. A man squeezed by a crowd yields some way,
    /// and then he is bone.
    pub core_mm: i32,
}

impl Params {
    pub const PLACEHOLDER: Params = Params {
        radius_mm: 280,
        iterations: 2,
        stiffness_q8: 128,
        turn_per_tick: 1_638,
        press_mm_s: 300,
        brace_mm_s: 600,
        // The centres of the shared priors (priors-shared.md) at this radius: crushing begins
        // at 6.5 men per m², and at 8.5 a man is disabled in 134 s on average.
        crush_mm: 831,
        crush_rate_q32_per_mm: 100_830,
        core_mm: 150,
    };
}

/// The purpose word for a crush injury draw on the substrate stream.
pub const PURPOSE_CRUSH: u32 = 1;

/// Men per task in the per-man phases: large enough that scheduling costs little.
const MEN_PER_TASK: usize = 4_096;

pub struct Pairwise {
    pub p: Params,
    /// The ground's walls, from the scenario. No one passes through them.
    pub walls: Vec<Wall>,
    /// Threads for the parallel phases. Results are identical for any count: every phase
    /// either writes disjoint slices, or sums integer corrections, and integer addition does
    /// not care about order.
    pub threads: usize,
    pool: Option<(usize, rayon::ThreadPool)>,
    grid: Grid,
    /// Corrections and summed overlaps of one pass, in the grid's sorted order.
    corr: Vec<Vec2>,
    over: Vec<i32>,
    /// Summed overlaps of the last pass, by man.
    pressure: Vec<i32>,
    weight: Vec<i32>,
    old: Vec<Vec2>,
    /// Where each man wants to go, as a direction 1024 long, or zero.
    want: Vec<Vec2>,
    /// The summed directions, 1024 long each, of the men he touched ahead of him (in the half
    /// toward where he wanted to go) at the last pass. Sorted order during a pass, then by man.
    blk: Vec<Vec2>,
    block: Vec<Vec2>,
    /// The direction each man leans in, 1024 long, or zero; and his summed overlap with the
    /// men ahead of him in that direction at the last pass (sorted order, then by man).
    lean: Vec<Vec2>,
    fr: Vec<i32>,
    front: Vec<i32>,
}

/// The standing men sorted by cell, row by row, ties by id, with their positions and weights
/// copied into that order. A cell's men are contiguous, and so are the three cells below it in
/// the next row, so the neighbours of a man are two runs of the sorted list whose bounds only
/// ever move forward. No array is kept per cell, so empty ground costs nothing.
///
/// Cells are at least one diameter wide, and wider when men are spread thin. Their edges sit on
/// multiples of the cell size, so they stay put as the men move, and a column of empty cells
/// pads each side of every row, so a cell's neighbour to the east or below-left can never wrap
/// into another row's men.
///
/// Men rarely change cells between passes, so each build re-sorts the last order by insertion,
/// and falls back to a full sort when too many have moved. Both give the one order by (cell,
/// id), so which ran cannot change a result.
#[derive(Default)]
struct Grid {
    w: u32,
    /// Cell key (`row * w + col`) of each sorted man.
    key: Vec<u32>,
    /// Man ids in sorted order.
    id: Vec<u32>,
    px: Vec<i32>,
    py: Vec<i32>,
    wt: Vec<i32>,
    want: Vec<Vec2>,
    lean: Vec<Vec2>,
    // Scratch: (key << 32 | id) per sorted man, and whether each man was in the last build.
    v: Vec<u64>,
    listed: Vec<bool>,
}

const UNLISTED: u64 = u64::MAX;

impl Grid {
    fn build(
        &mut self,
        diameter: i32,
        pos: &[Vec2],
        weight: &[i32],
        want: &[Vec2],
        lean: &[Vec2],
        posture: &[Posture],
    ) {
        let n = pos.len();
        let (x0, y0, x1, y1, m) = pos
            .par_chunks(MEN_PER_TASK)
            .zip(posture.par_chunks(MEN_PER_TASK))
            .map(|(ps, qs)| {
                let mut b = (i32::MAX, i32::MAX, i32::MIN, i32::MIN, 0usize);
                for (p, &q) in ps.iter().zip(qs) {
                    if q != Posture::Down {
                        b = (
                            b.0.min(p.x),
                            b.1.min(p.y),
                            b.2.max(p.x),
                            b.3.max(p.y),
                            b.4 + 1,
                        );
                    }
                }
                b
            })
            .reduce(
                || (i32::MAX, i32::MAX, i32::MIN, i32::MIN, 0),
                |a, b| {
                    (
                        a.0.min(b.0),
                        a.1.min(b.1),
                        a.2.max(b.2),
                        a.3.max(b.3),
                        a.4 + b.4,
                    )
                },
            );
        let (x0, y0, x1, y1) = if m == 0 {
            (0, 0, 0, 0)
        } else {
            (x0, y0, x1, y1)
        };
        let (spanx, spany) = ((x1 - x0) as i64 + 1, (y1 - y0) as i64 + 1);
        let mut size = diameter as i64;
        // At most about eight cells per man, which keeps keys small.
        let cap = 8 * m as i64 + 4096;
        while (spanx / size + 1) * (spany / size + 1) > cap {
            size *= 2;
        }
        let size = size as i32;
        let (x0, y0) = (x0.div_euclid(size) * size, y0.div_euclid(size) * size);
        let w = ((x1 - x0) / size + 3) as u32;
        self.w = w;
        let cell = |p: Vec2| -> u64 {
            let row = ((p.y - y0) / size) as u64;
            let col = ((p.x - x0) / size) as u64 + 1;
            (row * w as u64 + col) << 32
        };
        // The last order, with the fallen dropped, then anyone newly standing, by id.
        let prev = core::mem::take(&mut self.id);
        self.v.resize(prev.len(), 0);
        self.v
            .par_chunks_mut(MEN_PER_TASK)
            .zip(prev.par_chunks(MEN_PER_TASK))
            .for_each(|(vs, ids)| {
                for (v, &i) in vs.iter_mut().zip(ids) {
                    let i = i as usize;
                    *v = if i < n && posture[i] != Posture::Down {
                        cell(pos[i]) | i as u64
                    } else {
                        UNLISTED
                    };
                }
            });
        self.v.retain(|&v| v != UNLISTED);
        self.listed.clear();
        self.listed.resize(n, false);
        for &v in &self.v {
            self.listed[v as u32 as usize] = true;
        }
        for i in 0..n {
            if !self.listed[i] && posture[i] != Posture::Down {
                self.v.push(cell(pos[i]) | i as u64);
            }
        }
        debug_assert_eq!(self.v.len(), m);
        if !insertion_sort(&mut self.v, 4 * m + 64) {
            self.v.sort_unstable();
        }
        self.id = prev;
        self.id.resize(m, 0);
        self.key.resize(m, 0);
        self.px.resize(m, 0);
        self.py.resize(m, 0);
        self.wt.resize(m, 0);
        self.want.resize(m, Vec2::ZERO);
        self.lean.resize(m, Vec2::ZERO);
        let v = &self.v;
        self.id
            .par_chunks_mut(MEN_PER_TASK)
            .zip(self.key.par_chunks_mut(MEN_PER_TASK))
            .zip(self.px.par_chunks_mut(MEN_PER_TASK))
            .zip(self.py.par_chunks_mut(MEN_PER_TASK))
            .zip(self.wt.par_chunks_mut(MEN_PER_TASK))
            .zip(self.want.par_chunks_mut(MEN_PER_TASK))
            .zip(self.lean.par_chunks_mut(MEN_PER_TASK))
            .zip(v.par_chunks(MEN_PER_TASK))
            .for_each(|(((((((ids, ks), xs), ys), ws), wa), le), vs)| {
                for k in 0..vs.len() {
                    let i = vs[k] as u32 as usize;
                    ids[k] = i as u32;
                    ks[k] = (vs[k] >> 32) as u32;
                    xs[k] = pos[i].x;
                    ys[k] = pos[i].y;
                    ws[k] = weight[i];
                    wa[k] = want.get(i).copied().unwrap_or(Vec2::ZERO);
                    le[k] = lean.get(i).copied().unwrap_or(Vec2::ZERO);
                }
            });
    }

    /// One past the last sorted man that the men of `run` can touch: the end of the cells
    /// below-right of the run's last man.
    fn reach_end(&self, run: Range<usize>) -> usize {
        if run.is_empty() {
            return run.start;
        }
        let k = self.key[run.end - 1] + self.w + 1;
        self.key.partition_point(|&x| x <= k)
    }
}

/// Sorts `v` by insertion if that takes at most `budget` moves, and says whether it did. Cheap
/// when the list is nearly sorted already, which it is from one pass to the next.
fn insertion_sort(v: &mut [u64], budget: usize) -> bool {
    let mut moves = 0;
    for i in 1..v.len() {
        let x = v[i];
        let mut j = i;
        while j > 0 && v[j - 1] > x {
            v[j] = v[j - 1];
            j -= 1;
        }
        v[j] = x;
        moves += i - j;
        if moves > budget {
            return false;
        }
    }
    true
}

/// The contact between sorted men `a` and `b`, if any, added to `corr`, `over` and `blk`, which
/// start at sorted position `base`. Symmetric in `a` and `b` except for exact coincidence,
/// which is broken by man id, so the result does not depend on which of the two finds the
/// other.
#[inline]
fn contact(g: &Grid, p: &Params, a: usize, b: usize, base: usize, out: &mut Out) {
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
    let core = p.core_mm as i64;
    let push = overlap.min(core) * p.stiffness_q8 as i64 + (overlap - core).max(0) * 256;
    let ma = push * wb / (wa + wb) / 256;
    let mb = push * wa / (wa + wb) / 256;
    let (i, j) = (a - base, b - base);
    out.corr[i] -= d.mul_div(ma, dist);
    out.corr[j] += d.mul_div(mb, dist);
    out.over[i] += overlap as i32;
    out.over[j] += overlap as i32;
    // Each man is blocked by a neighbour in the half toward where he wants to go.
    let unit = d.mul_div(1024, dist);
    if g.want[a].dot(d) > 0 {
        out.blk[i] += unit;
    }
    if g.want[b].dot(d) < 0 {
        out.blk[j] -= unit;
    }
    if g.lean[a].dot(d) > 0 {
        out.front[i] += overlap as i32;
    }
    if g.lean[b].dot(d) < 0 {
        out.front[j] += overlap as i32;
    }
}

/// One run's slices of the shared lists.
struct Out<'a> {
    corr: &'a mut [Vec2],
    over: &'a mut [i32],
    blk: &'a mut [Vec2],
    front: &'a mut [i32],
}

/// All contacts of the sorted men in `run`: each man with those after him in his own cell and
/// the cell to his east, and with the three cells below him in the next row, so each pair is
/// found exactly once. Writes stay below `g.reach_end(run)`.
fn pairs(g: &Grid, p: &Params, run: Range<usize>, base: usize, out: &mut Out) {
    if run.is_empty() {
        return;
    }
    let (key, w, m) = (&g.key, g.w, g.key.len());
    let k0 = key[run.start];
    let mut east = key.partition_point(|&x| x <= k0 + 1);
    let mut below = key.partition_point(|&x| x < k0 + w - 1);
    let mut below_end = key.partition_point(|&x| x <= k0 + w + 1);
    for a in run {
        let k = key[a];
        while east < m && key[east] <= k + 1 {
            east += 1;
        }
        while below < m && key[below] < k + w - 1 {
            below += 1;
        }
        while below_end < m && key[below_end] <= k + w + 1 {
            below_end += 1;
        }
        for b in a + 1..east {
            contact(g, p, a, b, base, out);
        }
        for b in below..below_end {
            contact(g, p, a, b, base, out);
        }
    }
}

/// Splits `s`, which starts at sorted position `bounds[0]`, at each of `bounds[1..]`.
fn split_at<'a, T>(mut s: &'a mut [T], bounds: &[usize]) -> Vec<&'a mut [T]> {
    let mut parts = Vec::with_capacity(bounds.len());
    for pair in bounds.windows(2) {
        let (head, tail) = s.split_at_mut(pair[1] - pair[0]);
        parts.push(head);
        s = tail;
    }
    parts
}

impl Pairwise {
    pub fn new(p: Params) -> Self {
        Pairwise {
            p,
            walls: Vec::new(),
            threads: 1,
            pool: None,
            grid: Grid::default(),
            corr: Vec::new(),
            over: Vec::new(),
            pressure: Vec::new(),
            weight: Vec::new(),
            old: Vec::new(),
            want: Vec::new(),
            blk: Vec::new(),
            block: Vec::new(),
            lean: Vec::new(),
            fr: Vec::new(),
            front: Vec::new(),
        }
    }

    /// Each man's summed overlap with his neighbours after the last tick, in mm. Private to
    /// this substrate: men read crowding from the View, never this.
    pub fn pressure(&self) -> &[i32] {
        &self.pressure
    }

    /// One relaxation pass. The sorted list is cut into runs of men; a run's contacts reach at
    /// most into the next run, so the even runs can be done together, and then the odd ones,
    /// each writing into the one shared list of corrections.
    fn relax(&mut self, bodies: &mut Bodies, last: bool) {
        self.grid.build(
            2 * self.p.radius_mm,
            &bodies.pos,
            &self.weight,
            &self.want,
            &self.lean,
            &bodies.posture,
        );
        let g = &self.grid;
        let m = g.id.len();
        // Many more runs than threads: contacts per man vary a great deal between a packed edge
        // and a loose interior, and work stealing evens that out only if there is work to steal.
        let mut runs = if self.threads > 1 {
            16 * self.threads
        } else {
            1
        };
        let cuts = loop {
            let cuts: Vec<usize> = (0..=runs).map(|j| j * m / runs).collect();
            let fits =
                (0..runs).all(|j| g.reach_end(cuts[j]..cuts[j + 1]) <= cuts[(j + 2).min(runs)]);
            if fits || runs == 1 {
                break cuts;
            }
            runs /= 2;
        };
        self.corr.clear();
        self.corr.resize(m, Vec2::ZERO);
        self.over.clear();
        self.over.resize(m, 0);
        self.blk.clear();
        self.blk.resize(m, Vec2::ZERO);
        self.fr.clear();
        self.fr.resize(m, 0);
        let p = &self.p;
        for parity in 0..2 {
            // Run j may write from cuts[j] up to cuts[j + 2].
            let mut bounds: Vec<usize> = (parity..=runs).step_by(2).map(|j| cuts[j]).collect();
            if *bounds.last().expect("at least one bound") != m {
                bounds.push(m);
            }
            let lo = bounds[0];
            // The i-th slice belongs to run parity + 2i.
            let tasks: Vec<_> = split_at(&mut self.corr[lo..], &bounds)
                .into_iter()
                .zip(split_at(&mut self.over[lo..], &bounds))
                .zip(split_at(&mut self.blk[lo..], &bounds))
                .zip(split_at(&mut self.fr[lo..], &bounds))
                .enumerate()
                .map(|(i, (((corr, over), blk), front))| {
                    (
                        parity + 2 * i,
                        Out {
                            corr,
                            over,
                            blk,
                            front,
                        },
                    )
                })
                .collect();
            let cuts = &cuts;
            tasks.into_par_iter().for_each(|(j, mut out)| {
                pairs(g, p, cuts[j]..cuts[j + 1], cuts[j], &mut out);
            });
        }
        // Walls do not move, so a man touching one takes the whole correction. A wall counts
        // toward how hard he is pressed, and blocks his walking and his lean as a man would.
        if !self.walls.is_empty() {
            let (walls, r) = (&self.walls, self.p.radius_mm as i64);
            self.corr
                .par_chunks_mut(MEN_PER_TASK)
                .zip(self.over.par_chunks_mut(MEN_PER_TASK))
                .zip(self.blk.par_chunks_mut(MEN_PER_TASK))
                .zip(self.fr.par_chunks_mut(MEN_PER_TASK))
                .enumerate()
                .for_each(|(chunk, (((cs, os), bs), fs))| {
                    let first = chunk * MEN_PER_TASK;
                    for k in 0..cs.len() {
                        let a = first + k;
                        let at = Vec2::new(g.px[a], g.py[a]);
                        for w in walls {
                            let d = at - w.nearest(at);
                            let d2 = d.len_sq();
                            if d2 >= r * r {
                                continue;
                            }
                            let dist = (d2 as u64).isqrt() as i64;
                            let unit = if dist == 0 {
                                w.left()
                            } else {
                                d.mul_div(1_024, dist)
                            };
                            let overlap = r - dist;
                            cs[k] += unit.mul_div(overlap, 1_024);
                            os[k] += overlap as i32;
                            if g.want[a].dot(unit) < 0 {
                                bs[k] -= unit;
                            }
                            if g.lean[a].dot(unit) < 0 {
                                fs[k] += overlap as i32;
                            }
                        }
                    }
                });
        }
        if last {
            self.pressure.clear();
            self.pressure.resize(bodies.len(), 0);
            self.block.clear();
            self.block.resize(bodies.len(), Vec2::ZERO);
            self.front.clear();
            self.front.resize(bodies.len(), 0);
        }
        for (k, &i) in g.id.iter().enumerate() {
            bodies.pos[i as usize] += self.corr[k];
            if last {
                self.pressure[i as usize] = self.over[k];
                self.block[i as usize] = self.blk[k];
                self.front[i as usize] = self.fr[k];
            }
        }
    }

    fn advance_in_pool(
        &mut self,
        bodies: &mut Bodies,
        targets: &[Target],
        dt: SimMs,
        rng: &Rng,
        tick: u32,
        out: &mut Vec<Event>,
    ) {
        self.old.clear();
        self.old.extend_from_slice(&bodies.pos);
        self.weight.clear();
        self.weight
            .extend(targets.iter().map(|t| 256 + t.press as i32));
        let p = self.p;
        let n = bodies.len();
        self.want.resize(n, Vec2::ZERO);
        self.block.resize(n, Vec2::ZERO);
        self.lean.resize(n, Vec2::ZERO);
        self.front.resize(n, 0);
        let (block, front) = (&self.block, &self.front);
        bodies
            .pos
            .par_chunks_mut(MEN_PER_TASK)
            .zip(self.want.par_chunks_mut(MEN_PER_TASK))
            .zip(self.lean.par_chunks_mut(MEN_PER_TASK))
            .zip(targets.par_chunks(MEN_PER_TASK))
            .zip(bodies.posture.par_chunks(MEN_PER_TASK))
            .enumerate()
            .for_each(|(chunk, ((((ps, ws), ls), ts), qs))| {
                let first = chunk * MEN_PER_TASK;
                for (k, ((((pos, want), lean_dir), t), &q)) in ps
                    .iter_mut()
                    .zip(ws.iter_mut())
                    .zip(ls.iter_mut())
                    .zip(ts)
                    .zip(qs)
                    .enumerate()
                {
                    *lean_dir = Vec2::ZERO;
                    if q == Posture::Down {
                        *want = Vec2::ZERO;
                        continue;
                    }
                    let to = t.point - *pos;
                    *want = if to == Vec2::ZERO {
                        Vec2::ZERO
                    } else {
                        to.with_len(1024)
                    };
                    let speed = if t.speed == 0 { p.brace_mm_s } else { t.speed };
                    let step = (speed as i64 * dt.0 as i64 / 1000) as i32;
                    let mut mv = if to.len_sq() <= step as i64 * step as i64 {
                        to
                    } else {
                        to.with_len(step)
                    };
                    // A man cannot walk into the men he touches ahead of him; he slides along
                    // them, or stands. Only his lean, below, pushes them.
                    let b = block[first + k];
                    let along = mv.dot(b);
                    if along > 0 {
                        mv -= b.mul_div(along, b.len_sq());
                    }
                    // He leans into the men ahead until they are pressed to the bone: a man
                    // cannot push further than bodies give.
                    if t.press > 0
                        && let Some(f) = t.face
                    {
                        *lean_dir = f.unit(1024);
                        let room = (p.core_mm - front[first + k]).max(0) as i64;
                        let lean = p.press_mm_s as i64 * t.press as i64 / 255 * dt.0 as i64 / 1000
                            * room
                            / p.core_mm.max(1) as i64;
                        mv += f.unit(lean as i32);
                    }
                    *pos += mv;
                }
            });
        for pass in 0..self.p.iterations {
            self.relax(bodies, pass + 1 == self.p.iterations);
        }
        if self.p.iterations == 0 {
            self.pressure.clear();
            self.pressure.resize(bodies.len(), 0);
        }
        let (pos, old, pressure) = (&bodies.pos, &self.old, &self.pressure);
        let crush_step = is_man_step(tick);
        let events: Vec<Vec<Event>> = bodies
            .vel
            .par_chunks_mut(MEN_PER_TASK)
            .zip(bodies.facing.par_chunks_mut(MEN_PER_TASK))
            .zip(bodies.posture.par_chunks(MEN_PER_TASK))
            .enumerate()
            .map(|(chunk, ((vs, fs), qs))| {
                let mut ev = Vec::new();
                let first = chunk * MEN_PER_TASK;
                for (k, ((v, f), &q)) in vs.iter_mut().zip(fs.iter_mut()).zip(qs).enumerate() {
                    let i = first + k;
                    if q == Posture::Down {
                        *v = Vec2::ZERO;
                        continue;
                    }
                    *v = (pos[i] - old[i]).mul_div(1000, dt.0 as i64);
                    if let Some(face) = targets[i].face {
                        *f = f.rotate_toward(face, p.turn_per_tick);
                    }
                    // Crushing is drawn at the man's step, over the whole step: the same hazard
                    // as drawing every tick, at a fifth of the cost.
                    let excess = pressure[i] - p.crush_mm;
                    if crush_step && excess > 0 {
                        let rate = ((excess as u64 * p.crush_rate_q32_per_mm as u64) >> 16)
                            .min(u32::MAX as u64);
                        let chance = Prob::from_rate(rate as u32, MAN_STEP);
                        if rng.chance(
                            chance,
                            Key::new(Stream::Substrate, i as u32, tick, PURPOSE_CRUSH),
                        ) {
                            ev.push(Event {
                                tick,
                                man: cannae_core::ManId(i as u32),
                                pos: pos[i],
                                kind: EventKind::Wound {
                                    to: Wound::Disabled,
                                },
                                causes: [Cause::Crush, Cause::None, Cause::None],
                            });
                        }
                    }
                }
                ev
            })
            .collect();
        out.extend(events.into_iter().flatten());
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
        let threads = self.threads.max(1);
        let pool = match self.pool.take() {
            Some((t, pool)) if t == threads => pool,
            _ => rayon::ThreadPoolBuilder::new()
                .num_threads(threads)
                .build()
                .expect("a thread pool"),
        };
        pool.install(|| self.advance_in_pool(bodies, targets, dt, rng, tick, out));
        self.pool = Some((threads, pool));
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
    fn a_pressing_ring_cannot_walk_through_a_standing_crowd() {
        // A 20 x 20 crowd at 0.6 m, standing; a ring of men around it walking at its centre and
        // leaning in with all their weight. They may press its edge, but not pass through it,
        // and nobody is squeezed past the bone.
        let mut b = Bodies::default();
        for r in 0..20 {
            for f in 0..20 {
                b.push(man(f * 600 - 5_700, r * 600 - 5_700));
            }
        }
        let inner = b.len();
        for k in 0..240 {
            let a = Turn((k * 65_536 / 240) as u16);
            let ring = 9_000 + (k % 3) * 600;
            b.push(Observed {
                pos: a.unit(ring),
                ..Default::default()
            });
        }
        let mut t = stay(&b);
        for (i, tt) in t.iter_mut().enumerate().skip(inner) {
            *tt = Target {
                point: Vec2::ZERO,
                speed: 1_300,
                face: Turn::of(-b.pos[i]),
                press: 255,
            };
        }
        let mut s = Pairwise::new(Params::PLACEHOLDER);
        run(&mut s, &mut b, &t, 600);
        // The crowd is squeezed from 2.8 men per m2 toward the bone, which for this disc and core
        // is about 6.9 per m2 (contact at 2r - core = 410 mm): its side shrinks from 11.4 m to
        // about 7.2 m. No ring man gets into its middle, and it is no denser than bone allows.
        let inside = |i: usize, half: i32| b.pos[i].x.abs() <= half && b.pos[i].y.abs() <= half;
        assert!(
            (inner..b.len()).all(|i| !inside(i, 3_000)),
            "a ring man got into the crowd"
        );
        let packed = (0..inner).filter(|&i| inside(i, 2_000)).count();
        assert!(packed <= 120, "{packed} men in 16 m2");
        let worst = *s.pressure().iter().max().unwrap();
        assert!(
            worst < 6 * Params::PLACEHOLDER.core_mm,
            "summed overlap {worst} mm"
        );
    }

    #[test]
    fn walls_stop_men_and_a_gap_lets_them_through() {
        // A wall across x = 2 m with a gap of 1.2 m at y = 0; men behind it walking at full
        // press to a point beyond. Some get through the gap; none pass the wall elsewhere, and
        // none ends up inside it.
        let mut b = Bodies::default();
        for r in 0..6 {
            for f in 0..10 {
                b.push(man(-r * 600, (f - 5) * 600 + 300));
            }
        }
        let t: Vec<Target> = b
            .pos
            .iter()
            .map(|_| Target {
                point: Vec2::new(10_000, 0),
                speed: 1_300,
                face: Some(Turn(0)),
                press: 255,
            })
            .collect();
        let mut s = Pairwise::new(Params::PLACEHOLDER);
        s.walls = vec![
            Wall::new(Vec2::new(2_000, 600), Vec2::new(2_000, 10_000)),
            Wall::new(Vec2::new(2_000, -600), Vec2::new(2_000, -10_000)),
        ];
        run(&mut s, &mut b, &t, 400);
        let through: Vec<usize> = (0..b.len()).filter(|&i| b.pos[i].x > 2_300).collect();
        assert!(through.len() >= 5, "{} through", through.len());
        let r = Params::PLACEHOLDER.radius_mm;
        for (i, p) in b.pos.iter().enumerate() {
            for w in &s.walls {
                let d = p.dist_sq(w.nearest(*p));
                assert!(d >= (r as i64 - 60).pow(2), "man {i} is in a wall at {p:?}");
            }
        }
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
        // An early onset and a fast rate, so ten seconds show it.
        let mut s = Pairwise::new(Params {
            crush_mm: 250,
            crush_rate_q32_per_mm: 42_949_673,
            ..Params::PLACEHOLDER
        });
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
