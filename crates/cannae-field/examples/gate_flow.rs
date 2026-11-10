//! The field's walking capacity against measured bottleneck flows: the substrate alone, with
//! no man model.
//!
//! The geometry is Liao et al.'s wide bottlenecks (arXiv 1506.02433, Table 2, experiment AO),
//! as in the pairwise engine's calibration (`cannae-pairwise --example passage`): 360 men at
//! about 3 per m² in a waiting area 18 m wide, in front of a bottleneck 1 m long and 2.4 to
//! 5 m wide. They measured a flow per metre of width that is constant across those widths.
//! Seyfried et al. 2009 measured 1.6 to 2.3 persons per metre per second in corridors of 0.8
//! to 1.2 m, which are narrower than a cell and cannot be run here.
//!
//! `cargo run --release -p cannae-field --example gate_flow [-- --jam-q8 N] [--crush-q8 N] [--length-mm N]
//! [--spread] [--flows A,B,..]`
//!
//! `--spread` changes the steering, to tell the gap's capacity from the supply to it: each man
//! makes for the point of the mouth that is as far across it as he is across the waiting
//! area, so the whole mouth is used.
#![allow(clippy::float_arithmetic)]

use cannae_core::schedule::BASE_TICK;
use cannae_core::{Bodies, Observed, Rng, Substrate, Target, Turn, Vec2, Wall};
use cannae_field::{Field, Params};

const WALK: i32 = 1_300;
/// Men across the waiting area and deep, and their spacing.
const WIDE: i32 = 30;
const DEEP: i32 = 12;
const DY: i32 = 600;
const DX: i32 = 540;
const HALF_AREA: i32 = 9_000;

#[derive(Clone, Copy)]
struct Setup {
    /// The bottleneck's length, mm.
    exit_x: i32,
    spread: bool,
}

struct Passed {
    /// Seconds at which each man passed the far end, in order.
    times: Vec<f64>,
    crushed: usize,
    /// The densest any cell was, men per m².
    peak: f64,
}

