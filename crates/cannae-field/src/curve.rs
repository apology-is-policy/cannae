//! How fast a man walks in a crowd.
//!
//! The shape is Weidmann's fit to pedestrian studies: speed falls from free walking to nothing
//! at the jam density, as `1 - exp(-k (1/s - 1))`, with s the density as a share of the jam
//! density. Weidmann's own steepness is k = 1.913 / 5.4, fitted to people walking in the open.
//! Here the steepness is set by what it decides, the most a walking crowd passes through a
//! gap, which is drawn (docs/slice0/priors-field.md, F8).
//!
//! The curve is a table, so that no float touches a step. It is built in integers when the
//! field is made, with no library exponential, so that a run is the same on any machine.

/// Steps of the table from an empty field to the jam density.
pub const STEPS: u32 = 128;

/// The steepest curve that can be asked for, Q16. At this steepness the capacity is about
/// three quarters of the jam density.
const STEEPEST_Q16: u32 = 16 << 16;

/// 65536 * 2^(-i/64).
const HALVING: [u32; 65] = [
    65_536, 64_830, 64_132, 63_441, 62_757, 62_081, 61_413, 60_751, 60_097, 59_449, 58_809, 58_176,
    57_549, 56_929, 56_316, 55_709, 55_109, 54_515, 53_928, 53_347, 52_773, 52_204, 51_642, 51_085,
    50_535, 49_991, 49_452, 48_920, 48_393, 47_871, 47_356, 46_846, 46_341, 45_842, 45_348, 44_859,
    44_376, 43_898, 43_425, 42_958, 42_495, 42_037, 41_584, 41_136, 40_693, 40_255, 39_821, 39_392,
    38_968, 38_548, 38_133, 37_722, 37_316, 36_914, 36_516, 36_123, 35_734, 35_349, 34_968, 34_591,
    34_219, 33_850, 33_486, 33_125, 32_768,
];

/// 65536 * exp(-x) for x in Q16, to within 2.
fn exp_neg_q16(x_q16: u64) -> u32 {
    // exp(-x) = 2^(-x / ln 2), and 1 / ln 2 is 94,548 in Q16.
    let y = (x_q16 * 94_548) >> 16;
    let whole = y >> 16;
    if whole > 16 {
        return 0;
    }
    let frac = (y & 0xFFFF) as u32;
    let (i, r) = ((frac >> 10) as usize, frac & 1_023);
    ((HALVING[i] * (1_024 - r) + HALVING[i + 1] * r) >> 10) >> whole
}

/// Speed as a share of free speed, Q16, at `step` of `STEPS` toward the jam density.
fn share_q16(steep_q16: u32, step: u32) -> u32 {
    if step == 0 {
        return 65_536;
    }
    let step = step.min(STEPS) as u64;
    65_536 - exp_neg_q16(steep_q16 as u64 * (STEPS as u64 - step) / step)
}

/// The most the curve passes: the greatest product of density and speed, in steps times Q16.
fn capacity(steep_q16: u32) -> u64 {
    (1..STEPS)
        .map(|s| s as u64 * share_q16(steep_q16, s) as u64)
        .max()
        .unwrap_or(0)
}

/// The walking curve of one field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Curve {
    steep_q16: u32,
    /// Speed as a share of free speed, Q8, at each `STEPS`-th of the jam density.
    q8: [u16; STEPS as usize + 1],
}

impl Curve {
    /// The curve of a given steepness, Q16.
    pub fn of_steepness(steep_q16: u32) -> Curve {
        let mut q8 = [0; STEPS as usize + 1];
        for (step, v) in q8.iter_mut().enumerate() {
            *v = ((share_q16(steep_q16, step as u32) + 128) >> 8) as u16;
        }
        Curve { steep_q16, q8 }
    }

    /// The least steep curve whose capacity is `flow_q8` where the jam density is `jam_q8`,
    /// both in men per square metre, Q8. The capacity is the most men that pass a metre of
    /// width for each metre a free man walks.
    pub fn of_capacity(flow_q8: u32, jam_q8: u32) -> Curve {
        let want = flow_q8 as u64 * STEPS as u64 * 65_536 / jam_q8.max(1) as u64;
        assert!(
            capacity(STEEPEST_Q16) >= want,
            "a walking crowd cannot pass more than about three quarters of the jam density"
        );
        // Capacity never falls as the curve steepens, so halving finds the least that does.
        let (mut under, mut enough) = (0, STEEPEST_Q16);
        while enough - under > 1 {
            let mid = under + (enough - under) / 2;
            if capacity(mid) >= want {
                enough = mid;
            } else {
                under = mid;
            }
        }
        Curve::of_steepness(enough)
    }

    /// Speed as a share of free speed, Q8, at `step` of `STEPS` toward the jam density.
    pub fn walk_q8(&self, step: u32) -> i32 {
        self.q8[step.min(STEPS) as usize] as i32
    }

    pub fn steepness_q16(&self) -> u32 {
        self.steep_q16
    }

