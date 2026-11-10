//! Walls: straight segments that no one passes through, such as a camp's rampart and gate posts,
//! or the sides of a bottleneck in the passage study (S0.8).
//!
//! A scenario lists them, and each substrate is handed them when it is built, so the
//! `Substrate` trait does not change. They are part of the ground, not of anyone's state, and
//! never move.

use crate::units::Vec2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Wall {
    pub a: Vec2,
    pub b: Vec2,
}

impl Wall {
    pub const fn new(a: Vec2, b: Vec2) -> Wall {
        Wall { a, b }
    }

    /// The point of the wall nearest `p`.
    pub fn nearest(&self, p: Vec2) -> Vec2 {
        let ab = self.b - self.a;
        let len2 = ab.len_sq();
        if len2 == 0 {
            return self.a;
        }
        let t = (p - self.a).dot(ab).clamp(0, len2);
        self.a + ab.mul_div(t, len2)
    }

    /// A unit normal 1024 long, to the left of the way from `a` to `b`: the side a man standing
    /// exactly on the wall is put on.
    pub fn left(&self) -> Vec2 {
        let ab = self.b - self.a;
        if ab == Vec2::ZERO {
            return Vec2::new(0, 1_024);
        }
        Vec2::new(-ab.y, ab.x).with_len(1_024)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_nearest_point_is_on_the_segment() {
        let w = Wall::new(Vec2::new(0, 0), Vec2::new(10_000, 0));
        assert_eq!(w.nearest(Vec2::new(3_000, 500)), Vec2::new(3_000, 0));
        assert_eq!(w.nearest(Vec2::new(-2_000, 500)), Vec2::new(0, 0));
        assert_eq!(w.nearest(Vec2::new(12_000, -500)), Vec2::new(10_000, 0));
        assert_eq!(w.left(), Vec2::new(0, 1_024));
        let point = Wall::new(Vec2::new(5, 5), Vec2::new(5, 5));
        assert_eq!(point.nearest(Vec2::new(100, 100)), Vec2::new(5, 5));
    }
}
