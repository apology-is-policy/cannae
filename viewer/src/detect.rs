//! The detectors of the visual QC pass (docs/slice0/qc-pass.md): each reads the men's
//! observable state after every man step and flags what would look wrong. The document fixes
//! what each one flags and its thresholds; nothing here may be tuned to what a battle shows.
//!
//! How the document's sentences are read:
//!
//! - **A sample** is the state after a man step; samples are 250 ms apart. "Over 2 s" compares
//!   a sample with the one eight before it. A state that must last "D s or more" must hold at
//!   consecutive samples spanning D, and "more than D s" at samples spanning more than D. A
//!   movement that must last 1 s must be made in four consecutive steps.
//! - **A flag is an episode:** a condition holding for one subject (a man, a pair of men, a
//!   body, a cell) at consecutive samples. It is written once, with its first and last sample.
//! - **A kind** is a mechanical sub-class of a flag, for grouping what is looked at. It is
//!   never a judgement.

use cannae_core::view::{BodyState, Order, SlotView, Standard, nb};
use cannae_core::{BodyId, Event, EventKind, Posture, Side, Turn, Vec2};
use std::collections::{HashMap, VecDeque};

use crate::BodyInfo;

/// Seconds to samples.
const fn samples(seconds: u32) -> u32 {
    seconds * 4
}

/// The base ticks in one man step.
const TICKS: u32 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Detector {
    SlotJump = 1,
    HaltedBodyWalks,
    PinnedRunner,
    OverlapEnemies,
    OverlapOwn,
    Crowd,
    Jump,
    DownMoves,
    FacingInPlace,
    FlightToward,
    BoutReach,
    BoutState,
    BoutThrough,
    Flicker,
    StandardFront,
    OutOfPlace,
    ForwardIdle,
    Gap,
    Leaderless,
    Far,
    // 21 and 22 are the two dormancy checks, which are not detectors of this kind.
    SlotTwice = 23,
    DownRises,
    SlotBack,
    Stalled,
    OverPacked,
    SuddenFlight,
}

