//! Volleys: missiles falling on an area (docs/rival/02-order-and-contact.md, sec. 11).
//!
//! A volley is an event onto an area, not missiles flown one by one. While it lasts, every man
//! standing under it has the same small chance at each man step of being struck, and a strike
//! hampers, disables or kills him. Nobody throws: the fall is the scenario's, so that every
//! run has the same fire to remember, and it starts no bout.
//!
//! Other men see a man fall to a missile exactly as they see one fall in a bout, and they see
//! nothing else of it. There is no row in any man's table for a missile that fells nobody.

use crate::event::{Cause, Event, EventKind};
use crate::fixed::Prob;
use crate::model::Resolver;
use crate::observe::{Bodies, ManId, Posture, Wound};
use crate::rng::{Key, Rng, Stream};
use crate::schedule::MAN_EVERY;
use crate::units::Vec2;
use crate::view::FrameQuery;

/// The purpose word for a volley's draw on the missile stream; the draw's index is the
/// volley's number.
pub const PURPOSE_VOLLEY: u32 = 0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Volley {
    /// It falls at every man step with `from <= tick < until`.
    pub from: u32,
    pub until: u32,
    /// The area: both corners belong to it.
    pub lo: Vec2,
    pub hi: Vec2,
    /// A man's chances at each man step under it.
    pub hamper: Prob,
    pub disable: Prob,
    pub kill: Prob,
}

impl Volley {
    /// A fall from tick `from` to `until` over the rectangle with corners `a` and `b`, whose
    /// dose is given for the whole of it: of every thousand men who stand under it from
    /// beginning to end, about how many it hampers, disables and kills. The chance is spread
    /// evenly over the man steps. "About", because a man hampered may be struck again, and a
    /// man down is struck no more.
    pub fn of_dose(from: u32, until: u32, a: Vec2, b: Vec2, per_thousand: [u32; 3]) -> Volley {
        let steps = until
            .div_ceil(MAN_EVERY)
            .saturating_sub(from.div_ceil(MAN_EVERY));
        let each = |dose: u32| Prob::from_ratio(dose as u64, 1000 * steps.max(1) as u64);
        Volley {
            from,
            until,
            lo: Vec2::new(a.x.min(b.x), a.y.min(b.y)),
            hi: Vec2::new(a.x.max(b.x), a.y.max(b.y)),
            hamper: each(per_thousand[0]),
            disable: each(per_thousand[1]),
            kill: each(per_thousand[2]),
        }
    }

    /// Whether a man standing at `p` is under it.
    pub fn covers(&self, p: Vec2) -> bool {
        (self.lo.x..=self.hi.x).contains(&p.x) && (self.lo.y..=self.hi.y).contains(&p.y)
    }

    /// What one draw does to a man under it.
    fn strike(&self, draw: u32) -> Wound {
        let draw = draw as u64;
        let (kill, disable) = (self.kill.q32(), self.kill.q32() + self.disable.q32());
        if draw < kill {
            Wound::Dead
        } else if draw < disable {
            Wound::Disabled
        } else if draw < disable + self.hamper.q32() {
            Wound::Hampered
        } else {
            Wound::Sound
        }
    }
}

/// Every volley of a scenario. With none, it does nothing and draws nothing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Volleys {
    volleys: Vec<Volley>,
    took: Vec<(ManId, Wound)>,
}

