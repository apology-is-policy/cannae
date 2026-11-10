//! Rung 3 of the memory ladder: fear and resolve (docs/slice0/rungs-3-4.md).
//!
//! Two gauges per man. **Fear** rises while he sees frightening acts and decays when he does
//! not, so a fright outlasts its cause by minutes. **Resolve** is spent by fear while he is
//! awake to something and comes back slowly, so a long fight wears him down. Together they add
//! one term to his alarm: fear, plus a share of his nerve for the resolve he has spent.
//!
//! [`Rung3`] is the rung as a man's [`Memory`] in `cannae-man`: rung 1's *has fled*, with fear
//! and resolve in place of rung 2's *shaken*.
//!
//! Dormancy is exact. Both gauges are stored with the tick they were last written and read
//! forward in closed form, and `absorb` is called only at steps the man is awake, by the
//! visible layer's own wake test (the hook's rule 4), so stepping every man and skipping the
//! dormant ones give the same bits. Nothing here draws a random number.

use cannae_core::fixed::exp_neg_q32;
use cannae_core::hash::StateHasher;
use cannae_core::schedule::{BASE_TICK, MAN_STEP};
use cannae_core::{Cause, Event, ManId, View};
use cannae_man::{Marks, Memory, Parts, Rung, Sums};

/// Fear is held in alarm units with 24 fractional bits: a step's increment is far below one
/// unit, and would vanish at the visible layer's 8 bits.
const FEAR_FRAC: u32 = 24;
/// Resolve is held in parts per million of full.
const FULL: i64 = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Params {
    /// How long fear lasts: its time constant, in ms.
    pub fear_tau_ms: u32,
    /// How much of a fright's weight he keeps carrying, per mille: under frightening acts held
    /// at A, fear settles at `keep_pm` / 1000 times A.
    pub keep_pm: u32,
    /// How long a fear of one alarm unit takes to empty full resolve, in ms.
    pub spend_ms: u32,
    /// How long resolve takes to come back: its time constant, in ms.
    pub resolve_tau_ms: u32,
    /// Alarm at empty resolve, in alarm units with 8 fractional bits.
    pub empty_q8: i32,
}

impl Params {
    /// The centres of the priors in docs/slice0/rungs-3-4.md, near enough: fear lasts about
    /// three minutes and is carried at five eighths of its weight; resolve is spent over
    /// half an hour of steady fear and comes back over half an hour; empty, it costs two units
    /// of alarm.
    pub const CENTRE: Params = Params {
        fear_tau_ms: 180_000,
        keep_pm: 625,
        spend_ms: 1_800_000,
        resolve_tau_ms: 1_800_000,
        empty_q8: 512,
    };
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
struct Man {
    fear: i64,
    fear_tick: u32,
    /// Resolve spent, in parts per million of full: zero is full.
    spent: i64,
    spent_tick: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Gauges {
    pub params: Params,
    men: Vec<Man>,
}

fn ms(ticks: u32) -> u64 {
    ticks as u64 * BASE_TICK.0 as u64
}

/// exp(-t / tau) in Q32.
fn keep(t_ms: u64, tau_ms: u32) -> u64 {
    exp_neg_q32((t_ms << 32) / tau_ms.max(1) as u64)
}

impl Gauges {
    pub fn new(params: Params, men: usize) -> Gauges {
        Gauges {
            params,
            men: vec![Man::default(); men],
        }
    }

    fn fear_at(&self, m: &Man, tick: u32) -> i64 {
        let k = keep(
            ms(tick.saturating_sub(m.fear_tick)),
            self.params.fear_tau_ms,
        );
        ((m.fear as i128 * k as i128) >> 32) as i64
    }

    fn spent_at(&self, m: &Man, tick: u32) -> i64 {
        let k = keep(
            ms(tick.saturating_sub(m.spent_tick)),
            self.params.resolve_tau_ms,
        );
        ((m.spent as i128 * k as i128) >> 32) as i64
    }

    /// His fear at `tick`, in alarm units with 8 fractional bits.
    pub fn fear_q8(&self, man: usize, tick: u32) -> i32 {
        (self.fear_at(&self.men[man], tick) >> (FEAR_FRAC - 8)) as i32
    }

    /// His resolve at `tick`, per mille of full.
    pub fn resolve_pm(&self, man: usize, tick: u32) -> u32 {
        ((FULL - self.spent_at(&self.men[man], tick)) / 1000) as u32
    }

    /// The two parts of his alarm term at `tick`, with their causes. A part of zero names no
    /// cause.
    pub fn alarm_term(&self, man: usize, tick: u32) -> [(i32, Cause); 2] {
        let m = &self.men[man];
        let fear = (self.fear_at(m, tick) >> (FEAR_FRAC - 8)) as i32;
        let worn = (self.params.empty_q8 as i64 * self.spent_at(m, tick) / FULL) as i32;
        [
            (fear, if fear > 0 { Cause::Fear } else { Cause::None }),
            (
                worn,
                if worn > 0 {
                    Cause::Resolve
                } else {
                    Cause::None
                },
            ),
        ]
    }

