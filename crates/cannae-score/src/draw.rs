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
use cannae_core::power::{GaitSpeeds, PowerTable};
use cannae_core::rng::philox4x32_10;
use cannae_core::stamina::StaminaParams;
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
    /// What each activity costs, as a multiple of a man's critical power.
    pub power: PowerTable,
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
        power: power(&d),
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

// Paullus's lines. Each name is its line's address among the draws, so it never changes
// after the freeze. Values are in the stored unit of the field they fill.

/// priors-shared.md, "Stamina, power and gait": W'/CP, 50-100 s.
pub const RESERVE: Line = Line {
    name: "stamina.reserve_us",
    prior: Prior::Uniform(50_000_000, 100_000_000),
};
/// The scale on Skiba's recovery time constant, 0.7-1.4.
pub const TAU_SCALE: Line = Line {
    name: "stamina.tau_scale_pm",
    prior: Prior::Uniform(700, 1_400),
};
/// Walking at 1.3 m/s in kit, 0.40-0.60 CP.
pub const WALK_CP: Line = Line {
    name: "power.walk_pm",
    prior: Prior::Uniform(400, 600),
};
/// Running at the run gait, 1.2-1.6 CP.
pub const RUN_CP: Line = Line {
    name: "power.run_pm",
    prior: Prior::Uniform(1_200, 1_600),
};
/// Leaning into the men in front, 0.9-1.8 CP.
pub const PRESS_CP: Line = Line {
    name: "power.press_pm",
    prior: Prior::Uniform(900, 1_800),
};
/// A man forward, striking and warding, 1.1-2.0 CP.
pub const FIGHT_CP: Line = Line {
    name: "power.fight_pm",
    prior: Prior::Uniform(1_100, 2_000),
};
/// The run gait, 3-5 m/s.
pub const RUN_SPEED: Line = Line {
    name: "gait.run_mm_s",
    prior: Prior::Uniform(3_000, 5_000),
};

/// priors-shared.md, "Crowd physics both substrates have": drawn once for both, and each
/// engine maps them onto its own mechanics. The density at which crushing begins, 5-8 men per
/// m², in Q8.
pub const CRUSH_ONSET: Line = Line {
    name: "crowd.crush_onset_q8",
    prior: Prior::Uniform(1_280, 2_048),
};
/// The mean time to be disabled at 2 men per m² above the onset, 30-600 s.
pub const CRUSH_TIME: Line = Line {
    name: "crowd.crush_time_ms",
    prior: Prior::LogUniform(30_000, 600_000),
};
/// The fastest turn, 120-200 degrees per second, in turn units per base tick.
pub const TURN: Line = Line {
    name: "crowd.turn_per_tick",
    prior: Prior::Uniform(1_092, 1_820),
};
/// How fast a man at full press edges forward, 200-700 mm/s.
pub const PRESS_SPEED: Line = Line {
    name: "crowd.press_mm_s",
    prior: Prior::Uniform(200, 700),
};

/// priors-pairwise.md, the substrate.
pub const RADIUS: Line = Line {
    name: "pairwise.radius_mm",
    prior: Prior::Uniform(280, 300),
};
pub const BRACE: Line = Line {
    name: "pairwise.brace_mm_s",
    prior: Prior::Uniform(300, 1_200),
};
pub const CORE: Line = Line {
    name: "pairwise.core_mm",
    prior: Prior::Uniform(100, 200),
};
pub const STIFFNESS: Line = Line {
    name: "pairwise.stiffness_q8",
    prior: Prior::Uniform(64, 192),
};

/// priors-pairwise.md, rung 3: read at rung 3 on both engines.
pub const FEAR_TAU: Line = Line {
    name: "gauges.fear_tau_ms",
    prior: Prior::LogUniform(60_000, 600_000),
};
pub const KEEP: Line = Line {
    name: "gauges.keep_pm",
    prior: Prior::Uniform(250, 1_000),
};
pub const SPEND: Line = Line {
    name: "gauges.spend_ms",
    prior: Prior::LogUniform(600_000, 3_600_000),
};
pub const RESOLVE_TAU: Line = Line {
    name: "gauges.resolve_tau_ms",
    prior: Prior::LogUniform(600_000, 3_600_000),
};
pub const EMPTY: Line = Line {
    name: "gauges.empty_q8",
    prior: Prior::Uniform(256, 768),
};

