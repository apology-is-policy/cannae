//! The field substrate on small hand-built scenes.

use crate::{Contact, Field, Params};
use cannae_core::schedule::{BASE_TICK, MAN_EVERY};
use cannae_core::{
    Bodies, Cause, Event, EventKind, Observed, Posture, Rng, Side, Substrate, Target, Turn, Vec2,
    Wound,
};

const WALK: i32 = 1_300;
const RUN: i32 = 4_000;

/// A lattice of men, `nx` by `ny`, from its south-west corner.
#[allow(clippy::too_many_arguments)]
fn lattice(
    b: &mut Bodies,
    x0: i32,
    y0: i32,
    nx: i32,
    ny: i32,
    dx: i32,
    dy: i32,
    side: Side,
    in_place: bool,
) {
    for r in 0..ny {
        for f in 0..nx {
            b.push(Observed {
                pos: Vec2::new(x0 + f * dx, y0 + r * dy),
                side,
                in_place,
                ..Default::default()
            });
        }
    }
}

fn stay(b: &Bodies) -> Vec<Target> {
    b.pos
        .iter()
        .map(|&point| Target {
            point,
            ..Default::default()
        })
        .collect()
}

fn toward(b: &Bodies, point: Vec2, speed: i32, press: u8) -> Vec<Target> {
    b.pos
        .iter()
        .map(|&p| Target {
            point,
            speed,
            face: Turn::of(point - p),
            press,
        })
        .collect()
}

/// Runs `steps` man steps from step `first`, calling the substrate at every base tick as the
/// runner does.
fn run_from(f: &mut Field, b: &mut Bodies, t: &[Target], first: u32, steps: u32) -> Vec<Event> {
    let rng = Rng::new(1);
    let mut out = Vec::new();
    for tick in first * MAN_EVERY..(first + steps) * MAN_EVERY {
        f.advance(b, t, BASE_TICK, &rng, tick, &mut out);
    }
    out
}

fn run(f: &mut Field, b: &mut Bodies, t: &[Target], steps: u32) -> Vec<Event> {
    run_from(f, b, t, 0, steps)
}

fn per_m2(q8: u32) -> u32 {
    q8 / 256
}

fn nearest_pair(b: &Bodies, enemies_only: bool) -> i64 {
    let mut best = i64::MAX;
    for i in 0..b.len() {
        for j in i + 1..b.len() {
            if enemies_only && b.side[i] == b.side[j] {
                continue;
            }
            best = best.min(b.pos[i].dist_sq(b.pos[j]));
        }
    }
    (best as u64).isqrt() as i64
}

#[test]
fn a_lone_man_walks_to_his_target_and_stops_on_it() {
    let mut b = Bodies::default();
    lattice(&mut b, 100, -700, 1, 1, 0, 0, Side::A, false);
    let goal = Vec2::new(4_100, 2_300);
    let t = toward(&b, goal, WALK, 0);
    let mut f = Field::new(Params::CENTRE);
    run(&mut f, &mut b, &t, 4);
    // A second of walking, less at most a millimetre of rounding a step.
    let gone = (b.pos[0] - Vec2::new(100, -700)).length();
    assert!((WALK as i64 - 8..=WALK as i64).contains(&gone), "{gone}");
    assert!((b.vel[0].length() - WALK as i64).abs() <= 8);
    run(&mut f, &mut b, &t, 20);
    assert_eq!(b.pos[0], goal);
    assert_eq!(b.vel[0], Vec2::ZERO);
    assert_eq!(f.stats().hot, 0);
    assert_eq!(f.stats().free, 1);
}

#[test]
fn the_whole_step_is_taken_at_the_man_step() {
    let mut b = Bodies::default();
    lattice(&mut b, 0, 0, 1, 1, 0, 0, Side::A, false);
    let t = toward(&b, Vec2::new(10_000, 0), WALK, 0);
    let mut f = Field::new(Params::CENTRE);
    let rng = Rng::new(1);
    let mut out = Vec::new();
    f.advance(&mut b, &t, BASE_TICK, &rng, 0, &mut out);
    let after = b.clone();
    assert_eq!(after.pos[0].x, WALK / 4);
    for tick in 1..MAN_EVERY {
        f.advance(&mut b, &t, BASE_TICK, &rng, tick, &mut out);
        assert_eq!(b, after);
    }
}

