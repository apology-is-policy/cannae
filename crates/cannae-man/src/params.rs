//! Every number the man is made of. Each field is one free parameter with a prior in
//! `docs/slice0/priors-field.md`; the values here are the centres of those priors.
//!
//! Weights are Q8: 256 is one unit. The unit of alarm is one man of his own side, turned, in
//! his view. The unit of urge is one man of his own side, forward, in his view.

use cannae_core::stamina::StaminaParams;

/// One unit, in Q8.
pub const UNIT: i32 = 256;

/// How far a man can make out what his leader is doing.
pub const SIGHT_LEADER_MM: i32 = 20_000;
/// How far he can see whether his standard still stands. Standards are made to be seen.
pub const SIGHT_STANDARD_MM: i32 = 60_000;
/// The most *shaken* a man can be.
pub const SHAKEN_MAX: u8 = 3;

/// The rung of the memory ladder: what a man carries from one moment to the next, besides his
/// wounds and his breath. Each rung adds one mark to the rung below.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Rung {
    /// Nothing. Every hazard is read off what he sees now.
    NoMarks = 0,
    /// One bit: he has turned his back before.
    HasFled = 1,
    /// And a small count of recent frights, which wears off.
    Shaken = 2,
}

/// The weights of the two sums and the thresholds of the exits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Params {
    // Alarm: what pushes a man toward giving ground. A man of his side turned counts one.
    /// A man of his side giving ground.
    pub back: i32,
    /// A man of his side who went down this step. A pulse: it lasts one step.
    pub fall: i32,
    /// A standing enemy within reach on his left or right.
    pub enemy_flank: i32,
    /// A standing enemy within reach behind him.
    pub enemy_rear: i32,
    /// An enemy who has stepped forward at him, within reach, in front.
    pub enemy_front: i32,
    /// A slot beside his own that exists and stands empty.
    pub gap: i32,
    /// His standard is down, or is being carried off.
    pub standard_down: i32,
    /// His leader is down within sight.
    pub leader_down: i32,

    // Support: what holds him. Counted per man of his side who stands, to a cap of six.
    /// Beside him and ahead of him.
    pub beside: i32,
    /// Behind him.
    pub behind: i32,
    /// How many men his leader counts for, in what the leader is seen to do: standing, going
    /// forward, giving ground, turning.
    pub leader: i32,

    // The limit: his nerve, less what wears it.
    /// A wound that hampers him.
    pub hampered: i32,
    /// He has fled before. Rung 1 and up.
    pub has_fled: i32,
    /// Each level of *shaken*. Rung 2.
    pub shaken: i32,
    /// Base ticks for one level of *shaken* to wear off. Rung 2.
    pub shaken_wear: u32,

    // The alarm exits: steady to back, back to turned or steady, turned to steady.
    /// Doublings of the hazard per unit of alarm.
    pub k_alarm: i32,
    /// How far below the edge his alarm must fall before a man giving ground stands again.
    pub steady_again: i32,
    /// How much more alarm it takes to turn than to give ground. May be negative: a man who
    /// has given ground is then nearer to flight than a steady man is to giving ground.
    pub turn: i32,
    /// What open ground behind him is worth to a man about to turn.
    pub room: i32,
    /// What a closer standing behind him is worth against turning.
    pub closer: i32,
    /// How far below the edge his alarm must fall before a turned man rallies.
    pub rally: i32,

    // Urge: what sends a man forward. A man of his side forward counts one.
    /// An enemy ahead who has turned his back.
    pub enemy_turned: i32,
    /// An enemy ahead giving ground.
    pub enemy_back: i32,
    /// An order to charge or pursue.
    pub charge: i32,
    /// A man of his side giving ground or turned.
    pub own_unsteady: i32,
    /// Each level of being winded: one for each full quarter of his breath gone, to three.
    pub breath: i32,
    /// Doublings of the hazard per unit of urge.
    pub k_urge: i32,
    /// The urge at which a steady man steps forward at the edge rate.
    pub go: i32,
    /// How far below that his urge must fall before a forward man steps back into line.
    pub stay: i32,

    /// The fastest any man changes posture, Q16 per second: his reaction time.
    pub react_q16: u32,

    // How far he moves in each posture.
    /// How far ahead of his slot a forward man stands when no enemy is within reach. Fixed,
    /// not drawn: it has to carry him from any standoff the tie allows to within reach.
    pub step_forward_mm: i32,
    /// How far behind his slot a man giving ground stands.
    pub step_back_mm: i32,
    /// How near a forward man closes with the enemy he has chosen.
    pub stand_off_mm: i32,
    /// How hard a man going for an enemy leans on whatever is in front of him while he is kept
    /// from that distance (0-255). Only men out of their places press in Slice 0.
    pub press: u8,
}

