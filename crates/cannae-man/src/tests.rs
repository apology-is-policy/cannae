//! The man, one View at a time. Every View here is built by hand, so each test says exactly
//! what the man sees.

use super::*;
use cannae_core::view::{Counts, LeaderSeen, NearEnemy, Shocks, SlotView};
use cannae_core::{BodyId, Vec2};

/// Men who are all alike, so that the arithmetic is exact: nerve 2, ardour 0.
const PLAIN: Troop = Troop {
    nerve: Spread { mean: 512, sd: 0 },
    ardour: Spread { mean: 0, sd: 0 },
    front_ardour: 0,
    leader_ardour: 512,
    officer_nerve: 0,
    ..Troop::HASTATI
};

const EDGE: u32 = curve::EDGE_Q16;
const CAP: u32 = Params::CENTRE.react_q16;

fn men(rung: Rung, role: Role) -> Men {
    let mut b = Bodies::default();
    b.push(Observed {
        role,
        body: Some(BodyId(0)),
        ..Default::default()
    });
    Men::new(rung, Params::CENTRE, &b, &[PLAIN], &Rng::new(1))
}

const ME: ManId = ManId(0);

/// A man in the middle of his body, in his place, his standard and his leader standing near
/// him, and nobody else in view.
fn view(posture: Posture) -> View {
    let all = nb::LEFT | nb::RIGHT | nb::FRONT | nb::BEHIND;
    View {
        man: ME,
        tick: 0,
        own: Observed {
            posture,
            body: Some(BodyId(0)),
            in_place: true,
            ..Default::default()
        },
        counts: Counts::default(),
        near: [None; 8],
        crowding: 0,
        shocks: Shocks::default(),
        slot: Some(SlotView {
            target: Vec2::ZERO,
            facing: Turn(0),
            in_place: true,
            slots: all,
            neighbours: all,
        }),
        standard: Some(StandardSeen::Up {
            dist_mm: 3_000,
            posture: Posture::Steady,
        }),
        leader: Some(LeaderSeen {
            posture: Posture::Steady,
            dist_mm: 5_000,
        }),
        closer_behind: false,
        order: Order::Hold,
    }
}

fn set(v: &mut View, g: Group, r: Rel, p: Posture, n: u16) {
    v.counts.0[g as usize][r as usize][p as usize] = n;
}

/// The same view with enemies standing ahead, which wakes him and lets him go forward.
fn facing_the_enemy(posture: Posture) -> View {
    let mut v = view(posture);
    set(&mut v, Group::Ahead, Rel::Enemy, Posture::Steady, 5);
    v
}

fn rate<M: Memory>(m: &Men<M>, v: &View, to: Posture) -> u32 {
    m.explain(ME, v)
        .ways
        .iter()
        .flatten()
        .find(|t| t.to == to)
        .map_or(0, |t| t.rate_q16)
}

fn causes<M: Memory>(m: &Men<M>, v: &View, to: Posture) -> Causes {
    m.explain(ME, v)
        .ways
        .iter()
        .flatten()
        .find(|t| t.to == to)
        .map_or([Cause::None; 3], |t| t.causes)
}

fn near(sector: Sector, posture: Posture, faces_me: bool) -> Option<NearEnemy> {
    Some(NearEnemy {
        id: ManId(9),
        offset: Vec2::new(1_000, 0),
        posture,
        sector,
        faces_me,
    })
}

/// How many times `b` is `a`, in sixteenths of a doubling, to the curve's own rounding.
#[allow(clippy::float_arithmetic)]
fn doublings_x16(a: u32, b: u32) -> i32 {
    ((b as f64 / a as f64).log2() * 16.0).round() as i32
}

