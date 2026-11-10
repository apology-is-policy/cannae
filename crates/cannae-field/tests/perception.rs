//! The field perception against the reference: on random crowds, and through a fight.
//!
//! The troops in the fight are made for short tests, not for any battle. Nothing in this file
//! is a scored run.

use cannae_core::power::GaitSpeeds;
use cannae_core::view::{BodyState, FrameQuery, NoFrames, Order, SlotView, Standard};
use cannae_core::{
    Bodies, BodyFrames, BodyId, BoutParams, Bouts, Cause, Event, EventKind, GridPerception, Key,
    ManId, Observed, Perceive, Posture, Rng, Role, Side, Stream, Turn, Vec2, Wound,
};
use cannae_field::FieldPerception;
use cannae_harness::conformance::views_match;
use cannae_harness::kinematic::Kinematic;
use cannae_harness::scenario::frames;
use cannae_harness::{Dormancy, Runner, Scenario, place};
use cannae_man::{Men, Params, Rung, Spread, Troop};

/// Bodies with standards carried and down, leaders, and one order to charge.
struct Stub;

impl FrameQuery for Stub {
    fn slot(&self, _: ManId) -> Option<SlotView> {
        None
    }

    fn body(&self, b: BodyId) -> Option<BodyState> {
        Some(BodyState {
            standard: if b.0.is_multiple_of(2) {
                Standard::Carried(ManId(b.0 * 7))
            } else {
                Standard::Down(Vec2::new(3_000, -4_000))
            },
            leader: (b.0 != 3).then_some(ManId(b.0 * 11 + 1)),
            order: if b.0 == 1 { Order::Charge } else { Order::Hold },
            order_tick: b.0 * 3,
        })
    }
}

/// A crowd that straddles the corner where four tiles meet at the origin, a thin scatter of
/// men far off in every direction, and a knot of both sides, packed, at another tile corner.
fn crowd(seed: u64) -> (Bodies, Vec<Event>) {
    let rng = Rng::new(seed);
    let draw = |i: u32, what: u32, n: u32| rng.below(n, Key::new(Stream::Setup, i, 0, what));
    let mut b = Bodies::default();
    for i in 0..1_100u32 {
        let pos = match i {
            0..800 => Vec2::new(
                draw(i, 0, 40_000) as i32 - 20_000,
                draw(i, 1, 30_000) as i32 - 15_000,
            ),
            800..1_000 => Vec2::new(
                61_000 + draw(i, 0, 6_000) as i32,
                -67_000 + draw(i, 1, 6_000) as i32,
            ),
            _ => Vec2::new(
                draw(i, 0, 600_000) as i32 - 300_000,
                draw(i, 1, 600_000) as i32 - 300_000,
            ),
        };
        let posture = match draw(i, 2, 20) {
            0..10 => Posture::Steady,
            10..12 => Posture::Forward,
            12..14 => Posture::Back,
            14..17 => Posture::Turned,
            _ => Posture::Down,
        };
        // Some facings sit exactly on the lines where the window's split changes.
        let facing = match draw(i, 3, 8) {
            0 => 8_191,
            1 => 8_192,
            2 => 24_576,
            3 => 40_960,
            4 => 57_344,
            _ => draw(i, 4, 65_536),
        };
        b.push(Observed {
            pos,
            facing: Turn(facing as u16),
            posture,
            side: if draw(i, 5, 2) == 0 { Side::A } else { Side::B },
            role: if draw(i, 6, 12) == 0 {
                Role::Closer
            } else {
                Role::Soldier
            },
            body: (draw(i, 7, 6) < 5).then_some(BodyId(i % 5)),
            in_place: draw(i, 8, 2) == 0,
            ..Default::default()
        });
    }
    let n = b.len() as u32;
    let mut ev = Vec::new();
    let mut push = |man: u32, pos: Vec2, kind: EventKind| {
        ev.push(Event {
            tick: 7,
            man: ManId(man),
            pos,
            kind,
            causes: [Cause::Bout, Cause::None, Cause::None],
        })
    };
    let went = |to| EventKind::Posture {
        from: Posture::Steady,
        to,
    };
    for k in 0..120u32 {
        let man = draw(k, 9, n);
        let pos = match draw(k, 10, 4) {
            // Where he stands now, or where he stood: an empty cell, or far from everyone.
            0 => Vec2::new(
                draw(k, 11, 100_000) as i32 - 50_000,
                draw(k, 12, 100_000) as i32 - 50_000,
            ),
            _ => b.pos[man as usize],
        };
        let to = match draw(k, 13, 5) {
            0 | 1 => Posture::Down,
            2 | 3 => Posture::Turned,
            _ => Posture::Back,
        };
        push(man, pos, went(to));
    }
    // The leader of body 0 falls; the leader of body 2 only turns.
    push(1, b.pos[1], went(Posture::Down));
    push(23, b.pos[23], went(Posture::Turned));
    push(
        5,
        b.pos[5],
        EventKind::Wound {
            to: Wound::Hampered,
        },
    );
    push(14, b.pos[14], EventKind::StandardDown { body: BodyId(2) });
    push(14, b.pos[14], EventKind::StandardUp { body: BodyId(4) });
    (b, ev)
}

