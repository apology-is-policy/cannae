//! The clock and the order of phases, shared by both engines.
//!
//! One base tick is 50 ms. Everything else is a multiple: the pairwise substrate moves every
//! base tick, the field substrate moves every man step with substeps for fast bodies on the
//! same 50 ms grid, and the man decides every fifth base tick (4 Hz) in both, so the man is the
//! same in all four cells of the experiment.
//!
//! **Updates are synchronous.** At a man step every View is built from one snapshot of the
//! observable state, every decision is made from its View, and the resulting events become
//! visible at the next man step. That is what makes reaction time a model parameter in both
//! engines rather than an accident of iteration order.
//!
//! Phases of a man step, in order:
//!
//! 1. **Orders and frames:** officers' orders, carriers moving, drills, slot maps.
//! 2. **Views:** built from the snapshot, for every man who is not dormant.
//! 3. **Decisions:** each man absorbs what he saw, his hazards are drawn, and he picks an intent.
//! 4. **Events:** posture changes applied in (tick, man id) order.
//! 5. **Motion:** the substrate moves bodies toward their targets (every base tick).
//! 6. **Bouts and volleys:** resolved together, outcomes applied at the end of the phase.
//! 7. **Observers:** metrics, the state hash, the frame dump.

use crate::units::SimMs;

pub const BASE_TICK: SimMs = SimMs(50);

/// Base ticks per man step.
pub const MAN_EVERY: u32 = 5;

pub const MAN_STEP: SimMs = SimMs(BASE_TICK.0 * MAN_EVERY);

pub const fn is_man_step(tick: u32) -> bool {
    tick.is_multiple_of(MAN_EVERY)
}

pub const fn time_of(tick: u32) -> SimMs {
    SimMs(tick * BASE_TICK.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn four_man_steps_a_second() {
        let per_second = (0..20).filter(|&t| is_man_step(t)).count();
        assert_eq!(per_second, 4);
        assert_eq!(time_of(20), SimMs(1000));
        assert_eq!(MAN_STEP, SimMs(250));
    }
}
