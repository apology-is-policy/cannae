//! `cell --substrate pairwise|field --rung 0..3 [--variant base] [--draws D] [--every E]
//! [--sets A,B,..] [--seeds N] [--freeze-seed F] [--jobs J] [--secs S] [--whole yes] [--audit no] [--out DIR]
//! [--nominal yes]`
//!
//! Runs one cell of the experiment: parameter sets 0 to D-1 (`cannae_score::draw`, from the
//! freeze seed F), each on seeds 1 to N of a patch variant (`scenarios/patch/<variant>.toml`),
//! J runs at a time. With `--every E`, only sets 0, E, 2E and so on below D are run (as
//! `relief` and `backed` are). `--sets` runs just the sets listed (the top-up, which `score`
//! names). Prints one line per run and the cost of the
//! batch. A run stops where
//! scoring stops, 15 minutes after a side breaks, unless `--whole yes`.
//!
//! **The audit** (preregistration step 3): sets 0, 20, 40 and so on are also run to the end on
//! every seed, unless `--audit no`, and each such run records what happened after its window.
//!
//! **`--out DIR`** appends each run, as one JSON line, to `DIR/<engine>-r<rung>-<variant>.jsonl`
//! the moment it ends. Started again with the same arguments, it skips the runs the file
//! already holds, so a batch of several nights survives a stop. `score` reads these files.
//!
//! `--nominal yes` runs the one set with everything at its centre or placeholder instead
//! (`draw::nominal`): for looking at a mechanism, never for scoring.
//!
//! Floats here only report wall time; nothing they compute reaches a run.
#![allow(clippy::float_arithmetic)]

use cannae_harness::Scenario;
use cannae_score::draw::{Cell, drawn, nominal};
use cannae_score::{Engine, Record, Run, Spec, run};
use rayon::prelude::*;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Mutex;
use std::time::Instant;

/// Every this many sets, one is audited.
const AUDIT_EVERY: u32 = 20;