#[test]
fn a_man_with_nothing_to_see_has_no_hazards_and_sleeps() {
    let m = men(Rung::Shaken, Role::Soldier);
    let v = view(Posture::Steady);
    assert_eq!(m.explain(ME, &v), Hazards::NONE);
    let calm = Coarse::default();
    assert!(m.dormant(ME, &v.own, &calm, 0));
    // Each thing that can wake him wakes him, and only a steady man sleeps.
    for c in [
        Coarse {
            enemy_near: true,
            ..calm
        },
        Coarse {
            shock_near: true,
            ..calm
        },
        Coarse {
            own_unsteady: true,
            ..calm
        },
        Coarse {
            order: Order::Charge,
            ..calm
        },
        Coarse {
            order: Order::Pursue,
            ..calm
        },
    ] {
        assert!(!m.dormant(ME, &v.own, &c, 0), "{c:?}");
    }
    for p in [Posture::Forward, Posture::Back, Posture::Turned] {
        assert!(!m.dormant(ME, &view(p).own, &calm, 0));
    }
    // Being out of his place, or a new order to advance, moves his feet and not his mind.
    let marching = Coarse {
        out_of_place: true,
        order_new: true,
        order: Order::Advance,
        ..calm
    };
    assert!(m.dormant(ME, &v.own, &marching, 0));
}

#[test]
fn conditions_alone_move_no_one() {
    // His standard is down, his leader is down, the slots beside him are empty: and nothing
    // is happening. He is not happy, but he has no hazard, because nothing he sees is an act.
    let m = men(Rung::NoMarks, Role::Soldier);
    let mut v = view(Posture::Steady);
    v.standard = Some(StandardSeen::Down { dist_mm: 3_000 });
    v.leader = Some(LeaderSeen {
        posture: Posture::Down,
        dist_mm: 5_000,
    });
    v.slot.as_mut().unwrap().neighbours = 0;
    assert_eq!(m.explain(ME, &v), Hazards::NONE);
    // One man of his side giving ground in view is an act, and now the conditions count.
    // Alarm: 0.35 + gaps 2 x 0.5 + standard 2 + leader 1.5, against nerve 2: the edge is
    // passed by 2.85 units, and he gives ground as fast as a man can.
    set(&mut v, Group::Beside, Rel::Own, Posture::Back, 1);
    assert_eq!(rate(&m, &v, Posture::Back), CAP);
    assert_eq!(
        causes(&m, &v, Posture::Back),
        [Cause::StandardDown, Cause::LeaderDown, Cause::EmptySlot]
    );
}

#[test]
fn each_man_who_turns_multiplies_the_hazard_of_giving_ground() {
    let m = men(Rung::NoMarks, Role::Soldier);
    let with = |turned: u16| {
        let mut v = view(Posture::Steady);
        set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, turned);
        rate(&m, &v, Posture::Back)
    };
    // One turned man: alarm 1, less the leader standing (0.75), less nerve 2: 1.75 below the
    // edge, at 2.5 doublings a unit.
    assert_eq!(doublings_x16(EDGE, with(1)), -70);
    // Each further man is 2.5 doublings: about 5.7 times the hazard.
    assert_eq!(doublings_x16(with(1), with(2)), 40);
    assert_eq!(doublings_x16(with(2), with(3)), 40);
    assert_eq!(with(8), CAP);
    let mut v = view(Posture::Steady);
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 2);
    assert_eq!(causes(&m, &v, Posture::Back)[0], Cause::NeighboursTurned);
}

#[test]
fn men_standing_around_him_hold_him() {
    let m = men(Rung::NoMarks, Role::Soldier);
    let mut v = view(Posture::Steady);
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 3);
    let alone = rate(&m, &v, Posture::Back);
    // Four men standing beside him: one unit of support, 2.5 doublings.
    set(&mut v, Group::Beside, Rel::Own, Posture::Steady, 4);
    let beside = rate(&m, &v, Posture::Back);
    assert_eq!(doublings_x16(alone, beside), -40);
    // Ranks behind him hold him as well, and only six men count.
    set(&mut v, Group::Behind, Rel::Own, Posture::Steady, 40);
    let behind = rate(&m, &v, Posture::Back);
    assert_eq!(doublings_x16(beside, behind), -60);
    // A leader who has turned is worth three men turning, and no longer holds anyone.
    v.leader.as_mut().unwrap().posture = Posture::Turned;
    assert_eq!(
        doublings_x16(behind, rate(&m, &v, Posture::Back)),
        (3 * 40) + 30
    );
    // Out of sight, he is neither.
    v.leader.as_mut().unwrap().dist_mm = SIGHT_LEADER_MM + 1;
    assert_eq!(doublings_x16(behind, rate(&m, &v, Posture::Back)), 30);
}

