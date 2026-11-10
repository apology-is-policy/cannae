//! The three lower rungs under the harness runner, with frames and bouts: dormancy stays exact
//! through a fight and a rout, the ladder is nested, and a man who turned for nothing rallies.
//!
//! The troops here are made for short tests, not for any battle. Nothing in this file is a
//! scored run, and nothing here was used to choose a prior.

use cannae_core::power::GaitSpeeds;
use cannae_core::{
    BodyFrames, BoutParams, Bouts, Cause, Event, EventKind, FrameQuery, GridPerception, ManId,
    ManModel, Posture, Rng, Side,
};
use cannae_harness::kinematic::Kinematic;
use cannae_harness::scenario::frames;
use cannae_harness::{Dormancy, Runner, Scenario, place};
use cannae_man::{Men, Params, Rung, Spread, Troop};

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

const ALONE: &str = r#"
    name = "alone"
    seed = 9
    duration_s = 60

    [[body]]
    name = "line"
    side = "A"
    troop = "hastati"
    files = 6
    ranks = 4
    file_mm = 1000
    rank_mm = 1200
    centre_mm = [0, 0]
    facing_deg = 0
    closer = true
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

const RUNGS: [Rung; 3] = [Rung::NoMarks, Rung::HasFled, Rung::Shaken];

type Field = Runner<GridPerception, Men, Kinematic, BodyFrames, Bouts>;

fn field(text: &str, troops: &[Troop], rung: Rung, mode: Dormancy) -> Field {
    let s = Scenario::from_toml(text).unwrap();
    let (bodies, placed) = place(&s);
    let f = frames(&s, &placed, &bodies).unwrap();
    let rng = Rng::new(s.seed);
    let men = Men::new(rung, Params::CENTRE, &bodies, troops, &rng);
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

fn clash(rung: Rung, mode: Dormancy) -> Field {
    field(CLASH, &[EAGER, BRITTLE], rung, mode)
}

fn went(r: &Field, to: Posture) -> Vec<&Event> {
    r.log
        .all()
        .iter()
        .filter(|e| matches!(e.kind, EventKind::Posture { to: t, .. } if t == to))
        .collect()
}

#[test]
fn every_rung_keeps_dormancy_exact_through_a_fight_and_a_rout() {
    for rung in RUNGS {
        let mut skip = clash(rung, Dormancy::Skip);
        let mut all = clash(rung, Dormancy::EvaluateAll);
        let mut check = clash(rung, Dormancy::Check);
        let mut again = clash(rung, Dormancy::Skip);
        for r in [&mut skip, &mut all, &mut check, &mut again] {
            r.run_until(3_600);
        }
        assert_eq!(skip.hashes, all.hashes, "{rung:?}");
        assert_eq!(skip.hashes, check.hashes, "{rung:?}");
        assert_eq!(skip.hashes, again.hashes, "{rung:?}");
        assert!(
            skip.skipped > 1_000,
            "{rung:?}: only {} slept",
            skip.skipped
        );

        // The run was worth checking: men went forward, fought, gave ground, turned and fell.
        let bouts = skip
            .log
            .all()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Bout { .. }))
            .count();
        assert!(bouts > 20, "{rung:?}: {bouts} bouts");
        for to in [
            Posture::Forward,
            Posture::Steady,
            Posture::Back,
            Posture::Turned,
            Posture::Down,
        ] {
            assert!(!went(&skip, to).is_empty(), "{rung:?}: no one went {to:?}");
        }
        // A man who stepped forward, or rallied, can say why.
        for e in skip.log.all() {
            if let EventKind::Posture { from, to } = e.kind
                && (to == Posture::Forward || from == Posture::Turned && to == Posture::Steady)
            {
                assert_ne!(e.causes[0], Cause::None, "{rung:?}: {e:?}");
            }
        }
        // The band broke. (Whether the line holds depends on the rung: in a fight this much
        // deadlier than any prior, men who can be shaken are.)
        let band = (0..skip.bodies.len())
            .filter(|&i| skip.bodies.side[i] == Side::B)
            .filter(|&i| skip.bodies.posture[i] == Posture::Turned)
            .count();
        assert!(band >= 10, "{rung:?}: {band} of the band turned");

        // The marks are kept at their own rungs and nowhere else.
        let n = skip.bodies.len() as u32;
        let fled = (0..n).filter(|&i| skip.model.has_fled(ManId(i))).count();
        // A level of *shaken* lasts five minutes, so any raised in these three is still there.
        let shaken = (0..n)
            .filter(|&i| skip.model.shaken(ManId(i), skip.tick) > 0)
            .count();
        assert_eq!(fled > 0, rung >= Rung::HasFled, "{rung:?}: {fled} fled");
        assert_eq!(shaken > 0, rung >= Rung::Shaken, "{rung:?}: {shaken}");
    }
}

