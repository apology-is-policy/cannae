//! `cell --substrate pairwise|field --rung 0..3 [--variant base] [--seeds N] [--jobs J]
//! [--secs S] [--whole yes]`
//!
//! Runs one cell of the experiment on seeds 1 to N of a patch variant
//! (`scenarios/patch/<variant>.toml`), J runs at a time, with every parameter at its
//! placeholder or its prior's centre. Prints one line per run and the cost of the batch. A run
//! stops where scoring stops, 15 minutes after a side breaks, unless `--whole yes`.
//!
//! Floats here only report wall time; nothing they compute reaches a run.
#![allow(clippy::float_arithmetic)]

use cannae_harness::Scenario;
use cannae_score::{Engine, Run, Spec, run};
use rayon::prelude::*;
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    let mut engine = Engine::Pairwise;
    let mut rung = 1u8;
    let mut variant = "base".to_string();
    let mut seeds = 1u64;
    let mut jobs = 1usize;
    let mut secs = None;
    let mut stop_at_rout_end = true;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let v = args.next().unwrap_or_default();
        match a.as_str() {
            "--substrate" => {
                engine = match v.as_str() {
                    "pairwise" => Engine::Pairwise,
                    "field" => Engine::Field,
                    _ => return usage(),
                }
            }
            "--rung" => rung = v.parse().unwrap_or(9),
            "--variant" => variant = v,
            "--seeds" => seeds = v.parse().unwrap_or(1),
            "--jobs" => jobs = v.parse().unwrap_or(1),
            "--secs" => secs = v.parse().ok(),
            "--whole" => stop_at_rout_end = v != "yes",
            _ => return usage(),
        }
    }
    let path = format!(
        "{}/../../scenarios/patch/{variant}.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let s = match std::fs::read_to_string(&path)
        .map_err(|e| e.to_string())
        .and_then(|t| Scenario::from_toml(&t).map_err(|e| e.to_string()))
    {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let duration_s = secs.unwrap_or(s.duration_s);
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .expect("thread pool");
    let started = Instant::now();
    let runs: Vec<Result<Run, String>> = pool.install(|| {
        (1..=seeds)
            .into_par_iter()
            .map(|seed| {
                let r = run(
                    &s,
                    Spec {
                        engine,
                        rung,
                        seed,
                        duration_s,
                        stop_at_rout_end,
                    },
                );
                if let Ok(r) = &r {
                    print_run(r);
                }
                r
            })
            .collect()
    });
    let wall = started.elapsed().as_secs_f64();
    let mut ok = Vec::new();
    for r in runs {
        match r {
            Ok(r) => ok.push(r),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        }
    }
    let per_run: f64 = ok.iter().map(|r| r.wall.as_secs_f64()).sum::<f64>() / ok.len() as f64;
    let to_break: Vec<f64> = ok
        .iter()
        .filter_map(|r| {
            r.wall_at_break
                .map(|w| w.as_secs_f64() / r.wall.as_secs_f64())
        })
        .collect();
    let sim = ok.iter().map(|r| r.sim_s as f64).sum::<f64>() / ok.len() as f64;
    println!(
        "{:?} rung {rung} {variant}: {} runs of up to {duration_s} s (a mean {sim:.0} s), {jobs} at \
         a time, in {wall:.1} s ({per_run:.2} s per run, {:.0}x real time per run; {:.2} s per run \
         of throughput)",
        engine,
        ok.len(),
        sim / per_run,
        wall / ok.len() as f64,
    );
    if !to_break.is_empty() {
        println!(
            "{} of {} broke; at the break, a mean {:.0} % of the run's wall time was spent",
            to_break.len(),
            ok.len(),
            100.0 * to_break.iter().sum::<f64>() / to_break.len() as f64
        );
    }
    ExitCode::SUCCESS
}

fn print_run(r: &Run) {
    let s = &r.summary;
    let at = |t: Option<u32>| t.map_or("-".to_string(), |t| format!("{}", t / 20));
    let broke = s.loser.and_then(|l| s.side_break[l as usize]);
    println!(
        "seed {:>3}: {} men, {:.2} s wall ({:.0}x), contact {} s, {:?} broke at {} s, \
         {} events, evaluated {} skipped {}, at the end {:?} spanning {:?} mm",
        r.seed,
        r.men,
        r.wall.as_secs_f64(),
        r.sim_s as f64 / r.wall.as_secs_f64(),
        at(s.contact),
        s.loser,
        at(broke),
        r.events,
        r.evaluated,
        r.skipped,
        r.end_postures,
        r.end_span_mm,
    );
}

fn usage() -> ExitCode {
    eprintln!(
        "usage: cell --substrate pairwise|field --rung 0..3 [--variant base] [--seeds N] \
         [--jobs J] [--secs S] [--whole yes]"
    );
    ExitCode::FAILURE
}
