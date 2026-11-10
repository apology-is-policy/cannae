//! The passage study (S0.8): one scripted passage, run on any substrate.
//!
//! Two passages from [rival 06](../../../docs/rival/06-experiment.md) §2. A maniple falls back
//! through an interval in the line behind it while the enemy follows it up; a routed crowd runs
//! for a gate in a camp's rampart. Every substrate gets the same bodies and the same steering at
//! every man step, so what differs between runs is the physics alone. There is no man model.
//! The steering is the least a person does: head for the mouth of the gap, then through it, then
//! on. A man who aims past the mouth from beside it walks into the frame
//! (docs/slice0/passage-calibration.md).
//!
//! Both mouths are at x = 0, with the way through running toward -x.
#![allow(clippy::float_arithmetic)]

use crate::runner::apply;
use cannae_core::schedule::{BASE_TICK, MAN_EVERY, is_man_step};
use cannae_core::{
    Bodies, Cause, Event, EventKind, Observed, Posture, Rng, Side, Substrate, Target, Turn, Vec2,
    Wall,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A maniple falls back through the interval between two maniples of the line behind it.
    Interval,
    /// A routed crowd runs for a gate in a camp's rampart.
    Gate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Case {
    pub kind: Kind,
    /// The clear width of the gap, mm: between the walls of a gate, or between the shoulders of
    /// the innermost files either side of an interval.
    pub width_mm: i32,
    /// Interval: the enemy follows the maniple up and leans on it. Gate: the crowd runs and
    /// shoves instead of walking.
    pub pressed: bool,
    /// How long to run at most, s. The run stops early once every passer is through.
    pub seconds: u32,
}

/// The parts men play.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Part {
    /// Goes through the gap.
    Passer,
    /// Holds his slot beside the gap.
    Holder,
    /// Follows the passers up, and stops short of the line.
    Enemy,
}

/// How far men keep from a gap's edge when they aim at its mouth, mm: about a body's radius
/// and a hand.
const EDGE_MM: i32 = 400;
/// How far in front of the line the enemy stops, mm.
const ENEMY_STOP_MM: i32 = 1_500;
/// The depth of a gate's passage through the rampart, mm.
const GATE_DEPTH_MM: i32 = 3_000;
/// The rampart runs this far either side of a gate, mm.
const RAMPART_MM: i32 = 60_000;
const WALK: i32 = 1_300;
const RUN: i32 = 4_000;

/// The maniples' shape: 120 men, 12 files of 10 ranks, a metre apart. Frontage and intervals
/// are parameters of the experiment (STATUS, "Contested mechanics"); this is one point of them.
const FILES: i32 = 12;
const RANKS: i32 = 10;
const SPACING_MM: i32 = 1_000;

struct Setup {
    bodies: Bodies,
    parts: Vec<Part>,
    /// Each man's spot at the start: a holder's slot, an enemy's file.
    home: Vec<Vec2>,
    walls: Vec<Wall>,
    /// The line a passer has passed once he is beyond it (x below it).
    through_x: i32,
}

fn man(pos: Vec2, facing: Turn, side: Side) -> Observed {
    Observed {
        pos,
        facing,
        side,
        ..Default::default()
    }
}

