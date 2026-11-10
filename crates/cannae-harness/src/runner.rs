//! The runner: one battle, one thread, the phases of `cannae_core::schedule` in order.

use cannae_core::hash::StateHasher;
use cannae_core::intent::resolve;
use cannae_core::model::decide;
use cannae_core::power::GaitSpeeds;
use cannae_core::schedule::{BASE_TICK, is_man_step};
use cannae_core::{
    Bodies, Event, EventKind, EventLog, Frames, Gait, Intent, ManId, ManModel, Perceive, Posture,
    Resolver, Rng, Substrate, Target, Wound,
};

/// How dormant men are treated. All three must give identical hashes; that is the conformance
/// test of a man model's dormancy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dormancy {
    /// Skip dormant men: the normal mode.
    Skip,
    /// Ignore dormancy and decide for everyone.
    EvaluateAll,
    /// Skip dormant men, but build their Views and assert that their hazards are exactly zero.
    Check,
}

pub struct Runner<P, M, S, F, R> {
    pub bodies: Bodies,
    pub log: EventLog,
    pub tick: u32,
    pub perception: P,
    pub model: M,
    pub substrate: S,
    pub frames: F,
    pub resolver: R,
    pub rng: Rng,
    pub speeds: GaitSpeeds,
    pub dormancy: Dormancy,
    /// The state hash after every man step.
    pub hashes: Vec<u64>,
    pub evaluated: u64,
    pub skipped: u64,
    intents: Vec<(Intent, Gait)>,
    targets: Vec<Target>,
    prev_step: u32,
}

/// Applies an event's effect on observable state.
pub fn apply(bodies: &mut Bodies, e: &Event) {
    let i = e.man.0 as usize;
    match e.kind {
        EventKind::Posture { to, .. } => bodies.posture[i] = to,
        EventKind::Wound { to } => {
            bodies.wound[i] = to;
            if to >= Wound::Disabled {
                bodies.posture[i] = Posture::Down;
            }
        }
        EventKind::StandardDown { .. } | EventKind::StandardUp { .. } | EventKind::Bout { .. } => {}
    }
}

