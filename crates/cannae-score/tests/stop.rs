//! A run that stops where scoring stops scores exactly as the same run carried on to the end.

use cannae_core::schedule::BASE_TICK;
use cannae_harness::Scenario;
use cannae_harness::metrics::ROUT_WINDOW_S;
use cannae_score::{Engine, Spec, run};

fn base() -> Scenario {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/patch/base.toml"
    ))
    .unwrap();
    Scenario::from_toml(&text).unwrap()
}

#[test]
fn stopping_at_the_end_of_the_rout_window_changes_no_statistic() {
    let s = base();
    let mut cut_something = false;
    for engine in [Engine::Pairwise, Engine::Field] {
        let spec = |stop| Spec {
            engine,
            rung: 1,
            seed: 1,
            duration_s: 1_500,
            stop_at_rout_end: stop,
        };
        let whole = run(&s, spec(false)).unwrap();
        let cut = run(&s, spec(true)).unwrap();
        // The test means something only if a side broke early enough for the run to stop
        // short, and some whole run went on to log more (the field's rout logs almost nothing
        // once it is under way). At these settings both engines' lines break within the first
        // few minutes; if that changes, pick another seed.
        let broke = whole.summary.side_break.iter().flatten().min().copied();
        let broke = broke.unwrap_or_else(|| panic!("{engine:?}: no side broke"));
        let stop_s = broke * BASE_TICK.0 / 1000 + ROUT_WINDOW_S;
        assert!(
            stop_s < 1_500,
            "{engine:?}: broke too late, at tick {broke}"
        );
        assert!(cut.sim_s < whole.sim_s);
        cut_something |= cut.events < whole.events;
        assert_eq!(cut.summary, whole.summary, "{engine:?}");
    }
    assert!(cut_something, "no event after the window in either engine");
}