#[test]
fn turning_takes_time() {
    let mut b = Bodies::default();
    lattice(&mut b, 0, 0, 1, 1, 0, 0, Side::A, false);
    let mut t = stay(&b);
    // Three eighths of a turn to his left, at a half turn a second.
    t[0].face = Some(Turn(24_576));
    let mut f = Field::new(Params::CENTRE);
    run(&mut f, &mut b, &t, 1);
    assert_eq!(b.facing[0], Turn(5 * Params::CENTRE.turn_per_tick));
    run(&mut f, &mut b, &t, 1);
    assert_eq!(b.facing[0], Turn(10 * Params::CENTRE.turn_per_tick));
    run(&mut f, &mut b, &t, 2);
    assert_eq!(b.facing[0], Turn(24_576));
}

/// A walker in a standing crowd of his own side, two men to the square metre.
fn through_a_crowd(in_place: bool, spacing: i32) -> i64 {
    let mut b = Bodies::default();
    lattice(&mut b, 0, 0, 40, 20, spacing, spacing, Side::A, false);
    let mut t = stay(&b);
    let walker = b.len();
    let start = Vec2::new(5 * spacing + spacing / 2, 10 * spacing + spacing / 2);
    b.push(Observed {
        pos: start,
        in_place,
        ..Default::default()
    });
    t.push(Target {
        point: start + Vec2::new(40_000, 0),
        speed: WALK,
        face: Some(Turn(0)),
        press: 0,
    });
    let mut f = Field::new(Params::CENTRE);
    run(&mut f, &mut b, &t, 8);
    (b.pos[walker] - start).length()
}

#[test]
fn walking_slows_as_the_crowd_thickens() {
    // Two seconds. Alone he would cover 2.6 m; at two men to the square metre Weidmann's curve
    // gives 0.46 of that.
    let at_two = through_a_crowd(false, 707);
    assert!((900..1_500).contains(&at_two), "{at_two}");
    // At six and a quarter the crowd is past the jam density and he does not get through.
    let at_six = through_a_crowd(false, 400);
    assert!(at_six < 300, "{at_six}");
}

#[test]
fn a_man_in_his_place_is_moved_by_his_tie_alone() {
    assert_eq!(through_a_crowd(true, 400), 2 * WALK as i64);
}

#[test]
fn the_fallen_are_no_obstacle() {
    let mut b = Bodies::default();
    lattice(&mut b, 0, 0, 40, 20, 400, 400, Side::A, false);
    for q in b.posture.iter_mut() {
        *q = Posture::Down;
    }
    let before = b.pos.clone();
    let mut t = stay(&b);
    let walker = b.len();
    b.push(Observed {
        pos: Vec2::new(2_200, 4_200),
        ..Default::default()
    });
    t.push(Target {
        point: Vec2::new(40_000, 4_200),
        speed: WALK,
        face: None,
        press: 0,
    });
    let mut f = Field::new(Params::CENTRE);
    run(&mut f, &mut b, &t, 8);
    assert_eq!(b.pos[walker].x, 2_200 + 2 * WALK);
    assert_eq!(&b.pos[..walker], &before[..]);
}

/// Nine hundred men, a metre apart, all making for one point.
fn converge(press: u8, contact: Contact, steps: u32) -> (Field, Bodies, Vec<Event>) {
    let mut b = Bodies::default();
    lattice(
        &mut b,
        -14_500,
        -14_500,
        30,
        30,
        1_000,
        1_000,
        Side::A,
        false,
    );
    let t = toward(&b, Vec2::ZERO, WALK, press);
    let mut f = Field::new(Params {
        contact,
        ..Params::CENTRE
    });
    let ev = run(&mut f, &mut b, &t, steps);
    (f, b, ev)
}

