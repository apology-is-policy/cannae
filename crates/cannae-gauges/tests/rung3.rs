//! Rung 3 under the harness runner, on Varro's men, with frames and bouts: dormancy stays exact
//! through a fight and a rout, and the rung is nested on rung 1.
//!
//! The battle and troops are those of cannae-man's own ladder tests, made for short tests and
//! not for any battle. Nothing in this file is a scored run, and nothing here was used to
//! choose a prior.

use cannae_core::power::GaitSpeeds;
use cannae_core::{
    BodyFrames, BoutParams, Bouts, Event, EventKind, GridPerception, ManId, Posture, Rng, Side,
};
use cannae_gauges::{Params as Gauges, Rung3};
use cannae_harness::kinematic::Kinematic;
use cannae_harness::scenario::frames;
use cannae_harness::{Dormancy, Runner, Scenario, place};
use cannae_man::{Memory, Men, Params, Rung, Spread, Troop};

const CLASH: &str = r#"
    name = "clash"
    seed = 5
    duration_s = 180

    [[body]]
    name = "line"
    side = "A"
    troop = "eager"
    files = 6
    ranks = 4
    file_mm = 1000
    rank_mm = 1200
    centre_mm = [0, 0]
    facing_deg = 0
    closer = true

    [[body]]
    name = "band"
    side = "B"
    troop = "brittle"
    files = 6
    ranks = 4
    file_mm = 1000
    rank_mm = 1200
    centre_mm = [16000, 0]
    facing_deg = 180
    leader = "front-centre"
    tie = "loose"

    [[order]]
    body = "band"
    at_s = 0
    order = "advance"
"#;

/// Men who step out within the first minute.
const EAGER: Troop = Troop {
    ardour: Spread { mean: 768, sd: 128 },
    ..Troop::HASTATI
};

/// Men who come on boldly and have no nerve at all.
const BRITTLE: Troop = Troop {
    nerve: Spread {
        mean: -256,
        sd: 128,
    },
    ardour: Spread { mean: 512, sd: 192 },
    ..Troop::WARBAND
};

type Battle<M> = Runner<GridPerception, Men<M>, Kinematic, BodyFrames, Bouts>;

fn battle<M: Memory>(memory: impl FnOnce(usize) -> M, mode: Dormancy) -> Battle<M> {
    let s = Scenario::from_toml(CLASH).unwrap();
    let (bodies, placed) = place(&s);
    let f = frames(&s, &placed, &bodies).unwrap();
    let rng = Rng::new(s.seed);
    let men = Men::with_memory(
        memory(bodies.len()),
        Params::CENTRE,
        &bodies,
        &[EAGER, BRITTLE],
        &rng,
    );
    // Deadlier than any prior, so that three minutes are enough to see men fall.
    let params = BoutParams {
        rate_q16: 1 << 15,
        ..BoutParams::PLACEHOLDER
    };
    let bouts = Bouts::new(params, men.kits());
    let mut r = Runner::new(
        bodies,
        GridPerception::default(),
        men,
        Kinematic::default(),
        f,
        bouts,
        rng,
        GaitSpeeds::PLACEHOLDER,
    );
    r.dormancy = mode;
    r
}

fn rung3(gauges: Gauges, mode: Dormancy) -> Battle<Rung3> {
    battle(|n| Rung3::new(&Params::CENTRE, gauges, n), mode)
}

fn went<M>(r: &Battle<M>, to: Posture) -> usize {
    r.log
        .all()
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Posture { to: t, .. } if t == to))
        .count()
}

#[test]
fn rung_3_keeps_dormancy_exact_through_a_fight_and_a_rout() {
    let mut runs = [
        Dormancy::Skip,
        Dormancy::EvaluateAll,
        Dormancy::Check,
        Dormancy::Skip,
    ]
    .map(|mode| rung3(Gauges::CENTRE, mode));
    for r in &mut runs {
        r.run_until(3_600);
    }
    let [skip, all, check, again] = &runs;
    assert_eq!(skip.hashes, all.hashes);
    assert_eq!(skip.hashes, check.hashes);
    assert_eq!(skip.hashes, again.hashes);
    assert!(skip.skipped > 1_000, "only {} slept", skip.skipped);

    // The run was worth checking: men fought, turned and fell, and the band broke.
    for to in [Posture::Forward, Posture::Turned, Posture::Down] {
        assert!(went(skip, to) > 0, "no one went {to:?}");
    }
    let band = (0..skip.bodies.len())
        .filter(|&i| skip.bodies.side[i] == Side::B && skip.bodies.posture[i] == Posture::Turned)
        .count();
    assert!(band >= 10, "{band} of the band turned");

    // Both gauges moved, and *has fled* is kept, as at rung 1.
    let n = skip.bodies.len();
    let memory = &skip.model.memory;
    let afraid = (0..n)
        .filter(|&i| memory.gauges.fear_q8(i, skip.tick) > 0)
        .count();
    let worn = (0..n)
        .filter(|&i| memory.gauges.resolve_pm(i, skip.tick) < 1000)
        .count();
    let fled = (0..n)
        .filter(|&i| memory.marks.has_fled(ManId(i as u32)))
        .count();
    assert!(afraid > 0 && worn > 0 && fled > 0, "{afraid} {worn} {fled}");
}

/// Everything anyone could see: the log of events, and the bodies at the end.
fn seen<M>(r: &Battle<M>) -> (Vec<Event>, u64) {
    (r.log.all().to_vec(), r.bodies.state_hash())
}

#[test]
fn rung_3_with_its_gauges_switched_off_is_rung_1() {
    // Nothing kept of a fright and nothing lost with resolve: the same battle as rung 1, event
    // for event. (The step hashes differ, because they cover the gauges' own state.)
    let off = Gauges {
        keep_pm: 0,
        empty_q8: 0,
        ..Gauges::CENTRE
    };
    let mut r3 = rung3(off, Dormancy::Skip);
    let mut r1 = battle(
        |n| cannae_man::Marks::new(Rung::HasFled, &Params::CENTRE, n),
        Dormancy::Skip,
    );
    r3.run_until(3_600);
    r1.run_until(3_600);
    assert_eq!(seen(&r3), seen(&r1));

    // With the gauges at the centres of their priors, fear changes the battle.
    let mut on = rung3(Gauges::CENTRE, Dormancy::Skip);
    on.run_until(3_600);
    assert_ne!(seen(&on), seen(&r1));
}
