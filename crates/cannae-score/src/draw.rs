//! The parameter sets of a cell ([preregistration](../../docs/slice0/preregistration.md), "The
//! procedure" and "Common draws").
//!
//! A drawn value is a function of the freeze seed, the name of its line in a priors file, and
//! the number of the draw, and of nothing else: not of the cell, and not of what was drawn
//! before it. So draw *i* has the same shared values in every cell, and the same weights at
//! every rung, by construction. Adding a line later moves no other line.
//!
//! Everything here is integers, in each parameter's stored unit, so the sets are the same on
//! any machine.
//!
//! **Who maps what.** Each priors file's author writes the function that fills his structs
//! from a [`Draw`]: Paullus for stamina, power, gait, the pairwise parameters and the gauges;
//! Varro for ties, bouts, kits, the man, the troops and the field. Until a mapping is written
//! its values are the centres, whatever the draw.

use crate::Engine;
use cannae_core::fixed::exp_neg_q32;
use cannae_core::power::GaitSpeeds;
use cannae_core::rng::philox4x32_10;
use cannae_core::{BoutParams, Tie};
use cannae_harness::scenario::TieSpec;
use cannae_man::{Rung, Troop};

/// One cell of the experiment: an engine, and a rung of the memory ladder, 0 to 3.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell {
    pub engine: Engine,
    pub rung: u8,
}

/// How a line of a priors file is drawn. The bounds are in the parameter's stored unit and
/// both are included.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prior {
    /// Every whole value from the first to the second is as likely as any other.
    Uniform(i64, i64),
    /// The logarithm is uniform. Both bounds are above zero.
    LogUniform(i64, i64),
}

/// A line of a priors file: its name, which is its address among the draws, and its prior.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line {
    pub name: &'static str,
    pub prior: Prior,
}

/// One of the parameter sets: the source of its numbers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Draw {
    seed: u64,
    i: u32,
}

/// Marks the words drawn here off from every other use of the generator.
const PURPOSE: u32 = 0x4452_4157;

/// ln 2 in Q32.
const LN2_Q32: u64 = 2_977_044_472;

impl Draw {
    pub fn new(freeze_seed: u64, i: u32) -> Draw {
        Draw {
            seed: freeze_seed,
            i,
        }
    }

    /// The number of this draw.
    pub fn index(&self) -> u32 {
        self.i
    }

    /// The line's value in this draw.
    pub fn of(&self, line: &Line) -> i64 {
        match line.prior {
            Prior::Uniform(lo, hi) => self.uniform(line.name, lo, hi),
            Prior::LogUniform(lo, hi) => self.log_uniform(line.name, lo, hi),
        }
    }

    /// A whole number from `lo` to `hi`, both included, each as likely as any other.
    pub fn uniform(&self, line: &str, lo: i64, hi: i64) -> i64 {
        assert!(lo <= hi, "{line}: a prior from {lo} to {hi}");
        let span = (hi - lo) as u128 + 1;
        lo + ((self.word(line) as u128 * span) >> 64) as i64
    }

    /// A whole number from `lo` to `hi` whose logarithm is uniform, rounded to the nearest.
    /// The unit should be fine enough that the rounding does not show.
    pub fn log_uniform(&self, line: &str, lo: i64, hi: i64) -> i64 {
        assert!(
            0 < lo && lo <= hi && hi >> 30 < lo,
            "{line}: a log-uniform prior from {lo} to {hi}"
        );
        // hi * exp(-u ln(hi / lo)), with u uniform from 0 to 1.
        let ln = ln_ratio_q32(lo as u64, hi as u64) as u128;
        let x = ((self.word(line) >> 32) as u128 * ln) >> 32;
        let v = ((hi as u128 * exp_neg_q32(x as u64) as u128 + (1 << 31)) >> 32) as i64;
        v.clamp(lo, hi)
    }

    /// Sixty-four bits that depend on the seed, the line's name and the draw's number.
    fn word(&self, line: &str) -> u64 {
        let name = fnv1a(line);
        let w = philox4x32_10(
            [self.i, name as u32, (name >> 32) as u32, PURPOSE],
            [self.seed as u32, (self.seed >> 32) as u32],
        );
        (w[0] as u64) << 32 | w[1] as u64
    }
}

/// The 64-bit FNV-1a hash of a line's name.
pub fn fnv1a(s: &str) -> u64 {
    s.bytes().fold(0xCBF2_9CE4_8422_2325, |h, b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3)
    })
}

/// ln(hi / lo) in Q32, for 0 < lo <= hi, to within a few parts in 2^32.
fn ln_ratio_q32(lo: u64, hi: u64) -> u64 {
    // Double the lower end until the ratio is under 2, counting the doublings. What is left
    // is found by halving an interval on the exponential, which only falls.
    let (mut lo, hi) = (lo as u128, hi as u128);
    let mut twos = 0;
    while hi >= 2 * lo {
        lo *= 2;
        twos += 1;
    }
    // The largest x with exp(-x) >= lo / hi. It is under 1, since the ratio is under 2.
    let enough = |x: u64| exp_neg_q32(x) as u128 * hi >= lo << 32;
    let (mut yes, mut no) = (0u64, 1u64 << 32);
    while no - yes > 1 {
        let mid = yes + (no - yes) / 2;
        if enough(mid) {
            yes = mid;
        } else {
            no = mid;
        }
    }
    yes + twos * LN2_Q32
}