#[test]
fn men_who_only_walk_stop_at_the_jam() {
    let (f, _, ev) = converge(0, Contact::Line, 240);
    let peak = f.peak_q8();
    // A jam formed, and no cell went far past the jam density of 5.4.
    assert!(peak >= 4 * 256, "peak {} per m2", per_m2(peak));
    assert!(peak <= 13 * 128, "peak {}/256 per m2", peak);
    assert!(
        ev.is_empty(),
        "no one is crushed in a crowd that only walks"
    );
}

#[test]
fn men_who_press_pack_tighter_and_stop_at_the_packing_density() {
    let (f, _, ev) = converge(255, Contact::Line, 240);
    let peak = f.peak_q8();
    assert!(peak > 13 * 128, "peak {}/256 per m2", peak);
    assert!(peak <= 21 * 128, "peak {}/256 per m2", peak);
    assert!(!ev.is_empty());
    assert!(ev.iter().all(|e| e.causes[0] == Cause::Crush
        && e.kind
            == EventKind::Wound {
                to: Wound::Disabled
            }));
}

#[test]
fn a_packed_crowd_eases_when_no_one_presses() {
    let mut b = Bodies::default();
    // Twenty by twenty at a third of a metre: nine men to the square metre.
    lattice(&mut b, 5_000, 5_000, 20, 20, 333, 333, Side::A, false);
    let t = stay(&b);
    let centroid = |b: &Bodies| {
        let (sx, sy) = b.pos.iter().fold((0i64, 0i64), |(sx, sy), p| {
            (sx + p.x as i64, sy + p.y as i64)
        });
        Vec2::new((sx / b.len() as i64) as i32, (sy / b.len() as i64) as i32)
    };
    let start = centroid(&b);
    let mut f = Field::new(Params::CENTRE);
    run(&mut f, &mut b, &t, 1);
    let packed = f.peak_q8();
    assert!(packed > 7 * 256);
    run(&mut f, &mut b, &t, 480);
    let eased = f.peak_q8();
    eprintln!("packed {packed} eased {eased}");
    assert!(eased < 5 * 256, "still {} per m2", per_m2(eased));
    // It spread evenly: the crowd as a whole went nowhere, and no one was thrown out of it.
    let drift = (centroid(&b) - start).length();
    assert!(drift <= 60, "drifted {drift} mm");
    let widest = b.pos.iter().map(|p| (*p - start).length()).max().unwrap();
    assert!(widest < 15_000, "{widest}");
}

const FILES: usize = 40;

/// Two lines of forty, five metres apart; the second is set off sideways by `stagger`. The
/// first goes at the second at `speed`, and the second comes on at `speed_b`. Returns the men
/// at the end, and the nearest any two enemies came at any step.
fn head_on(stagger: i32, speed: i32, speed_b: i32, steps: u32) -> (Bodies, i64) {
    let mut b = Bodies::default();
    lattice(&mut b, 0, 0, 1, FILES as i32, 0, 900, Side::A, false);
    lattice(
        &mut b,
        5_000,
        stagger,
        1,
        FILES as i32,
        0,
        900,
        Side::B,
        false,
    );
    let t: Vec<Target> = b
        .pos
        .iter()
        .zip(&b.side)
        .map(|(&p, &s)| Target {
            point: p + Vec2::new(if s == Side::A { 30_000 } else { -30_000 }, 0),
            speed: if s == Side::A { speed } else { speed_b },
            face: Some(Turn(if s == Side::A { 0 } else { Turn::HALF })),
            press: 0,
        })
        .collect();
    let mut f = Field::new(Params::CENTRE);
    let mut nearest = i64::MAX;
    let mut met = false;
    for k in 0..steps {
        run_from(&mut f, &mut b, &t, k, 1);
        nearest = nearest.min(nearest_pair(&b, true));
        met |= f.stats().hot > 0;
    }
    assert!(met);
    (b, nearest)
}

