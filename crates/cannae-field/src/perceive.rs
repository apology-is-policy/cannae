//! The field engine's perception.
//!
//! It gives exactly the Views of the reference (`cannae_core::GridPerception`); the conformance
//! tests hold it to that. The difference is in what it keeps. Each canonical cell holds counts
//! by side and posture, and a few flags, so that the test for dormancy is one or two loads per
//! man, and a View adds cells' counts and looks at single men only where it must: for enemies
//! within reach, for crowding, and for a closer.

use crate::tiles::{NONE, TILE, TILE_SHIFT, Tiles};
use cannae_core::view::{
    BodyState, CELL_MM, CROWD_CAP, CROWD_MM, Counts, FrameQuery, Group, LeaderSeen, MAX_NEAR,
    NearEnemy, Order, REACH_MM, Rel, Sector, Shocks, Standard, StandardSeen, WINDOW, axis, cell_of,
    strikes_first,
};
use cannae_core::{
    Bodies, BodyId, Coarse, Event, EventKind, ManId, Perceive, Posture, Role, Turn, Vec2, View,
};

const SPAN: usize = (2 * WINDOW + 1) as usize;
const CELLS: usize = SPAN * SPAN;

/// Each cell is cut into four quarters for the lists of single men, so that a search for the
/// nearest enemies looks at the men of a few quarters and not of whole cells.
const QUARTER_MM: i32 = CELL_MM / 2;
const _: () = assert!(CELL_MM % 2 == 0);
/// Rings of quarters about a man's own that cover his reach.
const RINGS: i32 = (REACH_MM as i32 + QUARTER_MM - 1) / QUARTER_MM;
/// Lists to a cell: four quarters by two sides.
const LISTS: usize = 8;

/// Quarters about a man's own that can hold someone within his reach, in the order they are
/// searched: ring by ring outward.
const SPOTS: usize = ((2 * RINGS + 1) * (2 * RINGS + 1)) as usize;
const _: () = assert!(SPOTS <= 64 && RINGS < 2 * WINDOW);
/// The spots of his own quarter and the ring about it: all that can hold a man within
/// crowding distance.
const CROWD_SPOTS: u64 = (1 << 9) - 1;
const _: () = assert!(CROWD_MM <= QUARTER_MM as i64);

/// One quarter as seen from a man's own: how many quarters east and north, and on which ring.
#[derive(Clone, Copy)]
struct Spot {
    dx: i8,
    dy: i8,
    ring: i8,
}

const fn spots() -> [Spot; SPOTS] {
    let mut out = [Spot {
        dx: 0,
        dy: 0,
        ring: 0,
    }; SPOTS];
    let mut k = 0;
    let mut r = 0;
    while r <= RINGS {
        let mut dy = -r;
        while dy <= r {
            let mut dx = -r;
            while dx <= r {
                if dx.abs() == r || dy.abs() == r {
                    out[k] = Spot {
                        dx: dx as i8,
                        dy: dy as i8,
                        ring: r as i8,
                    };
                    k += 1;
                }
                dx += 1;
            }
            dy += 1;
        }
        r += 1;
    }
    out
}

static SPOT: [Spot; SPOTS] = spots();

/// For a man in quarter `p` of his cell (0 to 3): which spots the occupied quarters of window
/// cell `c` are, given the cell's four quarters as bits. A quarter beyond his reach is no spot.
const fn spot_masks() -> [[[u64; 16]; CELLS]; 4] {
    let order = spots();
    let mut out = [[[0u64; 16]; CELLS]; 4];
    let mut p = 0;
    while p < 4 {
        let (hx, hy) = (2 * WINDOW + (p as i32 & 1), 2 * WINDOW + (p as i32 >> 1));
        let mut c = 0;
        while c < CELLS {
            let (wx, wy) = ((c % SPAN) as i32, (c / SPAN) as i32);
            let mut q = 0;
            while q < 4 {
                let dx = 2 * wx + (q & 1) - hx;
                let dy = 2 * wy + (q >> 1) - hy;
                let mut k = 0;
                while k < SPOTS {
                    if order[k].dx as i32 == dx && order[k].dy as i32 == dy {
                        let mut bits = 0;
                        while bits < 16 {
                            if bits >> q & 1 == 1 {
                                out[p][c][bits] |= 1 << k;
                            }
                            bits += 1;
                        }
                    }
                    k += 1;
                }
                q += 1;
            }
            c += 1;
        }
        p += 1;
    }
    out
}