#[test]
fn the_ladder_is_nested() {
    let [r0, r1, r2] = RUNGS.map(|rung| {
        let mut r = clash(rung, Dormancy::Skip);
        r.run_until(3_600);
        r
    });
    // Rungs 0 and 1 are the same battle until the first man turns his back.
    let first_turn = went(&r1, Posture::Turned)[0].tick as usize / 5;
    assert_eq!(r0.hashes[..first_turn], r1.hashes[..first_turn]);
    assert_ne!(r0.hashes[first_turn], r1.hashes[first_turn]);
    // Rungs 1 and 2 are the same battle until the first man is seen to fall: the step after
    // he goes down.
    let first_fall = went(&r2, Posture::Down)[0].tick as usize / 5;
    assert_eq!(r1.hashes[..=first_fall], r2.hashes[..=first_fall]);
    assert_ne!(r1.hashes[first_fall + 1], r2.hashes[first_fall + 1]);
}

#[test]
fn a_different_seed_is_a_different_battle() {
    let mut a = clash(Rung::Shaken, Dormancy::Skip);
    let mut b = clash(Rung::Shaken, Dormancy::Skip);
    b.rng = Rng::new(6);
    a.run_until(2_400);
    b.run_until(2_400);
    assert_ne!(a.hashes.last(), b.hashes.last());
}

/// Men whom one runner cannot move.
const STOLID: Troop = Troop {
    nerve: Spread { mean: 1_024, sd: 0 },
    ..Troop::HASTATI
};

/// A body standing alone, in which one man in the third rank turns his back at one second for
/// no reason anyone else can see.
fn startled(rung: Rung, mode: Dormancy) -> (Field, ManId) {
    let mut r = field(ALONE, &[STOLID], rung, mode);
    let man = ManId(2 * 6 + 2);
    r.run_until(20);
    let i = man.0 as usize;
    let e = Event {
        tick: 20,
        man,
        pos: r.bodies.pos[i],
        kind: EventKind::Posture {
            from: Posture::Steady,
            to: Posture::Turned,
        },
        causes: [Cause::None; 3],
    };
    r.bodies.posture[i] = Posture::Turned;
    r.model.on_event(&e);
    (r, man)
}

#[test]
fn a_man_who_turned_for_nothing_rallies_and_takes_a_place_again() {
    for rung in RUNGS {
        let (mut r, man) = startled(rung, Dormancy::Skip);
        let i = man.0 as usize;
        // Nothing is after him, his standard stands, and the men around him stand.
        r.run_until(240);
        assert_eq!(r.bodies.posture[i], Posture::Steady, "{rung:?}");
        let rally = went(&r, Posture::Steady)[0];
        assert_eq!(rally.man, man);
        assert_eq!(rally.causes[0], Cause::NoPursuer);
        // He walks back to a slot: his own, if the file had not yet closed over it.
        r.run_until(600);
        assert!(r.bodies.in_place[i], "{rung:?}");
        assert_eq!(r.bodies.pos[i], r.frames.slot(man).unwrap().target);
        assert_eq!(r.model.has_fled(man), rung >= Rung::HasFled);
        // No one else moved, and by now everyone sleeps again.
        assert_eq!(
            went(&r, Posture::Back).len() + went(&r, Posture::Turned).len(),
            0
        );
        let before = r.evaluated;
        r.run_until(700);
        assert_eq!(r.evaluated, before);

        let (mut all, _) = startled(rung, Dormancy::EvaluateAll);
        let (mut check, _) = startled(rung, Dormancy::Check);
        all.run_until(700);
        check.run_until(700);
        assert_eq!(r.hashes, all.hashes, "{rung:?}");
        assert_eq!(r.hashes, check.hashes, "{rung:?}");
    }
}
