//! `cargo run --release -p cannae-score --bin table -- --freeze-seed N [--draws 200]`
//!
//! Writes the table of record to standard output: one row for each parameter set and one
//! column for each line of the priors files, in each line's stored unit. It is written and
//! hashed at the freeze ([preregistration](../../../../docs/slice0/preregistration.md)).

use cannae_score::draw::table;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse::<u64>().ok())
    };
    let Some(seed) = arg("--freeze-seed") else {
        eprintln!("usage: table --freeze-seed N [--draws 200]");
        return ExitCode::FAILURE;
    };
    print!("{}", table(seed, arg("--draws").unwrap_or(200) as u32));
    ExitCode::SUCCESS
}
