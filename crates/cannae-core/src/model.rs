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
use crate::view::{Coarse, View};

/// A rung of the memory ladder. Every rung shares the visible layer: postures, hazards read off
/// what a man sees, and the event log. Rungs differ in the inner state behind the hazards.
///
/// **Inner state is the model's own and only the model's.** Other men never read it.
///
/// **Dormancy.** A model may call a man dormant only if his hazards would be exactly zero for
/// any full View consistent with the coarse one: hazards need a dead zone. A dormant man is not
/// evaluated, consumes no draws, and his inner state does not change. Inner state that decays
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
}

/// Moves bodies. The pairwise substrate moves every base tick; the field substrate may batch
/// its work but must present the state at base-tick granularity to the runner.
pub trait Substrate {
    fn advance(&mut self, bodies: &mut Bodies, targets: &[Target], dt: SimMs, rng: &Rng, tick: u32);
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