impl Volleys {
    pub fn new(volleys: Vec<Volley>) -> Volleys {
        Volleys {
            volleys,
            took: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.volleys.is_empty()
    }

    pub fn all(&self) -> &[Volley] {
        &self.volleys
    }
}

impl Resolver for Volleys {
    fn resolve<F: FrameQuery>(
        &mut self,
        bodies: &Bodies,
        _frames: &F,
        tick: u32,
        rng: &Rng,
        out: &mut Vec<Event>,
    ) {
        self.took.clear();
        for (k, v) in self.volleys.iter().enumerate() {
            if tick < v.from || tick >= v.until {
                continue;
            }
            for m in 0..bodies.len() {
                if bodies.posture[m] == Posture::Down || !v.covers(bodies.pos[m]) {
                    continue;
                }
                let key = Key::new(Stream::Missile, m as u32, tick, PURPOSE_VOLLEY).at(k as u32);
                let wound = v.strike(rng.u32(key));
                if wound > bodies.wound[m] {
                    self.took.push((ManId(m as u32), wound));
                }
            }
        }
        // Under two volleys at once, a man takes the worse of what they do to him.
        self.took.sort_unstable();
        for i in 0..self.took.len() {
            let (man, wound) = self.took[i];
            if self.took.get(i + 1).is_some_and(|next| next.0 == man) {
                continue;
            }
            let m = man.0 as usize;
            let event = |kind| Event {
                tick,
                man,
                pos: bodies.pos[m],
                kind,
                causes: [Cause::Missile, Cause::None, Cause::None],
            };
            out.push(event(EventKind::Wound { to: wound }));
            if wound >= Wound::Disabled {
                // What the men about him see is the fall, as with a bout's.
                out.push(event(EventKind::Posture {
                    from: bodies.posture[m],
                    to: Posture::Down,
                }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observe::Observed;
    use crate::view::NoFrames;

    const LO: Vec2 = Vec2::new(0, 0);
    const HI: Vec2 = Vec2::new(100_000, 40_000);
    /// Two minutes: 480 man steps.
    const UNTIL: u32 = 2_400;

    /// `n` men a metre apart in rows of a hundred, from `(x0, 0)`.
    fn men(n: i32, x0: i32) -> Bodies {
        let mut b = Bodies::default();
        for i in 0..n {
            b.push(Observed {
                pos: Vec2::new(x0 + (i % 100) * 1000, (i / 100) * 1000),
                ..Default::default()
            });
        }
        b
    }

    /// Runs the volleys over every tick of `ticks`, applying what they do as the runner does.
    fn fall(v: &mut Volleys, b: &mut Bodies, ticks: std::ops::Range<u32>) -> Vec<Event> {
        fall_with(&Rng::new(11), v, b, ticks)
    }

    fn fall_with(
        rng: &Rng,
        v: &mut Volleys,
        b: &mut Bodies,
        ticks: std::ops::Range<u32>,
    ) -> Vec<Event> {
        let mut all = Vec::new();
        for t in ticks.filter(|t| t.is_multiple_of(MAN_EVERY)) {
            let mut out = Vec::new();
            v.resolve(b, &NoFrames, t, rng, &mut out);
            for e in &out {
                let m = e.man.0 as usize;
                match e.kind {
                    EventKind::Wound { to } => {
                        assert!(to > b.wound[m], "a wound that is no worse: {e:?}");
                        b.wound[m] = to;
                    }
                    EventKind::Posture { from, to } => {
                        assert_eq!((from, to), (b.posture[m], Posture::Down), "{e:?}");
                        b.posture[m] = to;
                    }
                    _ => panic!("{e:?}"),
                }
                assert_eq!(e.causes, [Cause::Missile, Cause::None, Cause::None]);
            }
            all.extend(out);
        }
        all
    }

    fn count(b: &Bodies, w: Wound) -> usize {
        b.wound.iter().filter(|&&x| x == w).count()
    }

    #[test]
    fn the_dose_over_the_whole_fall_is_the_scripts() {
        let mut b = men(4000, 0);
        let mut v = Volleys::new(vec![Volley::of_dose(0, UNTIL, HI, LO, [75, 20, 5])]);
        let events = fall(&mut v, &mut b, 0..UNTIL + 400);
        // Of 4,000: 20 killed, 80 disabled, and 300 hampered less the few felled afterwards.
        // Each band is four standard deviations wide on either side.
        let (dead, disabled, hampered) = (
            count(&b, Wound::Dead),
            count(&b, Wound::Disabled),
            count(&b, Wound::Hampered),
        );
        assert!((3..=38).contains(&dead), "{dead} killed");
        assert!((45..=115).contains(&disabled), "{disabled} disabled");
        assert!((225..=365).contains(&hampered), "{hampered} hampered");
        // A man down is always seen to fall, once.
        let falls = events
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Posture { .. }))
            .count();
        assert_eq!(falls, dead + disabled);
        for m in 0..b.len() {
            assert_eq!(b.posture[m] == Posture::Down, b.wound[m] >= Wound::Disabled);
        }
        // Nothing falls after it ends, and it is spread over its length, not struck at once.
        let last = events.iter().map(|e| e.tick).max().unwrap();
        assert!(last < UNTIL, "a strike at tick {last}");
        let early = events.iter().filter(|e| e.tick < UNTIL / 2).count();
        assert!(
            (events.len() / 3..=events.len() * 2 / 3).contains(&early),
            "{early} of {} events in the first half",
            events.len()
        );
    }

    #[test]
    fn it_strikes_only_men_standing_in_its_area_and_only_while_it_lasts() {
        // Certain death for whoever is under it.
        let sure =
            |from, until| Volley::of_dose(from, until, LO, Vec2::new(9_000, 0), [0, 0, 1000]);
        let mut b = men(12, 0);
        b.posture[3] = Posture::Down;
        let mut v = Volleys::new(vec![sure(100, 105)]);
        assert!(fall(&mut v, &mut b, 0..100).is_empty(), "before it begins");
        let struck = fall(&mut v, &mut b, 100..105);
        let who: Vec<u32> = struck
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Wound { .. }))
            .map(|e| e.man.0)
            .collect();
        // Men 0 to 9 stand from 0 to 9 m, both ends of the area included; man 3 is down
        // already, and men 10 and 11 stand beyond it.
        assert_eq!(who, vec![0, 1, 2, 4, 5, 6, 7, 8, 9]);
        let mut b = men(12, 0);
        assert!(fall(&mut v, &mut b, 105..400).is_empty(), "after it ends");
    }

    #[test]
    fn a_mans_luck_does_not_depend_on_who_else_is_there() {
        let v = Volley::of_dose(0, UNTIL, LO, HI, [300, 100, 50]);
        let (mut crowd, mut few) = (men(400, 0), men(40, 0));
        let all = fall(&mut Volleys::new(vec![v]), &mut crowd, 0..UNTIL);
        let some = fall(&mut Volleys::new(vec![v]), &mut few, 0..UNTIL);
        let of_the_few: Vec<Event> = all.into_iter().filter(|e| e.man.0 < 40).collect();
        assert!(some.len() > 5, "only {} events", some.len());
        assert_eq!(some, of_the_few);
    }

    #[test]
    fn under_two_volleys_at_once_a_man_takes_the_worse() {
        let one = |dose| Volley::of_dose(0, 5, LO, HI, dose);
        // The first hampers every man; the second kills about half of them.
        let mut b = men(200, 0);
        let mut v = Volleys::new(vec![one([1000, 0, 0]), one([0, 0, 500])]);
        let events = fall(&mut v, &mut b, 0..5);
        let (dead, hampered) = (count(&b, Wound::Dead), count(&b, Wound::Hampered));
        assert_eq!(dead + hampered, 200);
        assert!((70..=130).contains(&dead), "{dead} killed");
        // One wound for each man, and a fall for each of the dead: `fall` has checked that
        // none was a wound no worse than the one he had.
        assert_eq!(events.len(), 200 + dead);

        // Each volley has its own luck: two that each kill half kill three in four together.
        let mut b = men(200, 0);
        let mut v = Volleys::new(vec![one([0, 0, 500]), one([0, 0, 500])]);
        fall(&mut v, &mut b, 0..5);
        let dead = count(&b, Wound::Dead);
        assert!((125..=175).contains(&dead), "{dead} killed");
    }

    #[test]
    fn its_luck_is_the_missile_streams_and_no_other() {
        let v = Volley::of_dose(0, UNTIL, LO, HI, [300, 100, 50]);
        let run =
            |rng: Rng| fall_with(&rng, &mut Volleys::new(vec![v]), &mut men(100, 0), 0..UNTIL);
        let plain = run(Rng::new(11));
        assert!(plain.len() > 20, "only {} events", plain.len());
        assert_eq!(run(Rng::new(11).fork(Stream::Combat, 0, 1)), plain);
        assert_eq!(run(Rng::new(11).fork(Stream::Hazard, 0, 1)), plain);
        assert_ne!(run(Rng::new(11).fork(Stream::Missile, 0, 1)), plain);
        // Forked half way through, it is the same fire until then.
        let forked = run(Rng::new(11).fork(Stream::Missile, UNTIL / 2, 1));
        let early = |events: &[Event]| events.iter().filter(|e| e.tick < UNTIL / 2).count();
        assert_ne!(forked, plain);
        assert_eq!(forked[..early(&forked)], plain[..early(&plain)]);
    }

    #[test]
    fn no_volleys_do_nothing() {
        let mut b = men(50, 0);
        let mut v = Volleys::default();
        assert!(v.is_empty());
        assert!(fall(&mut v, &mut b, 0..UNTIL).is_empty());
    }
}
