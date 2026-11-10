//! Bodies of men and their frames: standards, slots, ties and drills (docs/rival/02 sec. 1-4).
//!
//! A body is the men who follow one standard, and its frame is never fitted to where the men
//! are. On the march the frame is wherever the standard-bearer stands,
//! `origin = his position - his slot's offset`, taken afresh at every man step: the body goes
//! where he goes. Halted, the frame stands where it halted, and he has a place in it like any
//! other man. It moves only when he is driven further from that place than the tie's tolerance,
//! and then it re-forms on him. So a pressed body braces on a fixed line and gives ground a step
//! at a time, and a body whose standard-bearer steps out follows its standard. Every other man
//! has a slot in the frame and walks to it.
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
    /// Whether another body is passing through this one's files, in a relief. Its closer then
    /// stands in the line of a file, so that every lane between the files is clear.
    lanes_open: bool,
}

impl Frame {
    fn offset(&self, file: u16, rank: u16) -> Vec2 {
        let ahead = (self.depth as i32 - 1 - 2 * rank as i32) * self.rank_mm / 2;
        let left = (self.files.len() as i32 - 1 - 2 * file as i32) * self.file_mm / 2;
        self.basis.at(ahead, left)
    }

    /// The closer's post: one rank behind the deepest file, at the body's centre. With an even
    /// number of files that is between two files, in a lane. While another body passes through
    /// this one he stands behind the file to the right of the centre, the standard's file.
    fn closer_offset(&self) -> Vec2 {
        let deepest = self.files.iter().map(Vec::len).max().unwrap_or(0) as i32;
        let ahead = (self.depth as i32 - 1 - 2 * deepest) * self.rank_mm / 2;
        let files = self.files.len() as i32;
        let left = if self.lanes_open {
            (files - 1 - 2 * (files / 2)) * self.file_mm / 2
        } else {
            0
        };
        self.basis.at(ahead, left)
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

/// A relief (docs/rival/02 sec. 4): in a lull, a tired body falls back through the fresh body
/// behind it, which comes up and takes the front. Each relief happens once.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Relief {
    tired: BodyId,
    fresh: BodyId,
    /// The first tick at which it may be ordered.
    not_before: u32,
    /// How long the tired body's front must have been quiet, in ticks.
    lull: u32,
    state: Relieving,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Relieving {
    /// Not yet ordered. The tired body's front has been quiet since the tick given, if it is.
    Waiting(Option<u32>),
    /// The fresh body is coming up through the tired one, from where its frame stood. The
    /// tired body stands until the fresh one has the front: were they to move at once, the
    /// enemy would follow the one into the ground the other had not yet reached.
    Coming(Vec2),
    /// The tired body is falling back, until its frame stands where the fresh body's stood.
    Going(Vec2),
    Done,
}

/// How far from a body's front or rear an enemy who has stepped forward still disturbs its
/// lull: what he covers at a run while two bodies pass through each other.
pub const LULL_DEPTH_MM: i64 = 20_000;

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
    reliefs: Vec<Relief>,
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
            lanes_open: false,
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

    /// Has `fresh` relieve `tired` in the first lull from `not_before` on. Once the front of
    /// `tired` has been quiet for `lull` ticks, `fresh` is ordered to advance. When it has come
    /// up through `tired` and has the front, `tired` is ordered to retire, and halts where
    /// `fresh` stood. Both bodies must have been added.
    pub fn relieve(&mut self, tired: BodyId, fresh: BodyId, not_before: u32, lull: u32) {
        assert!(tired != fresh, "a body cannot relieve itself");
        self.reliefs.push(Relief {
            tired,
            fresh,
            not_before,
            lull,
            state: Relieving::Waiting(None),
        });
    }

    /// Whether the body's standard is carried by a man on his feet and facing the enemy.
    fn formed(f: &Frame, bodies: &Bodies) -> bool {
        matches!(f.standard, Standard::Carried(c) if up(bodies, c))
    }

    /// Whether the body's front is quiet: it is formed, none of its men is forward, giving
    /// ground or in flight, and no enemy within its frontage and [`LULL_DEPTH_MM`] of its front
    /// or its rear has stepped forward. A scan of every man, as [`Self::gap_ahead`] is.
    fn quiet(f: &Frame, bodies: &Bodies) -> bool {
        let stirring = |m: &ManId| {
            matches!(
                bodies.posture[m.0 as usize],
                Posture::Forward | Posture::Back | Posture::Turned
            )
        };
        if !Self::formed(f, bodies) || f.files.iter().flatten().chain(&f.closer).any(stirring) {
            return false;
        }
        let front = ((f.depth as i32 - 1) * f.rank_mm / 2) as i64;
        let half_width = (f.files.len() as i64 + 1) * f.file_mm as i64 / 2;
        !(0..bodies.len()).any(|i| {
            let (along, left) = f.basis.split(bodies.pos[i] - f.origin);
            bodies.side[i] != f.side
                && bodies.posture[i] == Posture::Forward
                && left.abs() <= half_width
                && along.abs() <= front + LULL_DEPTH_MM
        })
    }

    fn open_lanes(&mut self, r: &Relief, open: bool) {
        for body in [r.tired, r.fresh] {
            self.frames[body.0 as usize].lanes_open = open;
        }
    }

    /// Orders each relief whose lull has come, sends each tired body back once the fresh one
    /// has the front, and halts it when it has fallen back far enough.
    fn relieve_in_lulls(&mut self, bodies: &Bodies, tick: u32) {
        for k in 0..self.reliefs.len() {
            let r = self.reliefs[k];
            let (tired, fresh) = (
                &self.frames[r.tired.0 as usize],
                &self.frames[r.fresh.0 as usize],
            );
            let state = match r.state {
                Relieving::Waiting(since) => {
                    let since = Self::quiet(tired, bodies).then(|| since.unwrap_or(tick));
                    let lull = since.is_some_and(|s| tick - s >= r.lull);
                    let ready = fresh.order == Order::Hold && Self::formed(fresh, bodies);
                    if lull && ready && tick >= r.not_before {
                        let from = fresh.origin;
                        self.order(r.fresh, Order::Advance, tick);
                        self.open_lanes(&r, true);
                        Relieving::Coming(from)
                    } else {
                        Relieving::Waiting(since)
                    }
                }
                Relieving::Coming(from) => {
                    // The fresh body has the front when its front rank has passed the tired
                    // body's, or when it has halted within a rank of it, by the rule every
                    // advance has: it did not lead at the last step.
                    let front = |f: &Frame| (f.depth as i32 - 1) * f.rank_mm / 2;
                    let its_front = fresh.origin + Basis::scaled(fresh.basis.ahead, front(fresh));
                    let short = front(tired) as i64 - tired.basis.split(its_front - tired.origin).0;
                    let halted = fresh.lead == Vec2::ZERO && Self::formed(fresh, bodies);
                    if short <= 0 || (halted && short <= tired.rank_mm as i64) {
                        self.order(r.tired, Order::Retire, tick);
                        Relieving::Going(from)
                    } else {
                        r.state
                    }
                }
                Relieving::Going(to) => {
                    if tired.basis.split(tired.origin - to).0 <= 0 {
                        self.order(r.tired, Order::Hold, tick);
                        self.open_lanes(&r, false);
                        Relieving::Done
                    } else {
                        r.state
                    }
                }
                Relieving::Done => r.state,
            };
            self.reliefs[k].state = state;
        }
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
    ///
    /// A frame that was led at the last step follows him. A halted frame stays put until he is
    /// out of his place in it. Re-anchoring a halted frame at every step would give a held line
    /// no fixed point: its slots would follow every shove he took.
    fn anchor(&mut self, fi: usize, bodies: &Bodies) {
        let f = &mut self.frames[fi];
        let on_the_march = f.lead != Vec2::ZERO;
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
        let on_him = bodies.pos[c.0 as usize] - offset;
        let tol = f.tie.tolerance_mm as i64;
        if on_the_march || on_him.dist_sq(f.origin) > tol * tol {
            f.origin = on_him;
        }
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
        self.relieve_in_lulls(bodies, tick);
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
    fn a_halted_frame_stands_until_its_standard_is_driven_off() {
        let (mut b, mut f) = century();
        step(&mut f, &mut b, 0);
        let before = f.slot(ManId(0)).unwrap().target;
        let his = f.slot(ManId(4)).unwrap().target;
        // The standard-bearer is pushed back half a metre, inside a drilled man's tolerance.
        // The frame does not move: his slot is where it was, and he is pulled back to it.
        b.pos[4] += Vec2::new(-500, 0);
        step(&mut f, &mut b, 5);
        assert_eq!(f.slot(ManId(0)).unwrap().target, before);
        assert_eq!(f.slot(ManId(4)).unwrap().target, his);
        assert!(b.in_place[4] && b.in_place[0]);
        // Driven a metre from his place, he takes the frame with him: the body has given a
        // metre of ground, and the men who were not pushed are out of their places.
        b.pos[4] += Vec2::new(-500, 0);
        step(&mut f, &mut b, 10);
        assert_eq!(
            f.slot(ManId(0)).unwrap().target,
            before + Vec2::new(-1_000, 0)
        );
        assert_eq!(f.slot(ManId(4)).unwrap().target, b.pos[4]);
        assert!(b.in_place[4] && !b.in_place[0]);
        // And it stands again where he now is.
        b.pos[4] += Vec2::new(300, 0);
        step(&mut f, &mut b, 15);
        assert_eq!(
            f.slot(ManId(0)).unwrap().target,
            before + Vec2::new(-1_000, 0)
        );
    }

    #[test]
    fn a_frame_on_the_march_goes_where_the_standard_goes() {
        let (mut b, mut f) = century();
        f.order(BodyId(0), Order::Advance, 0);
        step(&mut f, &mut b, 0);
        let lead = Vec2::new(1_500, 0);
        assert_eq!(f.slot(ManId(0)).unwrap().target, b.pos[0] + lead);
        // Only the standard-bearer moves, and by less than the tolerance. Every slot follows.
        b.pos[4] += Vec2::new(300, 0);
        step(&mut f, &mut b, 5);
        assert_eq!(
            f.slot(ManId(0)).unwrap().target,
            b.pos[0] + lead + Vec2::new(300, 0)
        );
        // Halted by an order, the frame stands where he stood at the halt.
        f.order(BodyId(0), Order::Hold, 5);
        b.pos[4] += Vec2::new(300, 0);
        step(&mut f, &mut b, 10);
        let halted = b.pos[0] + Vec2::new(600, 0);
        assert_eq!(f.slot(ManId(0)).unwrap().target, halted);
        b.pos[4] += Vec2::new(300, 0);
        step(&mut f, &mut b, 15);
        assert_eq!(f.slot(ManId(0)).unwrap().target, halted);
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

    /// The century of [`century`] at the front, a second like it 12 m behind and half a file
    /// to its left (men 10-18, closer 19, standard-bearer 14), and one enemy standing 4 m from
    /// the front rank, facing it (man 20). The second relieves the first from tick 100 on,
    /// after a lull of 40 ticks.
    fn two_lines() -> (Bodies, BodyFrames) {
        let (mut b, mut f) = century();
        let behind = Vec2::new(-12_000, 500);
        body(
            &mut b,
            &mut f,
            Side::A,
            behind,
            Turn(0),
            (3, 3),
            Tie::DRILLED,
        );
        foe(&mut b, 6_000, 0, Posture::Steady);
        f.relieve(BodyId(0), BodyId(1), 100, 40);
        (b, f)
    }

    fn foe(b: &mut Bodies, x: i32, y: i32, posture: Posture) -> usize {
        let m = b.push(Observed {
            pos: Vec2::new(x, y),
            side: Side::B,
            facing: Turn(Turn::HALF),
            posture,
            ..Default::default()
        });
        m.0 as usize
    }

    fn orders(f: &BodyFrames) -> [(Order, u32); 2] {
        [0, 1].map(|i| {
            let st = f.body(BodyId(i)).unwrap();
            (st.order, st.order_tick)
        })
    }

    /// Steps the frames at every man step from `from` to `to`, and gives the tick at which
    /// the relief was ordered, if it was: the fresh body is told to come up.
    fn relieved_at(f: &mut BodyFrames, b: &mut Bodies, from: u32, to: u32) -> Option<u32> {
        (from..=to).step_by(5).find(|&t| {
            step(f, b, t);
            orders(f)[1].0 == Order::Advance
        })
    }

    #[test]
    fn a_relief_waits_for_its_time_and_then_for_a_lull() {
        // Quiet from the start: ordered at the first tick it may be, and to both bodies.
        let (mut b, mut f) = two_lines();
        assert_eq!(relieved_at(&mut f, &mut b, 0, 400), Some(100));
        // The fresh body is ordered up. The tired one stands until it has come.
        assert_eq!(orders(&f), [(Order::Hold, 0), (Order::Advance, 100)]);

        // A man of the tired body who is forward or giving ground is no lull, and the lull is
        // counted from when he stands again.
        for posture in [Posture::Forward, Posture::Back] {
            let (mut b, mut f) = two_lines();
            b.posture[1] = posture;
            assert_eq!(relieved_at(&mut f, &mut b, 0, 200), None, "{posture:?}");
            b.posture[1] = Posture::Steady;
            assert_eq!(
                relieved_at(&mut f, &mut b, 205, 400),
                Some(245),
                "{posture:?}"
            );
        }
        // A man in flight is no lull either, until his file has closed over him 40 ticks
        // later: then he is no longer of the body, and its lull begins.
        let (mut b, mut f) = two_lines();
        assert_eq!(relieved_at(&mut f, &mut b, 0, 75), None);
        b.posture[1] = Posture::Turned;
        assert_eq!(relieved_at(&mut f, &mut b, 80, 160), None);
        assert_eq!(relieved_at(&mut f, &mut b, 165, 400), Some(165));
        // A man down is no one's business, and neither is what the fresh body's men do.
        let (mut b, mut f) = two_lines();
        b.posture[1] = Posture::Down;
        b.posture[11] = Posture::Back;
        assert_eq!(relieved_at(&mut f, &mut b, 0, 400), Some(100));
    }

    #[test]
    fn an_enemy_who_has_stepped_forward_before_the_body_breaks_its_lull() {
        // The front rank stands at x = +2,000 and the rear at -2,000. The outer files stand
        // at y = +1,000 and -1,000, and the frontage has a file's width to spare on each side.
        let disturbs = |x: i32, y: i32, posture: Posture| {
            let (mut b, mut f) = two_lines();
            foe(&mut b, x, y, posture);
            relieved_at(&mut f, &mut b, 0, 200).is_none()
        };
        assert!(disturbs(5_000, 0, Posture::Forward));
        assert!(!disturbs(5_000, 0, Posture::Steady));
        assert!(!disturbs(5_000, 0, Posture::Back));
        // Out to 20 m from the front rank, and no further.
        assert!(disturbs(22_000, 0, Posture::Forward));
        assert!(!disturbs(22_100, 0, Posture::Forward));
        // Within the frontage, and not beside it.
        assert!(disturbs(5_000, 2_000, Posture::Forward));
        assert!(!disturbs(5_000, 2_100, Posture::Forward));
        assert!(disturbs(5_000, -2_000, Posture::Forward));
        // Behind the body as well as before it: the fresh body comes up through there.
        assert!(disturbs(-8_000, 0, Posture::Forward));
        assert!(!disturbs(-22_100, 0, Posture::Forward));

        // When he stands again the lull begins.
        let (mut b, mut f) = two_lines();
        let him = foe(&mut b, 5_000, 0, Posture::Forward);
        assert_eq!(relieved_at(&mut f, &mut b, 0, 300), None);
        b.posture[him] = Posture::Steady;
        assert_eq!(relieved_at(&mut f, &mut b, 305, 500), Some(345));
    }

    #[test]
    fn no_relief_while_the_standard_is_down_or_the_fresh_body_is_not_standing_ready() {
        // The tired body's standard falls at tick 90. No relief until it is lifted, and then
        // a lull of 40 ticks more, counted from the first step that finds it carried.
        let (mut b, mut f) = two_lines();
        assert_eq!(relieved_at(&mut f, &mut b, 0, 85), None);
        b.posture[4] = Posture::Down;
        b.wound[4] = Wound::Dead;
        let mut lifted = None;
        let mut at = None;
        for t in (90..600).step_by(5) {
            let events = step(&mut f, &mut b, t);
            if events
                .iter()
                .any(|e| matches!(e.kind, EventKind::StandardUp { .. }))
            {
                lifted = Some(t);
            }
            if orders(&f)[1].0 == Order::Advance {
                at = Some(t);
                break;
            }
        }
        let lifted = lifted.expect("a drilled body lifts its standard");
        assert!(lifted > 100, "lifted at {lifted}");
        assert_eq!(at, Some(lifted + 45));

        // The fresh body has been sent somewhere else: it relieves no one until it is halted.
        let (mut b, mut f) = two_lines();
        f.order(BodyId(1), Order::Retire, 0);
        assert_eq!(relieved_at(&mut f, &mut b, 0, 300), None);
        f.order(BodyId(1), Order::Hold, 300);
        assert_eq!(relieved_at(&mut f, &mut b, 305, 400), Some(305));
        // Its standard-bearer has run: it is no body to relieve with.
        let (mut b, mut f) = two_lines();
        b.posture[14] = Posture::Turned;
        assert_eq!(relieved_at(&mut f, &mut b, 0, 300), None);
    }

    #[test]
    fn the_fresh_body_comes_up_and_then_the_tired_one_falls_back_to_where_it_stood() {
        let (mut b, mut f) = two_lines();
        assert_eq!(relieved_at(&mut f, &mut b, 0, 400), Some(100));
        let front = b.pos[0].x;
        // Walk both bodies in step, each man toward his slot's target, 300 mm a step.
        let mut tick = 100;
        let (mut retired, mut halted) = (None, None);
        while tick < 1_000 {
            for i in 0..20 {
                let to = f.slot(ManId(i as u32)).unwrap().target;
                b.pos[i].x += (to.x - b.pos[i].x).clamp(-300, 300);
            }
            tick += 5;
            step(&mut f, &mut b, tick);
            let tired = orders(&f)[0].0;
            if retired.is_none() && tired == Order::Retire {
                retired = Some(tick);
                // Not before the fresh body's front rank is up with its own and has halted
                // there: the front is never left open. The tired body has not moved.
                let behind = b.pos[0].x - b.pos[10].x;
                assert!((-300..=300).contains(&behind), "{behind} mm behind");
                assert_eq!(b.pos[0].x, front);
                // And both closers have stepped off the centre line. With three files the
                // centre is a file's line already, so nothing moves here; with four it is a
                // lane (the next test).
                assert_eq!(f.slot(ManId(9)).unwrap().target.y, 0);
            }
            if retired.is_some() && halted.is_none() && tired == Order::Hold {
                halted = Some(tick);
            }
        }
        // The fresh body had 12 m to come, at 300 mm a step of five ticks, and is seen to
        // have halted at the step after it does; the tired one then has 12 m to go.
        let retired = retired.expect("the tired body was never ordered back");
        assert!((300..=310).contains(&retired), "ordered back at {retired}");
        let halted = halted.expect("the tired body never halted");
        assert_eq!(orders(&f)[0], (Order::Hold, halted));
        assert!((500..=515).contains(&halted), "halted at {halted}");
        let back = front - b.pos[0].x;
        assert!((12_000..12_600).contains(&back), "it went back {back} mm");
        // It kept its own files: it did not step sideways onto the fresh body's.
        assert_eq!(b.pos[0].y, 1_000);
        // The fresh body stands at the standoff from the enemy, where the tired one stood,
        // and is still under its order to advance: if the enemy gives ground it follows.
        let gap = b.pos[20].x - b.pos[10].x;
        assert!((3_700..=4_000).contains(&gap), "{gap} mm from the enemy");
        assert_eq!(orders(&f)[1], (Order::Advance, 100));
        // Every man of both is in his place, and the relief is not ordered a second time.
        for i in 0..20 {
            assert!(b.in_place[i], "man {i} at {:?}", b.pos[i]);
        }
    }

    #[test]
    fn with_no_enemy_to_halt_it_the_fresh_body_passes_and_the_tired_one_goes_back() {
        let (mut b, mut f) = two_lines();
        // The enemy is gone, so nothing halts the fresh body's advance.
        b.posture[20] = Posture::Down;
        assert_eq!(relieved_at(&mut f, &mut b, 0, 400), Some(100));
        let mut tick = 100;
        let mut retired = None;
        while tick < 700 {
            for i in 0..20 {
                let to = f.slot(ManId(i as u32)).unwrap().target;
                b.pos[i].x += (to.x - b.pos[i].x).clamp(-300, 300);
            }
            tick += 5;
            step(&mut f, &mut b, tick);
            if retired.is_none() && orders(&f)[0].0 == Order::Retire {
                // The tired body goes back once the fresh one's front rank has passed its
                // own, and not before.
                retired = Some(tick);
                let ahead = b.pos[10].x - b.pos[0].x;
                assert!((0..=600).contains(&ahead), "{ahead} mm ahead");
            }
        }
        assert!(retired.is_some(), "the tired body was never ordered back");
        assert_eq!(orders(&f)[0].0, Order::Hold);
        assert!(
            b.pos[10].x > 20_000,
            "the fresh body halted at {}",
            b.pos[10].x
        );
    }

    #[test]
    fn while_bodies_pass_through_each_other_their_closers_clear_the_lanes() {
        // Two bodies of four files: the centre of each is a lane, and its closer stands in it.
        let mut b = Bodies::default();
        let mut f = BodyFrames::default();
        let behind = Vec2::new(-12_000, 500);
        body(
            &mut b,
            &mut f,
            Side::A,
            Vec2::ZERO,
            Turn(0),
            (4, 2),
            Tie::DRILLED,
        );
        body(
            &mut b,
            &mut f,
            Side::A,
            behind,
            Turn(0),
            (4, 2),
            Tie::DRILLED,
        );
        foe(&mut b, 6_000, 0, Posture::Steady);
        f.relieve(BodyId(0), BodyId(1), 100, 40);
        // Men 0-7 and the closer 8; men 9-16 and the closer 17. Files stand at y = +1500,
        // +500, -500 and -1500 in the first body, and half a file to the left in the second.
        let closers = |f: &BodyFrames| [8, 17].map(|m| f.slot(ManId(m)).unwrap().target.y);
        assert_eq!(relieved_at(&mut f, &mut b, 0, 95), None);
        assert_eq!(closers(&f), [0, 500]);
        // At the order each steps behind the file to the right of his body's centre: the
        // first closer out of the lane the second body's file comes up, and the second out
        // of the line of the first body's file.
        assert_eq!(relieved_at(&mut f, &mut b, 100, 100), Some(100));
        assert_eq!(closers(&f), [-500, 0]);
        // When the relief is done they go back to the centre.
        let mut tick = 100;
        while orders(&f)[0] == (Order::Hold, 0) || orders(&f)[0].0 == Order::Retire {
            for i in 0..18 {
                let to = f.slot(ManId(i as u32)).unwrap().target;
                b.pos[i].x += (to.x - b.pos[i].x).clamp(-300, 300);
                b.pos[i].y += (to.y - b.pos[i].y).clamp(-300, 300);
            }
            tick += 5;
            step(&mut f, &mut b, tick);
            assert!(tick < 1_000, "the relief never ended");
        }
        assert_eq!(closers(&f), [0, 500]);
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
