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
};
use cannae_core::{
    Bodies, BodyId, Coarse, Event, EventKind, ManId, Perceive, Posture, Role, Turn, Vec2, View,
};

const SPAN: usize = (2 * WINDOW + 1) as usize;
const CELLS: usize = SPAN * SPAN;

/// Bits of a cell's flags, each shifted left by the side it speaks of.
mod flag {
    /// A man of the side is on his feet here.
    pub const UP: u8 = 1;
    /// A man of the side is giving ground or has turned here.
    pub const UNSTEADY: u8 = 1 << 2;
    /// A man of the side fell or turned here since the last man step.
    pub const SHOCK: u8 = 1 << 4;
}

/// A man's tag in the cell-ordered list: his side, and whether he is a closer on his feet and
/// facing the fight.
mod tag {
    pub const SIDE: u8 = 1;
    pub const CLOSER: u8 = 2;
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
}

#[derive(Clone, Debug, Default)]
pub struct FieldPerception {
    tiles: Tiles,
    agg: Vec<Agg>,
    flags: Vec<u8>,
    /// Where each cell's men begin in the cell-ordered lists; one longer than the cells.
    start: Vec<u32>,
    cursor: Vec<u32>,
    /// Men in cell order, by id within a cell: id, position, tag.
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

/// How far a man at `o` within his cell (0..CELL_MM) is from column `d` of his window, along
/// one axis.
fn gap(o: i32, d: i32) -> i64 {
    let lo = d * CELL_MM - o;
    let hi = lo + CELL_MM - 1;
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
        self.start.resize(cells + 1, 0);
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
            self.start[a + 1] += 1;
        }
        for (a, side, down) in shocks {
            let c = &mut self.agg[a as usize].shocks[side][!down as usize];
            *c = c.saturating_add(1);
            self.flags[a as usize] |= flag::SHOCK << side;
        }
        for a in 0..cells {
            self.start[a + 1] += self.start[a];
        }
        self.cursor.clear();
        self.cursor.extend_from_slice(&self.start[..cells]);
        self.id.resize(n, 0);
        self.at.resize(n, Vec2::ZERO);
        self.tag.resize(n, 0);
        for i in 0..n {
            let c = &mut self.cursor[self.cell[i] as usize];
            let k = *c as usize;
            *c += 1;
            let q = bodies.posture[i];
            let closer =
                bodies.role[i] == Role::Closer && matches!(q, Posture::Steady | Posture::Forward);
            self.id[k] = i as u32;
            self.at[k] = bodies.pos[i];
            self.tag[k] = bodies.side[i] as u8 * tag::SIDE + closer as u8 * tag::CLOSER;
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
        let (cx, cy) = cell_of(own.pos);
        let (ox, oy) = (own.pos.x - cx * CELL_MM, own.pos.y - cy * CELL_MM);

        let mut counts = Counts::default();
        let mut near = [(i64::MAX, u32::MAX, None::<NearEnemy>); MAX_NEAR];
        let mut crowding: u32 = 0;
        let mut closer_behind = false;
        let (mut falls, mut turns) = (0u32, 0u32);

        let w = self.window(self.cell[i], own.pos);
        let mut k = 0;
        for dy in -WINDOW..=WINDOW {
            let gy = gap(oy, dy);
            for dx in -WINDOW..=WINDOW {
                let a = w[k];
                k += 1;
                if a == NONE {
                    continue;
                }
                let g = &self.agg[a as usize];
                let group = match (dx * ax + dy * ay).signum() {
                    1 => Group::Ahead,
                    0 => Group::Beside,
                    _ => Group::Behind,
                };
                let (own_side, enemy_side) = counts.0[group as usize].split_at_mut(1);
                for (c, &m) in own_side[0].iter_mut().zip(&g.n[me]) {
                    *c = c.saturating_add(m);
                }
                for (c, &m) in enemy_side[0].iter_mut().zip(&g.n[1 - me]) {
                    *c = c.saturating_add(m);
                }
                falls += g.shocks[me][0] as u32;
                turns += g.shocks[me][1] as u32;

                // Single men matter only for what is exact: reach, crowding, a closer.
                let gx = gap(ox, dx);
                let d2 = gx * gx + gy * gy;
                let enemies = g.n[1 - me].iter().any(|&c| c > 0);
                if !(d2 <= CROWD_MM * CROWD_MM
                    || (enemies && d2 <= REACH_MM * REACH_MM)
                    || g.closers[me] > 0)
                {
                    continue;
                }
                let run = self.start[a as usize] as usize..self.start[a as usize + 1] as usize;
                for s in run {
                    let j = self.id[s];
                    if j == man.0 {
                        continue;
                    }
                    let offset = self.at[s] - own.pos;
                    let t = self.tag[s];
                    let friend = (t & tag::SIDE) as usize == me;
                    if friend && t & tag::CLOSER != 0 && offset.dot(ahead) < 0 {
                        closer_behind = true;
                    }
                    let d2 = offset.len_sq();
                    if d2 <= CROWD_MM * CROWD_MM {
                        crowding += 1;
                    }
                    if !friend && d2 <= REACH_MM * REACH_MM {
                        // Keep the eight nearest, nearest first, ties by id.
                        let mut at = MAX_NEAR;
                        while at > 0 && (d2, j) < (near[at - 1].0, near[at - 1].1) {
                            at -= 1;
                        }
                        if at < MAX_NEAR {
                            near.copy_within(at..MAX_NEAR - 1, at + 1);
                            let ju = j as usize;
                            let to_me = Turn::of(-offset);
                            near[at] = (
                                d2,
                                j,
                                Some(NearEnemy {
                                    id: ManId(j),
                                    offset,
                                    posture: bodies.posture[ju],
                                    sector: Turn::of(offset)
                                        .map_or(Sector::Front, |d| Sector::of(own.facing, d)),
                                    faces_me: to_me.is_none_or(|d| {
                                        Sector::of(bodies.facing[ju], d) == Sector::Front
                                    }),
                                }),
                            );
                        }
                    }
                }
            }
        }
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
            near: near.map(|e| e.2),
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
        assert_eq!(gap(300, 0), 0);
        assert_eq!(gap(300, 1), 1_700);
        assert_eq!(gap(300, 2), 3_700);
        assert_eq!(gap(300, -1), 301);
        assert_eq!(gap(300, -2), 2_301);
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
