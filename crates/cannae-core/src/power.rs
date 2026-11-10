//! What each activity costs in power, for W'. One table for both engines.
//!
//! The values are placeholders. Their priors are fixed in S0.3 from published locomotion and
//! load-carriage physiology, before any scored run.

use crate::units::MilliW;

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
    pub mw: [MilliW; 6],
}

impl PowerTable {
    pub const PLACEHOLDER: PowerTable = PowerTable {
        mw: [80_000, 120_000, 200_000, 450_000, 400_000, 500_000],
    };

    pub const fn of(&self, a: Activity) -> MilliW {
        self.mw[a as usize]
    }
}

/// Speeds by gait in mm/s. Placeholders, like the power table.
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