fn main() -> ExitCode {
    let mut engine = Engine::Pairwise;
    let mut rung = 1u8;
    let mut variant = "base".to_string();
    let mut seeds = 1u64;
    let mut draws = 1u32;
    let mut every = 1u32;
    let mut only: Option<Vec<u32>> = None;
    let mut freeze_seed = 0u64;
    let mut nominal_set = false;
    let mut jobs = 1usize;
    let mut secs = None;
    let mut stop_at_rout_end = true;
    let mut audit = true;
    let mut out_dir: Option<PathBuf> = None;
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
            "--draws" => draws = v.parse().unwrap_or(1),
            "--every" => every = v.parse().unwrap_or(1).max(1),
            "--sets" => only = Some(v.split(',').filter_map(|d| d.parse().ok()).collect()),
            "--freeze-seed" => freeze_seed = v.parse().unwrap_or(0),
            "--jobs" => jobs = v.parse().unwrap_or(1),
            "--secs" => secs = v.parse().ok(),
            "--whole" => stop_at_rout_end = v != "yes",
            "--audit" => audit = v != "no",
            "--out" => out_dir = Some(PathBuf::from(v)),
            "--nominal" => nominal_set = v == "yes",
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
    let cell = Cell { engine, rung };
    let sets = match if nominal_set {
        nominal(cell).map(|d| vec![d])
    } else if let Some(list) = &only {
        list.iter().map(|&i| drawn(freeze_seed, cell, i)).collect()
    } else {
        (0..draws)
            .step_by(every as usize)
            .map(|i| drawn(freeze_seed, cell, i))
            .collect::<Result<Vec<_>, _>>()
    } {
        Ok(sets) => sets,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::FAILURE;
        }
    };
    let freeze = (!nominal_set).then_some(freeze_seed);
    let (file, done) = match &out_dir {
        Some(dir) => match open_out(dir, cell, &variant, freeze) {
            Ok((f, done)) => (Some(Mutex::new(f)), done),
            Err(e) => {
                eprintln!("{e}");
                return ExitCode::FAILURE;
            }
        },
        None => (None, HashSet::new()),
    };
    // (set, seed, carried to the end)
    let mut todo = Vec::new();
    for d in &sets {
        for seed in 1..=seeds {
            todo.push((d, seed, !stop_at_rout_end));
            if audit && stop_at_rout_end && !nominal_set && d.i % AUDIT_EVERY == 0 {
                todo.push((d, seed, true));
            }
        }
    }
    let planned = todo.len();
    todo.retain(|&(d, seed, whole)| !done.contains(&(d.i, seed, whole)));
    if todo.len() < planned {
        println!(
            "{} of {planned} runs are already in the file; running the other {}",
            planned - todo.len(),
            todo.len()
        );
    }
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(jobs)
        .build()
        .expect("thread pool");
    let started = Instant::now();
    let runs: Vec<Result<Run, String>> = pool.install(|| {
        todo.into_par_iter()
            .map(|(d, seed, whole)| {
                let r = run(
                    &s,
                    d,
                    Spec {
                        seed,
                        duration_s,
                        stop_at_rout_end: !whole,
                    },
                )?;
                print_run(&r);
                if let Some(f) = &file {
                    let rec = Record::new(cell, &variant, freeze, whole, &r);
                    let line = serde_json::to_string(&rec).map_err(|e| e.to_string())? + "\n";
                    let mut f = f.lock().map_err(|e| e.to_string())?;
                    f.write_all(line.as_bytes())
                        .and_then(|()| f.flush())
                        .map_err(|e| e.to_string())?;
                }
                Ok(r)
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
    if ok.is_empty() {
        return ExitCode::SUCCESS;
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

/// The cell's file in `dir`, opened to append, and the runs it already holds. A line cut short
/// by a stop is skipped, and its run is done again.
#[allow(clippy::type_complexity)]
fn open_out(
    dir: &Path,
    cell: Cell,
    variant: &str,
    freeze: Option<u64>,
) -> Result<(File, HashSet<(u32, u64, bool)>), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let path = dir.join(cannae_score::record_file(cell, variant));
    let mut f = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(&path)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let mut text = String::new();
    f.read_to_string(&mut text)
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let mut done = HashSet::new();
    for (n, line) in text.lines().enumerate() {
        match serde_json::from_str::<Record>(line) {
            Ok(r) if r.freeze_seed != freeze => {
                return Err(format!(
                    "{}:{}: runs from freeze seed {:?}, not {freeze:?}",
                    path.display(),
                    n + 1,
                    r.freeze_seed
                ));
            }
            Ok(r) => {
                done.insert((r.draw, r.seed, r.whole));
            }
            Err(e) => eprintln!("{}:{}: skipped ({e})", path.display(), n + 1),
        }
    }
    if !text.is_empty() && !text.ends_with('\n') {
        f.seek(SeekFrom::End(0)).map_err(|e| e.to_string())?;
        f.write_all(b"\n").map_err(|e| e.to_string())?;
    }
    Ok((f, done))
}

fn print_run(r: &Run) {
    let s = &r.summary;
    let at = |t: Option<u32>| t.map_or("-".to_string(), |t| format!("{}", t / 20));
    let broke = s.loser.and_then(|l| s.side_break[l as usize]);
    println!(
        "draw {:>3} seed {:>3}: {} men, {:.2} s wall ({:.0}x), contact {} s, {:?} broke at {} \
         s, {} events, evaluated {} skipped {}, at the end {:?} spanning {:?} mm{}",
        r.draw,
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
        r.audit
            .map_or(String::new(), |a| format!(", after the window {a:?}")),
    );
}

fn usage() -> ExitCode {
    eprintln!(
        "usage: cell --substrate pairwise|field --rung 0..3 [--variant base] [--seeds N] \
         [--draws D] [--every E] [--sets A,B,..] [--freeze-seed F] [--jobs J] [--secs S] [--whole yes] \
         [--audit no] [--out DIR] [--nominal yes]"
    );
    ExitCode::FAILURE
}
