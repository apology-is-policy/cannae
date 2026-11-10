//! The S0.7 speed spike for the pairwise substrate: 150,000 bodies, substrate only (no man
//! model yet), measured as simulated seconds per wall second.
//!
//! `cargo run --release -p cannae-pairwise --example spike -- <line|surge|pocket|rout>
//!  [--threads N] [--perceive [grid|field]] [--seconds S] [--wall-s W] [--report-every-s R]`
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
//! `--perceive` adds the perception's work at each man step, as the field spike does: the
//! rebuild, the dormancy test for every man, and a full View for every man with an enemy in
//! his window, on one thread. `grid` (the default) is the core's reference perception, which
//! this engine uses in the scored runs; `field` is Varro's, which depends only on the core and
//! is held to the same View by `cannae-field/tests/perception.rs`, so either can serve either
//! substrate.
//!
//! `--wall-s` stops after that much wall time, for sustained (throttled) readings; it cannot
//! change the state reached at a given tick.
#![allow(clippy::float_arithmetic)]

use cannae_core::schedule::{BASE_TICK, is_man_step};
use cannae_core::view::NoFrames;
use cannae_core::{
    Bodies, EventKind, GridPerception, ManId, Observed, Perceive, Rng, Side, Substrate, Target,
    Turn, Vec2, Wall,
};
use cannae_field::FieldPerception;
use cannae_pairwise::{Pairwise, Params};
use std::time::Instant;

/// What a perception's work costs at one man step, as the field spike times it: the rebuild,
/// the dormancy test for every man, and a View for each man with an enemy near. Adds the
/// nanoseconds of each to `pp` and returns (views, total crowding seen).
fn perceive_step<P: Perceive>(
    p: &mut P,
    bodies: &Bodies,
    tick: u32,
    awake: &mut Vec<ManId>,
    pp: &mut [u64; 3],
) -> (u64, u64) {
    let t0 = Instant::now();
    p.rebuild(bodies, &[], tick, tick.saturating_sub(5));
    let t1 = Instant::now();
    awake.clear();
    for i in 0..bodies.len() {
        let man = ManId(i as u32);
        if p.coarse(bodies, &NoFrames, man).enemy_near {
            awake.push(man);
        }
    }
    let t2 = Instant::now();
    let mut seen = 0u64;
    for &man in awake.iter() {
        let v = std::hint::black_box(p.view(bodies, &NoFrames, man));
        seen += v.crowding as u64;
    }
    pp[0] += (t1 - t0).as_nanos() as u64;
    pp[1] += (t2 - t1).as_nanos() as u64;
    pp[2] += t2.elapsed().as_nanos() as u64;
    (awake.len() as u64, seen)
}

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
    let mut args = std::env::args().skip(1).peekable();
    let mut config = String::from("line");
    let mut threads = 1usize;
    let mut seconds = 60u32;
    let mut report_every_s = 10u64;
    let mut wall_s = u64::MAX;
    let mut perceive: Option<&str> = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--threads" => threads = args.next().and_then(|v| v.parse().ok()).unwrap_or(1),
            "--perceive" => {
                perceive = Some(match args.peek().map(String::as_str) {
                    Some("field") => {
                        args.next();
                        "field"
                    }
                    Some("grid") => {
                        args.next();
                        "grid"
                    }
                    _ => "grid",
                })
            }
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
        "{config}: {} bodies, {threads} thread(s), perception {}, {seconds} simulated s",
        bodies.len(),
        perceive.unwrap_or("off")
    );
    let (mut grid, mut field) = (GridPerception::default(), FieldPerception::default());
    let mut awake: Vec<ManId> = Vec::new();
    let mut pp = [0u64; 3];
    let (mut viewed, mut seen) = (0u64, 0u64);
    let start = Instant::now();
    let (mut last_t, mut last_tick) = (start, 0u32);
    let mut done = 0u32;
    for tick in 0..ticks {
        if let Some(which) = perceive
            && is_man_step(tick)
        {
            let (v, c) = if which == "field" {
                perceive_step(&mut field, &bodies, tick, &mut awake, &mut pp)
            } else {
                perceive_step(&mut grid, &bodies, tick, &mut awake, &mut pp)
            };
            viewed += v;
            seen += c;
        }
        if config == "surge" && is_man_step(tick) {
            for (i, t) in targets.iter_mut().enumerate() {
                if let Some(f) = t.face {
                    t.point = bodies.pos[i] + f.unit(3_000);
                }
            }
        }
        if config == "rout" && is_man_step(tick) {
            rout_targets(&bodies, &mut targets);
        }
        s.advance(&mut bodies, &targets, BASE_TICK, &rng, tick, &mut out);
        done = tick + 1;
        crushed += out
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Wound { .. }))
            .count();
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
                "  wall {:>5.0} s  sim {:>6.1} s  rate {:>5.2}x  touching {touching}  crushed {}{inside}  views {viewed}",
                now.duration_since(start).as_secs_f64(),
                (tick + 1) as f64 * BASE_TICK.0 as f64 / 1000.0,
                sim / wall,
                crushed
            );
            if perceive.is_some() {
                let ms = |v: u64| v as f64 / 1e6;
                println!(
                    "    perception, ms: rebuild {:.0}, coarse {:.0}, views {:.0}",
                    ms(pp[0]),
                    ms(pp[1]),
                    ms(pp[2])
                );
                pp = [0; 3];
            }
            (last_t, last_tick) = (now, tick + 1);
        }
        if now.duration_since(start).as_secs() >= wall_s {
            break;
        }
    }
    let wall = start.elapsed().as_secs_f64();
    let sim = done as f64 * BASE_TICK.0 as f64 / 1000.0;
    println!(
        "done: {sim:.0} simulated s in {wall:.0} s, {:.2}x real time overall, views {viewed} \
         (check {seen}), final hash {:016x}",
        sim / wall,
        bodies.state_hash()
    );
}
