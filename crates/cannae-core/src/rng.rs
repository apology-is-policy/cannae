//! Counter-based random numbers: Philox4x32-10 (Salmon, Moraes, Dror and Shaw, "Parallel
//! random numbers: as easy as 1, 2, 3", SC 2011).
//!
//! Every draw is a pure function of the seed and a counter built from (stream, entity, tick,
//! purpose). Results do not depend on the order in which draws are made, on thread count, or on
//! which engine asks, so the two engines in the experiment can share streams draw for draw.

use crate::fixed::Prob;

/// Separate streams, so that the variance of an outcome can be split by source: what was fixed
/// by hidden traits, what by combat dice, what by command friction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Stream {
    /// Dispositions and anything else drawn once at setup.
    Setup = 0,
    /// Posture events: going forward, giving ground, turning, rallying.
    Hazard = 1,
    /// Bouts.
    Combat = 2,
    /// Volleys.
    Missile = 3,
    /// Officers, messengers and signals.
    Command = 4,
    /// Any randomness inside a substrate, such as tie-breaking jitter.
    Substrate = 5,
}

/// The address of one draw.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Key {
    pub stream: Stream,
    /// Usually a `ManId`; a body or officer id where that is the actor.
    pub entity: u32,
    /// Usually the man step number.
    pub tick: u32,
    /// Distinguishes draws by the same entity on the same tick. Only the low 24 bits are used.
    pub purpose: u32,
    /// For repeated draws with one purpose, such as one per bout partner.
    pub index: u32,
}

impl Key {
    pub const fn new(stream: Stream, entity: u32, tick: u32, purpose: u32) -> Self {
        Key {
            stream,
            entity,
            tick,
            purpose,
            index: 0,
        }
    }

    pub const fn at(self, index: u32) -> Self {
        Key { index, ..self }
    }

    const fn counter(self) -> [u32; 4] {
        [
            self.entity,
            self.tick,
            ((self.stream as u32) << 24) | (self.purpose & 0x00FF_FFFF),
            self.index,
        ]
    }
}

const STREAMS: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Branch {
    from_tick: u32,
    key: [u32; 2],
}

/// The generator: a seed, plus the branches that forks have taken.
///
/// A fork gives chosen streams, from a chosen tick on, a key derived from (seed, branch). The
/// branch lives in the key rather than the counter, so there are 2^32 of them and no counter
/// bits are spent.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Rng {
    base: [u32; 2],
    branches: [Option<Branch>; STREAMS],
}

impl Rng {
    pub const fn new(seed: u64) -> Self {
        Rng {
            base: [seed as u32, (seed >> 32) as u32],
            branches: [None; STREAMS],
        }
    }

    /// The same generator, except that draws on `stream` at ticks from `from_tick` on come
    /// from branch `branch`. Draws before that tick, and on other streams, are unchanged.
    pub fn fork(&self, stream: Stream, from_tick: u32, branch: u32) -> Rng {
        let d = philox4x32_10([branch, 0xB7A9_C4D1, stream as u32, from_tick], self.base);
        let mut r = *self;
        r.branches[stream as usize] = Some(Branch {
            from_tick,
            key: [d[0], d[1]],
        });
        r
    }

    fn key_for(&self, k: Key) -> [u32; 2] {
        match self.branches[k.stream as usize] {
            Some(b) if k.tick >= b.from_tick => b.key,
            _ => self.base,
        }
    }

    /// Four independent `u32` words.
    pub fn block(&self, k: Key) -> [u32; 4] {
        philox4x32_10(k.counter(), self.key_for(k))
    }

    pub fn u32(&self, k: Key) -> u32 {
        self.block(k)[0]
    }

    pub fn chance(&self, p: Prob, k: Key) -> bool {
        p.hits(self.u32(k))
    }

    /// Uniform in `0..n` by multiply-shift. The bias is below 2^-32 * n, which no test can see.
    pub fn below(&self, n: u32, k: Key) -> u32 {
        ((self.u32(k) as u64 * n as u64) >> 32) as u32
    }

    /// A standard normal draw in Q16 (65,536 is one standard deviation): the Irwin-Hall sum of
    /// twelve 10-bit uniforms from one block. Bounded at six standard deviations, which suits
    /// trait distributions.
    pub fn normal_q16(&self, k: Key) -> i32 {
        let w = self.block(k);
        let v = w[0] as u128 | (w[1] as u128) << 32 | (w[2] as u128) << 64 | (w[3] as u128) << 96;
        let sum: i32 = (0..12).map(|i| ((v >> (10 * i)) & 0x3FF) as i32).sum();
        (sum - 6138) * 64
    }
}