fn run(width_mm: i32, params: Params, setup: Setup) -> Passed {
    let exit_x = setup.exit_x;
    let mut b = Bodies::default();
    for row in 0..WIDE {
        for col in 0..DEEP {
            b.push(Observed {
                pos: Vec2::new(
                    -500 - col * DX - (row % 2) * 150,
                    (2 * row - (WIDE - 1)) * DY / 2,
                ),
                facing: Turn(0),
                ..Default::default()
            });
        }
    }
    let half = width_mm / 2;
    let walls = [
        Wall::new(Vec2::new(-12_000, HALF_AREA), Vec2::new(0, HALF_AREA)),
        Wall::new(Vec2::new(-12_000, -HALF_AREA), Vec2::new(0, -HALF_AREA)),
        Wall::new(Vec2::new(0, half), Vec2::new(0, HALF_AREA)),
        Wall::new(Vec2::new(0, -half), Vec2::new(0, -HALF_AREA)),
        Wall::new(Vec2::new(0, half), Vec2::new(exit_x, half)),
        Wall::new(Vec2::new(0, -half), Vec2::new(exit_x, -half)),
    ];
    let mut f = Field::with_walls(params, &walls);
    let rng = Rng::new(1);
    let mut out = Vec::new();
    let mut passed: Vec<Option<f64>> = vec![None; b.len()];
    let mut crushed = 0;
    let mut peak = 0u32;
    // The bench's steering: the nearest point of the mouth, 0.4 m in from its edges; then
    // straight through; then straight on.
    let lane = (half - 400).max(0);
    for tick in 0..600_000 / BASE_TICK.0 {
        let targets: Vec<Target> = b
            .pos
            .iter()
            .map(|p| {
                let point = if p.x < 0 && setup.spread {
                    Vec2::new(300, (p.y as i64 * lane as i64 / HALF_AREA as i64) as i32)
                } else if p.x < 0 {
                    Vec2::new(300, p.y.clamp(-lane, lane))
                } else if p.x < exit_x {
                    Vec2::new(exit_x + 5_000, p.y.clamp(-lane, lane))
                } else {
                    Vec2::new(p.x + 5_000, p.y)
                };
                Target {
                    point,
                    speed: WALK,
                    face: Turn::of(point - *p),
                    press: 0,
                }
            })
            .collect();
        f.advance(&mut b, &targets, BASE_TICK, &rng, tick, &mut out);
        crushed += out.len() / 2;
        out.clear();
        peak = peak.max(f.peak_q8());
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
    Passed {
        times,
        crushed,
        peak: peak as f64 / 256.0,
    }
}

/// Persons per second from the tenth to the ninetieth man in a hundred.
fn flow(times: &[f64], n: usize) -> Option<f64> {
    let (a, z) = (n / 10 - 1, n * 9 / 10 - 1);
    if times.len() <= z {
        return None;
    }
    (times[z] > times[a]).then(|| (z - a) as f64 / (times[z] - times[a]))
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut jam_q8 = Params::CENTRE.jam_q8;
    let mut crush_q8 = Params::CENTRE.crush_q8;
    let mut setup = Setup {
        exit_x: 1_000,
        spread: false,
    };
    let mut flows = vec![235, 280, 315, 335, 355, 374, 395, 415, 435, 455, 475, 500];
    while let Some(a) = args.next() {
        let mut number = || args.next().and_then(|v| v.parse().ok());
        match a.as_str() {
            "--jam-q8" => jam_q8 = number().unwrap_or(jam_q8),
            "--crush-q8" => crush_q8 = number().unwrap_or(crush_q8),
            "--length-mm" => setup.exit_x = number().unwrap_or(1_000) as i32,
            "--spread" => setup.spread = true,
            "--flows" => {
                flows = args
                    .next()
                    .map(|v| v.split(',').filter_map(|f| f.parse().ok()).collect())
                    .unwrap_or(flows)
            }
            _ => {}
        }
    }
    let n = (WIDE * DEEP) as usize;
    let widths = [2_400, 3_000, 3_600, 4_400, 5_000];
    println!(
        "Field substrate, contact Line; 360 men at 3 per m2 before a bottleneck {:.1} m long; \
         jam density {:.2} per m2; steering {}",
        setup.exit_x as f64 / 1000.0,
        jam_q8 as f64 / 256.0,
        if setup.spread {
            "spread across the mouth"
        } else {
            "to the nearest point of the mouth"
        }
    );
    println!(
        "Capacity: what the walking curve allows, persons per metre per second at a walk of \
         1.3 m/s. Js: what passed, from the 10th to the 90th man in a hundred."
    );
    print!("{:>8} {:>9}", "flow_q8", "capacity");
    for w in widths {
        print!(" {:>7.1}m", w as f64 / 1000.0);
    }
    println!("  {:>5} {:>7}", "peak", "crushed");
    for flow_q8 in flows {
        let params = Params {
            flow_q8,
            jam_q8,
            crush_q8,
            ..Params::CENTRE
        };
        print!(
            "{:>8} {:>9.2}",
            flow_q8,
            flow_q8 as f64 / 256.0 * WALK as f64 / 1000.0
        );
        let (mut peak, mut crushed) = (0.0f64, 0);
        for w in widths {
            let r = run(w, params, setup);
            peak = peak.max(r.peak);
            crushed += r.crushed;
            match flow(&r.times, n) {
                Some(j) if r.times.len() == n => print!(" {:>8.2}", j / (w as f64 / 1000.0)),
                Some(j) => print!(" {:>4.2}*{:<3}", j / (w as f64 / 1000.0), r.times.len()),
                None => print!(" {:>5}*{:<3}", "-", r.times.len()),
            }
        }
        println!("  {peak:>5.1} {crushed:>7}");
    }
    println!();
    println!("Measured: Liao et al., flow per metre constant from 2.4 to 5.0 m; Seyfried et al.");
    println!("          2009, Js 1.6 to 2.3 with no pushing (about 1.9).");
}
