//! `cargo run --release -p cannae-score --example contact_causes -- [--seeds N] [--rung R]
//!     [--seconds N] [--flank Q8]`
//!
//! The hand-built scene `scenarios/scenes/first-contact.toml` on both engines: when the first
//! man steps forward, when the century breaks, and how its men came to give ground and to turn,
//! by the first cause each hazard named. A look at the table of hazards, not at the patch; it
//! reads no pass mark. `--flank` overrides one weight, to tell that row's doing from the rest;
//! it is a diagnostic and not a prior.

use cannae_core::{Cause, EventKind, Posture, Side};
use cannae_harness::{Scenario, place};
use cannae_score::draw::{Cell, nominal};
use cannae_score::{Engine, Spec, run_logged};
use std::collections::BTreeMap;

const SCENE: &str = include_str!("../../../scenarios/scenes/first-contact.toml");

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse::<u64>().ok())
    };
    let seeds = arg("--seeds").unwrap_or(16);
    let rung = arg("--rung").unwrap_or(0) as u8;
    let seconds = arg("--seconds").unwrap_or(900) as u32;
    let s = Scenario::from_toml(SCENE).unwrap();
    let (bodies, _) = place(&s);
    let romans = bodies.side.iter().filter(|&&x| x == Side::A).count();
    for engine in [Engine::Pairwise, Engine::Field] {
        let mut d = nominal(Cell { engine, rung }).unwrap();
        if let Some(f) = arg("--flank") {
            d.man.enemy_flank = f as i32;
        }
        let (mut broke, mut times) = (0, Vec::new());
        let mut down_at_break = Vec::new();
        let (mut driven, mut gave) = (0u32, 0u32);
        let mut gave_by: BTreeMap<Cause, u32> = BTreeMap::new();
        let mut turned_by: BTreeMap<Cause, u32> = BTreeMap::new();
        for seed in 1..=seeds {
            let spec = Spec {
                seed,
                duration_s: seconds,
                stop_at_rout_end: true,
            };
            let (r, log) = run_logged(&s, &d, spec).unwrap();
            let t_break = r.summary.side_break[Side::A as usize];
            let first_forward = log
                .iter()
                .find(|e| {
                    matches!(
                        e.kind,
                        EventKind::Posture {
                            to: Posture::Forward,
                            ..
                        }
                    )
                })
                .map(|e| e.tick);
            if let (Some(b), Some(f)) = (t_break, first_forward) {
                broke += 1;
                times.push((b.saturating_sub(f)) / 20);
            }
            let mut down = 0;
            for e in &log {
                if t_break.is_some_and(|b| e.tick > b) {
                    break;
                }
                if bodies.side[e.man.0 as usize] != Side::A {
                    continue;
                }
                match e.kind {
                    EventKind::Posture {
                        from: Posture::Steady,
                        to: Posture::Back,
                    } => {
                        if e.causes[0] == Cause::Bout {
                            driven += 1;
                        } else {
                            gave += 1;
                            *gave_by.entry(e.causes[0]).or_default() += 1;
                        }
                    }
                    EventKind::Posture {
                        to: Posture::Turned,
                        ..
                    } => *turned_by.entry(e.causes[0]).or_default() += 1,
                    EventKind::Posture {
                        to: Posture::Down, ..
                    } => down += 1,
                    _ => {}
                }
            }
            if t_break.is_some() {
                down_at_break.push(down);
            }
        }
        times.sort_unstable();
        down_at_break.sort_unstable();
        let mid = |v: &[u32]| v.get(v.len() / 2).copied();
        println!("{engine:?}, rung {rung}, {seeds} seeds of {seconds} s, a century of {romans}:");
        println!(
            "  the century broke in {broke}; from the first man forward to the break: median {:?} s, least {:?}, most {:?}; down at the break: median {:?}",
            mid(&times),
            times.first(),
            times.last(),
            mid(&down_at_break)
        );
        println!(
            "  Romans up to their break: driven back by a bout {driven}, gave ground of themselves {gave}"
        );
        println!("  gave ground, by first cause: {gave_by:?}");
        println!("  turned, by first cause: {turned_by:?}");
    }
}