fn setup(case: &Case) -> Setup {
    let mut s = Setup {
        bodies: Bodies::default(),
        parts: Vec::new(),
        home: Vec::new(),
        walls: Vec::new(),
        through_x: 0,
    };
    let add = |s: &mut Setup, pos: Vec2, facing: Turn, side: Side, part: Part| {
        s.bodies.push(man(pos, facing, side));
        s.parts.push(part);
        s.home.push(pos);
    };
    let half = case.width_mm / 2;
    match case.kind {
        Kind::Interval => {
            // The line behind: a maniple either side of the interval, facing the enemy (+x),
            // its front rank half a metre behind the mouth.
            for side in [-1, 1] {
                for r in 0..RANKS {
                    for f in 0..FILES {
                        let y = side * (half + 300 + f * SPACING_MM);
                        add(
                            &mut s,
                            Vec2::new(-500 - r * SPACING_MM, y),
                            Turn(0),
                            Side::A,
                            Part::Holder,
                        );
                    }
                }
            }
            // The maniple falling back, three metres out, centred on the interval and facing
            // the enemy, who is in contact with its front.
            let maniple_back = 3_000;
            for r in 0..RANKS {
                for f in 0..FILES {
                    let y = (2 * f - (FILES - 1)) * SPACING_MM / 2;
                    add(
                        &mut s,
                        Vec2::new(maniple_back + r * SPACING_MM, y),
                        Turn(0),
                        Side::A,
                        Part::Passer,
                    );
                }
            }
            let enemy_front = maniple_back + RANKS * SPACING_MM;
            for r in 0..6 {
                for f in 0..16 {
                    let y = (2 * f - 15) * SPACING_MM / 2;
                    add(
                        &mut s,
                        Vec2::new(enemy_front + r * SPACING_MM, y),
                        Turn(Turn::HALF),
                        Side::B,
                        Part::Enemy,
                    );
                }
            }
            s.through_x = -500 - RANKS * SPACING_MM;
        }
        Kind::Gate => {
            for side in [-1, 1] {
                s.walls.push(Wall::new(
                    Vec2::new(0, side * half),
                    Vec2::new(0, side * RAMPART_MM),
                ));
                s.walls.push(Wall::new(
                    Vec2::new(0, side * half),
                    Vec2::new(-GATE_DEPTH_MM, side * half),
                ));
            }
            // Six hundred men in a loose mass, 30 wide and 20 deep at 1.2 m, ten metres out,
            // all facing the gate.
            for r in 0..20 {
                for f in 0..30 {
                    let y = (2 * f - 29) * 600 + (r % 2) * 300;
                    add(
                        &mut s,
                        Vec2::new(10_000 + r * 1_200, y),
                        Turn(Turn::HALF),
                        Side::A,
                        Part::Passer,
                    );
                }
            }
            s.through_x = -GATE_DEPTH_MM;
        }
    }
    s
}

/// Where a passer at `p` heads: the mouth while he is in front of it, then straight through,
/// then straight on.
fn passer_target(p: Vec2, half: i32, through_x: i32) -> Vec2 {
    let lane = (half - EDGE_MM).max(0);
    let y = p.y.clamp(-lane, lane);
    if p.x >= 0 {
        Vec2::new(-300, y)
    } else if p.x >= through_x {
        Vec2::new(through_x - 5_000, y)
    } else {
        Vec2::new(p.x - 5_000, p.y)
    }
}

fn targets(case: &Case, s: &Setup, out: &mut Vec<Target>) {
    let half = case.width_mm / 2;
    // A man running for a gate faces where he is going, so that his shove is toward the
    // mouth and not into the rampart beside it.
    let (pass_speed, pass_press, faces_ahead) = match (case.kind, case.pressed) {
        // A maniple falling back keeps its face to the enemy and does not shove its own line.
        (Kind::Interval, _) => (WALK, 0, false),
        (Kind::Gate, false) => (WALK, 0, true),
        (Kind::Gate, true) => (RUN, 255, true),
    };
    out.clear();
    for (i, &part) in s.parts.iter().enumerate() {
        let p = s.bodies.pos[i];
        let home = s.home[i];
        out.push(if s.bodies.posture[i] == Posture::Down {
            Target {
                point: p,
                speed: 0,
                face: None,
                press: 0,
            }
        } else {
            match part {
                Part::Passer => {
                    let point = passer_target(p, half, s.through_x);
                    Target {
                        point,
                        speed: pass_speed,
                        face: if faces_ahead {
                            Turn::of(point - p).or(Some(Turn(Turn::HALF)))
                        } else {
                            None
                        },
                        press: pass_press,
                    }
                }
                Part::Holder => Target {
                    point: home,
                    speed: 0,
                    face: Some(Turn(0)),
                    press: 0,
                },
                Part::Enemy if case.pressed => Target {
                    point: Vec2::new(ENEMY_STOP_MM, home.y),
                    speed: WALK,
                    face: Some(Turn(Turn::HALF)),
                    press: 255,
                },
                Part::Enemy => Target {
                    point: home,
                    speed: 0,
                    face: Some(Turn(Turn::HALF)),
                    press: 0,
                },
            }
        });
    }
}

