//! How the losing side's men came to turn, before its break: from which posture each first
//! turned, and the first transitions of the run. A look at the mechanism, asked by Varro
//! (call 0001, turn 34) about the costing runs; it reads no pass mark.
//!
//! `cargo run --release -p cannae-score --example first_turns -- [--rung R] [--seeds N]`

use cannae_core::{EventKind, Posture, Side};
use cannae_harness::{Scenario, place};
use cannae_score::{Engine, Spec, run_logged};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str, default: u64| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    };
    let (rung, seeds) = (arg("--rung", 1) as u8, arg("--seeds", 16));
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/patch/base.toml"
    ))
    .unwrap();
    let s = Scenario::from_toml(&text).unwrap();
    let (bodies, _) = place(&s);
    for engine in [Engine::Pairwise, Engine::Field] {
        // Turns by the posture turned from, before the loser's break: Steady, Forward, Back.
        let mut from = [0u32; 5];
        let mut first_moves = [[0u32; 5]; 5];
        for seed in 1..=seeds {
            let spec = Spec {
                engine,
                rung,
                seed,
                duration_s: 1_200,
                stop_at_rout_end: true,
            };
            let (r, log) = run_logged(&s, spec).unwrap();
            let (Some(loser), Some(t_break)) = (
                r.summary.loser,
                r.summary
                    .loser
                    .and_then(|l| r.summary.side_break[l as usize]),
            ) else {
                continue;
            };
            let mine = |m: usize| bodies.side[m] == loser;
            let mut seen = vec![false; bodies.len()];
            let mut moves = 0;
            for e in &log {
                if e.tick > t_break {
                    break;
                }
                let EventKind::Posture { from: f, to } = e.kind else {
                    continue;
                };
                let m = e.man.0 as usize;
                if !mine(m) {
                    continue;
                }
                // The first ten changes of posture on the losing side, by kind.
                if moves < 10 {
                    first_moves[f as usize][to as usize] += 1;
                    moves += 1;
                }
                if to == Posture::Turned && !seen[m] {
                    seen[m] = true;
                    from[f as usize] += 1;
                }
            }
            let side = if loser == Side::A {
                "Romans"
            } else {
                "warband"
            };
            eprintln!("{engine:?} seed {seed}: the {side} broke at tick {t_break}");
        }
        println!("{engine:?}, rung {rung}, {seeds} seeds, the losing side before its break:");
        println!(
            "  first turns, by the posture turned from: steady {}, forward {}, back {}",
            from[0], from[1], from[2]
        );
        println!("  the first ten changes of posture in each run, from -> to:");
        let name = ["steady", "forward", "back", "turned", "down"];
        for (f, row) in first_moves.iter().enumerate() {
            for (t, &c) in row.iter().enumerate() {
                if c > 0 {
                    println!("    {} -> {}: {c}", name[f], name[t]);
                }
            }
        }
    }
}
