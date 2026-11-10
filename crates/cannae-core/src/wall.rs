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

    /// Which side of the wall's line `p` is on: above zero to the left of the way from `a` to
    /// `b`, below zero to the right, zero on the line.
    pub fn side(&self, p: Vec2) -> i64 {
        (self.b - self.a).cross(p - self.a)
    }

    /// Whether the wall bars the straight way from `p` to `q`: `p` is off its line, `q` is on
    /// the line or beyond it, and the way meets the wall between its ends or at one of them.
    pub fn bars(&self, p: Vec2, q: Vec2) -> bool {
        let (at_p, at_q) = (self.side(p).signum(), self.side(q).signum());
        if at_p == 0 || at_p == at_q {
            return false;
        }
        let way = q - p;
        way.cross(self.a - p).signum() != way.cross(self.b - p).signum()
    }
}

/// The side of a cell in the wall index, mm.
const INDEX_CELL_MM: i32 = 4_000;

/// The walls binned on a coarse grid, so that a man checks only the walls that may reach him,
/// not all of them. A camp's rampart and gates are a dozen walls, and checking each of 150,000
/// men against every one at every pass would cost more than all the contacts. Walls do not
/// move, so the index is rebuilt only when they change.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WallIndex {
    /// The walls and the reach it was built for.
    walls: Vec<Wall>,
    reach: i32,
    origin: Vec2,
    w: i32,
    h: i32,
    /// For each cell, the walls that come within `reach` of some point in it.
    cells: Vec<Vec<u32>>,
}

impl WallIndex {
    /// An index of `walls` that answers for points within `reach` of them.
    pub fn build(walls: &[Wall], reach: i32) -> WallIndex {
        let mut ix = WallIndex {
            walls: walls.to_vec(),
            reach,
            ..Default::default()
        };
        if walls.is_empty() {
            return ix;
        }
        let lo = |f: fn(&Vec2) -> i32| walls.iter().map(|w| f(&w.a).min(f(&w.b))).min();
        let hi = |f: fn(&Vec2) -> i32| walls.iter().map(|w| f(&w.a).max(f(&w.b))).max();
        let (x0, y0) = (lo(|v| v.x).unwrap() - reach, lo(|v| v.y).unwrap() - reach);
        let (x1, y1) = (hi(|v| v.x).unwrap() + reach, hi(|v| v.y).unwrap() + reach);
        ix.origin = Vec2::new(x0, y0);
        ix.w = (x1 - x0) / INDEX_CELL_MM + 1;
        ix.h = (y1 - y0) / INDEX_CELL_MM + 1;
        ix.cells = vec![Vec::new(); (ix.w * ix.h) as usize];
        // A point within `reach` of the wall lies in a cell whose centre is within `reach`
        // plus half the cell's diagonal of it; a whole cell's side is more than that.
        let lim = (reach + INDEX_CELL_MM) as i64;
        for (k, wall) in walls.iter().enumerate() {
            let (cx0, cy0) = ix.cell(Vec2::new(
                wall.a.x.min(wall.b.x) - reach,
                wall.a.y.min(wall.b.y) - reach,
            ));
            let (cx1, cy1) = ix.cell(Vec2::new(
                wall.a.x.max(wall.b.x) + reach,
                wall.a.y.max(wall.b.y) + reach,
            ));
            for cy in cy0..=cy1 {
                for cx in cx0..=cx1 {
                    let centre = ix.origin
                        + Vec2::new(
                            cx * INDEX_CELL_MM + INDEX_CELL_MM / 2,
                            cy * INDEX_CELL_MM + INDEX_CELL_MM / 2,
                        );
                    if (centre - wall.nearest(centre)).len_sq() <= lim * lim {
                        ix.cells[(cy * ix.w + cx) as usize].push(k as u32);
                    }
                }
            }
        }
        ix
    }

    fn cell(&self, p: Vec2) -> (i32, i32) {
        (
            (p.x - self.origin.x).div_euclid(INDEX_CELL_MM),
            (p.y - self.origin.y).div_euclid(INDEX_CELL_MM),
        )
    }

    /// The walls it was built from.
    pub fn walls(&self) -> &[Wall] {
        &self.walls
    }

    /// The reach it was built for.
    pub fn reach(&self) -> i32 {
        self.reach
    }

    /// The walls that may come within `reach` of `p`, as indices into `walls()`.
    pub fn near(&self, p: Vec2) -> &[u32] {
        let (cx, cy) = self.cell(p);
        if cx < 0 || cy < 0 || cx >= self.w || cy >= self.h {
            return &[];
        }
        &self.cells[(cy * self.w + cx) as usize]
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

    #[test]
    fn a_wall_bars_the_way_across_it_and_no_other() {
        let w = Wall::new(Vec2::new(0, 0), Vec2::new(10_000, 0));
        let from = Vec2::new(5_000, 300);
        assert!(w.side(from) > 0 && w.side(Vec2::new(5_000, -300)) < 0);
        // Across it, onto it, and through either end.
        assert!(w.bars(from, Vec2::new(5_000, -300)));
        assert!(w.bars(from, Vec2::new(4_000, 0)));
        assert!(w.bars(Vec2::new(-300, 300), Vec2::new(300, -300)));
        assert!(w.bars(Vec2::new(9_700, 300), Vec2::new(10_300, -300)));
        // Along it, away from it, short of it, and past its end.
        assert!(!w.bars(from, Vec2::new(8_000, 300)));
        assert!(!w.bars(from, Vec2::new(5_000, 900)));
        assert!(!w.bars(from, Vec2::new(5_000, 1)));
        assert!(!w.bars(Vec2::new(10_200, 300), Vec2::new(10_400, -300)));
        // A man standing on the line is on neither side, and a post has no sides.
        assert!(!w.bars(Vec2::new(5_000, 0), Vec2::new(5_000, -300)));
        let post = Wall::new(Vec2::new(5, 5), Vec2::new(5, 5));
        assert!(!post.bars(Vec2::new(0, 0), Vec2::new(10, 10)));
    }

    #[test]
    fn the_wall_index_never_misses_a_wall_in_reach() {
        // Walls at all angles, short and long, and points scattered over and around them: every
        // wall that reaches a point must be among those the index offers for it.
        let mut x = 12_345u64;
        let mut next = |m: i32| {
            x = x
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            ((x >> 33) % (2 * m as u64)) as i32 - m
        };
        let walls: Vec<Wall> = (0..40)
            .map(|_| {
                let a = Vec2::new(next(60_000), next(60_000));
                let b = a + Vec2::new(next(20_000), next(20_000));
                Wall::new(a, b)
            })
            .chain([Wall::new(Vec2::new(5, 5), Vec2::new(5, 5))])
            .collect();
        let reach = 300;
        let ix = WallIndex::build(&walls, reach);
        let mut checked = 0;
        for _ in 0..200_000 {
            let p = Vec2::new(next(85_000), next(85_000));
            let near = ix.near(p);
            for (k, w) in walls.iter().enumerate() {
                if (p - w.nearest(p)).len_sq() < (reach as i64).pow(2) {
                    checked += 1;
                    assert!(near.contains(&(k as u32)), "wall {k} missed at {p:?}");
                }
            }
        }
        assert!(
            checked > 100,
            "too few points in reach of a wall: {checked}"
        );
    }
}
