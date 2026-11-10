//! The `shock` variant does what it says: a band falls on the Roman rear between one minute
//! and three and then leaves, and the warband in front fights no one before six minutes. This
//! checks the scenario, not any pattern.

use cannae_core::schedule::BASE_TICK;
use cannae_core::{BodyId, EventKind};
use cannae_harness::{Scenario, place};
use cannae_score::draw::{Cell, drawn};
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
        let d = drawn(0, Cell { engine, rung: 0 }, 0).unwrap();
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
