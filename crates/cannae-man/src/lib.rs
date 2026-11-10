//! The rival's man (docs/rival/01 and 07): five postures, and hazards read off what he sees.
//!
//! Two sums decide everything. **Alarm** is what he sees that pushes him toward flight, less
//! what he sees that holds him, less his nerve. **Urge** is what sends him forward at the
//! enemy. Each way out of a posture is one hazard on one curve ([`curve::rate_q16`]) of one of
//! the two sums against a threshold.
//!
//! What he carries from one moment to the next is his wounds and posture (which everyone can
//! see), his breath, and his memory: whatever his rung of the ladder gives him
//! ([`memory::Memory`]). At rungs 0 to 2 that is the marks ([`Marks`]); nothing else, no fear
//! level, no morale. Other men never read any of it.
//!
//! Dormancy is exact. A steady man with no enemy in his window, no shock, no one of his side
//! wavering in view and no order to charge has no hazards at all, by a gate and not by a small
//! number. Breath and every memory are stored with the tick they were last written and read in
//! closed form, so skipping a man can never change a bit.

pub mod curve;
pub mod memory;
pub mod params;

pub use memory::{Marks, Memory, NO_PARTS, Parts, Sums};
pub use params::{Params, Rung, Spread, Troop};

use cannae_core::hash::StateHasher;
use cannae_core::power::{Activity, PowerTable};
use cannae_core::schedule::BASE_TICK;
use cannae_core::stamina::{self, StaminaParams};
use cannae_core::view::{Group, Order, Rel, Sector, StandardSeen, nb};
use cannae_core::{
    Bodies, Cause, Causes, Coarse, CpUs, Event, EventKind, Gait, Hazards, Intent, Key, Kit, ManId,
    ManModel, Observed, Posture, Rng, Role, SimMs, Stream, Transition, Turn, View, Wound,
};
use curve::rate_q16;
use params::{KEPT_MM, SIGHT_LEADER_MM, SIGHT_STANDARD_MM, UNIT};
use std::cmp::Reverse;

/// Purpose words for the dispositions drawn at setup, on the setup stream.
pub const PURPOSE_NERVE: u32 = 0;
pub const PURPOSE_ARDOUR: u32 = 1;

#[derive(Clone, Copy, Debug)]
struct Man {
    nerve: i32,
    ardour: i32,
    stamina: StaminaParams,
    kit: Kit,
    /// His breath (W') as it stood at `breath_tick`, and what he has been doing since. Written
    /// only when his posture changes.
    breath: CpUs,
    breath_tick: u32,
    activity: Activity,
}

/// The men of a battle, with the memory of one rung of the ladder.
#[derive(Clone, Debug)]
pub struct Men<M = Marks> {
    pub params: Params,
    /// What each activity costs. Shared with the other engine; not a parameter of this one.
    pub power: PowerTable,
    pub memory: M,
    men: Vec<Man>,
}

/// The named parts of a sum, so that an exit can say what moved it most.
#[derive(Clone, Copy, Default)]
struct Terms {
    parts: [(i32, Cause); 16],
    len: usize,
}

impl Terms {
    /// Records a part and returns it. Parts with the same name are added together; a part of
    /// zero or less names no cause.
    fn add(&mut self, value: i32, cause: Cause) -> i32 {
        if value > 0 {
            match self.parts[..self.len].iter_mut().find(|p| p.1 == cause) {
                Some(p) => p.0 += value,
                None => {
                    self.parts[self.len] = (value, cause);
                    self.len += 1;
                }
            }
        }
        value
    }

    /// The three largest parts, largest first.
    fn top(&self) -> Causes {
        let mut parts = self.parts;
        let live = &mut parts[..self.len];
        live.sort_unstable_by_key(|&(value, cause)| (Reverse(value), cause));
        let mut out = [Cause::None; 3];
        for (o, p) in out.iter_mut().zip(live.iter()) {
            *o = p.1;
        }
        out
    }
}

fn capped(n: u32, cap: u32) -> i32 {
    n.min(cap) as i32
}

fn standing(p: Posture) -> bool {
    matches!(p, Posture::Steady | Posture::Forward)
}

fn charging(order: Order) -> bool {
    matches!(order, Order::Charge | Order::Pursue)
}

/// An enemy who is up, turned or not, anywhere in his window.
fn enemy_in_view(v: &View) -> bool {
    [
        Posture::Steady,
        Posture::Forward,
        Posture::Back,
        Posture::Turned,
    ]
    .iter()
    .any(|&p| v.counts.total(Rel::Enemy, p) > 0)
}

