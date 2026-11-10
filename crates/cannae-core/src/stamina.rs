//! Stamina: the critical-power / W' model of exercise physiology, shared by both engines.
//!
//! Above critical power (CP) the anaerobic reserve W' drains in proportion to the excess. Below
//! it, the spent part recovers exponentially, faster the further below CP the man works (Skiba,
//! Chidnok, Vanhatalo and Jones, "Modeling the expenditure and reconstitution of work capacity
//! above critical power", Med. Sci. Sports Exerc. 44(8): 1526-1532, 2012).
//!
//! Everything is relative to the man's own CP, because the literature agrees on ratios more
//! than on watts (docs/slice0/priors-shared.md). Power is in per mille of CP, so 1000 is CP.
//! W' is in CP-microseconds: working at 2 CP, a full reserve of `reserve_us` lasts exactly that
//! long. Microseconds keep rounding in the recovery step far below anything that matters, and
//! make the drain exact: (power - 1000) per mille for `dt` ms is that many CP-microseconds.
//!
//! Skiba's time constant, tau = 546 exp(-0.01 D_CP) + 316 s, was fitted in mechanical watts on
//! subjects whose CP was about 240 W, so D_CP = 240 f W for a margin f = (CP - P) / CP below
//! CP. The citation is confirmed from PubMed and the formula from two secondary sources
//! (docs/slice0/physiology.md, sec. 4). The paper computes tau from the session's mean sub-CP
//! power; this applies it to the current power at each step, a simplification that the prior on
//! `tau_scale_pm` is meant to cover.

use crate::fixed::exp_neg_q32;
use crate::units::SimMs;

/// Power as a multiple of the man's critical power, in per mille: 1000 is CP.
pub type CpPerMille = u32;

/// W' in CP-microseconds.
pub type CpUs = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StaminaParams {
    /// A full W' over CP, in microseconds (W'/CP).
    pub reserve_us: CpUs,
    /// A scale on Skiba's recovery time constant, per mille.
    pub tau_scale_pm: u32,
}

impl StaminaParams {
    /// Mid-prior values: W'/CP of 75 s, Skiba's constant as published.
    pub const PLACEHOLDER: StaminaParams = StaminaParams {
        reserve_us: 75_000_000,
        tau_scale_pm: 1_000,
    };
}

/// Recovery time constant in milliseconds when working at `power` below CP.
pub fn recovery_tau_ms(power: CpPerMille, p: &StaminaParams) -> u64 {
    let margin = 1000u64.saturating_sub(power as u64);
    // 0.01 per W times 240 W of CP: 2.4 per unit of margin.
    let x = ((margin * 12) << 32) / 5_000;
    let tau = ((546_000u128 * exp_neg_q32(x) as u128) >> 32) as u64 + 316_000;
    tau * p.tau_scale_pm as u64 / 1000
}

/// W' after working at `power` for `dt`. Never below zero or above the reserve. At zero the
/// man cannot sustain more than CP; enforcing that is the caller's business.
pub fn step(w: CpUs, power: CpPerMille, p: &StaminaParams, dt: SimMs) -> CpUs {
    if power > 1000 {
        let drain = (power - 1000) as u64 * dt.0 as u64;
        (w as u64).saturating_sub(drain) as CpUs
    } else {
        let tau = recovery_tau_ms(power, p).max(1);
        let keep = exp_neg_q32(((dt.0 as u64) << 32) / tau);
        let deficit = p.reserve_us.saturating_sub(w) as u128;
        p.reserve_us - ((deficit * keep as u128) >> 32) as CpUs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::power::{Activity, PowerTable};

    const SOLDIER: StaminaParams = StaminaParams::PLACEHOLDER;
    const STEP: SimMs = SimMs(250);

    #[test]
    fn a_full_reserve_lasts_its_length_at_twice_cp() {
        let mut w = SOLDIER.reserve_us;
        let mut t = 0;
        while w > 0 {
            w = step(w, 2_000, &SOLDIER, STEP);
            t += STEP.0;
        }
        assert_eq!(t, SOLDIER.reserve_us / 1000);
        // Fighting at the placeholder's 1.55 CP empties it in a little over two minutes,
        // inside the "one to five minutes" of the sources.
        let fight = PowerTable::PLACEHOLDER.of(Activity::Fight);
        let (mut w, mut t) = (SOLDIER.reserve_us, 0);
        while w > 0 {
            w = step(w, fight, &SOLDIER, STEP);
            t += STEP.0;
        }
        assert!((130_000..140_000).contains(&t), "{t} ms");
    }

    #[test]
    fn rest_recovers_over_several_minutes() {
        // At rest, Skiba's constant comes back as the ~377 s reported for 20 W of recovery.
        let rest = PowerTable::PLACEHOLDER.of(Activity::Rest);
        let tau = recovery_tau_ms(rest, &SOLDIER);
        assert!((376_000..380_000).contains(&tau), "{tau}");
        let mut w = 0;
        for _ in 0..(tau / STEP.0 as u64) {
            w = step(w, rest, &SOLDIER, STEP);
        }
        // After one time constant about 63 % is back.
        let back = w as u64 * 1000 / SOLDIER.reserve_us as u64;
        assert!((628..637).contains(&back), "{back} per mille");
        for _ in 0..20_000 {
            w = step(w, rest, &SOLDIER, STEP);
        }
        assert!(w <= SOLDIER.reserve_us && w > SOLDIER.reserve_us - 2);
        // Working just below CP recovers far more slowly, and the scale applies.
        assert!(recovery_tau_ms(990, &SOLDIER) > 840_000);
        let slow = StaminaParams {
            tau_scale_pm: 1_400,
            ..SOLDIER
        };
        assert_eq!(recovery_tau_ms(rest, &slow), tau * 1_400 / 1_000);
    }
}