    /// One man step, at which he was awake, of what he saw: the visible layer's sum of
    /// frightening acts before his nerve, in alarm units with 8 fractional bits.
    pub fn absorb(&mut self, man: usize, tick: u32, acts_q8: i32) {
        let p = self.params;
        let fear = self.fear_at(&self.men[man], tick);
        let spent = self.spent_at(&self.men[man], tick);
        let step = MAN_STEP.0 as i64;
        // Fear rises at keep / tau times the acts, so that held acts settle at keep times
        // their weight.
        let rise = ((acts_q8.max(0) as i64) << (FEAR_FRAC - 8)) * p.keep_pm as i64 * step
            / (1000 * p.fear_tau_ms.max(1) as i64);
        // A fear of one unit spends full resolve in `spend_ms`.
        let spend = (fear as i128 * FULL as i128 * step as i128
            / ((p.spend_ms.max(1) as i128) << FEAR_FRAC)) as i64;
        let m = &mut self.men[man];
        m.fear = fear + rise;
        m.fear_tick = tick;
        m.spent = (spent + spend).min(FULL);
        m.spent_tick = tick;
    }

    /// Everything a step could change, for the per-step state hash.
    pub fn hash_parts(&self) -> impl Iterator<Item = u64> + '_ {
        self.men.iter().flat_map(|m| {
            [
                m.fear as u64,
                m.spent as u64,
                (m.fear_tick as u64) << 32 | m.spent_tick as u64,
            ]
        })
    }
}

/// Rung 3 as a man's memory: rung 1's *has fled*, and fear and resolve in place of rung 2's
/// *shaken*.
#[derive(Clone, Debug)]
pub struct Rung3 {
    pub marks: Marks,
    pub gauges: Gauges,
}

impl Rung3 {
    pub fn new(man: &cannae_man::Params, gauges: Params, men: usize) -> Rung3 {
        Rung3 {
            marks: Marks::new(Rung::HasFled, man, men),
            gauges: Gauges::new(gauges, men),
        }
    }
}

impl Memory for Rung3 {
    const READS_SUMS: bool = true;

    fn alarm_term(&self, man: ManId, tick: u32) -> Parts {
        // The marks fill the first two parts, and at rung 1 the second is always empty.
        let mut parts = self.marks.alarm_term(man, tick);
        let [fear, worn] = self.gauges.alarm_term(man.0 as usize, tick);
        parts[2] = fear;
        parts[3] = worn;
        parts
    }

    fn absorb(&mut self, man: ManId, v: &View, sums: &Sums) {
        self.marks.absorb(man, v, sums);
        self.gauges.absorb(man.0 as usize, v.tick, sums.acts);
    }

    fn on_event(&mut self, e: &Event) {
        self.marks.on_event(e);
    }

    fn hash(&self, h: &mut StateHasher) {
        self.marks.hash(h);
        for x in self.gauges.hash_parts() {
            h.write_u64(x);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cannae_core::schedule::MAN_EVERY;

    const P: Params = Params::CENTRE;
    const UNIT: i32 = 256;

    /// Steps one man for `steps` man steps from `tick`, seeing `acts` and awake throughout.
    fn watch(g: &mut Gauges, tick: &mut u32, steps: u32, acts_q8: i32) {
        for _ in 0..steps {
            *tick += MAN_EVERY;
            g.absorb(0, *tick, acts_q8);
        }
    }

    fn steps_in(ms: u32) -> u32 {
        ms / MAN_STEP.0
    }

    #[test]
    fn fear_settles_at_its_share_of_what_he_sees_and_then_fades() {
        let mut g = Gauges::new(P, 1);
        let mut t = 0;
        // Two units of frightening acts, held for ten time constants.
        watch(&mut g, &mut t, steps_in(10 * P.fear_tau_ms), 2 * UNIT);
        let settled = g.fear_q8(0, t);
        let want = 2 * UNIT * P.keep_pm as i32 / 1000;
        assert!(
            (settled - want).abs() <= want / 50,
            "{settled} against {want}"
        );
        // Then nothing frightening, and he is not awake to anything: one time constant later
        // about 37 % is left.
        let later = t + P.fear_tau_ms / BASE_TICK.0;
        let left = g.fear_q8(0, later) as i64 * 1000 / settled as i64;
        assert!((360..=376).contains(&left), "{left} per mille");
    }

    #[test]
    fn a_long_fright_spends_resolve_and_rest_brings_it_back() {
        let mut g = Gauges::new(P, 1);
        let mut t = 0;
        assert_eq!(g.resolve_pm(0, t), 1000);
        // Under steady acts for an hour he loses most of his resolve; the alarm term grows
        // toward the cost of empty resolve.
        watch(&mut g, &mut t, steps_in(3_600_000), 2 * UNIT);
        let low = g.resolve_pm(0, t);
        assert!(low < 300, "{low} per mille left after an hour");
        let [(_, c1), (worn, c2)] = g.alarm_term(0, t);
        assert_eq!((c1, c2), (Cause::Fear, Cause::Resolve));
        assert!(worn > P.empty_q8 * 7 / 10);
        // Two hours out of it, he is nearly whole again.
        let rested = g.resolve_pm(0, t + 7_200_000 / BASE_TICK.0);
        assert!(rested > 900, "{rested} per mille after rest");
    }

    #[test]
    fn an_untouched_man_has_no_term() {
        let g = Gauges::new(P, 1);
        assert_eq!(
            g.alarm_term(0, 1_000_000),
            [(0, Cause::None), (0, Cause::None)]
        );
    }
}
