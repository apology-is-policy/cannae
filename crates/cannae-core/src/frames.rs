//! Bodies of men and their frames: standards, slots, ties and drills (docs/rival/02 sec. 1-4).
//!
//! A body is the men who follow one standard. Its frame is wherever the standard-bearer stands:
//! `origin = his position - his slot's offset`, taken afresh at every man step. So the frame goes
//! where he goes and stops when he is stopped; it is never fitted to where the men are. Every
//! other man has a slot in the frame and walks to it.
//!
//! A file is a list of men, front to rear, and a man's rank is his index in it. Closing a file
//! over a gap is removing a man from the list, so gaps travel to the rear, and a file that has
//! lost men is short at the back.
//!
//! Nothing here draws a random number. Who lifts a fallen standard, and when, is a rule.

use crate::angle::Turn;
use crate::event::{Cause, Event, EventKind};
use crate::model::Frames;
use crate::observe::{Bodies, BodyId, ManId, Posture, Role, Side, Wound};
use crate::units::Vec2;
use crate::view::{BodyState, FrameQuery, Order, SlotView, Standard, nb};

/// How a troop type keeps its order. All of it is drill.
///
/// Times are in base ticks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tie {
    /// A man within this distance of his slot is in his place.
    pub tolerance_mm: i32,
    /// How long before a file closes over a man who has fallen or fled. `None`: it never does.
    pub close_after: Option<u32>,
    /// How long before a man standing by a fallen standard lifts it. `None`: no one does.
    pub lift_after: Option<u32>,
    /// How near the fallen standard he must stand.
    pub lift_reach_mm: i32,
    /// How long before the closer takes a fallen leader's place. `None`: no succession.
    pub succeed_after: Option<u32>,
    /// How far ahead of the men the frame leads while the body moves. It must exceed what a man
    /// covers in one man step at a run, or the men would overtake their own slots.
    pub lead_mm: i32,
    /// The gap between the fronts at which an advancing body halts: just out of reach.
    pub standoff_mm: i32,
    /// The same for a charge, which stops only at striking distance.
    pub contact_mm: i32,
}

impl Tie {
    /// A maniple: stiff, quick, with rules for the standard and the succession. Placeholder
    /// values; their priors are fixed with the others before any scored run.
    pub const DRILLED: Tie = Tie {
        tolerance_mm: 600,
        close_after: Some(40),
        lift_after: Some(60),
        lift_reach_mm: 2_500,
        succeed_after: Some(100),
        lead_mm: 1_500,
        standoff_mm: 4_000,
        contact_mm: 1_200,
    };

    /// A warband: loose, slow, and with no rule for who replaces whom. Placeholder values.
    pub const LOOSE: Tie = Tie {
        tolerance_mm: 1_200,
        close_after: Some(200),
        lift_after: None,
        lift_reach_mm: 2_500,
        succeed_after: None,
        lead_mm: 1_500,
        standoff_mm: 4_000,
        contact_mm: 1_200,
    };
}

/// One body as it stands at setup: `files` by `ranks` men numbered from `first`, rank by rank
/// from the front, each rank from its left-hand file, and then a closer, if any, as the next
/// man. This is the order in which the harness places them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub body: BodyId,
    pub centre: Vec2,
    pub facing: Turn,
    pub files: u32,
    pub ranks: u32,
    pub file_mm: i32,
    pub rank_mm: i32,
    pub first: ManId,
    pub closer: bool,
    pub tie: Tie,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Place {
    /// No slot: he never had one, or his file closed over it.
    Adrift,
    File {
        frame: u32,
        file: u16,
        rank: u16,
    },
    /// Behind the rear rank.
    Closer {
        frame: u32,
    },
}

/// A frame's two axes in Q15, so that a slot's position costs four multiplications. The
/// arithmetic repeats [`Turn::unit`] exactly, which is what the harness places men with.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Basis {
    ahead: (i32, i32),
    left: (i32, i32),
}

impl Basis {
    fn of(facing: Turn) -> Basis {
        let l = Turn(facing.0.wrapping_add(Turn::QUARTER));
        Basis {
            ahead: (facing.cos_q15(), facing.sin_q15()),
            left: (l.cos_q15(), l.sin_q15()),
        }
    }

    fn scaled(axis: (i32, i32), len: i32) -> Vec2 {
        Vec2::new(
            ((axis.0 as i64 * len as i64) >> 15) as i32,
            ((axis.1 as i64 * len as i64) >> 15) as i32,
        )
    }