/// What a passage run measured.
#[derive(Clone, Debug, PartialEq)]
pub struct Outcome {
    /// How many men were to pass.
    pub passers: usize,
    /// When each man who passed was through, ms, in order.
    pub passed_ms: Vec<u32>,
    /// The densest it got in front of the mouth: at each second, the 90th percentile over
    /// passers within 5 m of the mouth of the bodies within a metre of each, per m²; the
    /// largest of those.
    pub peak_density: f64,
    /// Men disabled in the crush.
    pub crushed: usize,
    /// Interval: the farthest any holder was pushed from his slot, mm.
    pub holder_max_mm: i32,
}

impl Outcome {
    /// Persons per second through the gap, from the tenth to the ninetieth man in a hundred,
    /// so that the start and the stragglers do not count.
    pub fn flow(&self) -> Option<f64> {
        let n = self.passed_ms.len();
        if n < 10 || n * 10 < self.passers * 9 {
            return None;
        }
        let (a, z) = (self.passers / 10, self.passers * 9 / 10 - 1);
        let span = self.passed_ms[z].saturating_sub(self.passed_ms[a]);
        (span > 0).then(|| (z - a) as f64 * 1000.0 / span as f64)
    }

    /// The time at which nine in ten were through, s.
    pub fn t90_s(&self) -> Option<f64> {
        let k = (self.passers * 9).div_ceil(10);
        (k > 0 && self.passed_ms.len() >= k).then(|| self.passed_ms[k - 1] as f64 / 1000.0)
    }

    /// The longest wait between two men passing, among the middle eight tenths, s: a clog.
    pub fn longest_gap_s(&self) -> Option<f64> {
        let n = self.passed_ms.len();
        if n * 10 < self.passers * 9 || n < 10 {
            return None;
        }
        let (a, z) = (self.passers / 10, self.passers * 9 / 10 - 1);
        self.passed_ms[a..=z]
            .windows(2)
            .map(|w| w[1] - w[0])
            .max()
            .map(|g| g as f64 / 1000.0)
    }
}

/// Runs one passage on a substrate built for its walls.
pub fn run<S: Substrate>(case: &Case, make: impl FnOnce(&[Wall]) -> S) -> Outcome {
    let mut s = setup(case);
    let mut sub = make(&s.walls);
    let rng = Rng::new(1);
    let n = s.bodies.len();
    let passers = s.parts.iter().filter(|&&p| p == Part::Passer).count();
    let mut passed: Vec<Option<u32>> = vec![None; n];
    let mut tg = Vec::with_capacity(n);
    let mut out: Vec<Event> = Vec::new();
    let mut outcome = Outcome {
        passers,
        passed_ms: Vec::new(),
        peak_density: 0.0,
        crushed: 0,
        holder_max_mm: 0,
    };
    let ticks = case.seconds * 1000 / BASE_TICK.0;
    let per_second = 1000 / BASE_TICK.0;
    let mut through = 0;
    for tick in 0..ticks {
        if is_man_step(tick) {
            targets(case, &s, &mut tg);
        }
        sub.advance(&mut s.bodies, &tg, BASE_TICK, &rng, tick, &mut out);
        for e in out.drain(..) {
            if matches!(e.kind, EventKind::Wound { .. }) && e.causes[0] == Cause::Crush {
                outcome.crushed += 1;
            }
            apply(&mut s.bodies, &e);
        }
        let t = (tick + 1) * BASE_TICK.0;
        for (i, when) in passed.iter_mut().enumerate() {
            if s.parts[i] == Part::Passer && when.is_none() && s.bodies.pos[i].x < s.through_x {
                *when = Some(t);
                through += 1;
            }
        }
        if (tick + 1) % per_second == 0 {
            outcome.peak_density = outcome.peak_density.max(mouth_density(&s));
            for i in 0..n {
                if s.parts[i] == Part::Holder {
                    let d = (s.bodies.pos[i].dist_sq(s.home[i]) as f64).sqrt() as i32;
                    outcome.holder_max_mm = outcome.holder_max_mm.max(d);
                }
            }
        }
        if through == passers && tick % MAN_EVERY == 0 {
            break;
        }
    }
    outcome.passed_ms = passed.into_iter().flatten().collect();
    outcome.passed_ms.sort_unstable();
    outcome
}

