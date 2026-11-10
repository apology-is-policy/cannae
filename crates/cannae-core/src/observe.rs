//! What anyone on the field can see of every man. Men, metrics and the viewer read this and
//! nothing else of each other; a man's inner state lives in the man model, behind the seam.

use crate::angle::Turn;
use crate::hash::StateHasher;
use crate::units::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ManId(pub u32);

/// A body of men: the men who follow one carrier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BodyId(pub u32);

/// The five postures of the common design, each a thing a witness could see.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Posture {
    /// In his place, facing the enemy, on guard.
    #[default]
    Steady = 0,
    /// Stepped out to strike or throw; committed. Against turned backs, pursuit.
    Forward = 1,
    /// Giving ground, still facing the enemy.
    Back = 2,
    /// His back to the enemy, running.
    Turned = 3,
    /// On the ground: disabled, dead, or knocked down.
    Down = 4,
}

impl Posture {
    pub const ALL: [Posture; 5] = [
        Posture::Steady,
        Posture::Forward,
        Posture::Back,
        Posture::Turned,
        Posture::Down,
    ];
}

/// Wound classes. A man disabled or dead is always `Down`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Wound {
    #[default]
    Sound = 0,
    Hampered = 1,
    Disabled = 2,
    Dead = 3,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Side {
    #[default]
    A = 0,
    B = 1,
}

impl Side {
    pub const fn enemy(self) -> Side {
        match self {
            Side::A => Side::B,
            Side::B => Side::A,
        }
    }
}

/// Roles are generic so that one mechanism covers maniple, warband and turma.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Role {
    #[default]
    Soldier = 0,
    /// Front of a file: the frame is steadied by these.
    FileLeader = 1,
    /// Behind the rear rank, blocking the step from giving ground to running (the optio).
    Closer = 2,
    /// Carries the body's frame: a standard-bearer.
    Carrier = 3,
    /// Centurion, chieftain or decurion.
    Leader = 4,
}

/// One man's observable state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Observed {
    pub pos: Vec2,
    pub vel: Vec2,
    pub facing: Turn,
    pub posture: Posture,
    pub wound: Wound,
    pub side: Side,
    pub role: Role,
    pub body: Option<BodyId>,
    pub in_place: bool,
}

/// Observable state of every man, as a struct of arrays indexed by `ManId`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bodies {
    pub pos: Vec<Vec2>,
    pub vel: Vec<Vec2>,
    pub facing: Vec<Turn>,
    pub posture: Vec<Posture>,
    pub wound: Vec<Wound>,
    pub side: Vec<Side>,
    pub role: Vec<Role>,
    pub body: Vec<Option<BodyId>>,
    pub in_place: Vec<bool>,
}

impl Bodies {
    pub fn len(&self) -> usize {
        self.pos.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pos.is_empty()
    }

    pub fn push(&mut self, m: Observed) -> ManId {
        let id = ManId(self.len() as u32);
        self.pos.push(m.pos);
        self.vel.push(m.vel);
        self.facing.push(m.facing);
        self.posture.push(m.posture);
        self.wound.push(m.wound);
        self.side.push(m.side);
        self.role.push(m.role);
        self.body.push(m.body);
        self.in_place.push(m.in_place);
        id
    }

    pub fn get(&self, id: ManId) -> Observed {
        let i = id.0 as usize;
        Observed {
            pos: self.pos[i],
            vel: self.vel[i],
            facing: self.facing[i],
            posture: self.posture[i],
            wound: self.wound[i],
            side: self.side[i],
            role: self.role[i],
            body: self.body[i],
            in_place: self.in_place[i],
        }
    }

    /// A hash of the whole observable state, in a fixed order. Two runs agree bit for bit
    /// exactly when their hashes agree, up to the odds of a 64-bit collision.
    pub fn state_hash(&self) -> u64 {
        let mut h = StateHasher::new();
        h.write_u64(self.len() as u64);
        for i in 0..self.len() {
            h.write_u64(((self.pos[i].x as u32 as u64) << 32) | self.pos[i].y as u32 as u64);
            h.write_u64(((self.vel[i].x as u32 as u64) << 32) | self.vel[i].y as u32 as u64);
            h.write_u64(
                (self.facing[i].0 as u64) << 48
                    | (self.posture[i] as u64) << 40
                    | (self.wound[i] as u64) << 32
                    | (self.side[i] as u64) << 24
                    | (self.role[i] as u64) << 16
                    | (self.in_place[i] as u64) << 8
                    | self.body[i].is_some() as u64,
            );
            h.write_u64(self.body[i].map_or(0, |b| b.0 as u64));
        }
        h.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn man(x: i32, posture: Posture) -> Observed {
        Observed {
            pos: Vec2::new(x, 0),
            posture,
            body: Some(BodyId(1)),
            in_place: true,
            ..Default::default()
        }
    }

    #[test]
    fn push_get_round_trip() {
        let mut b = Bodies::default();
        let m = man(1500, Posture::Forward);
        let id = b.push(m);
        assert_eq!(id, ManId(0));
        assert_eq!(b.get(id), m);
        assert_eq!(Side::A.enemy(), Side::B);
    }

    #[test]
    fn hash_sees_every_field_and_order() {
        let mut a = Bodies::default();
        a.push(man(0, Posture::Steady));
        a.push(man(900, Posture::Steady));
        let base = a.state_hash();
        let mut b = a.clone();
        b.posture[1] = Posture::Back;
        assert_ne!(b.state_hash(), base);
        let mut c = a.clone();
        c.in_place[0] = false;
        assert_ne!(c.state_hash(), base);
        let mut d = Bodies::default();
        d.push(man(900, Posture::Steady));
        d.push(man(0, Posture::Steady));
        assert_ne!(d.state_hash(), base);
        assert_eq!(a.clone().state_hash(), base);
    }
}