    fn at(&self, ahead_mm: i32, left_mm: i32) -> Vec2 {
        Basis::scaled(self.ahead, ahead_mm) + Basis::scaled(self.left, left_mm)
    }

    /// The components of `v` along the facing and to its left.
    fn split(&self, v: Vec2) -> (i64, i64) {
        let along = (v.x as i64 * self.ahead.0 as i64 + v.y as i64 * self.ahead.1 as i64) >> 15;
        let left = (v.x as i64 * self.left.0 as i64 + v.y as i64 * self.left.1 as i64) >> 15;
        (along, left)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Frame {
    body: BodyId,
    side: Side,
    origin: Vec2,
    facing: Turn,
    basis: Basis,
    file_mm: i32,
    rank_mm: i32,
    /// Depth at setup. It fixes where the front rank stands in the frame, so that a file which
    /// loses men is short at the back, not shifted.
    depth: u32,
    files: Vec<Vec<ManId>>,
    closer: Option<ManId>,
    standard: Standard,
    standard_tick: u32,
    leader: Option<ManId>,
    leader_file: u16,
    leader_fell: Option<u32>,
    order: Order,
    order_tick: u32,
    /// Added to every slot's target while the body moves.
    lead: Vec2,
    /// Men who lost their slots and may fall in again.
    adrift: Vec<ManId>,
    tie: Tie,
}

impl Frame {
    fn offset(&self, file: u16, rank: u16) -> Vec2 {
        let ahead = (self.depth as i32 - 1 - 2 * rank as i32) * self.rank_mm / 2;
        let left = (self.files.len() as i32 - 1 - 2 * file as i32) * self.file_mm / 2;
        self.basis.at(ahead, left)
    }

    /// The closer's post: one rank behind the deepest file.
    fn closer_offset(&self) -> Vec2 {
        let deepest = self.files.iter().map(Vec::len).max().unwrap_or(0) as i32;
        let ahead = (self.depth as i32 - 1 - 2 * deepest) * self.rank_mm / 2;
        self.basis.at(ahead, 0)
    }

    fn offset_of(&self, place: Place) -> Option<Vec2> {
        match place {
            Place::File { file, rank, .. } => Some(self.offset(file, rank)),
            Place::Closer { .. } => Some(self.closer_offset()),
            Place::Adrift => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Timed {
    tick: u32,
    body: BodyId,
    order: Order,
}

/// The frames of every body on the field.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BodyFrames {
    frames: Vec<Frame>,
    place: Vec<Place>,
    /// The tick at which each man was first seen fallen or fled; `u32::MAX` while he stands.
    gone_since: Vec<u32>,
    seen: Vec<Option<SlotView>>,
    /// Orders to come, soonest last.
    pending: Vec<Timed>,
}

const PRESENT: u32 = u32::MAX;

fn up(bodies: &Bodies, m: ManId) -> bool {
    !matches!(
        bodies.posture[m.0 as usize],
        Posture::Down | Posture::Turned
    )
}

impl BodyFrames {
    fn grow(&mut self, men: usize) {
        if self.place.len() < men {
            self.place.resize(men, Place::Adrift);
            self.gone_since.resize(men, PRESENT);
            self.seen.resize(men, None);
        }
    }

    /// Adds a body. Bodies must be added in the order of their ids, starting at zero. The
    /// leader and the standard-bearer are found by their roles.
    pub fn add(&mut self, l: Layout, bodies: &Bodies) {
        assert_eq!(l.body.0 as usize, self.frames.len(), "bodies out of order");
        self.grow(bodies.len());
        let frame = self.frames.len() as u32;
        let grid = l.files * l.ranks;
        let man = |k: u32| ManId(l.first.0 + k);
        let mut files = vec![Vec::with_capacity(l.ranks as usize); l.files as usize];
        let (mut leader, mut carrier) = (None, None);
        for k in 0..grid {
            let (rank, file) = ((k / l.files) as u16, (k % l.files) as u16);
            let m = man(k);
            files[file as usize].push(m);
            self.place[m.0 as usize] = Place::File { frame, file, rank };
            match bodies.role[m.0 as usize] {
                Role::Leader => leader = Some((m, file)),
                Role::Carrier => carrier = Some(m),
                _ => {}
            }
        }
        let closer = l.closer.then(|| {
            let m = man(grid);
            self.place[m.0 as usize] = Place::Closer { frame };
            m
        });
        let carrier = carrier.expect("a body needs a standard-bearer");
        self.frames.push(Frame {
            body: l.body,
            side: bodies.side[l.first.0 as usize],
            origin: l.centre,
            facing: l.facing,
            basis: Basis::of(l.facing),
            file_mm: l.file_mm,
            rank_mm: l.rank_mm,
            depth: l.ranks,
            files,
            closer,
            standard: Standard::Carried(carrier),
            standard_tick: 0,
            leader: leader.map(|(m, _)| m),
            leader_file: leader.map_or(0, |(_, f)| f),
            leader_fell: None,
            order: Order::Hold,
            order_tick: 0,
            lead: Vec2::ZERO,
            adrift: Vec::new(),
            tie: l.tie,
        });
    }

    /// Gives a body an order now.
    pub fn order(&mut self, body: BodyId, order: Order, tick: u32) {
        let f = &mut self.frames[body.0 as usize];
        f.order = order;
        f.order_tick = tick;
    }

    /// Gives a body an order at a later tick. Orders for the same tick apply in the order given.
    pub fn order_at(&mut self, body: BodyId, order: Order, tick: u32) {
        // Kept soonest last, so that the next order is a pop.
        let at = self.pending.partition_point(|t| t.tick > tick);
        self.pending.insert(at, Timed { tick, body, order });
    }

    fn renumber(place: &mut [Place], frame: u32, file: u16, men: &[ManId]) {
        for (rank, m) in men.iter().enumerate() {
            place[m.0 as usize] = Place::File {
                frame,
                file,
                rank: rank as u16,
            };
        }
    }

    /// The standard falls with its bearer and lies until a man of the body, standing in his
    /// place beside it, lifts it.
    fn standard(&mut self, fi: usize, bodies: &mut Bodies, tick: u32, out: &mut Vec<Event>) {
        let f = &mut self.frames[fi];
        match f.standard {
            Standard::Carried(c) if bodies.posture[c.0 as usize] == Posture::Down => {
                let at = bodies.pos[c.0 as usize];
                f.standard = Standard::Down(at);
                f.standard_tick = tick;
                out.push(Event {
                    tick,
                    man: c,
                    pos: at,
                    kind: EventKind::StandardDown { body: f.body },
                    causes: [Cause::None; 3],
                });
            }
            Standard::Down(at) => {
                let Some(wait) = f.tie.lift_after else {
                    return;
                };
                if tick < f.standard_tick + wait {
                    return;
                }
                let reach = f.tie.lift_reach_mm as i64;
                let lifter = f
                    .files
                    .iter()
                    .flatten()
                    .chain(f.closer.iter())
                    .copied()
                    .filter(|&m| {
                        let i = m.0 as usize;
                        up(bodies, m)
                            && bodies.wound[i] < Wound::Disabled
                            && bodies.in_place[i]
                            && bodies.pos[i].dist_sq(at) <= reach * reach
                    })
                    .min_by_key(|&m| (bodies.pos[m.0 as usize].dist_sq(at), m));
                if let Some(m) = lifter {
                    let i = m.0 as usize;
                    f.standard = Standard::Carried(m);
                    f.standard_tick = tick;
                    if matches!(bodies.role[i], Role::Soldier | Role::FileLeader) {
                        bodies.role[i] = Role::Carrier;
                    }
                    out.push(Event {
                        tick,
                        man: m,
                        pos: bodies.pos[i],
                        kind: EventKind::StandardUp { body: f.body },
                        causes: [Cause::None; 3],
                    });
                }
            }
            Standard::Carried(_) => {}
        }
    }

    /// Whether the closer is about to take a fallen leader's place, which keeps the leader's
    /// slot open for him.
    fn succession_pending(f: &Frame, bodies: &Bodies) -> bool {
        f.tie.succeed_after.is_some()
            && f.leader
                .is_some_and(|l| bodies.posture[l.0 as usize] == Posture::Down)
            && f.closer.is_some_and(|c| up(bodies, c))
    }

    /// When a leader falls, the closer takes his place and his slot. A body with no such rule
    /// keeps a fallen leader.
    fn succession(&mut self, fi: usize, bodies: &mut Bodies, tick: u32) {
        let f = &mut self.frames[fi];
        let Some(old) = f.leader else { return };
        if bodies.posture[old.0 as usize] != Posture::Down {
            f.leader_fell = None;
            return;
        }
        let fell = *f.leader_fell.get_or_insert(tick);
        if !Self::succession_pending(f, bodies) {
            return;
        }
        let (Some(wait), Some(new)) = (f.tie.succeed_after, f.closer) else {
            return;
        };
        if tick < fell + wait {
            return;
        }
        let file = f.leader_file;
        let men = &mut f.files[file as usize];
        match men.iter().position(|&m| m == old) {
            Some(i) => men[i] = new,
            None => men.insert(0, new),
        }
        bodies.role[new.0 as usize] = Role::Leader;
        f.leader = Some(new);
        f.leader_fell = None;
        f.closer = None;
        self.place[old.0 as usize] = Place::Adrift;
        Self::renumber(
            &mut self.place,
            fi as u32,
            file,
            &self.frames[fi].files[file as usize],
        );
    }

    /// Files close over men who have been down or in flight for long enough. A man who fled
    /// and lives is set adrift, and may fall in again.
    fn close_files(&mut self, fi: usize, bodies: &Bodies, tick: u32) {
        let f = &mut self.frames[fi];
        let keep_leader = Self::succession_pending(f, bodies);
        let leader = f.leader;
        let gone_since = &mut self.gone_since;
        let mut note = |m: ManId| -> u32 {
            let g = &mut gone_since[m.0 as usize];
            if up(bodies, m) {
                *g = PRESENT;
            } else if *g == PRESENT {
                *g = tick;
            }
            *g
        };
        let Some(wait) = f.tie.close_after else {
            for &m in f.files.iter().flatten().chain(f.closer.iter()) {
                note(m);
            }
            return;
        };
        let due = |since: u32| since != PRESENT && tick >= since + wait;
        let mut dropped = Vec::new();
        for (file, men) in f.files.iter_mut().enumerate() {
            let before = men.len();
            men.retain(|&m| {
                let since = note(m);
                let hold = keep_leader && Some(m) == leader;
                if due(since) && !hold {
                    dropped.push(m);
                    false
                } else {
                    true
                }
            });
            if men.len() != before {
                Self::renumber(&mut self.place, fi as u32, file as u16, men);
            }
        }
        if let Some(c) = f.closer
            && due(note(c))
        {
            f.closer = None;
            dropped.push(c);
        }
        for m in dropped {
            self.place[m.0 as usize] = Place::Adrift;
            if bodies.posture[m.0 as usize] != Posture::Down {
                f.adrift.push(m);
            }
        }
    }

    /// A man of the body who has no slot and stands again falls in at the rear of the shortest
    /// file.
    fn fall_in(&mut self, fi: usize, bodies: &Bodies) {
        let f = &mut self.frames[fi];
        let mut i = 0;
        while i < f.adrift.len() {
            let m = f.adrift[i];
            if bodies.posture[m.0 as usize] == Posture::Down {
                f.adrift.remove(i);
            } else if up(bodies, m) {
                f.adrift.remove(i);
                let file = (0..f.files.len())
                    .min_by_key(|&k| (f.files[k].len(), k))
                    .expect("a body has files");
                f.files[file].push(m);
                self.gone_since[m.0 as usize] = PRESENT;
                self.place[m.0 as usize] = Place::File {
                    frame: fi as u32,
                    file: file as u16,
                    rank: (f.files[file].len() - 1) as u16,
                };
            } else {
                i += 1;
            }
        }
    }

    /// The gap from the front rank to the nearest enemy who stands facing the body, within its
    /// frontage. A scan of every man: at full scale this wants a grid query.
    fn gap_ahead(f: &Frame, bodies: &Bodies) -> Option<i64> {
        let front = ((f.depth as i32 - 1) * f.rank_mm / 2) as i64;
        let half_width = (f.files.len() as i64 + 1) * f.file_mm as i64 / 2;
        (0..bodies.len())
            .filter(|&i| {
                bodies.side[i] != f.side
                    && matches!(
                        bodies.posture[i],
                        Posture::Steady | Posture::Forward | Posture::Back
                    )
            })
            .filter_map(|i| {
                let (along, left) = f.basis.split(bodies.pos[i] - f.origin);
                (left.abs() <= half_width && along > front - f.rank_mm as i64)
                    .then_some(along - front)
            })
            .min()
    }

    /// Anchors the frame on its standard-bearer and sets how far it leads. A standard that is
    /// down, or carried by a man in flight, anchors nothing: the frame stays where it was.
    fn anchor(&mut self, fi: usize, bodies: &Bodies) {
        let f = &mut self.frames[fi];
        f.lead = Vec2::ZERO;
        let Standard::Carried(c) = f.standard else {
            return;
        };
        let Some(offset) = f.offset_of(self.place[c.0 as usize]) else {
            return;
        };
        if !up(bodies, c) {
            return;
        }
        f.origin = bodies.pos[c.0 as usize] - offset;
        let halt_at = match f.order {
            Order::Hold => return,
            Order::Retire => {
                f.lead = Basis::scaled(f.basis.ahead, -f.tie.lead_mm);
                return;
            }
            Order::Advance => f.tie.standoff_mm,
            Order::Charge | Order::Pursue => f.tie.contact_mm,
        };
        if Self::gap_ahead(f, bodies).is_none_or(|gap| gap > halt_at as i64) {
            f.lead = Basis::scaled(f.basis.ahead, f.tie.lead_mm);
        }
    }

    /// Every slotted man's view of his slot, and whether he is in his place.
    fn look(&mut self, fi: usize, bodies: &mut Bodies) {
        let f = &self.frames[fi];
        let tol = f.tie.tolerance_mm as i64;
        let filled = |file: usize, rank: usize| {
            f.files
                .get(file)
                .and_then(|men| men.get(rank))
                .is_some_and(|&m| up(bodies, m))
        };
        let mut views = Vec::new();
        for (file, men) in f.files.iter().enumerate() {
            for (rank, &m) in men.iter().enumerate() {
                let mut slots = 0;
                let mut neighbours = 0;
                let mut side = |bit: u8, exists: bool, file: usize, rank: usize| {
                    if exists {
                        slots |= bit;
                        if filled(file, rank) {
                            neighbours |= bit;
                        }
                    }
                };
                side(nb::LEFT, file > 0, file.wrapping_sub(1), rank);
                side(nb::RIGHT, file + 1 < f.files.len(), file + 1, rank);
                side(nb::FRONT, rank > 0, file, rank.wrapping_sub(1));
                side(nb::BEHIND, rank + 1 < men.len(), file, rank + 1);
                let at = f.origin + f.offset(file as u16, rank as u16);
                views.push((m, at, slots, neighbours));
            }
        }
        if let Some(c) = f.closer {
            views.push((c, f.origin + f.closer_offset(), 0, 0));
        }
        for (m, at, slots, neighbours) in views {
            let i = m.0 as usize;
            let in_place = bodies.pos[i].dist_sq(at) <= tol * tol;
            bodies.in_place[i] = in_place;
            self.seen[i] = Some(SlotView {
                target: at + f.lead,
                facing: f.facing,
                in_place,
                slots,
                neighbours,
            });
        }
    }
}

impl FrameQuery for BodyFrames {
    fn slot(&self, man: ManId) -> Option<SlotView> {
        self.seen[man.0 as usize]
    }

    fn body(&self, body: BodyId) -> Option<BodyState> {
        self.frames.get(body.0 as usize).map(|f| BodyState {
            standard: f.standard,
            leader: f.leader,
            order: f.order,
            order_tick: f.order_tick,
        })
    }
}

impl Frames for BodyFrames {
    fn update(&mut self, bodies: &mut Bodies, tick: u32, out: &mut Vec<Event>) {
        self.grow(bodies.len());
        while self.pending.last().is_some_and(|t| t.tick <= tick) {
            let t = self.pending.pop().expect("checked");
            self.order(t.body, t.order, tick);
        }
        for fi in 0..self.frames.len() {
            self.standard(fi, bodies, tick, out);
            self.succession(fi, bodies, tick);
            self.close_files(fi, bodies, tick);
            self.fall_in(fi, bodies);
            self.anchor(fi, bodies);
        }
        for (i, p) in self.place.iter().enumerate() {
            if *p == Place::Adrift {
                self.seen[i] = None;
                bodies.in_place[i] = false;
            }
        }
        for fi in 0..self.frames.len() {
            self.look(fi, bodies);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observe::Observed;

    /// Places a body exactly as the harness does. A body with a rule of succession has a closer.
    fn body(
        b: &mut Bodies,
        frames: &mut BodyFrames,
        side: Side,
        centre: Vec2,
        facing: Turn,
        (files, ranks): (u32, u32),
        tie: Tie,
    ) {
        let (file_mm, rank_mm) = (1_000, 2_000);
        let id = BodyId(frames.frames.len() as u32);
        let closer = tie.succeed_after.is_some();
        let left = Turn(facing.0.wrapping_add(Turn::QUARTER));
        let at = |rank2: i32, file2: i32| {
            centre + facing.unit(rank2 * rank_mm / 2) + left.unit(file2 * file_mm / 2)
        };
        let first = ManId(b.len() as u32);
        let mut put = |pos: Vec2, role: Role| {
            b.push(Observed {
                pos,
                facing,
                side,
                role,
                body: Some(id),
                in_place: true,
                ..Default::default()
            });
        };
        for r in 0..ranks {
            for f in 0..files {
                let role = if r == 0 && f == files - 1 {
                    Role::Leader
                } else if r == ranks.min(2) - 1 && f == files / 2 {
                    Role::Carrier
                } else if r == 0 {
                    Role::FileLeader
                } else {
                    Role::Soldier
                };
                put(
                    at(
                        ranks as i32 - 1 - 2 * r as i32,
                        files as i32 - 1 - 2 * f as i32,
                    ),
                    role,
                );
            }
        }
        if closer {
            put(at(-(ranks as i32) - 1, 0), Role::Closer);
        }
        frames.add(
            Layout {
                body: id,
                centre,
                facing,
                files,
                ranks,
                file_mm,
                rank_mm,
                first,
                closer,
                tie,
            },
            b,
        );
    }

    /// A 3 x 3 body facing +x, with a closer: men 0-8, the closer 9. The leader is man 2
    /// (front right), the standard-bearer man 4 (second rank, centre).
    fn century() -> (Bodies, BodyFrames) {
        let mut b = Bodies::default();
        let mut f = BodyFrames::default();
        body(
            &mut b,
            &mut f,
            Side::A,
            Vec2::ZERO,
            Turn(0),
            (3, 3),
            Tie::DRILLED,
        );
        (b, f)
    }

    fn step(f: &mut BodyFrames, b: &mut Bodies, tick: u32) -> Vec<Event> {
        let mut out = Vec::new();
        f.update(b, tick, &mut out);
        out
    }

    #[test]
    fn slots_are_where_the_men_were_placed() {
        for facing in [0u16, 16_384, 32_768, 49_152, 7_000, 40_123] {
            let mut b = Bodies::default();
            let mut f = BodyFrames::default();
            body(
                &mut b,
                &mut f,
                Side::A,
                Vec2::new(12_345, -6_789),
                Turn(facing),
                (4, 3),
                Tie::DRILLED,
            );
            step(&mut f, &mut b, 0);
            for i in 0..b.len() {
                let s = f.slot(ManId(i as u32)).expect("every man has a slot");
                assert_eq!(s.target, b.pos[i], "man {i}, facing {facing}");
                assert!(s.in_place && b.in_place[i]);
                assert_eq!(s.facing, Turn(facing));
            }
        }
    }

    #[test]
    fn edges_have_no_slots_and_the_middle_has_four_neighbours() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        // Man 4 is in the middle of the body.
        let mid = f.slot(ManId(4)).unwrap();
        assert_eq!(mid.slots, nb::LEFT | nb::RIGHT | nb::FRONT | nb::BEHIND);
        assert_eq!(mid.neighbours, mid.slots);
        // Man 0 is front left: nothing to his left or in front, and that is an edge, not a gap.
        let corner = f.slot(ManId(0)).unwrap();
        assert_eq!(corner.slots, nb::RIGHT | nb::BEHIND);
        assert_eq!(corner.neighbours, corner.slots);
        // The closer stands behind the rear rank's centre, on no file.
        let closer = f.slot(ManId(9)).unwrap();
        assert_eq!(closer.target, Vec2::new(-4_000, 0));
        assert_eq!(closer.slots, 0);
    }

    #[test]
    fn the_frame_goes_where_the_standard_goes() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        let before = f.slot(ManId(0)).unwrap().target;
        // The standard-bearer is pushed back half a metre. Every slot follows him, so the
        // others are now out of their places and he is not.
        b.pos[4] += Vec2::new(-500, 0);
        step(&mut f, &mut b, 5);
        assert_eq!(
            f.slot(ManId(0)).unwrap().target,
            before + Vec2::new(-500, 0)
        );
        assert!(b.in_place[4]);
        assert!(
            b.in_place[0],
            "half a metre is inside a drilled man's tolerance"
        );
        b.pos[4] += Vec2::new(-500, 0);
        step(&mut f, &mut b, 10);
        assert!(b.in_place[4] && !b.in_place[0]);
    }

    #[test]
    fn an_advance_leads_the_men_and_halts_at_the_standoff() {
        let (mut b, mut f) = century();
        // An enemy faces the body 10 m ahead of its front rank, which stands at x = +2000.
        let foe = b.push(Observed {
            pos: Vec2::new(12_000, 0),
            side: Side::B,
            facing: Turn(Turn::HALF),
            ..Default::default()
        });
        f.order_at(BodyId(0), Order::Advance, 5);
        step(&mut f, &mut b, 0);
        assert_eq!(f.slot(ManId(0)).unwrap().target, b.pos[0]);
        assert_eq!(f.body(BodyId(0)).unwrap().order, Order::Hold);
        step(&mut f, &mut b, 5);
        let st = f.body(BodyId(0)).unwrap();
        assert_eq!((st.order, st.order_tick), (Order::Advance, 5));
        // Every man's target is a lead ahead of his slot, and he is still in his place.
        assert_eq!(
            f.slot(ManId(0)).unwrap().target,
            b.pos[0] + Vec2::new(1_500, 0)
        );
        assert!(b.in_place[0]);
        // Walk the body forward in step until the frame stops leading.
        let mut tick = 5;
        while f.slot(ManId(0)).unwrap().target != b.pos[0] {
            for i in 0..10 {
                b.pos[i] += Vec2::new(300, 0);
            }
            tick += 5;
            step(&mut f, &mut b, tick);
            assert!(tick < 400, "the advance never halted");
        }
        let gap = b.pos[foe.0 as usize].x - b.pos[0].x;
        assert!(
            (3_700..=4_000).contains(&gap),
            "halted {gap} mm from the enemy"
        );
        // A charge goes on to striking distance.
        f.order(BodyId(0), Order::Charge, tick);
        step(&mut f, &mut b, tick + 5);
        assert_ne!(f.slot(ManId(0)).unwrap().target, b.pos[0]);
        // An enemy who has turned holds no one off.
        f.order(BodyId(0), Order::Advance, tick);
        b.posture[foe.0 as usize] = Posture::Turned;
        step(&mut f, &mut b, tick + 10);
        assert_ne!(f.slot(ManId(0)).unwrap().target, b.pos[0]);
        // A retirement leads backwards, facing unchanged.
        f.order(BodyId(0), Order::Retire, tick);
        step(&mut f, &mut b, tick + 15);
        let s = f.slot(ManId(0)).unwrap();
        assert_eq!(s.target, b.pos[0] - Vec2::new(1_500, 0));
        assert_eq!(s.facing, Turn(0));
    }

    #[test]
    fn a_file_closes_over_a_fallen_man_and_the_gap_goes_to_the_rear() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        // Man 3 is second rank, left file; man 6 stands behind him, man 4 beside him.
        b.posture[3] = Posture::Down;
        step(&mut f, &mut b, 5);
        let beside = f.slot(ManId(4)).unwrap();
        assert_eq!(beside.slots & nb::LEFT, nb::LEFT);
        assert_eq!(beside.neighbours & nb::LEFT, 0, "a gap on his left");
        let behind = f.slot(ManId(6)).unwrap();
        assert_eq!(behind.neighbours & nb::FRONT, 0);
        let old = behind.target;
        // After the closing time the man behind is given the fallen man's slot.
        step(&mut f, &mut b, 5 + 40);
        let moved = f.slot(ManId(6)).unwrap();
        assert_eq!(moved.target, old + Vec2::new(2_000, 0));
        assert!(!moved.in_place && !b.in_place[6]);
        assert_eq!(
            moved.slots & nb::BEHIND,
            0,
            "he is now the last of a short file"
        );
        assert_eq!(f.slot(ManId(3)), None);
        // The gap is now in the rear rank: man 7 has a slot on his left and no one in it.
        let rear = f.slot(ManId(7)).unwrap();
        assert_eq!(rear.slots & nb::LEFT, nb::LEFT);
        assert_eq!(rear.neighbours & nb::LEFT, 0);
        // And man 4 has a neighbour on his left again.
        assert_eq!(f.slot(ManId(4)).unwrap().neighbours & nb::LEFT, nb::LEFT);
    }

    #[test]
    fn the_standard_falls_freezes_the_frame_and_is_lifted() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        f.order(BodyId(0), Order::Advance, 0);
        b.posture[4] = Posture::Down;
        let ev = step(&mut f, &mut b, 5);
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].kind, EventKind::StandardDown { body: BodyId(0) });
        assert_eq!(ev[0].man, ManId(4));
        let st = f.body(BodyId(0)).unwrap();
        assert_eq!(st.standard, Standard::Down(b.pos[4]));
        // No standard, no advance.
        assert_eq!(f.slot(ManId(0)).unwrap().target, b.pos[0]);
        assert!(step(&mut f, &mut b, 10).is_empty());
        // After the lifting time the nearest man in his place takes it up. Men 3 and 5 stand
        // beside it, equally near; the lower id breaks the tie.
        let ev = step(&mut f, &mut b, 5 + 60);
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].kind, EventKind::StandardUp { body: BodyId(0) });
        assert_eq!(ev[0].man, ManId(3));
        assert_eq!(b.role[3], Role::Carrier);
        assert_eq!(
            f.body(BodyId(0)).unwrap().standard,
            Standard::Carried(ManId(3))
        );
        // The frame is anchored on him now, and the advance resumes.
        assert_eq!(
            f.slot(ManId(0)).unwrap().target,
            b.pos[0] + Vec2::new(1_500, 0)
        );
    }

    #[test]
    fn a_warband_does_not_lift_its_standard() {
        let mut b = Bodies::default();
        let mut f = BodyFrames::default();
        body(
            &mut b,
            &mut f,
            Side::B,
            Vec2::ZERO,
            Turn(0),
            (3, 3),
            Tie::LOOSE,
        );
        step(&mut f, &mut b, 0);
        b.posture[4] = Posture::Down;
        assert_eq!(step(&mut f, &mut b, 5).len(), 1);
        assert!(step(&mut f, &mut b, 4_000).is_empty());
        assert!(matches!(
            f.body(BodyId(0)).unwrap().standard,
            Standard::Down(_)
        ));
    }

    #[test]
    fn the_closer_takes_a_fallen_leaders_place() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        let leaders_slot = f.slot(ManId(2)).unwrap().target;
        b.posture[2] = Posture::Down;
        step(&mut f, &mut b, 5);
        assert_eq!(f.body(BodyId(0)).unwrap().leader, Some(ManId(2)));
        // The file does not close over the leader's slot while his successor is coming.
        step(&mut f, &mut b, 5 + 40);
        assert_eq!(f.slot(ManId(5)).unwrap().neighbours & nb::FRONT, 0);
        step(&mut f, &mut b, 5 + 100);
        assert_eq!(f.body(BodyId(0)).unwrap().leader, Some(ManId(9)));
        assert_eq!(b.role[9], Role::Leader);
        assert_eq!(f.slot(ManId(9)).unwrap().target, leaders_slot);
        assert!(!b.in_place[9], "he has the length of the file to walk");
        assert_eq!(f.slot(ManId(2)), None);
        // No one closes the rear any more.
        assert_eq!(f.frames[0].closer, None);
    }

    #[test]
    fn a_man_who_fled_loses_his_slot_and_falls_in_at_the_rear() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        // Man 0 is front left.
        b.posture[0] = Posture::Turned;
        step(&mut f, &mut b, 5);
        assert!(f.slot(ManId(0)).is_some(), "his slot waits a while");
        step(&mut f, &mut b, 5 + 40);
        assert_eq!(f.slot(ManId(0)), None);
        assert!(!b.in_place[0]);
        // The left file is now two deep, led by the man who stood behind him.
        assert_eq!(f.slot(ManId(3)).unwrap().slots & nb::FRONT, 0);
        // He rallies, and is given the rear of the shortest file: his own.
        b.posture[0] = Posture::Steady;
        step(&mut f, &mut b, 100);
        let s = f.slot(ManId(0)).unwrap();
        assert_eq!(s.target, Vec2::new(-2_000, 1_000));
        assert_eq!(s.slots & nb::BEHIND, 0);
        // A dead man is never set adrift.
        b.posture[1] = Posture::Down;
        step(&mut f, &mut b, 105);
        step(&mut f, &mut b, 105 + 40);
        assert!(f.frames[0].adrift.is_empty());
    }

    #[test]
    fn a_fleeing_standard_leaves_the_frame_behind() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        let slot = f.slot(ManId(0)).unwrap().target;
        b.posture[4] = Posture::Turned;
        b.pos[4] += Vec2::new(-9_000, 0);
        step(&mut f, &mut b, 5);
        assert_eq!(f.slot(ManId(0)).unwrap().target, slot);
        assert_eq!(
            f.body(BodyId(0)).unwrap().standard,
            Standard::Carried(ManId(4))
        );
    }
}
