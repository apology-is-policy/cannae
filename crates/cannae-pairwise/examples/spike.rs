//! The S0.7 speed spike for the pairwise substrate: 150,000 bodies, substrate only (no man
//! model yet), measured as simulated seconds per wall second.
//!
//! `cargo run --release -p cannae-pairwise --example spike -- <line|surge|pocket|rout>
//!  [--threads N] [--seconds S] [--wall-s W] [--report-every-s R]`
//!
//! `line`: two armies of 75,000, 1,000 files by 75 ranks each, pressed together along 1.5 km.
//! It settles into a still jam within seconds.
//! `surge`: the same line, but every man is re-targeted 3 m ahead of where he stands at each
//! man step, so the jam never settles.
//! `pocket`: 60,000 men packed in a disc, 90,000 around them pressing inward.
//! `rout`: 150,000 men running for a camp, 500 across and 300 deep at about one per m², 20 m
//! out from a rampart 640 m long with three gates 12 m wide. Each runs and shoves for the
//! mouth of the nearest gate, through it, and stops 300 m inside. Most of them are pressed
//! against the rampart for most of the run: the worst case for contact with walls.
//!
//! `--wall-s` stops after that much wall time, for sustained (throttled) readings; it cannot
//! change the state reached at a given tick.
#![allow(clippy::float_arithmetic)]

use cannae_core::schedule::BASE_TICK;
use cannae_core::{Bodies, Observed, Rng, Side, Substrate, Target, Turn, Vec2, Wall};
use cannae_pairwise::{Pairwise, Params};
use std::time::Instant;

fn line() -> (Bodies, Vec<Target>) {
    let mut b = Bodies::default();
    let mut t = Vec::new();
    for (side, sign) in [(Side::A, -1i32), (Side::B, 1i32)] {
        let facing = if side == Side::A {
            Turn(0)
        } else {
            Turn(Turn::HALF)
        };
        for r in 0..75 {
            for f in 0..1000 {
                let pos = Vec2::new(sign * (500 + r * 1_200), (f - 500) * 1_500);
                b.push(Observed {
                    pos,
                    facing,
                    side,
                    ..Default::default()
                });
                t.push(Target {
                    point: pos - Vec2::new(sign * 3_000, 0),
                    speed: 1_300,
                    face: Some(facing),
                    press: if r < 20 { 128 } else { 0 },
                });
            }
        }
    }
    (b, t)
}

fn pocket() -> (Bodies, Vec<Target>) {
    let mut b = Bodies::default();
    let mut t = Vec::new();
    // Sunflower spirals give even packing in a disc and in a ring.
    let golden = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    let inner = 83_000.0;
    for k in 0..60_000 {
        let r = inner * ((k as f64 + 0.5) / 60_000.0).sqrt();
        let a = k as f64 * golden;
        let pos = Vec2::new((r * a.cos()) as i32, (r * a.sin()) as i32);
        b.push(Observed {
            pos,
            side: Side::A,
            ..Default::default()
        });
        t.push(Target {
            point: pos,
            speed: 0,
            face: None,
            press: 128,
        });
    }
    let outer: f64 = 146_000.0;
    for k in 0..90_000 {
        let r2 = (inner + 1_000.0).powi(2)
            + (outer.powi(2) - (inner + 1_000.0).powi(2)) * (k as f64 + 0.5) / 90_000.0;
        let a = k as f64 * golden;
        let pos = Vec2::new((r2.sqrt() * a.cos()) as i32, (r2.sqrt() * a.sin()) as i32);
        b.push(Observed {
            pos,
            side: Side::B,
            ..Default::default()
        });
        t.push(Target {
            point: Vec2::ZERO,
            speed: 1_300,
            face: Turn::of(-pos),
            press: 200,
        });
    }
    (b, t)
}

/// The rout's camp: the gates' centres along the rampart (x = 0), mm.
const GATES: [i32; 3] = [-160_000, 0, 160_000];
const GATE_HALF: i32 = 6_000;
const GATE_DEPTH: i32 = 3_000;
const CAMP_HALF: i32 = 320_000;

fn rout() -> (Bodies, Vec<Target>, Vec<Wall>) {
    let mut walls = Vec::new();
    let mut edges = vec![-CAMP_HALF];
    for g in GATES {
        edges.extend([g - GATE_HALF, g + GATE_HALF]);
        for e in [g - GATE_HALF, g + GATE_HALF] {
            walls.push(Wall::new(Vec2::new(0, e), Vec2::new(-GATE_DEPTH, e)));
        }
    }
    edges.push(CAMP_HALF);
    for pair in edges.chunks(2) {
        walls.push(Wall::new(Vec2::new(0, pair[0]), Vec2::new(0, pair[1])));
    }
    for side in [-CAMP_HALF, CAMP_HALF] {
        walls.push(Wall::new(Vec2::new(0, side), Vec2::new(-640_000, side)));
    }
    let mut b = Bodies::default();
    for r in 0..300 {
        for f in 0..500 {
            b.push(Observed {
                pos: Vec2::new(20_000 + r * 833, (f - 250) * 1_200 + (r % 2) * 600),
                facing: Turn(Turn::HALF),
                ..Default::default()
            });
        }
    }
    let mut t = vec![Target::default(); b.len()];
    rout_targets(&b, &mut t);
    (b, t, walls)
}