#[test]
fn two_men_who_meet_stop_each_other() {
    for speed in [WALK, RUN] {
        let (b, nearest) = head_on(0, speed, speed, 100);
        // Each man is still on his own side of the man he met. Where two pushes are unequal
        // the pair wanders, so one file's meeting place is not its neighbour's.
        let mut met = 0;
        for k in 0..FILES {
            let (a, e) = (b.pos[k], b.pos[FILES + k]);
            assert!(
                a.x < e.x && a.y == e.y,
                "speed {speed}: file {k}: {a:?} {e:?}"
            );
            met += (a.x + e.x) / 2;
        }
        // Neither side gained ground on the other: they met near the middle.
        let met = met / FILES as i32;
        assert!(
            (2_000..=3_000).contains(&met),
            "speed {speed}: met at {met}"
        );
        assert!(nearest >= Params::CENTRE.guard_mm as i64 - 5, "{nearest}");
    }
}

#[test]
fn a_line_that_stands_is_a_wall_even_out_of_order() {
    // One rank of men, none in his place, standing. Men who walk or run at its gaps are held
    // in front of it, and it is not shoved back.
    for (stagger, speed) in [(450, WALK), (300, WALK), (450, RUN), (300, RUN), (0, RUN)] {
        let (b, nearest) = head_on(stagger, speed, 0, 100);
        let what = format!("stagger {stagger} speed {speed}");
        assert!(
            nearest >= Params::CENTRE.guard_mm as i64 - 5,
            "{what}: enemies {nearest} mm apart"
        );
        // The first man of a line set off sideways stands opposite no gap but the open end,
        // and walks round it.
        let first = (stagger > 0) as usize;
        let east_a = b.pos[first..FILES].iter().map(|p| p.x).max().unwrap();
        assert!(east_a < 5_000 - 350, "{what}: {east_a}");
        assert!(b.pos[FILES..].iter().all(|p| p.x == 5_000), "{what}");
    }
}

#[test]
fn two_thin_lines_that_both_come_on_slip_past_but_no_one_is_stood_upon() {
    // Set against each other's gaps, and each man bent on getting through, one rank out of
    // order is no wall: men sidestep each other. That is men with no order and no depth. Even
    // so, at no step do two enemies stand much nearer than their guard.
    for (stagger, speed) in [(450, WALK), (300, WALK), (450, RUN), (300, RUN)] {
        let (_, nearest) = head_on(stagger, speed, speed, 100);
        // A man wedged between two who are themselves wedged is not quite settled in a step.
        assert!(
            nearest >= Params::CENTRE.guard_mm as i64 * 97 / 100,
            "stagger {stagger} speed {speed}: enemies {nearest} mm apart"
        );
    }
}

#[test]
fn men_who_go_for_each_other_are_held_a_guard_apart() {
    // Each of a pair makes for the other, wherever he now is. Pairs differ in how they lie on
    // the grid, and in whether both move.
    for (k, speed_b) in [(0, WALK), (1, WALK), (2, RUN), (3, 0), (4, 0)] {
        let mut b = Bodies::default();
        let at_a = Vec2::new(k * 337, k * 211);
        let at_b = at_a + Vec2::new(1_500 + k * 90, 400 - k * 170);
        lattice(&mut b, at_a.x, at_a.y, 1, 1, 0, 0, Side::A, false);
        lattice(&mut b, at_b.x, at_b.y, 1, 1, 0, 0, Side::B, false);
        let mut f = Field::new(Params::CENTRE);
        let mut nearest = i64::MAX;
        for step in 0..60 {
            let t = [
                Target {
                    point: b.pos[1],
                    speed: WALK,
                    face: None,
                    press: 0,
                },
                Target {
                    point: b.pos[0],
                    speed: speed_b,
                    face: None,
                    press: 0,
                },
            ];
            run_from(&mut f, &mut b, &t, step, 1);
            nearest = nearest.min(nearest_pair(&b, true));
        }
        let guard = Params::CENTRE.guard_mm as i64;
        assert!(nearest >= guard - 2, "pair {k}: came within {nearest}");
        let end = nearest_pair(&b, true);
        assert!(end <= guard + 5, "pair {k}: ended {end} apart");
        assert_eq!(f.stats().hot, 2);
        if speed_b == 0 {
            assert_eq!(b.pos[1], at_b, "pair {k}: a man who stands is not shoved");
        }
    }
}

