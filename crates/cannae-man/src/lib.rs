//! The rival's man (docs/rival/01 and 07): five postures, and hazards read off what he sees.
//!
//! Two sums decide everything. **Alarm** is what he sees that pushes him toward flight, less
//! what he sees that holds him, less his nerve. **Urge** is what sends him forward at the
//! enemy. Each way out of a posture is one hazard on one curve ([`curve::rate_q16`]) of one of
//! the two sums against a threshold.
//!
//! What he carries from one moment to the next is his wounds and posture (which everyone can
//! see), his breath, and the marks of his rung of the memory ladder ([`Rung`]). Nothing else:
//! no fear level, no morale. Other men never read any of it.
//!
//! Dormancy is exact. A steady man with no enemy in his window, no shock, no one of his side
//! wavering in view and no order to charge has no hazards at all, by a gate and not by a small
//! number. Breath and *shaken* are stored with the tick they were last written and read in
//! closed form, so skipping a man can never change a bit.

pub mod curve;
pub mod params;

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
use params::{SHAKEN_MAX, SIGHT_LEADER_MM, SIGHT_STANDARD_MM, UNIT};
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
    has_fled: bool,
    /// *Shaken* as it stood at `shaken_tick`. Written only when a fright raises it.
    shaken: u8,
    shaken_tick: u32,
}

/// The men of a battle, at one rung of the memory ladder.
#[derive(Clone, Debug)]
pub struct Men {
    pub rung: Rung,
    pub params: Params,
    /// What each activity costs. Shared with the other engine; not a parameter of this one.
    pub power: PowerTable,
    men: Vec<Man>,
}