const M0: u64 = 0xD251_1F53;
const M1: u64 = 0xCD9E_8D57;
const W0: u32 = 0x9E37_79B9;
const W1: u32 = 0xBB67_AE85;

fn round(c: [u32; 4], k: [u32; 2]) -> [u32; 4] {
    let p0 = M0 * c[0] as u64;
    let p1 = M1 * c[2] as u64;
    [
        (p1 >> 32) as u32 ^ c[1] ^ k[0],
        p1 as u32,
        (p0 >> 32) as u32 ^ c[3] ^ k[1],
        p0 as u32,
    ]
}

/// Philox4x32 with ten rounds, as in Random123.
pub fn philox4x32_10(mut c: [u32; 4], mut k: [u32; 2]) -> [u32; 4] {
    c = round(c, k);
    for _ in 1..10 {
        k = [k[0].wrapping_add(W0), k[1].wrapping_add(W1)];
        c = round(c, k);
    }
    c
}

#[cfg(test)]
#[allow(clippy::float_arithmetic)]
mod tests {
    use super::*;

    /// Known-answer vectors from Random123's kat_vectors for philox4x32 with 10 rounds.
    #[test]
    fn known_answers() {
        assert_eq!(
            philox4x32_10([0; 4], [0; 2]),
            [0x6627_e8d5, 0xe169_c58d, 0xbc57_ac4c, 0x9b00_dbd8]
        );
        assert_eq!(
            philox4x32_10([u32::MAX; 4], [u32::MAX; 2]),
            [0x408f_276d, 0x41c8_3b0e, 0xa20b_c7c6, 0x6d54_51fd]
        );
        assert_eq!(
            philox4x32_10(
                [0x243f_6a88, 0x85a3_08d3, 0x1319_8a2e, 0x0370_7344],
                [0xa409_3822, 0x299f_31d0]
            ),
            [0xd16c_fe09, 0x94fd_cceb, 0x5001_e420, 0x2412_6ea1]
        );
    }

    #[test]
    fn draws_are_addressed_not_sequenced() {
        let r = Rng::new(42);
        let k1 = Key::new(Stream::Hazard, 7, 100, 1);
        let k2 = Key::new(Stream::Combat, 7, 100, 1);
        let first = r.u32(k1);
        let _ = r.u32(k2);
        assert_eq!(r.u32(k1), first);
        assert_ne!(r.u32(k1), r.u32(k2));
        assert_ne!(Rng::new(43).u32(k1), first);
    }

    #[test]
    fn forks_change_only_chosen_streams_after_the_tick() {
        let r = Rng::new(7);
        let f = r.fork(Stream::Combat, 1000, 3);
        let before = Key::new(Stream::Combat, 5, 999, 0);
        let after = Key::new(Stream::Combat, 5, 1000, 0);
        let other = Key::new(Stream::Hazard, 5, 2000, 0);
        assert_eq!(f.u32(before), r.u32(before));
        assert_ne!(f.u32(after), r.u32(after));
        assert_eq!(f.u32(other), r.u32(other));
        assert_ne!(f.u32(after), r.fork(Stream::Combat, 1000, 4).u32(after));
        assert_eq!(f.u32(after), r.fork(Stream::Combat, 1000, 3).u32(after));
        assert_ne!(r.u32(after), r.u32(after.at(1)));
    }

    #[test]
    fn normal_has_unit_spread() {
        let r = Rng::new(1);
        let n = 20_000;
        let (mut s, mut s2) = (0f64, 0f64);
        for i in 0..n {
            let z = r.normal_q16(Key::new(Stream::Setup, i, 0, 0)) as f64 / 65_536.0;
            s += z;
            s2 += z * z;
        }
        let mean = s / n as f64;
        let var = s2 / n as f64 - mean * mean;
        assert!(mean.abs() < 0.03, "mean {mean}");
        assert!((var - 1.0).abs() < 0.05, "variance {var}");
    }

    #[test]
    fn below_and_chance_are_fair() {
        let r = Rng::new(9);
        let mut counts = [0u32; 4];
        let mut hits = 0;
        let quarter = Prob::from_ratio(1, 4);
        for i in 0..40_000 {
            counts[r.below(4, Key::new(Stream::Combat, i, 0, 0)) as usize] += 1;
            hits += r.chance(quarter, Key::new(Stream::Combat, i, 0, 1)) as u32;
        }
        for c in counts {
            assert!((9_500..10_500).contains(&c), "{counts:?}");
        }
        assert!((9_500..10_500).contains(&hits), "{hits}");
    }
}
