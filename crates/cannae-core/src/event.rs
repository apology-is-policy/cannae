//! The event log. Every change of posture, every wound, every fallen standard is an event with
//! its tick, its place and the kinds of cause that contributed most. Metrics build the
//! genealogy of a rout from the log; a man never needs to know who did what.

use crate::observe::{BodyId, ManId, Posture, Wound};
use crate::units::Vec2;

/// The kinds of cause a hazard can name. Each is a visible act or condition, or an inner mark,
/// in the common design's vocabulary, so the inspector can say "why" in a line.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Cause {
    #[default]
    None = 0,
    // Alarm.
    NeighboursTurned,
    NeighboursBack,
    EmptySlot,
    EnemyOnFlank,
    EnemyBehind,
    LeaderDown,
    StandardDown,
    FrontPressure,
    Crowding,
    Outnumbered,
    // Support.
    SteadyNeighbours,
    RanksBehind,
    LeaderInView,
    StandardInView,
    CloserBehind,
    FreshLineBehind,
    // Going forward.
    Ardour,
    LeaderForward,
    NeighboursForward,
    EnemyBacks,
    // Body.
    Breath,
    Wounded,
    // Orders.
    OrderCharge,
    OrderRetire,
    OrderHold,
    // Memory.
    HasFled,
    Shaken,
    // Rally.
    NoPursuer,
    CarrierStanding,
    // Resolvers.
    Bout,
    Missile,
    Crush,
}

/// The three largest contributions, largest first; unused slots are `Cause::None`.
pub type Causes = [Cause; 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventKind {
    Posture { from: Posture, to: Posture },
    Wound { to: Wound },
    StandardDown { body: BodyId },
    StandardUp { body: BodyId },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Event {
    pub tick: u32,
    pub man: ManId,
    pub pos: Vec2,
    pub kind: EventKind,
    pub causes: Causes,
}

/// An append-only log in canonical order: by tick, then by man id, then by insertion within a
/// man. Events produced within one phase are sorted before they are applied.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EventLog {
    events: Vec<Event>,
}

impl EventLog {
    pub fn all(&self) -> &[Event] {
        &self.events
    }

    /// Events from `tick` on. The log is in tick order, so this is a binary search.
    pub fn since(&self, tick: u32) -> &[Event] {
        let i = self.events.partition_point(|e| e.tick < tick);
        &self.events[i..]
    }

    /// Appends one phase's events after putting them in canonical order. A stable sort keeps
    /// one man's events in the order he produced them.
    pub fn append_phase(&mut self, mut batch: Vec<Event>) {
        batch.sort_by_key(|e| (e.tick, e.man));
        debug_assert!(
            self.events
                .last()
                .zip(batch.first())
                .is_none_or(|(a, b)| a.tick <= b.tick),
            "events out of tick order"
        );
        self.events.extend(batch);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(tick: u32, man: u32) -> Event {
        Event {
            tick,
            man: ManId(man),
            pos: Vec2::ZERO,
            kind: EventKind::Wound {
                to: Wound::Hampered,
            },
            causes: [Cause::Bout, Cause::None, Cause::None],
        }
    }

    #[test]
    fn phases_are_sorted_and_searchable() {
        let mut log = EventLog::default();
        log.append_phase(vec![ev(5, 9), ev(5, 2), ev(5, 4)]);
        log.append_phase(vec![ev(10, 1)]);
        let men: Vec<u32> = log.all().iter().map(|e| e.man.0).collect();
        assert_eq!(men, vec![2, 4, 9, 1]);
        assert_eq!(log.since(6).len(), 1);
        assert_eq!(log.since(0).len(), 4);
        assert_eq!(log.since(11).len(), 0);
    }
}