#[test]
fn random_crowds_look_the_same_to_both() {
    let mut p = FieldPerception::default();
    for seed in 1..=6 {
        let (b, ev) = crowd(seed);
        assert_eq!(
            views_match(&mut p, &b, &ev, 10, 5, &Stub),
            Ok(()),
            "seed {seed}"
        );
        assert_eq!(
            views_match(&mut p, &b, &[], 0, 0, &NoFrames),
            Ok(()),
            "seed {seed}"
        );
        assert_eq!(
            views_match(&mut p, &b, &ev, 2, 2, &NoFrames),
            Ok(()),
            "seed {seed}"
        );
    }
}

/// What random crowds never land on: men exactly on the lines between cells and between the
/// quarters of a cell, several men on one spot, enemies tied at the eighth nearest, and enemies
/// at exactly the reach.
#[test]
fn men_on_the_lines_tied_and_on_one_spot_look_the_same_to_both() {
    let mut b = Bodies::default();
    let mut put = |x: i32, y: i32, side: Side, k: i32| {
        b.push(Observed {
            pos: Vec2::new(x, y),
            side,
            facing: Turn((k as u16).wrapping_mul(2_048)),
            ..Default::default()
        });
    };
    let chess = |x: i32, y: i32| if (x + y) & 1 == 0 { Side::A } else { Side::B };
    // A metre lattice through the origin with the sides as on a chessboard. Every man stands
    // on a quarter's corner. His enemies are four at 1 m, eight at 2.236 m, of whom four are
    // among his eight nearest, and four at exactly 3 m.
    for y in -12..=12 {
        for x in -12..=12 {
            put(x * 1_000, y * 1_000, chess(x, y), 7 * x + 13 * y);
        }
    }
    // The same at 3 m, far off: a man's only enemies within reach stand exactly at it.
    for y in 0..8 {
        for x in 0..8 {
            put(90_000 + x * 3_000, y * 3_000, chess(x, y), x - 5 * y);
        }
    }
    // Men on one spot: a friend and an enemy on top of a man of the lattice, three enemies
    // together, and a pair at a corner of four tiles.
    for (x, y, side) in [
        (0, 0, Side::A),
        (0, 0, Side::B),
        (3_000, -2_000, Side::A),
        (3_000, -2_000, Side::A),
        (3_000, -2_000, Side::A),
        (64_000, 64_000, Side::A),
        (64_000, 64_000, Side::B),
    ] {
        put(x, y, side, x / 1_000);
    }
    let mut p = FieldPerception::default();
    assert_eq!(views_match(&mut p, &b, &[], 0, 0, &NoFrames), Ok(()));
}

#[test]
fn nobody_at_all_is_no_trouble() {
    let mut p = FieldPerception::default();
    assert_eq!(
        views_match(&mut p, &Bodies::default(), &[], 5, 0, &NoFrames),
        Ok(())
    );
}

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

fn clash<P: Perceive + Default>(mode: Dormancy) -> Runner<P, Men, Kinematic, BodyFrames, Bouts> {
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
    let mut r = Runner::new(
        bodies,
        P::default(),
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

#[test]
fn a_fight_and_a_rout_are_the_same_battle_under_either() {
    let mut reference = clash::<GridPerception>(Dormancy::Skip);
    let mut field = clash::<FieldPerception>(Dormancy::Skip);
    let mut checked = clash::<FieldPerception>(Dormancy::Check);
    let mut candidate = FieldPerception::default();
    while reference.tick < 3_600 {
        if reference.step() && reference.tick % 200 == 1 {
            // Mid-battle snapshots, with whatever happened in the last two seconds as shocks.
            let t = reference.tick;
            let recent = reference.log.range(t.saturating_sub(40), t);
            assert_eq!(
                views_match(
                    &mut candidate,
                    &reference.bodies,
                    recent,
                    t,
                    t.saturating_sub(5),
                    &reference.frames
                ),
                Ok(()),
                "tick {t}"
            );
        }
    }
    field.run_until(3_600);
    checked.run_until(3_600);
    assert_eq!(reference.hashes, field.hashes);
    assert_eq!(reference.hashes, checked.hashes);
    assert_eq!(reference.log.all(), field.log.all());
    assert_eq!(reference.skipped, field.skipped);
    // The run was worth checking: men fell, and men turned.
    let went = |to: Posture| {
        reference
            .log
            .all()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Posture { to: t, .. } if t == to))
            .count()
    };
    assert!(went(Posture::Down) >= 3 && went(Posture::Turned) >= 10);
}