impl<P, M, S, F, R> Runner<P, M, S, F, R>
where
    P: Perceive,
    M: ManModel,
    S: Substrate,
    F: Frames,
    R: Resolver,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        bodies: Bodies,
        perception: P,
        model: M,
        substrate: S,
        frames: F,
        resolver: R,
        rng: Rng,
        speeds: GaitSpeeds,
    ) -> Self {
        let n = bodies.len();
        let targets = bodies
            .pos
            .iter()
            .map(|&point| Target {
                point,
                ..Default::default()
            })
            .collect();
        Runner {
            bodies,
            log: EventLog::default(),
            tick: 0,
            perception,
            model,
            substrate,
            frames,
            resolver,
            rng,
            speeds,
            dormancy: Dormancy::Skip,
            hashes: Vec::new(),
            evaluated: 0,
            skipped: 0,
            intents: vec![(Intent::Stand, Gait::Stand); n],
            targets,
            prev_step: 0,
        }
    }

    /// Observable state, inner state and the length of the log, in one hash.
    pub fn state_hash(&self) -> u64 {
        let mut h = StateHasher::new();
        h.write_u64(self.bodies.state_hash());
        h.write_u64(self.model.inner_hash());
        h.write_u64(self.log.all().len() as u64);
        h.finish()
    }

    fn apply_phase(&mut self, batch: Vec<Event>) {
        for e in &batch {
            apply(&mut self.bodies, e);
            self.model.on_event(e);
        }
        self.log.append_phase(batch);
    }

    fn man_step(&mut self) {
        let t = self.tick;
        let mut batch = Vec::new();
        self.frames.update(&mut self.bodies, t, &mut batch);
        self.apply_phase(batch);

        self.perception.rebuild(
            &self.bodies,
            self.log.range(self.prev_step, t),
            t,
            self.prev_step,
        );
        let mut batch = Vec::new();
        for i in 0..self.bodies.len() {
            let man = ManId(i as u32);
            if self.bodies.posture[i] == Posture::Down {
                self.intents[i] = (Intent::Stand, Gait::Stand);
                continue;
            }
            let own = self.bodies.get(man);
            // Everyone decides at the first step, so that every man has an intent.
            if t > 0 && self.dormancy != Dormancy::EvaluateAll {
                let coarse = self.perception.coarse(&self.bodies, &self.frames, man);
                if self.model.dormant(man, &own, &coarse, t) {
                    if self.dormancy == Dormancy::Check {
                        let v = self.perception.view(&self.bodies, &self.frames, man);
                        let h = self.model.explain(man, &v);
                        assert!(
                            h.is_zero(),
                            "dormant man {man:?} at tick {t} has hazards {h:?}"
                        );
                    }
                    self.skipped += 1;
                    continue;
                }
            }
            let view = self.perception.view(&self.bodies, &self.frames, man);
            let d = decide(&mut self.model, &view, &self.rng);
            self.evaluated += 1;
            self.intents[i] = (d.intent, d.gait);
            if let Some((to, causes)) = d.change {
                batch.push(Event {
                    tick: t,
                    man,
                    pos: own.pos,
                    kind: EventKind::Posture {
                        from: own.posture,
                        to,
                    },
                    causes,
                });
            }
        }
        self.apply_phase(batch);

        for (i, &(intent, gait)) in self.intents.iter().enumerate() {
            self.targets[i] = resolve(
                ManId(i as u32),
                intent,
                gait,
                &self.bodies,
                &self.frames,
                &self.speeds,
            );
        }
        self.prev_step = t;
    }

    /// One base tick. Returns whether it was a man step, after which observers may record.
    pub fn step(&mut self) -> bool {
        let t = self.tick;
        let man_step = is_man_step(t);
        if man_step {
            self.man_step();
        }
        let mut moved = Vec::new();
        self.substrate.advance(
            &mut self.bodies,
            &self.targets,
            BASE_TICK,
            &self.rng,
            t,
            &mut moved,
        );
        if !moved.is_empty() {
            self.apply_phase(moved);
        }
        if man_step {
            let mut out = Vec::new();
            self.resolver
                .resolve(&self.bodies, &self.frames, t, &self.rng, &mut out);
            self.apply_phase(out);
            self.hashes.push(self.state_hash());
        }
        self.tick += 1;
        man_step
    }

    pub fn run_until(&mut self, tick: u32) {
        while self.tick < tick {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kinematic::Kinematic;
    use crate::scenario::{Scenario, place};
    use crate::toy::Toy;
    use cannae_core::view::NoFrames;
    use cannae_core::{GridPerception, NoBouts};

    const FACE_OFF: &str = r#"
        name = "face-off"
        seed = 11
        duration_s = 60

        [[body]]
        name = "line"
        side = "A"
        troop = "recruits"
        files = 10
        ranks = 4
        file_mm = 900
        rank_mm = 1200
        centre_mm = [0, 0]
        facing_deg = 0
        closer = true

        [[body]]
        name = "band"
        side = "B"
        troop = "warriors"
        files = 10
        ranks = 4
        file_mm = 900
        rank_mm = 1200
        centre_mm = [14000, 0]
        facing_deg = 180
        leader = "front-centre"
    "#;

    type ToyRunner = Runner<GridPerception, Toy, Kinematic, NoFrames, NoBouts>;

    fn runner(mode: Dormancy) -> ToyRunner {
        let s = Scenario::from_toml(FACE_OFF).unwrap();
        let (bodies, _) = place(&s);
        let n = bodies.len();
        let mut r = Runner::new(
            bodies,
            GridPerception::default(),
            Toy::new(n),
            Kinematic::default(),
            NoFrames,
            NoBouts,
            Rng::new(s.seed),
            GaitSpeeds::PLACEHOLDER,
        );
        r.dormancy = mode;
        r
    }

    #[test]
    fn same_seed_same_battle() {
        let mut a = runner(Dormancy::Skip);
        let mut b = runner(Dormancy::Skip);
        a.run_until(1200);
        b.run_until(1200);
        assert_eq!(a.hashes, b.hashes);
        // The toy is not idle: men gave ground and turned.
        let turned = a.log.all().iter().filter(|e| {
            matches!(
                e.kind,
                EventKind::Posture {
                    to: Posture::Turned,
                    ..
                }
            )
        });
        assert!(turned.count() > 5, "{} events", a.log.all().len());
    }

    #[test]
    fn dormancy_cannot_change_a_bit() {
        let mut skip = runner(Dormancy::Skip);
        let mut all = runner(Dormancy::EvaluateAll);
        let mut check = runner(Dormancy::Check);
        skip.run_until(1200);
        all.run_until(1200);
        check.run_until(1200);
        assert_eq!(skip.hashes, all.hashes);
        assert_eq!(skip.hashes, check.hashes);
        assert!(skip.skipped > 0 && skip.evaluated < all.evaluated);
    }

    #[test]
    fn a_different_seed_is_a_different_battle() {
        let mut a = runner(Dormancy::Skip);
        let mut b = runner(Dormancy::Skip);
        b.rng = Rng::new(12);
        a.run_until(1200);
        b.run_until(1200);
        assert_ne!(a.hashes.last(), b.hashes.last());
    }
}

#[cfg(test)]
mod with_frames_and_bouts {
    use super::*;
    use crate::kinematic::Kinematic;
    use crate::scenario::{Scenario, frames, place};
    use cannae_core::view::{Coarse, Rel, View};
    use cannae_core::{
        BodyFrames, BoutParams, Bouts, Cause, GridPerception, Hazards, Kit, Observed, Side,
        Transition,
    };

    const CLASH: &str = r#"
        name = "clash"
        seed = 5
        duration_s = 120

        [[body]]
        name = "line"
        side = "A"
        troop = "hastati"
        files = 6
        ranks = 4
        file_mm = 1000
        rank_mm = 1200
        centre_mm = [0, 0]
        facing_deg = 0
        closer = true

        [[body]]
        name = "band"
        side = "B"
        troop = "warriors"
        files = 6
        ranks = 4
        file_mm = 1000
        rank_mm = 1200
        centre_mm = [16000, 0]
        facing_deg = 180
        leader = "front-centre"
        tie = "loose"

        [[order]]
        body = "band"
        at_s = 1
        order = "advance"
    "#;

    /// A test man, not a rung: side B steps forward at any enemy in his window and closes with
    /// him; side A stands. Everyone counts the falls he sees.
    struct Brawler {
        falls_seen: Vec<u32>,
    }

    impl ManModel for Brawler {
        fn dormant(&self, _: ManId, own: &Observed, c: &Coarse, _: u32) -> bool {
            own.posture == Posture::Steady && !c.enemy_near && !c.shock_near
        }

        fn absorb(&mut self, man: ManId, v: &View) {
            self.falls_seen[man.0 as usize] += v.shocks.falls as u32;
        }

        fn explain(&self, _: ManId, v: &View) -> Hazards {
            let foes = v.counts.total(Rel::Enemy, Posture::Steady)
                + v.counts.total(Rel::Enemy, Posture::Forward)
                + v.counts.total(Rel::Enemy, Posture::Back);
            let mut h = Hazards::NONE;
            if v.own.side == Side::B && v.own.posture == Posture::Steady && foes > 0 {
                h.ways[0] = Some(Transition {
                    to: Posture::Forward,
                    rate_q16: 1 << 16,
                    causes: [Cause::Ardour, Cause::None, Cause::None],
                });
            }
            h
        }

        fn intent(&self, _: ManId, v: &View, posture: Posture) -> (Intent, Gait) {
            match (posture, v.near().next()) {
                (Posture::Forward, Some(e)) => (
                    Intent::Close {
                        enemy: e.id,
                        stand_off_mm: 1_000,
                        press: 0,
                    },
                    Gait::Walk,
                ),
                (Posture::Forward, None) => (
                    Intent::HoldSlot {
                        forward_mm: 3_000,
                        press: 0,
                    },
                    Gait::Walk,
                ),
                _ => (
                    Intent::HoldSlot {
                        forward_mm: 0,
                        press: 0,
                    },
                    Gait::Walk,
                ),
            }
        }

        fn on_event(&mut self, _: &Event) {}

        fn inner_hash(&self) -> u64 {
            let mut h = StateHasher::new();
            for f in &self.falls_seen {
                h.write_u64(*f as u64);
            }
            h.finish()
        }
    }

    type Clash = Runner<GridPerception, Brawler, Kinematic, BodyFrames, Bouts>;

    fn clash(mode: Dormancy) -> Clash {
        let s = Scenario::from_toml(CLASH).unwrap();
        let (bodies, placed) = place(&s);
        let f = frames(&s, &placed, &bodies).unwrap();
        let n = bodies.len();
        // Deadlier than any prior, so that two minutes are enough to see men fall.
        let params = BoutParams {
            rate_q16: 1 << 15,
            ..BoutParams::PLACEHOLDER
        };
        let mut r = Runner::new(
            bodies,
            GridPerception::default(),
            Brawler {
                falls_seen: vec![0; n],
            },
            Kinematic::default(),
            f,
            Bouts::new(params, vec![Kit::PLAIN; n]),
            Rng::new(s.seed),
            GaitSpeeds::PLACEHOLDER,
        );
        r.dormancy = mode;
        r
    }

    #[test]
    fn the_band_marches_in_order_fights_and_the_files_close() {
        use cannae_core::view::nb;
        use cannae_core::{FrameQuery, Role};
        let mut r = clash(Dormancy::Skip);
        let start = r.bodies.clone();
        // Five seconds: the band was told to advance at one second and has walked for four, at
        // 1.3 m/s, every man still in his place.
        r.run_until(100);
        let band: Vec<usize> = (0..r.bodies.len())
            .filter(|&i| r.bodies.side[i] == Side::B)
            .collect();
        for &i in &band {
            let walked = start.pos[i].x - r.bodies.pos[i].x;
            assert!((4_800..=5_600).contains(&walked), "man {i} walked {walked}");
            assert!(r.bodies.in_place[i], "man {i} lost his place on the march");
        }
        r.run_until(2_400);
        let bouts = r
            .log
            .all()
            .iter()
            .filter(|e| matches!(e.kind, EventKind::Bout { .. }))
            .count();
        let down = |side: Side| {
            (0..r.bodies.len())
                .filter(|&i| r.bodies.side[i] == side && r.bodies.posture[i] == Posture::Down)
                .count()
        };
        let fallen = down(Side::A) + down(Side::B);
        assert!(bouts > 50, "{bouts} bouts");
        assert!(fallen > 4, "{fallen} fell");
        // Every fall by a bout is a posture event, so the men around saw it.
        let seen: u32 = r.model.falls_seen.iter().sum();
        assert!(seen as usize > fallen, "{seen} sightings of {fallen} falls");
        // Files closed over the fallen: men who began behind the front rank now head a file.
        let promoted = (0..r.bodies.len())
            .filter(|&i| matches!(r.bodies.role[i], Role::Soldier | Role::Carrier))
            .filter_map(|i| r.frames.slot(ManId(i as u32)))
            .filter(|s| s.slots & nb::FRONT == 0)
            .count();
        assert!(promoted > 0, "no file closed");
    }

    #[test]
    fn frames_and_bouts_keep_dormancy_exact() {
        let mut skip = clash(Dormancy::Skip);
        let mut all = clash(Dormancy::EvaluateAll);
        let mut check = clash(Dormancy::Check);
        let mut again = clash(Dormancy::Skip);
        for r in [&mut skip, &mut all, &mut check, &mut again] {
            r.run_until(2_400);
        }
        assert_eq!(skip.hashes, all.hashes);
        assert_eq!(skip.hashes, check.hashes);
        assert_eq!(skip.hashes, again.hashes);
        assert!(skip.skipped > 0);
    }
}