/// How far short of his striking distance a man must be before he leans: a pace. Fixed, not
/// drawn: it only says that a man who has got there is no longer kept from it.
pub const KEPT_MM: i32 = 500;

impl Params {
    pub const CENTRE: Params = Params {
        back: 90,
        fall: 512,
        enemy_flank: 384,
        enemy_rear: 768,
        enemy_front: 128,
        gap: 128,
        standard_down: 512,
        leader_down: 384,
        beside: 64,
        behind: 64,
        leader: 768,
        hampered: 256,
        has_fled: 256,
        shaken: 192,
        shaken_wear: 6_000,
        k_alarm: 640,
        steady_again: 1_024,
        turn: -256,
        room: 384,
        closer: 384,
        rally: 768,
        enemy_turned: 384,
        enemy_back: 128,
        charge: 1_536,
        own_unsteady: 256,
        breath: 512,
        k_urge: 768,
        go: 1_024,
        stay: 256,
        react_q16: 2 << 16,
        step_forward_mm: 2_250,
        step_back_mm: 1_000,
        stand_off_mm: 1_000,
        press: 128,
    };
}

/// A mean and a spread between men, Q8.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Spread {
    pub mean: i32,
    pub sd: i32,
}

/// What the men of one troop type are like. These are inputs: a scenario names a troop, and
/// the troop's values say who came to the battle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Troop {
    /// The alarm a man can bear, standing alone and unsupported.
    pub nerve: Spread,
    /// His wish to get at the enemy.
    pub ardour: Spread,
    /// Added to the ardour of the front rank: champions fight in front.
    pub front_ardour: i32,
    /// Added to the ardour of the leader, who leads by going first.
    pub leader_ardour: i32,
    /// Added to the nerve of the leader, the closer and the standard-bearer.
    pub officer_nerve: i32,
    /// Skill at arms, in the bout resolver's steps.
    pub skill: i8,
    pub leader_skill: i8,
    pub reach_mm: i32,
    /// Shield and armour, per mille of the reference man's chance of harm.
    pub guard_pm: u32,
    /// His breath. Like skill, reach and guard it is a shared input
    /// (docs/slice0/priors-shared.md); until that table gives troops their own, every troop
    /// has the placeholder.
    pub stamina: StaminaParams,
}

impl Troop {
    /// Men of the first line, seasoned but not old: the reference troop.
    pub const HASTATI: Troop = Troop {
        nerve: Spread { mean: 512, sd: 192 },
        ardour: Spread { mean: 0, sd: 128 },
        front_ardour: 0,
        leader_ardour: 512,
        officer_nerve: 384,
        skill: 0,
        leader_skill: 2,
        reach_mm: 1_500,
        guard_pm: 1_000,
        stamina: StaminaParams::PLACEHOLDER,
    };

    pub const VETERANS: Troop = Troop {
        nerve: Spread { mean: 768, sd: 128 },
        skill: 1,
        leader_skill: 3,
        ..Troop::HASTATI
    };

    pub const RECRUITS: Troop = Troop {
        nerve: Spread { mean: 320, sd: 192 },
        skill: -1,
        leader_skill: 1,
        ..Troop::HASTATI
    };

    /// A warband: fierce at the onset, uneven in nerve, its champions in front, and no armour
    /// to speak of.
    pub const WARBAND: Troop = Troop {
        nerve: Spread { mean: 448, sd: 256 },
        ardour: Spread { mean: 128, sd: 192 },
        front_ardour: 256,
        leader_ardour: 512,
        officer_nerve: 256,
        skill: 0,
        leader_skill: 2,
        reach_mm: 1_500,
        guard_pm: 1_250,
        stamina: StaminaParams::PLACEHOLDER,
    };

    /// The troop a scenario names.
    pub fn named(name: &str) -> Option<Troop> {
        Some(match name {
            "hastati" => Troop::HASTATI,
            "hastati-veterans" => Troop::VETERANS,
            "hastati-recruits" => Troop::RECRUITS,
            "gallic-warband" => Troop::WARBAND,
            _ => return None,
        })
    }
}
