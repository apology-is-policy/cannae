//! Scenario files: TOML a person can read. They place bodies of men; what the men are like is
//! the man model's business, keyed by each body's troop type.

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::Order;
use cannae_core::{
    Bodies, BodyFrames, BodyId, ManId, Observed, Posture, Role, Side, Tie, Turn, Vec2, Volley,
    Volleys,
};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub name: String,
    pub seed: u64,
    pub duration_s: u32,
    #[serde(rename = "body")]
    pub bodies: Vec<BodySpec>,
    /// Orders given at set times, executed by the frames module.
    #[serde(default, rename = "order")]
    pub orders: Vec<OrderSpec>,
    /// Walls, handed to the substrate when it is built.
    #[serde(default, rename = "wall")]
    pub walls: Vec<WallSpec>,
    /// Missiles falling on an area at set times, resolved beside the bouts.
    #[serde(default, rename = "volley")]
    pub volleys: Vec<VolleySpec>,
    /// Reliefs, each ordered by the frames in a lull.
    #[serde(default, rename = "relief")]
    pub reliefs: Vec<ReliefSpec>,
}

/// A relief (docs/rival/02-order-and-contact.md, sec. 4): in the first lull from
/// `not_before_s` on, counted from the start of the scenario, body `by` comes up through
/// body `of` and takes the front, and `of` then falls back to where `by` stood. A lull is
/// `lull_s` in which no man of `of` is forward, giving ground or in flight, and no enemy near
/// it has stepped forward (`BodyFrames::relieve`).
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReliefSpec {
    /// The tired body's `name`.
    pub of: String,
    /// The fresh body's `name`.
    pub by: String,
    pub not_before_s: u32,
    pub lull_s: u32,
}

/// Missiles falling on an area for a stretch of time (docs/rival/02-order-and-contact.md,
/// sec. 11). Nobody throws them: the fall is the script's, so that every run has the same
/// fire, and it starts no bout.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct VolleySpec {
    pub from_s: u32,
    pub until_s: u32,
    /// Two opposite corners of the area, in mm. Whoever stands in it, of either side, is under
    /// the fall.
    pub area_mm: [[i32; 2]; 2],
    /// Of every thousand men who stand under it from beginning to end, about how many it
    /// hampers, disables and kills.
    pub hampered_per_thousand: u32,
    pub disabled_per_thousand: u32,
    pub killed_per_thousand: u32,
}

/// A straight wall from one point to another, in mm.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WallSpec {
    pub from_mm: [i32; 2],
    pub to_mm: [i32; 2],
}

impl Scenario {
    /// The scenario's walls, for the substrate.
    pub fn walls(&self) -> Vec<cannae_core::Wall> {
        self.walls
            .iter()
            .map(|w| {
                cannae_core::Wall::new(
                    Vec2::new(w.from_mm[0], w.from_mm[1]),
                    Vec2::new(w.to_mm[0], w.to_mm[1]),
                )
            })
            .collect()
    }

