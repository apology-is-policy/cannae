//! Walls in the field.
//!
//! Two rules, and both act only on men near a wall.
//!
//! 1. **No one crosses a wall,** and no man's centre comes nearer to one than half a body's
//!    width. A man who would walk into a wall is put back on the side he came from and keeps
//!    what he gained along it: he slides.
//! 2. **A wall cuts the grid.** A man's presence is shared among the four cells whose centres
//!    surround him. A cell whose centre he cannot reach in a straight line for a wall gets none
//!    of it, and he reads nothing from it; the cells he can reach take its share. So a crowd
//!    against a wall reads its own density right up to the wall, and two crowds either side of
//!    a wall do not feel each other. Without this a man against a wall on a cell's edge reads
//!    about 0.63 of the density he stands in, and a crowd packs half as dense again at the wall
//!    as anywhere else.
//!
//! What it does not do. A passage narrower than a cell has no cell centre inside it, so its
//! men see none of their four cells; they are then left with all four, as if the walls were
//! not there. And enemies either side of a wall still keep a guard's distance: a wall has no
//! thickness, and the guard is wider than a body.

use cannae_core::{Vec2, Wall, WallIndex};

/// How far past the clear distance a man is put, mm: more than rounding can lose.
const SPARE: i64 = 4;
/// The most times a man is put clear of the walls about him in one move. Clear of one may be
/// into another; a corner needs two.
const ROUNDS: usize = 4;

#[derive(Clone, Debug, Default)]
pub(crate) struct Walls {
    index: WallIndex,
    /// Every wall, for a move too long for the index.
    all: Vec<u32>,
    /// The nearest a man's centre comes to a wall, mm.
    clear: i64,
    /// The square of the longest move the index answers for.
    long2: i64,
}

