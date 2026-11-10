//! The `relief` variant's drill on both engines: the files of two centuries pass through each
//! other, the fresh one takes the front and the tired one the place behind it. And until the
//! relief is ordered, the variant is the base patch. This checks the drill and the scenario,
//! not any pattern.
//!
//! The passage is run with men who never step forward and never give ground, so that what is
//! seen is the drill and nothing a hazard did. Those troops are made for this test and for no
//! battle, and nothing here was used to choose a prior.

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::Order;
use cannae_core::{
    BodyFrames, BodyId, Bouts, EventKind, FrameQuery, GridPerception, Perceive, Posture, Rng, Side,
    Substrate, Turn, Volleys,
};
use cannae_field::{Field, FieldPerception};
use cannae_harness::scenario::frames_with;
use cannae_harness::{Dormancy, Runner, Scenario, place};
use cannae_man::{Marks, Men, Rung, Spread, Troop};
use cannae_pairwise::Pairwise;
use cannae_score::draw::{Cell, Drawn, SubstrateParams, nominal};
use cannae_score::{Engine, Spec, run_logged};

const RELIEF: &str = include_str!("../../../scenarios/patch/relief.toml");
const BASE: &str = include_str!("../../../scenarios/patch/base.toml");

/// Men whom nothing moves: they follow their slots and do nothing else.
const STOLID: Troop = Troop {
    nerve: Spread { mean: 8_192, sd: 0 },
    ardour: Spread {
        mean: -8_192,
        sd: 0,
    },
    front_ardour: 0,
    leader_ardour: 0,
    ..Troop::HASTATI
};

type Drill<P, S> = Runner<P, Men, S, BodyFrames, (Bouts, Volleys)>;

/// The relief patch with its relief allowed from half a minute on, stolid men in every body,
/// and everything else as the nominal set has it.
fn drill<P: Perceive + Default, S: Substrate>(
    d: &Drawn,
    substrate: S,
    mode: Dormancy,
) -> Drill<P, S> {
    let s =
        Scenario::from_toml(&RELIEF.replace("not_before_s = 600", "not_before_s = 30")).unwrap();
    let (bodies, placed) = place(&s);
    let f = frames_with(&s, &placed, &bodies, |t| d.tie(t)).unwrap();
    let rng = Rng::new(3);
    let troops = vec![STOLID; s.bodies.len()];
    let marks = Marks::new(Rung::Shaken, &d.man, bodies.len());
    let men = Men::with_memory(marks, d.man, &bodies, &troops, &rng);
    let fighting = (Bouts::new(d.bouts, men.kits()), s.volleys().unwrap());
    let mut r = Runner::new(
        bodies,
        P::default(),
        men,
        substrate,
        f,
        fighting,
        rng,
        d.speeds,
    );
    r.dormancy = mode;
    r
}

/// Where a body's men stand along the line of advance: its rearmost and its foremost man.
fn span<P: Perceive, S: Substrate>(r: &Drill<P, S>, body: u32) -> (i32, i32) {
    let xs = (0..r.bodies.len())
        .filter(|&i| r.bodies.body[i] == Some(BodyId(body)))
        .map(|i| r.bodies.pos[i].x);
    (xs.clone().min().unwrap(), xs.max().unwrap())
}