/// Every line of Paullus's, for the checks.
pub const PAULLUS_LINES: [&Line; 20] = [
    &RESERVE,
    &TAU_SCALE,
    &WALK_CP,
    &RUN_CP,
    &PRESS_CP,
    &FIGHT_CP,
    &RUN_SPEED,
    &CRUSH_ONSET,
    &CRUSH_TIME,
    &TURN,
    &PRESS_SPEED,
    &RADIUS,
    &BRACE,
    &CORE,
    &STIFFNESS,
    &FEAR_TAU,
    &KEEP,
    &SPEND,
    &RESOLVE_TAU,
    &EMPTY,
];

/// priors-shared.md, "Stamina, power and gait" (Paullus). The walk is fixed at Vegetius's
/// military step.
fn speeds(d: &Draw) -> GaitSpeeds {
    GaitSpeeds {
        walk: 1_300,
        run: d.of(&RUN_SPEED) as i32,
    }
}

/// The same section: rest and standing are fixed at 0.09 and 0.14 CP.
fn power(d: &Draw) -> PowerTable {
    PowerTable {
        pm: [
            90,
            140,
            d.of(&WALK_CP) as u32,
            d.of(&RUN_CP) as u32,
            d.of(&PRESS_CP) as u32,
            d.of(&FIGHT_CP) as u32,
        ],
    }
}