/// Whether anything he sees could move him. This is the dead zone the dormancy contract asks
/// for, and it reads the View exactly as [`Men::dormant`] reads the coarse test.
fn awake(v: &View) -> bool {
    v.own.posture != Posture::Steady
        || enemy_in_view(v)
        || v.shocks.any()
        || v.counts.total(Rel::Own, Posture::Back) + v.counts.total(Rel::Own, Posture::Turned) > 0
        || charging(v.order)
}

/// What his leader is doing, if he can make it out and is not the leader himself.
fn leader_seen(v: &View) -> Option<Posture> {
    v.leader
        .filter(|l| v.own.role != Role::Leader && l.dist_mm <= SIGHT_LEADER_MM)
        .map(|l| l.posture)
}

/// His standard is lost to him: down, or carried off by a man in flight.
fn standard_lost(v: &View) -> bool {
    match v.standard {
        Some(StandardSeen::Down { .. }) => true,
        Some(StandardSeen::Up { posture, .. }) => {
            posture == Posture::Turned && v.own.role != Role::Carrier
        }
        None => false,
    }
}

/// His standard stands where he can see it: the place to rally. The man who bears it is his
/// own rallying point.
fn standard_stands(v: &View) -> bool {
    match v.standard {
        Some(StandardSeen::Up { dist_mm, posture }) => {
            dist_mm <= SIGHT_STANDARD_MM
                && (posture != Posture::Turned || v.own.role == Role::Carrier)
        }
        _ => false,
    }
}

fn way(to: Posture, rate_q16: u32, causes: Causes) -> Option<Transition> {
    (rate_q16 > 0).then_some(Transition {
        to,
        rate_q16,
        causes,
    })
}

impl Men<Marks> {
    /// Men at one of the rungs 0 to 2.
    pub fn new(rung: Rung, params: Params, bodies: &Bodies, troops: &[Troop], rng: &Rng) -> Men {
        let marks = Marks::new(rung, &params, bodies.len());
        Men::with_memory(marks, params, bodies, troops, rng)
    }

    pub fn rung(&self) -> Rung {
        self.memory.rung()
    }

    /// His level of *shaken* at `tick`: always zero below rung 2.
    pub fn shaken(&self, man: ManId, tick: u32) -> u8 {
        self.memory.shaken(man, tick)
    }

    pub fn has_fled(&self, man: ManId) -> bool {
        self.memory.has_fled(man)
    }
}

impl<M: Memory> Men<M> {
    /// Draws every man's dispositions from his troop's, on the setup stream. `troops` is
    /// indexed by body; a man of no body is taken for the reference troop. The dispositions do
    /// not depend on the memory, so the same seed makes the same men at every rung.
    pub fn with_memory(
        memory: M,
        params: Params,
        bodies: &Bodies,
        troops: &[Troop],
        rng: &Rng,
    ) -> Men<M> {
        let men = (0..bodies.len())
            .map(|i| {
                let t = bodies.body[i].map_or(Troop::HASTATI, |b| troops[b.0 as usize]);
                let draw = |s: Spread, purpose: u32| {
                    let z = rng.normal_q16(Key::new(Stream::Setup, i as u32, 0, purpose));
                    s.mean + ((s.sd as i64 * z as i64) >> 16) as i32
                };
                let mut nerve = draw(t.nerve, PURPOSE_NERVE);
                let mut ardour = draw(t.ardour, PURPOSE_ARDOUR);
                let mut skill = t.skill;
                match bodies.role[i] {
                    Role::Soldier => {}
                    Role::FileLeader => ardour += t.front_ardour,
                    Role::Leader => {
                        ardour += t.front_ardour + t.leader_ardour;
                        nerve += t.officer_nerve;
                        skill = t.leader_skill;
                    }
                    Role::Closer | Role::Carrier => nerve += t.officer_nerve,
                }
                Man {
                    nerve,
                    ardour,
                    stamina: t.stamina,
                    kit: Kit {
                        reach_mm: t.reach_mm,
                        guard_pm: t.guard_pm,
                        skill,
                    },
                    breath: t.stamina.reserve_us,
                    breath_tick: 0,
                    activity: Activity::Stand,
                }
            })
            .collect();
        Men {
            params,
            power: PowerTable::PLACEHOLDER,
            memory,
            men,
        }
    }

    /// What each man fights with, for the bout resolver.
    pub fn kits(&self) -> Vec<Kit> {
        self.men.iter().map(|m| m.kit).collect()
    }