/// The substrate's own parameters, for the engine of the cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SubstrateParams {
    Pairwise(cannae_pairwise::Params),
    Field(cannae_field::Params),
}

/// Everything a run needs that is drawn: one parameter set of one cell.
#[derive(Clone, Debug, PartialEq)]
pub struct Drawn {
    pub cell: Cell,
    /// The number of the draw.
    pub i: u32,
    pub speeds: GaitSpeeds,
    pub bouts: BoutParams,
    /// The weights and thresholds of the visible layer: the same at every rung.
    pub man: cannae_man::Params,
    /// Read at rung 3 only.
    pub gauges: cannae_gauges::Params,
    pub substrate: SubstrateParams,
    drilled: Tie,
    loose: Tie,
    troops: [(&'static str, Troop); 4],
}

impl Drawn {
    /// The rung of the marks, 0 to 2; `None` at rung 3, whose memory is the gauges.
    pub fn rung(&self) -> Option<Rung> {
        match self.cell.rung {
            0 => Some(Rung::NoMarks),
            1 => Some(Rung::HasFled),
            2 => Some(Rung::Shaken),
            _ => None,
        }
    }

    /// The troop a scenario names, as this set has it.
    pub fn troop(&self, name: &str) -> Option<Troop> {
        self.troops
            .iter()
            .find(|(n, _)| *n == name)
            .map(|&(_, t)| t)
    }

    /// The tie a scenario names, as this set has it.
    pub fn tie(&self, spec: TieSpec) -> Tie {
        match spec {
            TieSpec::Drilled => self.drilled,
            TieSpec::Loose => self.loose,
        }
    }
}

/// The names a scenario may give a troop.
const TROOPS: [&str; 4] = [
    "hastati",
    "hastati-veterans",
    "hastati-recruits",
    "gallic-warband",
];

/// Parameter set `i` of a cell.
pub fn drawn(freeze_seed: u64, cell: Cell, i: u32) -> Result<Drawn, String> {
    if cell.rung > 3 {
        return Err(format!("no rung {}", cell.rung));
    }
    let d = Draw::new(freeze_seed, i);
    Ok(Drawn {
        cell,
        i,
        speeds: speeds(&d),
        bouts: bouts(&d),
        man: man(&d),
        gauges: gauges(&d),
        substrate: match cell.engine {
            Engine::Pairwise => SubstrateParams::Pairwise(pairwise(&d)),
            Engine::Field => SubstrateParams::Field(field(&d)),
        },
        drilled: tie(&d, TieSpec::Drilled),
        loose: tie(&d, TieSpec::Loose),
        troops: TROOPS.map(|name| (name, troop(&d, name))),
    })
}

// The mappings. Each is at its centre until its author writes it (the module's head says who).

/// priors-shared.md, "Stamina, power and gait" (Paullus).
fn speeds(_d: &Draw) -> GaitSpeeds {
    GaitSpeeds::PLACEHOLDER
}

/// priors-shared.md, "Bouts" (Varro).
fn bouts(_d: &Draw) -> BoutParams {
    BoutParams::PLACEHOLDER
}

/// priors-shared.md, "Ties" (Varro).
fn tie(_d: &Draw, spec: TieSpec) -> Tie {
    match spec {
        TieSpec::Drilled => Tie::DRILLED,
        TieSpec::Loose => Tie::LOOSE,
    }
}

/// priors-field.md, §1 (Varro).
fn man(_d: &Draw) -> cannae_man::Params {
    cannae_man::Params::CENTRE
}

/// priors-field.md, §2, with the kit and the stamina of priors-shared.md (Varro and Paullus).
fn troop(_d: &Draw, name: &str) -> Troop {
    Troop::named(name).unwrap_or(Troop::HASTATI)
}

/// priors-gauges.md (Paullus).
fn gauges(_d: &Draw) -> cannae_gauges::Params {
    cannae_gauges::Params::CENTRE
}

/// priors-pairwise.md, and the crowd physics of priors-shared.md (Paullus).
fn pairwise(_d: &Draw) -> cannae_pairwise::Params {
    cannae_pairwise::Params::PLACEHOLDER
}

/// priors-field.md, §4, and the crowd physics of priors-shared.md (Varro).
fn field(_d: &Draw) -> cannae_field::Params {
    cannae_field::Params::CENTRE
}

#[cfg(test)]
mod tests {
    use super::*;

    const SEED: u64 = 20_261_110;

    #[test]
    fn a_value_depends_on_the_seed_the_line_and_the_draw_and_nothing_else() {
        let d = Draw::new(SEED, 7);
        let v = d.uniform("man.back", 0, 1 << 40);
        assert_eq!(v, Draw::new(SEED, 7).uniform("man.back", 0, 1 << 40));
        assert_ne!(v, Draw::new(SEED, 8).uniform("man.back", 0, 1 << 40));
        assert_ne!(v, Draw::new(SEED + 1, 7).uniform("man.back", 0, 1 << 40));
        assert_ne!(v, d.uniform("man.fall", 0, 1 << 40));
        // A line read through its table entry is the same value.
        let line = Line {
            name: "man.back",
            prior: Prior::Uniform(0, 1 << 40),
        };
        assert_eq!(d.of(&line), v);
    }

    #[test]
    fn a_uniform_line_covers_its_range_evenly() {
        // Ten values, 20,000 draws: each comes up about 2,000 times.
        let mut count = [0u32; 10];
        for i in 0..20_000 {
            let v = Draw::new(SEED, i).uniform("a line", -3, 6);
            count[(v + 3) as usize] += 1;
        }
        // Three standard deviations of a count of 2,000 are about 130.
        assert!(
            count.iter().all(|&c| (1_850..=2_150).contains(&c)),
            "{count:?}"
        );
        // One value is one value.
        assert_eq!(Draw::new(SEED, 0).uniform("a line", 5, 5), 5);
        // The widest range does not overflow.
        let v = Draw::new(SEED, 0).uniform("a line", i64::MIN / 2, i64::MAX / 2);
        assert!((i64::MIN / 2..=i64::MAX / 2).contains(&v));
    }

    #[test]
    fn two_lines_are_drawn_independently() {
        // Of 20,000 draws, about a quarter fall in the lower half of both lines.
        let both = (0..20_000)
            .filter(|&i| {
                let d = Draw::new(SEED, i);
                d.uniform("one", 0, 999) < 500 && d.uniform("two", 0, 999) < 500
            })
            .count();
        assert!((4_800..=5_200).contains(&both), "{both}");
    }

    #[test]
    #[allow(clippy::float_arithmetic)]
    fn the_logarithm_of_a_ratio_is_right() {
        for (lo, hi) in [
            (1, 1),
            (1_000, 1_001),
            (30_000, 600_000),
            (1, 5),
            (50, 200),
            (3, 1 << 40),
        ] {
            let want = (hi as f64 / lo as f64).ln();
            let got = ln_ratio_q32(lo, hi) as f64 / (1u64 << 32) as f64;
            assert!(
                (got - want).abs() < 1e-8,
                "ln({hi}/{lo}): {got} against {want}"
            );
        }
    }

    #[test]
    fn a_log_uniform_line_is_uniform_in_its_logarithm() {
        // From 30 s to 600 s in ms. The geometric mean is 134.2 s: half the draws fall below
        // it, a quarter below the geometric mean of that and the low end, and so on.
        let (lo, hi) = (30_000, 600_000);
        let cuts = [63_442, 134_164, 283_723];
        let mut below = [0u32; 3];
        for i in 0..20_000 {
            let v = Draw::new(SEED, i).log_uniform("a time", lo, hi);
            assert!((lo..=hi).contains(&v));
            for (k, &c) in cuts.iter().enumerate() {
                below[k] += (v < c) as u32;
            }
        }
        for (k, want) in [5_000, 10_000, 15_000].into_iter().enumerate() {
            assert!(below[k].abs_diff(want) <= 220, "{below:?}");
        }
        assert_eq!(Draw::new(SEED, 0).log_uniform("a time", 77, 77), 77);
    }

    #[test]
    fn a_set_is_the_same_in_every_cell_but_for_what_the_cell_adds() {
        let cells = [Engine::Pairwise, Engine::Field]
            .into_iter()
            .flat_map(|engine| (0..4).map(move |rung| Cell { engine, rung }));
        let first = drawn(
            SEED,
            Cell {
                engine: Engine::Field,
                rung: 0,
            },
            11,
        )
        .unwrap();
        for cell in cells {
            let s = drawn(SEED, cell, 11).unwrap();
            assert_eq!(s.speeds, first.speeds);
            assert_eq!(s.bouts, first.bouts);
            assert_eq!(s.man, first.man);
            assert_eq!(s.gauges, first.gauges);
            assert_eq!(s.tie(TieSpec::Loose), first.tie(TieSpec::Loose));
            assert_eq!(s.troop("gallic-warband"), first.troop("gallic-warband"));
            assert_eq!(s.rung().is_none(), cell.rung == 3);
            match (cell.engine, s.substrate) {
                (Engine::Pairwise, SubstrateParams::Pairwise(_)) => {}
                (Engine::Field, SubstrateParams::Field(p)) => {
                    assert_eq!(SubstrateParams::Field(p), first.substrate)
                }
                other => panic!("{other:?}"),
            }
        }
        assert!(
            drawn(
                SEED,
                Cell {
                    engine: Engine::Field,
                    rung: 4
                },
                0
            )
            .is_err()
        );
        assert_eq!(first.troop("nobody"), None);
        assert!(TROOPS.iter().all(|n| first.troop(n) == Troop::named(n)));
    }
}
