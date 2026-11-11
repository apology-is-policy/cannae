//! The `relief` variant's drill on both engines: the files of two centuries pass through each
//! other, the fresh one takes the front and the tired one the place behind it. And until the
//! relief is ordered, the variant is the base patch, and the `backed` one. This checks the
//! drill and the scenarios, not any pattern.
//!
//! The passage is run with men who never step forward and never give ground, so that what is
//! seen is the drill and nothing a hazard did. A relief in a real fight is run with the
//! patch's own men made unable to flee, since at the nominal set their line breaks before any
//! lull. Those troops are made for these tests and for no battle, and nothing here was used
//! to choose a prior.

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::Order;
use cannae_core::{
    BodyFrames, BodyId, Bouts, EventKind, FrameQuery, GridPerception, Perceive, Posture, Rng, Side,
    Substrate, Turn, Volleys,
};
use cannae_field::{Field, FieldPerception};
use cannae_harness::scenario::{BodySpec, frames_with};
use cannae_harness::{Dormancy, Runner, Scenario, place};
use cannae_man::{Marks, Men, Rung, Spread, Troop};
use cannae_pairwise::Pairwise;
use cannae_score::draw::{Cell, Drawn, SubstrateParams, nominal};
use cannae_score::{Engine, Record, Spec, run, run_logged};

const RELIEF: &str = include_str!("../../../scenarios/patch/relief.toml");
const BASE: &str = include_str!("../../../scenarios/patch/base.toml");
const BACKED: &str = include_str!("../../../scenarios/patch/backed.toml");

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

/// Who fills a scenario's bodies.
#[derive(Clone, Copy)]
enum Who {
    /// Men whom nothing moves.
    Stolid,
    /// The troops the scenario names.
    Named,
    /// The troops it names, but with a nerve no alarm reaches: nobody flees, so the fight
    /// lasts, and everything else about them is as the nominal set has it.
    Unbroken,
}

