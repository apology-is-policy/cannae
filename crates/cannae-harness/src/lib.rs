//! The Slice 0 harness: scenarios, the runner, frame dumps. Metrics follow in S0.3.

pub mod kinematic;
pub mod runner;
pub mod scenario;
pub mod svg;
pub mod toy;

pub use runner::{Dormancy, Runner};
pub use scenario::{Scenario, place};
