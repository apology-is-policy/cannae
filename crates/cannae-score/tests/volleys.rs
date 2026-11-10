//! The `volleys` variant does what it says: missiles fall on the Roman line from one minute to
//! three, nobody fights before the warband comes on at six, and what the fire cost is
//! reported as the Romans' losses before contact. This checks the scenario and the metrics'
//! reading of it, not any pattern.

use cannae_core::schedule::BASE_TICK;
use cannae_core::{Cause, EventKind, Side, Wound};
use cannae_harness::{Scenario, place};
use cannae_score::draw::{Cell, nominal};
use cannae_score::{Engine, Spec, run_logged};

#[test]
fn the_fire_falls_on_the_romans_alone_and_the_front_comes_on_after() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/patch/volleys.toml"
    ))
    .unwrap();
    let s = Scenario::from_toml(&text).unwrap();
    let (bodies, _) = place(&s);
    let secs = |t: u32| t * BASE_TICK.0 / 1000;
    for engine in [Engine::Pairwise, Engine::Field] {
        let d = nominal(Cell { engine, rung: 2 }).unwrap();
        let spec = Spec {
            seed: 1,
            duration_s: 600,
            stop_at_rout_end: false,
        };
        let (run, log) = run_logged(&s, &d, spec).unwrap();
        let (mut felled, mut hampered) = (0, 0);
        for e in &log {
            match e.kind {
                EventKind::Wound { to } if e.causes[0] == Cause::Missile => {
                    assert!(
                        (60..180).contains(&secs(e.tick)),
                        "{engine:?}: a missile at {} s",
                        secs(e.tick)
                    );
                    assert_eq!(bodies.side[e.man.0 as usize], Side::A, "{engine:?}");
                    if to >= Wound::Disabled {
                        felled += 1;
                    } else {
                        hampered += 1;
                    }
                }
                EventKind::Bout { .. } => assert!(
                    secs(e.tick) >= 360,
                    "{engine:?}: a bout at {} s",
                    secs(e.tick)
                ),
                _ => {}
            }
        }
        // About 4 and 12 are expected of 162.
        assert!((1..=12).contains(&felled), "{engine:?}: {felled} felled");
        assert!(
            (3..=26).contains(&hampered),
            "{engine:?}: {hampered} hampered"
        );
        // The fire is not contact. The fronts meet after six minutes, or never, if the fire
        // alone broke the line: that scores as a time to break of nothing.
        let sum = &run.summary;
        match sum.contact.map(secs) {
            Some(c) => assert!(c >= 360, "{engine:?}: contact at {c} s"),
            None => assert_eq!(sum.time_to_break_s, Some(0.0), "{engine:?}"),
        }
        // What it cost is the Romans' losses before contact, and the warband's are none.
        if sum.contact.is_some() {
            assert_eq!(sum.losses_before_contact, [felled, 0], "{engine:?}");
        }
    }
}

#[test]
fn over_ten_seeds_the_fire_costs_what_the_script_says() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/patch/volleys.toml"
    ))
    .unwrap();
    let s = Scenario::from_toml(&text).unwrap();
    let engine = Engine::Field;
    let d = nominal(Cell { engine, rung: 0 }).unwrap();
    let (mut felled, mut hampered) = (0, 0);
    for seed in 1..=10 {
        let spec = Spec {
            seed,
            duration_s: 200,
            stop_at_rout_end: false,
        };
        let (_, log) = run_logged(&s, &d, spec).unwrap();
        for e in &log {
            match e.kind {
                EventKind::Wound { to } if to >= Wound::Disabled => felled += 1,
                EventKind::Wound { .. } => hampered += 1,
                _ => {}
            }
        }
    }
    // Of 1,620 men the script fells 40 and hampers 122, less those hampered and then felled.
    // Each band is three and a half standard deviations wide on either side.
    assert!((18..=63).contains(&felled), "{felled} felled");
    assert!((82..=160).contains(&hampered), "{hampered} hampered");
}