static SPOT_MASKS: [[[u64; 16]; CELLS]; 4] = spot_masks();

/// The quarter a point lies in, counted over the whole plane.
fn quarter_of(p: Vec2) -> (i32, i32) {
    (p.x.div_euclid(QUARTER_MM), p.y.div_euclid(QUARTER_MM))
}

/// Which of his cell's lists a man is in.
fn list_of(p: Vec2, side: usize) -> usize {
    let (qx, qy) = quarter_of(p);
    2 * ((qy & 1) * 2 + (qx & 1)) as usize + side
}

/// Bits of a cell's flags, each shifted left by the side it speaks of.
mod flag {
    /// A man of the side is on his feet here.
    pub const UP: u8 = 1;
    /// A man of the side is giving ground or has turned here.
    pub const UNSTEADY: u8 = 1 << 2;
    /// A man of the side fell or turned here since the last man step.
    pub const SHOCK: u8 = 1 << 4;
}

/// A man's tag in the cell-ordered list: whether he is a closer on his feet and facing the
/// fight.
mod tag {
    pub const CLOSER: u8 = 1;
}

/// What one cell holds.
#[derive(Clone, Copy, Debug, Default)]
struct Agg {
    /// Men by `[side][posture]`.
    n: [[u16; 5]; 2],
    /// Closers standing steady or forward, by side.
    closers: [u16; 2],
    /// Since the last man step, by `[side]`: men who went down, men who turned.
    shocks: [[u16; 2]; 2],
    /// Which of its quarters hold men: bit `q` for side 0, bit `4 + q` for side 1.
    quarters: u8,
}

#[derive(Clone, Debug, Default)]
pub struct FieldPerception {
    tiles: Tiles,
    agg: Vec<Agg>,
    flags: Vec<u8>,
    /// Where each list of men begins: cell `a`, quarter `q`, side `s` is entry
    /// `LISTS * a + 2q + s`. One longer than `LISTS` times the cells.
    start: Vec<u32>,
    cursor: Vec<u32>,
    /// Men in the order of the lists, by id within a list: id, position, tag.
    id: Vec<u32>,
    at: Vec<Vec2>,
    tag: Vec<u8>,
    /// Each man's cell.
    cell: Vec<u32>,
    standards_down: Vec<BodyId>,
    /// Men who went down since the last man step, sorted.
    fallen: Vec<u32>,
    tick: u32,
    prev_step: u32,
}

/// How far a man at `o` within his square (0..size) is from the column of squares `d` away,
/// along one axis.
fn gap(o: i32, d: i32, size: i32) -> i64 {
    let lo = d * size - o;
    let hi = lo + size - 1;
    if lo > 0 {
        lo as i64
    } else if hi < 0 {
        -hi as i64
    } else {
        0
    }
}

impl FieldPerception {
    /// The addresses of a window's cells, row by row from its south-west corner; `NONE` where
    /// no one stands.
    fn window(&self, own: u32, p: Vec2) -> [u32; CELLS] {
        let mut w = [NONE; CELLS];
        let lx = (own as i32) & (TILE - 1);
        let ly = ((own >> TILE_SHIFT) as i32) & (TILE - 1);
        let inside = |l: i32| (WINDOW..TILE - WINDOW).contains(&l);
        if inside(lx) && inside(ly) {
            let mut a = own as i32 - WINDOW * TILE - WINDOW;
            for row in w.chunks_exact_mut(SPAN) {
                for (dx, c) in row.iter_mut().enumerate() {
                    *c = (a + dx as i32) as u32;
                }
                a += TILE;
            }
        } else {
            let (cx, cy) = cell_of(p);
            let mut k = 0;
            for dy in -WINDOW..=WINDOW {
                for dx in -WINDOW..=WINDOW {
                    w[k] = self.tiles.find(cx + dx, cy + dy);
                    k += 1;
                }
            }
        }
        w
    }

