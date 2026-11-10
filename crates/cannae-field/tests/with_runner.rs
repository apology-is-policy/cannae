//! The whole of the rival's engine under the harness runner: its perception, its man, its
//! substrate, with the shared frames and bouts.
//!
//! The troops here are made for short tests, not for any battle. Nothing in this file is a
//! scored run, and nothing here was used to choose a prior.

use cannae_core::power::GaitSpeeds;
use cannae_core::{
    BodyFrames, BoutParams, Bouts, Cause, EventKind, GridPerception, Perceive, Posture, Rng,
    Volleys,
};
use cannae_field::{Contact, Field, FieldPerception, Params as FieldParams};
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

type Engine = Runner<FieldPerception, Men, Field, BodyFrames, Bouts>;

fn clash(mode: Dormancy, contact: Contact) -> Engine {
    let s = Scenario::from_toml(CLASH).unwrap();
    let (bodies, placed) = place(&s);
    let f = frames(&s, &placed, &bodies).unwrap();
    let rng = Rng::new(s.seed);
    let men = Men::new(
        Rung::Shaken,
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
    let field = Field::new(FieldParams {
        contact,
        ..FieldParams::CENTRE
    });
    let mut r = Runner::new(
        bodies,
        FieldPerception::default(),
        men,
        field,
        f,
        bouts,
        rng,
        GaitSpeeds::PLACEHOLDER,
    );
    r.dormancy = mode;
    r
}

#[test]
fn the_engine_repeats_itself_and_keeps_dormancy_exact() {
    for contact in [Contact::Line, Contact::Crowd] {
        let mut skip = clash(Dormancy::Skip, contact);
        let mut all = clash(Dormancy::EvaluateAll, contact);
        let mut check = clash(Dormancy::Check, contact);
        let mut again = clash(Dormancy::Skip, contact);
        for r in [&mut skip, &mut all, &mut check, &mut again] {
            r.run_until(3_600);
        }
        assert_eq!(skip.hashes, all.hashes, "{contact:?}");
        assert_eq!(skip.hashes, check.hashes, "{contact:?}");
        assert_eq!(skip.hashes, again.hashes, "{contact:?}");
        assert!(skip.skipped > 1_000, "only {} slept", skip.skipped);
    }
}

/// The patch's `volleys` variant on the field, with the troops it names and its volleys
/// beside the bouts, under either perception.
fn volleys<P: Perceive + Default>(
    mode: Dormancy,
) -> Runner<P, Men, Field, BodyFrames, (Bouts, Volleys)> {
    let s = Scenario::from_toml(include_str!("../../../scenarios/patch/volleys.toml")).unwrap();
    let (bodies, placed) = place(&s);
    let f = frames(&s, &placed, &bodies).unwrap();
    let rng = Rng::new(s.seed);
    let troops: Vec<Troop> = s
        .bodies
        .iter()
        .map(|b| Troop::named(&b.troop).unwrap())
        .collect();
    let men = Men::new(Rung::Shaken, Params::CENTRE, &bodies, &troops, &rng);
    let fighting = (
        Bouts::new(BoutParams::PLACEHOLDER, men.kits()),
        s.volleys().unwrap(),
    );
    let mut r = Runner::new(
        bodies,
        P::default(),
        men,
        Field::new(FieldParams::CENTRE),
        f,
        fighting,
        rng,
        GaitSpeeds::PLACEHOLDER,
    );
    r.dormancy = mode;
    r
}

#[test]
fn under_the_patchs_volleys_dormancy_is_exact_and_both_perceptions_see_the_same() {
    let mut skip = volleys::<FieldPerception>(Dormancy::Skip);
    let mut all = volleys::<FieldPerception>(Dormancy::EvaluateAll);
    let mut check = volleys::<FieldPerception>(Dormancy::Check);
    let mut reference = volleys::<GridPerception>(Dormancy::Skip);
    // One minute of quiet, two of fire, and ten seconds after.
    skip.run_until(3_800);
    all.run_until(3_800);
    check.run_until(3_800);
    reference.run_until(3_800);
    assert_eq!(skip.hashes, all.hashes);
    assert_eq!(skip.hashes, check.hashes);
    assert_eq!(skip.hashes, reference.hashes);
    assert!(skip.skipped > 3 * skip.evaluated, "{} slept", skip.skipped);
    // Men fell to the missiles, and the men about them were shaken by it.
    let falls = skip
        .log
        .all()
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                EventKind::Posture {
                    to: Posture::Down,
                    ..
                }
            )
        })
        .inspect(|e| assert_eq!(e.causes[0], Cause::Missile))
        .count();
    let shaken = (0..skip.bodies.len() as u32)
        .filter(|&m| skip.model.memory.shaken(cannae_core::ManId(m), 3_600) > 0)
        .count();
    assert!(
        falls > 0 && shaken > 10 * falls,
        "{falls} falls, {shaken} shaken"
    );
}

#[test]
fn men_fight_at_arms_length_and_never_stand_on_an_enemy() {
    let mut r = clash(Dormancy::Skip, Contact::Line);
    let guard = FieldParams::CENTRE.guard_mm as i64;
    let mut nearest = i64::MAX;
    while r.tick < 3_600 {
        if !r.step() {
            continue;
        }
        let b = &r.bodies;
        for i in 0..b.len() {
            for j in i + 1..b.len() {
                if b.side[i] != b.side[j]
                    && b.posture[i] != Posture::Down
                    && b.posture[j] != Posture::Down
                {
                    nearest = nearest.min(b.pos[i].dist_sq(b.pos[j]));
                }
            }
        }
    }
    let nearest = (nearest as u64).isqrt() as i64;
    assert!(nearest >= guard - 30, "enemies came within {nearest} mm");
    let count = |f: &dyn Fn(&EventKind) -> bool| r.log.all().iter().filter(|e| f(&e.kind)).count();
    let bouts = count(&|k| matches!(k, EventKind::Bout { .. }));
    let falls = count(&|k| {
        matches!(
            k,
            EventKind::Posture {
                to: Posture::Down,
                ..
            }
        )
    });
    let turns = count(&|k| {
        matches!(
            k,
            EventKind::Posture {
                to: Posture::Turned,
                ..
            }
        )
    });
    assert!(bouts >= 50, "{bouts} bouts");
    assert!(falls >= 3, "{falls} falls");
    assert!(turns >= 10, "{turns} turned");
}
