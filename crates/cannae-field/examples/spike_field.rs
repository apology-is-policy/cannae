//! The speed spike for the field substrate: 150,000 bodies on one thread, measured as simulated
//! seconds per wall second. The three scenes are those of the pairwise spike, body for body.
//!
//! `cargo run --release -p cannae-field --example spike_field -- <line|surge|pocket>
//!  [--contact line|crowd|all] [--perceive] [--seconds S] [--wall-s W] [--report-every-s R]`
//!
//! `line`: two armies of 75,000, 1,000 files by 75 ranks each, pressed together along 1.5 km.
//! `surge`: the same, but every man is re-targeted 3 m ahead of where he stands at each man
//! step, so the jam never settles.
//! `pocket`: 60,000 men packed in a disc, 90,000 around them pressing inward.
//!
//! No man model runs, and no man is in his place, so every body is on the field: this is the
//! substrate's worst case. `--perceive` adds the perception's work at each man step: the
//! rebuild, the dormancy test for every man, and a full View for every man with an enemy in
//! his window.
//!
//! `--wall-s` stops after that much wall time, for sustained (throttled) readings; it cannot
//! change the state reached at a given tick.
#![allow(clippy::float_arithmetic)]

use cannae_core::schedule::{BASE_TICK, is_man_step};
use cannae_core::view::NoFrames;
use cannae_core::{Bodies, ManId, Observed, Perceive, Rng, Side, Substrate, Target, Turn, Vec2};
use cannae_field::{Contact, Field, FieldPerception, Params};
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
    let mut contact = Contact::Line;
    let mut perceive = false;
    let mut seconds = 60u32;
    let mut report_every_s = 10u64;
    let mut wall_s = u64::MAX;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--contact" => {
                contact = match args.next().as_deref() {
                    Some("line") => Contact::Line,
                    Some("crowd") => Contact::Crowd,
                    Some("all") => Contact::All,
                    other => {
                        eprintln!("unknown contact {other:?}: line, crowd or all");
                        std::process::exit(2);
                    }
                }
            }
            "--perceive" => perceive = true,
            "--seconds" => seconds = args.next().and_then(|v| v.parse().ok()).unwrap_or(60),
            "--wall-s" => wall_s = args.next().and_then(|v| v.parse().ok()).unwrap_or(u64::MAX),
            "--report-every-s" => {
                report_every_s = args.next().and_then(|v| v.parse().ok()).unwrap_or(10)
            }
            other => config = other.to_string(),
        }
    }
    let (mut bodies, mut targets) = match config.as_str() {
        "pocket" => pocket(),
        "line" | "surge" => line(),
        other => {
            eprintln!("unknown configuration {other:?}: line, surge or pocket");
            std::process::exit(2);
        }
    };
    let mut s = Field::new(Params {
        contact,
        ..Params::CENTRE
    });
    let mut p = FieldPerception::default();
    let rng = Rng::new(1);
    let mut out = Vec::new();
    let mut crushed = 0usize;
    let mut viewed = 0u64;
    let mut seen = 0u64;
    let ticks = seconds * 1000 / BASE_TICK.0;
    println!(
        "{config}: {} bodies, 1 thread, contact {contact:?}, perception {}, {seconds} simulated s",
        bodies.len(),
        if perceive { "on" } else { "off" }
    );
    let start = Instant::now();
    let (mut last_t, mut last_tick) = (start, 0u32);
    let mut done = 0u32;
    let mut peak = 0u32;
    for tick in 0..ticks {
        if is_man_step(tick) {
            if perceive {
                p.rebuild(&bodies, &[], tick, tick.saturating_sub(5));
                for i in 0..bodies.len() {
                    let man = ManId(i as u32);
                    if p.coarse(&bodies, &NoFrames, man).enemy_near {
                        let v = p.view(&bodies, &NoFrames, man);
                        seen += v.crowding as u64;
                        viewed += 1;
                    }
                }
            }
            if config == "surge" {
                for (i, t) in targets.iter_mut().enumerate() {
                    if let Some(f) = t.face {
                        t.point = bodies.pos[i] + f.unit(3_000);
                    }
                }
            }
        }
        s.advance(&mut bodies, &targets, BASE_TICK, &rng, tick, &mut out);
        done = tick + 1;
        crushed += out.len();
        out.clear();
        if !is_man_step(tick) {
            continue;
        }
        peak = peak.max(s.peak_q8());
        let now = Instant::now();
        if now.duration_since(last_t).as_secs() >= report_every_s {
            let sim = (tick + 1 - last_tick) as f64 * BASE_TICK.0 as f64 / 1000.0;
            let wall = now.duration_since(last_t).as_secs_f64();
            let st = s.stats();
            println!(
                "  wall {:>5.0} s  sim {:>6.1} s  rate {:>6.2}x  free {}  hot {}  passes {}  \
                 tiles {}  densest {:.1} (peak {:.1}) per m2  crushed {}  views {}",
                now.duration_since(start).as_secs_f64(),
                (tick + 1) as f64 * BASE_TICK.0 as f64 / 1000.0,
                sim / wall,
                st.free,
                st.hot,
                st.passes,
                st.tiles,
                s.peak_q8() as f64 / 256.0,
                peak as f64 / 256.0,
                crushed,
                viewed
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
        "done: {sim:.0} simulated s in {wall:.0} s, {:.2}x real time overall, peak {:.1} per m2, \
         crushed {crushed}, views {viewed} (check {seen}), final hash {:016x}",
        sim / wall,
        peak as f64 / 256.0,
        bodies.state_hash()
    );
}