impl Walls {
    /// `cell` is the field's cell and `clear` half a body's width, both mm.
    pub fn new(walls: &[Wall], cell: i32, clear: i32) -> Walls {
        // A man reads cells whose centres lie up to a cell and a half from him, and moves less
        // than a cell and a half in a step unless he is faster than anything on foot.
        let reach = 2 * cell;
        let long = (reach - clear) as i64 - SPARE;
        Walls {
            index: WallIndex::build(walls, reach),
            all: (0..walls.len() as u32).collect(),
            clear: clear as i64,
            long2: long * long,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.all.is_empty()
    }

    /// Whether any wall is near enough to `p` to matter to a man there.
    pub fn near(&self, p: Vec2) -> bool {
        !self.index.near(p).is_empty()
    }

    /// Where a man ends who would go from `from` to `to`.
    pub fn clip(&self, from: Vec2, to: Vec2) -> Vec2 {
        let near = if (to - from).len_sq() > self.long2 {
            &self.all[..]
        } else {
            self.index.near(from)
        };
        if near.is_empty() {
            return to;
        }
        let walls = self.index.walls();
        let r = self.clear;
        let mut q = to;
        for _ in 0..ROUNDS {
            let mut moved = false;
            for &k in near {
                let w = &walls[k as usize];
                let foot = w.nearest(q);
                let off = q - foot;
                let barred = w.bars(from, q);
                if !barred && off.len_sq() >= r * r {
                    continue;
                }
                let dist = off.length();
                let unit = if barred || dist == 0 {
                    // Back to the side he came from.
                    let left = w.left();
                    if w.side(from) < 0 { -left } else { left }
                } else {
                    off.mul_div(1_024, dist)
                };
                q = foot + unit.mul_div(r + SPARE, 1_024);
                moved = true;
            }
            if !moved {
                return q;
            }
        }
        // Walls he cannot be clear of all at once: he stays where he was.
        let free = near.iter().all(|&k| {
            let w = &walls[k as usize];
            !w.bars(from, q) && (q - w.nearest(q)).len_sq() >= r * r
        });
        if free { q } else { from }
    }

    /// For a man standing against a wall: the way out from it, 1024 long. Zero for anyone else.
    /// Such a man cannot give ground toward the wall, whoever leans on him.
    pub fn held(&self, p: Vec2) -> Vec2 {
        let walls = self.index.walls();
        let r = self.clear + 2 * SPARE;
        let mut best = (r * r + 1, Vec2::ZERO);
        for &k in self.index.near(p) {
            let off = p - walls[k as usize].nearest(p);
            let d2 = off.len_sq();
            if d2 < best.0 && d2 > 0 {
                best = (d2, off);
            }
        }
        if best.1 == Vec2::ZERO {
            return Vec2::ZERO;
        }
        best.1.mul_div(1_024, (best.0 as u64).isqrt() as i64)
    }

    /// The shares of a man at `p` among the four cells of his block, of 256, and which of the
    /// four he can see. `centre(k)` is the centre of cell `k`; `w` are his shares with no wall.
    /// The shares returned sum to 256, and a cell he cannot see has none.
    pub fn cut(
        &self,
        p: Vec2,
        centre: impl Fn(usize) -> Vec2,
        w: [i32; 4],
    ) -> ([i32; 4], [bool; 4]) {
        let near = self.index.near(p);
        let walls = self.index.walls();
        let seen: [bool; 4] =
            core::array::from_fn(|k| !near.iter().any(|&n| walls[n as usize].bars(p, centre(k))));
        if seen == [true; 4] || seen == [false; 4] {
            return (w, [true; 4]);
        }
        let visible: i32 = (0..4).filter(|&k| seen[k]).map(|k| w[k]).sum();
        let first = seen.iter().position(|&s| s).unwrap_or(0);
        let mut out = [0; 4];
        if visible == 0 {
            out[first] = 256;
            return (out, seen);
        }
        let (mut rest, mut most) = (256, first);
        for k in 0..4 {
            if seen[k] {
                out[k] = w[k] * 256 / visible;
                rest -= out[k];
                if w[k] > w[most] {
                    most = k;
                }
            }
        }
        out[most] += rest;
        (out, seen)
    }
}

/// For a cell a man cannot see, the cell of his block that stands for it when he reads how the
/// crowd thickens: the one beside it, else the one above or below, else the one across. So the
/// crowd neither pushes him through a wall nor pulls him to it.
/// Whether a man held by a wall, with `out` the way out from it, is being pushed into it by a
/// push along `push`: more into the wall than along it.
pub(crate) fn into(out: Vec2, push: Vec2) -> bool {
    let d = push.dot(out);
    d < 0 && d * d > push.len_sq() * (512 * 512)
}

pub(crate) fn stand_in(seen: &[bool; 4], k: usize) -> usize {
    [k ^ 1, k ^ 2, k ^ 3]
        .into_iter()
        .find(|&j| seen[j])
        .unwrap_or(k)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEAR: i32 = 180;

    fn far(walls: &[Wall], p: Vec2) -> i64 {
        walls
            .iter()
            .map(|w| (p - w.nearest(p)).length())
            .min()
            .unwrap()
    }

    /// A rampart along x = 0 from y = 1.5 m up, and a gate's side running back from its end.
    fn post() -> Vec<Wall> {
        vec![
            Wall::new(Vec2::new(0, 1_500), Vec2::new(0, 60_000)),
            Wall::new(Vec2::new(0, 1_500), Vec2::new(-3_000, 1_500)),
        ]
    }

    #[test]
    fn a_man_walking_into_a_wall_stops_clear_of_it_and_slides() {
        let walls = post();
        let w = Walls::new(&walls, 1_024, CLEAR);
        let from = Vec2::new(400, 5_000);
        // Straight at it, and slanting through it.
        let stopped = w.clip(from, Vec2::new(-400, 5_000));
        assert_eq!(stopped.y, 5_000);
        assert!((CLEAR..CLEAR + 8).contains(&stopped.x), "{stopped:?}");
        let slid = w.clip(from, Vec2::new(-400, 4_200));
        assert_eq!(slid.y, 4_200);
        assert!((CLEAR..CLEAR + 8).contains(&slid.x), "{slid:?}");
        // Short of it, nothing changes.
        let short = Vec2::new(250, 4_800);
        assert_eq!(w.clip(from, short), short);
        // Far from any wall, nothing is looked at.
        let (a, b) = (Vec2::new(30_000, 0), Vec2::new(29_000, 500));
        assert!(!w.near(a));
        assert_eq!(w.clip(a, b), b);
    }

    #[test]
    fn a_man_rounds_a_post_and_never_cuts_through_it() {
        let walls = post();
        let w = Walls::new(&walls, 1_024, CLEAR);
        // From outside the rampart beside the gate, in steps toward a point inside the gate.
        let mut p = Vec2::new(300, 2_200);
        let goal = Vec2::new(-2_000, 800);
        for _ in 0..40 {
            let to = p + (goal - p).clamp_len(300);
            let q = w.clip(p, to);
            assert!(walls.iter().all(|wall| !wall.bars(p, q)), "{p:?} -> {q:?}");
            assert!(far(&walls, q) >= CLEAR as i64, "{q:?}");
            p = q;
        }
        assert!(p.dist_sq(goal) < 100 * 100, "he did not get there: {p:?}");
    }

    #[test]
    fn a_move_too_long_for_the_index_is_still_stopped() {
        let walls = post();
        let w = Walls::new(&walls, 1_024, CLEAR);
        let from = Vec2::new(9_000, 5_000);
        assert!(!w.near(from));
        let q = w.clip(from, Vec2::new(-9_000, 5_000));
        assert!(q.x >= CLEAR && q.x < CLEAR + 8, "{q:?}");
    }

    #[test]
    fn a_man_in_a_corner_too_tight_for_him_stays_where_he_was() {
        // Two walls 200 mm apart: no point between them is clear of both.
        let walls = vec![
            Wall::new(Vec2::new(0, 0), Vec2::new(5_000, 0)),
            Wall::new(Vec2::new(0, 200), Vec2::new(5_000, 200)),
        ];
        let w = Walls::new(&walls, 1_024, CLEAR);
        let from = Vec2::new(-1_000, 100);
        assert_eq!(w.clip(from, Vec2::new(1_000, 100)), from);
    }

    #[test]
    fn a_wall_on_a_cells_edge_gives_all_of_a_man_to_his_own_side() {
        let walls = vec![Wall::new(Vec2::new(0, -10_000), Vec2::new(0, 10_000))];
        let w = Walls::new(&walls, 1_024, CLEAR);
        // Cell centres at -512 and 512 east, 512 and 1536 north.
        let centre =
            |k: usize| Vec2::new(-512 + 1_024 * (k & 1) as i32, 512 + 1_024 * (k >> 1) as i32);
        let even = [64, 64, 64, 64];
        let (east, seen) = w.cut(Vec2::new(200, 1_024), centre, even);
        assert_eq!(seen, [false, true, false, true]);
        assert_eq!(east, [0, 128, 0, 128]);
        let (west, seen) = w.cut(Vec2::new(-200, 1_024), centre, even);
        assert_eq!(seen, [true, false, true, false]);
        assert_eq!(west, [128, 0, 128, 0]);
        // Shares that do not divide evenly still sum to a whole man.
        let (odd, _) = w.cut(Vec2::new(200, 700), centre, [50, 150, 14, 42]);
        assert_eq!(odd.iter().sum::<i32>(), 256);
        assert_eq!((odd[0], odd[2]), (0, 0));
        // All of him on cells he cannot see: he goes to one he can.
        let (moved, _) = w.cut(Vec2::new(200, 700), centre, [256, 0, 0, 0]);
        assert_eq!(moved, [0, 256, 0, 0]);
        // No wall between him and any of them.
        let far_off = |k: usize| centre(k) + Vec2::new(4_096, 0);
        assert_eq!(
            w.cut(Vec2::new(4_296, 1_024), far_off, even),
            (even, [true; 4])
        );
    }

    #[test]
    fn a_passage_narrower_than_a_cell_is_left_uncut() {
        let walls = vec![
            Wall::new(Vec2::new(-5_000, 300), Vec2::new(5_000, 300)),
            Wall::new(Vec2::new(-5_000, -300), Vec2::new(5_000, -300)),
        ];
        let w = Walls::new(&walls, 1_024, CLEAR);
        let centre = |k: usize| {
            Vec2::new(
                -512 + 1_024 * (k & 1) as i32,
                -512 + 1_024 * (k >> 1) as i32,
            )
        };
        let shares = [10, 20, 100, 126];
        assert_eq!(w.cut(Vec2::new(0, 0), centre, shares), (shares, [true; 4]));
    }

    #[test]
    fn a_man_against_a_wall_is_held_and_knows_which_way_is_out() {
        let walls = post();
        let w = Walls::new(&walls, 1_024, CLEAR);
        let out = w.held(Vec2::new(CLEAR + 4, 5_000));
        assert_eq!(out, Vec2::new(1_024, 0));
        assert_eq!(w.held(Vec2::new(CLEAR + 40, 5_000)), Vec2::ZERO);
        assert_eq!(w.held(Vec2::new(30_000, 0)), Vec2::ZERO);
        // Pushed straight at the wall or mostly at it, he cannot give; along it or away, he can.
        assert!(into(out, Vec2::new(-300, 0)));
        assert!(into(out, Vec2::new(-300, 300)));
        assert!(!into(out, Vec2::new(-100, 300)));
        assert!(!into(out, Vec2::new(0, 300)));
        assert!(!into(out, Vec2::new(300, 0)));
        assert!(!into(Vec2::ZERO, Vec2::new(-300, 0)));
    }

    #[test]
    fn a_hidden_cell_is_stood_in_for_by_one_he_can_see() {
        assert_eq!(stand_in(&[false, true, false, true], 0), 1);
        assert_eq!(stand_in(&[false, true, false, true], 2), 3);
        assert_eq!(stand_in(&[false, false, true, true], 0), 2);
        assert_eq!(stand_in(&[false, false, false, true], 0), 3);
    }
}
