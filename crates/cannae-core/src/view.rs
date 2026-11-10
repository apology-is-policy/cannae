//! What a man sees, defined once for both engines.
//!
//! Counts come from a canonical grid of 2 m cells aligned to the world origin. A man's window is
//! the 5 x 5 block of cells centred on his own, split by his facing (snapped to the nearest
//! axis) into the rows ahead of him, his own row, and the rows behind. Enemies within reach and
//! crowding are exact, from positions. Both engines must produce identical Views from identical
//! states; [`GridPerception`] is the reference, and a conformance test holds every other
//! implementation to it.

use crate::angle::Turn;
use crate::event::{Event, EventKind};
use crate::observe::{Bodies, BodyId, ManId, Observed, Posture, Role, Side};
use crate::units::Vec2;

pub const CELL_MM: i32 = 2_000;
/// Cells on each side of a man's own cell in his window.
pub const WINDOW: i32 = 2;
pub const REACH_MM: i64 = 3_000;
pub const MAX_NEAR: usize = 8;
pub const CROWD_MM: i64 = 1_000;
pub const CROWD_CAP: u8 = 15;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Group {
    Ahead = 0,
    Beside = 1,
    Behind = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Rel {
    Own = 0,
    Enemy = 1,
}

/// Posture counts in a man's window, indexed `[group][own or enemy][posture]`. He does not
/// count himself.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Counts(pub [[[u16; 5]; 2]; 3]);

impl Counts {
    pub fn get(&self, g: Group, r: Rel, p: Posture) -> u16 {
        self.0[g as usize][r as usize][p as usize]
    }

    /// Over all three groups.
    pub fn total(&self, r: Rel, p: Posture) -> u32 {
        self.0
            .iter()
            .map(|g| g[r as usize][p as usize] as u32)
            .sum()
    }
}

/// Where something lies relative to a man's facing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Sector {
    Front = 0,
    Left = 1,
    Right = 2,
    Rear = 3,
}

