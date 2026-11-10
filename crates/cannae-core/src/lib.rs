//! Shared core for the Slice 0 experiment.
//!
//! Both engines (the vision of record's contact substrate and the rival's field substrate) build
//! on this crate, so that the experiment compares the man and the substrate and nothing else.
//!
//! Authoritative state is integer throughout: positions in millimetres, time in milliseconds,
//! energy in millijoules, probabilities in Q32. Bit-exact replay then holds on any machine
//! without rules about fused multiply-add or float libraries.

pub mod angle;
pub mod bouts;
pub mod event;
pub mod fixed;
pub mod frames;
pub mod hash;
pub mod hazard;
pub mod intent;
pub mod model;
pub mod observe;
pub mod power;
pub mod rng;
pub mod schedule;
pub mod stamina;
pub mod units;
pub mod view;
pub mod volleys;
pub mod wall;

pub use angle::Turn;
pub use bouts::{BoutParams, Bouts, Kit};
pub use event::{BoutOutcome, Cause, Causes, Event, EventKind, EventLog};
pub use fixed::Prob;
pub use frames::{BodyFrames, Layout, Tie};
pub use hazard::{Hazards, Transition};
pub use intent::{Gait, Intent, Target};
pub use model::{Decision, Frames, ManModel, NoBouts, Resolver, Substrate};
pub use observe::{Bodies, BodyId, ManId, Observed, Posture, Role, Side, Wound};
pub use rng::{Key, Rng, Stream};
pub use stamina::{CpPerMille, CpUs, StaminaParams};
pub use units::{SimMs, Vec2};
pub use view::{Coarse, FrameQuery, GridPerception, Perceive, View};
pub use volleys::{Volley, Volleys};
pub use wall::{Wall, WallIndex};
