//! The field substrate: one step every man step.
//!
//! Each step has four parts.
//!
//! 1. **Deposit.** Every man on his feet spreads his presence over the four cells whose centres
//!    surround him, by side.
//! 2. **Steps.** A man in his place steps straight toward his target. A man out of his place
//!    reads the field: the density a cell ahead sets how fast he can walk, and the densities
//!    around him push him from packed ground toward open ground.
//! 3. **Exact positions.** Any free man with an enemy near is handled by positions: no two
//!    enemies on their feet may stand closer than a guard's distance. Where a crowd is packed
//!    and moving, its own men are kept a body apart in the same way.
//! 4. **Move,** turn, and draw for crushing.
//!
//! Density is bounded by construction, not by a stiff pressure: a walking man cannot step into
//! ground at the jam density, and a pressing man cannot step into ground at the packing
//! density. The pressure only spreads a crowd that is no longer pressed.
//!
//! Two crowds do not pass through each other for the same reason: enemies ahead count toward
//! the jam at a weight that makes an open line of them a wall, so no one walks onto ground the
//! enemy holds. He gets it only when they are pushed off it or leave it.
//!
//! Walls of the ground stop every man, in his place or out of it, and cut the grid where they
//! run: see [`crate::walls`].

use crate::curve::{STEPS, walk_q8};
use crate::tiles::{CELL_BITS, NONE, TILE, TILE_SHIFT, Tiles, local};
use crate::walls::{Walls, into, stand_in};
use cannae_core::schedule::{MAN_EVERY, is_man_step};
use cannae_core::{
    Bodies, Cause, Event, EventKind, Key, ManId, Posture, Prob, Rng, SimMs, Stream, Substrate,
    Target, Vec2, Wall, Wound,
};

/// Which men are kept apart by their exact positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Contact {
    /// Enemies only: the contact line.
    Line,
    /// Enemies, and men of one side where the crowd is packed and moving.
    Crowd,
    /// Every man out of his place, with every man near him.
    All,
}

/// Densities are men per square metre in Q8. Every value has a prior in
/// docs/slice0/priors-field.md.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Params {
    /// The cell is `1 << cell_shift` mm wide.
    pub cell_shift: u32,
    /// The density at which walking stops.
    pub jam_q8: u32,
    /// The density at which even a man who presses gains no ground. It also sets how near two
    /// bodies can be: discs at this density, packed as tightly as discs go.
    pub pack_q8: u32,
    /// The density of enemies on their feet at which ground is held: no one walks or leans
    /// onto it. Thinner enemy presence counts toward the jam in proportion.
    pub hold_q8: u32,
    /// Below this density of the men around him, a crowd does not push a man.
    pub ease_q8: u32,
    /// How fast a packed crowd spreads: mm/s for each man per m2 of difference across a metre.
    pub spread_mm_s: i32,
    /// The most that spreading moves a man, mm/s.
    pub spread_max_mm_s: i32,
    /// How fast a man at full press gains ground while there is ground to gain, mm/s.
    pub press_mm_s: i32,
    /// The nearest two enemies on their feet can stand, centre to centre.
    pub guard_mm: i32,
    pub contact: Contact,
    /// Under `Contact::Crowd`: the density of the men around him above which a moving man
    /// touches them.
    pub crowd_q8: u32,
    /// Under `Contact::Crowd`: a man whose own going, his walk and his lean, is slower than
    /// this is not moving. What the crowd's push does to him does not count.
    pub moving_mm_s: i32,
    /// The density of the men around him above which a man is being crushed.
    pub crush_q8: u32,
    /// Hazard of being disabled, per second, for each man per m2 above `crush_q8`; Q16.
    pub crush_rate_q16: u32,
    /// The most a man turns in one base tick, in turn units.
    pub turn_per_tick: u16,
}

impl Params {
    /// The centre of every prior.
    pub const CENTRE: Params = Params {
        cell_shift: 10,
        jam_q8: 1_382,
        pack_q8: 2_304,
        hold_q8: 205,
        ease_q8: 640,
        spread_mm_s: 100,
        spread_max_mm_s: 300,
        press_mm_s: 450,
        guard_mm: 600,
        contact: Contact::Line,
        crowd_q8: 768,
        moving_mm_s: 60,
        crush_q8: 1_664,
        // A mean of 134 s to disablement at 2 men per m2 above the onset.
        crush_rate_q16: 244,
        turn_per_tick: 1_638,
    };
}