/// Two crowds of two hundred, ten deep, go at each other.
fn masses(speed: i32, press: u8, contact: Contact) -> Bodies {
    let mut b = Bodies::default();
    lattice(&mut b, -10_000, 0, 10, 20, 900, 900, Side::A, false);
    lattice(&mut b, 3_000, 450, 10, 20, 900, 900, Side::B, false);
    let t: Vec<Target> = b
        .pos
        .iter()
        .zip(&b.side)
        .map(|(&p, &s)| Target {
            point: p + Vec2::new(if s == Side::A { 60_000 } else { -60_000 }, 0),
            speed,
            face: Some(Turn(if s == Side::A { 0 } else { Turn::HALF })),
            press,
        })
        .collect();
    let mut f = Field::new(Params {
        contact,
        ..Params::CENTRE
    });
    run(&mut f, &mut b, &t, 240);
    b
}

#[test]
fn two_masses_do_not_pass_through_each_other() {
    for (speed, press, contact) in [
        (WALK, 0, Contact::Line),
        (RUN, 0, Contact::Line),
        (WALK, 255, Contact::Line),
        (WALK, 255, Contact::Crowd),
    ] {
        let b = masses(speed, press, contact);
        let what = format!("speed {speed} press {press} {contact:?}");
        let mut xa: Vec<i32> = b.pos[..200].iter().map(|p| p.x).collect();
        let mut xb: Vec<i32> = b.pos[200..].iter().map(|p| p.x).collect();
        xa.sort_unstable();
        xb.sort_unstable();
        // A minute on, the two are still two: the middle of each is on its own side of the
        // other's foremost tenth, and they are still in contact.
        assert!(xa[100] < xb[20], "{what}: {} {}", xa[100], xb[20]);
        assert!(xb[100] > xa[180], "{what}: {} {}", xb[100], xa[180]);
        // Few have got in among the enemy.
        let among = xa.iter().filter(|&&x| x > xb[100]).count()
            + xb.iter().filter(|&&x| x < xa[100]).count();
        assert!(among <= 20, "{what}: {among} men among the enemy");
    }
}

/// A formed block, twelve files by four ranks, and men of `side` running at it from the west.
fn charge_a_block(side: Side) -> (Bodies, usize) {
    let mut b = Bodies::default();
    lattice(&mut b, 10_000, 0, 4, 12, 1_200, 900, Side::B, true);
    let mut t = stay(&b);
    let runners = b.len();
    for (k, y) in [450, 900, 2_000, 4_750, 5_150].into_iter().enumerate() {
        let pos = Vec2::new(-(k as i32) * 137, y);
        b.push(Observed {
            pos,
            side,
            ..Default::default()
        });
        t.push(Target {
            point: pos + Vec2::new(60_000, 0),
            speed: RUN,
            face: Some(Turn(0)),
            press: 0,
        });
    }
    let mut f = Field::new(Params::CENTRE);
    run(&mut f, &mut b, &t, 40);
    (b, runners)
}

#[test]
fn a_runner_cannot_hop_over_a_formed_enemy_line() {
    let (b, runners) = charge_a_block(Side::A);
    // The deepest a man can stand is in the notch between two files, a guard from each.
    for p in &b.pos[runners..] {
        assert!(p.x < 10_000 - 350, "{p:?}");
    }
    let gap = nearest_pair(&b, true);
    assert!(gap >= Params::CENTRE.guard_mm as i64 - 5, "{gap}");
    // The block did not stir.
    assert!(b.pos[..runners].iter().all(|p| p.x >= 10_000));
}

#[test]
fn friends_make_way() {
    let (b, runners) = charge_a_block(Side::B);
    for p in &b.pos[runners..] {
        assert!(p.x > 10_000 + 3 * 1_200, "{p:?}");
    }
}