#[test]
fn a_neighbours_opponent_is_not_a_flank_attack() {
    let m = men(Rung::NoMarks, Role::Soldier);
    let mut v = facing_the_enemy(Posture::Steady);
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 2);
    let base = rate(&m, &v, Posture::Back);
    // An enemy beside him who is facing someone else.
    v.near[0] = near(Sector::Left, Posture::Forward, false);
    assert_eq!(rate(&m, &v, Posture::Back), base);
    // The same enemy facing him: 1.5 units.
    v.near[0] = near(Sector::Left, Posture::Forward, true);
    assert_eq!(doublings_x16(base, rate(&m, &v, Posture::Back)), 60);
    assert_eq!(causes(&m, &v, Posture::Back)[1], Cause::EnemyOnFlank);
    // Behind him, whichever way the enemy faces: 3 units, as fast as a man can go.
    v.near[0] = near(Sector::Rear, Posture::Steady, false);
    assert_eq!(rate(&m, &v, Posture::Back), CAP);
    assert_eq!(causes(&m, &v, Posture::Back)[0], Cause::EnemyBehind);
    // An enemy who has turned, or is down, threatens no one.
    v.near[0] = near(Sector::Rear, Posture::Turned, false);
    assert_eq!(rate(&m, &v, Posture::Back), base);
    // An enemy in front counts only when he has stepped forward: half a unit.
    v.near[0] = near(Sector::Front, Posture::Steady, true);
    assert_eq!(rate(&m, &v, Posture::Back), base);
    v.near[0] = near(Sector::Front, Posture::Forward, true);
    assert_eq!(doublings_x16(base, rate(&m, &v, Posture::Back)), 20);
}

#[test]
fn a_man_giving_ground_turns_where_there_is_room_and_no_closer() {
    let m = men(Rung::NoMarks, Role::Soldier);
    let mut v = view(Posture::Back);
    // Two turned ahead of him, four standing beside: alarm 2 - 1 - 0.75 - 2 = -1.75.
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 2);
    set(&mut v, Group::Beside, Rel::Own, Posture::Steady, 4);
    // A man behind him: no room. The margin to turn is -1.75 + 1 = -0.75.
    let hemmed = rate(&m, &v, Posture::Turned);
    assert_eq!(doublings_x16(EDGE, hemmed), -30);
    // The slot behind him empty, or none there at all (the rear rank): 1.5 units easier.
    v.slot.as_mut().unwrap().neighbours &= !nb::BEHIND;
    let room = rate(&m, &v, Posture::Turned);
    assert_eq!(doublings_x16(hemmed, room), 60);
    v.slot.as_mut().unwrap().slots &= !nb::BEHIND;
    assert_eq!(rate(&m, &v, Posture::Turned), room);
    // The closer behind him takes the room away again.
    v.closer_behind = true;
    assert_eq!(rate(&m, &v, Posture::Turned), hemmed);
    // He steadies only when his alarm is 4 units under the edge. Here it is not.
    assert!(rate(&m, &v, Posture::Steady) < EDGE / 30);
    // With the turned men gone and the ranks around him standing, it is.
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 0);
    set(&mut v, Group::Behind, Rel::Own, Posture::Steady, 6);
    assert!(rate(&m, &v, Posture::Steady) > EDGE);
    assert_eq!(causes(&m, &v, Posture::Steady)[0], Cause::RanksBehind);
}