/// The purpose word for a crush draw on the substrate stream.
pub const PURPOSE_CRUSH: u32 = 1;

/// What the last step did, for the spike and for tests. Not state: nothing reads it back.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Stats {
    /// Men out of their places.
    pub free: u32,
    /// Men handled by exact positions.
    pub hot: u32,
    /// Passes of the position solve.
    pub passes: u32,
    pub tiles: u32,
}

// A man's kind for one step.
const FREE: u8 = 1;
const HOT: u8 = 2;
/// Packed and moving: he touches the men of his own side too.
const TOUCHES: u8 = 4;
/// His four cells lie in one tile.
const WHOLE: u8 = 8;

const EDGE: i32 = TILE - 1;
const UNIT_Q: i64 = 4_096;
/// Passes of the position solve after the step is fully taken.
const SETTLE: i64 = 2;
/// The most times one man is moved clear of his neighbours in one pass. A man wedged between
/// two needs several; most need one.
const SWEEPS: u32 = 8;
/// Densities above the crush onset, in grid units, whose chance is kept in a table.
const CRUSH_TABLE: usize = 4_096;

/// One of the men a hot man may touch this step.
#[derive(Clone, Copy, Debug)]
struct Near {
    man: u32,
    /// The distance to keep, centre to centre.
    gap: i32,
    /// His share of any overlap between the two, of 256. The man who is walking into the other
    /// gives way; a man who stands is not shoved by one who comes at him; two who come at each
    /// other give in proportion. All of it is his when the other is in his place.
    mine_q8: i32,
}

/// The four cells whose centres surround a point: south-west, south-east, north-west,
/// north-east.
type Block = [u32; 4];

fn whole(b: u32) -> Block {
    [b, b + 1, b + TILE as u32, b + TILE as u32 + 1]
}

/// The share of a man that each cell of his block holds, of 256. They sum to 256 exactly.
fn weights(fx: i32, fy: i32) -> [i32; 4] {
    let ne = (fx * fy) >> 8;
    [256 - fx - fy + ne, fx - ne, fy - ne, ne]
}

fn find_block(tiles: &Tiles, bx: i32, by: i32, hint: (i32, i32, u32)) -> Block {
    let (lx, ly) = (bx & EDGE, by & EDGE);
    if lx < EDGE && ly < EDGE {
        let (tx, ty) = (bx >> TILE_SHIFT, by >> TILE_SHIFT);
        let t = if (tx, ty) == (hint.0, hint.1) {
            hint.2
        } else {
            match tiles.find_tile(tx, ty) {
                Some(t) => t,
                None => return [NONE; 4],
            }
        };
        whole((t << CELL_BITS) | local(bx, by))
    } else {
        [
            tiles.find(bx, by),
            tiles.find(bx + 1, by),
            tiles.find(bx, by + 1),
            tiles.find(bx + 1, by + 1),
        ]
    }
}

fn read(den: &[[u16; 2]], blk: &Block) -> [[u16; 2]; 4] {
    blk.map(|a| if a == NONE { [0, 0] } else { den[a as usize] })
}

/// The parameters in the grid's own units: a man is 256, a density is per cell.
#[derive(Clone, Copy, Debug)]
struct Grid {
    shift: u32,
    cell: i32,
    jam: i32,
    pack: i32,
    hold: i32,
    /// What one enemy ahead counts for against walking, in men of his own side; Q8.
    foe_q8: i64,
    ease: i32,
    crowd: i32,
    crush: i32,
    /// Steps of the walking curve per unit of density, Q16.
    walk_q16: i64,
    /// mm/s of spreading per unit of density difference across a cell, Q16.
    spread_q16: i64,
    /// The nearest two men of one side can stand in a packed, moving crowd.
    body_mm: i32,
}

