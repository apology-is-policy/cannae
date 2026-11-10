//! Fixed-point probability and the one transcendental the engines need, exp(-x).

use crate::units::SimMs;

const ONE_Q32: u64 = 1 << 32;

/// A probability in Q32: `1 << 32` is certainty. Stored in `u64` so that certainty is exact.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Prob(u64);

impl Prob {
    pub const ZERO: Prob = Prob(0);
    pub const ONE: Prob = Prob(ONE_Q32);

    /// Clamped to [0, 1].
    pub const fn from_q32(v: u64) -> Self {
        Prob(if v > ONE_Q32 { ONE_Q32 } else { v })
    }

    /// `num / den`, rounded down and clamped to [0, 1]. Panics if `den` is zero.
    pub const fn from_ratio(num: u64, den: u64) -> Self {
        let q = ((num as u128) << 32) / den as u128;
        Prob::from_q32(if q > ONE_Q32 as u128 {
            ONE_Q32
        } else {
            q as u64
        })
    }

    /// The chance that an event with a constant hazard `rate` (per second, Q16) happens within
    /// `dt`: 1 - exp(-rate * dt). This is how a hazard becomes a per-step probability.
    pub fn from_rate(rate_per_s_q16: u32, dt: SimMs) -> Self {
        let x = ((rate_per_s_q16 as u128 * dt.0 as u128) << 16) / 1000;
        let x = if x > u64::MAX as u128 {
            u64::MAX
        } else {
            x as u64
        };
        Prob(ONE_Q32 - exp_neg_q32(x))
    }

    pub const fn q32(self) -> u64 {
        self.0
    }

    /// Whether a uniform `u32` draw falls inside this probability.
    pub const fn hits(self, draw: u32) -> bool {
        (draw as u64) < self.0
    }

    pub const fn complement(self) -> Prob {
        Prob(ONE_Q32 - self.0)
    }

    pub const fn and(self, o: Prob) -> Prob {
        Prob(((self.0 as u128 * o.0 as u128) >> 32) as u64)
    }
}

/// exp(-x) for x >= 0, with x and the result in Q32.
///
/// Internally Q62: range reduction halves x until it is below 1/16, a Taylor series to the
/// sixth power evaluates it there, and repeated squaring undoes the halving. The working
/// precision keeps the result within a few units of the last Q32 place.
pub fn exp_neg_q32(x: u64) -> u64 {
    // exp(-23) is below 2^-32.
    if x >= 23 << 32 {
        return 0;
    }
    const ONE: u128 = 1 << 62;
    let x62 = (x as u128) << 30;
    let bits = 128 - x62.leading_zeros();
    let k = bits.saturating_sub(58);
    let y = x62 >> k;
    let y2 = (y * y) >> 62;
    let y3 = (y2 * y) >> 62;
    let y4 = (y3 * y) >> 62;
    let y5 = (y4 * y) >> 62;
    let y6 = (y5 * y) >> 62;
    let mut e = ONE + y2 / 2 + y4 / 24 + y6 / 720 - y - y3 / 6 - y5 / 120;
    for _ in 0..k {
        e = (e * e + (1 << 61)) >> 62;
    }
    ((e + (1 << 29)) >> 30) as u64
}

#[cfg(test)]
#[allow(clippy::float_arithmetic)]
mod tests {
    use super::*;

    fn to_f(q: u64) -> f64 {
        q as f64 / ONE_Q32 as f64
    }

    #[test]
    fn exp_neg_matches_f64() {
        // Q32 limits absolute precision to about 2e-10, so small results are checked in
        // absolute terms and the rest in relative terms.
        let (mut worst_abs, mut worst_rel) = (0.0f64, 0.0f64);
        for i in 0..=23_000u64 {
            let x = i * ONE_Q32 / 1000; // 0.000 .. 23.000
            let got = to_f(exp_neg_q32(x));
            let want = (-(x as f64) / ONE_Q32 as f64).exp();
            worst_abs = worst_abs.max((got - want).abs());
            if want > 1e-3 {
                worst_rel = worst_rel.max((got - want).abs() / want);
            }
        }
        assert!(worst_abs < 2e-9, "worst absolute error {worst_abs}");
        assert!(worst_rel < 1e-6, "worst relative error {worst_rel}");
        assert_eq!(exp_neg_q32(0), ONE_Q32);
        assert_eq!(exp_neg_q32(30 << 32), 0);
    }

    #[test]
    fn rate_to_step_probability() {
        // A hazard of 1 per second over a quarter second: 1 - e^-0.25 = 0.2212.
        let p = Prob::from_rate(1 << 16, SimMs(250));
        assert!((to_f(p.q32()) - 0.221_199).abs() < 1e-6);
        assert_eq!(Prob::from_rate(0, SimMs(250)), Prob::ZERO);
        // A huge hazard is near certainty but never above it.
        assert!(Prob::from_rate(u32::MAX, SimMs(1000)) <= Prob::ONE);
    }

    #[test]
    fn hits_and_bounds() {
        assert!(!Prob::ZERO.hits(0));
        assert!(Prob::ONE.hits(u32::MAX));
        let half = Prob::from_ratio(1, 2);
        assert!(half.hits(u32::MAX / 2));
        assert!(!half.hits(u32::MAX / 2 + 1));
        assert_eq!(half.and(half), Prob::from_ratio(1, 4));
        assert_eq!(half.complement(), half);
    }
}