#[test]
fn only_the_bold_lead_and_the_rest_follow() {
    let per_hour = |q16: u32| (q16 as u64 * 3600) >> 16;
    // An ordinary man, with the enemy standing in front of him and no one forward, steps
    // out about once in eighteen hours.
    let soldier = men(Rung::NoMarks, Role::Soldier);
    let v = facing_the_enemy(Posture::Steady);
    assert_eq!(
        doublings_x16(EDGE, rate(&soldier, &v, Posture::Forward)),
        -192
    );
    assert_eq!(per_hour(rate(&soldier, &v, Posture::Forward)), 0);
    // A centurion, two units bolder, about once in seventeen minutes.
    let leader = men(Rung::NoMarks, Role::Leader);
    assert_eq!(
        doublings_x16(EDGE, rate(&leader, &v, Posture::Forward)),
        -96
    );
    assert_eq!(per_hour(rate(&leader, &v, Posture::Forward)), 3);
    assert_eq!(causes(&leader, &v, Posture::Forward)[0], Cause::Ardour);
    // When the leader goes, the soldier who sees him follows within seconds: the leader is
    // one man forward and worth three more.
    let mut led = v;
    set(&mut led, Group::Ahead, Rel::Own, Posture::Forward, 1);
    led.leader.as_mut().unwrap().posture = Posture::Forward;
    assert_eq!(rate(&soldier, &led, Posture::Forward), EDGE);
    assert_eq!(
        causes(&soldier, &led, Posture::Forward),
        [Cause::LeaderForward, Cause::NeighboursForward, Cause::None]
    );
    // With no enemy in view there is nothing to go at, unless he is ordered to charge.
    let mut idle = view(Posture::Steady);
    set(&mut idle, Group::Beside, Rel::Own, Posture::Back, 1);
    assert_eq!(rate(&soldier, &idle, Posture::Forward), 0);
    let mut charge = view(Posture::Steady);
    charge.order = Order::Charge;
    assert_eq!(rate(&soldier, &charge, Posture::Forward), CAP);
    assert_eq!(
        causes(&soldier, &charge, Posture::Forward)[0],
        Cause::OrderCharge
    );
    // A man with a wound that hampers him never steps forward.
    let mut hurt = led;
    hurt.own.wound = Wound::Hampered;
    assert_eq!(rate(&soldier, &hurt, Posture::Forward), 0);
    // Enemy backs draw everyone: four enemies turned ahead are six units.
    let mut backs = v;
    set(&mut backs, Group::Ahead, Rel::Enemy, Posture::Turned, 4);
    assert_eq!(rate(&soldier, &backs, Posture::Forward), CAP);
    assert_eq!(
        causes(&soldier, &backs, Posture::Forward)[0],
        Cause::EnemyBacks
    );
}

fn posture_event(tick: u32, to: Posture) -> Event {
    Event {
        tick,
        man: ME,
        pos: Vec2::ZERO,
        kind: EventKind::Posture {
            from: Posture::Steady,
            to,
        },
        causes: [Cause::None; 3],
    }
}

#[test]
fn a_man_fights_until_his_breath_is_spent_and_gets_it_back_standing() {
    let mut m = men(Rung::NoMarks, Role::Soldier);
    let full = PLAIN.stamina.reserve_us;
    assert_eq!(m.breath(ME, 0), full);
    assert_eq!(m.breath(ME, 100_000), full);
    // He steps forward at tick 100 among four others and behind his leader.
    m.on_event(&posture_event(100, Posture::Forward));
    let after = |seconds: u32| 100 + 20 * seconds;
    let fighting = |seconds: u32| {
        let mut v = facing_the_enemy(Posture::Forward);
        v.tick = after(seconds);
        set(&mut v, Group::Beside, Rel::Own, Posture::Forward, 4);
        v.leader.as_mut().unwrap().posture = Posture::Forward;
        v
    };
    let stop = |m: &Men, seconds: u32| rate(m, &fighting(seconds), Posture::Steady);
    // Fresh, his urge is 7 against a threshold of 3 to stay: he stays.
    assert_eq!(doublings_x16(EDGE, stop(&m, 0)), -192);
    // Fighting costs 1.55 times what he can sustain, and his reserve is 75 s at twice that:
    // it is gone in 136 s.
    assert_eq!(m.breath(ME, after(50)), full - 27_500_000);
    assert!(m.breath(ME, after(136)) > 0);
    assert_eq!(m.breath(ME, after(137)), 0);
    assert_eq!(m.breath(ME, after(500)), 0);
    // The first quarter costs him nothing. Each quarter after it takes two units off his
    // urge: with a third of his breath gone he is two units nearer to stepping back, with
    // more than half gone he is at the threshold, and with a quarter left he steps back as
    // fast as a man can.
    assert_eq!(stop(&m, 30), 1);
    assert_eq!(doublings_x16(EDGE, stop(&m, 50)), -96);
    assert_eq!(stop(&m, 85), EDGE);
    assert_eq!(stop(&m, 110), CAP);
    assert_eq!(
        causes(&m, &fighting(110), Posture::Steady)[0],
        Cause::Breath
    );
    // Out of breath he cannot run, even after the enemy's backs.
    let mut chase = fighting(140);
    chase.near[0] = near(Sector::Front, Posture::Turned, false);
    assert_eq!(m.intent(ME, &chase, Posture::Forward).1, Gait::Walk);
    assert_eq!(m.intent(ME, &chase, Posture::Turned).1, Gait::Walk);
    // He steps back at 120 s with an eighth of his breath left, and stands. After one time
    // constant of standing, 63 % of what he had spent is back.
    let back = after(120);
    m.on_event(&posture_event(back, Posture::Steady));
    assert_eq!(m.breath(ME, back), full - 66_000_000);
    let tau = stamina::recovery_tau_ms(m.power.of(Activity::Stand), &PLAIN.stamina);
    assert!((300_000..500_000).contains(&tau), "{tau} ms");
    let rested_at = back + (tau / BASE_TICK.0 as u64) as u32;
    let regained = (m.breath(ME, rested_at) - (full - 66_000_000)) as u64 * 1000 / 66_000_000;
    assert!((628..637).contains(&regained), "{regained} per mille");
    let mut rested = chase;
    rested.tick = rested_at;
    assert_eq!(m.intent(ME, &rested, Posture::Forward).1, Gait::Run);
    // And while he is winded he is slow to step out again: he now has two thirds of his
    // breath, which still takes two units off the urge of a man who would otherwise follow
    // his leader at once.
    let mut led = facing_the_enemy(Posture::Steady);
    led.tick = rested_at;
    set(&mut led, Group::Ahead, Rel::Own, Posture::Forward, 1);
    led.leader.as_mut().unwrap().posture = Posture::Forward;
    assert_eq!(doublings_x16(EDGE, rate(&m, &led, Posture::Forward)), -96);
}

