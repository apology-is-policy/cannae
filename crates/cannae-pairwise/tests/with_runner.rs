//! The pairwise substrate under the harness runner, with the toy man: determinism and the
//! dormancy conformance test.

use cannae_core::power::GaitSpeeds;
use cannae_core::view::NoFrames;
use cannae_core::{GridPerception, NoBouts, Rng};
use cannae_harness::toy::Toy;
use cannae_harness::{Dormancy, Runner, Scenario, place};
use cannae_pairwise::{Pairwise, Params};

fn runner(mode: Dormancy) -> Runner<GridPerception, Toy, Pairwise, NoFrames, NoBouts> {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../scenarios/patch/base.toml"
    ))
    .unwrap();
    let s = Scenario::from_toml(&text).unwrap();
    let (bodies, _) = place(&s);
    let n = bodies.len();
    let mut r = Runner::new(
        bodies,
        GridPerception::default(),
        Toy::new(n),
        Pairwise::new(Params::PLACEHOLDER),
        NoFrames,
        NoBouts,
        Rng::new(s.seed),
        GaitSpeeds::PLACEHOLDER,
    );
    r.dormancy = mode;
    r
}

#[test]
fn deterministic_and_dormancy_conformant() {
    let mut a = runner(Dormancy::Skip);
    let mut b = runner(Dormancy::Skip);
    let mut c = runner(Dormancy::EvaluateAll);
    for r in [&mut a, &mut b, &mut c] {
        r.run_until(2_400);
    }
    assert_eq!(a.hashes, b.hashes);
    assert_eq!(a.hashes, c.hashes);
    // Bodies touched rather than passed through: no two standing men ended up closer than
    // one radius.
    let standing: Vec<usize> = (0..a.bodies.len())
        .filter(|&i| a.bodies.posture[i] != cannae_core::Posture::Down)
        .collect();
    let r = Params::PLACEHOLDER.radius_mm as i64;
    for (k, &i) in standing.iter().enumerate() {
        for &j in &standing[k + 1..] {
            let d2 = a.bodies.pos[i].dist_sq(a.bodies.pos[j]);
            assert!(d2 >= r * r, "men {i} and {j} overlap by more than a radius");
        }
    }
}
