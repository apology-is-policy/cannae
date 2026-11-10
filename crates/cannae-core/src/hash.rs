//! A stable 64-bit state hash for determinism checks.
//!
//! `std`'s `DefaultHasher` may change between Rust releases, and a hash recorded in a regression
//! test must not. Not cryptographic.

pub struct StateHasher(u64);

impl Default for StateHasher {
    fn default() -> Self {
        Self::new()
    }
}

impl StateHasher {
    pub const fn new() -> Self {
        StateHasher(0x243F_6A88_85A3_08D3)
    }

    pub fn write_u64(&mut self, v: u64) {
        self.0 = (self.0.rotate_left(23) ^ v).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    }

    /// The SplitMix64 finaliser, so that nearby states give unrelated hashes.
    pub fn finish(&self) -> u64 {
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pins the function itself: if this value changes, every recorded hash is void.
    #[test]
    fn pinned() {
        let mut h = StateHasher::new();
        h.write_u64(1);
        h.write_u64(2);
        let v = h.finish();
        let mut h2 = StateHasher::new();
        h2.write_u64(2);
        h2.write_u64(1);
        assert_ne!(v, h2.finish());
        assert_eq!(v, PINNED);
    }

    const PINNED: u64 = 2_611_466_706_963_675_965;
}