#[test]
fn the_marks_are_the_only_difference_between_the_rungs() {
    let mut v = view(Posture::Steady);
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 1);
    let mut by_rung = [Rung::NoMarks, Rung::HasFled, Rung::Shaken].map(|r| men(r, Role::Soldier));
    let fresh = rate(&by_rung[0], &v, Posture::Back);
    for m in &by_rung {
        assert_eq!(rate(m, &v, Posture::Back), fresh);
    }
    // Each man turns, and later rallies.
    for m in &mut by_rung {
        m.on_event(&posture_event(10, Posture::Turned));
        m.on_event(&posture_event(50, Posture::Steady));
    }
    assert!(!by_rung[0].has_fled(ME));
    assert!(by_rung[1].has_fled(ME) && by_rung[2].has_fled(ME));
    // At rung 0 he is the man he was. At rungs 1 and 2 he has a unit less nerve.
    v.tick = 20_000;
    let [r0, r1, r2] = [0, 1, 2].map(|i| rate(&by_rung[i], &v, Posture::Back));
    assert_eq!(r0, fresh);
    assert_eq!(doublings_x16(fresh, r1), 40);
    assert_eq!(r2, r1);
    assert_eq!(causes(&by_rung[1], &v, Posture::Back)[1], Cause::HasFled);
    // Two men fall in his view. The fall itself is a pulse, the same at every rung; at rung 2
    // it also leaves him shaken, two levels, each three quarters of a unit.
    let mut shock = v;
    shock.shocks.falls = 2;
    for m in &mut by_rung {
        m.absorb(ME, &shock);
    }
    assert_eq!([0, 1, 2].map(|i| by_rung[i].shaken(ME, 20_000)), [0, 0, 2]);
    v.tick = 20_001;
    assert_eq!(rate(&by_rung[1], &v, Posture::Back), r1);
    assert_eq!(
        doublings_x16(r1, rate(&by_rung[2], &v, Posture::Back)),
        2 * 30
    );
    assert_eq!(causes(&by_rung[2], &v, Posture::Back)[0], Cause::Shaken);
    // It wears off a level every five minutes, and reading it changes nothing.
    let hash = by_rung[2].inner_hash();
    assert_eq!(by_rung[2].shaken(ME, 20_000 + 5_999), 2);
    assert_eq!(by_rung[2].shaken(ME, 20_000 + 6_000), 1);
    assert_eq!(by_rung[2].shaken(ME, 20_000 + 12_000), 0);
    v.tick = 20_000 + 12_000;
    assert_eq!(rate(&by_rung[2], &v, Posture::Back), r1);
    assert_eq!(by_rung[2].inner_hash(), hash);
    // A view without a fright writes nothing either, whenever it comes.
    by_rung[2].absorb(ME, &v);
    assert_eq!(by_rung[2].inner_hash(), hash);
    // A new fright adds to what is left, the fall of the standard counts as one, and three
    // levels are the most a man can carry.
    let mut again = shock;
    again.tick = 20_000 + 6_000;
    again.shocks = Shocks {
        falls: 1,
        standard_fell: true,
        ..Default::default()
    };
    by_rung[2].absorb(ME, &again);
    assert_eq!(by_rung[2].shaken(ME, again.tick), 3);
    again.shocks.falls = 200;
    by_rung[2].absorb(ME, &again);
    assert_eq!(by_rung[2].shaken(ME, again.tick), 3);
    // The three rungs now hold different inner states.
    assert_ne!(by_rung[0].inner_hash(), by_rung[1].inner_hash());
    assert_ne!(by_rung[1].inner_hash(), by_rung[2].inner_hash());
}