    /// The flags of every cell in a man's window, together.
    fn window_flags(&self, own: u32, p: Vec2) -> u8 {
        let lx = (own as i32) & (TILE - 1);
        let ly = ((own >> TILE_SHIFT) as i32) & (TILE - 1);
        let inside = |l: i32| (WINDOW..TILE - WINDOW).contains(&l);
        let mut f = 0;
        if inside(lx) && inside(ly) {
            let mut a = (own as i32 - WINDOW * TILE - WINDOW) as usize;
            for _ in 0..SPAN {
                for &c in &self.flags[a..a + SPAN] {
                    f |= c;
                }
                a += TILE as usize;
            }
        } else {
            for a in self.window(own, p) {
                if a != NONE {
                    f |= self.flags[a as usize];
                }
            }
        }
        f
    }

    /// The nearest enemies within reach, nearest first with ties by id, and how many men stand
    /// within crowding distance. `foes` and `friends` say which quarters about his own hold
    /// any, as bits in the order of `SPOT`.
    ///
    /// Single men are looked at only here. Quarters are searched ring by ring outward. Once
    /// the rings inside ring r are done, every enemy nearer than r - 1 quarters' width has
    /// been seen, so the search ends as soon as the eighth nearest is no farther than that.
    fn seek(
        &self,
        man: ManId,
        pos: Vec2,
        me: usize,
        w: &[u32; CELLS],
        foes: u64,
        friends: u64,
    ) -> ([(i64, u32, Vec2); MAX_NEAR], u32) {
        let mut near = [(i64::MAX, u32::MAX, Vec2::ZERO); MAX_NEAR];
        let mut crowding: u32 = 0;
        let (qx, qy) = quarter_of(pos);
        let (px, py) = (pos.x - qx * QUARTER_MM, pos.y - qy * QUARTER_MM);
        // His own quarter, counted from the south-west corner of his window.
        let (hx, hy) = (2 * WINDOW + (qx & 1), 2 * WINDOW + (qy & 1));
        let (reach2, crowd2) = (REACH_MM * REACH_MM, CROWD_MM * CROWD_MM);
        let last = MAX_NEAR - 1;
        let lists_at = |spot: Spot| {
            let (x, y) = (hx + spot.dx as i32, hy + spot.dy as i32);
            let a = w[(y >> 1) as usize * SPAN + (x >> 1) as usize];
            LISTS * a as usize + 2 * ((y & 1) * 2 + (x & 1)) as usize
        };
        let apart = |spot: Spot| {
            let gx = gap(px, spot.dx as i32, QUARTER_MM);
            let gy = gap(py, spot.dy as i32, QUARTER_MM);
            gx * gx + gy * gy
        };

        let mut m = foes;
        while m != 0 {
            let spot = SPOT[m.trailing_zeros() as usize];
            m &= m - 1;
            let covered = (spot.ring as i64 - 1) * QUARTER_MM as i64;
            if spot.ring >= 2 && near[last].0 <= covered * covered {
                break;
            }
            let d2 = apart(spot);
            // Past the cap no more crowding is counted.
            let crowd_here = d2 <= crowd2 && crowding < CROWD_CAP as u32;
            if d2 > reach2.min(near[last].0) && !crowd_here {
                continue;
            }
            let foes = lists_at(spot) + 1 - me;
            for s in self.start[foes] as usize..self.start[foes + 1] as usize {
                let offset = self.at[s] - pos;
                let d2 = offset.len_sq();
                crowding += (d2 <= crowd2) as u32;
                // Keep the eight nearest, nearest first, ties by id.
                if d2 > reach2.min(near[last].0) {
                    continue;
                }
                let j = self.id[s];
                let mut slot = MAX_NEAR;
                while slot > 0 && (d2, j) < (near[slot - 1].0, near[slot - 1].1) {
                    slot -= 1;
                }
                if slot < MAX_NEAR {
                    near.copy_within(slot..last, slot + 1);
                    near[slot] = (d2, j, offset);
                }
            }
        }

        let mut m = friends & CROWD_SPOTS;
        while m != 0 && crowding < CROWD_CAP as u32 {
            let spot = SPOT[m.trailing_zeros() as usize];
            m &= m - 1;
            if apart(spot) > crowd2 {
                continue;
            }
            let friends = lists_at(spot) + me;
            for s in self.start[friends] as usize..self.start[friends + 1] as usize {
                let near_enough = (self.at[s] - pos).len_sq() <= crowd2;
                crowding += (near_enough & (self.id[s] != man.0)) as u32;
            }
        }
        (near, crowding)
    }