/// The named parts of a sum, so that an exit can say what moved it most.
#[derive(Clone, Copy, Default)]
struct Terms {
    parts: [(i32, Cause); 12],
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

impl Men {
    /// Draws every man's dispositions from his troop's, on the setup stream. `troops` is
    /// indexed by body; a man of no body is taken for the reference troop.
    pub fn new(rung: Rung, params: Params, bodies: &Bodies, troops: &[Troop], rng: &Rng) -> Men {
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
                    has_fled: false,
                    shaken: 0,
                    shaken_tick: 0,
                }
            })
            .collect();
        Men {
            rung,
            params,
            power: PowerTable::PLACEHOLDER,
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

    /// His level of *shaken* at `tick`: always zero below rung 2.
    pub fn shaken(&self, man: ManId, tick: u32) -> u8 {
        self.shaken_at(&self.men[man.0 as usize], tick)
    }

    pub fn has_fled(&self, man: ManId) -> bool {
        self.men[man.0 as usize].has_fled
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

    fn shaken_at(&self, m: &Man, tick: u32) -> u8 {
        let worn = tick.saturating_sub(m.shaken_tick) / self.params.shaken_wear.max(1);
        m.shaken.saturating_sub(worn.min(u8::MAX as u32) as u8)
    }

    /// His alarm: what he sees that pushes him toward flight, less what holds him, less his
    /// nerve. Zero is the edge. Also returns what frightens and what steadies him, by name.
    fn alarm(&self, m: &Man, v: &View) -> (i32, Terms, Terms) {
        let p = &self.params;
        let c = &v.counts;
        let own = |posture| c.total(Rel::Own, posture);
        let mut fear = Terms::default();
        let mut a = 0;

        // Acts.
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
                // An enemy beside him who faces his neighbour is his neighbour's business.
                Sector::Left | Sector::Right if e.faces_me => flank += 1,
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

        // Conditions.
        if let Some(s) = v.slot {
            let open = s.slots & !s.neighbours & (nb::LEFT | nb::RIGHT);
            a += fear.add(p.gap * open.count_ones() as i32, Cause::EmptySlot);
        }
        if standard_lost(v) {
            a += fear.add(p.standard_down, Cause::StandardDown);
        }

        // Support.
        let mut calm = Terms::default();
        let stand = |g| {
            c.get(g, Rel::Own, Posture::Steady) as u32 + c.get(g, Rel::Own, Posture::Forward) as u32
        };
        a -= calm.add(
            p.beside * capped(stand(Group::Beside) + stand(Group::Ahead), 6),
            Cause::SteadyNeighbours,
        );
        a -= calm.add(
            p.behind * capped(stand(Group::Behind), 6),
            Cause::RanksBehind,
        );
        if leader_seen(v).is_some_and(standing) {
            a -= calm.add((p.leader * p.beside) >> 8, Cause::LeaderInView);
        }

        // His nerve, and what has worn it.
        a -= m.nerve;
        if v.own.wound == Wound::Hampered {
            a += fear.add(p.hampered, Cause::Wounded);
        }
        if self.rung >= Rung::HasFled && m.has_fled {
            a += fear.add(p.has_fled, Cause::HasFled);
        }
        if self.rung >= Rung::Shaken {
            a += fear.add(p.shaken * self.shaken_at(m, v.tick) as i32, Cause::Shaken);
        }
        (a, fear, calm)
    }

    /// His urge to go forward, with what drives it and what checks it, by name.
    fn urge(&self, m: &Man, v: &View) -> (i32, Terms, Terms) {
        let p = &self.params;
        let c = &v.counts;
        let own = |posture| c.total(Rel::Own, posture);
        let ahead = |posture| c.get(Group::Ahead, Rel::Enemy, posture) as u32;
        let mut drive = Terms::default();
        let mut check = Terms::default();
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
        (u, drive, check)
    }
}

impl ManModel for Men {
    fn dormant(&self, _: ManId, own: &Observed, c: &Coarse, _: u32) -> bool {
        own.posture == Posture::Steady
            && !c.enemy_near
            && !c.shock_near
            && !c.own_unsteady
            && !charging(c.order)
    }

    /// Rung 2 only: each fall he sees, and the fall of his standard or his leader, raises
    /// *shaken* a level. A rule, not a draw, so the seam needs no generator here.
    fn absorb(&mut self, man: ManId, v: &View) {
        if self.rung < Rung::Shaken {
            return;
        }
        let frights = v.shocks.falls.min(SHAKEN_MAX)
            + v.shocks.standard_fell as u8
            + v.shocks.leader_fell as u8;
        if frights == 0 {
            return;
        }
        let now = self.shaken_at(&self.men[man.0 as usize], v.tick);
        let m = &mut self.men[man.0 as usize];
        m.shaken = (now + frights).min(SHAKEN_MAX);
        m.shaken_tick = v.tick;
    }

    fn explain(&self, man: ManId, v: &View) -> Hazards {
        if !awake(v) {
            return Hazards::NONE;
        }
        let p = &self.params;
        let m = &self.men[man.0 as usize];
        let (a, fear, calm) = self.alarm(m, v);
        let rate = |k: i32, margin: i32| rate_q16(k, margin, p.react_q16);
        let enemy = enemy_in_view(v);
        let mut h = Hazards::NONE;
        match v.own.posture {
            Posture::Steady => {
                // He goes forward only at an enemy he can see, or on an order, and never
                // with a wound that hampers him.
                if (enemy || charging(v.order)) && v.own.wound == Wound::Sound {
                    let (u, drive, _) = self.urge(m, v);
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
                    let (u, _, check) = self.urge(m, v);
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
                Some(e) => (
                    Intent::Close {
                        enemy: e.id,
                        stand_off_mm: p.stand_off_mm,
                    },
                    if charging(v.order) || e.posture == Posture::Turned {
                        run
                    } else {
                        Gait::Walk
                    },
                ),
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
        if to == Posture::Turned && self.rung >= Rung::HasFled {
            m.has_fled = true;
        }
    }

    fn inner_hash(&self) -> u64 {
        let mut h = StateHasher::new();
        for m in &self.men {
            h.write_u64(m.breath as u64);
            h.write_u64((m.breath_tick as u64) << 32 | m.shaken_tick as u64);
            h.write_u64(
                m.activity as u64
                    | (m.has_fled as u64) << 8
                    | (m.shaken as u64) << 16
                    | (m.nerve as u16 as u64) << 32
                    | (m.ardour as u16 as u64) << 48,
            );
        }
        h.finish()
    }
}

#[cfg(test)]
mod tests;
