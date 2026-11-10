//! Shared core for the Slice 0 experiment.
//!
//! Both engines (the vision of record's contact substrate and the rival's field substrate) build
//! on this crate, so that the experiment compares the man and the substrate and nothing else.
//!
//! Authoritative state is integer throughout: positions in millimetres, time in milliseconds,
//! energy in millijoules, probabilities in Q32. Bit-exact replay then holds on any machine
//! without rules about fused multiply-add or float libraries.

pub mod angle;
pub mod fixed;
pub mod hash;
pub mod observe;
pub mod rng;
pub mod stamina;
pub mod units;

pub use angle::Turn;
pub use fixed::Prob;
pub use observe::{Bodies, BodyId, ManId, Posture, Role, Side, Wound};
pub use rng::{Key, Rng, Stream};
pub use units::{MilliJ, MilliW, SimMs, Vec2};