    /// Whether an enemy at `enemy`, in the reach of the man `me` at `pos`, would strike another
    /// man of `me`'s side first: one in his window, on his feet, whom [`strikes_first`] puts
    /// before him. Such a man is no farther from the enemy than `me` is, so only the quarters
    /// that near to the enemy are looked at, and the first one found ends the search.
    #[allow(clippy::too_many_arguments)]
    fn another_first(
        &self,
        bodies: &Bodies,
        w: &[u32; CELLS],
        (me, pos, side): (ManId, Vec2, usize),
        enemy: Vec2,
        facing: Turn,
        d2: i64,
    ) -> bool {
        let (qx, qy) = quarter_of(pos);
        // His window's south-west quarter, and the quarters of it the enemy's circle covers.
        let (wx, wy) = (qx - (qx & 1) - 2 * WINDOW, qy - (qy & 1) - 2 * WINDOW);
        let r = (d2 as u64).isqrt() as i32 + 1;
        let side_len = 2 * SPAN as i32;
        let span = |at: i32, w0: i32| {
            let lo = (at - r).div_euclid(QUARTER_MM) - w0;
            let hi = (at + r).div_euclid(QUARTER_MM) - w0;
            lo.max(0)..=hi.min(side_len - 1)
        };
        for y in span(enemy.y, wy) {
            for x in span(enemy.x, wx) {
                let a = w[(y >> 1) as usize * SPAN + (x >> 1) as usize];
                if a == NONE {
                    continue;
                }
                let friends = LISTS * a as usize + 2 * ((y & 1) * 2 + (x & 1)) as usize + side;
                for s in self.start[friends] as usize..self.start[friends + 1] as usize {
                    let friend = ManId(self.id[s]);
                    if friend != me
                        && bodies.posture[friend.0 as usize] != Posture::Down
                        && strikes_first(enemy, facing, (self.at[s], friend), (pos, me))
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn body_shocks(&self, body: Option<BodyId>, state: Option<BodyState>) -> (bool, bool) {
        let standard = body.is_some_and(|b| self.standards_down.contains(&b));
        let leader = !self.fallen.is_empty()
            && state
                .and_then(|s| s.leader)
                .is_some_and(|l| self.fallen.binary_search(&l.0).is_ok());
        (standard, leader)
    }
}

impl Perceive for FieldPerception {
    fn rebuild(&mut self, bodies: &Bodies, recent: &[Event], tick: u32, prev_step: u32) {
        self.tick = tick;
        self.prev_step = prev_step;
        let n = bodies.len();
        self.tiles.clear();
        self.cell.clear();
        for p in &bodies.pos {
            let (cx, cy) = cell_of(*p);
            self.cell.push(self.tiles.touch(cx, cy));
        }
        // A shock is seen where it happened, even if no one stands there now.
        self.standards_down.clear();
        self.fallen.clear();
        let mut shocks = Vec::new();
        for e in recent {
            match e.kind {
                EventKind::Posture { to, .. } if matches!(to, Posture::Down | Posture::Turned) => {
                    let (cx, cy) = cell_of(e.pos);
                    let side = bodies.side[e.man.0 as usize] as usize;
                    let down = to == Posture::Down;
                    shocks.push((self.tiles.touch(cx, cy), side, down));
                    if down {
                        self.fallen.push(e.man.0);
                    }
                }
                EventKind::StandardDown { body } => self.standards_down.push(body),
                _ => {}
            }
        }
        self.fallen.sort_unstable();

        let cells = self.tiles.cells();
        self.agg.clear();
        self.agg.resize(cells, Agg::default());
        self.flags.clear();
        self.flags.resize(cells, 0);
        self.start.clear();
        self.start.resize(LISTS * cells + 1, 0);
        for i in 0..n {
            let a = self.cell[i] as usize;
            let (s, q) = (bodies.side[i] as usize, bodies.posture[i]);
            let g = &mut self.agg[a];
            g.n[s][q as usize] = g.n[s][q as usize].saturating_add(1);
            let mut f = 0;
            if q != Posture::Down {
                f |= flag::UP << s;
            }
            if matches!(q, Posture::Back | Posture::Turned) {
                f |= flag::UNSTEADY << s;
            }
            if bodies.role[i] == Role::Closer && matches!(q, Posture::Steady | Posture::Forward) {
                g.closers[s] = g.closers[s].saturating_add(1);
            }
            self.flags[a] |= f;
            let list = list_of(bodies.pos[i], s);
            g.quarters |= 1 << (list / 2 + 4 * s);
            self.start[LISTS * a + list + 1] += 1;
        }
        for (a, side, down) in shocks {
            let c = &mut self.agg[a as usize].shocks[side][!down as usize];
            *c = c.saturating_add(1);
            self.flags[a as usize] |= flag::SHOCK << side;
        }
        for a in 0..LISTS * cells {
            self.start[a + 1] += self.start[a];
        }
        self.cursor.clear();
        self.cursor.extend_from_slice(&self.start[..LISTS * cells]);
        self.id.resize(n, 0);
        self.at.resize(n, Vec2::ZERO);
        self.tag.resize(n, 0);
        for i in 0..n {
            let list = list_of(bodies.pos[i], bodies.side[i] as usize);
            let c = &mut self.cursor[LISTS * self.cell[i] as usize + list];
            let k = *c as usize;
            *c += 1;
            let q = bodies.posture[i];
            let closer =
                bodies.role[i] == Role::Closer && matches!(q, Posture::Steady | Posture::Forward);
            self.id[k] = i as u32;
            self.at[k] = bodies.pos[i];
            self.tag[k] = closer as u8 * tag::CLOSER;
        }
    }

    fn coarse<F: FrameQuery>(&self, bodies: &Bodies, frames: &F, man: ManId) -> Coarse {
        let i = man.0 as usize;
        let side = bodies.side[i] as u8;
        let f = self.window_flags(self.cell[i], bodies.pos[i]);
        let body = bodies.body[i].and_then(|b| frames.body(b));
        let (standard_fell, leader_fell) = self.body_shocks(bodies.body[i], body);
        Coarse {
            enemy_near: f & (flag::UP << (1 - side)) != 0,
            shock_near: f & (flag::SHOCK << side) != 0 || standard_fell || leader_fell,
            out_of_place: bodies.body[i].is_some() && !bodies.in_place[i],
            order_new: body.is_some_and(|b| b.order_tick >= self.prev_step && self.tick > 0),
            own_unsteady: f & (flag::UNSTEADY << side) != 0,
            order: body.map_or(Order::Hold, |b| b.order),
        }
    }

    fn view<F: FrameQuery>(&self, bodies: &Bodies, frames: &F, man: ManId) -> View {
        let i = man.0 as usize;
        let own = bodies.get(man);
        let me = own.side as usize;
        let (ax, ay) = axis(own.facing);
        let ahead = own.facing.unit(1 << 10);
        let (qx, qy) = quarter_of(own.pos);
        let masks = &SPOT_MASKS[((qy & 1) * 2 + (qx & 1)) as usize];

        let mut counts = Counts::default();
        let mut closer_behind = false;
        let (mut falls, mut turns) = (0u32, 0u32);
        // The quarters about his own that hold enemies, and friends.
        let (mut foes, mut friends) = (0u64, 0u64);

        let w = self.window(self.cell[i], own.pos);
        let mut k = 0;
        for dy in -WINDOW..=WINDOW {
            for dx in -WINDOW..=WINDOW {
                let a = w[k];
                k += 1;
                if a == NONE {
                    continue;
                }
                let g = &self.agg[a as usize];
                let held = g.quarters as usize;
                foes |= masks[k - 1][(held >> (4 * (1 - me))) & 15];
                friends |= masks[k - 1][(held >> (4 * me)) & 15];
                const BY_SIGN: [Group; 3] = [Group::Behind, Group::Beside, Group::Ahead];
                let group = BY_SIGN[((dx * ax + dy * ay).signum() + 1) as usize];
                let (own_side, enemy_side) = counts.0[group as usize].split_at_mut(1);
                for (c, &m) in own_side[0].iter_mut().zip(&g.n[me]) {
                    *c = c.saturating_add(m);
                }
                for (c, &m) in enemy_side[0].iter_mut().zip(&g.n[1 - me]) {
                    *c = c.saturating_add(m);
                }
                falls += g.shocks[me][0] as u32;
                turns += g.shocks[me][1] as u32;

                // One closer behind him is as good as two.
                if g.closers[me] > 0 && !closer_behind {
                    for q in 0..4 {
                        let friends = LISTS * a as usize + 2 * q + me;
                        for s in self.start[friends] as usize..self.start[friends + 1] as usize {
                            if self.tag[s] & tag::CLOSER != 0
                                && self.id[s] != man.0
                                && (self.at[s] - own.pos).dot(ahead) < 0
                            {
                                closer_behind = true;
                            }
                        }
                    }
                }
            }
        }

        let (near, crowding) = self.seek(man, own.pos, me, &w, foes, friends);
        let near = near.map(|(d2, j, offset)| {
            (d2 != i64::MAX).then(|| {
                let ju = j as usize;
                let to_me = Turn::of(-offset);
                NearEnemy {
                    id: ManId(j),
                    offset,
                    posture: bodies.posture[ju],
                    sector: Turn::of(offset).map_or(Sector::Front, |d| Sector::of(own.facing, d)),
                    faces_me: to_me
                        .is_none_or(|d| Sector::of(bodies.facing[ju], d) == Sector::Front),
                    strikes_another: self.another_first(
                        bodies,
                        &w,
                        (man, own.pos, me),
                        own.pos + offset,
                        bodies.facing[ju],
                        d2,
                    ),
                }
            })
        });
        // He does not count himself.
        let mine = &mut counts.0[Group::Beside as usize][Rel::Own as usize][own.posture as usize];
        *mine = mine.saturating_sub(1);

        let body = bodies.body[i].and_then(|b| frames.body(b));
        let (standard_fell, leader_fell) = self.body_shocks(bodies.body[i], body);
        let dist = |q: Vec2| (q - own.pos).length().min(i32::MAX as i64) as i32;
        View {
            man,
            tick: self.tick,
            own,
            counts,
            near,
            crowding: crowding.min(CROWD_CAP as u32) as u8,
            shocks: Shocks {
                falls: falls.min(u8::MAX as u32) as u8,
                turns: turns.min(u8::MAX as u32) as u8,
                standard_fell,
                leader_fell,
            },
            slot: frames.slot(man),
            standard: body.map(|b| match b.standard {
                Standard::Carried(c) => StandardSeen::Up {
                    dist_mm: dist(bodies.pos[c.0 as usize]),
                    posture: bodies.posture[c.0 as usize],
                },
                Standard::Down(p) => StandardSeen::Down { dist_mm: dist(p) },
            }),
            leader: body.and_then(|b| b.leader).map(|l| LeaderSeen {
                posture: bodies.posture[l.0 as usize],
                dist_mm: dist(bodies.pos[l.0 as usize]),
            }),
            closer_behind,
            order: body.map_or(Order::Hold, |b| b.order),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tiles::{CELL_BITS, TILE_CELLS};

    #[test]
    fn the_gap_to_a_column_is_exact() {
        // A man 300 mm into his cell.
        assert_eq!(gap(300, 0, CELL_MM), 0);
        assert_eq!(gap(300, 1, CELL_MM), 1_700);
        assert_eq!(gap(300, 2, CELL_MM), 3_700);
        assert_eq!(gap(300, -1, CELL_MM), 301);
        assert_eq!(gap(300, -2, CELL_MM), 2_301);
    }

    #[test]
    fn a_window_inside_a_tile_needs_no_search() {
        let mut p = FieldPerception::default();
        let mut b = Bodies::default();
        let spot = Vec2::new(10 * CELL_MM + 5, 10 * CELL_MM + 5);
        b.push(cannae_core::Observed {
            pos: spot,
            ..Default::default()
        });
        p.rebuild(&b, &[], 5, 0);
        let w = p.window(p.cell[0], spot);
        assert!(w.iter().all(|&a| (a as usize) < TILE_CELLS));
        assert_eq!(w[CELLS / 2], p.cell[0]);
        assert_eq!(w[0], p.tiles.find(8, 8));
        assert_eq!(w[CELLS - 1], p.tiles.find(12, 12));
        assert_eq!(w[0] >> CELL_BITS, 0);
    }
}