impl Grid {
    fn of(p: &Params) -> Grid {
        assert!(
            (8..=12).contains(&p.cell_shift),
            "a cell of 256 mm to 4 m is supported"
        );
        assert!(
            p.pack_q8 > p.jam_q8 && p.jam_q8 > 0,
            "packing must be denser than the jam"
        );
        assert!(p.hold_q8 > 0, "some density of enemies must hold ground");
        let cell = 1i64 << p.cell_shift;
        let units = |q8: u32| (q8 as i64 * cell * cell / 1_000_000) as i32;
        let jam = units(p.jam_q8).max(1);
        // Discs at hexagonal packing: one man to 2 sqrt(3) r^2.
        let r2 = 1_000_000i64 * 256 * 10_000 / (34_641 * p.pack_q8 as i64);
        Grid {
            shift: p.cell_shift,
            cell: cell as i32,
            jam,
            pack: units(p.pack_q8).max(jam + 1),
            hold: units(p.hold_q8).max(1),
            foe_q8: p.jam_q8 as i64 * 256 / p.hold_q8 as i64,
            ease: units(p.ease_q8),
            crowd: units(p.crowd_q8),
            crush: units(p.crush_q8),
            walk_q16: ((STEPS as i64) << 16) / jam as i64,
            spread_q16: p.spread_mm_s as i64 * 1_000_000_000 * 256 / (cell * cell * cell),
            body_mm: 2 * (r2 as u64).isqrt() as i32,
        }
    }

    /// The south-west cell of the block around a point, and how far the point lies toward the
    /// north-east cell's centre, of 256.
    fn split(&self, p: Vec2) -> (i32, i32, i32, i32) {
        let half = self.cell >> 1;
        let (x, y) = (p.x.wrapping_sub(half), p.y.wrapping_sub(half));
        let frac = |v: i32| (v & (self.cell - 1)) >> (self.shift - 8);
        (x >> self.shift, y >> self.shift, frac(x), frac(y))
    }

    /// The centre of cell `k` of the block whose south-west cell is (`bx`, `by`).
    fn centre(&self, bx: i32, by: i32, k: usize) -> Vec2 {
        let half = self.cell >> 1;
        Vec2::new(
            ((bx + (k & 1) as i32) << self.shift) + half,
            ((by + (k >> 1) as i32) << self.shift) + half,
        )
    }

    fn q8(&self, units: i64) -> i64 {
        units * 1_000_000 / (self.cell as i64 * self.cell as i64)
    }
}

pub struct Field {
    p: Params,
    g: Grid,
    /// The ground's walls. They never move.
    walls: Walls,
    tiles: Tiles,
    /// Presence by cell and side, 256 to a man.
    den: Vec<[u16; 2]>,
    /// By man: the south-west cell of his block, and his own cell. `NONE` for the fallen.
    base: Vec<u32>,
    cell: Vec<u32>,
    kind: Vec<u8>,
    /// By man: where the field would take him this step.
    step: Vec<Vec2>,
    /// By man: what exact positions add to that.
    corr: Vec<Vec2>,
    /// By man, where there are walls: for a man out of his place who stands against one, the
    /// way out from it, 1024 long; zero for the rest.
    held: Vec<Vec2>,
    hot: Vec<u32>,
    /// Whether any man touches the men of his own side this step.
    touching: bool,
    /// Men being crushed, with the density above the onset.
    crushing: Vec<(u32, i32)>,
    /// The chance of being disabled in one step, by density above the onset in grid units.
    /// Filled as densities are met; it depends on the parameters alone.
    crush_chance: Vec<Prob>,
    /// The step the table was filled for, ms.
    crush_dt: i64,
    // The men of each field cell and side, for the position solve: cell `a`, side `s` is
    // entry `2a + s`.
    cstart: Vec<u32>,
    ccursor: Vec<u32>,
    corder: Vec<u32>,
    /// For each hot man, the men he may touch this step.
    lists: Vec<Near>,
    ranges: Vec<u32>,
    delta: Vec<Vec2>,
    stats: Stats,
}

impl Field {
    pub fn new(p: Params) -> Self {
        Field::with_walls(p, &[])
    }

