//! The Slice 0 harness: scenarios, the runner, metrics, frame dumps.

pub mod kinematic;
pub mod metrics;
pub mod runner;
pub mod scenario;
pub mod svg;
pub mod toy;

pub use runner::{Dormancy, Runner};
pub use scenario::{Scenario, place};
