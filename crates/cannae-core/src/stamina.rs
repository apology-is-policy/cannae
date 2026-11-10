//! Stamina: the critical-power / W' model of exercise physiology, shared by both engines.
//!
//! Above critical power the anaerobic reserve W' drains joule for joule. Below it, the spent part
//! recovers exponentially with a time constant that is shorter the further below critical power
//! the man works (Skiba, Chidnok, Vanhatalo and Jones, "Modeling the expenditure and
//! reconstitution of work capacity above critical power", Med. Sci. Sports Exerc. 44(8):
//! 1526-1532, 2012). The citation is confirmed from PubMed; the formula only through a secondary
//! source (docs/slice0/lethality-skeletal.md, B1). The paper computes tau from the session's mean
//! sub-CP power; this applies it to the current power at each step, a simplification. Skiba et
//! al.'s later differential model (Eur. J. Appl. Physiol. 115, 2015) is the alternative.

use crate::fixed::exp_neg_q32;
use crate::units::{MilliJ, MilliW, SimMs};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StaminaParams {
    /// Critical power.
    pub cp: MilliW,
    /// W' capacity.
    pub w_max: MilliJ,
}

/// Recovery time constant in milliseconds when working `dcp` below critical power:
/// tau = 546 exp(-0.01 DCP) + 316 seconds, with DCP in watts.
pub fn recovery_tau_ms(dcp: MilliW) -> u64 {
    let x = ((dcp.max(0) as u64) << 32) / 100_000;
    ((546_000u64 as u128 * exp_neg_q32(x) as u128) >> 32) as u64 + 316_000
}

/// W' after working at `power` for `dt`. Never below zero or above capacity. At zero the man
/// cannot sustain more than critical power; enforcing that is the caller's business.
pub fn step(w: MilliJ, power: MilliW, p: &StaminaParams, dt: SimMs) -> MilliJ {
    if power > p.cp {
        let drain = (power - p.cp) as i64 * dt.0 as i64 / 1000;
        (w as i64 - drain).max(0) as MilliJ
    } else {
        let tau = recovery_tau_ms(p.cp - power);
        let keep = exp_neg_q32(((dt.0 as u64) << 32) / tau);
        let deficit = (p.w_max - w).max(0) as u128;
        p.w_max - ((deficit * keep as u128) >> 32) as MilliJ
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOLDIER: StaminaParams = StaminaParams {
        cp: 250_000,
        w_max: 20_000_000,
    };
    const STEP: SimMs = SimMs(250);

    #[test]
    fn hard_fighting_empties_in_minutes() {
        // 200 W above critical power drains 20 kJ in 100 s: inside the 1-3 minutes of 04.
        let mut w = SOLDIER.w_max;
        let mut t = 0;
        while w > 0 {
            w = step(w, 450_000, &SOLDIER, STEP);
            t += STEP.0;
        }
        assert_eq!(t, 100_000);
    }

    #[test]
    fn rest_recovers_over_several_minutes() {
        // Standing at 100 W against a critical power of 250 W: tau is about 438 s.
        let tau = recovery_tau_ms(150_000);
        assert!((437_000..439_000).contains(&tau), "{tau}");
        let mut w = 0;
        for _ in 0..(tau / STEP.0 as u64) {
            w = step(w, 100_000, &SOLDIER, STEP);
        }
        // After one time constant about 63 % is back.
        let back = w as i64 * 1000 / SOLDIER.w_max as i64;
        assert!((628..637).contains(&back), "{back} per mille");
        for _ in 0..20_000 {
            w = step(w, 100_000, &SOLDIER, STEP);
        }
        assert!(w <= SOLDIER.w_max && w > SOLDIER.w_max - 2);
    }
}
