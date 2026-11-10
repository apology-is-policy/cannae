//! Competing hazards, resolved with one draw over a fixed partition.
//!
//! From each posture a man has at most two ways out (steady: forward or back; back: turned or
//! steady). Their rates are per second. The chance that something happens in a step is
//! 1 - exp(-total rate * dt), and the draw that decides whether it happens also decides which,
//! in proportion to the rates. One draw per man per step, in a fixed partition, so that the
//! two engines consume the same random numbers for the same decisions.

use crate::event::Causes;
use crate::fixed::Prob;
use crate::observe::Posture;
use crate::units::SimMs;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Transition {
    pub to: Posture,
    /// Hazard rate per second, Q16. Exactly zero below a threshold: dormancy depends on it.
    pub rate_q16: u32,
    pub causes: Causes,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Hazards {
    pub ways: [Option<Transition>; 2],
}

impl Hazards {
    pub const NONE: Hazards = Hazards { ways: [None, None] };

    pub fn is_zero(&self) -> bool {
        self.ways.iter().flatten().all(|t| t.rate_q16 == 0)
    }

    pub fn total_rate_q16(&self) -> u64 {
        self.ways.iter().flatten().map(|t| t.rate_q16 as u64).sum()
    }
}

/// Which transition, if any, a `draw` selects over one step of length `dt`.
pub fn choose(h: &Hazards, dt: SimMs, draw: u32) -> Option<Transition> {
    let total = h.total_rate_q16();
    if total == 0 {
        return None;
    }
    let p = Prob::from_rate(total.min(u32::MAX as u64) as u32, dt);
    if !p.hits(draw) {
        return None;
    }
    let mut acc: u64 = 0;
    let mut last = None;
    for t in h.ways.iter().flatten().filter(|t| t.rate_q16 > 0) {
        acc += t.rate_q16 as u64;
        let bound = (p.q32() as u128 * acc as u128 / total as u128) as u64;
        if (draw as u64) < bound {
            return Some(*t);
        }
        last = Some(*t);
    }
    // Only reachable through rounding in the last bound.
    last
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Cause;
    use crate::rng::{Key, Rng, Stream};

    fn way(to: Posture, rate_per_s: u32) -> Option<Transition> {
        Some(Transition {
            to,
            rate_q16: rate_per_s << 16,
            causes: [Cause::None; 3],
        })
    }

    #[test]
    fn zero_rates_never_fire() {
        let h = Hazards {
            ways: [way(Posture::Forward, 0), None],
        };
        assert!(h.is_zero());
        assert_eq!(choose(&h, SimMs(250), 0), None);
    }

    #[test]
    fn split_follows_the_rates() {
        // Rates 1/s and 3/s over a quarter second: P(any) = 1 - e^-1 = 0.632, split 1:3.
        let h = Hazards {
            ways: [way(Posture::Forward, 1), way(Posture::Back, 3)],
        };
        let r = Rng::new(3);
        let (mut fwd, mut back) = (0u32, 0u32);
        let n = 100_000;
        for i in 0..n {
            match choose(&h, SimMs(250), r.u32(Key::new(Stream::Hazard, i, 0, 0))) {
                Some(t) if t.to == Posture::Forward => fwd += 1,
                Some(_) => back += 1,
                None => {}
            }
        }
        let any = (fwd + back) * 1000 / n;
        assert!((626..639).contains(&any), "{any} per mille");
        let share = fwd * 1000 / (fwd + back);
        assert!((240..260).contains(&share), "{share} per mille forward");
    }
}
