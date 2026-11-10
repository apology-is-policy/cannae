//! The rival's engine below the man: a perception kept as counts, and a substrate that moves
//! men on a density field once every man step.
//!
//! Order decides the physics ([docs/rival/02](../../../docs/rival/02-order-and-contact.md) §5):
//!
//! - a man in his place moves by his tie to his slot, and nothing else;
//! - a man out of his place walks through the field, slower as it grows denser, and a packed
//!   field pushes its men apart;
//! - enemies are kept apart by their exact positions, never by cells;
//! - where a crowd is both packed and moving, its men also touch each other.
//!
//! One battle runs on one thread, and all state is integer.

pub mod curve;
pub mod field;
pub mod perceive;
pub mod tiles;

#[cfg(test)]
mod tests;

pub use field::{Contact, Field, Params, Stats};
pub use perceive::FieldPerception;