#[test]
fn a_turned_man_rallies_clear_of_the_enemy_in_sight_of_his_standard() {
    let m = men(Rung::NoMarks, Role::Soldier);
    // Alone, he has only his nerve: 2 units under the edge, and it takes 3 to rally.
    let mut v = view(Posture::Turned);
    v.leader = None;
    assert_eq!(doublings_x16(EDGE, rate(&m, &v, Posture::Steady)), -40);
    // Among men who stand, he stops.
    set(&mut v, Group::Beside, Rel::Own, Posture::Steady, 6);
    set(&mut v, Group::Behind, Rel::Own, Posture::Steady, 6);
    assert_eq!(doublings_x16(EDGE, rate(&m, &v, Posture::Steady)), 80);
    assert_eq!(
        causes(&m, &v, Posture::Steady),
        [
            Cause::NoPursuer,
            Cause::CarrierStanding,
            Cause::SteadyNeighbours
        ]
    );
    // Among men who run, he runs.
    let mut stream = v;
    set(&mut stream, Group::Beside, Rel::Own, Posture::Turned, 8);
    assert_eq!(rate(&m, &stream, Posture::Steady), 0);
    // With an enemy in his window he does not stop, whoever stands by him.
    let mut chased = v;
    set(&mut chased, Group::Behind, Rel::Enemy, Posture::Forward, 1);
    assert_eq!(m.explain(ME, &chased), Hazards::NONE);
    // Nor when his standard is down, out of sight, or being carried off.
    for standard in [
        None,
        Some(StandardSeen::Down { dist_mm: 3_000 }),
        Some(StandardSeen::Up {
            dist_mm: SIGHT_STANDARD_MM + 1,
            posture: Posture::Steady,
        }),
        Some(StandardSeen::Up {
            dist_mm: 3_000,
            posture: Posture::Turned,
        }),
    ] {
        let mut lost = v;
        lost.standard = standard;
        assert_eq!(m.explain(ME, &lost), Hazards::NONE, "{standard:?}");
    }
    // The man who carries it off can stop: he is his own rallying point.
    let bearer = men(Rung::NoMarks, Role::Carrier);
    let mut his = v;
    his.own.role = Role::Carrier;
    his.standard = Some(StandardSeen::Up {
        dist_mm: 0,
        posture: Posture::Turned,
    });
    assert!(rate(&bearer, &his, Posture::Steady) > EDGE);
}

