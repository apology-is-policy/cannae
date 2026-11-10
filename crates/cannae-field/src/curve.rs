//! How fast a man walks in a crowd.
//!
//! The shape is Weidmann's fit to pedestrian studies: speed falls from free walking to nothing
//! at the jam density, as `1 - exp(-1.913 (1/d - 1/5.4))` with d in men per square metre. It is
//! stored as a table, so that no float touches a step, and it is stretched along the density
//! axis to whatever jam density the parameters give.

/// Steps of the table from an empty field to the jam density.
pub const STEPS: u32 = 128;

/// Speed as a share of free speed, Q8, at each `STEPS`-th of the jam density.
const WALK_Q8: [u16; STEPS as usize + 1] = [
    256, 256, 256, 256, 256, 256, 256, 255, 255, 254, 252, 250, 248, 245, 242, 238, 235, 231, 227,
    222, 218, 214, 210, 205, 201, 197, 192, 188, 184, 180, 176, 172, 168, 164, 160, 156, 152, 149,
    145, 142, 139, 135, 132, 129, 126, 123, 120, 117, 114, 111, 109, 106, 103, 101, 98, 96, 94, 91,
    89, 87, 85, 83, 80, 78, 76, 74, 72, 71, 69, 67, 65, 63, 62, 60, 58, 57, 55, 54, 52, 50, 49, 48,
    46, 45, 43, 42, 41, 39, 38, 37, 36, 34, 33, 32, 31, 30, 29, 27, 26, 25, 24, 23, 22, 21, 20, 19,
    18, 17, 16, 15, 14, 14, 13, 12, 11, 10, 9, 8, 8, 7, 6, 5, 4, 4, 3, 2, 1, 1, 0,
];

/// Speed as a share of free speed, Q8, at `step` of `STEPS` toward the jam density.
pub fn walk_q8(step: u32) -> i32 {
    WALK_Q8[step.min(STEPS) as usize] as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The table against the formula it was made from.
    #[test]
    #[allow(clippy::float_arithmetic)]
    fn the_table_is_weidmanns_curve() {
        for step in 1..=STEPS {
            let d = 5.4 * step as f64 / STEPS as f64;
            let v = 1.0 - (-1.913 * (1.0 / d - 1.0 / 5.4)).exp();
            assert!(
                (walk_q8(step) as f64 - 256.0 * v).abs() <= 0.5 + 1e-9,
                "at {d} per m2"
            );
        }
        assert_eq!(walk_q8(0), 256);
        assert_eq!(walk_q8(STEPS + 9), 0);
        assert!((1..=STEPS).all(|s| walk_q8(s) <= walk_q8(s - 1)));
    }
}