    /// A field on ground with walls. No one crosses them, and no man's centre comes nearer to
    /// one than half a body's width.
    pub fn with_walls(p: Params, walls: &[Wall]) -> Self {
        let g = Grid::of(&p);
        Field {
            p,
            g,
            walls: Walls::new(walls, g.cell, g.body_mm / 2),
            tiles: Tiles::default(),
            den: Vec::new(),
            base: Vec::new(),
            cell: Vec::new(),
            kind: Vec::new(),
            step: Vec::new(),
            corr: Vec::new(),
            held: Vec::new(),
            hot: Vec::new(),
            touching: false,
            crushing: Vec::new(),
            crush_chance: Vec::new(),
            crush_dt: 0,
            cstart: Vec::new(),
            ccursor: Vec::new(),
            corder: Vec::new(),
            lists: Vec::new(),
            ranges: Vec::new(),
            delta: Vec::new(),
            stats: Stats::default(),
        }
    }

    pub fn params(&self) -> &Params {
        &self.p
    }

    pub fn stats(&self) -> Stats {
        self.stats
    }

    /// The nearest two men of one side can stand where a crowd is packed and moving, mm.
    pub fn body_mm(&self) -> i32 {
        self.g.body_mm
    }

    /// The densest cell at the last step, men per m2 in Q8. Private to this substrate: men read
    /// crowding from the View.
    pub fn peak_q8(&self) -> u32 {
        let most = self
            .den
            .iter()
            .map(|c| c[0] as i64 + c[1] as i64)
            .max()
            .unwrap_or(0);
        self.g.q8(most) as u32
    }

    fn deposit(&mut self, bodies: &Bodies) {
        let n = bodies.len();
        self.tiles.clear();
        self.den.clear();
        self.base.clear();
        self.base.resize(n, NONE);
        self.cell.clear();
        self.cell.resize(n, NONE);
        self.kind.clear();
        self.kind.resize(n, 0);
        let walled = !self.walls.is_empty();
        for i in 0..n {
            if bodies.posture[i] == Posture::Down {
                continue;
            }
            let (bx, by, fx, fy) = self.g.split(bodies.pos[i]);
            let one_tile = (bx & EDGE) < EDGE && (by & EDGE) < EDGE;
            let blk = if one_tile {
                whole(self.tiles.touch(bx, by))
            } else {
                [
                    self.tiles.touch(bx, by),
                    self.tiles.touch(bx + 1, by),
                    self.tiles.touch(bx, by + 1),
                    self.tiles.touch(bx + 1, by + 1),
                ]
            };
            if self.den.len() < self.tiles.cells() {
                self.den.resize(self.tiles.cells(), [0; 2]);
            }
            let mut w = weights(fx, fy);
            if walled && self.walls.near(bodies.pos[i]) {
                let g = &self.g;
                w = self.walls.cut(bodies.pos[i], |k| g.centre(bx, by, k), w).0;
            }
            let side = bodies.side[i] as usize;
            for k in 0..4 {
                let c = &mut self.den[blk[k] as usize][side];
                *c = c.saturating_add(w[k] as u16);
            }
            self.base[i] = blk[0];
            self.cell[i] = blk[(fx >= 128) as usize + 2 * (fy >= 128) as usize];
            let free = if bodies.in_place[i] { 0 } else { FREE };
            self.kind[i] = free | (one_tile as u8 * WHOLE);
        }
    }