    pub fn nerve(&self, man: ManId) -> i32 {
        self.men[man.0 as usize].nerve
    }

    pub fn ardour(&self, man: ManId) -> i32 {
        self.men[man.0 as usize].ardour
    }

    /// His breath (W') at `tick`.
    pub fn breath(&self, man: ManId, tick: u32) -> CpUs {
        self.breath_at(&self.men[man.0 as usize], tick)
    }

    fn breath_at(&self, m: &Man, tick: u32) -> CpUs {
        let dt = SimMs(tick.saturating_sub(m.breath_tick) * BASE_TICK.0);
        stamina::step(m.breath, self.power.of(m.activity), &m.stamina, dt)
    }

    /// How winded he is: 0 while more than three quarters of his breath is left, and one
    /// more for each quarter gone after that, to 3 with less than a quarter left.
    fn winded(&self, m: &Man, tick: u32) -> i32 {
        let left = self.breath_at(m, tick) as u64 * 4 / m.stamina.reserve_us.max(1) as u64;
        3 - left.min(3) as i32
    }

    /// What he sees that bears on his alarm, in its three kinds: frightening acts, standing
    /// conditions, and support. Names each part among his fears or among what holds him.
    fn seen(&self, v: &View, fear: &mut Terms, calm: &mut Terms) -> (i32, i32, i32) {
        let p = &self.params;
        let c = &v.counts;
        let own = |posture| c.total(Rel::Own, posture);

        let mut a = 0;
        a += fear.add(
            UNIT * capped(own(Posture::Turned), 8),
            Cause::NeighboursTurned,
        );
        a += fear.add(
            p.back * capped(own(Posture::Back), 8),
            Cause::NeighboursBack,
        );
        a += fear.add(
            p.fall * capped(v.shocks.falls as u32, 3),
            Cause::NeighbourFell,
        );
        let (mut flank, mut rear, mut front) = (0, 0, 0);
        for e in v.near() {
            if !matches!(
                e.posture,
                Posture::Steady | Posture::Forward | Posture::Back
            ) {
                continue;
            }
            match e.sector {
                // An enemy beside him counts only if he faces him and would strike him first:
                // one who is fighting his neighbour is his neighbour's business.
                Sector::Left | Sector::Right if e.faces_me && !e.strikes_another => flank += 1,
                Sector::Rear => rear += 1,
                Sector::Front if e.posture == Posture::Forward => front += 1,
                _ => {}
            }
        }
        a += fear.add(p.enemy_flank * capped(flank, 3), Cause::EnemyOnFlank);
        a += fear.add(p.enemy_rear * capped(rear, 3), Cause::EnemyBehind);
        a += fear.add(p.enemy_front * capped(front, 3), Cause::FrontPressure);
        match leader_seen(v) {
            Some(Posture::Turned) => a += fear.add(p.leader, Cause::NeighboursTurned),
            Some(Posture::Back) => {
                a += fear.add((p.leader * p.back) >> 8, Cause::NeighboursBack);
            }
            Some(Posture::Down) => a += fear.add(p.leader_down, Cause::LeaderDown),
            _ => {}
        }
        let acts = a;

        let mut conditions = 0;
        if let Some(s) = v.slot {
            let open = s.slots & !s.neighbours & (nb::LEFT | nb::RIGHT);
            conditions += fear.add(p.gap * open.count_ones() as i32, Cause::EmptySlot);
        }
        if standard_lost(v) {
            conditions += fear.add(p.standard_down, Cause::StandardDown);
        }
        if v.own.wound == Wound::Hampered {
            conditions += fear.add(p.hampered, Cause::Wounded);
        }

        let mut support = 0;
        let stand = |g| {
            c.get(g, Rel::Own, Posture::Steady) as u32 + c.get(g, Rel::Own, Posture::Forward) as u32
        };
        support += calm.add(
            p.beside * capped(stand(Group::Beside) + stand(Group::Ahead), 6),
            Cause::SteadyNeighbours,
        );
        support += calm.add(
            p.behind * capped(stand(Group::Behind), 6),
            Cause::RanksBehind,
        );
        if leader_seen(v).is_some_and(standing) {
            support += calm.add((p.leader * p.beside) >> 8, Cause::LeaderInView);
        }
        (acts, conditions, support)
    }

