//! `score --substrate pairwise|field --rung 0..3 --dir DIR [--seeds 10]`
//!
//! Scores one cell from the records `cell --out DIR` wrote for each variant
//! (`cannae_score::report`), against the frozen bands in `scenarios/prereg.toml`.

use cannae_harness::metrics::Bands;
use cannae_score::Engine;
use cannae_score::draw::Cell;
use cannae_score::report::Cellbook;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut engine = None;
    let mut rung = None;
    let mut dir = None;
    let mut seeds = 10usize;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let v = args.next().unwrap_or_default();
        match a.as_str() {
            "--substrate" => {
                engine = match v.as_str() {
                    "pairwise" => Some(Engine::Pairwise),
                    "field" => Some(Engine::Field),
                    _ => None,
                }
            }
            "--rung" => rung = v.parse().ok(),
            "--dir" => dir = Some(PathBuf::from(v)),
            "--seeds" => seeds = v.parse().unwrap_or(seeds),
            _ => return usage(),
        }
    }
    let (Some(engine), Some(rung), Some(dir)) = (engine, rung, dir) else {
        return usage();
    };
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../scenarios/prereg.toml");
    let bands = match std::fs::read_to_string(path)
        .map_err(|e| e.to_string())
        .and_then(|t| Bands::from_toml(&t).map_err(|e| e.to_string()))
    {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{path}: {e}");
            return ExitCode::FAILURE;
        }
    };
    match Cellbook::read(&dir, Cell { engine, rung }, seeds) {
        Ok(book) => {
            print!("{}", book.score(&bands));
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn usage() -> ExitCode {
    eprintln!("usage: score --substrate pairwise|field --rung 0..3 --dir DIR [--seeds 10]");
    ExitCode::FAILURE
}
