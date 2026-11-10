//! Runs written to disk come back exactly, the audit sees past the window, and the scorer
//! reads them as the preregistration says.

use cannae_core::Side;
use cannae_harness::Scenario;
use cannae_harness::metrics::Bands;
use cannae_harness::scenario::SideSpec;
use cannae_score::draw::{Cell, nominal};
use cannae_score::report::{Cellbook, ROMANS, VARIANTS};
use cannae_score::{Engine, Record, Spec, run};

fn read(rel: &str) -> String {
    std::fs::read_to_string(format!("{}/../../{rel}", env!("CARGO_MANIFEST_DIR"))).unwrap()
}

#[test]
fn a_record_survives_the_disk_and_the_scorer_reads_it() {
    let s = Scenario::from_toml(&read("scenarios/patch/base.toml")).unwrap();
    let cell = Cell {
        engine: Engine::Field,
        rung: 1,
    };
    let d = nominal(cell).unwrap();
    // Seed 5 at the nominal set breaks early and logs on after the window (tests/stop.rs).
    let spec = |stop| Spec {
        seed: 5,
        duration_s: 1_500,
        stop_at_rout_end: stop,
    };
    let stopped = run(&s, &d, spec(true)).unwrap();
    let whole = run(&s, &d, spec(false)).unwrap();
    assert_eq!(stopped.audit, None);
    let audit = whole.audit.expect("the window closed before the run ended");
    assert!(
        audit.bouts + audit.rallies > 0 || audit.second_break,
        "{audit:?}"
    );

    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("records");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let mut text = String::new();
    for (r, w) in [(&stopped, false), (&whole, true)] {
        let rec = Record::new(cell, "base", Some(7), w, r);
        let line = serde_json::to_string(&rec).unwrap();
        let back: Record = serde_json::from_str(&line).unwrap();
        assert_eq!(back, rec);
        text += &line;
        text.push('\n');
    }
    // A line cut short by a stop is skipped, not fatal.
    text += "{\"engine\":\"Fie";
    let file = dir.join(cannae_score::record_file(cell, "base"));
    std::fs::write(&file, text).unwrap();

    let bands = Bands::from_toml(&read("scenarios/prereg.toml")).unwrap();
    let rep = Cellbook::read(&dir, cell, 1).unwrap().score(&bands);
    assert_eq!(rep.sets.len(), 1);
    assert_eq!(rep.skipped_lines, 1);
    assert_eq!(rep.freeze_seeds, vec![Some(7)]);
    let set = &rep.sets[0];
    // No veterans, recruits, flank or relief on disk: pattern 6 cannot pass with them.
    assert!(set.dose.iter().all(Option::is_none));
    assert!(!set.passes_all);
    let (_, base) = rep.audits.iter().find(|(v, _)| v == "base").unwrap();
    assert_eq!(
        (base.whole, base.closed, base.paired, base.equal),
        (1, 1, 1, 1)
    );
    // The report prints.
    assert!(rep.to_string().contains("All eight: 0 of 1"));
}

#[test]
fn the_roman_line_is_side_a_in_every_variant() {
    for v in VARIANTS {
        let path = format!(
            "{}/../../scenarios/patch/{v}.toml",
            env!("CARGO_MANIFEST_DIR")
        );
        let Ok(text) = std::fs::read_to_string(&path) else {
            continue;
        };
        let s = Scenario::from_toml(&text).unwrap();
        for b in s.bodies.iter().filter(|b| b.front) {
            let gallic = b.troop.starts_with("gallic");
            assert_eq!(gallic, b.side == SideSpec::B, "{v}: {}", b.name);
        }
    }
    assert_eq!(ROMANS, Side::A);
}
