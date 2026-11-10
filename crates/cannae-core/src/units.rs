//! Integer units: millimetres, milliseconds, millijoules, milliwatts.

use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

/// A position in millimetres, or a velocity in millimetres per second, by context.
///
/// A battlefield 20 km across fits in `i32` many times over, and a squared distance across it
/// fits in `i64`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2 { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }

    pub const fn dot(self, o: Vec2) -> i64 {
        self.x as i64 * o.x as i64 + self.y as i64 * o.y as i64
    }

    /// The z component of the 3D cross product. Positive when `o` lies counter-clockwise of
    /// `self`, which is how "on his left" and "on his right" are decided.
    pub const fn cross(self, o: Vec2) -> i64 {
        self.x as i64 * o.y as i64 - self.y as i64 * o.x as i64
    }

    pub const fn len_sq(self) -> i64 {
        self.dot(self)
    }

    /// Length, rounded down.
    pub fn length(self) -> i64 {
        (self.len_sq() as u64).isqrt() as i64
    }

    pub fn dist_sq(self, o: Vec2) -> i64 {
        (o - self).len_sq()
    }

    /// The same direction with the given length, truncated toward zero. The zero vector stays
    /// zero.
    pub fn with_len(self, len: i32) -> Vec2 {
        let l = self.length();
        if l == 0 {
            return Vec2::ZERO;
        }
        Vec2 {
            x: (self.x as i64 * len as i64 / l) as i32,
            y: (self.y as i64 * len as i64 / l) as i32,
        }
    }

    /// Clamps the length to at most `max`.
    pub fn clamp_len(self, max: i32) -> Vec2 {
        if self.len_sq() > max as i64 * max as i64 {
            self.with_len(max)
        } else {
            self
        }
    }

    /// Scales by `num / den`, truncated toward zero.
    pub fn mul_div(self, num: i64, den: i64) -> Vec2 {
        Vec2 {
            x: (self.x as i64 * num / den) as i32,
            y: (self.y as i64 * num / den) as i32,
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x + o.x, self.y + o.y)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, o: Vec2) -> Vec2 {
        Vec2::new(self.x - o.x, self.y - o.y)
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.x, -self.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, o: Vec2) {
        *self = *self + o;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, o: Vec2) {
        *self = *self - o;
    }
}

/// Simulated time in milliseconds. `u32` covers 49 days, far beyond any battle.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SimMs(pub u32);

impl SimMs {
    pub const fn from_secs(s: u32) -> Self {
        SimMs(s * 1000)
    }
}

/// Energy in millijoules. A W' capacity of 20-30 kJ is 2-3 x 10^7, well inside `i32`.
pub type MilliJ = i32;

/// Power in milliwatts. A critical power of 250 W is 250,000.
pub type MilliW = i32;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lengths_and_products() {
        let a = Vec2::new(3000, 4000);
        assert_eq!(a.length(), 5000);
        assert_eq!(a.with_len(1000), Vec2::new(600, 800));
        assert_eq!(Vec2::ZERO.with_len(1000), Vec2::ZERO);
        assert_eq!(a.clamp_len(10_000), a);
        assert_eq!(a.clamp_len(500), Vec2::new(300, 400));
        // Counter-clockwise is positive: +y lies to the left of +x.
        assert!(Vec2::new(1, 0).cross(Vec2::new(0, 1)) > 0);
    }

    #[test]
    fn far_apart_points_do_not_overflow() {
        let a = Vec2::new(-10_000_000, -10_000_000);
        let b = Vec2::new(10_000_000, 10_000_000);
        assert_eq!(a.dist_sq(b), 2 * 20_000_000i64 * 20_000_000);
    }
}
