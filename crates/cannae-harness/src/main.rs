//! `cannae-harness <scenario.toml> [--svg-dir DIR] [--svg-every SECONDS]`
//!
//! Runs a scenario with the toy man on the kinematic substrate, until the engines exist, and
//! prints the final state hash. With `--svg-dir`, writes a frame every `--svg-every` seconds.

use cannae_core::power::GaitSpeeds;
use cannae_core::schedule::{BASE_TICK, MAN_EVERY};
use cannae_core::view::NoFrames;
use cannae_core::{GridPerception, NoBouts, Rng};
use cannae_harness::kinematic::Kinematic;
use cannae_harness::toy::Toy;
use cannae_harness::{Runner, Scenario, place, svg};
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let mut scenario = None;
    let mut svg_dir: Option<PathBuf> = None;
    let mut svg_every_s: u32 = 10;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--svg-dir" => svg_dir = args.next().map(PathBuf::from),
            "--svg-every" => svg_every_s = args.next().and_then(|v| v.parse().ok()).unwrap_or(10),
            _ => scenario = Some(PathBuf::from(a)),
        }
    }
    let Some(path) = scenario else {
        eprintln!("usage: cannae-harness <scenario.toml> [--svg-dir DIR] [--svg-every SECONDS]");
        return ExitCode::FAILURE;
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}: {e}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let s = match Scenario::from_toml(&text) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}: {e}", path.display());
            return ExitCode::FAILURE;
        }
    };
    let (bodies, _) = place(&s);
    let n = bodies.len();
    let mut r = Runner::new(
        bodies,
        GridPerception::default(),
        Toy::new(n),
        Kinematic::default(),
        NoFrames,
        NoBouts,
        Rng::new(s.seed),
        GaitSpeeds::PLACEHOLDER,
    );
    let end = s.duration_s * 1000 / BASE_TICK.0;
    let every = (svg_every_s * 1000 / BASE_TICK.0).max(MAN_EVERY);
    if let Some(dir) = &svg_dir
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        eprintln!("{}: {e}", dir.display());
        return ExitCode::FAILURE;
    }
    while r.tick < end {
        if let Some(dir) = &svg_dir
            && r.tick % every == 0
        {
            let name = dir.join(format!("{}-{:06}.svg", s.name, r.tick / 20));
            let title = format!("{} t={}s", s.name, r.tick / 20);
            if let Err(e) = std::fs::write(&name, svg::render(&r.bodies, &title)) {
                eprintln!("{}: {e}", name.display());
                return ExitCode::FAILURE;
            }
        }
        r.step();
    }
    println!(
        "{}: {} men, {} s, {} events, evaluated {} skipped {}, hash {:016x}",
        s.name,
        n,
        s.duration_s,
        r.log.all().len(),
        r.evaluated,
        r.skipped,
        r.state_hash()
    );
    ExitCode::SUCCESS
}
