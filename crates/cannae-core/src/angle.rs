//! Directions as integer fractions of a turn, with integer sine, cosine and atan2.

use crate::units::Vec2;

/// A direction: 65,536 units to the full turn, counter-clockwise from +x.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Turn(pub u16);

const Q30: i64 = 1 << 30;
/// round(pi/2 * 2^30)
const HALF_PI_Q30: i64 = 1_686_629_713;
/// round(pi/4 * 2^30)
const QUARTER_PI_Q30: i64 = 843_314_857;
/// round(65536 / (2 pi) * 2^16): radians in Q30 to turn units, after a shift of 46.
const TURN_PER_RAD_Q16: i64 = 683_565_276;
/// The coefficients 0.2447 and 0.0663 of the atan approximation below, in Q30.
const ATAN_A_Q30: i64 = 262_744_624;
const ATAN_B_Q30: i64 = 71_189_083;

/// sin(u / 16384 * pi/2) in Q30 for u in 0..=16384. Taylor series to x^9: the error at pi/2
/// is about 4e-6, below Q15 resolution.
fn sin_quarter_q30(u: i64) -> i64 {
    let x = u * HALF_PI_Q30 / 16384;
    let x2 = (x * x) >> 30;
    let mut t = Q30 - x2 / 72;
    t = Q30 - ((x2 * t) >> 30) / 42;
    t = Q30 - ((x2 * t) >> 30) / 20;
    t = Q30 - ((x2 * t) >> 30) / 6;
    (x * t) >> 30
}

/// atan(z) for z in [0, 1], both in Q30. The approximation
/// atan(z) = pi/4 z - z (z - 1)(0.2447 + 0.0663 z) is within about 0.0015 rad (0.09 degrees),
/// ample for a man's facing.
fn atan_unit_q30(z: i64) -> i64 {
    let term = (z * (z - Q30)) >> 30;
    let poly = ATAN_A_Q30 + ((ATAN_B_Q30 * z) >> 30);
    ((QUARTER_PI_Q30 * z) >> 30) - ((term * poly) >> 30)
}

impl Turn {
    pub const QUARTER: u16 = 16384;
    pub const HALF: u16 = 32768;

    /// The sine in Q15: 32768 is 1.0. Exactly antisymmetric.
    pub fn sin_q15(self) -> i32 {
        let t = self.0 as i64;
        let u = t & 0x3FFF;
        let (mag, neg) = match t >> 14 {
            0 => (sin_quarter_q30(u), false),
            1 => (sin_quarter_q30(16384 - u), false),
            2 => (sin_quarter_q30(u), true),
            _ => (sin_quarter_q30(16384 - u), true),
        };
        let q15 = ((mag + (1 << 14)) >> 15) as i32;
        if neg { -q15 } else { q15 }
    }

    pub fn cos_q15(self) -> i32 {
        Turn(self.0.wrapping_add(Self::QUARTER)).sin_q15()
    }

    /// A vector of length `len` (to within a unit) pointing this way.
    pub fn unit(self, len: i32) -> Vec2 {
        Vec2::new(
            ((self.cos_q15() as i64 * len as i64) >> 15) as i32,
            ((self.sin_q15() as i64 * len as i64) >> 15) as i32,
        )
    }

    /// The direction of `v`, or `None` for the zero vector.
    pub fn of(v: Vec2) -> Option<Turn> {
        if v == Vec2::ZERO {
            return None;
        }
        let (x, y) = (v.x as i64, v.y as i64);
        let (ax, ay) = (x.abs(), y.abs());
        let (mn, mx) = if ax >= ay { (ay, ax) } else { (ax, ay) };
        let rad = atan_unit_q30((mn << 30) / mx);
        let mut t = (rad * TURN_PER_RAD_Q16 + (1 << 45)) >> 46;
        if ay > ax {
            t = 16384 - t;
        }
        if x < 0 {
            t = 32768 - t;
        }
        if y < 0 {
            t = 65536 - t;
        }
        Some(Turn((t & 0xFFFF) as u16))
    }

    /// The signed smallest rotation from `self` to `to`, in turn units.
    pub fn diff(self, to: Turn) -> i16 {
        to.0.wrapping_sub(self.0) as i16
    }

    /// Rotates toward `target` by at most `max_step` units.
    pub fn rotate_toward(self, target: Turn, max_step: u16) -> Turn {
        let d = self.diff(target) as i32;
        let m = max_step as i32;
        if d.abs() <= m {
            target
        } else {
            Turn(self.0.wrapping_add_signed(d.signum() as i16 * m as i16))
        }
    }
}

#[cfg(test)]
#[allow(clippy::float_arithmetic)]
mod tests {
    use super::*;
    use core::f64::consts::TAU;

    #[test]
    fn sine_and_cosine_match_f64() {
        for t in (0..=65535u32).step_by(7) {
            let a = t as f64 / 65536.0 * TAU;
            let s = Turn(t as u16).sin_q15() as f64 / 32768.0;
            let c = Turn(t as u16).cos_q15() as f64 / 32768.0;
            assert!((s - a.sin()).abs() < 1e-4, "sin at {t}: {s} vs {}", a.sin());
            assert!((c - a.cos()).abs() < 1e-4, "cos at {t}");
        }
        assert_eq!(Turn(Turn::QUARTER).sin_q15(), 32768);
        assert_eq!(Turn(0).sin_q15(), 0);
        assert_eq!(
            Turn(1000).sin_q15(),
            -Turn(0u16.wrapping_sub(1000)).sin_q15()
        );
    }

    #[test]
    fn atan2_matches_f64() {
        let mut worst = 0i32;
        for t in (0..65536u32).step_by(97) {
            let v = Turn(t as u16).unit(1_000_000);
            let got = Turn::of(v).unwrap();
            let want = (v.y as f64).atan2(v.x as f64).rem_euclid(TAU) / TAU * 65536.0;
            let err = (Turn((want.round() as u32 & 0xFFFF) as u16).diff(got) as i32).abs();
            worst = worst.max(err);
        }
        // 0.0015 rad is about 16 units.
        assert!(worst <= 20, "worst error {worst} units");
        assert_eq!(Turn::of(Vec2::ZERO), None);
        assert_eq!(Turn::of(Vec2::new(5, 0)), Some(Turn(0)));
        assert_eq!(Turn::of(Vec2::new(0, 5)), Some(Turn(16384)));
        assert_eq!(Turn::of(Vec2::new(-5, 0)), Some(Turn(32768)));
        assert_eq!(Turn::of(Vec2::new(0, -5)), Some(Turn(49152)));
    }

    #[test]
    fn rotation_takes_the_short_way() {
        let a = Turn(65000);
        let b = Turn(500);
        assert_eq!(a.diff(b), 1036);
        assert_eq!(a.rotate_toward(b, 100), Turn(65100));
        assert_eq!(a.rotate_toward(b, 2000), b);
        assert_eq!(b.rotate_toward(a, 600), Turn(65436));
    }
}
