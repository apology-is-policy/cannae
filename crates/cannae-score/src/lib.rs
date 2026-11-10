//! Slice 0's scored runs: one run of the patch on either substrate at any rung of the memory
//! ladder, summarised from observable state and the event log
//! ([preregistration](../../docs/slice0/preregistration.md)).
//!
//! Every parameter is at its placeholder or its prior's centre until the sampler draws them.
//! The wall-clock readings are for costing the batch only; they never touch the simulation.

pub mod draw;

use cannae_core::power::GaitSpeeds;
use cannae_core::schedule::BASE_TICK;
use cannae_core::{
    BoutParams, Bouts, Event, GridPerception, Perceive, Posture, Rng, Substrate, Vec2,
};
use cannae_field::{Field, FieldPerception};
use cannae_gauges::Rung3;
use cannae_harness::metrics::{Inputs, Recorder, RunSummary, broken_sides, scoring_end, summarize};
use cannae_harness::scenario::{Layout, frames};
use cannae_harness::{Runner, Scenario, place};
use cannae_man::{Marks, Memory, Men, Params, Rung, Troop};
use cannae_pairwise::Pairwise;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Engine {
    Pairwise,
    Field,
}

/// One run: which engine, which rung (0-3), which seed, and how long to simulate.
#[derive(Clone, Copy, Debug)]
pub struct Spec {
    pub engine: Engine,
    pub rung: u8,
    pub seed: u64,
    pub duration_s: u32,
    /// Stop where scoring stops, [`cannae_harness::metrics::ROUT_WINDOW_S`] after the first side breaks. The summary is
    /// the same as the full run's (tested).
    pub stop_at_rout_end: bool,
}

#[derive(Clone, Debug)]
pub struct Run {
    pub seed: u64,
    pub summary: RunSummary,
    pub men: usize,
    pub sim_s: u32,
    pub wall: Duration,
    /// Wall time when the losing side broke, to the nearest simulated minute: what a stop
    /// soon after the break would save.
    pub wall_at_break: Option<Duration>,
    pub events: usize,
    /// Men in each posture at the end, by `Posture as usize`.
    pub end_postures: [u32; 5],
    /// The span of the men still standing at the end, in mm.
    pub end_span_mm: (i32, i32),
    pub evaluated: u64,
    pub skipped: u64,
}

/// The share of standing men who must have turned for a body or a side to count as broken
/// (`scenarios/prereg.toml`).
pub const BREAK_SHARE: f64 = 0.4;

pub fn run(s: &Scenario, spec: Spec) -> Result<Run, String> {
    run_logged(s, spec).map(|(run, _)| run)
}

/// The run, and its whole event log, for looking into what happened in it.
pub fn run_logged(s: &Scenario, spec: Spec) -> Result<(Run, Vec<Event>), String> {
    let troops = s
        .bodies
        .iter()
        .map(|b| Troop::named(&b.troop).ok_or_else(|| format!("unknown troop {:?}", b.troop)))
        .collect::<Result<Vec<_>, _>>()?;
    let rung = match spec.rung {
        0 => Some(Rung::NoMarks),
        1 => Some(Rung::HasFled),
        2 => Some(Rung::Shaken),
        3 => None,
        r => return Err(format!("no rung {r}")),
    };
    let marks = |n| Marks::new(rung.unwrap_or(Rung::HasFled), &Params::CENTRE, n);
    let rung3 = |n| Rung3::new(&Params::CENTRE, cannae_gauges::Params::CENTRE, n);
    let walls = s.walls();
    match (spec.engine, rung) {
        (Engine::Pairwise, Some(_)) => play(
            s,
            spec,
            &troops,
            GridPerception::default(),
            marks,
            pairwise(walls),
        ),
        (Engine::Pairwise, None) => play(
            s,
            spec,
            &troops,
            GridPerception::default(),
            rung3,
            pairwise(walls),
        ),
        (Engine::Field, Some(_)) => play(
            s,
            spec,
            &troops,
            FieldPerception::default(),
            marks,
            Field::with_walls(cannae_field::Params::CENTRE, &walls),
        ),
        (Engine::Field, None) => play(
            s,
            spec,
            &troops,
            FieldPerception::default(),
            rung3,
            Field::with_walls(cannae_field::Params::CENTRE, &walls),
        ),
    }
}