/// The same section: one wind for every troop (Livy's Gauls fading at Sentinum would be a
/// variant, not a prior).
fn stamina(d: &Draw) -> StaminaParams {
    StaminaParams {
        reserve_us: d.of(&RESERVE) as u32,
        tau_scale_pm: d.of(&TAU_SCALE) as u32,
    }
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
fn troop(d: &Draw, name: &str) -> Troop {
    Troop {
        stamina: stamina(d),
        ..Troop::named(name).unwrap_or(Troop::HASTATI)
    }
}

/// priors-pairwise.md, rung 3 (Paullus).
fn gauges(d: &Draw) -> cannae_gauges::Params {
    cannae_gauges::Params {
        fear_tau_ms: d.of(&FEAR_TAU) as u32,
        keep_pm: d.of(&KEEP) as u32,
        spend_ms: d.of(&SPEND) as u32,
        resolve_tau_ms: d.of(&RESOLVE_TAU) as u32,
        empty_q8: d.of(&EMPTY) as i32,
    }
}

/// priors-pairwise.md, and the crowd physics of priors-shared.md (Paullus). Relaxation passes
/// are fixed at 2.
fn pairwise(d: &Draw) -> cannae_pairwise::Params {
    let radius_mm = d.of(&RADIUS) as i32;
    let onset_q8 = d.of(&CRUSH_ONSET);
    cannae_pairwise::Params {
        radius_mm,
        iterations: 2,
        stiffness_q8: d.of(&STIFFNESS) as i32,
        turn_per_tick: d.of(&TURN) as u16,
        press_mm_s: d.of(&PRESS_SPEED) as i32,
        brace_mm_s: d.of(&BRACE) as i32,
        crush_mm: crush_mm(radius_mm, onset_q8),
        crush_rate_q32_per_mm: crush_rate_q32_per_mm(d.of(&CRUSH_TIME), onset_q8),
        core_mm: d.of(&CORE) as i32,
    }
}

/// The spacing of discs packed hexagonally at a density in men per m², Q8, in hundredths of
/// a mm: the square root of 2 / (√3 ρ).
fn hex_spacing_cmm(density_q8: i64) -> i64 {
    // 2 m² × 256 × 10^6 mm² per m² × 10^4, over √3 × 10^6 times the density.
    let s2 = 5_120_000_000_000_000_000u64 / (1_732_051 * density_q8 as u64);
    s2.isqrt() as i64
}

/// The summed overlap with six neighbours at which crushing begins: six contacts' overlap at
/// the onset density in hexagonal packing, 6 (2r − s).
fn crush_mm(radius_mm: i32, onset_q8: i64) -> i32 {
    ((6 * (200 * radius_mm as i64 - hex_spacing_cmm(onset_q8))) / 100) as i32
}

/// The crush rate per mm of overlap beyond `crush_mm`, Q32 per second, such that a man is
/// disabled in `time_ms` on average at 2 men per m² above the onset. The extra overlap there
/// does not depend on the radius.
fn crush_rate_q32_per_mm(time_ms: i64, onset_q8: i64) -> u32 {
    let extra_cmm = 6 * (hex_spacing_cmm(onset_q8) - hex_spacing_cmm(onset_q8 + 512));
    ((1u64 << 32) * 100_000 / (time_ms as u64 * extra_cmm.max(1) as u64)).min(u32::MAX as u64)
        as u32
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
        // Each troop is the named one, but for its drawn wind.
        assert!(TROOPS.iter().all(|n| {
            first.troop(n).map(|t| Troop {
                stamina: StaminaParams::PLACEHOLDER,
                ..t
            }) == Troop::named(n).map(|t| Troop {
                stamina: StaminaParams::PLACEHOLDER,
                ..t
            })
        }));
    }

    #[test]
    fn paullus_lines_have_distinct_names_and_stay_in_their_priors() {
        let mut names: Vec<&str> = PAULLUS_LINES.iter().map(|l| l.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), PAULLUS_LINES.len());
        for line in PAULLUS_LINES {
            let (lo, hi) = match line.prior {
                Prior::Uniform(lo, hi) | Prior::LogUniform(lo, hi) => (lo, hi),
            };
            let (mut least, mut most) = (i64::MAX, i64::MIN);
            for i in 0..2_000 {
                let v = Draw::new(SEED, i).of(line);
                least = least.min(v);
                most = most.max(v);
            }
            assert!(lo <= least && most <= hi, "{}: {least}-{most}", line.name);
            // Two thousand draws come within a twentieth of each end.
            let reach = (hi - lo) / 20;
            assert!(least <= lo + reach && most >= hi - reach, "{}", line.name);
        }
    }

    #[test]
    fn the_crush_lines_give_the_placeholders_at_the_centres() {
        // Onset 6.5 per m² at r = 280: the placeholder's 831 mm and 134 s at 8.5 per m².
        assert!((829..=833).contains(&crush_mm(280, 1_664)));
        let rate = crush_rate_q32_per_mm(134_000, 1_664) as i64;
        assert!((rate - 100_830).abs() <= 500, "{rate}");
        // The corners of the onset prior at r = 280, as priors-shared.md gives them.
        assert!((474..=479).contains(&crush_mm(280, 1_280)));
        assert!((1_078..=1_083).contains(&crush_mm(280, 2_048)));
        // Slower to disable at a later onset is not implied, but faster at a shorter time is.
        assert!(crush_rate_q32_per_mm(30_000, 1_664) > crush_rate_q32_per_mm(600_000, 1_664));
    }

    #[test]
    fn a_pairwise_set_fills_every_line_of_paullus() {
        let s = drawn(
            SEED,
            Cell {
                engine: Engine::Pairwise,
                rung: 3,
            },
            5,
        )
        .unwrap();
        let d = Draw::new(SEED, 5);
        let SubstrateParams::Pairwise(p) = s.substrate else {
            panic!("not pairwise")
        };
        assert_eq!(p.radius_mm as i64, d.of(&RADIUS));
        assert_eq!(p.core_mm as i64, d.of(&CORE));
        assert_eq!(p.press_mm_s as i64, d.of(&PRESS_SPEED));
        assert_eq!(p.crush_mm, crush_mm(p.radius_mm, d.of(&CRUSH_ONSET)));
        assert_eq!(s.speeds.run as i64, d.of(&RUN_SPEED));
        assert_eq!(s.power.pm[5] as i64, d.of(&FIGHT_CP));
        assert_eq!(s.gauges.spend_ms as i64, d.of(&SPEND));
        let t = s.troop("hastati").unwrap();
        assert_eq!(t.stamina.reserve_us as i64, d.of(&RESERVE));
        assert_eq!(s.troop("gallic-warband").unwrap().stamina, t.stamina);
    }
}
