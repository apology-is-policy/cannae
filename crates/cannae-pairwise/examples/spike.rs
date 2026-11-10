//! The S0.7 speed spike for the pairwise substrate: 150,000 bodies, substrate only (no man
//! model yet), measured as simulated seconds per wall second.
//!
//! `cargo run --release -p cannae-pairwise --example spike -- <line|pocket> [--threads N]
//!  [--seconds S] [--report-every-s R]`
//!
//! `line`: two armies of 75,000, 1,000 files by 75 ranks each, pressed together along 1.5 km.
//! `pocket`: 60,000 men packed in a disc, 90,000 around them pressing inward.
#![allow(clippy::float_arithmetic)]

use cannae_core::schedule::BASE_TICK;
use cannae_core::{Bodies, Observed, Rng, Side, Substrate, Target, Turn, Vec2};
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

fn main() {
    let mut args = std::env::args().skip(1);
    let mut config = String::from("line");
    let mut threads = 1usize;
    let mut seconds = 60u32;
    let mut report_every_s = 10u64;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--threads" => threads = args.next().and_then(|v| v.parse().ok()).unwrap_or(1),
            "--seconds" => seconds = args.next().and_then(|v| v.parse().ok()).unwrap_or(60),
            "--report-every-s" => {
                report_every_s = args.next().and_then(|v| v.parse().ok()).unwrap_or(10)
            }
            other => config = other.to_string(),
        }
    }
    let (mut bodies, targets) = match config.as_str() {
        "pocket" => pocket(),
        _ => line(),
    };
    let mut s = Pairwise::new(Params::PLACEHOLDER);
    s.threads = threads;
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
    for tick in 0..ticks {
        s.advance(&mut bodies, &targets, BASE_TICK, &rng, tick, &mut out);
        crushed += out.len();
        out.clear();
        let now = Instant::now();
        if now.duration_since(last_t).as_secs() >= report_every_s {
            let sim = (tick + 1 - last_tick) as f64 * BASE_TICK.0 as f64 / 1000.0;
            let wall = now.duration_since(last_t).as_secs_f64();
            let p = s.pressure();
            let touching = p.iter().filter(|&&x| x > 0).count();
            println!(
                "  wall {:>5.0} s  sim {:>6.1} s  rate {:>5.2}x  touching {touching}  crushed {}",
                now.duration_since(start).as_secs_f64(),
                (tick + 1) as f64 * BASE_TICK.0 as f64 / 1000.0,
                sim / wall,
                crushed
            );
            (last_t, last_tick) = (now, tick + 1);
        }
    }
    let wall = start.elapsed().as_secs_f64();
    println!(
        "done: {:.2}x real time overall, final hash {:016x}",
        seconds as f64 / wall,
        bodies.state_hash()
    );
}
