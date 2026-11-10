//! `cargo run --release -p cannae-man --example first_contact -- [--seed N] [--seconds N]
//!     [--rung 0|1|2] [--events N] [--looks N] [--quiet] [--flank Q8]`
//!
//! The hand-built scene `scenarios/scenes/first-contact.toml`: a century of the reference troop
//! receives a warband, with every prior at its centre, on the kinematic test substrate (men
//! walk through each other, so nothing here is a crowd's doing). For looking at what the table
//! of hazards does in the first seconds of contact. It is not the patch and reads no pass mark.
//!
//! Prints the first `--events` changes of posture with the causes each hazard named, and for
//! the first `--looks` men who gave ground or turned of themselves, what each saw when he
//! decided. Then one line for the run, and how often an enemy stood beside a Roman and faced
//! him, split by whether that enemy would have struck another man first (and so is not
//! counted on his flank). `--flank` overrides one weight, to tell that row's doing from the
//! rest; it is a diagnostic and not a prior.

use cannae_core::power::GaitSpeeds;
use cannae_core::schedule::{BASE_TICK, is_man_step};
use cannae_core::view::{Group, Rel, Sector};
use cannae_core::{
    BoutParams, Bouts, Cause, EventKind, GridPerception, ManId, Perceive, Posture, Rng, Side,
};
use cannae_harness::kinematic::Kinematic;
use cannae_harness::scenario::{Layout, frames};
use cannae_harness::{Runner, Scenario, place};
use cannae_man::{Men, Params, Rung, Troop};

const SCENE: &str = include_str!("../../../scenarios/scenes/first-contact.toml");

