//! The seam: what a man model and a substrate must provide.
//!
//! Static dispatch only: the runner is generic over both, so the hot path has no `dyn`.

use crate::event::{Causes, Event};
use crate::hazard::{Hazards, choose};
use crate::intent::{Gait, Intent, Target};
use crate::observe::{Bodies, ManId, Observed, Posture};
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