fn pairwise(walls: Vec<cannae_core::Wall>) -> Pairwise {
    let mut p = Pairwise::new(cannae_pairwise::Params::PLACEHOLDER);
    p.walls = walls;
    p
}

fn play<P: Perceive, M: Memory, S: Substrate>(
    s: &Scenario,
    spec: Spec,
    troops: &[Troop],
    perception: P,
    memory: impl FnOnce(usize) -> M,
    substrate: S,
) -> Result<(Run, Vec<Event>), String> {
    let started = Instant::now();
    let (bodies, placed) = place(s);
    let f = frames(s, &placed, &bodies)?;
    let initial = bodies.clone();
    let layout = Layout::of(&placed);
    let mut rec = Recorder::new(&initial, placed.len());
    let n = bodies.len();
    let rng = Rng::new(spec.seed);
    let men = Men::with_memory(memory(n), Params::CENTRE, &bodies, troops, &rng);
    let bouts = Bouts::new(BoutParams::PLACEHOLDER, men.kits());
    let mut r = Runner::new(
        bodies,
        perception,
        men,
        substrate,
        f,
        bouts,
        rng,
        GaitSpeeds::PLACEHOLDER,
    );
    let end = spec.duration_s * 1000 / BASE_TICK.0;
    let minute = 60_000 / BASE_TICK.0;
    let mut clock = Vec::with_capacity((end / minute + 1) as usize);
    let side_of_body: Vec<_> = placed
        .iter()
        .map(|p| initial.side[p.first as usize])
        .collect();
    let mut stop = end;
    while r.tick < stop {
        if r.tick % minute == 0 {
            clock.push(started.elapsed());
        }
        if r.step() {
            rec.record(r.tick - 1, &r.bodies);
            if spec.stop_at_rout_end
                && stop == end
                && let Some(smp) = rec.samples.last()
                && broken_sides(smp, &side_of_body, BREAK_SHARE).contains(&true)
            {
                stop = scoring_end(Some(smp.tick), end);
            }
        }
    }
    let summary = summarize(&Inputs {
        initial: &initial,
        placed: &placed,
        layout: &layout,
        samples: &rec.samples,
        events: r.log.all(),
        end_tick: stop,
        break_share: BREAK_SHARE,
    });
    let wall_at_break = summary
        .loser
        .and_then(|l| summary.side_break[l as usize])
        .and_then(|t| clock.get((t / minute) as usize + 1).copied());
    let mut end_postures = [0u32; 5];
    let (mut lo, mut hi) = (Vec2::new(i32::MAX, i32::MAX), Vec2::new(i32::MIN, i32::MIN));
    for i in 0..n {
        end_postures[r.bodies.posture[i] as usize] += 1;
        if r.bodies.posture[i] != Posture::Down {
            let p = r.bodies.pos[i];
            lo = Vec2::new(lo.x.min(p.x), lo.y.min(p.y));
            hi = Vec2::new(hi.x.max(p.x), hi.y.max(p.y));
        }
    }
    let run = Run {
        end_postures,
        end_span_mm: (hi.x.saturating_sub(lo.x), hi.y.saturating_sub(lo.y)),
        seed: spec.seed,
        men: n,
        sim_s: stop * BASE_TICK.0 / 1000,
        wall: started.elapsed(),
        wall_at_break,
        events: r.log.all().len(),
        evaluated: r.evaluated,
        skipped: r.skipped,
        summary,
    };
    Ok((run, r.log.all().to_vec()))
}