#[test]
fn each_posture_has_its_way_of_moving() {
    let m = men(Rung::NoMarks, Role::Soldier);
    let hold = |forward_mm| Intent::HoldSlot {
        forward_mm,
        press: 0,
    };
    let v = facing_the_enemy(Posture::Steady);
    assert_eq!(m.intent(ME, &v, Posture::Steady), (hold(0), Gait::Walk));
    assert_eq!(m.intent(ME, &v, Posture::Back), (hold(-1_000), Gait::Walk));
    // Forward with no enemy in reach: a step ahead of his slot. Under a charge, at a run.
    assert_eq!(
        m.intent(ME, &v, Posture::Forward),
        (hold(2_250), Gait::Walk)
    );
    let mut charge = v;
    charge.order = Order::Charge;
    assert_eq!(
        m.intent(ME, &charge, Posture::Forward),
        (hold(2_250), Gait::Run)
    );
    // With an enemy in reach he closes with the nearest who is still up: at a walk if the
    // man faces him, at a run after a back.
    let mut close = v;
    close.near[0] = Some(NearEnemy {
        id: ManId(7),
        posture: Posture::Down,
        ..near(Sector::Front, Posture::Down, false).unwrap()
    });
    close.near[1] = near(Sector::Front, Posture::Steady, true);
    let at = |enemy| Intent::Close {
        enemy,
        stand_off_mm: 1_000,
        press: 0,
    };
    assert_eq!(
        m.intent(ME, &close, Posture::Forward),
        (at(ManId(9)), Gait::Walk)
    );
    close.near[1] = near(Sector::Front, Posture::Turned, false);
    assert_eq!(
        m.intent(ME, &close, Posture::Forward),
        (at(ManId(9)), Gait::Run)
    );
    // Turned, he runs the way his body's back was; with no slot left, the way he faces.
    let mut fled = v;
    fled.slot.as_mut().unwrap().facing = Turn(1_000);
    let away = |heading| Intent::Flee {
        heading: Turn(heading),
    };
    assert_eq!(
        m.intent(ME, &fled, Posture::Turned),
        (away(1_000 + Turn::HALF), Gait::Run)
    );
    fled.slot = None;
    fled.own.facing = Turn(40_000);
    assert_eq!(
        m.intent(ME, &fled, Posture::Turned),
        (away(40_000), Gait::Run)
    );
}

#[test]
fn men_differ_and_the_same_seed_makes_the_same_men() {
    let mut b = Bodies::default();
    for i in 0..400 {
        b.push(Observed {
            role: match i {
                0 => Role::Leader,
                1 => Role::Closer,
                2 => Role::Carrier,
                3..20 => Role::FileLeader,
                _ => Role::Soldier,
            },
            body: Some(BodyId(0)),
            ..Default::default()
        });
    }
    let t = Troop::WARBAND;
    let m = Men::new(Rung::Shaken, Params::CENTRE, &b, &[t], &Rng::new(3));
    let again = Men::new(Rung::Shaken, Params::CENTRE, &b, &[t], &Rng::new(3));
    let other = Men::new(Rung::Shaken, Params::CENTRE, &b, &[t], &Rng::new(4));
    assert_eq!(m.inner_hash(), again.inner_hash());
    assert_ne!(m.inner_hash(), other.inner_hash());
    // The rank and file: mean and spread as the troop says.
    let soldiers: Vec<i64> = (20..400).map(|i| m.nerve(ManId(i)) as i64).collect();
    let n = soldiers.len() as i64;
    let mean = soldiers.iter().sum::<i64>() / n;
    let var = soldiers.iter().map(|x| (x - mean).pow(2)).sum::<i64>() / n;
    assert!((mean - t.nerve.mean as i64).abs() < 40, "mean {mean}");
    let sd = t.nerve.sd as i64;
    assert!(
        (sd * sd * 3 / 4..sd * sd * 5 / 4).contains(&var),
        "variance {var}"
    );
    // Champions stand in front; the leader is bolder still, steadier, and better with a sword.
    let front: i64 = (3..20).map(|i| m.ardour(ManId(i)) as i64).sum::<i64>() / 17;
    let rear: i64 = (20..400).map(|i| m.ardour(ManId(i)) as i64).sum::<i64>() / n;
    assert!(front > rear + 128, "front {front}, rear {rear}");
    let kits = m.kits();
    assert_eq!(kits[0].skill, t.leader_skill);
    assert_eq!(kits[50].skill, t.skill);
    assert!(
        kits.iter()
            .all(|k| k.guard_pm == 1_250 && k.reach_mm == 1_500)
    );
    assert_eq!(Troop::named("gallic-warband"), Some(Troop::WARBAND));
    assert_eq!(Troop::named("hastati-veterans"), Some(Troop::VETERANS));
    assert_eq!(Troop::named("nobody"), None);
}

/// A memory that writes down what it is told, and adds what the test sets.
#[derive(Clone, Debug)]
struct Told {
    seen: Vec<(u32, Sums)>,
    alarm: Parts,
    urge: (i32, Cause),
}

impl Memory for Told {
    const READS_SUMS: bool = true;

    fn alarm_term(&self, _: ManId, _: u32) -> Parts {
        self.alarm
    }