/// A scenario under the nominal set.
fn built<P: Perceive + Default, S: Substrate>(
    text: &str,
    d: &Drawn,
    who: Who,
    substrate: S,
    mode: Dormancy,
) -> Drill<P, S> {
    let s = Scenario::from_toml(text).unwrap();
    let (bodies, placed) = place(&s);
    let f = frames_with(&s, &placed, &bodies, |t| d.tie(t)).unwrap();
    let rng = Rng::new(3);
    let named = |b: &BodySpec| d.troop(&b.troop).unwrap();
    let troops: Vec<Troop> = s
        .bodies
        .iter()
        .map(|b| match who {
            Who::Stolid => STOLID,
            Who::Named => named(b),
            Who::Unbroken => Troop {
                nerve: Spread { mean: 8_192, sd: 0 },
                ..named(b)
            },
        })
        .collect();
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

/// The relief patch with its relief allowed from half a minute on and not from twenty, so
/// that a short run can see one.
fn soon() -> String {
    let text = RELIEF.replace("not_before_s = 1200", "not_before_s = 30");
    assert_ne!(text, RELIEF);
    text
}

/// That patch with stolid men in every body, and everything else as the nominal set has it.
fn drill<P: Perceive + Default, S: Substrate>(
    d: &Drawn,
    substrate: S,
    mode: Dormancy,
) -> Drill<P, S> {
    built(&soon(), d, Who::Stolid, substrate, mode)
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

fn ticks_of<P: Perceive, S: Substrate>(r: &Drill<P, S>, what: fn(&EventKind) -> bool) -> Vec<u32> {
    let all = r.log.all().iter();
    all.filter(|e| what(&e.kind)).map(|e| e.tick).collect()
}

fn is_bout(k: &EventKind) -> bool {
    matches!(k, EventKind::Bout { .. })
}

fn is_turn(k: &EventKind) -> bool {
    let to = Posture::Turned;
    matches!(k, EventKind::Posture { to: t, .. } if *t == to)
}

/// `backed` is the comparison for `relief`, so the two must be one battle until a relief is
/// ordered: a relief that waits for its time moves nobody and changes nobody's luck. And with
/// the troops the files name, dormancy stays exact through the fight and the rout.
fn a_waiting_relief_changes_nothing<P: Perceive + Default, S: Substrate>(
    name: &str,
    d: &Drawn,
    substrate: impl Fn() -> S,
) {
    // Two minutes at the nominal set: long enough for the first line to fight and to break,
    // and for its flight to reach the second line; and far short of the twenty minutes before
    // which no relief comes.
    let run = |text: &str, mode: Dormancy| {
        let mut r = built::<P, S>(text, d, Who::Named, substrate(), mode);
        r.run_until(120 * 1000 / BASE_TICK.0);
        r
    };
    let backed = run(BACKED, Dormancy::Skip);
    let relief = run(RELIEF, Dormancy::Skip);
    assert_eq!(backed.hashes, relief.hashes, "{name}");
    assert_eq!(backed.log.all(), relief.log.all(), "{name}");
    assert!(backed.frames.relieved().is_empty(), "{name}");
    assert_eq!(relief.frames.relieved(), [None, None], "{name}");
    for b in 0..5 {
        let order = |r: &Drill<P, S>| r.frames.body(BodyId(b)).unwrap().order;
        assert_eq!(order(&backed), order(&relief), "{name}: body {b}");
    }
    // Allowed from half a minute, still no relief comes at the nominal set: the onset leaves
    // no lull, and after it there is no line to relieve.
    let early = run(&soon(), Dormancy::Skip);
    assert_eq!(early.frames.relieved(), [None, None], "{name}");
    assert_eq!(early.hashes, backed.hashes, "{name}");
    let all = run(RELIEF, Dormancy::EvaluateAll);
    let check = run(RELIEF, Dormancy::Check);
    assert_eq!(relief.hashes, all.hashes, "{name}");
    assert_eq!(relief.hashes, check.hashes, "{name}");
    assert!(relief.skipped > relief.evaluated, "{name}: few slept");

    // The run was worth comparing: men fought, men turned their backs, and the second line,
    // whose men come last in the file, was caught up in it.
    let (bouts, turned) = (ticks_of(&relief, is_bout), ticks_of(&relief, is_turn));
    let second = relief.log.all().iter().filter(|e| e.man.0 >= 322).count();
    assert!(
        bouts.len() > 20 && turned.len() > 20 && second > 20,
        "{name}: {} bouts, {} turned, {second} events in the second line",
        bouts.len(),
        turned.len()
    );
}

/// With the troops the file names, but nobody able to flee, the fight lasts; and then the
/// relief comes as the script means it to. The onset is no lull. The first lull comes when the
/// onset has spent itself, the fresh centuries come up through men who have fought, and the
/// tired ones go back behind them. `after` is how many bouts the fresh line must have fought
/// by the end, once the tired one had halted.
fn the_relief_comes_in_the_first_lull_of_a_fight<P: Perceive + Default, S: Substrate>(
    name: &str,
    d: &Drawn,
    substrate: impl Fn() -> S,
    (end_s, after): (u32, usize),
) {
    let secs = |s: u32| s * 1000 / BASE_TICK.0;
    let run = |mode: Dormancy| {
        let mut r = built::<P, S>(&soon(), d, Who::Unbroken, substrate(), mode);
        r.run_until(secs(end_s));
        r
    };
    let r = run(Dormancy::Skip);
    let body = |b: usize| r.frames.body(BodyId(b as u32)).unwrap();
    // Men on their feet: the fallen lie where the front was.
    let standing = |b: usize| {
        let xs = (0..r.bodies.len())
            .filter(|&i| r.bodies.body[i] == Some(BodyId(b as u32)))
            .filter(|&i| r.bodies.posture[i] != Posture::Down)
            .map(|i| r.bodies.pos[i].x);
        (xs.clone().min().unwrap(), xs.max().unwrap())
    };
    let bouts = ticks_of(&r, is_bout);
    let mut halted = 0;
    for (k, at) in r.frames.relieved().into_iter().enumerate() {
        let at = at.unwrap_or_else(|| panic!("{name}: century {k} was never relieved"));
        // Allowed from half a minute, it waited for the onset to pass: between one minute
        // and three.
        assert!(
            (secs(60)..=secs(180)).contains(&at),
            "{name}: century {k} relieved at tick {at}"
        );
        // The fresh century has the front. The tired one went back once it had come, and
        // stands behind it.
        let (tired, fresh) = (body(k), body(3 + k));
        assert_eq!(
            (fresh.order, fresh.order_tick),
            (Order::Advance, at),
            "{name}"
        );
        assert_eq!(tired.order, Order::Hold, "{name}: century {k}");
        assert!(
            (at + secs(20)..=at + secs(90)).contains(&tired.order_tick),
            "{name}: century {k} halted at tick {}",
            tired.order_tick
        );
        let (old, new) = (standing(k), standing(3 + k));
        assert!(new.0 > old.1 + 5_000, "{name}: {old:?} behind {new:?}");
        halted = halted.max(tired.order_tick);
    }
    let first = r.frames.relieved().into_iter().flatten().min().unwrap();
    let before = bouts.iter().filter(|&&t| t < first).count();
    assert!(before > 50, "{name}: {before} bouts before the relief");
    let fought_on = bouts.iter().filter(|&&t| t > halted).count();
    assert!(fought_on >= after, "{name}: {fought_on} bouts after it");
    assert!(ticks_of(&r, is_turn).is_empty(), "{name}: somebody fled");

    // Dormancy is exact through the fight, the passage and what comes after.
    let (all, check) = (run(Dormancy::EvaluateAll), run(Dormancy::Check));
    assert_eq!(r.hashes, all.hashes, "{name}");
    assert_eq!(r.hashes, check.hashes, "{name}");
}

fn on_the_pairwise_engine(test: impl Fn(&str, &Drawn, &dyn Fn() -> Pairwise)) {
    let d = nominal(Cell {
        engine: Engine::Pairwise,
        rung: 2,
    })
    .unwrap();
    let SubstrateParams::Pairwise(p) = d.substrate else {
        panic!()
    };
    test("pairwise", &d, &|| Pairwise::new(p));
}

fn on_the_field(test: impl Fn(&str, &Drawn, &dyn Fn() -> Field)) {
    let d = nominal(Cell {
        engine: Engine::Field,
        rung: 2,
    })
    .unwrap();
    let SubstrateParams::Field(p) = d.substrate else {
        panic!()
    };
    test("field", &d, &|| Field::new(p));
}

#[test]
fn until_a_relief_is_ordered_the_relief_patch_is_the_backed_one() {
    on_the_pairwise_engine(|name, d, substrate| {
        a_waiting_relief_changes_nothing::<GridPerception, _>(name, d, substrate)
    });
    on_the_field(|name, d, substrate| {
        a_waiting_relief_changes_nothing::<FieldPerception, _>(name, d, substrate)
    });
}

#[test]
fn in_a_fight_that_lasts_the_relief_comes_in_the_first_lull_on_the_pairwise_engine() {
    on_the_pairwise_engine(|name, d, substrate| {
        the_relief_comes_in_the_first_lull_of_a_fight::<GridPerception, _>(
            name,
            d,
            substrate,
            (240, 0),
        )
    });
}

#[test]
fn in_a_fight_that_lasts_the_relief_comes_in_the_first_lull_on_the_field() {
    // Ten minutes: on the field the warband comes on again in the ninth, against the fresh
    // line.
    on_the_field(|name, d, substrate| {
        the_relief_comes_in_the_first_lull_of_a_fight::<FieldPerception, _>(
            name,
            d,
            substrate,
            (600, 20),
        )
    });
}

#[test]
fn a_runs_record_says_when_its_reliefs_were_ordered() {
    let cell = Cell {
        engine: Engine::Pairwise,
        rung: 2,
    };
    let d = nominal(cell).unwrap();
    let spec = Spec {
        seed: 4,
        duration_s: 90,
        stop_at_rout_end: false,
    };
    let of = |text: &str| run(&Scenario::from_toml(text).unwrap(), &d, spec).unwrap();
    assert!(of(BACKED).relieved_s.is_empty());
    assert_eq!(of(RELIEF).relieved_s, [None, None]);
    // With the warband told to stand where it is, nobody fights and the front is quiet from
    // the start: both reliefs are ordered at the first second they may be.
    let quiet = soon().replace("order = \"advance\"", "order = \"hold\"");
    assert_ne!(quiet, soon());
    let r = of(&quiet);
    assert_eq!(r.relieved_s, [Some(30), Some(30)]);
    let rec = Record::new(cell, "relief", None, true, &r);
    assert_eq!(rec.relieved_s, r.relieved_s);
    // To disk and back. A line written before records said this reads as a run with no
    // relief.
    let line = serde_json::to_string(&rec).unwrap();
    assert_eq!(serde_json::from_str::<Record>(&line).unwrap(), rec);
    let mut old: serde_json::Value = serde_json::from_str(&line).unwrap();
    old.as_object_mut().unwrap().remove("relieved_s").unwrap();
    let old: Record = serde_json::from_value(old).unwrap();
    assert!(old.relieved_s.is_empty());
}

#[test]
fn a_runs_record_says_how_often_a_frame_stood_for_its_standard_bearer() {
    let cell = Cell {
        engine: Engine::Pairwise,
        rung: 0,
    };
    let d = nominal(cell).unwrap();
    let spec = Spec {
        seed: 1,
        duration_s: 60,
        stop_at_rout_end: false,
    };
    let of = |text: &str| run(&Scenario::from_toml(text).unwrap(), &d, spec).unwrap();
    // With the warband told to stand where it is, nobody fights, nobody falls and no file
    // closes.
    let quiet = BASE.replace("order = \"advance\"", "order = \"hold\"");
    assert_ne!(quiet, BASE);
    assert_eq!(of(&quiet).held, 0);
    // In this fight the man ahead of a century's standard-bearer is down by the half-minute
    // (seen in the viewer by Michal: the frames once re-formed on the standard-bearer there).
    let r = of(BASE);
    assert!(r.held >= 1, "{}", r.held);
    let rec = Record::new(cell, "base", None, true, &r);
    assert_eq!(rec.held, r.held);
    // To disk and back. A line written before records said this reads as none.
    let line = serde_json::to_string(&rec).unwrap();
    assert_eq!(serde_json::from_str::<Record>(&line).unwrap(), rec);
    let mut old: serde_json::Value = serde_json::from_str(&line).unwrap();
    old.as_object_mut().unwrap().remove("held").unwrap();
    assert_eq!(serde_json::from_value::<Record>(old).unwrap().held, 0);
}