    /// Where the field would take each man. Returns the square of the longest step.
    fn steps(&mut self, bodies: &Bodies, targets: &[Target], dt: i64) -> i64 {
        let Field {
            p,
            g,
            walls,
            tiles,
            den,
            base,
            kind,
            step,
            held,
            hot,
            crushing,
            touching,
            stats,
            ..
        } = self;
        let walled = !walls.is_empty();
        *touching = false;
        let n = bodies.len();
        step.clear();
        step.resize(n, Vec2::ZERO);
        held.clear();
        if walled {
            held.resize(n, Vec2::ZERO);
        }
        hot.clear();
        crushing.clear();
        let moving = p.moving_mm_s as i64 * dt / 1000;
        let moving2 = moving * moving;
        let mut far = 0i64;
        let mut free = 0u32;
        for i in 0..n {
            if base[i] == NONE {
                continue;
            }
            let pos = bodies.pos[i];
            let tg = &targets[i];
            let to = tg.point - pos;
            if kind[i] & FREE == 0 {
                // In his place: the tie alone moves him.
                let reach = tg.speed as i64 * dt / 1000;
                let s = if to.len_sq() <= reach * reach {
                    to
                } else {
                    to.with_len(reach as i32)
                };
                step[i] = s;
                far = far.max(s.len_sq());
                continue;
            }
            free += 1;
            let side = bodies.side[i] as usize;
            let (bx, by, fx, fy) = g.split(pos);
            let mut w = weights(fx, fy);
            let own = if kind[i] & WHOLE != 0 {
                whole(base[i])
            } else {
                find_block(tiles, bx, by, (i32::MAX, i32::MAX, NONE))
            };
            let mut d = read(den, &own);
            // Cells a wall hides from him: he reads nothing from them.
            let mut seen = [true; 4];
            if walled && walls.near(pos) {
                held[i] = walls.held(pos);
                (w, seen) = walls.cut(pos, |k| g.centre(bx, by, k), w);
                for k in 0..4 {
                    if !seen[k] {
                        d[k] = [0, 0];
                    }
                }
            }
            // The men around him: the field less his own share of it.
            let mut foe = 0u16;
            let mut t = [0i32; 4];
            for k in 0..4 {
                foe |= d[k][1 - side];
                t[k] = d[k][0] as i32 + d[k][1] as i32 - w[k];
            }
            if seen != [true; 4] {
                let around = t;
                for k in 0..4 {
                    if !seen[k] {
                        t[k] = around[stand_in(&seen, k)];
                    }
                }
            }
            let here = (t[0] * w[0] + t[1] * w[1] + t[2] * w[2] + t[3] * w[3]) / 256;

            let mut s = Vec2::ZERO;
            let e = t.map(|v| (v - g.ease).max(0));
            let gx = ((e[1] - e[0]) * (256 - fy) + (e[3] - e[2]) * fy) / 256;
            let gy = ((e[2] - e[0]) * (256 - fx) + (e[3] - e[1]) * fx) / 256;
            if gx != 0 || gy != 0 {
                let push = Vec2::new(
                    (-(gx as i64) * g.spread_q16 / 65_536) as i32,
                    (-(gy as i64) * g.spread_q16 / 65_536) as i32,
                )
                .clamp_len(p.spread_max_mm_s);
                s = push.mul_div(dt, 1000);
            }

            // What follows is his own going; the crowd's push above is not.
            let pushed = s;
            let lean = if tg.press > 0 { tg.face } else { None };
            let dist = if tg.speed > 0 { to.length() } else { 0 };
            let dir = if dist > 0 {
                to.mul_div(UNIT_Q, dist)
            } else {
                lean.map_or(Vec2::ZERO, |f| f.unit(UNIT_Q as i32))
            };
            if dir != Vec2::ZERO {
                // What lies ahead decides how fast he may go: a cell ahead, or where he is
                // going if that is nearer.
                let look = if dist > 0 {
                    dist.min(g.cell as i64)
                } else {
                    g.cell as i64
                };
                let mut q = pos + dir.mul_div(look, UNIT_Q);
                if walled {
                    // He looks no farther than a wall lets him go.
                    q = walls.clip(pos, q);
                }
                let (qx, qy, ux, uy) = g.split(q);
                let hint = (bx >> TILE_SHIFT, by >> TILE_SHIFT, base[i] >> CELL_BITS);
                let mut a = read(den, &find_block(tiles, qx, qy, hint));
                let mut wq = weights(ux, uy);
                if walled && walls.near(q) {
                    let ahead;
                    (wq, ahead) = walls.cut(q, |k| g.centre(qx, qy, k), wq);
                    for k in 0..4 {
                        if !ahead[k] {
                            a[k] = [0, 0];
                        }
                    }
                }
                let (mut mine, mut theirs) = (0, 0);
                for k in 0..4 {
                    foe |= a[k][1 - side];
                    mine += a[k][side] as i32 * wq[k];
                    theirs += a[k][1 - side] as i32 * wq[k];
                }
                let (mine, theirs) = (mine / 256, theirs / 256);
                if dist > 0 {
                    // Enemies ahead count for more than friends: an open line of them is a
                    // wall to a walking man.
                    let thick = mine as i64 + theirs as i64 * g.foe_q8 / 256;
                    let share = walk_q8(((thick * g.walk_q16) >> 16) as u32) as i64;
                    let reach = tg.speed as i64 * share * dt / 256_000;
                    s += if reach >= dist {
                        to
                    } else {
                        dir.mul_div(reach, UNIT_Q)
                    };
                }
                if let Some(face) = lean {
                    // A press gains ground while there is room short of packing, and none on
                    // ground the enemy holds: he must be pushed off it first.
                    let room = ((g.pack - mine - theirs) as i64 * 256 / (g.pack - g.jam) as i64)
                        .clamp(0, 256);
                    let open = (g.hold - theirs).max(0) as i64;
                    let gain = p.press_mm_s as i64 * tg.press as i64 * room * open * dt
                        / (255 * 256 * 1000 * g.hold as i64);
                    if gain > 0 {
                        s += face.unit(gain as i32);
                    }
                }
            }

            let packed_and_moving = match p.contact {
                Contact::Line => false,
                Contact::Crowd => here >= g.crowd && (s - pushed).len_sq() >= moving2,
                Contact::All => true,
            };
            let mut k = kind[i];
            if packed_and_moving {
                k |= TOUCHES;
                *touching = true;
            }
            if foe != 0 || packed_and_moving {
                k |= HOT;
                hot.push(i as u32);
            }
            kind[i] = k;
            if here > g.crush {
                crushing.push((i as u32, here - g.crush));
            }
            step[i] = s;
            far = far.max(s.len_sq());
        }
        stats.free = free;
        far
    }

