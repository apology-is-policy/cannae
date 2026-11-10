//! How dense a crowd pressed against a wall gets, and the summed overlap it reaches: the check
//! behind deriving `crush_mm` from a density (priors-shared.md, crush onset).
//!
//! Four hundred men, 10 across a channel 11 m wide, walk and shove into a wall for 90 s. For
//! the men within 3 m of the wall it prints the local density (bodies within a metre, per m²)
//! and the summed overlap, beside what hexagonal packing predicts at the median density,
//! 6 (2r - s) with s = sqrt(2 / (sqrt(3) rho)).
//!
//! `cargo run --release -p cannae-pairwise --example press_wall`
#![allow(clippy::float_arithmetic)]
use cannae_core::schedule::BASE_TICK;
use cannae_core::{Bodies, Observed, Rng, Substrate, Target, Turn, Vec2, Wall};
use cannae_pairwise::{Pairwise, Params};

fn pct(v: &mut [f64], q: f64) -> f64 {
    v.sort_by(f64::total_cmp);
    v[((v.len() - 1) as f64 * q) as usize]
}

fn main() {
    // The placeholders, then each prior's far end alone, then the softest corner.
    for (core, stiff, press) in [
        (150, 128, 300),
        (150, 128, 700),
        (200, 128, 300),
        (150, 64, 300),
        (200, 64, 700),
    ] {
        let p = Params {
            core_mm: core,
            stiffness_q8: stiff,
            press_mm_s: press,
            crush_mm: i32::MAX,
            ..Params::PLACEHOLDER
        };
        let mut b = Bodies::default();
        for r in 0..40 {
            for f in 0..10 {
                b.push(Observed {
                    pos: Vec2::new(1_000 + r * 900, -4_500 + f * 1_000 + (r % 2) * 300),
                    facing: Turn(Turn::HALF),
                    ..Default::default()
                });
            }
        }
        let mut s = Pairwise::new(p);
        s.walls = vec![
            Wall::new(Vec2::new(0, -5_500), Vec2::new(0, 5_500)),
            Wall::new(Vec2::new(0, 5_500), Vec2::new(60_000, 5_500)),
            Wall::new(Vec2::new(0, -5_500), Vec2::new(60_000, -5_500)),
        ];
        let rng = Rng::new(1);
        let mut out = Vec::new();
        let tg: Vec<Target> = (0..b.len())
            .map(|i| Target {
                point: Vec2::new(-5_000, b.pos[i].y),
                speed: 1_300,
                face: Some(Turn(Turn::HALF)),
                press: 255,
            })
            .collect();
        for tick in 0..90_000 / BASE_TICK.0 {
            s.advance(&mut b, &tg, BASE_TICK, &rng, tick, &mut out);
            out.clear();
        }
        let near: Vec<usize> = (0..b.len()).filter(|&i| b.pos[i].x < 3_000).collect();
        let mut dens: Vec<f64> = near
            .iter()
            .map(|&i| {
                (0..b.len())
                    .filter(|&j| b.pos[j].dist_sq(b.pos[i]) <= 1_000_000)
                    .count() as f64
                    / std::f64::consts::PI
            })
            .collect();
        let mut pr: Vec<f64> = near.iter().map(|&i| s.pressure()[i] as f64).collect();
        let rho = pct(&mut dens.clone(), 0.5);
        let s_m = (2.0 / (3f64.sqrt() * rho)).sqrt();
        let hex = 6.0 * (2.0 * p.radius_mm as f64 - s_m * 1000.0);
        println!(
            "core {core:>3} stiffness {stiff:>3} press {press:>3}: density p50 {rho:.1} p90 {:.1} \
             max {:.1} per m2; overlap p50 {:.0} p90 {:.0} max {:.0} mm (hex at p50: {hex:.0})",
            pct(&mut dens.clone(), 0.9),
            pct(&mut dens, 1.0),
            pct(&mut pr.clone(), 0.5),
            pct(&mut pr.clone(), 0.9),
            pct(&mut pr, 1.0),
        );
    }
}
