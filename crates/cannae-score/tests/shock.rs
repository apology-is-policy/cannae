//! The `shock` variant does what it says: a band falls on the Roman rear between one minute
//! and three and then leaves, and the warband in front fights no one before six minutes. And
//! the band, which is not a front, starts no contact. This checks the scenario and the metrics'
//! reading of it, not any pattern.

use cannae_core::schedule::BASE_TICK;
use cannae_core::{BodyId, EventKind};
use cannae_harness::{Scenario, place};
use cannae_score::draw::{Cell, nominal};
use cannae_score::{Engine, Spec, run_logged};

#[test]
fn the_rear_band_strikes_and_goes_before_the_front_comes_on() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/patch/shock.toml"
    ))
    .unwrap();
    let s = Scenario::from_toml(&text).unwrap();
    let (bodies, _) = place(&s);
    let body_of = |name: &str| BodyId(s.bodies.iter().position(|b| b.name == name).unwrap() as u32);
    let (warband, band) = (body_of("warband"), body_of("rear band"));
    let secs = |t: u32| t * BASE_TICK.0 / 1000;
    for engine in [Engine::Pairwise, Engine::Field] {
        let d = nominal(Cell { engine, rung: 0 }).unwrap();
        let spec = Spec {
            seed: 1,
            duration_s: 360,
            stop_at_rout_end: false,
        };
        let (_, log) = run_logged(&s, &d, spec).unwrap();
        let mut band_bouts = Vec::new();
        for e in &log {
            if let EventKind::Bout { attacker, .. } = e.kind {
                let of = |m: u32| bodies.body[m as usize];
                let (a, b) = (of(attacker.0), of(e.man.0));
                assert!(
                    a != Some(warband) && b != Some(warband),
                    "{engine:?}: the warband fought at {} s",
                    secs(e.tick)
                );
                if a == Some(band) || b == Some(band) {
                    band_bouts.push(secs(e.tick));
                }
            }
        }
        assert!(!band_bouts.is_empty(), "{engine:?}: the band never struck");
        let (first, last) = (band_bouts[0], *band_bouts.last().unwrap());
        assert!(
            (60..=120).contains(&first),
            "{engine:?}: first bout at {first} s"
        );
        assert!(
            last <= 240,
            "{engine:?}: the band still fighting at {last} s"
        );
    }
}

#[test]
fn contact_is_the_fronts_meeting_not_the_shock() {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/patch/shock.toml"
    ))
    .unwrap();
    let marked = Scenario::from_toml(&text).unwrap();
    // The same scenario with the band taken for a front.
    let unmarked = Scenario::from_toml(&text.replace("front = false", "")).unwrap();
    let secs = |t: u32| t * BASE_TICK.0 / 1000;
    for engine in [Engine::Pairwise, Engine::Field] {
        let d = nominal(Cell { engine, rung: 0 }).unwrap();
        let spec = Spec {
            seed: 1,
            duration_s: 600,
            stop_at_rout_end: false,
        };
        let (fronts, _) = run_logged(&marked, &d, spec).unwrap();
        let (all, _) = run_logged(&unmarked, &d, spec).unwrap();
        // The fronts meet after six minutes, or never, if the shock alone broke the line. A
        // line broken before contact scores a time to break of nothing: at once.
        let contact = fronts.summary.contact.map(secs);
        let broke = fronts
            .summary
            .side_break
            .iter()
            .flatten()
            .min()
            .map(|&t| secs(t));
        match contact {
            Some(c) => assert!(c >= 360, "{engine:?}: contact at {c} s"),
            None => assert!(
                broke.is_some_and(|b| b < 360) && fronts.summary.time_to_break_s == Some(0.0),
                "{engine:?}: no contact, and broke at {broke:?} s"
            ),
        }
        let early = all.summary.contact.map(secs);
        assert!(
            early.is_some_and(|c| c < 180),
            "{engine:?}: unmarked, contact at {early:?} s"
        );
        // Whatever the band cost the Romans is reported as theirs before contact.
        assert!(fronts.summary.losses_before_contact[1] == 0);
    }
}
