//! The passage study's calibration (S0.8): the pairwise substrate against measured bottleneck
//! flows, the substrate alone, with no man model.
//!
//! The geometry follows Seyfried et al. 2009 ("New insights into pedestrian flow through
//! bottlenecks", Transportation Science 43:395-406): 60 people start in a waiting area at
//! about 3.3-4 per m², then pass a corridor 2.8 m long and b wide. Their walkers were told not
//! to push, and the specific flow came out near 1.9 persons per metre per second, linear in b.
//! Garcimartín et al. 2016 let people push, through doors of 0.69 and 0.75 m, and found about
//! 4.0. Sources: docs/slice0/crowd-physics.md.
//!
//! `cargo run --release -p cannae-pairwise --example passage`
#![allow(clippy::float_arithmetic)]

use cannae_core::schedule::BASE_TICK;
use cannae_core::{Bodies, Observed, Rng, Substrate, Target, Turn, Vec2, Wall};
use cannae_pairwise::{Pairwise, Params};

const EXIT_X: i32 = 2_800;

/// Seconds for each man to pass the corridor's far end, in order; `None` if he never did.
fn run(width_mm: i32, press: u8, params: Params) -> Vec<f64> {
    let mut b = Bodies::default();
    // Sixty men in a waiting area 3.3 m deep and 4.5 m wide, ending 0.5 m short of the
    // corridor: 6 columns by 10 rows, about 4 per m², slightly staggered.
    for row in 0..10 {
        for col in 0..6 {
            b.push(Observed {
                pos: Vec2::new(-500 - col * 550 - (row % 2) * 150, (row - 5) * 450 + 225),
                facing: Turn(0),
                ..Default::default()
            });
        }
    }
    let half = width_mm / 2;
    let mut s = Pairwise::new(params);
    s.walls = vec![
        // The waiting area's sides.
        Wall::new(Vec2::new(-6_000, 2_400), Vec2::new(0, 2_400)),
        Wall::new(Vec2::new(-6_000, -2_400), Vec2::new(0, -2_400)),
        // The face of the bottleneck, either side of its mouth.
        Wall::new(Vec2::new(0, half), Vec2::new(0, 2_400)),
        Wall::new(Vec2::new(0, -half), Vec2::new(0, -2_400)),
        // The corridor.
        Wall::new(Vec2::new(0, half), Vec2::new(EXIT_X, half)),
        Wall::new(Vec2::new(0, -half), Vec2::new(EXIT_X, -half)),
    ];
    let rng = Rng::new(1);
    let mut out = Vec::new();
    let mut passed: Vec<Option<f64>> = vec![None; b.len()];
    let ticks = 120_000 / BASE_TICK.0;
    for tick in 0..ticks {
        // The least steering a person does: head for the mouth of the bottleneck, then down
        // the corridor, then straight on. Aiming past the mouth from beside it would walk a man
        // into the door frame, which turns him away from the opening along its edge.
        let targets: Vec<Target> = b
            .pos
            .iter()
            .map(|p| Target {
                point: if p.x < 0 {
                    Vec2::new(300, 0)
                } else if p.x < EXIT_X {
                    Vec2::new(EXIT_X + 5_000, 0)
                } else {
                    Vec2::new(p.x + 5_000, p.y)
                },
                speed: 1_300,
                face: Some(Turn(0)),
                press,
            })
            .collect();
        s.advance(&mut b, &targets, BASE_TICK, &rng, tick, &mut out);
        out.clear();
        let t = (tick + 1) as f64 * BASE_TICK.0 as f64 / 1000.0;
        for (i, p) in b.pos.iter().enumerate() {
            if passed[i].is_none() && p.x >= EXIT_X {
                passed[i] = Some(t);
            }
        }
        if passed.iter().all(Option::is_some) {
            break;
        }
    }
    let mut times: Vec<f64> = passed.into_iter().flatten().collect();
    times.sort_by(f64::total_cmp);
    times
}

/// Persons per second over the middle of the stream: from the 6th to the 54th to pass, so the
/// start and the stragglers do not count.
fn flow(times: &[f64]) -> Option<f64> {
    if times.len() < 55 {
        return None;
    }
    let (a, z) = (times[5], times[53]);
    (z > a).then(|| 48.0 / (z - a))
}

fn main() {
    println!("Pairwise substrate, placeholder parameters but the disc radius; specific flow Js");
    println!(
        "in persons per metre of width per second, by disc radius (all 60 pass unless marked)"
    );
    println!(
        "{:>8} {:>6} {:>8} {:>8} {:>8}",
        "width", "press", "r 240", "r 280", "r 300"
    );
    for (press, widths) in [
        (0u8, vec![800, 900, 1_000, 1_100, 1_200]),
        (128, vec![690, 750, 1_000, 1_200]),
        (255, vec![690, 750, 1_000, 1_200]),
    ] {
        for w in widths {
            let cells: Vec<String> = [240, 280, 300]
                .iter()
                .map(|&r| {
                    let times = run(
                        w,
                        press,
                        Params {
                            radius_mm: r,
                            ..Params::PLACEHOLDER
                        },
                    );
                    match flow(&times) {
                        Some(j) if times.len() == 60 => format!("{:.2}", j / (w as f64 / 1000.0)),
                        Some(j) => format!("{:.2}*{}", j / (w as f64 / 1000.0), times.len()),
                        None => format!("-*{}", times.len()),
                    }
                })
                .collect();
            println!(
                "{:>5} mm {:>6} {:>8} {:>8} {:>8}",
                w, press, cells[0], cells[1], cells[2]
            );
        }
    }
    println!("Measured: Seyfried et al. 2009, no pushing, Js 1.6-2.3 (J = 1.9 b, linear in b);");
    println!("          Garcimartin et al. 2016, doors 0.69-0.75 m, Js about 4.0 at every level");
    println!("          of competitiveness.");
}