impl Sector {
    /// The sector of direction `dir` for a man facing `facing`. Front and rear are 90 degrees
    /// wide each.
    pub fn of(facing: Turn, dir: Turn) -> Sector {
        let d = facing.diff(dir) as i32;
        match d {
            -8192..=8192 => Sector::Front,
            8193..=24576 => Sector::Left,
            -24576..=-8193 => Sector::Right,
            _ => Sector::Rear,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NearEnemy {
    pub id: ManId,
    /// His position minus mine.
    pub offset: Vec2,
    pub posture: Posture,
    /// Where he stands relative to my facing.
    pub sector: Sector,
    /// Whether I stand in his front sector.
    pub faces_me: bool,
}

/// What happened in a man's window since the last man step: things that shock, as opposed to
/// states that alarm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Shocks {
    /// Men of his own side who went down.
    pub falls: u8,
    /// Men of his own side who turned.
    pub turns: u8,
    pub standard_fell: bool,
    pub leader_fell: bool,
}

impl Shocks {
    pub fn any(&self) -> bool {
        self.falls > 0 || self.turns > 0 || self.standard_fell || self.leader_fell
    }
}

/// Bits of [`SlotView::neighbours`]: which adjacent slots are filled.
pub mod nb {
    pub const LEFT: u8 = 1;
    pub const RIGHT: u8 = 2;
    pub const FRONT: u8 = 4;
    pub const BEHIND: u8 = 8;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SlotView {
    /// Where his slot is now.
    pub target: Vec2,
    /// The frame's facing.
    pub facing: Turn,
    pub in_place: bool,
    /// Which adjacent slots exist; see [`nb`]. The edge of a body has none on that side.
    pub slots: u8,
    /// Which adjacent slots are filled by a man who is up and has not turned. A slot that
    /// exists and is not filled is a gap.
    pub neighbours: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Order {
    #[default]
    Hold = 0,
    Advance = 1,
    Charge = 2,
    Retire = 3,
    Pursue = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Standard {
    Carried(ManId),
    Down(Vec2),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BodyState {
    pub standard: Standard,
    pub leader: Option<ManId>,
    pub order: Order,
    /// The tick at which the current order was given.
    pub order_tick: u32,
}

/// What the frames module tells perception and target resolution. The frames module (carriers,
/// slots, ties and drills) implements it; [`NoFrames`] serves tests.
pub trait FrameQuery {
    fn slot(&self, man: ManId) -> Option<SlotView>;
    fn body(&self, body: BodyId) -> Option<BodyState>;
}

pub struct NoFrames;

impl FrameQuery for NoFrames {
    fn slot(&self, _: ManId) -> Option<SlotView> {
        None
    }
    fn body(&self, _: BodyId) -> Option<BodyState> {
        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum StandardSeen {
    /// Carried. The bearer's posture says whether it stands, goes forward, or is being carried
    /// off: a standard in flight is no place to rally.
    Up {
        dist_mm: i32,
        posture: Posture,
    },
    Down {
        dist_mm: i32,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LeaderSeen {
    pub posture: Posture,
    pub dist_mm: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct View {
    pub man: ManId,
    pub tick: u32,
    pub own: Observed,
    pub counts: Counts,
    /// Enemies within reach, nearest first (ties by id); `None` after the last.
    pub near: [Option<NearEnemy>; MAX_NEAR],
    /// Bodies within 1 m, capped.
    pub crowding: u8,
    pub shocks: Shocks,
    pub slot: Option<SlotView>,
    pub standard: Option<StandardSeen>,
    pub leader: Option<LeaderSeen>,
    /// A closer of his own side stands behind him in his window, steady or forward.
    pub closer_behind: bool,
    pub order: Order,
}

impl View {
    pub fn near(&self) -> impl Iterator<Item = &NearEnemy> {
        self.near.iter().map_while(Option::as_ref)
    }
}

/// The cheap test that decides dormancy. A man model may declare a man dormant only when his
/// hazards would be exactly zero for any full View consistent with this.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Coarse {
    /// An enemy standing (not down) anywhere in his window.
    pub enemy_near: bool,
    /// A shock in his window, or his standard or leader fell.
    pub shock_near: bool,
    pub out_of_place: bool,
    /// His body was given an order since the last man step.
    pub order_new: bool,
    /// A man of his own side in his window is giving ground or has turned. These are states,
    /// not shocks: a man who turned ten seconds ago and now runs past still alarms.
    pub own_unsteady: bool,
    /// The order in force for his body. A charge keeps men awake until they have gone.
    pub order: Order,
}

/// Builds Views. Each engine provides one; they must agree with the reference.
pub trait Perceive {
    /// Refreshes from the snapshot at `tick`. `recent` holds the events since the previous man
    /// step, `prev_step` its tick.
    fn rebuild(&mut self, bodies: &Bodies, recent: &[Event], tick: u32, prev_step: u32);
    fn coarse<F: FrameQuery>(&self, bodies: &Bodies, frames: &F, man: ManId) -> Coarse;
    fn view<F: FrameQuery>(&self, bodies: &Bodies, frames: &F, man: ManId) -> View;
}

fn cell_of(p: Vec2) -> (i32, i32) {
    (p.x.div_euclid(CELL_MM), p.y.div_euclid(CELL_MM))
}

fn cell_key(cx: i32, cy: i32) -> i64 {
    ((cy as i64) << 32) | cx as u32 as i64
}

/// Which way the window is split: the facing snapped to the nearest axis, as (ahead x, ahead y).
fn axis(facing: Turn) -> (i32, i32) {
    match ((facing.0 as u32 + 8192) >> 14) & 3 {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        _ => (0, -1),
    }
}

#[derive(Clone, Copy, Debug)]
struct Shock {
    key: i64,
    side: Side,
    man: ManId,
    to: Posture,
}

/// The reference perception: a sorted list of (cell, man), rebuilt each man step. Plain and
/// slow enough to be obviously right.
#[derive(Clone, Debug, Default)]
pub struct GridPerception {
    cells: Vec<(i64, u32)>,
    shocks: Vec<Shock>,
    standards_down: Vec<BodyId>,
    tick: u32,
    prev_step: u32,
}

impl GridPerception {
    fn men_in(&self, cx: i32, cy: i32) -> impl Iterator<Item = ManId> + '_ {
        let k = cell_key(cx, cy);
        let lo = self.cells.partition_point(|c| c.0 < k);
        self.cells[lo..]
            .iter()
            .take_while(move |c| c.0 == k)
            .map(|c| ManId(c.1))
    }

    fn window(&self, p: Vec2) -> impl Iterator<Item = (i32, i32, ManId)> + '_ {
        let (cx, cy) = cell_of(p);
        (-WINDOW..=WINDOW).flat_map(move |dy| {
            (-WINDOW..=WINDOW)
                .flat_map(move |dx| self.men_in(cx + dx, cy + dy).map(move |m| (dx, dy, m)))
        })
    }

    fn in_window(&self, p: Vec2, key: i64) -> bool {
        let (cx, cy) = cell_of(p);
        let (kx, ky) = (key as u32 as i32, (key >> 32) as i32);
        (kx - cx).abs() <= WINDOW && (ky - cy).abs() <= WINDOW
    }

    fn shocks_for<F: FrameQuery>(&self, bodies: &Bodies, frames: &F, man: ManId) -> Shocks {
        let i = man.0 as usize;
        let (p, side) = (bodies.pos[i], bodies.side[i]);
        let mut s = Shocks::default();
        for sh in &self.shocks {
            if sh.side == side && self.in_window(p, sh.key) {
                match sh.to {
                    Posture::Down => s.falls = s.falls.saturating_add(1),
                    Posture::Turned => s.turns = s.turns.saturating_add(1),
                    _ => {}
                }
            }
        }
        if let Some(b) = bodies.body[i] {
            s.standard_fell = self.standards_down.contains(&b);
            if let Some(leader) = frames.body(b).and_then(|st| st.leader) {
                s.leader_fell = self
                    .shocks
                    .iter()
                    .any(|sh| sh.man == leader && sh.to == Posture::Down);
            }
        }
        s
    }
}

impl Perceive for GridPerception {
    fn rebuild(&mut self, bodies: &Bodies, recent: &[Event], tick: u32, prev_step: u32) {
        self.tick = tick;
        self.prev_step = prev_step;
        self.cells.clear();
        for (i, p) in bodies.pos.iter().enumerate() {
            let (cx, cy) = cell_of(*p);
            self.cells.push((cell_key(cx, cy), i as u32));
        }
        self.cells.sort_unstable();
        self.shocks.clear();
        self.standards_down.clear();
        for e in recent {
            match e.kind {
                EventKind::Posture { to, .. } if matches!(to, Posture::Down | Posture::Turned) => {
                    let (cx, cy) = cell_of(e.pos);
                    self.shocks.push(Shock {
                        key: cell_key(cx, cy),
                        side: bodies.side[e.man.0 as usize],
                        man: e.man,
                        to,
                    });
                }
                EventKind::StandardDown { body } => self.standards_down.push(body),
                _ => {}
            }
        }
    }

    fn coarse<F: FrameQuery>(&self, bodies: &Bodies, frames: &F, man: ManId) -> Coarse {
        let i = man.0 as usize;
        let (p, side) = (bodies.pos[i], bodies.side[i]);
        let enemy_near = self.window(p).any(|(_, _, m)| {
            let j = m.0 as usize;
            bodies.side[j] != side && bodies.posture[j] != Posture::Down
        });
        let own_unsteady = self.window(p).any(|(_, _, m)| {
            let j = m.0 as usize;
            bodies.side[j] == side && matches!(bodies.posture[j], Posture::Back | Posture::Turned)
        });
        let shock_near = self.shocks_for(bodies, frames, man).any();
        let body = bodies.body[i].and_then(|b| frames.body(b));
        Coarse {
            enemy_near,
            shock_near,
            out_of_place: bodies.body[i].is_some() && !bodies.in_place[i],
            order_new: body.is_some_and(|b| b.order_tick >= self.prev_step && self.tick > 0),
            own_unsteady,
            order: body.map_or(Order::Hold, |b| b.order),
        }
    }

    fn view<F: FrameQuery>(&self, bodies: &Bodies, frames: &F, man: ManId) -> View {
        let i = man.0 as usize;
        let own = bodies.get(man);
        let (ax, ay) = axis(own.facing);
        let mut counts = Counts::default();
        let mut near: Vec<(i64, NearEnemy)> = Vec::new();
        let mut crowding: u32 = 0;
        let mut closer_behind = false;
        let ahead = own.facing.unit(1 << 10);
        for (dx, dy, m) in self.window(own.pos) {
            if m == man {
                continue;
            }
            let j = m.0 as usize;
            let along = dx * ax + dy * ay;
            let g = match along.signum() {
                1 => Group::Ahead,
                0 => Group::Beside,
                _ => Group::Behind,
            };
            let r = if bodies.side[j] == own.side {
                Rel::Own
            } else {
                Rel::Enemy
            };
            let c = &mut counts.0[g as usize][r as usize][bodies.posture[j] as usize];
            *c = c.saturating_add(1);
            let offset = bodies.pos[j] - own.pos;
            if r == Rel::Own
                && bodies.role[j] == Role::Closer
                && matches!(bodies.posture[j], Posture::Steady | Posture::Forward)
                && offset.dot(ahead) < 0
            {
                closer_behind = true;
            }
            let d2 = offset.len_sq();
            if d2 <= CROWD_MM * CROWD_MM {
                crowding += 1;
            }
            if r == Rel::Enemy && d2 <= REACH_MM * REACH_MM {
                let to_me = Turn::of(-offset);
                near.push((
                    d2,
                    NearEnemy {
                        id: m,
                        offset,
                        posture: bodies.posture[j],
                        sector: Turn::of(offset)
                            .map_or(Sector::Front, |d| Sector::of(own.facing, d)),
                        faces_me: to_me
                            .is_none_or(|d| Sector::of(bodies.facing[j], d) == Sector::Front),
                    },
                ));
            }
        }
        near.sort_unstable_by_key(|(d2, e)| (*d2, e.id));
        let mut near_arr = [None; MAX_NEAR];
        for (slot, (_, e)) in near_arr.iter_mut().zip(near) {
            *slot = Some(e);
        }
        let body = bodies.body[i].and_then(|b| frames.body(b));
        let dist = |q: Vec2| (q - own.pos).length().min(i32::MAX as i64) as i32;
        View {
            man,
            tick: self.tick,
            own,
            counts,
            near: near_arr,
            crowding: crowding.min(CROWD_CAP as u32) as u8,
            shocks: self.shocks_for(bodies, frames, man),
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
    use crate::event::Cause;
    use crate::observe::{Observed, Role, Wound};

    fn put(b: &mut Bodies, x: i32, y: i32, side: Side, posture: Posture, facing: u16) -> ManId {
        b.push(Observed {
            pos: Vec2::new(x, y),
            side,
            posture,
            facing: Turn(facing),
            ..Default::default()
        })
    }

    #[test]
    fn groups_follow_snapped_facing() {
        let mut b = Bodies::default();
        let me = put(&mut b, 1_000, 1_000, Side::A, Posture::Steady, 0); // facing +x
        put(&mut b, 3_500, 1_000, Side::B, Posture::Steady, 32_768); // a cell ahead
        put(&mut b, 1_500, 1_200, Side::A, Posture::Steady, 0); // same cell
        put(&mut b, -1_500, 1_000, Side::A, Posture::Turned, 32_768); // a cell behind
        put(&mut b, 9_000, 1_000, Side::B, Posture::Steady, 0); // outside the window
        let mut g = GridPerception::default();
        g.rebuild(&b, &[], 5, 0);
        let v = g.view(&b, &NoFrames, me);
        assert_eq!(v.counts.get(Group::Ahead, Rel::Enemy, Posture::Steady), 1);
        assert_eq!(v.counts.get(Group::Beside, Rel::Own, Posture::Steady), 1);
        assert_eq!(v.counts.get(Group::Behind, Rel::Own, Posture::Turned), 1);
        assert_eq!(v.counts.total(Rel::Enemy, Posture::Steady), 1);
        // Turning to face -x swaps ahead and behind.
        b.facing[0] = Turn(32_768);
        g.rebuild(&b, &[], 5, 0);
        let v = g.view(&b, &NoFrames, me);
        assert_eq!(v.counts.get(Group::Behind, Rel::Enemy, Posture::Steady), 1);
        assert_eq!(v.counts.get(Group::Ahead, Rel::Own, Posture::Turned), 1);
    }

    #[test]
    fn a_closer_counts_only_behind_and_standing() {
        let mut b = Bodies::default();
        let me = put(&mut b, 0, 0, Side::A, Posture::Steady, 0); // facing +x
        let closer = put(&mut b, -1_500, 3_000, Side::A, Posture::Steady, 0);
        b.role[closer.0 as usize] = Role::Closer;
        let mut g = GridPerception::default();
        g.rebuild(&b, &[], 5, 0);
        assert!(g.view(&b, &NoFrames, me).closer_behind);
        assert!(!g.coarse(&b, &NoFrames, me).own_unsteady);
        // Once he gives ground he closes nothing, and he is one more man going back.
        b.posture[closer.0 as usize] = Posture::Back;
        g.rebuild(&b, &[], 5, 0);
        assert!(!g.view(&b, &NoFrames, me).closer_behind);
        assert!(g.coarse(&b, &NoFrames, me).own_unsteady);
        // A closer ahead of him is no closer to him.
        b.posture[closer.0 as usize] = Posture::Steady;
        b.pos[closer.0 as usize] = Vec2::new(1_500, 3_000);
        g.rebuild(&b, &[], 5, 0);
        assert!(!g.view(&b, &NoFrames, me).closer_behind);
    }

    #[test]
    fn near_enemies_are_exact_and_sorted() {
        let mut b = Bodies::default();
        let me = put(&mut b, 0, 0, Side::A, Posture::Steady, 0);
        let far = put(&mut b, 2_900, 0, Side::B, Posture::Steady, 32_768);
        let close = put(&mut b, 0, 1_000, Side::B, Posture::Forward, 0);
        put(&mut b, 3_100, 0, Side::B, Posture::Steady, 32_768); // just out of reach
        put(&mut b, -1_000, 0, Side::B, Posture::Steady, 0); // behind me, facing my back
        let mut g = GridPerception::default();
        g.rebuild(&b, &[], 5, 0);
        let v = g.view(&b, &NoFrames, me);
        let ids: Vec<u32> = v.near().map(|e| e.id.0).collect();
        assert_eq!(ids, vec![close.0, 4, far.0]);
        let e: Vec<_> = v.near().collect();
        assert_eq!(e[0].sector, Sector::Left);
        assert!(!e[0].faces_me); // he faces +x; I am to his -y side
        assert_eq!(e[1].sector, Sector::Rear);
        assert!(e[1].faces_me);
        assert_eq!(e[2].sector, Sector::Front);
        assert!(e[2].faces_me);
        assert_eq!(v.crowding, 2);
    }

    #[test]
    fn shocks_count_own_side_in_window_only() {
        let mut b = Bodies::default();
        let me = put(&mut b, 0, 0, Side::A, Posture::Steady, 0);
        let friend = put(&mut b, 2_000, 0, Side::A, Posture::Down, 0);
        let foe = put(&mut b, 2_000, 500, Side::B, Posture::Down, 0);
        let far_friend = put(&mut b, 20_000, 0, Side::A, Posture::Turned, 0);
        let ev = |man: ManId, pos: Vec2, to: Posture| Event {
            tick: 3,
            man,
            pos,
            kind: EventKind::Posture {
                from: Posture::Steady,
                to,
            },
            causes: [Cause::Bout, Cause::None, Cause::None],
        };
        let recent = [
            ev(friend, b.pos[1], Posture::Down),
            ev(foe, b.pos[2], Posture::Down),
            ev(far_friend, b.pos[3], Posture::Turned),
        ];
        let mut g = GridPerception::default();
        g.rebuild(&b, &recent, 5, 0);
        let v = g.view(&b, &NoFrames, me);
        assert_eq!(v.shocks.falls, 1);
        assert_eq!(v.shocks.turns, 0);
        let c = g.coarse(&b, &NoFrames, me);
        assert!(c.shock_near);
        assert!(!c.enemy_near); // the only enemy in the window is down
        assert_eq!(b.wound[0], Wound::Sound);
    }
}
