//! The seam: what a man model and a substrate must provide.
//!
//! Static dispatch only: the runner is generic over both, so the hot path has no `dyn`.

use crate::event::{Causes, Event, EventKind};
use crate::hazard::{Hazards, choose};
use crate::intent::{Gait, Intent, Target};
use crate::observe::{Bodies, ManId, Observed, Posture, Wound};
use crate::rng::{Key, Rng, Stream};
use crate::schedule::MAN_STEP;
use crate::units::SimMs;
use crate::view::{Coarse, FrameQuery, NoFrames, View};

/// A rung of the memory ladder. Every rung shares the visible layer: postures, hazards read off
/// what a man sees, and the event log. Rungs differ in the inner state behind the hazards.
///
/// **Inner state is the model's own and only the model's.** Other men never read it.
///
/// **Dormancy.** A model may call a man dormant only if his hazards would be exactly zero for
/// any full View consistent with the coarse one: hazards need a dead zone. A dormant man is not
/// evaluated, consumes no draws, keeps his last intent, and his inner state does not change. So
/// his intent, like his hazards, may depend only on what would wake him. Inner state that decays
/// must therefore be stored with the tick it was last updated and advanced in closed form when
/// read, in every mode, so that skipping a man can never change a bit. A rung that cannot meet
/// this (a noise term, say) simply never declares a man dormant, and pays for it in speed.
pub trait ManModel {
    fn dormant(&self, man: ManId, own: &Observed, coarse: &Coarse, tick: u32) -> bool;

    /// Takes in what he saw this step: raises marks or gauges from shocks, and so on. Touches
    /// only `man`'s inner state, so the order in which men are processed cannot matter.
    fn absorb(&mut self, man: ManId, view: &View);

    /// Pure: his hazards and their causes. The inspector, the tests and the genealogy all call
    /// this.
    fn explain(&self, man: ManId, view: &View) -> Hazards;

    /// What he means to do, in the posture he will hold after this step.
    fn intent(&self, man: ManId, view: &View, posture: Posture) -> (Intent, Gait);

    /// An event that happened to him (a posture change, a wound): updates his inner state,
    /// such as the *has fled* mark.
    fn on_event(&mut self, ev: &Event);

    /// A stable hash of all inner state, for the per-step state hash and the conformance tests.
    fn inner_hash(&self) -> u64;
}

/// The orders-and-frames phase: carriers, slot maps, drills, `in_place`. Runs first at every
/// man step and may emit events (a standard picked up).
pub trait Frames: FrameQuery {
    fn update(&mut self, bodies: &mut Bodies, tick: u32, out: &mut Vec<Event>);
}

impl Frames for NoFrames {
    fn update(&mut self, _: &mut Bodies, _: u32, _: &mut Vec<Event>) {}
}

/// The bout and volley phase. Reads post-motion positions, emits wounds and the posture
/// changes they cause; the runner applies them together at the end of the phase.
pub trait Resolver {
    fn resolve<F: FrameQuery>(
        &mut self,
        bodies: &Bodies,
        frames: &F,
        tick: u32,
        rng: &Rng,
        out: &mut Vec<Event>,
    );
}

/// No fighting: for tests of everything else.
pub struct NoBouts;

impl Resolver for NoBouts {
    fn resolve<F: FrameQuery>(&mut self, _: &Bodies, _: &F, _: u32, _: &Rng, _: &mut Vec<Event>) {}
}

/// Two resolvers in one phase, as bouts and volleys are. Each reads the same snapshot, so one
/// man may be struck by both: [`take_the_worst`] settles what is logged for him.
impl<A: Resolver, B: Resolver> Resolver for (A, B) {
    fn resolve<F: FrameQuery>(
        &mut self,
        bodies: &Bodies,
        frames: &F,
        tick: u32,
        rng: &Rng,
        out: &mut Vec<Event>,
    ) {
        let start = out.len();
        self.0.resolve(bodies, frames, tick, rng, out);
        let mid = out.len();
        self.1.resolve(bodies, frames, tick, rng, out);
        if mid > start && out.len() > mid {
            take_the_worst(out, start);
        }
    }
}

/// Settles one phase's outcomes from more than one source, from index `from` on. Of a man's
/// wounds only the worst is kept. A man who falls keeps one fall and no other change of
/// posture, so the men about him count him once. Bouts stay as they were logged.
pub fn take_the_worst(out: &mut Vec<Event>, from: usize) {
    let mut worst: Vec<(ManId, Wound)> = out[from..]
        .iter()
        .filter_map(|e| match e.kind {
            EventKind::Wound { to } => Some((e.man, to)),
            _ => None,
        })
        .collect();
    // By man, his wounds in rising order: the last of each man's is his worst.
    worst.sort_unstable();
    worst.dedup_by(|later, kept| {
        let same = later.0 == kept.0;
        if same {
            kept.1 = later.1;
        }
        same
    });
    let (mut wound_kept, mut fall_kept) = (vec![false; worst.len()], vec![false; worst.len()]);
    let mut i = 0;
    out.retain(|e| {
        i += 1;
        if i <= from {
            return true;
        }
        let Ok(w) = worst.binary_search_by_key(&e.man, |w| w.0) else {
            return true;
        };
        match e.kind {
            EventKind::Wound { to } => {
                to == worst[w].1 && !std::mem::replace(&mut wound_kept[w], true)
            }
            EventKind::Posture { to, .. } if worst[w].1 >= Wound::Disabled => {
                to == Posture::Down && !std::mem::replace(&mut fall_kept[w], true)
            }
            _ => true,
        }
    });
}