    fn urge_term(&self, _: ManId, _: u32) -> (i32, Cause) {
        self.urge
    }

    fn absorb(&mut self, _: ManId, v: &View, sums: &Sums) {
        self.seen.push((v.tick, *sums));
    }

    fn on_event(&mut self, _: &Event) {}

    fn hash(&self, h: &mut StateHasher) {
        h.write_u64(self.seen.len() as u64);
    }
}

fn told(alarm: Parts, urge: (i32, Cause)) -> Men<Told> {
    let mut b = Bodies::default();
    b.push(Observed {
        body: Some(BodyId(0)),
        ..Default::default()
    });
    let memory = Told {
        seen: Vec::new(),
        alarm,
        urge,
    };
    Men::with_memory(memory, Params::CENTRE, &b, &[PLAIN], &Rng::new(1))
}

#[test]
fn a_memory_is_told_what_an_awake_man_saw_and_nothing_else() {
    let p = Params::CENTRE;
    let mut m = told(NO_PARTS, (0, Cause::None));
    // Nothing to see: he sleeps, and his memory is not troubled, however often it is asked.
    let mut quiet = view(Posture::Steady);
    for tick in 0..50 {
        quiet.tick = tick;
        m.absorb(ME, &quiet);
    }
    assert!(m.memory.seen.is_empty());
    // The enemy ahead, one of his side turned, two standing beside him, an open slot at his
    // left, and his leader standing in view.
    let mut v = facing_the_enemy(Posture::Steady);
    v.tick = 60;
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 1);
    set(&mut v, Group::Beside, Rel::Own, Posture::Steady, 2);
    if let Some(s) = v.slot.as_mut() {
        s.neighbours &= !nb::LEFT;
    }
    m.absorb(ME, &v);
    let want = Sums {
        acts: UNIT,
        conditions: p.gap,
        support: 2 * p.beside + ((p.leader * p.beside) >> 8),
        urge: -p.own_unsteady,
    };
    assert_eq!(m.memory.seen, [(60, want)]);
    // Being told changes nothing he does: with no term, he is the man of rung 0.
    let plain = men(Rung::NoMarks, Role::Soldier);
    for to in [Posture::Back, Posture::Forward] {
        assert_eq!(rate(&m, &v, to), rate(&plain, &v, to));
    }
}

#[test]
fn what_a_memory_adds_moves_his_hazards_and_is_named() {
    let plain = men(Rung::NoMarks, Role::Soldier);
    let mut v = facing_the_enemy(Posture::Steady);
    set(&mut v, Group::Ahead, Rel::Own, Posture::Turned, 1);
    let mut worse = v;
    set(&mut worse, Group::Ahead, Rel::Own, Posture::Turned, 2);

    // A unit of remembered fear counts exactly as one more man of his side turning in view.
    let mut afraid = NO_PARTS;
    afraid[2] = (UNIT, Cause::Fear);
    let m = told(afraid, (0, Cause::None));
    assert_eq!(
        rate(&m, &v, Posture::Back),
        rate(&plain, &worse, Posture::Back)
    );
    assert!(causes(&m, &v, Posture::Back).contains(&Cause::Fear));

    // A part below zero steadies him by as much, and is named among what holds him.
    let mut steadied = NO_PARTS;
    steadied[0] = (-UNIT, Cause::Resolve);
    let m = told(steadied, (0, Cause::None));
    assert_eq!(
        rate(&m, &worse, Posture::Back),
        rate(&plain, &v, Posture::Back)
    );
    assert!(!causes(&m, &worse, Posture::Back).contains(&Cause::Resolve));
    let mut back = worse;
    back.own.posture = Posture::Back;
    assert!(causes(&m, &back, Posture::Steady).contains(&Cause::Resolve));

    // A unit of urge counts as one more man of his side going forward.
    let mut led = v;
    set(&mut led, Group::Beside, Rel::Own, Posture::Forward, 1);
    let m = told(NO_PARTS, (UNIT, Cause::Resolve));
    let pushed = rate(&m, &v, Posture::Forward);
    assert_eq!(pushed, rate(&plain, &led, Posture::Forward));
    assert!(pushed > rate(&plain, &v, Posture::Forward));
    assert!(causes(&m, &v, Posture::Forward).contains(&Cause::Resolve));
}
