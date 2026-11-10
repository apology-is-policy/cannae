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
//! Varro for ties, bouts, kits, the man, the troops and the field.

use crate::Engine;
use cannae_core::fixed::{Prob, exp_neg_q32};
use cannae_core::power::{GaitSpeeds, PowerTable};
use cannae_core::rng::philox4x32_10;
use cannae_core::schedule::BASE_TICK;
use cannae_core::stamina::StaminaParams;
use cannae_core::{BoutParams, Tie};
use cannae_harness::scenario::TieSpec;
use cannae_man::{Rung, Spread, Troop};

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

/// Declares lines and gathers them, in the order written, into one list.
macro_rules! lines {
    ($all:ident: $($(#[$doc:meta])* $id:ident = $name:literal, $prior:expr;)*) => {
        $($(#[$doc])* pub const $id: Line = Line { name: $name, prior: $prior };)*
        /// Every line of Varro's, for the checks and the table of record.
        pub const $all: &[&Line] = &[$(&$id),*];
    };
}

// Varro's lines. Weights of the man and of the troops are in Q8: 256 is one turned man of
// alarm, or one forward man of urge. Times are in ms unless the name says ticks.
use Prior::{LogUniform as L, Uniform as U};
lines! { VARRO_LINES:
    // priors-shared.md, "Ties": S1 to S6. A warband lifts no standard and has no second, so
    // S3 and S4 are drawn for the drilled body alone.
    TIE_DRILLED_TOLERANCE = "tie.drilled.tolerance_mm", U(400, 900);
    TIE_LOOSE_TOLERANCE = "tie.loose.tolerance_mm", U(800, 1_800);
    TIE_DRILLED_CLOSE = "tie.drilled.close_after_ms", L(1_000, 5_000);
    TIE_LOOSE_CLOSE = "tie.loose.close_after_ms", L(5_000, 30_000);
    TIE_LIFT = "tie.drilled.lift_after_ms", L(1_000, 10_000);
    TIE_SUCCEED = "tie.drilled.succeed_after_ms", L(2_000, 15_000);
    /// One draw for both ties.
    TIE_STANDOFF = "tie.standoff_mm", U(3_000, 4_500);
    /// One draw for both ties.
    TIE_CONTACT = "tie.contact_mm", U(1_000, 1_400);

    // priors-shared.md, "Bouts": S7 to S22.
    /// 0.05 to 0.2 bouts per second.
    BOUT_RATE = "bout.rate_q16", L(3_277, 13_107);
    /// 0.003 to 0.1, in parts per million.
    BOUT_HARM = "bout.harm_ppm", L(3_000, 100_000);
    BOUT_TARGET_FORWARD = "bout.target_forward_pm", U(1_000, 2_000);
    BOUT_TARGET_BACK = "bout.target_back_pm", U(700, 1_300);
    BOUT_TARGET_TURNED = "bout.target_turned_pm", U(4_000, 12_000);
    BOUT_SHIELD_SIDE = "bout.shield_side_pm", U(700, 1_000);
    BOUT_OPEN_SIDE = "bout.open_side_pm", U(1_300, 3_000);
    BOUT_BEHIND = "bout.behind_pm", U(2_000, 6_000);
    BOUT_UNATTENDED = "bout.unattended_pm", U(1_500, 3_000);
    BOUT_TARGET_HAMPERED = "bout.target_hampered_pm", U(1_200, 2_000);
    BOUT_STRIKER_HAMPERED = "bout.striker_hampered_pm", U(400, 800);
    BOUT_ANSWER = "bout.answer_pm", U(500, 1_000);
    BOUT_SKILL_STEP = "bout.skill_step_pm", U(1_100, 1_400);
    BOUT_DISABLE = "bout.disable_pm", U(250, 450);
    BOUT_KILL = "bout.kill_pm", U(80, 250);
    BOUT_DRIVE_BACK = "bout.drive_back_pm", U(100, 500);
    /// priors-shared.md, "Kit": S23, the one kit value that is drawn.
    KIT_WARBAND_GUARD = "kit.warband.guard_pm", U(1_000, 1_500);

    // priors-field.md, section 1: lines 1 to 33, in the file's order.
    MAN_BACK = "man.back", U(51, 128);
    MAN_FALL = "man.fall", U(256, 768);
    MAN_ENEMY_FLANK = "man.enemy_flank", U(256, 640);
    MAN_ENEMY_REAR = "man.enemy_rear", U(512, 1_024);
    MAN_ENEMY_FRONT = "man.enemy_front", U(64, 192);
    MAN_GAP = "man.gap", U(64, 256);
    MAN_STANDARD_DOWN = "man.standard_down", U(256, 768);
    MAN_LEADER_DOWN = "man.leader_down", U(192, 640);
    MAN_BESIDE = "man.beside", U(38, 90);
    MAN_BEHIND = "man.behind", U(38, 90);
    MAN_LEADER = "man.leader", U(512, 1_280);
    MAN_HAMPERED = "man.hampered", U(128, 384);
    /// Doublings per unit, in Q8.
    MAN_K_ALARM = "man.k_alarm", U(512, 768);
    MAN_STEADY_AGAIN = "man.steady_again", U(768, 1_280);
    MAN_TURN = "man.turn", U(-512, 0);
    MAN_ROOM = "man.room", U(192, 640);
    MAN_CLOSER = "man.closer", U(192, 640);
    MAN_RALLY = "man.rally", U(512, 1_024);
    MAN_ENEMY_TURNED = "man.enemy_turned", U(256, 640);
    MAN_ENEMY_BACK = "man.enemy_back", U(64, 192);
    MAN_CHARGE = "man.charge", U(1_280, 2_048);
    MAN_OWN_UNSTEADY = "man.own_unsteady", U(128, 384);
    MAN_BREATH = "man.breath", U(256, 768);
    /// Doublings per unit, in Q8.
    MAN_K_URGE = "man.k_urge", U(512, 1_024);
    /// Line 25: how often a centurion of the hastati steps out unprompted. `go` is derived
    /// from it, from `man.k_urge` and from his ardour.
    MAN_GO_INTERVAL = "man.go_interval_ms", L(300_000, 1_200_000);
    MAN_STAY = "man.stay", U(128, 384);
    /// One to four changes of posture a second.
    MAN_REACT = "man.react_q16", L(65_536, 262_144);
    MAN_STEP_BACK = "man.step_back_mm", U(600, 1_500);
    MAN_STAND_OFF = "man.stand_off_mm", U(800, 1_200);
    MAN_HAS_FLED = "man.has_fled", U(128, 512);
    MAN_SHAKEN = "man.shaken", U(102, 307);
    /// Two to ten minutes, in base ticks.
    MAN_SHAKEN_WEAR = "man.shaken_wear_ticks", L(2_400, 12_000);
    MAN_PRESS = "man.press", U(0, 255);

    // priors-field.md, section 2: T1 to T10. The hastati's mean ardour and front-rank ardour
    // are fixed at zero.
    HASTATI_NERVE = "troop.hastati.nerve", U(384, 640);
    HASTATI_NERVE_SD = "troop.hastati.nerve_sd", U(128, 256);
    HASTATI_ARDOUR_SD = "troop.hastati.ardour_sd", U(77, 179);
    HASTATI_LEADER_ARDOUR = "troop.hastati.leader_ardour", U(384, 640);
    HASTATI_OFFICER_NERVE = "troop.hastati.officer_nerve", U(256, 512);
    WARBAND_NERVE = "troop.warband.nerve", U(320, 576);
    WARBAND_NERVE_SD = "troop.warband.nerve_sd", U(192, 320);
    WARBAND_ARDOUR = "troop.warband.ardour", U(0, 256);
    WARBAND_ARDOUR_SD = "troop.warband.ardour_sd", U(128, 256);
    WARBAND_LEADER_ARDOUR = "troop.warband.leader_ardour", U(384, 640);
    WARBAND_FRONT_ARDOUR = "troop.warband.front_ardour", U(128, 384);
    WARBAND_OFFICER_NERVE = "troop.warband.officer_nerve", U(128, 384);
    /// Added to the hastati's mean nerve.
    VETERANS_NERVE_ABOVE = "troop.veterans.nerve_above", U(128, 384);
    /// The veterans' spread of nerve, per mille of the hastati's.
    VETERANS_SPREAD = "troop.veterans.spread_pm", U(500, 800);
    /// Taken from the hastati's mean nerve.
    RECRUITS_NERVE_BELOW = "troop.recruits.nerve_below", U(64, 320);

    // priors-field.md, section 4: F1 to F8. Densities are men per m2 in Q8.
    FIELD_JAM = "field.jam_q8", U(1_152, 1_664);
    FIELD_PACK = "field.pack_q8", U(2_048, 2_560);
    FIELD_HOLD = "field.hold_q8", U(154, 256);
    FIELD_EASE = "field.ease_q8", U(512, 768);
    FIELD_SPREAD = "field.spread_mm_s", U(50, 150);
    FIELD_GUARD = "field.guard_mm", U(500, 750);
    /// Read only under `Contact::Crowd`.
    FIELD_CROWD = "field.crowd_q8", U(640, 896);
    FIELD_FLOW = "field.flow_q8", U(315, 453);
}

/// Every line, Paullus's and then Varro's: the columns of the table of record.
pub fn lines() -> impl Iterator<Item = &'static Line> {
    PAULLUS_LINES.into_iter().chain(VARRO_LINES.iter().copied())
}

/// The table of record: one row for each draw and one column for each line, as
/// comma-separated text. It is written and hashed at the freeze, so that the sets a run used
/// can be checked against what was frozen.
pub fn table(freeze_seed: u64, draws: u32) -> String {
    let mut out = String::from("draw");
    for line in lines() {
        out.push(',');
        out.push_str(line.name);
    }
    out.push('\n');
    for i in 0..draws {
        let d = Draw::new(freeze_seed, i);
        out.push_str(&i.to_string());
        for line in lines() {
            out.push(',');
            out.push_str(&d.of(line).to_string());
        }
        out.push('\n');
    }
    out
}

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
fn bouts(d: &Draw) -> BoutParams {
    let pm = |line: &Line| d.of(line) as u32;
    BoutParams {
        rate_q16: pm(&BOUT_RATE),
        harm: Prob::from_ratio(d.of(&BOUT_HARM) as u64, 1_000_000),
        target_forward_pm: pm(&BOUT_TARGET_FORWARD),
        target_back_pm: pm(&BOUT_TARGET_BACK),
        target_turned_pm: pm(&BOUT_TARGET_TURNED),
        shield_side_pm: pm(&BOUT_SHIELD_SIDE),
        open_side_pm: pm(&BOUT_OPEN_SIDE),
        behind_pm: pm(&BOUT_BEHIND),
        unattended_pm: pm(&BOUT_UNATTENDED),
        target_hampered_pm: pm(&BOUT_TARGET_HAMPERED),
        striker_hampered_pm: pm(&BOUT_STRIKER_HAMPERED),
        answer_pm: pm(&BOUT_ANSWER),
        skill_step_pm: pm(&BOUT_SKILL_STEP),
        disable_pm: pm(&BOUT_DISABLE),
        kill_pm: pm(&BOUT_KILL),
        drive_back: Prob::from_ratio(d.of(&BOUT_DRIVE_BACK) as u64, 1_000),
    }
}

/// A time in ms as base ticks, to the nearest.
fn ticks(ms: i64) -> u32 {
    let tick = BASE_TICK.0 as i64;
    ((ms + tick / 2) / tick) as u32
}

/// priors-shared.md, "Ties" (Varro). How far the frame leads and how near a man must stand
/// to lift a standard are fixed.
fn tie(d: &Draw, spec: TieSpec) -> Tie {
    let drilled = spec == TieSpec::Drilled;
    let (tolerance, close) = if drilled {
        (&TIE_DRILLED_TOLERANCE, &TIE_DRILLED_CLOSE)
    } else {
        (&TIE_LOOSE_TOLERANCE, &TIE_LOOSE_CLOSE)
    };
    Tie {
        tolerance_mm: d.of(tolerance) as i32,
        close_after: Some(ticks(d.of(close))),
        lift_after: drilled.then(|| ticks(d.of(&TIE_LIFT))),
        succeed_after: drilled.then(|| ticks(d.of(&TIE_SUCCEED))),
        standoff_mm: d.of(&TIE_STANDOFF) as i32,
        contact_mm: d.of(&TIE_CONTACT) as i32,
        ..Tie::DRILLED
    }
}

/// The hazard at the edge is one in this many ms (priors-field.md, section 3).
const EDGE_MS: u64 = 16_000;

/// Line 25 of priors-field.md: the urge at which a steady man steps out at the edge rate,
/// such that a centurion of the hastati, whose ardour is his leader's addition and nothing
/// else, steps out once in the drawn interval. Each unit of urge under `go` halves his
/// hazard `k_urge` times, so `go` is his ardour plus log2(interval / edge) / `k_urge`.
fn go(d: &Draw) -> i32 {
    go_q8(
        d.of(&MAN_GO_INTERVAL) as u64,
        d.of(&MAN_K_URGE) as u64,
        d.of(&HASTATI_LEADER_ARDOUR) as i32,
    )
}

fn go_q8(interval_ms: u64, k_urge_q8: u64, ardour_q8: i32) -> i32 {
    let ln = ln_ratio_q32(EDGE_MS, interval_ms) as u128;
    let doublings_q32 = (ln << 32) / LN2_Q32 as u128;
    // Doublings over doublings per unit, as units in Q8: both Q8s cancel but one.
    let above_q8 = (doublings_q32 * 65_536 / k_urge_q8 as u128 + (1 << 31)) >> 32;
    ardour_q8 + above_q8 as i32
}

/// priors-field.md, section 1 (Varro). The forward step is fixed.
fn man(d: &Draw) -> cannae_man::Params {
    let q = |line: &Line| d.of(line) as i32;
    cannae_man::Params {
        back: q(&MAN_BACK),
        fall: q(&MAN_FALL),
        enemy_flank: q(&MAN_ENEMY_FLANK),
        enemy_rear: q(&MAN_ENEMY_REAR),
        enemy_front: q(&MAN_ENEMY_FRONT),
        gap: q(&MAN_GAP),
        standard_down: q(&MAN_STANDARD_DOWN),
        leader_down: q(&MAN_LEADER_DOWN),
        beside: q(&MAN_BESIDE),
        behind: q(&MAN_BEHIND),
        leader: q(&MAN_LEADER),
        hampered: q(&MAN_HAMPERED),
        has_fled: q(&MAN_HAS_FLED),
        shaken: q(&MAN_SHAKEN),
        shaken_wear: d.of(&MAN_SHAKEN_WEAR) as u32,
        k_alarm: q(&MAN_K_ALARM),
        steady_again: q(&MAN_STEADY_AGAIN),
        turn: q(&MAN_TURN),
        room: q(&MAN_ROOM),
        closer: q(&MAN_CLOSER),
        rally: q(&MAN_RALLY),
        enemy_turned: q(&MAN_ENEMY_TURNED),
        enemy_back: q(&MAN_ENEMY_BACK),
        charge: q(&MAN_CHARGE),
        own_unsteady: q(&MAN_OWN_UNSTEADY),
        breath: q(&MAN_BREATH),
        k_urge: q(&MAN_K_URGE),
        go: go(d),
        stay: q(&MAN_STAY),
        react_q16: d.of(&MAN_REACT) as u32,
        step_forward_mm: cannae_man::Params::CENTRE.step_forward_mm,
        step_back_mm: q(&MAN_STEP_BACK),
        stand_off_mm: q(&MAN_STAND_OFF),
        press: d.of(&MAN_PRESS) as u8,
    }
}

/// priors-field.md, section 2, with the kit and the wind of priors-shared.md (Varro and
/// Paullus). The veterans and the recruits are the drawn hastati with their differences.
/// Skill, reach and the leaders' skill are fixed.
fn troop(d: &Draw, name: &str) -> Troop {
    let q = |line: &Line| d.of(line) as i32;
    let hastati = Troop {
        nerve: Spread {
            mean: q(&HASTATI_NERVE),
            sd: q(&HASTATI_NERVE_SD),
        },
        ardour: Spread {
            mean: 0,
            sd: q(&HASTATI_ARDOUR_SD),
        },
        front_ardour: 0,
        leader_ardour: q(&HASTATI_LEADER_ARDOUR),
        officer_nerve: q(&HASTATI_OFFICER_NERVE),
        stamina: stamina(d),
        ..Troop::HASTATI
    };
    match name {
        "hastati-veterans" => Troop {
            nerve: Spread {
                mean: hastati.nerve.mean + q(&VETERANS_NERVE_ABOVE),
                sd: hastati.nerve.sd * q(&VETERANS_SPREAD) / 1000,
            },
            skill: Troop::VETERANS.skill,
            leader_skill: Troop::VETERANS.leader_skill,
            ..hastati
        },
        "hastati-recruits" => Troop {
            nerve: Spread {
                mean: hastati.nerve.mean - q(&RECRUITS_NERVE_BELOW),
                sd: hastati.nerve.sd,
            },
            skill: Troop::RECRUITS.skill,
            leader_skill: Troop::RECRUITS.leader_skill,
            ..hastati
        },
        "gallic-warband" => Troop {
            nerve: Spread {
                mean: q(&WARBAND_NERVE),
                sd: q(&WARBAND_NERVE_SD),
            },
            ardour: Spread {
                mean: q(&WARBAND_ARDOUR),
                sd: q(&WARBAND_ARDOUR_SD),
            },
            front_ardour: q(&WARBAND_FRONT_ARDOUR),
            leader_ardour: q(&WARBAND_LEADER_ARDOUR),
            officer_nerve: q(&WARBAND_OFFICER_NERVE),
            guard_pm: d.of(&KIT_WARBAND_GUARD) as u32,
            stamina: stamina(d),
            ..Troop::WARBAND
        },
        _ => hastati,
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

/// priors-field.md, section 4, and the crowd physics of priors-shared.md (Varro). The cell,
/// the cap on spreading, which men touch and what counts as moving are fixed.
fn field(d: &Draw) -> cannae_field::Params {
    let q = |line: &Line| d.of(line) as u32;
    cannae_field::Params {
        jam_q8: q(&FIELD_JAM),
        flow_q8: q(&FIELD_FLOW),
        pack_q8: q(&FIELD_PACK),
        hold_q8: q(&FIELD_HOLD),
        ease_q8: q(&FIELD_EASE),
        spread_mm_s: d.of(&FIELD_SPREAD) as i32,
        press_mm_s: d.of(&PRESS_SPEED) as i32,
        guard_mm: d.of(&FIELD_GUARD) as i32,
        crowd_q8: q(&FIELD_CROWD),
        crush_q8: q(&CRUSH_ONSET),
        crush_rate_q16: field_crush_rate_q16(d.of(&CRUSH_TIME)),
        turn_per_tick: d.of(&TURN) as u16,
        ..cannae_field::Params::CENTRE
    }
}

/// The field's crush hazard for each man per m2 above the onset, per second in Q16, such
/// that a man is disabled in `time_ms` on average at 2 per m2 above it: 1 / (2 T).
fn field_crush_rate_q16(time_ms: i64) -> u32 {
    (32_768_000 / time_ms) as u32
}

/// The set with everything at the value its prior was written around: the centres and
/// placeholders in the code. It is not a draw and no scored run uses it. It is for looking
/// at a mechanism with nothing drawn.
pub fn nominal(cell: Cell) -> Result<Drawn, String> {
    if cell.rung > 3 {
        return Err(format!("no rung {}", cell.rung));
    }
    Ok(Drawn {
        cell,
        i: u32::MAX,
        speeds: GaitSpeeds::PLACEHOLDER,
        power: PowerTable::PLACEHOLDER,
        bouts: BoutParams::PLACEHOLDER,
        man: cannae_man::Params::CENTRE,
        gauges: cannae_gauges::Params::CENTRE,
        substrate: match cell.engine {
            Engine::Pairwise => SubstrateParams::Pairwise(cannae_pairwise::Params::PLACEHOLDER),
            Engine::Field => SubstrateParams::Field(cannae_field::Params::CENTRE),
        },
        drilled: Tie::DRILLED,
        loose: Tie::LOOSE,
        troops: TROOPS.map(|name| (name, Troop::named(name).unwrap_or(Troop::HASTATI))),
    })
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
        // The nominal set is the constants in the code, and is no draw.
        let cell = Cell {
            engine: Engine::Field,
            rung: 2,
        };
        let n = nominal(cell).unwrap();
        assert_eq!(n.man, cannae_man::Params::CENTRE);
        assert_eq!(n.bouts, BoutParams::PLACEHOLDER);
        assert_eq!(n.tie(TieSpec::Loose), Tie::LOOSE);
        assert_eq!(
            n.substrate,
            SubstrateParams::Field(cannae_field::Params::CENTRE)
        );
        assert!(TROOPS.iter().all(|t| n.troop(t) == Troop::named(t)));
        assert_ne!(n.man, first.man);
        assert!(nominal(Cell { rung: 4, ..cell }).is_err());
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

    #[test]
    fn every_line_has_its_own_name_and_varros_stay_in_their_priors() {
        let mut names: Vec<&str> = lines().map(|l| l.name).collect();
        let all = names.len();
        assert_eq!(all, PAULLUS_LINES.len() + 81);
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), all, "two lines share a name");
        for line in VARRO_LINES {
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
            let reach = (hi - lo) / 20;
            assert!(least <= lo + reach && most >= hi - reach, "{}", line.name);
        }
    }

    #[test]
    fn go_puts_the_centurion_at_his_interval() {
        // The centres of rival/07: three doublings a unit, a centurion's ardour of 2, once in
        // 1,024 s, which is six doublings under the edge's 16 s: go is 2 + 6 / 3.
        assert_eq!(go_q8(1_024_000, 768, 512), 1_024);
        // At the edge itself he needs no more than his ardour.
        assert_eq!(go_q8(16_000, 768, 512), 512);
        // Whatever is drawn, his hazard of stepping out is one in the drawn interval. The
        // curve takes its doublings in sixteenths, rounded down, so it may run a twentieth low.
        for i in 0..200 {
            let d = Draw::new(SEED, i);
            let p = man(&d);
            let ardour = troop(&d, "hastati").leader_ardour;
            let rate = cannae_man::curve::rate_q16(p.k_urge, ardour - p.go, p.react_q16) as i64;
            let want = 65_536_000 / d.of(&MAN_GO_INTERVAL);
            assert!(
                rate <= want + 1 && rate * 20 >= want * 19 - 20,
                "draw {i}: {rate} against {want}"
            );
        }
    }

    #[test]
    fn varros_mappings_read_their_lines() {
        let d = Draw::new(SEED, 5);
        let s = drawn(
            SEED,
            Cell {
                engine: Engine::Field,
                rung: 2,
            },
            5,
        )
        .unwrap();
        // The man, line by line in the order of priors-field.md (25 is derived, and has its
        // own test). Written out a second time on purpose: a pair of weights swapped in the
        // mapping would otherwise pass unseen.
        let m = &s.man;
        let by_line: [(i64, &Line); 32] = [
            (m.back as i64, &MAN_BACK),
            (m.fall as i64, &MAN_FALL),
            (m.enemy_flank as i64, &MAN_ENEMY_FLANK),
            (m.enemy_rear as i64, &MAN_ENEMY_REAR),
            (m.enemy_front as i64, &MAN_ENEMY_FRONT),
            (m.gap as i64, &MAN_GAP),
            (m.standard_down as i64, &MAN_STANDARD_DOWN),
            (m.leader_down as i64, &MAN_LEADER_DOWN),
            (m.beside as i64, &MAN_BESIDE),
            (m.behind as i64, &MAN_BEHIND),
            (m.leader as i64, &MAN_LEADER),
            (m.hampered as i64, &MAN_HAMPERED),
            (m.k_alarm as i64, &MAN_K_ALARM),
            (m.steady_again as i64, &MAN_STEADY_AGAIN),
            (m.turn as i64, &MAN_TURN),
            (m.room as i64, &MAN_ROOM),
            (m.closer as i64, &MAN_CLOSER),
            (m.rally as i64, &MAN_RALLY),
            (m.enemy_turned as i64, &MAN_ENEMY_TURNED),
            (m.enemy_back as i64, &MAN_ENEMY_BACK),
            (m.charge as i64, &MAN_CHARGE),
            (m.own_unsteady as i64, &MAN_OWN_UNSTEADY),
            (m.breath as i64, &MAN_BREATH),
            (m.k_urge as i64, &MAN_K_URGE),
            (m.stay as i64, &MAN_STAY),
            (m.react_q16 as i64, &MAN_REACT),
            (m.step_back_mm as i64, &MAN_STEP_BACK),
            (m.stand_off_mm as i64, &MAN_STAND_OFF),
            (m.has_fled as i64, &MAN_HAS_FLED),
            (m.shaken as i64, &MAN_SHAKEN),
            (m.shaken_wear as i64, &MAN_SHAKEN_WEAR),
            (m.press as i64, &MAN_PRESS),
        ];
        for (got, line) in by_line {
            assert_eq!(got, d.of(line), "{}", line.name);
        }
        assert_eq!(s.man.step_forward_mm, 2_250);
        // The multipliers of a bout, S9 to S21 in the order of priors-shared.md.
        let b = &s.bouts;
        let by_line: [(u32, &Line); 14] = [
            (b.rate_q16, &BOUT_RATE),
            (b.target_forward_pm, &BOUT_TARGET_FORWARD),
            (b.target_back_pm, &BOUT_TARGET_BACK),
            (b.target_turned_pm, &BOUT_TARGET_TURNED),
            (b.shield_side_pm, &BOUT_SHIELD_SIDE),
            (b.open_side_pm, &BOUT_OPEN_SIDE),
            (b.behind_pm, &BOUT_BEHIND),
            (b.unattended_pm, &BOUT_UNATTENDED),
            (b.target_hampered_pm, &BOUT_TARGET_HAMPERED),
            (b.striker_hampered_pm, &BOUT_STRIKER_HAMPERED),
            (b.answer_pm, &BOUT_ANSWER),
            (b.skill_step_pm, &BOUT_SKILL_STEP),
            (b.disable_pm, &BOUT_DISABLE),
            (b.kill_pm, &BOUT_KILL),
        ];
        for (got, line) in by_line {
            assert_eq!(got as i64, d.of(line), "{}", line.name);
        }
        // The ties: times in ticks; only a drilled body lifts its standard and has a second;
        // the standoff is one draw for both.
        let (drilled, loose) = (s.tie(TieSpec::Drilled), s.tie(TieSpec::Loose));
        assert_eq!(drilled.tolerance_mm as i64, d.of(&TIE_DRILLED_TOLERANCE));
        assert_eq!(loose.tolerance_mm as i64, d.of(&TIE_LOOSE_TOLERANCE));
        assert_eq!(
            drilled.lift_after,
            Some(((d.of(&TIE_LIFT) + 25) / 50) as u32)
        );
        assert_eq!((loose.lift_after, loose.succeed_after), (None, None));
        assert_eq!(drilled.standoff_mm, loose.standoff_mm);
        assert_eq!((drilled.lead_mm, drilled.lift_reach_mm), (1_500, 2_500));
        // Bouts: a chance in parts per million, a multiplier, a share.
        assert_eq!(
            s.bouts.harm,
            Prob::from_ratio(d.of(&BOUT_HARM) as u64, 1_000_000)
        );
        assert_eq!(
            s.bouts.drive_back,
            Prob::from_ratio(d.of(&BOUT_DRIVE_BACK) as u64, 1_000)
        );
        // Troops: the variants are the drawn hastati with their differences.
        let name = |n| s.troop(n).unwrap();
        let (h, v, r, w) = (
            name("hastati"),
            name("hastati-veterans"),
            name("hastati-recruits"),
            name("gallic-warband"),
        );
        let by_line: [(i32, &Line); 12] = [
            (h.nerve.mean, &HASTATI_NERVE),
            (h.nerve.sd, &HASTATI_NERVE_SD),
            (h.ardour.sd, &HASTATI_ARDOUR_SD),
            (h.leader_ardour, &HASTATI_LEADER_ARDOUR),
            (h.officer_nerve, &HASTATI_OFFICER_NERVE),
            (w.nerve.mean, &WARBAND_NERVE),
            (w.nerve.sd, &WARBAND_NERVE_SD),
            (w.ardour.mean, &WARBAND_ARDOUR),
            (w.ardour.sd, &WARBAND_ARDOUR_SD),
            (w.leader_ardour, &WARBAND_LEADER_ARDOUR),
            (w.front_ardour, &WARBAND_FRONT_ARDOUR),
            (w.officer_nerve, &WARBAND_OFFICER_NERVE),
        ];
        for (got, line) in by_line {
            assert_eq!(got as i64, d.of(line), "{}", line.name);
        }
        assert_eq!((h.ardour.mean, h.front_ardour), (0, 0));
        assert_eq!(
            (v.nerve.mean - h.nerve.mean) as i64,
            d.of(&VETERANS_NERVE_ABOVE)
        );
        assert!(v.nerve.sd < h.nerve.sd && v.nerve.sd * 2 >= h.nerve.sd - 1);
        assert_eq!(
            (h.nerve.mean - r.nerve.mean) as i64,
            d.of(&RECRUITS_NERVE_BELOW)
        );
        assert_eq!((v.skill, r.skill, v.leader_skill), (1, -1, 3));
        assert_eq!(
            (v.ardour, v.leader_ardour, r.officer_nerve),
            (h.ardour, h.leader_ardour, h.officer_nerve)
        );
        assert_eq!(w.guard_pm as i64, d.of(&KIT_WARBAND_GUARD));
        assert_eq!((h.guard_pm, h.reach_mm, w.reach_mm), (1_000, 1_500, 1_500));
        // The field: its own lines, and the four it shares with the pairwise engine.
        let SubstrateParams::Field(f) = s.substrate else {
            panic!("not the field")
        };
        let by_line: [(i64, &Line); 8] = [
            (f.jam_q8 as i64, &FIELD_JAM),
            (f.pack_q8 as i64, &FIELD_PACK),
            (f.hold_q8 as i64, &FIELD_HOLD),
            (f.ease_q8 as i64, &FIELD_EASE),
            (f.spread_mm_s as i64, &FIELD_SPREAD),
            (f.guard_mm as i64, &FIELD_GUARD),
            (f.crowd_q8 as i64, &FIELD_CROWD),
            (f.flow_q8 as i64, &FIELD_FLOW),
        ];
        for (got, line) in by_line {
            assert_eq!(got, d.of(line), "{}", line.name);
        }
        assert_eq!(f.crush_q8 as i64, d.of(&CRUSH_ONSET));
        assert_eq!(f.press_mm_s as i64, d.of(&PRESS_SPEED));
        assert_eq!(f.turn_per_tick as i64, d.of(&TURN));
        assert_eq!(f.contact, cannae_field::Contact::Line);
        assert_eq!((f.cell_shift, f.spread_max_mm_s), (10, 300));
        // 134 s to disablement at two men per m2 above the onset is the centre's 244.
        assert_eq!(field_crush_rate_q16(134_000), 244);
        assert_eq!(f.crush_rate_q16, field_crush_rate_q16(d.of(&CRUSH_TIME)));
    }

    #[test]
    fn every_draw_makes_engines_that_can_be_built() {
        for i in 0..300 {
            let d = Draw::new(SEED, i);
            // The field: packing is denser than the jam, and the walking curve has the
            // drawn capacity (`Curve::of_capacity` refuses one it cannot reach).
            let f = field(&d);
            assert!(f.pack_q8 > f.jam_q8 && f.crowd_q8 >= f.ease_q8 - 128);
            // The table is in 256ths of free speed, so its capacity is within a hundredth.
            let curve = cannae_field::curve::Curve::of_capacity(f.flow_q8, f.jam_q8);
            let got = curve.capacity_q8(f.jam_q8);
            assert!(
                got.abs_diff(f.flow_q8) * 100 <= f.flow_q8,
                "draw {i}: {got}"
            );
            // The bouts: the shares of wounds leave room for the hampering ones.
            let b = bouts(&d);
            assert!(b.disable_pm + b.kill_pm <= 700);
            // The man: a forward man steps back at a lower urge than he stepped out at.
            let m = man(&d);
            assert!(m.stay > 0 && m.go > m.stay);
        }
    }

    #[test]
    fn the_table_of_record_has_a_row_for_each_draw_and_a_column_for_each_line() {
        let t = table(SEED, 3);
        let rows: Vec<Vec<&str>> = t.lines().map(|r| r.split(',').collect()).collect();
        assert_eq!(rows.len(), 4);
        assert!(rows.iter().all(|r| r.len() == 1 + lines().count()));
        assert_eq!(rows[0][0], "draw");
        assert_eq!(rows[0][1], RESERVE.name);
        assert_eq!(rows[3][0], "2");
        let col = 1 + lines().position(|l| l.name == "man.turn").unwrap();
        assert_eq!(rows[3][col], Draw::new(SEED, 2).of(&MAN_TURN).to_string());
    }
}