/// The 90th percentile, over standing passers within 5 m of the mouth, of the bodies within a
/// metre of each (himself included), per m².
fn mouth_density(s: &Setup) -> f64 {
    let b = &s.bodies;
    let near: Vec<usize> = (0..b.len())
        .filter(|&i| {
            s.parts[i] == Part::Passer
                && b.posture[i] != Posture::Down
                && b.pos[i].x >= 0
                && b.pos[i].x < 5_000
        })
        .collect();
    if near.is_empty() {
        return 0.0;
    }
    let mut counts: Vec<usize> = near
        .iter()
        .map(|&i| {
            (0..b.len())
                .filter(|&j| {
                    b.posture[j] != Posture::Down && b.pos[j].dist_sq(b.pos[i]) <= 1_000_000
                })
                .count()
        })
        .collect();
    counts.sort_unstable();
    let k = (counts.len() * 9).div_ceil(10) - 1;
    counts[k] as f64 / std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kinematic::Kinematic;
    use cannae_pairwise::{Pairwise, Params};

    fn pairwise(walls: &[Wall]) -> Pairwise {
        let mut s = Pairwise::new(Params::PLACEHOLDER);
        s.walls = walls.to_vec();
        s
    }

    #[test]
    fn without_contact_the_whole_maniple_passes_and_the_line_is_untouched() {
        let case = Case {
            kind: Kind::Interval,
            width_mm: 3_000,
            pressed: false,
            seconds: 60,
        };
        let o = run(&case, |_| Kinematic::default());
        assert_eq!(o.passers, 120);
        assert_eq!(o.passed_ms.len(), 120);
        assert!(o.passed_ms.windows(2).all(|w| w[0] <= w[1]));
        assert!(o.flow().is_some() && o.t90_s().is_some());
        assert_eq!(o.holder_max_mm, 0);
        assert_eq!(o.crushed, 0);
    }

    #[test]
    fn a_narrow_interval_passes_a_maniple_slower_than_a_wide_one() {
        let at = |width_mm| {
            run(
                &Case {
                    kind: Kind::Interval,
                    width_mm,
                    pressed: false,
                    seconds: 240,
                },
                pairwise,
            )
        };
        let (narrow, wide) = (at(3_000), at(12_000));
        assert_eq!(wide.passed_ms.len(), 120, "{wide:?}");
        let (tn, tw) = (narrow.t90_s(), wide.t90_s().unwrap());
        assert!(tn.is_none_or(|tn| tn > tw), "{tn:?} against {tw}");
        assert!(narrow.peak_density > wide.peak_density);
    }

    #[test]
    fn the_gate_is_walled_on_a_substrate_with_walls() {
        // Every man who is through came through the gate: none is beyond the rampart outside
        // the gate's width.
        let case = Case {
            kind: Kind::Gate,
            width_mm: 3_000,
            pressed: true,
            seconds: 30,
        };
        let mut s = setup(&case);
        let mut sub = pairwise(&s.walls);
        let rng = Rng::new(1);
        let mut tg = Vec::new();
        let mut out = Vec::new();
        for tick in 0..30_000 / BASE_TICK.0 {
            if is_man_step(tick) {
                targets(&case, &s, &mut tg);
            }
            sub.advance(&mut s.bodies, &tg, BASE_TICK, &rng, tick, &mut out);
            for e in out.drain(..) {
                apply(&mut s.bodies, &e);
            }
        }
        let inside: Vec<Vec2> = s.bodies.pos.iter().copied().filter(|p| p.x < 0).collect();
        assert!(!inside.is_empty());
        for p in inside {
            assert!(p.x < -GATE_DEPTH_MM || p.y.abs() <= 1_500, "{p:?}");
        }
    }
}