    /// The capacity of the table as it stands, in men per square metre, Q8, at `jam_q8`.
    pub fn capacity_q8(&self, jam_q8: u32) -> u32 {
        let most = (1..STEPS)
            .map(|s| s as u64 * self.q8[s as usize] as u64)
            .max()
            .unwrap_or(0);
        (most * jam_q8 as u64 / (STEPS as u64 * 256)) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Weidmann's own curve, 1.913 / 5.4 steep, as it was tabulated from the formula.
    const WEIDMANN_Q8: [u16; STEPS as usize + 1] = [
        256, 256, 256, 256, 256, 256, 256, 255, 255, 254, 252, 250, 248, 245, 242, 238, 235, 231,
        227, 222, 218, 214, 210, 205, 201, 197, 192, 188, 184, 180, 176, 172, 168, 164, 160, 156,
        152, 149, 145, 142, 139, 135, 132, 129, 126, 123, 120, 117, 114, 111, 109, 106, 103, 101,
        98, 96, 94, 91, 89, 87, 85, 83, 80, 78, 76, 74, 72, 71, 69, 67, 65, 63, 62, 60, 58, 57, 55,
        54, 52, 50, 49, 48, 46, 45, 43, 42, 41, 39, 38, 37, 36, 34, 33, 32, 31, 30, 29, 27, 26, 25,
        24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 14, 13, 12, 11, 10, 9, 8, 8, 7, 6, 5, 4, 4, 3,
        2, 1, 1, 0,
    ];

    /// 1.913 / 5.4 in Q16.
    const WEIDMANN_STEEP_Q16: u32 = 23_217;

    #[test]
    #[allow(clippy::float_arithmetic)]
    fn the_exponential_is_right_to_two_parts_in_65536() {
        for i in 0..40_000u64 {
            let x_q16 = i * i / 64;
            let want = 65_536.0 * (-(x_q16 as f64) / 65_536.0).exp();
            let got = exp_neg_q16(x_q16) as f64;
            assert!(
                (got - want).abs() <= 2.0,
                "at {x_q16}: {got} against {want}"
            );
        }
        assert_eq!(exp_neg_q16(0), 65_536);
        assert_eq!(exp_neg_q16(u32::MAX as u64 * 4_096), 0);
    }

    #[test]
    #[allow(clippy::float_arithmetic)]
    fn the_table_is_the_formula_at_every_steepness() {
        for steep_q16 in [
            6_000,
            WEIDMANN_STEEP_Q16,
            40_000,
            65_536,
            200_000,
            STEEPEST_Q16,
        ] {
            let c = Curve::of_steepness(steep_q16);
            let k = steep_q16 as f64 / 65_536.0;
            for step in 1..=STEPS {
                let s = step as f64 / STEPS as f64;
                let v = 1.0 - (-k * (1.0 / s - 1.0)).exp();
                assert!(
                    (c.walk_q8(step) as f64 - 256.0 * v).abs() <= 0.52,
                    "steepness {k} at {s} of the jam"
                );
            }
            assert_eq!(c.walk_q8(0), 256);
            assert_eq!(c.walk_q8(STEPS), 0);
            assert_eq!(c.walk_q8(STEPS + 9), 0);
            assert!((1..=STEPS).all(|s| c.walk_q8(s) <= c.walk_q8(s - 1)));
        }
    }

    #[test]
    fn weidmanns_steepness_gives_weidmanns_table() {
        let c = Curve::of_steepness(WEIDMANN_STEEP_Q16);
        for step in 0..=STEPS {
            let was = WEIDMANN_Q8[step as usize] as i32;
            assert!((c.walk_q8(step) - was).abs() <= 1, "step {step}");
        }
    }

    #[test]
    fn a_curve_made_for_a_capacity_has_it_and_no_more() {
        // Jam densities across their prior, capacities from Weidmann's to twice the measured.
        for jam_q8 in [1_152, 1_382, 1_664] {
            for flow_q8 in [200, 235, 300, 374, 450, 600] {
                let c = Curve::of_capacity(flow_q8, jam_q8);
                // The table is in 256ths of free speed, so its capacity is within one part
                // in a hundred of what was asked.
                let got = c.capacity_q8(jam_q8) as i64;
                assert!(
                    (got - flow_q8 as i64).abs() * 100 <= flow_q8 as i64,
                    "asked {flow_q8} at a jam of {jam_q8}, got {got}"
                );
                // The least steep that does: a little less steep falls short.
                let less = Curve::of_steepness(c.steepness_q16() - 1);
                assert!(capacity(less.steepness_q16()) < capacity(c.steepness_q16()));
            }
        }
        // A greater capacity needs a steeper curve, and a denser jam a less steep one.
        let steep = |flow, jam| Curve::of_capacity(flow, jam).steepness_q16();
        assert!(steep(374, 1_382) > steep(235, 1_382));
        assert!(steep(374, 1_664) < steep(374, 1_382));
    }

    #[test]
    #[should_panic(expected = "three quarters")]
    fn a_capacity_near_the_jam_density_is_refused() {
        Curve::of_capacity(1_300, 1_382);
    }
}