fn the_files_pass<P: Perceive + Default, S: Substrate>(
    name: &str,
    d: &Drawn,
    substrate: impl Fn() -> S,
) {
    let secs = |s: u32| s * 1000 / BASE_TICK.0;
    let order = |r: &Drill<P, S>, body: u32| {
        let st = r.frames.body(BodyId(body)).unwrap();
        (st.order, st.order_tick)
    };
    let mut r = drill::<P, S>(d, substrate(), Dormancy::Skip);
    // Bodies 0 and 1 are the first line, 2 the warband, 3 and 4 the second line.
    r.run_until(secs(30));
    for b in [0, 1, 3, 4] {
        assert_eq!(
            order(&r, b).0,
            Order::Hold,
            "{name}: body {b} before the relief"
        );
    }
    // The warband has come to the standoff, and the lines stand.
    let before = [0, 1, 3, 4].map(|b| span(&r, b));
    let enemy = span(&r, 2);
    // Quiet from the start, so both reliefs are ordered at the first step they may be: the
    // fresh centuries come up, and the tired ones stand until they have.
    r.run_until(secs(30) + 5);
    for (tired, fresh) in [(0, 3), (1, 4)] {
        assert_eq!(order(&r, tired).0, Order::Hold, "{name}");
        assert_eq!(order(&r, fresh), (Order::Advance, secs(30)), "{name}");
    }
    let (mut retired, mut halted) = ([None; 2], [None; 2]);
    while r.tick < secs(90) {
        r.step();
        let now = r.tick * BASE_TICK.0 / 1000;
        // Coming up or falling back, every Roman faces the enemy and stands steady: an
        // ordered retirement is no man's flight and no man's giving ground.
        for i in 0..r.bodies.len() {
            if r.bodies.side[i] == Side::A {
                assert_eq!(r.bodies.facing[i], Turn(0), "{name}: man {i} at {now} s");
                assert_eq!(r.bodies.posture[i], Posture::Steady, "{name}: man {i}");
            }
        }
        for tired in [0, 1] {
            let o = order(&r, tired as u32).0;
            if retired[tired].is_none() && o == Order::Retire {
                retired[tired] = Some(now);
                // The fresh century's front rank is up with the tired one's, which has not
                // moved: the front was never left open.
                let (old, new) = (span(&r, tired as u32), span(&r, 3 + tired as u32));
                assert!(
                    (old.1 - new.1).abs() <= 2_400,
                    "{name}: {old:?} and {new:?}"
                );
                assert!((old.1 - before[tired].1).abs() <= 600, "{name}: {old:?}");
            }
            if retired[tired].is_some() && halted[tired].is_none() && o == Order::Hold {
                halted[tired] = Some(now);
            }
        }
    }
    // Each fresh century took under a minute to come up, and each tired one under a minute
    // to fall back.
    for tired in [0, 1] {
        let up = retired[tired].unwrap_or_else(|| panic!("{name}: century {tired} never went"));
        let at = halted[tired].unwrap_or_else(|| panic!("{name}: century {tired} never halted"));
        assert!(
            (40..=90).contains(&up),
            "{name}: century {tired} went at {up} s"
        );
        assert!(
            (up + 10..=up + 60).contains(&at),
            "{name}: century {tired} halted at {at} s"
        );
    }
    // Nobody was hurt, nobody fought, and nobody changed his posture: it was a drill.
    assert!(r.log.all().is_empty(), "{name}: {:?}", r.log.all().first());
    let after = [0, 1, 3, 4].map(|b| span(&r, b));
    let warband = span(&r, 2);
    // The warband gained no ground by it.
    assert!(
        (warband.0 - enemy.0).abs() <= 600,
        "{name}: {enemy:?} then {warband:?}"
    );
    for k in [0, 1] {
        let (tired, fresh) = (after[k], after[2 + k]);
        // The tired century stands where the fresh one stood, 24 m back, to within a pace.
        let back = before[k].0 - tired.0;
        assert!(
            (23_500..=25_000).contains(&back),
            "{name}: went back {back} mm"
        );
        assert!(
            ((tired.1 - tired.0) - 14_400).abs() <= 1_000,
            "{name}: {tired:?}"
        );
        // The fresh one is in front of it, as deep as it was, and just out of the warband's
        // reach.
        assert!(fresh.0 > tired.1 + 5_000, "{name}: {fresh:?} and {tired:?}");
        assert!(
            ((fresh.1 - fresh.0) - 14_400).abs() <= 1_000,
            "{name}: {fresh:?}"
        );
        let gap = warband.0 - fresh.1;
        assert!(
            (3_000..=4_600).contains(&gap),
            "{name}: {gap} mm from the warband"
        );
    }
    // Every Roman is in his place and standing.
    for i in 0..r.bodies.len() {
        if r.bodies.side[i] == Side::A {
            assert!(
                r.bodies.in_place[i],
                "{name}: man {i} at {:?}",
                r.bodies.pos[i]
            );
            assert_eq!(r.bodies.posture[i], Posture::Steady, "{name}: man {i}");
        }
    }
    // Dormancy is exact through it: men who sleep follow their slots as men awake do.
    let mut all = drill::<P, S>(d, substrate(), Dormancy::EvaluateAll);
    let mut check = drill::<P, S>(d, substrate(), Dormancy::Check);
    all.run_until(secs(90));
    check.run_until(secs(90));
    assert_eq!(r.hashes, all.hashes, "{name}");
    assert_eq!(r.hashes, check.hashes, "{name}");
}

#[test]
fn the_files_of_two_centuries_pass_through_each_other_on_the_pairwise_engine() {
    let d = nominal(Cell {
        engine: Engine::Pairwise,
        rung: 2,
    })
    .unwrap();
    let SubstrateParams::Pairwise(p) = d.substrate else {
        panic!()
    };
    the_files_pass::<GridPerception, _>("pairwise", &d, || Pairwise::new(p));
}

#[test]
fn the_files_of_two_centuries_pass_through_each_other_on_the_field() {
    let d = nominal(Cell {
        engine: Engine::Field,
        rung: 2,
    })
    .unwrap();
    let SubstrateParams::Field(p) = d.substrate else {
        panic!()
    };
    the_files_pass::<FieldPerception, _>("field", &d, || Field::new(p));
}

#[test]
fn until_the_relief_the_first_line_and_the_warband_are_the_base_patch() {
    let (base, relief) = (
        Scenario::from_toml(BASE).unwrap(),
        Scenario::from_toml(RELIEF).unwrap(),
    );
    for engine in [Engine::Pairwise, Engine::Field] {
        let d = nominal(Cell { engine, rung: 2 }).unwrap();
        let spec = Spec {
            seed: 4,
            duration_s: 90,
            stop_at_rout_end: false,
        };
        let (_, alone) = run_logged(&base, &d, spec).unwrap();
        let (_, backed) = run_logged(&relief, &d, spec).unwrap();
        // Until a man turns his back, the same men do the same things at the same ticks,
        // bouts and all, and the second line does nothing. After that the two part, as they
        // should: men in flight run back among the second line, and it sees them.
        let first_turn = alone
            .iter()
            .find(|e| {
                matches!(
                    e.kind,
                    EventKind::Posture {
                        to: Posture::Turned,
                        ..
                    }
                )
            })
            .map_or(u32::MAX, |e| e.tick);
        let until = |log: &[cannae_core::Event]| {
            log.iter()
                .filter(|e| e.tick < first_turn)
                .copied()
                .collect::<Vec<_>>()
        };
        let (alone, backed) = (until(&alone), until(&backed));
        assert_eq!(alone, backed, "{engine:?}");
        let bouts = alone
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Bout { .. }))
            .count();
        assert!(bouts > 0, "{engine:?}: nobody had fought yet");
    }
}
