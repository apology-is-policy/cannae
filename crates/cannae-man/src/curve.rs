//! One curve for every exit: a hazard that doubles with each step of margin.
//!
//! `rate = EDGE * 2^(k * margin)`, capped. At zero margin a man goes in sixteen seconds on
//! average; `k` says how many times the rate doubles for one more unit of margin. Far enough
//! below the edge the rate is exactly zero, in integer arithmetic, with no threshold to tune.

/// The rate at zero margin: one in sixteen seconds. Q16 per second.
pub const EDGE_Q16: u32 = 1 << 12;

/// 2^(i/16) in Q16.
const ROOT: [u32; 16] = [
    65_536, 68_438, 71_468, 74_632, 77_936, 81_386, 84_990, 88_752, 92_682, 96_785, 101_070,
    105_545, 110_218, 115_098, 120_194, 125_515,
];

/// The hazard, Q16 per second, at `margin` (Q8 units) with steepness `k` (doublings per unit,
/// Q8), never above `cap`. Doublings are taken in sixteenths, rounded down.
pub fn rate_q16(k: i32, margin: i32, cap: u32) -> u32 {
    let sixteenths = (k as i64 * margin as i64) >> 12;
    let whole = sixteenths.div_euclid(16);
    let base = (EDGE_Q16 as u64 * ROOT[sixteenths.rem_euclid(16) as usize] as u64) >> 16;
    let rate = match whole {
        20.. => u64::MAX,
        0..20 => base << whole,
        -63..0 => base >> -whole,
        _ => 0,
    };
    rate.min(cap as u64) as u32
}

#[cfg(test)]
#[allow(clippy::float_arithmetic)]
mod tests {
    use super::*;

    const NO_CAP: u32 = u32::MAX;

    #[test]
    fn the_table_is_the_sixteenth_roots_of_two() {
        for (i, &r) in ROOT.iter().enumerate() {
            let want = (65_536.0 * 2f64.powf(i as f64 / 16.0)).round() as u32;
            assert_eq!(r, want, "entry {i}");
        }
    }

    #[test]
    fn the_edge_is_one_in_sixteen_seconds_and_each_unit_doubles_k_times() {
        assert_eq!(rate_q16(640, 0, NO_CAP), EDGE_Q16);
        // k = 2 doublings a unit: one unit up is four times the rate, one down a quarter.
        assert_eq!(rate_q16(512, 256, NO_CAP), EDGE_Q16 * 4);
        assert_eq!(rate_q16(512, -256, NO_CAP), EDGE_Q16 / 4);
        // Against the real curve, within the rounding of a sixteenth of a doubling.
        for margin in (-1_200..1_200).step_by(37) {
            let k = 640;
            let exact = EDGE_Q16 as f64 * 2f64.powf(k as f64 * margin as f64 / 65_536.0);
            let got = rate_q16(k, margin, NO_CAP) as f64;
            assert!(
                got <= exact * 1.001 + 1.0 && got >= exact / 1.045 - 1.0,
                "margin {margin}: {got} against {exact}"
            );
        }
    }

    #[test]
    fn far_below_the_edge_the_rate_is_exactly_zero() {
        // Thirteen doublings below one in sixteen seconds is less than one in 2^16 seconds.
        assert_eq!(rate_q16(256, -13 * 256, NO_CAP), 0);
        assert_eq!(rate_q16(256, -12 * 256, NO_CAP), 1);
        assert_eq!(rate_q16(640, i32::MIN / 1024, NO_CAP), 0);
    }

    #[test]
    fn the_cap_holds_however_large_the_margin() {
        let cap = 2 << 16;
        assert_eq!(rate_q16(640, 100 * 256, cap), cap);
        assert_eq!(rate_q16(640, i32::MAX / 1024, cap), cap);
        assert!(rate_q16(640, 256, cap) < cap);
    }
}