    /// The scenario's volleys, for the resolver that runs beside the bouts. A scenario with
    /// none gives a resolver that does nothing.
    pub fn volleys(&self) -> Result<Volleys, String> {
        let tick = |s: u32| s * 1000 / BASE_TICK.0;
        let corner = |c: [i32; 2]| Vec2::new(c[0], c[1]);
        self.volleys
            .iter()
            .map(|v| {
                let dose = [
                    v.hampered_per_thousand,
                    v.disabled_per_thousand,
                    v.killed_per_thousand,
                ];
                if v.until_s <= v.from_s {
                    return Err(format!(
                        "a volley from {} s until {} s lasts no time",
                        v.from_s, v.until_s
                    ));
                }
                if dose.iter().sum::<u32>() > 1000 {
                    return Err(format!("a volley's dose of {dose:?} is over a thousand"));
                }
                let [a, b] = v.area_mm;
                Ok(Volley::of_dose(
                    tick(v.from_s),
                    tick(v.until_s),
                    corner(a),
                    corner(b),
                    dose,
                ))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Volleys::new)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum OrderKind {
    Hold,
    Advance,
    Charge,
    Retire,
    Pursue,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct OrderSpec {
    /// The body's `name`.
    pub body: String,
    pub at_s: u32,
    pub order: OrderKind,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum SideSpec {
    A,
    B,
}

/// How the body keeps its order: the tie of each man to his slot.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum TieSpec {
    /// A maniple: files close, the standard is lifted, the closer succeeds the leader.
    #[default]
    Drilled,
    /// A warband: slow to close, and no rule for who replaces whom.
    Loose,
}

/// Where the body's leader stands.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum LeaderPlace {
    /// Front rank, right-hand file: a centurion (Polybius 6.24).
    #[default]
    FrontRight,
    /// Front rank, centre: a chieftain among his champions.
    FrontCentre,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BodySpec {
    pub name: String,
    pub side: SideSpec,
    /// Key into the man model's parameter tables.
    pub troop: String,
    pub files: u32,
    pub ranks: u32,
    pub file_mm: i32,
    pub rank_mm: i32,
    pub centre_mm: [i32; 2],
    /// Counter-clockwise from +x.
    pub facing_deg: i32,
    #[serde(default)]
    pub leader: LeaderPlace,
    /// A closer (the optio) behind the rear rank.
    #[serde(default)]
    pub closer: bool,
    #[serde(default)]
    pub tie: TieSpec,
    /// Whether the body is one of the fronts whose meeting is scored. A body that only
    /// prepares the fight, such as the band that strikes the rear in `shock`, is not: the
    /// metrics read front bodies only (docs/slice0/preregistration.md, the memory variants).
    #[serde(default = "yes")]
    pub front: bool,
}

fn yes() -> bool {
    true
}

impl Scenario {
    pub fn from_toml(text: &str) -> Result<Scenario, toml::de::Error> {
        toml::from_str(text)
    }
}

/// Integer degrees to turn units.
pub fn turn_of_deg(deg: i32) -> Turn {
    Turn((deg.rem_euclid(360) as i64 * 65_536 / 360) as u16)
}

/// One body's men, as placed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Placed {
    pub body: BodyId,
    pub first: u32,
    pub count: u32,
    pub leader: u32,
    pub carrier: u32,
    pub files: u32,
    pub ranks: u32,
    /// The scenario's `front`.
    pub front: bool,
}

/// Each man's place at setup: rank 0 is the front; a closer's rank is one past the rear.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    pub rank: Vec<u16>,
    pub file: Vec<u16>,
}

impl Layout {
    pub fn of(placed: &[Placed]) -> Layout {
        let mut l = Layout::default();
        for p in placed {
            for k in 0..p.count {
                let in_grid = k < p.files * p.ranks;
                l.rank
                    .push(if in_grid { k / p.files } else { p.ranks } as u16);
                l.file
                    .push(if in_grid { k % p.files } else { p.files / 2 } as u16);
            }
        }
        l
    }
}

/// The frames of every placed body, with the scenario's timed orders queued, and each tie at
/// its placeholder.
pub fn frames(s: &Scenario, placed: &[Placed], bodies: &Bodies) -> Result<BodyFrames, String> {
    frames_with(s, placed, bodies, |spec| match spec {
        TieSpec::Drilled => Tie::DRILLED,
        TieSpec::Loose => Tie::LOOSE,
    })
}

/// The same, with the tie for each kind the scenario names given by `tie`: a drawn parameter
/// set's, in the scored runs.
pub fn frames_with(
    s: &Scenario,
    placed: &[Placed],
    bodies: &Bodies,
    tie: impl Fn(TieSpec) -> Tie,
) -> Result<BodyFrames, String> {
    let mut frames = BodyFrames::default();
    for (b, p) in s.bodies.iter().zip(placed) {
        frames.add(
            cannae_core::Layout {
                body: p.body,
                centre: Vec2::new(b.centre_mm[0], b.centre_mm[1]),
                facing: turn_of_deg(b.facing_deg),
                files: b.files,
                ranks: b.ranks,
                file_mm: b.file_mm,
                rank_mm: b.rank_mm,
                first: ManId(p.first),
                closer: b.closer,
                tie: tie(b.tie),
            },
            bodies,
        );
    }
    let tick = |s: u32| s * 1000 / BASE_TICK.0;
    for r in &s.reliefs {
        let find = |name: &String| {
            s.bodies
                .iter()
                .position(|b| &b.name == name)
                .ok_or_else(|| format!("relief for unknown body {name:?}"))
        };
        let (of, by) = (find(&r.of)?, find(&r.by)?);
        if of == by || s.bodies[of].side != s.bodies[by].side {
            return Err(format!(
                "{:?} cannot relieve {:?}: it takes another body of the same side",
                r.by, r.of
            ));
        }
        frames.relieve(
            BodyId(of as u32),
            BodyId(by as u32),
            tick(r.not_before_s),
            tick(r.lull_s),
        );
    }
    for o in &s.orders {
        let body = s
            .bodies
            .iter()
            .position(|b| b.name == o.body)
            .ok_or_else(|| format!("order for unknown body {:?}", o.body))?;
        let order = match o.order {
            OrderKind::Hold => Order::Hold,
            OrderKind::Advance => Order::Advance,
            OrderKind::Charge => Order::Charge,
            OrderKind::Retire => Order::Retire,
            OrderKind::Pursue => Order::Pursue,
        };
        frames.order_at(BodyId(body as u32), order, tick(o.at_s));
    }
    Ok(frames)
}

/// Lays out every body in ranks and files. Rank 0 is the front, toward the facing; file 0 is the
/// left-hand file. The front rank are file leaders, the leader stands per `leader`, the carrier
/// (standard-bearer) in the second rank's centre file, and a closer, if any, behind the centre
/// of the rear rank.
pub fn place(s: &Scenario) -> (Bodies, Vec<Placed>) {
    let mut bodies = Bodies::default();
    let mut placed = Vec::new();
    for (bi, b) in s.bodies.iter().enumerate() {
        let body = BodyId(bi as u32);
        let side = match b.side {
            SideSpec::A => Side::A,
            SideSpec::B => Side::B,
        };
        let facing = turn_of_deg(b.facing_deg);
        let left = Turn(facing.0.wrapping_add(Turn::QUARTER));
        let centre = Vec2::new(b.centre_mm[0], b.centre_mm[1]);
        let at = |rank2: i32, file2: i32| {
            // Half-unit offsets so that even counts centre properly.
            centre + facing.unit(rank2 * b.rank_mm / 2) + left.unit(file2 * b.file_mm / 2)
        };
        let leader_file = match b.leader {
            LeaderPlace::FrontRight => b.files - 1,
            LeaderPlace::FrontCentre => b.files / 2,
        };
        let carrier_rank = if b.ranks > 1 { 1 } else { 0 };
        let first = bodies.len() as u32;
        let (mut leader, mut carrier) = (first, first);
        for r in 0..b.ranks {
            for f in 0..b.files {
                let pos = at(
                    b.ranks as i32 - 1 - 2 * r as i32,
                    b.files as i32 - 1 - 2 * f as i32,
                );
                let role = if r == 0 && f == leader_file {
                    Role::Leader
                } else if r == carrier_rank && f == b.files / 2 {
                    Role::Carrier
                } else if r == 0 {
                    Role::FileLeader
                } else {
                    Role::Soldier
                };
                let id = bodies.push(Observed {
                    pos,
                    facing,
                    side,
                    role,
                    body: Some(body),
                    in_place: true,
                    posture: Posture::Steady,
                    ..Default::default()
                });
                match role {
                    Role::Leader => leader = id.0,
                    Role::Carrier => carrier = id.0,
                    _ => {}
                }
            }
        }
        if b.closer {
            bodies.push(Observed {
                pos: at(-(b.ranks as i32) - 1, 0),
                facing,
                side,
                role: Role::Closer,
                body: Some(body),
                in_place: true,
                ..Default::default()
            });
        }
        placed.push(Placed {
            body,
            first,
            count: bodies.len() as u32 - first,
            leader,
            carrier,
            files: b.files,
            ranks: b.ranks,
            front: b.front,
        });
    }
    (bodies, placed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TWO_BY_THREE: &str = r#"
        name = "tiny"
        seed = 1
        duration_s = 10

        [[body]]
        name = "first century"
        side = "A"
        troop = "hastati"
        files = 3
        ranks = 2
        file_mm = 1000
        rank_mm = 2000
        centre_mm = [0, 0]
        facing_deg = 0
        closer = true
    "#;

    #[test]
    fn ranks_files_and_roles() {
        let s = Scenario::from_toml(TWO_BY_THREE).unwrap();
        let (b, p) = place(&s);
        assert_eq!(b.len(), 7);
        assert_eq!(p[0].count, 7);
        // Facing +x, so the front rank is at x = +1000 and file 0 (left) at y = +1000.
        assert_eq!(b.pos[0], Vec2::new(1000, 1000));
        assert_eq!(b.pos[2], Vec2::new(1000, -1000));
        assert_eq!(b.pos[3], Vec2::new(-1000, 1000));
        // The centurion stands front right, the standard second rank centre.
        assert_eq!(b.role[2], Role::Leader);
        assert_eq!(b.role[0], Role::FileLeader);
        assert_eq!(b.role[4], Role::Carrier);
        assert_eq!(b.role[6], Role::Closer);
        assert_eq!(b.pos[6], Vec2::new(-3000, 0));
        assert_eq!((p[0].leader, p[0].carrier), (2, 4));
        let l = Layout::of(&p);
        assert_eq!((l.rank[4], l.file[4]), (1, 1));
        assert_eq!(l.rank[6], 2);
    }

    #[test]
    fn frames_put_every_slot_where_the_man_was_placed() {
        use cannae_core::{FrameQuery, Frames};
        let text = format!(
            "{}\n[[order]]\nbody = \"warband\"\nat_s = 2\norder = \"advance\"\n",
            include_str!("../../../scenarios/patch/base.toml")
        );
        let mut s = Scenario::from_toml(&text).unwrap();
        // Only the order added here: the warband told to advance two seconds in.
        s.orders.retain(|o| o.at_s == 2);
        assert_eq!(s.orders.len(), 1);
        let (mut b, placed) = place(&s);
        let mut f = frames(&s, &placed, &b).unwrap();
        f.update(&mut b, 0, &mut Vec::new());
        for i in 0..b.len() {
            let slot = f.slot(ManId(i as u32)).expect("every man has a slot");
            assert_eq!(slot.target, b.pos[i], "man {i}");
            assert!(b.in_place[i]);
        }
        // The warband is told to advance two seconds in, and its slots then lead its men.
        f.update(&mut b, 35, &mut Vec::new());
        assert_eq!(f.body(BodyId(2)).unwrap().order, Order::Hold);
        f.update(&mut b, 40, &mut Vec::new());
        assert_eq!(f.body(BodyId(2)).unwrap().order, Order::Advance);
        let w = placed[2].first as usize;
        assert_eq!(
            f.slot(ManId(w as u32)).unwrap().target.x,
            b.pos[w].x - 1_500
        );
        let mut bad = s.clone();
        bad.orders[0].body = "nobody".into();
        assert!(frames(&bad, &placed, &b).is_err());
    }

    #[test]
    fn every_patch_scenario_places_its_bodies_and_frames() {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../../scenarios/patch");
        let mut names: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x == "toml"))
            .collect();
        names.sort();
        assert!(names.len() >= 4);
        for path in names {
            let s = Scenario::from_toml(&std::fs::read_to_string(&path).unwrap())
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let (b, placed) = place(&s);
            assert_eq!(placed.len(), s.bodies.len());
            frames(&s, &placed, &b).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            s.volleys()
                .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            // Not necessarily at once: in `shock` the first order comes at one minute.
            assert!(
                s.orders.iter().any(|o| o.order != OrderKind::Hold),
                "{}: someone must be ordered to move",
                path.display()
            );
        }
    }

    const VOLLEY: &str = r#"
        [[volley]]
        from_s = 2
        until_s = 5
        area_mm = [[3000, -2000], [-1000, 4000]]
        hampered_per_thousand = 60
        disabled_per_thousand = 30
        killed_per_thousand = 10
    "#;

    #[test]
    fn a_volley_is_read_with_its_time_its_area_and_its_dose() {
        let s = Scenario::from_toml(TWO_BY_THREE).unwrap();
        assert!(s.volleys().unwrap().is_empty());
        let s = Scenario::from_toml(&format!("{TWO_BY_THREE}{VOLLEY}")).unwrap();
        let (a, b) = (Vec2::new(-1000, -2000), Vec2::new(3000, 4000));
        // Twenty ticks to the second, and the dose in the order hampered, disabled, killed.
        let v = Volley::of_dose(40, 100, a, b, [60, 30, 10]);
        assert_eq!(s.volleys().unwrap().all(), [v]);
        assert_eq!((v.lo, v.hi), (a, b));
        assert!(v.kill < v.disable && v.disable < v.hamper);

        let bad = |from: &str, to: &str| {
            let text = format!("{TWO_BY_THREE}{}", VOLLEY.replace(from, to));
            Scenario::from_toml(&text).map(|s| s.volleys().map(|_| ()))
        };
        assert!(bad("until_s = 5", "until_s = 2").unwrap().is_err());
        assert!(
            bad("killed_per_thousand = 10", "killed_per_thousand = 911")
                .unwrap()
                .is_err()
        );
        assert_eq!(
            bad("killed_per_thousand = 10", "killed_per_thousand = 910").unwrap(),
            Ok(())
        );
        assert!(bad("from_s", "form_s").is_err());
    }

    #[test]
    fn the_patchs_volleys_fall_on_the_whole_roman_line_and_on_no_one_else() {
        let s = Scenario::from_toml(include_str!("../../../scenarios/patch/volleys.toml")).unwrap();
        let (b, _) = place(&s);
        let volleys = s.volleys().unwrap();
        let [v] = volleys.all() else {
            panic!("one volley, not {}", volleys.all().len());
        };
        // From one minute to three.
        assert_eq!((v.from, v.until), (1_200, 3_600));
        let (mut romans, mut others) = (0, 0);
        for i in 0..b.len() {
            let under = v.covers(b.pos[i]);
            assert_eq!(under, b.side[i] == Side::A, "man {i} at {:?}", b.pos[i]);
            romans += under as u32;
            others += !under as u32;
            // About a metre to spare: a man a pace out of his place is still under it.
            if under {
                for step in [
                    Vec2::new(900, 0),
                    Vec2::new(-900, 0),
                    Vec2::new(0, 900),
                    Vec2::new(0, -900),
                ] {
                    assert!(v.covers(b.pos[i] + step), "man {i} at {:?}", b.pos[i]);
                }
            }
        }
        assert_eq!((romans, others), (162, 160));
        // The base patch's Romans, where the base patch has them; only the warband is moved.
        let base = Scenario::from_toml(include_str!("../../../scenarios/patch/base.toml")).unwrap();
        assert_eq!(s.bodies[..2], base.bodies[..2]);
    }

    const RELIEF: &str = include_str!("../../../scenarios/patch/relief.toml");

    #[test]
    fn a_relief_names_two_bodies_of_one_side() {
        let s = Scenario::from_toml(RELIEF).unwrap();
        let (b, placed) = place(&s);
        assert!(frames(&s, &placed, &b).is_ok());
        let bad = |from: &str, to: &str| {
            let s = Scenario::from_toml(&RELIEF.replacen(from, to, 1)).unwrap();
            let (b, placed) = place(&s);
            frames(&s, &placed, &b).unwrap_err()
        };
        let first = "by = \"second line, first century\"";
        assert!(bad(first, "by = \"principes\"").contains("unknown body"));
        assert!(bad(first, "by = \"hastati, first century\"").contains("cannot relieve"));
        assert!(bad(first, "by = \"warband\"").contains("cannot relieve"));
        let unknown = RELIEF.replacen("lull_s", "lul_s", 1);
        assert!(Scenario::from_toml(&unknown).is_err());
    }

    #[test]
    fn a_relief_reaches_the_frames_with_its_two_times() {
        use cannae_core::view::Order;
        use cannae_core::{FrameQuery, Frames};
        // Not before 7 s, after a lull of 3 s; and nobody advances, so the front is quiet.
        let text = RELIEF
            .replace("not_before_s = 600", "not_before_s = 7")
            .replace("lull_s = 20", "lull_s = 3")
            .replace("order = \"advance\"", "order = \"hold\"");
        let s = Scenario::from_toml(&text).unwrap();
        // The ticks at which the two fresh centuries are ordered up.
        let ordered = |stirs_until: u32| {
            let (mut b, placed) = place(&s);
            let mut f = frames(&s, &placed, &b).unwrap();
            let mut at = [None; 2];
            for t in (0..400).step_by(5) {
                // A man of the first century is forward for a while.
                b.posture[0] = if t < stirs_until {
                    Posture::Forward
                } else {
                    Posture::Steady
                };
                f.update(&mut b, t, &mut Vec::new());
                for (k, at) in at.iter_mut().enumerate() {
                    let order = f.body(BodyId(3 + k as u32)).unwrap().order;
                    if at.is_none() && order == Order::Advance {
                        *at = Some(t);
                    }
                }
            }
            at
        };
        // Quiet from the start: both at 7 s, tick 140. With the first century quiet only from
        // tick 120 on, its relief comes 3 s after that, at 180, and the other's is its own.
        assert_eq!(ordered(0), [Some(140), Some(140)]);
        assert_eq!(ordered(120), [Some(180), Some(140)]);
    }

    #[test]
    fn the_relief_patch_is_the_base_patch_with_a_second_line_out_of_its_sight() {
        use cannae_core::view::{Group, Rel};
        use cannae_core::{Frames, GridPerception, Perceive};
        let base = Scenario::from_toml(include_str!("../../../scenarios/patch/base.toml")).unwrap();
        let s = Scenario::from_toml(RELIEF).unwrap();
        // The base patch's bodies and orders, in its order, and then the second line.
        assert_eq!(s.bodies[..3], base.bodies[..]);
        assert_eq!(s.orders, base.orders);
        assert_eq!((s.bodies.len(), s.reliefs.len()), (5, 2));
        for (k, r) in s.reliefs.iter().enumerate() {
            let (of, by) = (&s.bodies[k], &s.bodies[3 + k]);
            assert_eq!((&r.of, &r.by), (&of.name, &by.name));
            // Not before ten minutes, in a lull of twenty seconds.
            assert_eq!((r.not_before_s, r.lull_s), (600, 20));
            // The same men in the same array, 24 m back and half a file to the left.
            let moved = BodySpec {
                name: of.name.clone(),
                centre_mm: [by.centre_mm[0] + 24_000, by.centre_mm[1] - of.file_mm / 2],
                ..by.clone()
            };
            assert_eq!(&moved, of);
        }
        // What each man of the first line and of the warband sees at setup is what he sees in
        // the base patch: nobody of the second line is in anyone's window.
        let see = |s: &Scenario| {
            let (mut b, placed) = place(s);
            let mut f = frames(s, &placed, &b).unwrap();
            f.update(&mut b, 0, &mut Vec::new());
            let mut seen = GridPerception::default();
            seen.rebuild(&b, &[], 0, 0);
            (0..322)
                .map(|i| {
                    let v = seen.view(&b, &f, ManId(i));
                    let own = |g| {
                        [
                            Posture::Steady,
                            Posture::Forward,
                            Posture::Back,
                            Posture::Turned,
                        ]
                        .map(|p| v.counts.get(g, Rel::Own, p))
                    };
                    (own(Group::Ahead), own(Group::Beside), own(Group::Behind))
                })
                .collect::<Vec<_>>()
        };
        assert_eq!(see(&s), see(&base));
    }

    #[test]
    fn unknown_fields_are_errors() {
        let bad = TWO_BY_THREE.replace("closer = true", "closr = true");
        assert!(Scenario::from_toml(&bad).is_err());
        assert_eq!(turn_of_deg(90), Turn(16384));
        assert_eq!(turn_of_deg(-90), Turn(49152));
    }
}