impl Detector {
    pub const ALL: [Detector; 26] = [
        Detector::SlotJump,
        Detector::HaltedBodyWalks,
        Detector::PinnedRunner,
        Detector::OverlapEnemies,
        Detector::OverlapOwn,
        Detector::Crowd,
        Detector::Jump,
        Detector::DownMoves,
        Detector::FacingInPlace,
        Detector::FlightToward,
        Detector::BoutReach,
        Detector::BoutState,
        Detector::BoutThrough,
        Detector::Flicker,
        Detector::StandardFront,
        Detector::OutOfPlace,
        Detector::ForwardIdle,
        Detector::Gap,
        Detector::Leaderless,
        Detector::Far,
        Detector::SlotTwice,
        Detector::DownRises,
        Detector::SlotBack,
        Detector::Stalled,
        Detector::OverPacked,
        Detector::SuddenFlight,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Detector::SlotJump => "slot-jump",
            Detector::HaltedBodyWalks => "halted-body-walks",
            Detector::PinnedRunner => "pinned-runner",
            Detector::OverlapEnemies => "overlap-enemies",
            Detector::OverlapOwn => "overlap-own",
            Detector::Crowd => "crowd",
            Detector::Jump => "jump",
            Detector::DownMoves => "down-moves",
            Detector::FacingInPlace => "facing-in-place",
            Detector::FlightToward => "flight-toward",
            Detector::BoutReach => "bout-reach",
            Detector::BoutState => "bout-state",
            Detector::BoutThrough => "bout-through",
            Detector::Flicker => "flicker",
            Detector::StandardFront => "standard-front",
            Detector::OutOfPlace => "out-of-place",
            Detector::ForwardIdle => "forward-idle",
            Detector::Gap => "gap",
            Detector::Leaderless => "leaderless",
            Detector::Far => "far",
            Detector::SlotTwice => "slot-twice",
            Detector::DownRises => "down-rises",
            Detector::SlotBack => "slot-back",
            Detector::Stalled => "stalled",
            Detector::OverPacked => "over-packed",
            Detector::SuddenFlight => "sudden-flight",
        }
    }

    /// How many samples an episode's last must come after its first for it to be a flag.
    fn span(self) -> u32 {
        match self {
            Detector::FacingInPlace => samples(1),
            // Four steps: each hit is one step's movement.
            Detector::FlightToward => 3,
            Detector::OutOfPlace => samples(10),
            Detector::Gap => samples(10) + 1,
            Detector::Leaderless => samples(20) + 1,
            _ => 0,
        }
    }

    /// How many samples before its first hit the thing flagged had already been going on: a
    /// condition over 2 s is first met at the end of its 2 s.
    pub fn lead_in(self) -> u32 {
        match self {
            Detector::HaltedBodyWalks | Detector::PinnedRunner => samples(2),
            Detector::ForwardIdle => samples(20),
            Detector::Stalled => samples(10),
            Detector::FlightToward => 1,
            _ => 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Flag {
    pub detector: Detector,
    /// The first and the last sample at which the condition held.
    pub first: u32,
    pub last: u32,
    pub men: Vec<u32>,
    pub body: Option<u32>,
    /// Where it was at the first sample.
    pub at: Vec2,
    pub kind: &'static str,
    /// The largest value of the detector's measure over the episode.
    pub worst: i64,
    /// What was measured at the first sample.
    pub note: String,
}

/// What an episode is opened with.
struct Hit {
    men: Vec<u32>,
    body: Option<u32>,
    at: Vec2,
    kind: &'static str,
    note: String,
}

struct Open {
    first: u32,
    last: u32,
    worst: i64,
    hit: Hit,
}

#[derive(Default)]
struct Episodes {
    open: HashMap<(Detector, i64, i64), Open>,
    flags: Vec<Flag>,
}

impl Episodes {
    /// The condition of `d` holds for the subject `key` at sample `k`. `open` describes it,
    /// and is asked only when this starts an episode.
    fn hit(
        &mut self,
        k: u32,
        d: Detector,
        key: (i64, i64),
        worst: i64,
        open: impl FnOnce() -> Hit,
    ) {
        let e = self.open.entry((d, key.0, key.1)).or_insert_with(|| Open {
            first: k,
            last: k,
            worst,
            hit: open(),
        });
        e.last = k;
        e.worst = e.worst.max(worst);
    }

    /// Ends every episode that was not hit at sample `k`; all of them if `k` is `None`.
    fn sweep(&mut self, k: Option<u32>) {
        let flags = &mut self.flags;
        self.open.retain(|&(d, _, _), e| {
            if k.is_some_and(|k| e.last >= k) {
                return true;
            }
            if e.last - e.first >= d.span() {
                flags.push(Flag {
                    detector: d,
                    first: e.first,
                    last: e.last,
                    men: std::mem::take(&mut e.hit.men),
                    body: e.hit.body,
                    at: e.hit.at,
                    kind: e.hit.kind,
                    worst: e.worst,
                    note: std::mem::take(&mut e.hit.note),
                });
            }
            false
        });
    }
}

/// What does not change during a battle.
#[derive(Clone, Debug)]
pub struct Scene {
    pub side: Vec<Side>,
    pub body: Vec<Option<BodyId>>,
    pub bodies: Vec<BodyInfo>,
    /// The mean of the men's positions at the start.
    pub centre: Vec2,
}

impl Scene {
    pub fn new(side: &[Side], body: &[Option<BodyId>], bodies: &[BodyInfo], pos: &[Vec2]) -> Scene {
        let n = pos.len().max(1) as i64;
        let (x, y) = pos
            .iter()
            .fold((0i64, 0i64), |(x, y), p| (x + p.x as i64, y + p.y as i64));
        Scene {
            side: side.to_vec(),
            body: body.to_vec(),
            bodies: bodies.to_vec(),
            centre: Vec2::new((x / n) as i32, (y / n) as i32),
        }
    }
}

/// The state after one man step.
#[derive(Clone, Copy)]
pub struct Sample<'a> {
    pub pos: &'a [Vec2],
    pub facing: &'a [Turn],
    pub posture: &'a [Posture],
    pub slot: &'a [Option<SlotView>],
    /// By body.
    pub state: &'a [BodyState],
    /// The events logged since the last sample.
    pub events: &'a [Event],
    /// The postures the bout phase read at this man step.
    pub fought: &'a [Posture],
}

struct Past {
    pos: Vec<Vec2>,
    posture: Vec<Posture>,
}

fn standing(p: Posture) -> bool {
    matches!(p, Posture::Steady | Posture::Forward | Posture::Back)
}

/// The components of `v` ahead and to the left of `facing`.
fn split(v: Vec2, facing: Turn) -> (i64, i64) {
    let ahead = facing.unit(1 << 14);
    let left = Turn(facing.0.wrapping_add(Turn::QUARTER)).unit(1 << 14);
    (v.dot(ahead) >> 14, v.dot(left) >> 14)
}

fn cell(p: Vec2) -> (i32, i32) {
    (p.x.div_euclid(CELL_MM), p.y.div_euclid(CELL_MM))
}

const CELL_MM: i32 = 2_000;
const CROWD: usize = 24;
const OVER_PACKED: usize = 40;
const OVERLAP_MM: i64 = 300;

pub struct Detectors {
    scene: Scene,
    /// Samples taken so far.
    k: u32,
    /// The last nine samples, so that the front one is 2 s before the next.
    hist: VecDeque<Past>,
    prev_slot: Vec<Option<Vec2>>,
    prev_state: Vec<BodyState>,
    /// For how many samples in a row each man has been in flight.
    turned_run: Vec<u32>,
    /// For how many samples in a row each body's order has been to hold.
    hold_run: Vec<u32>,
    /// For how many samples in a row each body's order has been to advance, charge or pursue.
    led_run: Vec<u32>,
    /// Every man's slot at the last forty samples, so that the front one is 10 s before the
    /// next.
    slot_hist: VecDeque<Vec<Option<Vec2>>>,
    /// Where each man lay at the first sample at which he was down.
    fell_at: Vec<Option<Vec2>>,
    /// Each man's changes of posture in the last 5 s: the tick, and the two postures.
    changes: Vec<VecDeque<(u32, Posture, Posture)>>,
    /// The sample since which each man has been forward.
    forward_from: Vec<Option<u32>>,
    /// The sample of each man's last bout.
    last_bout: Vec<Option<u32>>,
    eps: Episodes,
}

impl Detectors {
    pub fn new(scene: Scene) -> Detectors {
        let (n, b) = (scene.side.len(), scene.bodies.len());
        Detectors {
            scene,
            k: 0,
            hist: VecDeque::new(),
            prev_slot: vec![None; n],
            prev_state: Vec::new(),
            turned_run: vec![0; n],
            hold_run: vec![0; b],
            led_run: vec![0; b],
            slot_hist: VecDeque::new(),
            fell_at: vec![None; n],
            changes: vec![VecDeque::new(); n],
            forward_from: vec![None; n],
            last_bout: vec![None; n],
            eps: Episodes::default(),
        }
    }

    /// The flags, in the order of their first sample. Episodes still open are ended.
    pub fn finish(mut self) -> Vec<Flag> {
        self.eps.sweep(None);
        let mut flags = self.eps.flags;
        flags.sort_by(|a, b| {
            (a.first, a.detector, &a.men, a.body).cmp(&(b.first, b.detector, &b.men, b.body))
        });
        flags
    }

    pub fn step(&mut self, s: &Sample) {
        let k = self.k;
        let n = s.pos.len();
        let scene = &self.scene;
        let eps = &mut self.eps;
        let up = |i: usize| s.posture[i] != Posture::Down;
        for i in 0..n {
            self.turned_run[i] = if s.posture[i] == Posture::Turned {
                self.turned_run[i] + 1
            } else {
                0
            };
        }
        for (b, st) in s.state.iter().enumerate() {
            self.hold_run[b] = if st.order == Order::Hold {
                self.hold_run[b] + 1
            } else {
                0
            };
            let led = matches!(st.order, Order::Advance | Order::Charge | Order::Pursue);
            self.led_run[b] = if led { self.led_run[b] + 1 } else { 0 };
        }
        let prev = self.hist.back();
        // The sample 2 s ago.
        let old = (self.hist.len() >= samples(2) as usize)
            .then(|| &self.hist[self.hist.len() - samples(2) as usize]);
        let men_of = |info: &BodyInfo| info.first as usize..(info.first + info.count) as usize;

        // 1. slot-jump; 25. slot-back; 23. slot-twice.
        for (b, info) in scene.bodies.iter().enumerate() {
            let mut slotted = 0;
            let mut facing = None;
            // Every slotted man who had a slot at the last sample, and how far it moved.
            let mut moves = Vec::new();
            let mut held: HashMap<(i32, i32), usize> = HashMap::new();
            for i in men_of(info) {
                let Some(sv) = s.slot[i] else { continue };
                slotted += 1;
                facing = Some(sv.facing);
                if let Some(was) = self.prev_slot[i] {
                    moves.push((i, sv.target - was));
                }
                if !up(i) {
                    continue;
                }
                if let Some(&j) = held.get(&(sv.target.x, sv.target.y)) {
                    eps.hit(k, Detector::SlotTwice, (j as i64, i as i64), 0, || Hit {
                        men: vec![j as u32, i as u32],
                        body: Some(b as u32),
                        at: sv.target,
                        kind: "",
                        note: format!(
                            "two men who are up hold the slot at ({}, {})",
                            sv.target.x, sv.target.y
                        ),
                    });
                } else {
                    held.insert((sv.target.x, sv.target.y), i);
                }
            }
            let Some(facing) = facing else { continue };
            // The vector the slots in `of` moved by, if half the body's slots moved by it to
            // within 100 mm, and which did.
            let common = |of: &[(usize, Vec2)]| -> Option<(Vec2, Vec<usize>)> {
                if of.is_empty() {
                    return None;
                }
                // Both medians of each coordinate: where exactly half moved, one of them is
                // the half that moved and the other the half that stayed.
                let medians = |f: fn(&Vec2) -> i32| {
                    let mut c: Vec<i32> = of.iter().map(|(_, v)| f(v)).collect();
                    c.sort_unstable();
                    [c[(c.len() - 1) / 2], c[c.len() / 2]]
                };
                let (xs, ys) = (medians(|v| v.x), medians(|v| v.y));
                xs.iter()
                    .flat_map(|&x| ys.iter().map(move |&y| Vec2::new(x, y)))
                    .map(|by| {
                        let same: Vec<usize> = of
                            .iter()
                            .filter(|(_, v)| (*v - by).len_sq() <= 100 * 100)
                            .map(|&(i, _)| i)
                            .collect();
                        (by, same)
                    })
                    .filter(|(_, same)| same.len() * 2 >= slotted)
                    .max_by_key(|(by, same)| (same.len(), by.len_sq()))
            };
            let (now, was) = (s.state[b], self.prev_state.get(b).copied());
            let lead = info.tie.lead_mm as i64;
            let told = |by: Vec2, same: &[usize]| {
                let (ahead, left) = split(by, facing);
                // The frame leads its men by a fixed distance while the body moves, so its
                // slots move by that much when the lead comes on or goes off.
                let kind = if left.abs() <= 100 && (ahead.abs() - lead).abs() <= 100 {
                    "by-the-lead"
                } else {
                    "other"
                };
                Hit {
                    men: Vec::new(),
                    body: Some(b as u32),
                    at: s.pos[same[0]],
                    kind,
                    note: format!(
                        "{} of {slotted} slots moved {ahead} mm ahead and {left} mm left; order \
                         {:?} -> {:?}; standard {:?} -> {:?}",
                        same.len(),
                        was.map(|w| w.order),
                        now.order,
                        was.map(|w| w.standard),
                        now.standard
                    ),
                }
            };
            let far: Vec<(usize, Vec2)> = moves
                .iter()
                .copied()
                .filter(|(_, v)| v.len_sq() > 1_200 * 1_200)
                .collect();
            if let Some((by, same)) = common(&far) {
                eps.hit(k, Detector::SlotJump, (b as i64, 0), by.length(), || {
                    told(by, &same)
                });
            }
            if let Some((by, same)) = common(&moves) {
                let back = match now.order {
                    Order::Hold => by.len_sq() > 300 * 300,
                    Order::Advance | Order::Charge | Order::Pursue => split(by, facing).0 < -300,
                    Order::Retire => false,
                };
                if back {
                    eps.hit(k, Detector::SlotBack, (b as i64, 0), by.length(), || {
                        told(by, &same)
                    });
                }
            }
        }

        // 2. halted-body-walks.
        if let Some(old) = old {
            for (b, info) in scene.bodies.iter().enumerate() {
                if self.hold_run[b] <= samples(2) {
                    continue;
                }
                let men: Vec<usize> = men_of(info).filter(|&i| standing(s.posture[i])).collect();
                let movers: Vec<(usize, Vec2)> = men
                    .iter()
                    .map(|&i| (i, s.pos[i] - old.pos[i]))
                    .filter(|(_, d)| d.len_sq() > 600 * 600)
                    .collect();
                if movers.is_empty() || movers.len() * 2 < men.len() {
                    continue;
                }
                // The most of them within 30 degrees of one direction: within one arc of 60.
                let mut angles: Vec<f64> = movers
                    .iter()
                    .map(|(_, d)| (d.y as f64).atan2(d.x as f64))
                    .collect();
                angles.sort_by(f64::total_cmp);
                let m = angles.len();
                let at =
                    |j: usize| angles[j % m] + if j >= m { std::f64::consts::TAU } else { 0.0 };
                let arc = std::f64::consts::FRAC_PI_3 + 1e-9;
                let (mut best, mut j) = (0, 0);
                for (i, &from) in angles.iter().enumerate() {
                    j = j.max(i);
                    while j < i + m && at(j) - from <= arc {
                        j += 1;
                    }
                    best = best.max(j - i);
                }
                if best * 2 < men.len() {
                    continue;
                }
                let sum = movers.iter().fold(Vec2::ZERO, |a, (_, d)| a + *d);
                let mean = Vec2::new(sum.x / m as i32, sum.y / m as i32);
                eps.hit(k, Detector::HaltedBodyWalks, (b as i64, 0), mean.length(), || {
                    let facing = men.iter().find_map(|&i| s.slot[i]).map(|sv| sv.facing);
                    let (ahead, left) = facing.map_or((0, 0), |f| split(mean, f));
                    Hit {
                        men: Vec::new(),
                        body: Some(b as u32),
                        at: s.pos[movers[0].0],
                        kind: "",
                        note: format!(
                            "{best} of {} standing men moved over 600 mm in 2 s within 30 degrees \
                             of one direction; the movers' mean is {ahead} mm ahead and {left} mm \
                             left",
                            men.len()
                        ),
                    }
                });
            }
        }

        // 3. pinned-runner.
        if let Some(old) = old {
            for i in 0..n {
                if self.turned_run[i] <= samples(2) {
                    continue;
                }
                let d = (s.pos[i] - old.pos[i]).length();
                if d < 500 {
                    eps.hit(k, Detector::PinnedRunner, (i as i64, 0), 500 - d, || Hit {
                        men: vec![i as u32],
                        body: None,
                        at: s.pos[i],
                        kind: "",
                        note: format!("in flight, and {d} mm in 2 s"),
                    });
                }
            }
        }

        // 4 to 6: overlaps and crowds, from one grid of the men who are up.
        let mut grid: HashMap<(i32, i32), Vec<u32>> = HashMap::new();
        for i in (0..n).filter(|&i| up(i)) {
            grid.entry(cell(s.pos[i])).or_default().push(i as u32);
        }
        for (&(cx, cy), men) in &grid {
            if men.len() >= CROWD {
                eps.hit(
                    k,
                    Detector::Crowd,
                    (cx as i64, cy as i64),
                    men.len() as i64,
                    || Hit {
                        men: Vec::new(),
                        body: None,
                        at: Vec2::new(cx * CELL_MM + CELL_MM / 2, cy * CELL_MM + CELL_MM / 2),
                        kind: "",
                        note: format!("{} men up in one 2 m cell", men.len()),
                    },
                );
            }
            if men.len() >= OVER_PACKED {
                eps.hit(
                    k,
                    Detector::OverPacked,
                    (cx as i64, cy as i64),
                    men.len() as i64,
                    || Hit {
                        men: Vec::new(),
                        body: None,
                        at: Vec2::new(cx * CELL_MM + CELL_MM / 2, cy * CELL_MM + CELL_MM / 2),
                        kind: "",
                        note: format!("{} men up in one 2 m cell", men.len()),
                    },
                );
            }
            for &i in men {
                for (dx, dy) in (-1..=1).flat_map(|dx| (-1..=1).map(move |dy| (dx, dy))) {
                    let Some(others) = grid.get(&(cx + dx, cy + dy)) else {
                        continue;
                    };
                    for &j in others.iter().filter(|&&j| j > i) {
                        let (a, b) = (i as usize, j as usize);
                        let d2 = s.pos[a].dist_sq(s.pos[b]);
                        if d2 >= OVERLAP_MM * OVERLAP_MM {
                            continue;
                        }
                        let own = scene.side[a] == scene.side[b];
                        let (d, kind) = if own {
                            let running = [a, b].iter().any(|&m| s.posture[m] == Posture::Turned);
                            (
                                Detector::OverlapOwn,
                                if running {
                                    "one-in-flight"
                                } else {
                                    "both-standing"
                                },
                            )
                        } else {
                            (Detector::OverlapEnemies, "")
                        };
                        let dist = (s.pos[a] - s.pos[b]).length();
                        eps.hit(k, d, (i as i64, j as i64), OVERLAP_MM - dist, || Hit {
                            men: vec![i, j],
                            body: None,
                            at: s.pos[a],
                            kind,
                            note: format!("{dist} mm between their centres"),
                        });
                    }
                }
            }
        }

        // 7. jump; 10. flight-toward.
        if let Some(prev) = prev {
            for i in (0..n).filter(|&i| up(i) && prev.posture[i] != Posture::Down) {
                let step = s.pos[i] - prev.pos[i];
                let d = step.length();
                if d > 1_500 {
                    eps.hit(k, Detector::Jump, (i as i64, 0), d, || Hit {
                        men: vec![i as u32],
                        body: None,
                        at: prev.pos[i],
                        kind: "",
                        note: format!("{d} mm in one man step"),
                    });
                }
                let fleeing = s.posture[i] == Posture::Turned && prev.posture[i] == Posture::Turned;
                // 500 mm/s is 125 mm a step.
                if !fleeing || d <= 125 {
                    continue;
                }
                let nearest = (0..n)
                    .filter(|&j| scene.side[j] != scene.side[i] && standing(prev.posture[j]))
                    .map(|j| (prev.pos[j].dist_sq(prev.pos[i]), j))
                    .filter(|&(d2, _)| d2 <= 10_000 * 10_000)
                    .min();
                let Some((_, j)) = nearest else { continue };
                let to = prev.pos[j] - prev.pos[i];
                let dot = step.dot(to) as i128;
                // Within 45 degrees: the cosine is at least the root of a half.
                if dot > 0 && 2 * dot * dot >= step.len_sq() as i128 * to.len_sq() as i128 {
                    eps.hit(k, Detector::FlightToward, (i as i64, 0), d * 4, || Hit {
                        men: vec![i as u32, j as u32],
                        body: None,
                        at: prev.pos[i],
                        kind: "",
                        note: format!(
                            "in flight at {} mm/s toward the nearest standing enemy, {} mm off",
                            d * 4,
                            to.length()
                        ),
                    });
                }
            }
        }

        // 24. down-rises; 28. sudden-flight.
        if let Some(prev) = prev {
            for i in 0..n {
                if prev.posture[i] == Posture::Down && up(i) {
                    eps.hit(k, Detector::DownRises, (i as i64, 0), 0, || Hit {
                        men: vec![i as u32],
                        body: None,
                        at: s.pos[i],
                        kind: "",
                        note: format!("down at the last man step, and now {:?}", s.posture[i]),
                    });
                }
                if s.posture[i] != Posture::Turned || prev.posture[i] == Posture::Turned {
                    continue;
                }
                // Steady at one of the three samples before: less than 1 s ago.
                let ago = self
                    .hist
                    .iter()
                    .rev()
                    .take(3)
                    .position(|past| past.posture[i] == Posture::Steady);
                if let Some(ago) = ago {
                    eps.hit(k, Detector::SuddenFlight, (i as i64, 0), 0, || Hit {
                        men: vec![i as u32],
                        body: None,
                        at: s.pos[i],
                        kind: "",
                        note: format!(
                            "turned to flight {} ms after he was steady",
                            (ago as u32 + 1) * 250
                        ),
                    });
                }
            }
        }

        // 26. stalled.
        if self.slot_hist.len() == samples(10) as usize {
            let then = &self.slot_hist[0];
            for (b, info) in scene.bodies.iter().enumerate() {
                if self.led_run[b] <= samples(10) {
                    continue;
                }
                let moved: Vec<Vec2> = men_of(info)
                    .filter_map(|i| Some(s.slot[i]?.target - then[i]?))
                    .collect();
                if moved.is_empty() {
                    continue;
                }
                let median = |f: fn(&Vec2) -> i32| {
                    let mut c: Vec<i32> = moved.iter().map(f).collect();
                    c.sort_unstable();
                    c[c.len() / 2]
                };
                let by = Vec2::new(median(|v| v.x), median(|v| v.y));
                if by.len_sq() >= 500 * 500 {
                    continue;
                }
                let men: Vec<usize> = men_of(info).filter(|&i| standing(s.posture[i])).collect();
                if men.len() * 2 < info.count as usize {
                    continue;
                }
                let enemy_near = men.iter().any(|&i| {
                    (0..n).any(|j| {
                        scene.side[j] != info.side
                            && up(j)
                            && s.pos[j].dist_sq(s.pos[i]) <= 10_000 * 10_000
                    })
                });
                if enemy_near {
                    continue;
                }
                eps.hit(
                    k,
                    Detector::Stalled,
                    (b as i64, 0),
                    500 - by.length(),
                    || Hit {
                        men: Vec::new(),
                        body: Some(b as u32),
                        at: s.pos[men[0]],
                        kind: "",
                        note: format!(
                            "under the order {:?} for 10 s, {} of {} men standing, no enemy up \
                         within 10 m, and its slots {} mm from where they were",
                            s.state[b].order,
                            men.len(),
                            info.count,
                            by.length()
                        ),
                    },
                );
            }
        }

        // 8. down-moves.
        for i in 0..n {
            if up(i) {
                self.fell_at[i] = None;
                continue;
            }
            let lay = *self.fell_at[i].get_or_insert(s.pos[i]);
            let d = (s.pos[i] - lay).length();
            if d > 100 {
                eps.hit(k, Detector::DownMoves, (i as i64, 0), d, || Hit {
                    men: vec![i as u32],
                    body: None,
                    at: lay,
                    kind: "",
                    note: format!("down, and {d} mm from where he first lay"),
                });
            }
        }

        // 9. facing-in-place; 16. out-of-place; 18. gap; 20. far.
        for i in 0..n {
            let p = s.pos[i];
            if p.x.abs().max(p.y.abs()) > 5_000_000 {
                eps.hit(k, Detector::Far, (i as i64, 1), 0, || Hit {
                    men: vec![i as u32],
                    body: None,
                    at: p,
                    kind: "coordinate",
                    note: format!("at ({}, {}) mm", p.x, p.y),
                });
            }
            if !standing(s.posture[i]) {
                continue;
            }
            let out = (p - scene.centre).length();
            if out > 300_000 {
                eps.hit(k, Detector::Far, (i as i64, 0), out, || Hit {
                    men: vec![i as u32],
                    body: None,
                    at: p,
                    kind: "standing",
                    note: format!("standing {out} mm from the field's centre"),
                });
            }
            let Some(sv) = s.slot[i] else { continue };
            let off = (s.facing[i].diff(sv.facing) as i32).abs();
            if sv.in_place && off > Turn::QUARTER as i32 {
                eps.hit(
                    k,
                    Detector::FacingInPlace,
                    (i as i64, 0),
                    off as i64,
                    || Hit {
                        men: vec![i as u32],
                        body: None,
                        at: p,
                        kind: "",
                        note: format!(
                            "in his place, facing {} degrees off his slot's facing",
                            off * 360 / 65_536
                        ),
                    },
                );
            }
            if s.posture[i] == Posture::Steady && !sv.in_place {
                let d = (p - sv.target).length();
                eps.hit(k, Detector::OutOfPlace, (i as i64, 0), d, || Hit {
                    men: vec![i as u32],
                    body: None,
                    at: p,
                    kind: "",
                    note: format!("steady, {d} mm from his slot"),
                });
            }
            if sv.slots & nb::FRONT != 0 {
                continue;
            }
            for (bit, sign, kind) in [(nb::LEFT, 1i64, "left"), (nb::RIGHT, -1, "right")] {
                if sv.slots & bit == 0 || sv.neighbours & bit != 0 {
                    continue;
                }
                // Apart from the leader's slot during a succession: the one case in which a
                // man who is down is kept in his file.
                let b = scene.body[i].map(|b| b.0 as usize);
                let kept_for_leader = b.is_some_and(|b| {
                    let Some(l) = s.state[b].leader.map(|l| l.0 as usize) else {
                        return false;
                    };
                    let Some(sl) = s.slot[l] else { return false };
                    let to = sl.target - sv.target;
                    let reach = scene.bodies[b].file_mm as i64 + 100;
                    s.posture[l] == Posture::Down
                        && sl.slots & nb::FRONT == 0
                        && to.len_sq() <= reach * reach
                        && split(to, sv.facing).1 * sign > 0
                });
                if kept_for_leader {
                    continue;
                }
                eps.hit(k, Detector::Gap, (i as i64, bit as i64), 0, || Hit {
                    men: vec![i as u32],
                    body: b.map(|b| b as u32),
                    at: p,
                    kind,
                    note: format!("in the front rank, the slot on his {kind} exists and is empty"),
                });
            }
        }

        // 11 to 13: bouts; and the clock of 17.
        for e in s.events {
            let EventKind::Bout { attacker, .. } = e.kind else {
                continue;
            };
            let (a, d) = (attacker.0 as usize, e.man.0 as usize);
            self.last_bout[a] = Some(k);
            self.last_bout[d] = Some(k);
            let key = (a as i64, d as i64);
            let men = || vec![a as u32, d as u32];
            let reach = (s.pos[a] - s.pos[d]).length();
            if reach > 3_000 {
                eps.hit(k, Detector::BoutReach, key, reach, || Hit {
                    men: men(),
                    body: None,
                    at: s.pos[a],
                    kind: "",
                    note: format!("a bout between men {reach} mm apart"),
                });
            }
            let read = |i: usize| s.fought.get(i).copied().unwrap_or(s.posture[i]);
            let state = if scene.side[a] == scene.side[d] {
                Some("same-side")
            } else if read(d) == Posture::Down {
                Some("defender-down")
            } else if read(a) == Posture::Turned {
                Some("attacker-in-flight")
            } else if read(a) == Posture::Down {
                Some("attacker-down")
            } else {
                None
            };
            if let Some(kind) = state {
                eps.hit(k, Detector::BoutState, key, 0, || Hit {
                    men: men(),
                    body: None,
                    at: s.pos[a],
                    kind,
                    note: format!(
                        "a bout: the attacker was {:?}, the defender {:?}",
                        read(a),
                        read(d)
                    ),
                });
            }
            let line = s.pos[d] - s.pos[a];
            let len2 = line.len_sq();
            if len2 == 0 {
                continue;
            }
            let between = (0..n)
                .filter(|&p| p != a && p != d && read(p) != Posture::Down)
                .filter_map(|p| {
                    let to = s.pos[p] - s.pos[a];
                    let along = to.dot(line);
                    // In the middle three fifths of the line.
                    if along * 5 < len2 || along * 5 > len2 * 4 {
                        return None;
                    }
                    let across = to.cross(line) as i128;
                    (across * across < 250 * 250 * len2 as i128)
                        .then(|| ((across * across / len2 as i128) as i64, p))
                })
                .min();
            if let Some((off2, p)) = between {
                let off = (off2 as f64).sqrt() as i64;
                eps.hit(k, Detector::BoutThrough, key, 250 - off, || Hit {
                    men: vec![a as u32, d as u32, p as u32],
                    body: None,
                    at: s.pos[a],
                    kind: if scene.side[p] == scene.side[a] {
                        "past-a-comrade"
                    } else {
                        "past-an-enemy"
                    },
                    note: format!(
                        "a bout over {reach} mm with man {p} standing {off} mm from the line \
                         between the two"
                    ),
                });
            }
        }

        // 14. flicker.
        for e in s.events {
            if let EventKind::Posture { from, to } = e.kind {
                self.changes[e.man.0 as usize].push_back((e.tick, from, to));
            }
        }
        let now = k * TICKS;
        for (i, c) in self.changes.iter_mut().enumerate() {
            while c
                .front()
                .is_some_and(|&(t, _, _)| t + samples(5) * TICKS < now)
            {
                c.pop_front();
            }
            if c.len() >= 6 {
                let wavers = c
                    .iter()
                    .any(|&(_, from, to)| !(from <= Posture::Forward && to <= Posture::Forward));
                eps.hit(k, Detector::Flicker, (i as i64, 0), c.len() as i64, || {
                    Hit {
                        men: vec![i as u32],
                        body: None,
                        at: s.pos[i],
                        kind: if wavers {
                            "gives-way"
                        } else {
                            "steady-and-forward"
                        },
                        note: format!(
                            "{} changes of posture within 5 s: {}",
                            c.len(),
                            c.iter()
                                .map(|&(_, _, to)| format!("{to:?}"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    }
                });
            }
        }

        // 15. standard-front; 19. leaderless.
        for (b, info) in scene.bodies.iter().enumerate() {
            let st = s.state[b];
            if let Standard::Carried(c) = st.standard
                && standing(s.posture[c.0 as usize])
                && let Some(sv) = s.slot[c.0 as usize]
                && sv.slots & nb::FRONT == 0
            {
                let closer = info.closer == Some(c) && sv.slots == 0;
                eps.hit(k, Detector::StandardFront, (b as i64, 0), 0, || Hit {
                    men: vec![c.0],
                    body: Some(b as u32),
                    at: s.pos[c.0 as usize],
                    kind: if closer { "the-closer" } else { "front-rank" },
                    note: "the standard is carried by a man with no slot ahead of him".into(),
                });
            }
            if let (Some(l), Some(c)) = (st.leader, info.closer)
                && l != c
                && s.posture[l.0 as usize] == Posture::Down
                && s.posture[c.0 as usize] != Posture::Down
            {
                let fled = s.posture[c.0 as usize] == Posture::Turned;
                eps.hit(k, Detector::Leaderless, (b as i64, 0), 0, || Hit {
                    men: vec![l.0, c.0],
                    body: Some(b as u32),
                    at: s.pos[c.0 as usize],
                    kind: if fled {
                        "closer-in-flight"
                    } else {
                        "closer-standing"
                    },
                    note: "the leader is down and the closer lives".into(),
                });
            }
        }

        // 17. forward-idle.
        for i in 0..n {
            if s.posture[i] != Posture::Forward {
                self.forward_from[i] = None;
                continue;
            }
            let since = *self.forward_from[i].get_or_insert(k);
            let idle_from = self.last_bout[i].map_or(since, |b| b.max(since));
            if k - idle_from >= samples(20) {
                eps.hit(
                    k,
                    Detector::ForwardIdle,
                    (i as i64, 0),
                    (k - idle_from) as i64,
                    || Hit {
                        men: vec![i as u32],
                        body: None,
                        at: s.pos[i],
                        kind: "",
                        note: "forward for 20 s with no bout of his".into(),
                    },
                );
            }
        }

        for i in 0..n {
            self.prev_slot[i] = s.slot[i].map(|sv| sv.target);
        }
        self.slot_hist.push_back(self.prev_slot.clone());
        if self.slot_hist.len() > samples(10) as usize {
            self.slot_hist.pop_front();
        }
        self.prev_state = s.state.to_vec();
        self.hist.push_back(Past {
            pos: s.pos.to_vec(),
            posture: s.posture.to_vec(),
        });
        if self.hist.len() > samples(2) as usize + 1 {
            self.hist.pop_front();
        }
        self.eps.sweep(Some(k));
        self.k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cannae_core::{BoutOutcome, ManId, Tie};

    /// A hand-made scene: men who do only what a test moves them to do.
    struct World {
        pos: Vec<Vec2>,
        facing: Vec<Turn>,
        posture: Vec<Posture>,
        slot: Vec<Option<SlotView>>,
        state: Vec<BodyState>,
        events: Vec<Event>,
        fought: Vec<Posture>,
        det: Detectors,
        k: u32,
    }

    const A: Side = Side::A;
    const B: Side = Side::B;

    /// Men at `pos` of the sides given. Each run of men of one side is one body, holding, its
    /// standard with its first man, with no leader and no closer.
    fn world(men: &[(Side, i32, i32)]) -> World {
        let pos: Vec<Vec2> = men.iter().map(|&(_, x, y)| Vec2::new(x, y)).collect();
        let side: Vec<Side> = men.iter().map(|m| m.0).collect();
        let mut bodies: Vec<BodyInfo> = Vec::new();
        let mut body = Vec::new();
        for (i, &s) in side.iter().enumerate() {
            if bodies.last().is_none_or(|b| b.side != s) {
                bodies.push(BodyInfo {
                    name: format!("body {}", bodies.len()),
                    side: s,
                    first: i as u32,
                    count: 0,
                    file_mm: 1_800,
                    rank_mm: 1_800,
                    closer: None,
                    tie: Tie::DRILLED,
                });
            }
            bodies.last_mut().unwrap().count += 1;
            body.push(Some(BodyId(bodies.len() as u32 - 1)));
        }
        let state = bodies
            .iter()
            .map(|b| BodyState {
                standard: Standard::Carried(ManId(b.first)),
                leader: None,
                order: Order::Hold,
                order_tick: 0,
            })
            .collect();
        let n = pos.len();
        World {
            det: Detectors::new(Scene::new(&side, &body, &bodies, &pos)),
            pos,
            facing: vec![Turn(0); n],
            posture: vec![Posture::Steady; n],
            slot: vec![None; n],
            state,
            events: Vec::new(),
            fought: Vec::new(),
            k: 0,
        }
    }

    fn slot(x: i32, y: i32, in_place: bool, slots: u8, neighbours: u8) -> Option<SlotView> {
        Some(SlotView {
            target: Vec2::new(x, y),
            facing: Turn(0),
            in_place,
            slots,
            neighbours,
        })
    }

    const ALL: u8 = nb::LEFT | nb::RIGHT | nb::FRONT | nb::BEHIND;

    impl World {
        /// Takes `count` samples of the scene as it stands.
        fn take(&mut self, count: u32) {
            for _ in 0..count {
                self.det.step(&Sample {
                    pos: &self.pos,
                    facing: &self.facing,
                    posture: &self.posture,
                    slot: &self.slot,
                    state: &self.state,
                    events: &self.events,
                    fought: &self.fought,
                });
                self.events.clear();
                self.k += 1;
            }
        }

        /// Moves man `i` by `(dx, dy)` and takes a sample, `count` times.
        fn walk(&mut self, i: usize, dx: i32, dy: i32, count: u32) {
            for _ in 0..count {
                self.pos[i] += Vec2::new(dx, dy);
                self.take(1);
            }
        }

        fn event(&mut self, man: usize, kind: EventKind) {
            self.events.push(Event {
                tick: self.k * TICKS,
                man: ManId(man as u32),
                pos: self.pos[man],
                kind,
                causes: Default::default(),
            });
        }

        fn bout(&mut self, attacker: usize, defender: usize) {
            self.fought = self.posture.clone();
            self.event(
                defender,
                EventKind::Bout {
                    attacker: ManId(attacker as u32),
                    defender: BoutOutcome::Nothing,
                    attacker_took: BoutOutcome::Nothing,
                },
            );
        }

        fn flags(self, d: Detector) -> Vec<Flag> {
            let mut f = self.det.finish();
            f.retain(|f| f.detector == d);
            f
        }
    }

    /// Four men of one body in a square, each on his slot.
    fn square() -> World {
        let mut w = world(&[(A, 0, 0), (A, 0, 1_800), (A, 1_800, 0), (A, 1_800, 1_800)]);
        for i in 0..4 {
            w.slot[i] = slot(w.pos[i].x, w.pos[i].y, true, ALL, ALL);
        }
        w
    }

    fn shift_slots(w: &mut World, men: std::ops::Range<usize>, dx: i32) {
        for i in men {
            let mut sv = w.slot[i].unwrap();
            sv.target += Vec2::new(dx, 0);
            w.slot[i] = Some(sv);
        }
    }

    #[test]
    fn slot_jump_flags_a_frame_re_formed_a_rank_back() {
        let mut w = square();
        w.take(2);
        shift_slots(&mut w, 0..4, -1_800);
        w.take(2);
        let f = w.flags(Detector::SlotJump);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].last, f[0].kind), (2, 2, "other"));
        assert_eq!(f[0].worst, 1_800);
    }

    #[test]
    fn slot_jump_names_a_move_by_the_lead_and_passes_a_step() {
        let mut w = square();
        w.take(1);
        shift_slots(&mut w, 0..4, Tie::DRILLED.lead_mm);
        w.take(1);
        // A running step, and then a jump of half the body only.
        shift_slots(&mut w, 0..4, 1_200);
        w.take(1);
        shift_slots(&mut w, 0..1, 5_000);
        w.take(1);
        let f = w.flags(Detector::SlotJump);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].kind), (1, "by-the-lead"));
    }

    #[test]
    fn slot_jump_needs_the_slots_to_move_as_one() {
        let mut w = square();
        w.take(1);
        // Every slot moves far, each its own way.
        for (i, dx) in [2_000, 3_000, -2_000, 4_000].into_iter().enumerate() {
            shift_slots(&mut w, i..i + 1, dx);
        }
        w.take(1);
        assert_eq!(w.flags(Detector::SlotJump), []);
    }

    fn drift(order: Order, per_step: i32, spread: bool) -> Vec<Flag> {
        let mut w = square();
        w.state[0].order = order;
        w.take(1);
        for _ in 0..8 {
            for i in 0..4 {
                let (dx, dy) = match (spread, i) {
                    (false, _) | (true, 0) => (per_step, 0),
                    (true, 1) => (0, per_step),
                    (true, 2) => (-per_step, 0),
                    _ => (0, -per_step),
                };
                w.pos[i] += Vec2::new(dx, dy);
            }
            w.take(1);
        }
        w.flags(Detector::HaltedBodyWalks)
    }

    #[test]
    fn halted_body_walks_flags_a_holding_body_that_drifts() {
        // 76 mm a step is 608 mm in 2 s; 75 mm is 600, which is not more than 600.
        let f = drift(Order::Hold, 76, false);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].body), (8, Some(0)));
        assert_eq!(drift(Order::Hold, 75, false), []);
        assert_eq!(drift(Order::Advance, 76, false), []);
        assert_eq!(drift(Order::Hold, 76, true), []);
    }

    #[test]
    fn halted_body_walks_counts_half_the_standing_men() {
        let mut w = square();
        w.posture[3] = Posture::Down;
        w.take(1);
        // Two of three standing men move: more than half.
        for _ in 0..8 {
            w.pos[0] += Vec2::new(100, 0);
            w.pos[1] += Vec2::new(100, 10);
            w.take(1);
        }
        assert_eq!(w.flags(Detector::HaltedBodyWalks).len(), 1);
        let mut w = square();
        w.take(1);
        // One of four.
        w.walk(0, 100, 0, 8);
        assert_eq!(w.flags(Detector::HaltedBodyWalks), []);
    }

    #[test]
    fn pinned_runner_flags_a_man_in_flight_who_stays() {
        let mut w = world(&[(A, 0, 0)]);
        w.posture[0] = Posture::Turned;
        // 62 mm a step is 496 mm in 2 s.
        w.walk(0, 62, 0, 20);
        let f = w.flags(Detector::PinnedRunner);
        assert_eq!(f.len(), 1);
        // In flight from sample 0; the first 2 s end at sample 8.
        assert_eq!(
            (f[0].first, f[0].last, f[0].men.as_slice()),
            (8, 19, &[0][..])
        );
        assert_eq!(f[0].first - Detector::PinnedRunner.lead_in(), 0);
    }

    #[test]
    fn pinned_runner_passes_a_runner_who_gets_away_and_a_man_who_stands() {
        let mut w = world(&[(A, 0, 0), (A, 5_000, 0)]);
        w.posture[0] = Posture::Turned;
        // 63 mm a step is 504 mm in 2 s. The other man stands steady and still.
        w.walk(0, 63, 0, 20);
        assert_eq!(w.flags(Detector::PinnedRunner), []);
        // Still for eight samples of flight only: 2 s of flight is nine.
        let mut w = world(&[(A, 0, 0)]);
        w.take(4);
        w.posture[0] = Posture::Turned;
        w.take(8);
        w.posture[0] = Posture::Steady;
        w.take(4);
        assert_eq!(w.flags(Detector::PinnedRunner), []);
    }

    #[test]
    fn overlaps_are_told_by_side_and_end_at_300_mm() {
        let mut w = world(&[(A, 0, 0), (B, 299, 0), (A, 10_000, 0), (A, 10_000, 299)]);
        w.take(3);
        w.pos[1] = Vec2::new(300, 0);
        w.pos[3] = Vec2::new(10_000, 300);
        w.take(2);
        let det = w.det.finish();
        let of = |d| det.iter().filter(|f| f.detector == d).collect::<Vec<_>>();
        let (enemies, own) = (of(Detector::OverlapEnemies), of(Detector::OverlapOwn));
        assert_eq!(enemies.len(), 1);
        assert_eq!(
            (enemies[0].first, enemies[0].last, &enemies[0].men[..]),
            (0, 2, &[0, 1][..])
        );
        assert_eq!(own.len(), 1);
        assert_eq!(
            (own[0].kind, &own[0].men[..]),
            ("both-standing", &[2, 3][..])
        );
    }

    #[test]
    fn overlaps_are_found_across_cells_and_never_with_the_fallen() {
        // Either side of a cell's edge, and of the origin.
        let mut w = world(&[(A, -100, -100), (B, 100, 100), (A, 6_000, 0), (B, 6_100, 0)]);
        w.posture[3] = Posture::Down;
        w.take(1);
        let f = w.flags(Detector::OverlapEnemies);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].men, [0, 1]);
    }

    #[test]
    fn crowd_flags_24_men_up_in_a_cell() {
        let men = |count: i32| {
            let m: Vec<_> = (0..count)
                .map(|i| (A, 400 * (i % 5), 400 * (i / 5)))
                .collect();
            let mut w = world(&m);
            w.take(2);
            w.flags(Detector::Crowd)
        };
        let f = men(24);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].worst, f[0].at), (24, Vec2::new(1_000, 1_000)));
        assert_eq!(men(23), []);
    }

    #[test]
    fn jump_flags_more_than_a_run() {
        let mut w = world(&[(A, 0, 0), (A, 0, 5_000)]);
        w.take(1);
        w.walk(0, 1_501, 0, 1);
        w.walk(1, 1_500, 0, 1);
        let f = w.flags(Detector::Jump);
        assert_eq!(f.len(), 1);
        assert_eq!(
            (f[0].first, f[0].worst, &f[0].men[..]),
            (1, 1_501, &[0][..])
        );
    }

    #[test]
    fn down_moves_flags_a_fallen_man_who_is_moved() {
        let mut w = world(&[(A, 0, 0), (A, 0, 5_000)]);
        w.take(1);
        w.posture = vec![Posture::Down; 2];
        // Where he lies at the first sample at which he is down is where he fell.
        w.walk(0, 3_000, 0, 1);
        w.walk(0, 101, 0, 1);
        w.walk(1, 100, 0, 1);
        w.take(2);
        let f = w.flags(Detector::DownMoves);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].worst, &f[0].men[..]), (2, 101, &[0][..]));
    }

    fn turned_about(off: u16, count: u32) -> Vec<Flag> {
        let mut w = square();
        w.take(2);
        w.facing[1] = Turn(off);
        w.take(count);
        w.facing[1] = Turn(0);
        w.take(2);
        w.flags(Detector::FacingInPlace)
    }

    #[test]
    fn facing_in_place_flags_a_man_in_line_who_faces_away_for_a_second() {
        let f = turned_about(Turn::QUARTER + 1, 5);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].last), (2, 6));
        assert_eq!(turned_about(Turn::QUARTER + 1, 4), []);
        assert_eq!(turned_about(Turn::QUARTER, 20), []);
        // The other way round the circle.
        assert_eq!(turned_about(u16::MAX - Turn::QUARTER, 5).len(), 1);
    }

    fn flee(dx: i32, dy: i32, steps: u32, enemy_at: i32) -> Vec<Flag> {
        let mut w = world(&[(A, 0, 0), (B, enemy_at, 0)]);
        w.posture[0] = Posture::Turned;
        w.take(1);
        w.walk(0, dx, dy, steps);
        w.posture[0] = Posture::Steady;
        w.take(1);
        w.flags(Detector::FlightToward)
    }

    #[test]
    fn flight_toward_flags_a_runner_who_makes_for_the_enemy() {
        let f = flee(126, 0, 4, 9_000);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].last, &f[0].men[..]), (1, 4, &[0, 1][..]));
        // Three steps are under a second; 125 mm a step is not faster than 500 mm/s.
        assert_eq!(flee(126, 0, 3, 9_000), []);
        assert_eq!(flee(125, 0, 8, 9_000), []);
        // Away from him; across him, more than 45 degrees off; and with no enemy within 10 m.
        assert_eq!(flee(-200, 0, 8, 9_000), []);
        assert_eq!(flee(100, 110, 8, 9_000), []);
        assert_eq!(flee(110, 100, 4, 9_000).len(), 1);
        assert_eq!(flee(126, 0, 4, 10_200), []);
    }

    #[test]
    fn flight_toward_reads_an_enemy_who_stands_only() {
        let mut w = world(&[(A, 0, 0), (B, 5_000, 0)]);
        w.posture = vec![Posture::Turned; 2];
        w.take(1);
        w.walk(0, 200, 0, 6);
        assert_eq!(w.flags(Detector::FlightToward), []);
    }

    #[test]
    fn bout_reach_flags_a_bout_out_of_reach() {
        let mut w = world(&[(A, 0, 0), (B, 3_001, 0), (A, 0, 9_000), (B, 3_000, 9_000)]);
        w.take(1);
        w.bout(0, 1);
        w.bout(3, 2);
        w.take(1);
        let f = w.flags(Detector::BoutReach);
        assert_eq!(f.len(), 1);
        assert_eq!(
            (f[0].first, f[0].worst, &f[0].men[..]),
            (1, 3_001, &[0, 1][..])
        );
    }

    #[test]
    fn bout_state_flags_bouts_that_should_not_be_drawn() {
        let mut w = world(&[
            (A, 0, 0),
            (B, 1_000, 0),
            (A, 0, 9_000),
            (B, 1_000, 9_000),
            (A, 0, 20_000),
            (A, 900, 20_000),
        ]);
        w.posture[0] = Posture::Turned;
        w.posture[3] = Posture::Down;
        w.take(1);
        w.bout(0, 1);
        w.bout(2, 3);
        w.bout(4, 5);
        // A sound bout, and one against a man in flight, which may be drawn.
        w.bout(1, 2);
        w.bout(1, 0);
        w.take(1);
        let kinds: Vec<_> = w
            .flags(Detector::BoutState)
            .iter()
            .map(|f| (f.men[0], f.kind))
            .collect();
        assert_eq!(
            kinds,
            [
                (0, "attacker-in-flight"),
                (2, "defender-down"),
                (4, "same-side")
            ]
        );
    }

    #[test]
    fn bout_state_reads_what_the_bout_phase_read() {
        let mut w = world(&[(A, 0, 0), (B, 1_000, 0)]);
        w.take(1);
        w.bout(0, 1);
        // The bout brought him down: he was standing when it was drawn.
        w.posture[1] = Posture::Down;
        w.take(1);
        assert_eq!(w.flags(Detector::BoutState), []);
    }

    fn strike_past(x: i32, y: i32, third: Posture) -> Vec<Flag> {
        let mut w = world(&[(A, 0, 0), (B, 2_000, 0), (A, x, y)]);
        w.posture[2] = third;
        w.take(1);
        w.bout(0, 1);
        w.take(1);
        w.flags(Detector::BoutThrough)
    }

    #[test]
    fn bout_through_flags_a_strike_past_a_man_on_the_line() {
        let f = strike_past(1_000, 249, Posture::Steady);
        assert_eq!(f.len(), 1);
        assert_eq!(
            (f[0].kind, &f[0].men[..]),
            ("past-a-comrade", &[0, 1, 2][..])
        );
        assert_eq!(strike_past(1_000, 250, Posture::Steady), []);
        // In the middle three fifths: from 400 to 1,600 mm of 2,000.
        assert_eq!(strike_past(400, 0, Posture::Steady).len(), 1);
        assert_eq!(strike_past(399, 0, Posture::Steady), []);
        assert_eq!(strike_past(1_601, 0, Posture::Steady), []);
        assert_eq!(strike_past(1_000, 0, Posture::Down), []);
    }

    fn waver(changes: u32, every: u32) -> Vec<Flag> {
        let mut w = world(&[(A, 0, 0)]);
        w.take(1);
        for c in 0..changes {
            let (from, to) = if c % 2 == 0 {
                (Posture::Steady, Posture::Back)
            } else {
                (Posture::Back, Posture::Steady)
            };
            w.event(0, EventKind::Posture { from, to });
            w.take(every);
        }
        w.take(30);
        w.flags(Detector::Flicker)
    }

    #[test]
    fn flicker_flags_six_changes_within_five_seconds() {
        // Six changes 1 s apart span 5 s.
        let f = waver(6, 4);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].kind, f[0].worst), ("gives-way", 6));
        assert_eq!(waver(5, 1), []);
        // Six changes 1.25 s apart span 6.25 s.
        assert_eq!(waver(6, 5), []);
    }

    #[test]
    fn flicker_names_the_pulse_of_the_fight() {
        let mut w = world(&[(A, 0, 0)]);
        w.take(1);
        for c in 0..8 {
            let (from, to) = if c % 2 == 0 {
                (Posture::Steady, Posture::Forward)
            } else {
                (Posture::Forward, Posture::Steady)
            };
            w.event(0, EventKind::Posture { from, to });
            w.take(1);
        }
        let f = w.flags(Detector::Flicker);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].kind, "steady-and-forward");
    }

    #[test]
    fn standard_front_flags_a_standard_in_the_front_rank() {
        let mut w = square();
        w.take(2);
        w.slot[0] = slot(0, 0, true, ALL & !nb::FRONT, ALL & !nb::FRONT);
        w.take(3);
        // In flight, his body is not formed.
        w.posture[0] = Posture::Turned;
        w.take(2);
        let f = w.flags(Detector::StandardFront);
        assert_eq!(f.len(), 1);
        assert_eq!(
            (f[0].first, f[0].last, f[0].kind, f[0].body),
            (2, 4, "front-rank", Some(0))
        );
    }

    fn adrift(count: u32, posture: Posture) -> Vec<Flag> {
        let mut w = square();
        w.take(2);
        w.slot[2] = slot(9_000, 0, false, ALL, ALL);
        w.posture[2] = posture;
        w.take(count);
        w.slot[2] = slot(1_800, 0, true, ALL, ALL);
        w.take(2);
        w.flags(Detector::OutOfPlace)
    }

    #[test]
    fn out_of_place_flags_a_steady_man_ten_seconds_off_his_slot() {
        let f = adrift(41, Posture::Steady);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].last, f[0].worst), (2, 42, 7_200));
        assert_eq!(adrift(40, Posture::Steady), []);
        assert_eq!(adrift(60, Posture::Back), []);
    }

    #[test]
    fn forward_idle_flags_twenty_seconds_forward_without_a_bout() {
        let mut w = world(&[(A, 0, 0), (B, 1_000, 0), (A, 0, 9_000)]);
        w.take(1);
        w.posture[0] = Posture::Forward;
        w.posture[2] = Posture::Forward;
        // Man 2 has a bout 10 s in, and so is idle for 15 s only.
        w.take(40);
        w.bout(2, 1);
        w.take(41);
        w.posture[0] = Posture::Steady;
        w.take(19);
        w.posture[2] = Posture::Steady;
        w.take(1);
        let f = w.flags(Detector::ForwardIdle);
        assert_eq!(f.len(), 1);
        // Forward from sample 1: 20 s are up at sample 81.
        assert_eq!((f[0].first, f[0].last, &f[0].men[..]), (81, 81, &[0][..]));
    }

    #[test]
    fn forward_idle_counts_from_a_mans_last_bout() {
        let mut w = world(&[(A, 0, 0), (B, 1_000, 0)]);
        w.posture[0] = Posture::Forward;
        w.take(10);
        w.bout(1, 0);
        w.take(81);
        let f = w.flags(Detector::ForwardIdle);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].first, 90);
    }

    fn gap(count: u32, leader_down: bool) -> Vec<Flag> {
        // Man 0 in the front rank; the slot on his left is man 1's, at +y.
        let mut w = square();
        w.take(2);
        w.slot[0] = slot(0, 0, true, nb::LEFT | nb::BEHIND, nb::BEHIND);
        w.slot[1] = slot(
            0,
            1_800,
            true,
            nb::RIGHT | nb::BEHIND,
            nb::RIGHT | nb::BEHIND,
        );
        w.posture[1] = Posture::Down;
        if leader_down {
            w.state[0].leader = Some(ManId(1));
        }
        w.take(count);
        w.slot[0] = slot(0, 0, true, nb::LEFT | nb::BEHIND, nb::LEFT | nb::BEHIND);
        w.take(2);
        w.flags(Detector::Gap)
    }

    #[test]
    fn gap_flags_a_front_rank_slot_left_empty() {
        let f = gap(42, false);
        assert_eq!(f.len(), 1);
        assert_eq!(
            (f[0].first, f[0].last, f[0].kind, &f[0].men[..]),
            (2, 43, "left", &[0][..])
        );
        // Ten seconds is not more than ten.
        assert_eq!(gap(41, false), []);
        // The leader's slot, kept for the closer who succeeds him.
        assert_eq!(gap(60, true), []);
    }

    #[test]
    fn gap_keeps_only_the_leaders_own_slot_apart() {
        // The leader is down on man 0's left; the empty slot is on his right.
        let mut w = square();
        w.slot[0] = slot(0, 0, true, nb::LEFT | nb::RIGHT, nb::LEFT);
        w.slot[1] = slot(0, 1_800, true, nb::RIGHT, nb::RIGHT);
        w.posture[1] = Posture::Down;
        w.state[0].leader = Some(ManId(1));
        w.take(50);
        let f = w.flags(Detector::Gap);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].kind, "right");
    }

    fn headless(count: u32, closer: Posture) -> Vec<Flag> {
        let mut w = square();
        w.det.scene.bodies[0].closer = Some(ManId(3));
        w.state[0].leader = Some(ManId(2));
        w.take(2);
        w.posture[2] = Posture::Down;
        w.posture[3] = closer;
        w.take(count);
        // The closer succeeds him.
        w.state[0].leader = Some(ManId(3));
        w.take(2);
        w.flags(Detector::Leaderless)
    }

    #[test]
    fn leaderless_flags_a_body_whose_closer_does_not_succeed() {
        let f = headless(82, Posture::Steady);
        assert_eq!(f.len(), 1);
        assert_eq!(
            (f[0].first, f[0].last, f[0].kind),
            (2, 83, "closer-standing")
        );
        assert_eq!(headless(81, Posture::Steady), []);
        assert_eq!(headless(90, Posture::Turned)[0].kind, "closer-in-flight");
        assert_eq!(headless(90, Posture::Down), []);
    }

    #[test]
    fn far_flags_a_man_who_walked_off_and_a_number_gone_wrong() {
        // The centre is the mean of the four at the start: the origin.
        let mut w = world(&[(A, -1_000, 0), (A, 1_000, 0), (B, 0, -1_000), (B, 0, 1_000)]);
        w.take(1);
        w.pos[0] = Vec2::new(300_001, 0);
        w.pos[1] = Vec2::new(300_000, 0);
        w.pos[2] = Vec2::new(0, -5_000_001);
        w.posture[2] = Posture::Down;
        w.pos[3] = Vec2::new(400_000, 0);
        w.posture[3] = Posture::Turned;
        w.take(1);
        let f: Vec<_> = w
            .flags(Detector::Far)
            .iter()
            .map(|f| (f.men[0], f.kind))
            .collect();
        assert_eq!(f, [(0, "standing"), (2, "coordinate")]);
    }

    #[test]
    fn an_episode_is_one_flag_and_a_break_makes_two() {
        let mut w = world(&[(A, 0, 0), (B, 100, 0)]);
        w.take(3);
        w.pos[1] = Vec2::new(5_000, 0);
        w.take(1);
        w.pos[1] = Vec2::new(100, 0);
        w.take(2);
        let f = w.flags(Detector::OverlapEnemies);
        assert_eq!(
            f.iter().map(|f| (f.first, f.last)).collect::<Vec<_>>(),
            [(0, 2), (4, 5)]
        );
    }

    #[test]
    fn slot_twice_flags_two_men_up_in_one_slot() {
        let mut w = square();
        w.take(1);
        w.slot[1] = w.slot[0];
        w.take(2);
        // A fallen man still kept in his file holds no slot against a man who is up.
        w.posture[1] = Posture::Down;
        w.take(2);
        let f = w.flags(Detector::SlotTwice);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].last, &f[0].men[..]), (1, 2, &[0, 1][..]));
    }

    #[test]
    fn down_rises_flags_a_fallen_man_who_is_up_again() {
        let mut w = world(&[(A, 0, 0), (A, 0, 5_000)]);
        w.posture[0] = Posture::Down;
        w.take(3);
        w.posture[0] = Posture::Steady;
        w.take(2);
        let f = w.flags(Detector::DownRises);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].last, &f[0].men[..]), (3, 3, &[0][..]));
    }

    fn step_back(order: Order, dx: i32) -> Vec<Flag> {
        let mut w = square();
        w.state[0].order = order;
        w.take(2);
        shift_slots(&mut w, 0..4, dx);
        w.take(2);
        w.flags(Detector::SlotBack)
    }

    #[test]
    fn slot_back_flags_a_frame_that_shifts_halted_or_gives_ground_advancing() {
        let f = step_back(Order::Hold, 301);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].kind, f[0].worst), (2, "other", 301));
        assert_eq!(step_back(Order::Hold, -301).len(), 1);
        assert_eq!(step_back(Order::Hold, 300), []);
        // The men face +x. Under an order to go forward only a move against the facing counts.
        assert_eq!(step_back(Order::Advance, -301).len(), 1);
        assert_eq!(step_back(Order::Advance, -300), []);
        assert_eq!(step_back(Order::Advance, 900), []);
        assert_eq!(step_back(Order::Charge, -400).len(), 1);
        assert_eq!(step_back(Order::Pursue, -400).len(), 1);
        assert_eq!(step_back(Order::Retire, -900), []);
        // The lead going off as an advancing body halts.
        let halt = step_back(Order::Advance, -Tie::DRILLED.lead_mm);
        assert_eq!(halt[0].kind, "by-the-lead");
    }

    #[test]
    fn slot_back_counts_exactly_half_the_slots_as_half() {
        for dx in [-900, 900] {
            let mut w = square();
            w.take(2);
            shift_slots(&mut w, 0..2, dx);
            w.take(2);
            assert_eq!(w.flags(Detector::SlotBack).len(), 1, "{dx}");
        }
        let mut w = square();
        w.take(2);
        shift_slots(&mut w, 0..1, -900);
        w.take(2);
        assert_eq!(w.flags(Detector::SlotBack), []);
    }

    fn stall(order: Order, creep: i32, enemy_x: i32, down: usize) -> Vec<Flag> {
        let mut w = world(&[
            (A, 0, 0),
            (A, 0, 1_800),
            (A, 1_800, 0),
            (A, 1_800, 1_800),
            (B, enemy_x, 0),
        ]);
        for i in 0..4 {
            w.slot[i] = slot(w.pos[i].x, w.pos[i].y, true, ALL, ALL);
        }
        w.state[0].order = order;
        for i in 0..down {
            w.posture[i] = Posture::Down;
        }
        for _ in 0..41 {
            shift_slots(&mut w, 0..4, creep);
            w.take(1);
        }
        w.flags(Detector::Stalled)
    }

    #[test]
    fn stalled_flags_a_led_body_that_does_not_move() {
        // 12 mm a step is 480 mm in the 40 steps of 10 s; 13 mm is 520.
        let f = stall(Order::Advance, 12, 100_000, 0);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].body), (40, Some(0)));
        assert_eq!(f[0].first - Detector::Stalled.lead_in(), 0);
        assert_eq!(stall(Order::Advance, 13, 100_000, 0), []);
        assert_eq!(stall(Order::Hold, 0, 100_000, 0), []);
        assert_eq!(stall(Order::Retire, 0, 100_000, 0), []);
        // An enemy who is up 9.2 m from its nearest men, and one 10.1 m from them.
        assert_eq!(stall(Order::Advance, 0, 11_000, 0), []);
        assert_eq!(stall(Order::Advance, 0, 11_900, 0).len(), 1);
        // One of its four men standing; and two, which is half.
        assert_eq!(stall(Order::Advance, 0, 100_000, 3), []);
        assert_eq!(stall(Order::Charge, 0, 100_000, 2).len(), 1);
    }

    #[test]
    fn over_packed_flags_40_men_up_in_a_cell() {
        let men = |count: i32| {
            let m: Vec<_> = (0..count)
                .map(|i| (A, 300 * (i % 6), 300 * (i / 6)))
                .collect();
            let mut w = world(&m);
            w.take(1);
            w.flags(Detector::OverPacked)
        };
        assert_eq!(men(40).len(), 1);
        assert_eq!(men(39), []);
    }

    fn bolts(back_for: u32) -> Vec<Flag> {
        let mut w = world(&[(A, 0, 0)]);
        w.take(2);
        w.posture[0] = Posture::Back;
        w.take(back_for);
        w.posture[0] = Posture::Turned;
        w.take(3);
        w.flags(Detector::SuddenFlight)
    }

    #[test]
    fn sudden_flight_flags_a_man_in_flight_within_a_second_of_steady() {
        // Steady at sample 1, giving ground at 2 and 3, in flight at 4: 750 ms.
        let f = bolts(2);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].first, f[0].last), (4, 4));
        assert_eq!(bolts(0).len(), 1);
        // Three samples of giving ground: he was steady 1 s before, which is not less.
        assert_eq!(bolts(3), []);
    }

    #[test]
    fn every_detector_has_its_name_and_number() {
        // 21 and 22 are the dormancy checks.
        let numbers: Vec<usize> = Detector::ALL.iter().map(|d| *d as usize).collect();
        assert_eq!(numbers, (1..=20).chain(23..=28).collect::<Vec<_>>());
        let mut names: Vec<&str> = Detector::ALL.iter().map(|d| d.name()).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), Detector::ALL.len());
    }
}
