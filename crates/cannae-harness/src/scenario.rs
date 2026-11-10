//! Scenario files: TOML a person can read. They place bodies of men; what the men are like is
//! the man model's business, keyed by each body's troop type.

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::Order;
use cannae_core::{
    Bodies, BodyFrames, BodyId, ManId, Observed, Posture, Role, Side, Tie, Turn, Vec2,
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
        frames.order_at(BodyId(body as u32), order, o.at_s * 1000 / BASE_TICK.0);
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
            assert!(
                s.orders.iter().any(|o| o.at_s == 0),
                "{}: someone must move",
                path.display()
            );
        }
    }

    #[test]
    fn unknown_fields_are_errors() {
        let bad = TWO_BY_THREE.replace("closer = true", "closr = true");
        assert!(Scenario::from_toml(&bad).is_err());
        assert_eq!(turn_of_deg(90), Turn(16384));
        assert_eq!(turn_of_deg(-90), Turn(49152));
    }
}
