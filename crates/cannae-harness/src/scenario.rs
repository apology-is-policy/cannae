//! Scenario files: TOML a person can read. They place bodies of men; what the men are like is
//! the man model's business, keyed by each body's troop type.

use cannae_core::{Bodies, BodyId, Observed, Posture, Role, Side, Turn, Vec2};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub name: String,
    pub seed: u64,
    pub duration_s: u32,
    #[serde(rename = "body")]
    pub bodies: Vec<BodySpec>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
pub enum SideSpec {
    A,
    B,
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
    }

    #[test]
    fn unknown_fields_are_errors() {
        let bad = TWO_BY_THREE.replace("closer = true", "closr = true");
        assert!(Scenario::from_toml(&bad).is_err());
        assert_eq!(turn_of_deg(90), Turn(16384));
        assert_eq!(turn_of_deg(-90), Turn(49152));
    }
}
