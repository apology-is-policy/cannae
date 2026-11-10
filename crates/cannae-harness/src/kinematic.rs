//! A test substrate: every man walks straight to his target, through anyone in the way. It
//! exists to exercise the runner and the man models before either real substrate is built.

use cannae_core::{Bodies, Posture, Rng, SimMs, Substrate, Target, Vec2};

pub struct Kinematic {
    /// The most a man turns in one base tick, in turn units.
    pub turn_per_tick: u16,
}

impl Default for Kinematic {
    fn default() -> Self {
        // A half turn in a second.
        Kinematic {
            turn_per_tick: 1_638,
        }
    }
}

impl Substrate for Kinematic {
    fn advance(&mut self, bodies: &mut Bodies, targets: &[Target], dt: SimMs, _: &Rng, _: u32) {
        for (i, t) in targets.iter().enumerate() {
            if bodies.posture[i] == Posture::Down {
                bodies.vel[i] = Vec2::ZERO;
                continue;
            }
            let to = t.point - bodies.pos[i];
            let step = (t.speed as i64 * dt.0 as i64 / 1000) as i32;
            let mv = if to.len_sq() <= step as i64 * step as i64 {
                to
            } else {
                to.with_len(step)
            };
            bodies.pos[i] += mv;
            bodies.vel[i] = mv.mul_div(1000, dt.0 as i64);
            if let Some(f) = t.face {
                bodies.facing[i] = bodies.facing[i].rotate_toward(f, self.turn_per_tick);
            }
        }
    }
}
