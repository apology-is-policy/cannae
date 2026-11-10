//! `cargo run --release -p cannae-man --example run -- <scenario.toml> [--rung 0|1|2]
//!     [--seconds N] [--svg-dir DIR] [--svg-every SECONDS]`
//!
//! Runs a scenario with the rival's man at one rung, real frames and bouts, and the centre of
//! every prior, on the kinematic test substrate (men walk through each other) until the field
//! substrate exists. It is for looking at what the men do, not for scoring: a scored run draws
//! its parameters from the frozen priors and runs on a real substrate.

use cannae_core::power::GaitSpeeds;
use cannae_core::schedule::{BASE_TICK, MAN_EVERY};
use cannae_core::{BoutParams, Bouts, GridPerception, Rng};
use cannae_harness::kinematic::Kinematic;
use cannae_harness::metrics::{Inputs, Recorder, summarize};
use cannae_harness::scenario::{Layout, frames};
use cannae_harness::{Runner, Scenario, place, svg};
use cannae_man::{Men, Params, Rung, Troop};
use std::path::PathBuf;
use std::process::ExitCode;

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut scenario = None;
    let mut rung = Rung::Shaken;
    let mut seconds = None;
    let mut svg_dir: Option<PathBuf> = None;
    let mut svg_every_s: u32 = 10;
    while let Some(a) = args.next() {
        let mut number = |what: &str| -> Result<u32, String> {
            args.next()
                .and_then(|v| v.parse().ok())
                .ok_or(format!("{what} needs a number"))
        };
        match a.as_str() {
            "--rung" => {
                rung = match number("--rung")? {
                    0 => Rung::NoMarks,
                    1 => Rung::HasFled,
                    2 => Rung::Shaken,
                    n => return Err(format!("no rung {n} here: rungs 3 and 4 are Paullus's")),
                }
            }
            "--seconds" => seconds = Some(number("--seconds")?),
            "--svg-every" => svg_every_s = number("--svg-every")?,
            "--svg-dir" => svg_dir = args.next().map(PathBuf::from),
            _ => scenario = Some(PathBuf::from(a)),
        }
    }
    let path = scenario.ok_or(
        "usage: run <scenario.toml> [--rung 0|1|2] [--seconds N] [--svg-dir DIR] \
         [--svg-every SECONDS]",
    )?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let s = Scenario::from_toml(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let troops = s
        .bodies
        .iter()
        .map(|b| Troop::named(&b.troop).ok_or(format!("unknown troop {:?}", b.troop)))
        .collect::<Result<Vec<_>, _>>()?;

    let (bodies, placed) = place(&s);
    let initial = bodies.clone();
    let layout = Layout::of(&placed);
    let mut rec = Recorder::new(&initial, &placed);
    let n = bodies.len();
    let f = frames(&s, &placed, &bodies)?;
    let rng = Rng::new(s.seed);
    let men = Men::new(rung, Params::CENTRE, &bodies, &troops, &rng);
    let bouts = (
        Bouts::new(BoutParams::PLACEHOLDER, men.kits()),
        s.volleys()?,
    );
    let mut r = Runner::new(
        bodies,
        GridPerception::default(),
        men,
        Kinematic::default(),
        f,
        bouts,
        rng,
        GaitSpeeds::PLACEHOLDER,
    );

    let end = seconds.unwrap_or(s.duration_s) * 1000 / BASE_TICK.0;
    let every = (svg_every_s * 1000 / BASE_TICK.0).max(MAN_EVERY);
    if let Some(dir) = &svg_dir {
        std::fs::create_dir_all(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    }
    while r.tick < end {
        if let Some(dir) = &svg_dir
            && r.tick % every == 0
        {
            let name = dir.join(format!("{}-{:06}.svg", s.name, r.tick / 20));
            let title = format!("{} rung {} t={}s", s.name, rung as u8, r.tick / 20);
            std::fs::write(&name, svg::render(&r.bodies, &title))
                .map_err(|e| format!("{}: {e}", name.display()))?;
        }
        if r.step() {
            rec.record(r.tick - 1, &r.bodies);
        }
    }
    let summary = summarize(&Inputs {
        initial: &initial,
        placed: &placed,
        layout: &layout,
        samples: &rec.samples,
        events: r.log.all(),
        end_tick: end,
        break_share: 0.4,
    });
    println!("{summary:#?}");
    println!(
        "{}: rung {}, {} men, {} s, {} events, evaluated {} skipped {}, hash {:016x}",
        s.name,
        rung as u8,
        n,
        end / 20,
        r.log.all().len(),
        r.evaluated,
        r.skipped,
        r.state_hash()
    );
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}
