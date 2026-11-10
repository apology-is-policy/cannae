//! Conformance: any engine's perception must give the same Views, and the same coarse tests, as
//! the reference `GridPerception` on the same state. The field substrate keeps its counts up to
//! date by events; this is how we know it keeps them right.

use cannae_core::{Bodies, Event, FrameQuery, GridPerception, ManId, Perceive};

/// Compares `candidate` with the reference on one snapshot, for every man.
pub fn views_match<P: Perceive, F: FrameQuery>(
    candidate: &mut P,
    bodies: &Bodies,
    recent: &[Event],
    tick: u32,
    prev_step: u32,
    frames: &F,
) -> Result<(), String> {
    let mut reference = GridPerception::default();
    reference.rebuild(bodies, recent, tick, prev_step);
    candidate.rebuild(bodies, recent, tick, prev_step);
    for i in 0..bodies.len() {
        let m = ManId(i as u32);
        let (a, b) = (
            reference.coarse(bodies, frames, m),
            candidate.coarse(bodies, frames, m),
        );
        if a != b {
            return Err(format!(
                "man {i}, coarse:\n reference {a:?}\n candidate {b:?}"
            ));
        }
        let (a, b) = (
            reference.view(bodies, frames, m),
            candidate.view(bodies, frames, m),
        );
        if a != b {
            return Err(format!(
                "man {i}, view:\n reference {a:?}\n candidate {b:?}"
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::{Scenario, place};
    use cannae_core::view::NoFrames;

    #[test]
    fn the_reference_matches_itself() {
        let s = Scenario::from_toml(include_str!("../../../scenarios/patch/base.toml")).unwrap();
        let (bodies, _) = place(&s);
        let mut p = GridPerception::default();
        assert_eq!(views_match(&mut p, &bodies, &[], 5, 0, &NoFrames), Ok(()));
    }
}