    /// His alarm: what he sees that pushes him toward flight, less what holds him, less his
    /// nerve, with what his memory adds. Zero is the edge. Also returns what frightens and
    /// what steadies him, by name.
    fn alarm(&self, man: ManId, v: &View) -> (i32, Terms, Terms) {
        let (mut fear, mut calm) = (Terms::default(), Terms::default());
        let (acts, conditions, support) = self.seen(v, &mut fear, &mut calm);
        let mut a = acts + conditions - support - self.men[man.0 as usize].nerve;
        for (value, cause) in self.memory.alarm_term(man, v.tick) {
            a += value;
            fear.add(value, cause);
            calm.add(-value, cause);
        }
        (a, fear, calm)
    }

    /// The visible layer's sums, for a memory that reads them.
    fn sums(&self, man: ManId, v: &View) -> Sums {
        let (acts, conditions, support) =
            self.seen(v, &mut Terms::default(), &mut Terms::default());
        let m = &self.men[man.0 as usize];
        let urge = self.urge_seen(m, v, &mut Terms::default(), &mut Terms::default());
        Sums {
            acts,
            conditions,
            support,
            urge,
        }
    }

    /// His urge to go forward, with what drives it and what checks it, by name.
    fn urge(&self, man: ManId, v: &View) -> (i32, Terms, Terms) {
        let (mut drive, mut check) = (Terms::default(), Terms::default());
        let m = &self.men[man.0 as usize];
        let mut u = self.urge_seen(m, v, &mut drive, &mut check);
        let (value, cause) = self.memory.urge_term(man, v.tick);
        u += value;
        drive.add(value, cause);
        check.add(-value, cause);
        (u, drive, check)
    }

    /// His urge before his memory's term.
    fn urge_seen(&self, m: &Man, v: &View, drive: &mut Terms, check: &mut Terms) -> i32 {
        let p = &self.params;
        let c = &v.counts;
        let own = |posture| c.total(Rel::Own, posture);
        let ahead = |posture| c.get(Group::Ahead, Rel::Enemy, posture) as u32;
        let mut u = drive.add(m.ardour, Cause::Ardour);
        u += drive.add(
            UNIT * capped(own(Posture::Forward), 8),
            Cause::NeighboursForward,
        );
        if leader_seen(v) == Some(Posture::Forward) {
            u += drive.add(p.leader, Cause::LeaderForward);
        }
        u += drive.add(
            p.enemy_turned * capped(ahead(Posture::Turned), 6)
                + p.enemy_back * capped(ahead(Posture::Back), 6),
            Cause::EnemyBacks,
        );
        if charging(v.order) {
            u += drive.add(p.charge, Cause::OrderCharge);
        }
        u -= check.add(
            p.own_unsteady * capped(own(Posture::Back) + own(Posture::Turned), 6),
            Cause::NeighboursBack,
        );
        u -= check.add(p.breath * self.winded(m, v.tick), Cause::Breath);
        u
    }
}

impl<M: Memory> ManModel for Men<M> {
    fn dormant(&self, _: ManId, own: &Observed, c: &Coarse, _: u32) -> bool {
        own.posture == Posture::Steady
            && !c.enemy_near
            && !c.shock_near
            && !c.own_unsteady
            && !charging(c.order)
    }

    /// Hands his memory what he saw, if he is awake. The runner may call this for every man
    /// or only for those it did not skip; the wake test here makes the two the same.
    fn absorb(&mut self, man: ManId, v: &View) {
        if !awake(v) {
            return;
        }
        let sums = if M::READS_SUMS {
            self.sums(man, v)
        } else {
            Sums::default()
        };
        self.memory.absorb(man, v, &sums);
    }