    /// Keeps hot men apart from those they may not stand on, by position. The step is cut into
    /// passes short enough that no two men can cross within one. In each pass every man is
    /// moved against the others as they stood before it, so no man's turn in the loop favours
    /// him.
    fn solve(&mut self, bodies: &Bodies, far2: i64) {
        let Field {
            p,
            g,
            walls,
            tiles,
            cell,
            kind,
            step,
            corr,
            held,
            hot,
            touching,
            cstart,
            ccursor,
            corder,
            lists,
            ranges,
            delta,
            stats,
            ..
        } = self;
        let cells = tiles.cells();
        let groups = 2 * cells;
        cstart.clear();
        cstart.resize(groups + 1, 0);
        for (i, &c) in cell.iter().enumerate() {
            if c != NONE {
                cstart[2 * c as usize + bodies.side[i] as usize + 1] += 1;
            }
        }
        for a in 0..groups {
            cstart[a + 1] += cstart[a];
        }
        ccursor.clear();
        ccursor.extend_from_slice(&cstart[..groups]);
        corder.clear();
        corder.resize(cstart[groups] as usize, 0);
        for (i, &c) in cell.iter().enumerate() {
            if c != NONE {
                let k = &mut ccursor[2 * c as usize + bodies.side[i] as usize];
                corder[*k as usize] = i as u32;
                *k += 1;
            }
        }

        let walled = !walls.is_empty();
        let far = (far2 as u64).isqrt() as i64;
        let widest = p.guard_mm.max(g.body_mm) as i64;
        // Room for what the solve itself may move a man.
        let slack = widest / 2;
        lists.clear();
        ranges.clear();
        ranges.push(0);
        let mut far_hot = 0i64;
        for &i in hot.iter() {
            let iu = i as usize;
            let pi = bodies.pos[iu];
            let li = step[iu].length();
            far_hot = far_hot.max(li);
            let bound = widest + li + far + slack;
            let (x0, x1) = (
                ((pi.x as i64 - bound) >> g.shift) as i32,
                ((pi.x as i64 + bound) >> g.shift) as i32,
            );
            let (y0, y1) = (
                ((pi.y as i64 - bound) >> g.shift) as i32,
                ((pi.y as i64 + bound) >> g.shift) as i32,
            );
            let side = bodies.side[iu];
            let touches = kind[iu] & TOUCHES != 0;
            // Where no one touches his own side, only the enemy's men are looked at.
            let foes = 1 - side as usize;
            let (first, last) = if *touching { (0, 2) } else { (foes, foes + 1) };
            let mut tile = (i32::MAX, i32::MAX, NONE);
            for cy in y0..=y1 {
                for cx in x0..=x1 {
                    let (tx, ty) = (cx >> TILE_SHIFT, cy >> TILE_SHIFT);
                    if (tx, ty) != (tile.0, tile.1) {
                        tile = (tx, ty, tiles.find_tile(tx, ty).unwrap_or(NONE));
                    }
                    if tile.2 == NONE {
                        continue;
                    }
                    let a = 2 * ((tile.2 << CELL_BITS) | local(cx, cy)) as usize;
                    for &j in &corder[cstart[a + first] as usize..cstart[a + last] as usize] {
                        if j == i {
                            continue;
                        }
                        let ju = j as usize;
                        let kj = kind[ju];
                        let gap = if bodies.side[ju] != side {
                            p.guard_mm
                        } else if touches || kj & TOUCHES != 0 {
                            g.body_mm
                        } else {
                            continue;
                        };
                        let lim = gap as i64 + li + far + slack;
                        let toward = bodies.pos[ju] - pi;
                        if toward.len_sq() > lim * lim {
                            continue;
                        }
                        // A man held by a wall cannot give toward it: the other gives all.
                        let (i_held, j_held) = if walled {
                            (into(held[iu], -toward), into(held[ju], toward))
                        } else {
                            (false, false)
                        };
                        let mine_q8 = if kj & HOT == 0 || (j_held && !i_held) {
                            256
                        } else if i_held && !j_held {
                            0
                        } else {
                            let closing = step[iu].dot(toward).max(0);
                            let his = (-step[ju].dot(toward)).max(0);
                            if closing + his == 0 {
                                128
                            } else {
                                (256 * closing / (closing + his)) as i32
                            }
                        };
                        lists.push(Near {
                            man: j,
                            gap,
                            mine_q8,
                        });
                    }
                }
            }
            ranges.push(lists.len() as u32);
        }

        let finest = p.guard_mm.min(g.body_mm).max(1) as i64;
        let passes = ((far_hot + far + finest - 1) / finest).clamp(2, 8);
        delta.clear();
        delta.resize(hot.len(), Vec2::ZERO);
        let mut keep: Vec<(Vec2, i64)> = Vec::new();
        // After the last of the step, two passes more with no one advancing, to settle what
        // men who each gave half have left between them.
        for pass in 1..=passes + SETTLE {
            let k = pass.min(passes);
            for (h, &i) in hot.iter().enumerate() {
                let iu = i as usize;
                let from = bodies.pos[iu] + step[iu].mul_div(k, passes) + corr[iu];
                let near = &lists[ranges[h] as usize..ranges[h + 1] as usize];
                if near.is_empty() {
                    continue;
                }
                // What he must keep from each of the others, taken where they stood before
                // this pass. Of an overlap between two hot men each gives his share, so each
                // need only reach the gap less the other's share.
                keep.clear();
                for e in near {
                    let ju = e.man as usize;
                    let xj = bodies.pos[ju] + step[ju].mul_div(k, passes) + corr[ju];
                    let gap = e.gap as i64;
                    let d2 = from.dist_sq(xj);
                    let theirs = if e.mine_q8 < 256 && d2 < gap * gap {
                        (gap - (d2 as u64).isqrt() as i64) * (256 - e.mine_q8) as i64 / 256
                    } else {
                        0
                    };
                    keep.push((xj, gap - theirs));
                }
                // He is moved clear of each in turn, and again: clear of one may be into
                // another.
                let mut x = from;
                for _ in 0..SWEEPS {
                    let mut clear = true;
                    for (n, &(xj, gap)) in keep.iter().enumerate() {
                        let mut apart = x - xj;
                        let d2 = apart.len_sq();
                        if d2 >= gap * gap {
                            continue;
                        }
                        let mut dist = (d2 as u64).isqrt() as i64;
                        if dist == 0 {
                            // Exactly coincident: part them along x, lower id to the west.
                            apart = Vec2::new(if i < near[n].man { -1 } else { 1 }, 0);
                            dist = 1;
                        }
                        // Rounded up, so that a millimetre's overlap is not left for ever.
                        let out = gap - dist;
                        x += Vec2::new(
                            ((apart.x as i64 * out + apart.x.signum() as i64 * (dist - 1)) / dist)
                                as i32,
                            ((apart.y as i64 * out + apart.y.signum() as i64 * (dist - 1)) / dist)
                                as i32,
                        );
                        clear = false;
                    }
                    if walled {
                        // A wall gives nothing: all of it is his.
                        let y = walls.clip(bodies.pos[iu], x);
                        if y != x {
                            x = y;
                            clear = false;
                        }
                    }
                    if clear {
                        break;
                    }
                }
                delta[h] = x - from;
            }
            for (h, &i) in hot.iter().enumerate() {
                corr[i as usize] += delta[h];
            }
        }
        stats.passes = (passes + SETTLE) as u32;
    }
}

