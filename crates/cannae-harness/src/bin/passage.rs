//! The passage study (S0.8): both passages across gap widths, on one substrate.
//!
//! `cargo run --release -p cannae-harness --bin passage -- [--substrate
//! pairwise|field|kinematic] [--radius MM] [--contact line|crowd|all]`
//!
//! `--radius` is the pairwise engine's disc; `--contact` is which men the field keeps apart by
//! position (its three physics rungs).
#![allow(clippy::float_arithmetic)]

use cannae_core::Wall;
use cannae_field::{Contact, Field};
use cannae_harness::kinematic::Kinematic;
use cannae_harness::passage::{Case, Kind, Outcome, run};
use cannae_pairwise::{Pairwise, Params};

fn cell(o: &Outcome, width_mm: i32) -> String {
    let f = |x: Option<f64>, p: usize| x.map_or("-".to_string(), |v| format!("{v:.p$}"));
    format!(
        "{:>4}/{:<4} {:>6} {:>6} {:>6} {:>6} {:>6.1} {:>4} {:>6}",
        o.passed_ms.len(),
        o.passers,
        f(o.flow(), 2),
        f(o.flow().map(|j| j * 1000.0 / width_mm as f64), 2),
        f(o.t90_s(), 1),
        f(o.longest_gap_s(), 2),
        o.peak_density,
        o.crushed,
        o.holder_max_mm,
    )
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .cloned()
    };
    let substrate = arg("--substrate").unwrap_or_else(|| "pairwise".into());
    let radius: i32 = arg("--radius").map_or(Params::PLACEHOLDER.radius_mm, |r| {
        r.parse().expect("--radius takes mm")
    });
    let contact = match arg("--contact").as_deref() {
        None | Some("line") => Contact::Line,
        Some("crowd") => Contact::Crowd,
        Some("all") => Contact::All,
        Some(other) => {
            eprintln!("unknown contact {other}: line, crowd or all");
            std::process::exit(2);
        }
    };
    let one = |case: &Case| -> Outcome {
        match substrate.as_str() {
            "pairwise" => run(case, |walls: &[Wall]| {
                let mut s = Pairwise::new(Params {
                    radius_mm: radius,
                    ..Params::PLACEHOLDER
                });
                s.walls = walls.to_vec();
                s
            }),
            "field" => run(case, |walls: &[Wall]| {
                Field::with_walls(
                    cannae_field::Params {
                        contact,
                        ..cannae_field::Params::CENTRE
                    },
                    walls,
                )
            }),
            "kinematic" => run(case, |_: &[Wall]| Kinematic::default()),
            other => {
                eprintln!("unknown substrate {other}: pairwise, field or kinematic");
                std::process::exit(2);
            }
        }
    };
    match substrate.as_str() {
        "pairwise" => println!("Substrate pairwise, disc radius {radius} mm"),
        "field" => println!("Substrate field, contact {contact:?}, every prior at its centre"),
        other => println!("Substrate {other}"),
    }
    println!(
        "J and Js: persons per s, and per s per metre of width, from the 10th to the 90th in a \
         hundred; t90: s until nine in ten are through; gap: the longest wait in that stretch, s; \
         density: the densest at the mouth, per m2; crushed; holder: the farthest a man of the \
         line beside the interval was pushed from his slot, mm."
    );
    for (kind, label, seconds, widths) in [
        (
            Kind::Interval,
            "A maniple of 120 (12 files) falls back through an interval; pressed = the enemy \
             follows it up",
            300,
            vec![3_000, 6_000, 9_000, 12_000],
        ),
        (
            Kind::Gate,
            "A crowd of 600 makes for a camp gate; pressed = running and shoving, not walking",
            300,
            vec![3_000, 4_500, 6_000, 9_000, 12_000],
        ),
    ] {
        println!();
        println!("{label}");
        println!(
            "{:>6} {:>7} {:>9} {:>6} {:>6} {:>6} {:>6} {:>6} {:>4} {:>6}",
            "width", "pressed", "through", "J", "Js", "t90", "gap", "dens", "crsh", "holder"
        );
        for &w in &widths {
            for pressed in [false, true] {
                let o = one(&Case {
                    kind,
                    width_mm: w,
                    pressed,
                    seconds,
                });
                println!("{:>4} m {:>7} {}", w as f64 / 1000.0, pressed, cell(&o, w));
            }
        }
    }
    println!();
    println!("Measured, people not pushing: Js about 1.9 (1.6-2.3) from 0.8 to 1.2 m (Seyfried et");
    println!(
        "al. 2009), and constant as the width grows to 5 m (Liao et al., arXiv 2015). Pushing,"
    );
    println!(
        "through doors of 0.69-0.75 m: Js about 4.0 (Garcimartin et al. 2016). Js is capacity"
    );
    println!("only where a jam forms in front of the mouth; a wide gap passes what arrives.");
}