    fn explain(&self, man: ManId, v: &View) -> Hazards {
        if !awake(v) {
            return Hazards::NONE;
        }
        let p = &self.params;
        let (a, fear, calm) = self.alarm(man, v);
        let rate = |k: i32, margin: i32| rate_q16(k, margin, p.react_q16);
        let enemy = enemy_in_view(v);
        let mut h = Hazards::NONE;
        match v.own.posture {
            Posture::Steady => {
                // He goes forward only at an enemy he can see, or on an order, and never
                // with a wound that hampers him.
                if (enemy || charging(v.order)) && v.own.wound == Wound::Sound {
                    let (u, drive, _) = self.urge(man, v);
                    h.ways[0] = way(Posture::Forward, rate(p.k_urge, u - p.go), drive.top());
                }
                h.ways[1] = way(Posture::Back, rate(p.k_alarm, a), fear.top());
            }
            Posture::Forward => {
                let (rate_q16, causes) = if v.own.wound != Wound::Sound {
                    (p.react_q16, [Cause::Wounded, Cause::None, Cause::None])
                } else if !(enemy || charging(v.order)) {
                    // No one left to go at.
                    (p.react_q16, [Cause::None; 3])
                } else {
                    let (u, _, check) = self.urge(man, v);
                    (rate(p.k_urge, p.go - p.stay - u), check.top())
                };
                h.ways[0] = way(Posture::Steady, rate_q16, causes);
            }
            Posture::Back => {
                let room = v
                    .slot
                    .is_none_or(|s| s.slots & s.neighbours & nb::BEHIND == 0);
                let mut margin = a - p.turn;
                if room {
                    margin += p.room;
                }
                if v.closer_behind {
                    margin -= p.closer;
                }
                h.ways[0] = way(Posture::Turned, rate(p.k_alarm, margin), fear.top());
                h.ways[1] = way(
                    Posture::Steady,
                    rate(p.k_alarm, -a - p.steady_again),
                    calm.top(),
                );
            }
            Posture::Turned => {
                if !enemy && standard_stands(v) {
                    let causes = [Cause::NoPursuer, Cause::CarrierStanding, calm.top()[0]];
                    h.ways[0] = way(Posture::Steady, rate(p.k_alarm, -a - p.rally), causes);
                }
            }
            Posture::Down => {}
        }
        h
    }

    fn intent(&self, man: ManId, v: &View, posture: Posture) -> (Intent, Gait) {
        let p = &self.params;
        let spent = self.breath_at(&self.men[man.0 as usize], v.tick) == 0;
        let run = if spent { Gait::Walk } else { Gait::Run };
        match posture {
            // The same whatever he sees, so a dormant man's kept intent is always right.
            Posture::Steady => (
                Intent::HoldSlot {
                    forward_mm: 0,
                    press: 0,
                },
                Gait::Walk,
            ),
            Posture::Forward => match v.near().find(|e| e.posture != Posture::Down) {
                Some(e) => {
                    // A man who has got to where he would strike from fences and does not
                    // lean: leaning from there would deny the distance he chose. A man kept
                    // from it has someone in his way, and leans on him.
                    let kept = (p.stand_off_mm + KEPT_MM) as i64;
                    let press = if e.offset.len_sq() > kept * kept {
                        p.press
                    } else {
                        0
                    };
                    (
                        Intent::Close {
                            enemy: e.id,
                            stand_off_mm: p.stand_off_mm,
                            press,
                        },
                        if charging(v.order) || e.posture == Posture::Turned {
                            run
                        } else {
                            Gait::Walk
                        },
                    )
                }
                None => (
                    Intent::HoldSlot {
                        forward_mm: p.step_forward_mm,
                        press: 0,
                    },
                    if charging(v.order) { run } else { Gait::Walk },
                ),
            },
            Posture::Back => (
                Intent::HoldSlot {
                    forward_mm: -p.step_back_mm,
                    press: 0,
                },
                Gait::Walk,
            ),
            Posture::Turned => (
                Intent::Flee {
                    // Away from the way his body faced. Once the files have closed over his
                    // slot he has faced about, and runs the way he faces.
                    heading: v
                        .slot
                        .map_or(v.own.facing, |s| Turn(s.facing.0.wrapping_add(Turn::HALF))),
                },
                run,
            ),
            Posture::Down => (Intent::Stand, Gait::Stand),
        }
    }

    fn on_event(&mut self, e: &Event) {
        self.memory.on_event(e);
        let EventKind::Posture { to, .. } = e.kind else {
            return;
        };
        let breath = self.breath_at(&self.men[e.man.0 as usize], e.tick);
        let m = &mut self.men[e.man.0 as usize];
        m.breath = breath;
        m.breath_tick = e.tick;
        m.activity = match to {
            Posture::Steady => Activity::Stand,
            Posture::Forward => Activity::Fight,
            Posture::Back => Activity::Walk,
            Posture::Turned => Activity::Run,
            Posture::Down => Activity::Rest,
        };
    }

    fn inner_hash(&self) -> u64 {
        let mut h = StateHasher::new();
        for m in &self.men {
            h.write_u64(m.breath as u64);
            h.write_u64(
                m.activity as u64
                    | (m.nerve as u16 as u64) << 8
                    | (m.ardour as u16 as u64) << 24
                    | (m.breath_tick as u64) << 32,
            );
        }
        self.memory.hash(&mut h);
        h.finish()
    }
}

#[cfg(test)]
mod tests;
