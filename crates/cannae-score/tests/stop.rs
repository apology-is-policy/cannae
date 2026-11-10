//! A run that stops where scoring stops scores exactly as the same run carried on to the end.

use cannae_core::schedule::BASE_TICK;
use cannae_harness::Scenario;
use cannae_harness::metrics::ROUT_WINDOW_S;
use cannae_score::draw::{Cell, drawn};
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
        let d = drawn(0, Cell { engine, rung: 1 }, 0).unwrap();
        for seed in [1, 2] {
            let spec = |stop| Spec {
                seed,
                duration_s: 1_500,
                stop_at_rout_end: stop,
            };
            let whole = run(&s, &d, spec(false)).unwrap();
            let cut = run(&s, &d, spec(true)).unwrap();
            // The test means something only if a side broke early enough for the run to stop
            // short, and some whole run went on to log more (a rout often logs nothing once it
            // is under way). Here every line breaks within the first few minutes, and the
            // field's seed 2 logs on; if that changes, pick other seeds or another draw.
            let broke = whole.summary.side_break.iter().flatten().min().copied();
            let broke = broke.unwrap_or_else(|| panic!("{engine:?} {seed}: no side broke"));
            let stop_s = broke * BASE_TICK.0 / 1000 + ROUT_WINDOW_S;
            assert!(stop_s < 1_500, "{engine:?} {seed}: broke at tick {broke}");
            assert!(cut.sim_s < whole.sim_s);
            cut_something |= cut.events < whole.events;
            assert_eq!(cut.summary, whole.summary, "{engine:?} {seed}");
        }
    }
    assert!(cut_something, "no event after the window in either engine");
}