/// Each man makes for the mouth of the nearest gate, a body and a hand in from its posts, then
/// through it, then on, and stops 300 m inside.
fn rout_targets(b: &Bodies, t: &mut [Target]) {
    let lane = GATE_HALF - 400;
    for (i, t) in t.iter_mut().enumerate() {
        let p = b.pos[i];
        let g = GATES
            .iter()
            .copied()
            .min_by_key(|g| (p.y - g).abs())
            .unwrap();
        let y = p.y.clamp(g - lane, g + lane);
        let (point, speed) = if p.x >= 0 {
            (Vec2::new(-300, y), 4_000)
        } else if p.x >= -GATE_DEPTH {
            (Vec2::new(-GATE_DEPTH - 5_000, y), 4_000)
        } else if p.x > -300_000 {
            (Vec2::new(p.x - 5_000, p.y), 4_000)
        } else {
            (p, 0)
        };
        *t = Target {
            point,
            speed,
            face: Turn::of(point - p),
            press: if speed > 0 { 255 } else { 0 },
        };
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut config = String::from("line");
    let mut threads = 1usize;
    let mut seconds = 60u32;
    let mut report_every_s = 10u64;
    let mut wall_s = u64::MAX;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--threads" => threads = args.next().and_then(|v| v.parse().ok()).unwrap_or(1),
            "--seconds" => seconds = args.next().and_then(|v| v.parse().ok()).unwrap_or(60),
            "--wall-s" => wall_s = args.next().and_then(|v| v.parse().ok()).unwrap_or(u64::MAX),
            "--report-every-s" => {
                report_every_s = args.next().and_then(|v| v.parse().ok()).unwrap_or(10)
            }
            other => config = other.to_string(),
        }
    }
    let (mut bodies, mut targets, walls) = match config.as_str() {
        "pocket" => {
            let (b, t) = pocket();
            (b, t, Vec::new())
        }
        "line" | "surge" => {
            let (b, t) = line();
            (b, t, Vec::new())
        }
        "rout" => rout(),
        other => {
            eprintln!("unknown configuration {other:?}: line, surge, pocket or rout");
            std::process::exit(2);
        }
    };
    let mut s = Pairwise::new(Params::PLACEHOLDER);
    s.threads = threads;
    s.walls = walls;
    let rng = Rng::new(1);
    let mut out = Vec::new();
    let mut crushed = 0usize;
    let ticks = seconds * 1000 / BASE_TICK.0;
    println!(
        "{config}: {} bodies, {threads} thread(s), {seconds} simulated s",
        bodies.len()
    );
    let start = Instant::now();
    let (mut last_t, mut last_tick) = (start, 0u32);
    let mut done = 0u32;
    for tick in 0..ticks {
        if config == "surge" && cannae_core::schedule::is_man_step(tick) {
            for (i, t) in targets.iter_mut().enumerate() {
                if let Some(f) = t.face {
                    t.point = bodies.pos[i] + f.unit(3_000);
                }
            }
        }
        if config == "rout" && cannae_core::schedule::is_man_step(tick) {
            rout_targets(&bodies, &mut targets);
        }
        s.advance(&mut bodies, &targets, BASE_TICK, &rng, tick, &mut out);
        done = tick + 1;
        crushed += out.len();
        out.clear();
        let now = Instant::now();
        if now.duration_since(last_t).as_secs() >= report_every_s {
            let sim = (tick + 1 - last_tick) as f64 * BASE_TICK.0 as f64 / 1000.0;
            let wall = now.duration_since(last_t).as_secs_f64();
            let p = s.pressure();
            let touching = p.iter().filter(|&&x| x > 0).count();
            let inside = if config == "rout" {
                let n = bodies.pos.iter().filter(|q| q.x < -GATE_DEPTH).count();
                format!("  inside {n}")
            } else {
                String::new()
            };
            println!(
                "  wall {:>5.0} s  sim {:>6.1} s  rate {:>5.2}x  touching {touching}  crushed {}{inside}",
                now.duration_since(start).as_secs_f64(),
                (tick + 1) as f64 * BASE_TICK.0 as f64 / 1000.0,
                sim / wall,
                crushed
            );
            (last_t, last_tick) = (now, tick + 1);
        }
        if now.duration_since(start).as_secs() >= wall_s {
            break;
        }
    }
    let wall = start.elapsed().as_secs_f64();
    let sim = done as f64 * BASE_TICK.0 as f64 / 1000.0;
    println!(
        "done: {sim:.0} simulated s in {wall:.0} s, {:.2}x real time overall, final hash {:016x}",
        sim / wall,
        bodies.state_hash()
    );
}
