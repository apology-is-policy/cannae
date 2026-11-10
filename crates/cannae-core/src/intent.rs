//! Intents as kinds, turned into targets by the core, so both substrates steer at the same
//! points.

use crate::angle::Turn;
use crate::observe::{Bodies, BodyId, ManId};
use crate::power::GaitSpeeds;
use crate::units::Vec2;
use crate::view::{FrameQuery, Standard};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Gait {
    #[default]
    Stand = 0,
    Walk = 1,
    Run = 2,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Intent {
    /// Stay where he is.
    #[default]
    Stand,
    /// Keep his slot, standing `forward_mm` ahead of it (behind, if negative) along the frame's
    /// facing, and leaning into the press with `press` (0-255). How close he stands and how hard
    /// he leans live here, inside steady, and need no state.
    HoldSlot { forward_mm: i32, press: u8 },
    /// Step to striking distance of an enemy.
    Close { enemy: ManId, stand_off_mm: i32 },
    /// Run on a heading, chosen when he turned.
    Flee { heading: Turn },
    /// Make for his body's standard.
    RallyTo { body: BodyId },
}

/// Where the substrate should take a man, how fast, and which way he should face.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Target {
    pub point: Vec2,
    /// mm/s; zero means stand.
    pub speed: i32,
    pub face: Option<Turn>,
    pub press: u8,
}

/// Turns an intent into a target from the current snapshot.
pub fn resolve<F: FrameQuery>(
    man: ManId,
    intent: Intent,
    gait: Gait,
    bodies: &Bodies,
    frames: &F,
    speeds: &GaitSpeeds,
) -> Target {
    let pos = bodies.pos[man.0 as usize];
    let speed = match gait {
        Gait::Stand => 0,
        Gait::Walk => speeds.walk,
        Gait::Run => speeds.run,
    };
    let stand = Target {
        point: pos,
        speed: 0,
        face: None,
        press: 0,
    };
    match intent {
        Intent::Stand => stand,
        Intent::HoldSlot { forward_mm, press } => match frames.slot(man) {
            Some(s) => Target {
                point: s.target + s.facing.unit(forward_mm),
                speed,
                face: Some(s.facing),
                press,
            },
            None => stand,
        },
        Intent::Close {
            enemy,
            stand_off_mm,
        } => {
            let e = bodies.pos[enemy.0 as usize];
            let to = e - pos;
            Target {
                point: e - to.with_len(stand_off_mm),
                speed,
                face: Turn::of(to),
                press: 0,
            }
        }
        Intent::Flee { heading } => Target {
            point: pos + heading.unit(10_000),
            speed,
            face: Some(heading),
            press: 0,
        },
        Intent::RallyTo { body } => {
            let at = frames.body(body).map(|b| match b.standard {
                Standard::Carried(c) => bodies.pos[c.0 as usize],
                Standard::Down(p) => p,
            });
            match at {
                Some(point) => Target {
                    point,
                    speed,
                    face: Turn::of(point - pos),
                    press: 0,
                },
                None => stand,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observe::Observed;
    use crate::view::NoFrames;

    #[test]
    fn close_stops_short_and_faces_the_enemy() {
        let mut b = Bodies::default();
        let me = b.push(Observed::default());
        let foe = b.push(Observed {
            pos: Vec2::new(5_000, 0),
            ..Default::default()
        });
        let t = resolve(
            me,
            Intent::Close {
                enemy: foe,
                stand_off_mm: 1_200,
            },
            Gait::Walk,
            &b,
            &NoFrames,
            &GaitSpeeds::PLACEHOLDER,
        );
        assert_eq!(t.point, Vec2::new(3_800, 0));
        assert_eq!(t.face, Some(Turn(0)));
        assert_eq!(t.speed, GaitSpeeds::PLACEHOLDER.walk);
        // Without frames, holding a slot is standing.
        let h = resolve(
            me,
            Intent::HoldSlot {
                forward_mm: 0,
                press: 0,
            },
            Gait::Walk,
            &b,
            &NoFrames,
            &GaitSpeeds::PLACEHOLDER,
        );
        assert_eq!(h.speed, 0);
    }
}
