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
//! `cargo run --release -p cannae-pairwise --example passage [-- --spread]`
//!
//! `--spread` steers each man to the point of the mouth as far across it as he stands across
//! the waiting area, instead of to its nearest point.
#![allow(clippy::float_arithmetic)]

use cannae_core::schedule::BASE_TICK;
use cannae_core::{Bodies, Observed, Rng, Substrate, Target, Turn, Vec2, Wall};
use cannae_pairwise::{Pairwise, Params};

/// One experiment's layout: the crowd in its waiting area, and the bottleneck's length.
struct Layout {
    /// Men across the waiting area, and their spacing, mm.
    wide: i32,
    dy: i32,
    /// Men deep, and their spacing, mm.
    deep: i32,
    dx: i32,
    /// Half the waiting area's width, mm.
    half_area: i32,
    /// The bottleneck's length, mm.
    exit_x: i32,
}

/// Seyfried et al. 2009: sixty men in a waiting area 4.8 m wide, 3.3 m deep from 0.5 m short
/// of a corridor 2.8 m long; 6 deep by 10 across, about 4 per m², slightly staggered.
const SEYFRIED: Layout = Layout {
    wide: 10,
    dy: 450,
    deep: 6,
    dx: 550,
    half_area: 2_400,
    exit_x: 2_800,
};

/// Liao et al.'s wide bottlenecks (arXiv 1506.02433, Table 2, experiment AO): 350 people at
/// 3.0 per m² in front of a bottleneck 1.0 m long, in a waiting area 18 m wide. Here 360, 30
/// across by 12 deep.
const LIAO: Layout = Layout {
    wide: 30,
    dy: 600,
    deep: 12,
    dx: 540,
    half_area: 9_000,
    exit_x: 1_000,
};

/// Seconds for each man to pass the bottleneck's far end, in order, for those who did.
fn run(layout: &Layout, width_mm: i32, press: u8, params: Params, spread: bool) -> Vec<f64> {
    let mut b = Bodies::default();
    for row in 0..layout.wide {
        for col in 0..layout.deep {
            b.push(Observed {
                pos: Vec2::new(
                    -500 - col * layout.dx - (row % 2) * 150,
                    (2 * row - (layout.wide - 1)) * layout.dy / 2,
                ),
                facing: Turn(0),
                ..Default::default()
            });
        }
    }
    let half = width_mm / 2;
    let (area, exit_x) = (layout.half_area, layout.exit_x);
    let mut s = Pairwise::new(params);
    s.walls = vec![
        // The waiting area's sides.
        Wall::new(Vec2::new(-12_000, area), Vec2::new(0, area)),
        Wall::new(Vec2::new(-12_000, -area), Vec2::new(0, -area)),
        // The face of the bottleneck, either side of its mouth.
        Wall::new(Vec2::new(0, half), Vec2::new(0, area)),
        Wall::new(Vec2::new(0, -half), Vec2::new(0, -area)),
        // The bottleneck.
        Wall::new(Vec2::new(0, half), Vec2::new(exit_x, half)),
        Wall::new(Vec2::new(0, -half), Vec2::new(exit_x, -half)),
    ];
    let rng = Rng::new(1);
    let mut out = Vec::new();
    let mut passed: Vec<Option<f64>> = vec![None; b.len()];
    // Men aim at the nearest point of the mouth, a body's radius and a hand in from its edges.
    let lane = (half - 400).max(0);
    let ticks = 240_000 / BASE_TICK.0;
    for tick in 0..ticks {
        // The least steering a person does: head for the mouth of the bottleneck, then down
        // the corridor, then straight on. Aiming past the mouth from beside it would walk a man
        // into the door frame, which turns him away from the opening along its edge.
        let targets: Vec<Target> = b
            .pos
            .iter()
            .map(|p| Target {
                point: if p.x < 0 && spread {
                    // The whole mouth used: each man makes for the point as far across it as
                    // he stands across the waiting area.
                    Vec2::new(300, (p.y as i64 * lane as i64 / area as i64) as i32)
                } else if p.x < 0 {
                    Vec2::new(300, p.y.clamp(-lane, lane))
                } else if p.x < exit_x {
                    Vec2::new(exit_x + 5_000, p.y.clamp(-lane, lane))
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
            if passed[i].is_none() && p.x >= exit_x {
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

/// Persons per second over the middle of the stream, from the tenth to the ninetieth man in a
/// hundred (the 6th to the 54th of sixty), so the start and the stragglers do not count.
fn flow(times: &[f64], n: usize) -> Option<f64> {
    let (a, z) = (n / 10 - 1, n * 9 / 10 - 1);
    if times.len() <= z {
        return None;
    }
    let (ta, tz) = (times[a], times[z]);
    (tz > ta).then(|| (z - a) as f64 / (tz - ta))
}

fn main() {
    let spread = std::env::args().any(|a| a == "--spread");
    println!("Pairwise substrate, placeholder parameters but the disc radius; specific flow Js");
    println!("in persons per metre of width per second, by disc radius (all pass unless marked");
    println!("with how many did)");
    for (name, layout, cases) in [
        (
            "Seyfried et al. 2009 (60 people)",
            &SEYFRIED,
            vec![
                (0u8, vec![800, 900, 1_000, 1_100, 1_200]),
                (128, vec![690, 750, 1_000, 1_200]),
                (255, vec![690, 750, 1_000, 1_200]),
            ],
        ),
        (
            "Liao et al., wide bottlenecks (360 people)",
            &LIAO,
            vec![(0u8, vec![2_400, 3_000, 3_600, 4_400, 5_000])],
        ),
    ] {
        let n = (layout.wide * layout.deep) as usize;
        println!();
        println!("{name}");
        println!(
            "{:>8} {:>6} {:>8} {:>8} {:>8}",
            "width", "press", "r 240", "r 280", "r 300"
        );
        for (press, widths) in cases {
            for w in widths {
                let cells: Vec<String> = [240, 280, 300]
                    .iter()
                    .map(|&r| {
                        let times = run(
                            layout,
                            w,
                            press,
                            Params {
                                radius_mm: r,
                                ..Params::PLACEHOLDER
                            },
                            spread,
                        );
                        let js = |j: f64| j / (w as f64 / 1000.0);
                        match flow(&times, n) {
                            Some(j) if times.len() == n => format!("{:.2}", js(j)),
                            Some(j) => format!("{:.2}*{}", js(j), times.len()),
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
    }
    println!();
    println!("Measured: Seyfried et al. 2009, no pushing, Js 1.6-2.3 (J = 1.9 b, linear in b);");
    println!("          Liao et al.: \"the flow per unit width is constant as bottleneck width");
    println!("          changes\", out to 5.0 m;");
    println!("          Garcimartin et al. 2016, doors 0.69-0.75 m, Js about 4.0 at every level");
    println!("          of competitiveness.");
}
