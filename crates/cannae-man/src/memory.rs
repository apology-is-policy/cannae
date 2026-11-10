//! The memory ladder's hook.
//!
//! The visible layer reads everything else off what a man sees now. A [`Memory`] is what he
//! carries from one moment to the next that no one else can see. It adds a term to his alarm
//! and a term to his urge, and takes in what he saw at each step he is awake. The rungs of the
//! ladder are memories and nothing else, so the perception weights are the same on every rung.
//!
//! Five rules make dormancy exact, and every memory must keep them.
//!
//! 1. **A term cannot wake a man.** Whether he is awake is decided by what he sees alone
//!    (`Men::dormant`). His terms are read only while he is awake.
//! 2. **Terms are closed form.** A term is a function of what is stored and of the tick, and
//!    reading it changes nothing. Whatever decays is stored with the tick it was written at and
//!    read forward.
//! 3. **Each part names its cause,** so that an exit can say what moved it most. A part of
//!    zero names none.
//! 4. **`absorb` is called once at each man step for every man who is awake, and for no one
//!    else.** That holds whether dormant men are skipped or evaluated: `Men` applies the wake
//!    test itself before it calls.
//! 5. **Nothing is drawn.** `absorb` has no generator. A memory that needs chance must ask for
//!    it at the seam and say so in its priors.

use crate::params::{Params, Rung, SHAKEN_MAX};
use cannae_core::hash::StateHasher;
use cannae_core::{Cause, Event, EventKind, ManId, Posture, View};

/// The most parts one term may have.
pub const PARTS: usize = 4;

/// A term in parts, in alarm units with 8 fractional bits. A part above zero frightens him and
/// is named among his fears; a part below zero steadies him and is named among what holds him.
pub type Parts = [(i32, Cause); PARTS];

pub const NO_PARTS: Parts = [(0, Cause::None); PARTS];

/// The visible layer's sums at one step, before his nerve and before any memory, in alarm
/// units with 8 fractional bits: 256 is one man of his side turning his back.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Sums {
    /// Frightening acts in view: men of his side turned, giving ground or fallen; enemies at
    /// his flank, behind him, or coming on at his front; what his leader did.
    pub acts: i32,
    /// Standing conditions: an open slot beside him, his standard lost, a hampering wound.
    pub conditions: i32,
    /// What steadies him: men standing about him, the ranks behind, his leader in view. Above
    /// zero.
    pub support: i32,
    /// His urge to go forward, his ardour and his breath included.
    pub urge: i32,
}

pub trait Memory {
    /// Whether `absorb` reads the sums. Where it does not they are not worked out, and
    /// `absorb` is handed zeros.
    const READS_SUMS: bool = false;

    /// What his memory adds to his alarm at `tick`.
    fn alarm_term(&self, man: ManId, tick: u32) -> Parts;

    /// What it adds to his urge at `tick`: above zero drives him forward, below zero checks
    /// him.
    fn urge_term(&self, _man: ManId, _tick: u32) -> (i32, Cause) {
        (0, Cause::None)
    }

    /// One man step of what he saw, at `v.tick`. Only for a man who is awake (rule 4).
    fn absorb(&mut self, man: ManId, v: &View, sums: &Sums);

    /// Something happened to a man: a change of posture, a wound.
    fn on_event(&mut self, e: &Event);

    /// Everything a step could change.
    fn hash(&self, h: &mut StateHasher);
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Mark {
    has_fled: bool,
    /// *Shaken* as it stood at `shaken_tick`. Written only when a fright raises it.
    shaken: u8,
    shaken_tick: u32,
}

/// Rungs 0 to 2 of the ladder: no marks, *has fled*, and *shaken* as well.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Marks {
    rung: Rung,
    /// The weights, from the parameters the men were made with.
    has_fled: i32,
    shaken: i32,
    shaken_wear: u32,
    men: Vec<Mark>,
}

impl Marks {
    pub fn new(rung: Rung, p: &Params, men: usize) -> Marks {
        Marks {
            rung,
            has_fled: p.has_fled,
            shaken: p.shaken,
            shaken_wear: p.shaken_wear,
            men: vec![Mark::default(); men],
        }
    }

    pub fn rung(&self) -> Rung {
        self.rung
    }

    /// Whether he has turned his back before: always false at rung 0.
    pub fn has_fled(&self, man: ManId) -> bool {
        self.men[man.0 as usize].has_fled
    }

    /// His level of *shaken* at `tick`: always zero below rung 2.
    pub fn shaken(&self, man: ManId, tick: u32) -> u8 {
        let m = &self.men[man.0 as usize];
        let worn = tick.saturating_sub(m.shaken_tick) / self.shaken_wear.max(1);
        m.shaken.saturating_sub(worn.min(u8::MAX as u32) as u8)
    }
}

impl Memory for Marks {
    fn alarm_term(&self, man: ManId, tick: u32) -> Parts {
        let mut parts = NO_PARTS;
        if self.rung >= Rung::HasFled && self.has_fled(man) {
            parts[0] = (self.has_fled, Cause::HasFled);
        }
        if self.rung >= Rung::Shaken {
            parts[1] = (self.shaken * self.shaken(man, tick) as i32, Cause::Shaken);
        }
        parts
    }

    /// Rung 2 only: each fall he sees, and the fall of his standard or his leader, raises
    /// *shaken* a level. A rule, not a draw.
    fn absorb(&mut self, man: ManId, v: &View, _: &Sums) {
        if self.rung < Rung::Shaken {
            return;
        }
        let frights = v.shocks.falls.min(SHAKEN_MAX)
            + v.shocks.standard_fell as u8
            + v.shocks.leader_fell as u8;
        if frights == 0 {
            return;
        }
        let now = self.shaken(man, v.tick);
        let m = &mut self.men[man.0 as usize];
        m.shaken = (now + frights).min(SHAKEN_MAX);
        m.shaken_tick = v.tick;
    }

    fn on_event(&mut self, e: &Event) {
        let turned = matches!(
            e.kind,
            EventKind::Posture {
                to: Posture::Turned,
                ..
            }
        );
        if turned && self.rung >= Rung::HasFled {
            self.men[e.man.0 as usize].has_fled = true;
        }
    }

    fn hash(&self, h: &mut StateHasher) {
        for m in &self.men {
            h.write_u64(m.has_fled as u64 | (m.shaken as u64) << 8 | (m.shaken_tick as u64) << 32);
        }
    }
}
