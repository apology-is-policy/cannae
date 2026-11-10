//! A deliberately crude man model for exercising the runner, perception and dormancy. It is
//! not a rung of the ladder and claims nothing about real men.
//!
//! Side B walks forward. A man of side A gives ground when enough enemies stand ahead of him,
//! twice as readily while shaken; a man giving ground turns when his neighbours turn; a turned
//! man rallies when no enemy is in his window. *Shaken* is stored as the tick it wears off,
//! which is the closed form the dormancy contract asks for.

use cannae_core::hash::StateHasher;
use cannae_core::view::{Group, Rel};
use cannae_core::{
    Cause, Coarse, Event, EventKind, Gait, Hazards, Intent, ManId, ManModel, Observed, Posture,
    Side, Transition, Turn, View,
};

const SHAKEN_TICKS: u32 = 400;

pub struct Toy {
    shaken_until: Vec<u32>,
    flee: Vec<Turn>,
}

impl Toy {
    pub fn new(n: usize) -> Self {
        Toy {
            shaken_until: vec![0; n],
            flee: vec![Turn(0); n],
        }
    }

    fn shaken(&self, man: ManId, tick: u32) -> bool {
        tick < self.shaken_until[man.0 as usize]
    }
}

fn way(to: Posture, rate_q16: u32, cause: Cause) -> Option<Transition> {
    Some(Transition {
        to,
        rate_q16,
        causes: [cause, Cause::None, Cause::None],
    })
}

impl ManModel for Toy {
    fn dormant(&self, _: ManId, own: &Observed, c: &Coarse, _: u32) -> bool {
        own.posture == Posture::Steady && !c.enemy_near && !c.shock_near
    }

    fn absorb(&mut self, man: ManId, v: &View) {
        if v.shocks.any() {
            self.shaken_until[man.0 as usize] = v.tick + SHAKEN_TICKS;
        }
    }

    fn explain(&self, man: ManId, v: &View) -> Hazards {
        if v.own.side == Side::B {
            return Hazards::NONE;
        }
        let ahead = v.counts.get(Group::Ahead, Rel::Enemy, Posture::Steady) as u32
            + v.counts.get(Group::Ahead, Rel::Enemy, Posture::Forward) as u32;
        let turned = v.counts.total(Rel::Own, Posture::Turned);
        let enemies = v.counts.total(Rel::Enemy, Posture::Steady)
            + v.counts.total(Rel::Enemy, Posture::Forward);
        let mut h = Hazards::NONE;
        match v.own.posture {
            Posture::Steady if ahead >= 3 => {
                let mut rate = ahead * (1 << 16) / 10;
                if self.shaken(man, v.tick) {
                    rate *= 2;
                }
                h.ways[0] = way(Posture::Back, rate, Cause::Outnumbered);
            }
            Posture::Back => {
                h.ways[0] = way(
                    Posture::Turned,
                    (turned + 1) * (1 << 16) / 4,
                    Cause::NeighboursTurned,
                );
                if enemies == 0 {
                    h.ways[1] = way(Posture::Steady, 1 << 15, Cause::NoPursuer);
                }
            }
            Posture::Turned if enemies == 0 => {
                h.ways[0] = way(Posture::Steady, 1 << 15, Cause::NoPursuer);
            }
            _ => {}
        }
        h
    }

    fn intent(&self, man: ManId, v: &View, posture: Posture) -> (Intent, Gait) {
        match (v.own.side, posture) {
            (Side::B, Posture::Steady) => (
                Intent::Flee {
                    heading: v.own.facing,
                },
                Gait::Walk,
            ),
            (_, Posture::Turned) => (
                Intent::Flee {
                    heading: self.flee[man.0 as usize],
                },
                Gait::Run,
            ),
            _ => (Intent::Stand, Gait::Stand),
        }
    }

    fn on_event(&mut self, e: &Event) {
        if let EventKind::Posture {
            to: Posture::Turned,
            ..
        } = e.kind
        {
            // Away from side B, who come from +x in the face-off.
            self.flee[e.man.0 as usize] = Turn(Turn::HALF);
        }
    }

    fn inner_hash(&self) -> u64 {
        let mut h = StateHasher::new();
        for (s, f) in self.shaken_until.iter().zip(&self.flee) {
            h.write_u64((*s as u64) << 16 | f.0 as u64);
        }
        h.finish()
    }
}