fn standing(p: Posture) -> bool {
    matches!(p, Posture::Steady | Posture::Forward | Posture::Back)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg = |name: &str, default: u64| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    };
    let seed = arg("--seed", 1);
    let seconds = arg("--seconds", 900) as u32;
    let most = arg("--events", 60) as usize;
    let mut looks = arg("--looks", 4);
    let quiet = args.iter().any(|a| a == "--quiet");
    let rung = match arg("--rung", 0) {
        0 => Rung::NoMarks,
        1 => Rung::HasFled,
        _ => Rung::Shaken,
    };
    let params = Params {
        enemy_flank: arg("--flank", Params::CENTRE.enemy_flank as u64) as i32,
        ..Params::CENTRE
    };

    let s = Scenario::from_toml(SCENE).unwrap();
    let troops: Vec<Troop> = s
        .bodies
        .iter()
        .map(|b| Troop::named(&b.troop).unwrap())
        .collect();
    let (bodies, placed) = place(&s);
    let layout = Layout::of(&placed);
    let f = frames(&s, &placed, &bodies).unwrap();
    let rng = Rng::new(seed);
    let men = Men::new(rung, params, &bodies, &troops, &rng);
    let bouts = Bouts::new(BoutParams::PLACEHOLDER, men.kits());
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
    let who = |m: usize| {
        let p = placed
            .iter()
            .find(|p| (p.first..p.first + p.count).contains(&(m as u32)))
            .unwrap();
        let mark = if m as u32 == p.leader {
            " leader"
        } else if m as u32 == p.carrier {
            " standard"
        } else if layout.rank[m] as u32 == p.ranks {
            " closer"
        } else {
            ""
        };
        format!("rank {} file {}{mark}", layout.rank[m], layout.file[m])
    };

    let end = seconds * 1000 / BASE_TICK.0;
    let second = 1000 / BASE_TICK.0;
    let (mut shown, mut printed) = (0, 0);
    let (mut broke, mut first_forward) = (None, None);
    let (mut driven, mut gave) = (0, 0);
    // Enemies beside a Roman and facing him, by [he gives ground][they would strike another].
    let mut flank = [[0u32; 2]; 2];
    let mut before = r.bodies.clone();
    let mut seen = GridPerception::default();
    while r.tick < end {
        let man_step = is_man_step(r.tick);
        if man_step {
            // What every man is about to decide on: the snapshot his view is built from.
            before = r.bodies.clone();
            seen.rebuild(&before, &[], r.tick, r.tick);
        }
        r.step();
        if broke.is_none() && man_step {
            for i in 0..before.len() {
                if before.side[i] != Side::A
                    || !matches!(before.posture[i], Posture::Steady | Posture::Back)
                {
                    continue;
                }
                let v = seen.view(&before, &r.frames, ManId(i as u32));
                for n in v.near() {
                    let in_cone = standing(n.posture)
                        && n.faces_me
                        && matches!(n.sector, Sector::Left | Sector::Right);
                    if in_cone {
                        let back = (before.posture[i] == Posture::Back) as usize;
                        flank[back][n.strikes_another as usize] += 1;
                    }
                }
            }
        }
        let log = r.log.all();
        while shown < log.len() {
            let e = &log[shown];
            shown += 1;
            let m = e.man.0 as usize;
            let roman = r.bodies.side[m] == Side::A;
            let what = match e.kind {
                EventKind::Posture { from, to } => {
                    if to == Posture::Forward && first_forward.is_none() {
                        first_forward = Some(e.tick);
                    }
                    if broke.is_none() && roman && (from, to) == (Posture::Steady, Posture::Back) {
                        if e.causes[0] == Cause::Bout {
                            driven += 1;
                        } else {
                            gave += 1;
                        }
                    }
                    format!("{from:?} -> {to:?}")
                }
                EventKind::Wound { to } => format!("wound {to:?}"),
                EventKind::Bout { .. } => continue,
                k => format!("{k:?}"),
            };
            if quiet || printed >= most {
                continue;
            }
            printed += 1;
            println!(
                "{:4}.{:02} s  {:5} {:24} {:20} {:?}",
                e.tick / second,
                e.tick % second * 100 / second,
                if roman { "Roman" } else { "Gaul" },
                who(m),
                what,
                e.causes
            );
            let of_himself = matches!(
                e.kind,
                EventKind::Posture {
                    to: Posture::Back | Posture::Turned,
                    ..
                }
            ) && e.causes[0] != Cause::Bout;
            if of_himself && looks > 0 {
                looks -= 1;
                let v = seen.view(&before, &r.frames, e.man);
                println!(
                    "           nerve {} at {:?}, his slot at {:?}",
                    r.model.nerve(e.man),
                    v.own.pos,
                    v.slot.map(|s| s.target)
                );
                for g in [Group::Ahead, Group::Beside, Group::Behind] {
                    let n = |p| v.counts.get(g, Rel::Own, p);
                    println!(
                        "           {g:?}: of his side, steady {} forward {} back {} turned {}",
                        n(Posture::Steady),
                        n(Posture::Forward),
                        n(Posture::Back),
                        n(Posture::Turned)
                    );
                }
                println!(
                    "           leader {:?}, closer behind {}",
                    v.leader.map(|l| l.posture),
                    v.closer_behind
                );
                for n in v.near() {
                    println!(
                        "           enemy at {:?}: {:?}, {:?}, faces him {}, would strike \
                         another first {} ({})",
                        n.offset,
                        n.sector,
                        n.posture,
                        n.faces_me,
                        n.strikes_another,
                        who(n.id.0 as usize)
                    );
                }
            }
        }
        if broke.is_none() {
            let (mut up, mut turned) = (0, 0);
            for i in 0..r.bodies.len() {
                if r.bodies.side[i] == Side::A && r.bodies.posture[i] != Posture::Down {
                    up += 1;
                    turned += (r.bodies.posture[i] == Posture::Turned) as u32;
                }
            }
            if turned * 5 >= up * 2 {
                broke = Some(r.tick);
            }
        }
    }
    let at = |t: Option<u32>| t.map_or("never".to_string(), |t| format!("{} s", t / second));
    println!(
        "seed {seed}: first man forward at {}, the century broken at {}; before that its men \
         were driven back by a bout {driven} times and gave ground of themselves {gave} times",
        at(first_forward),
        at(broke)
    );
    println!(
        "   enemies beside a Roman and facing him, summed over man steps: {} for steady Romans, \
         of whom {} would strike another man first and are not counted on his flank; {} for \
         Romans giving ground, of whom {}",
        flank[0][0] + flank[0][1],
        flank[0][1],
        flank[1][0] + flank[1][1],
        flank[1][1]
    );
}