impl Substrate for Field {
    fn advance(
        &mut self,
        bodies: &mut Bodies,
        targets: &[Target],
        dt: SimMs,
        rng: &Rng,
        tick: u32,
        out: &mut Vec<Event>,
    ) {
        // The whole of a man step's motion is done at once; between man steps nothing reads
        // the state but the viewer, which interpolates.
        if !is_man_step(tick) {
            return;
        }
        let n = bodies.len();
        let dt = (dt.0 * MAN_EVERY) as i64;
        self.deposit(bodies);
        let far2 = self.steps(bodies, targets, dt);
        self.corr.resize(n, Vec2::ZERO);
        self.stats.passes = 0;
        if !self.hot.is_empty() {
            self.solve(bodies, far2);
        }
        let turn = (self.p.turn_per_tick as u32 * MAN_EVERY).min(u16::MAX as u32) as u16;
        let per_s = if 1000 % dt == 0 {
            (1000 / dt) as i32
        } else {
            0
        };
        let walled = !self.walls.is_empty();
        for (i, tg) in targets.iter().enumerate() {
            if self.base[i] == NONE {
                bodies.vel[i] = Vec2::ZERO;
                continue;
            }
            let mut mv = self.step[i] + self.corr[i];
            if walled {
                let at = bodies.pos[i];
                mv = self.walls.clip(at, at + mv) - at;
            }
            bodies.pos[i] += mv;
            bodies.vel[i] = if per_s != 0 {
                Vec2::new(mv.x * per_s, mv.y * per_s)
            } else {
                mv.mul_div(1000, dt)
            };
            if let Some(f) = tg.face {
                bodies.facing[i] = bodies.facing[i].rotate_toward(f, turn);
            }
        }
        for &i in &self.hot {
            self.corr[i as usize] = Vec2::ZERO;
        }
        let (g, rate_q16) = (self.g, self.p.crush_rate_q16 as u64);
        let chance_at = |over: i64| {
            let rate = (g.q8(over) as u64 * rate_q16) >> 8;
            Prob::from_rate(rate.min(u32::MAX as u64) as u32, SimMs(dt as u32))
        };
        if self.crush_dt != dt {
            self.crush_chance.clear();
            self.crush_dt = dt;
        }
        for &(i, over) in &self.crushing {
            let chance = if over as usize >= CRUSH_TABLE {
                chance_at(over as i64)
            } else {
                while self.crush_chance.len() <= over as usize {
                    self.crush_chance
                        .push(chance_at(self.crush_chance.len() as i64));
                }
                self.crush_chance[over as usize]
            };
            if rng.chance(chance, Key::new(Stream::Substrate, i, tick, PURPOSE_CRUSH)) {
                out.push(Event {
                    tick,
                    man: ManId(i),
                    pos: bodies.pos[i as usize],
                    kind: EventKind::Wound {
                        to: Wound::Disabled,
                    },
                    causes: [Cause::Crush, Cause::None, Cause::None],
                });
            }
        }
        self.stats.hot = self.hot.len() as u32;
        self.stats.tiles = self.tiles.len() as u32;
    }
}
