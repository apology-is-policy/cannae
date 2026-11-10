//! What each activity costs in power, for W'. One table for both engines.
//!
//! Power is a multiple of the man's own critical power, in per mille (see `stamina`). The values
//! are the middles of the priors in docs/slice0/priors-shared.md.

use crate::stamina::CpPerMille;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Activity {
    /// Sitting or lying down, out of the fight.
    Rest = 0,
    /// Standing in kit, on guard.
    Stand = 1,
    Walk = 2,
    Run = 3,
    /// Shoving in a press.
    Press = 4,
    /// A bout.
    Fight = 5,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PowerTable {
    pub pm: [CpPerMille; 6],
}

impl PowerTable {
    /// Rest 0.09, stand 0.14, walk 0.5, run 1.4, press 1.35 and fight 1.55 CP.
    pub const PLACEHOLDER: PowerTable = PowerTable {
        pm: [90, 140, 500, 1_400, 1_350, 1_550],
    };

    pub const fn of(&self, a: Activity) -> CpPerMille {
        self.pm[a as usize]
    }
}

/// Speeds by gait in mm/s. Walking is Vegetius's military step (1.9): 20 Roman miles in five
/// summer hours, about 1.3 m/s.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GaitSpeeds {
    pub walk: i32,
    pub run: i32,
}

impl GaitSpeeds {
    pub const PLACEHOLDER: GaitSpeeds = GaitSpeeds {
        walk: 1_300,
        run: 4_000,
    };
}
