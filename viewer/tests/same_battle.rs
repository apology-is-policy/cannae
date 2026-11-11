//! The battle the viewer and the QC pass build is the scorer's own: the same events, in the
//! same order, as `cannae_score::run_logged` gives for the same scenario, cell and seed. The
//! viewer's builder repeats the scorer's by hand and wraps the resolver to note what it read,
//! so this is what holds the two together.

use cannae_harness::Scenario;
use cannae_score::draw::{Cell, nominal};
use cannae_score::{Engine, Spec, run_logged};
use cannae_view::{Config, build};

fn same(engine: Engine, rung: u8, variant: &str, seconds: u32) {
    let seed = 3;
    let c = Config {
        engine,
        rung,
        variant: variant.to_string(),
        seed,
    };
    let mut battle = build(&c).expect("the viewer's battle").battle;
    // A base tick is 50 ms.
    while battle.live.tick() < seconds * 20 {
        battle.live.step();
    }
    let path = format!(
        "{}/../scenarios/patch/{variant}.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    let text = std::fs::read_to_string(&path).expect("the scenario");
    let s = Scenario::from_toml(&text).expect("the scenario parses");
    let d = nominal(Cell { engine, rung }).expect("the nominal set");
    let spec = Spec {
        seed,
        duration_s: seconds,
        stop_at_rout_end: false,
    };
    let (run, events) = run_logged(&s, &d, spec).expect("the scorer's run");
    assert_eq!(run.sim_s, seconds, "the scorer ran as long");
    assert!(
        events.len() > 100,
        "{} events: something happened",
        events.len()
    );
    let ours = battle.live.events();
    assert_eq!(ours.len(), events.len(), "as many events");
    if let Some(k) = ours.iter().zip(&events).position(|(a, b)| a != b) {
        panic!(
            "event {k} differs: the viewer's {:?}, the scorer's {:?}",
            ours[k], events[k]
        );
    }
}

#[test]
fn the_field_engine_with_marks() {
    same(Engine::Field, 0, "base", 120);
}

#[test]
fn the_field_engine_with_gauges() {
    same(Engine::Field, 3, "base", 120);
}

#[test]
fn the_pairwise_engine_with_marks() {
    same(Engine::Pairwise, 0, "base", 120);
}

#[test]
fn the_pairwise_engine_with_gauges() {
    same(Engine::Pairwise, 3, "base", 120);
}

/// Long enough for the missiles, which fall from one minute to three, and for the fronts to
/// meet after them: the warband stands until six minutes.
#[test]
fn missiles_beside_the_bouts() {
    same(Engine::Field, 1, "volleys", 600);
}

/// Long enough for the relief, which may be ordered from twenty minutes on.
#[test]
fn a_relief() {
    same(Engine::Field, 2, "relief", 1_500);
}