#[test]
fn in_a_packed_moving_crowd_men_touch_only_if_asked() {
    let close = |contact| {
        let (f, b, _) = converge(0, contact, 60);
        let near = f.body_mm() as i64 * 3 / 4;
        let mut n = 0;
        for i in 0..b.len() {
            for j in i + 1..b.len() {
                if b.pos[i].dist_sq(b.pos[j]) < near * near {
                    n += 1;
                }
            }
        }
        n
    };
    let (field, touching) = (close(Contact::Line), close(Contact::Crowd));
    assert!(field > 40, "{field}");
    assert!(touching * 3 < field, "{touching} against {field}");
}

/// Two mobs of enemies pressing on each other: every part of the step is exercised.
fn melee(offset: Vec2, contact: Contact) -> (Bodies, Vec<Event>) {
    let mut b = Bodies::default();
    lattice(&mut b, -9_000, -4_000, 12, 14, 700, 650, Side::A, false);
    lattice(&mut b, 1_500, -4_300, 12, 14, 730, 690, Side::B, false);
    // A few formed men in the middle of it.
    lattice(&mut b, -300, -900, 1, 3, 0, 900, Side::A, true);
    let mut t: Vec<Target> = b
        .pos
        .iter()
        .zip(&b.side)
        .map(|(&p, &s)| Target {
            point: Vec2::new(if s == Side::A { 6_000 } else { -6_000 }, p.y / 2) + offset,
            speed: WALK,
            face: Some(Turn(if s == Side::A { 0 } else { Turn::HALF })),
            press: 200,
        })
        .collect();
    let n = b.len();
    for (tg, &p) in t[n - 3..].iter_mut().zip(&b.pos[n - 3..]) {
        *tg = Target {
            point: p + offset,
            ..Default::default()
        };
    }
    for p in b.pos.iter_mut() {
        *p += offset;
    }
    let mut f = Field::new(Params {
        contact,
        ..Params::CENTRE
    });
    let ev = run(&mut f, &mut b, &t, 80);
    for p in b.pos.iter_mut() {
        *p -= offset;
    }
    (b, ev)
}

#[test]
fn same_inputs_same_bits() {
    for contact in [Contact::Line, Contact::Crowd, Contact::All] {
        let (a, ea) = melee(Vec2::ZERO, contact);
        let (b, eb) = melee(Vec2::ZERO, contact);
        assert_eq!(a.state_hash(), b.state_hash(), "{contact:?}");
        assert_eq!(ea, eb, "{contact:?}");
    }
}

#[test]
fn the_grid_leaves_no_mark() {
    // Moved by whole cells, the same scene gives the same bits, wherever the tiles' edges and
    // the origin fall.
    let cell = 1 << Params::CENTRE.cell_shift;
    for contact in [Contact::Line, Contact::Crowd] {
        let (here, _) = melee(Vec2::ZERO, contact);
        for (kx, ky) in [(37, 5), (-1_000, 16), (3, -70_001), (32, 32)] {
            let (there, _) = melee(Vec2::new(kx * cell, ky * cell), contact);
            assert_eq!(here.pos, there.pos, "{contact:?} moved by {kx}, {ky} cells");
        }
    }
}

#[test]
fn the_three_ways_of_touching_differ() {
    let (line, _) = melee(Vec2::ZERO, Contact::Line);
    let (crowd, _) = melee(Vec2::ZERO, Contact::Crowd);
    let (all, _) = melee(Vec2::ZERO, Contact::All);
    assert_ne!(line.pos, crowd.pos);
    assert_ne!(crowd.pos, all.pos);
    // On the field alone, enemies keep their guard. Where men of one side touch, the men
    // behind shove a man onto the enemy and the guard gives; the bodies do not.
    let gap = nearest_pair(&line, true);
    assert!(gap >= Params::CENTRE.guard_mm as i64 - 30, "{gap}");
    let body = Field::new(Params::CENTRE).body_mm() as i64;
    for b in [&crowd, &all] {
        let gap = nearest_pair(b, true);
        assert!(gap >= body * 3 / 4, "{gap}");
    }
}