/// Moves bodies. The pairwise substrate moves every base tick; the field substrate may batch
/// its work but must present the state at base-tick granularity to the runner. A substrate may
/// emit events of its own (a man injured in a crush, with `Cause::Crush`), which the runner
/// applies at the end of the base tick.
pub trait Substrate {
    fn advance(
        &mut self,
        bodies: &mut Bodies,
        targets: &[Target],
        dt: SimMs,
        rng: &Rng,
        tick: u32,
        out: &mut Vec<Event>,
    );
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Decision {
    pub change: Option<(Posture, Causes)>,
    pub intent: Intent,
    pub gait: Gait,
}

/// The purpose word for the posture draw on the hazard stream.
pub const PURPOSE_POSTURE: u32 = 0;

/// One man's decision at a man step: absorb, explain, one draw, intent.
pub fn decide<M: ManModel>(model: &mut M, view: &View, rng: &Rng) -> Decision {
    model.absorb(view.man, view);
    let h = model.explain(view.man, view);
    let draw = rng.u32(Key::new(
        Stream::Hazard,
        view.man.0,
        view.tick,
        PURPOSE_POSTURE,
    ));
    let t = choose(&h, MAN_STEP, draw);
    let posture = t.map_or(view.own.posture, |t| t.to);
    let (intent, gait) = model.intent(view.man, view, posture);
    Decision {
        change: t.map(|t| (t.to, t.causes)),
        intent,
        gait,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{BoutOutcome, Cause};
    use crate::units::Vec2;

    /// A resolver that says what it is told to.
    struct Says(Vec<Event>);

    impl Resolver for Says {
        fn resolve<F: FrameQuery>(
            &mut self,
            _: &Bodies,
            _: &F,
            _: u32,
            _: &Rng,
            out: &mut Vec<Event>,
        ) {
            out.extend(self.0.iter().copied());
        }
    }

    fn ev(man: u32, kind: EventKind, cause: Cause) -> Event {
        Event {
            tick: 5,
            man: ManId(man),
            pos: Vec2::ZERO,
            kind,
            causes: [cause, Cause::None, Cause::None],
        }
    }

    fn wound(man: u32, to: Wound, cause: Cause) -> Event {
        ev(man, EventKind::Wound { to }, cause)
    }

    fn posture(man: u32, to: Posture, cause: Cause) -> Event {
        let from = Posture::Steady;
        ev(man, EventKind::Posture { from, to }, cause)
    }

    fn both(a: Vec<Event>, b: Vec<Event>, before: Vec<Event>) -> Vec<Event> {
        let mut out = before;
        (Says(a), Says(b)).resolve(&Bodies::default(), &NoFrames, 5, &Rng::new(1), &mut out);
        out
    }

    #[test]
    fn a_man_struck_by_both_takes_the_worse_and_falls_once() {
        use Cause::{Bout, Missile};
        use Posture::{Back, Down};
        use Wound::{Dead, Disabled, Hampered};
        let bout = ev(
            7,
            EventKind::Bout {
                attacker: ManId(70),
                defender: BoutOutcome::DrivenBack,
                attacker_took: BoutOutcome::Nothing,
            },
            Bout,
        );
        let a = vec![
            // Man 3 is disabled in a bout and killed by a missile.
            wound(3, Disabled, Bout),
            posture(3, Down, Bout),
            // Man 5 is hampered in a bout and disabled by a missile.
            wound(5, Hampered, Bout),
            // Man 7 is driven back in a bout and disabled by a missile.
            bout,
            posture(7, Back, Bout),
            // Man 8 is killed in a bout and hampered by a missile.
            wound(8, Dead, Bout),
            posture(8, Down, Bout),
            // Man 9 is driven back, and no missile finds him.
            posture(9, Back, Bout),
            // Man 4 is disabled by both: the first to be logged stands.
            wound(4, Disabled, Bout),
            posture(4, Down, Bout),
            // Man 11 is driven back in a bout and hampered by a missile: he does both.
            posture(11, Back, Bout),
        ];
        let b = vec![
            wound(3, Dead, Missile),
            posture(3, Down, Missile),
            wound(5, Disabled, Missile),
            posture(5, Down, Missile),
            wound(7, Disabled, Missile),
            posture(7, Down, Missile),
            wound(8, Hampered, Missile),
            // Man 10 is hampered by a missile alone.
            wound(10, Hampered, Missile),
            wound(4, Disabled, Missile),
            posture(4, Down, Missile),
            wound(11, Hampered, Missile),
        ];
        // An earlier phase's wound of man 3 is no business of this one.
        let before = vec![wound(3, Hampered, Bout)];
        let out = both(a, b, before.clone());
        assert_eq!(
            out,
            [
                before,
                vec![
                    posture(3, Down, Bout),
                    bout,
                    wound(8, Dead, Bout),
                    posture(8, Down, Bout),
                    posture(9, Back, Bout),
                    wound(4, Disabled, Bout),
                    posture(4, Down, Bout),
                    posture(11, Back, Bout),
                    wound(3, Dead, Missile),
                    wound(5, Disabled, Missile),
                    posture(5, Down, Missile),
                    wound(7, Disabled, Missile),
                    posture(7, Down, Missile),
                    wound(10, Hampered, Missile),
                    wound(11, Hampered, Missile),
                ]
            ]
            .concat()
        );
    }

    #[test]
    fn one_resolver_alone_is_left_as_it_spoke() {
        // A lone source settles its own men; the pair must not touch what it says.
        let said = vec![
            wound(3, Wound::Hampered, Cause::Bout),
            wound(3, Wound::Dead, Cause::Bout),
        ];
        assert_eq!(both(said.clone(), vec![], vec![]), said);
        assert_eq!(both(vec![], said.clone(), vec![]), said);
    }
}
