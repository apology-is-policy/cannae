//! Bouts: what happens when a man who has stepped forward has an enemy within reach
//! (docs/rival/02 sec. 8).
//!
//! A bout is one exchange, drawn at a rate and settled in one step from what a witness could
//! see: who stands where, facing which way, in what posture, with what wounds. The man who
//! stepped forward strikes; the man he struck at answers, if he can. A man can attend to one
//! opponent at a time, so a second attacker strikes unanswered at a man who is busy. That, and
//! nothing else, is why two on one is deadly here.
//!
//! Only men who are forward start bouts. Two steady lines out of reach of each other do no
//! harm at all.
//!
//! Every chance below is a named prior. The values are placeholders until the lethality
//! search (S0.4) bounds them.

use crate::angle::Turn;
use crate::event::{BoutOutcome, Cause, Event, EventKind};
use crate::fixed::Prob;
use crate::model::Resolver;
use crate::observe::{Bodies, ManId, Posture, Wound};
use crate::rng::{Key, Rng, Stream};
use crate::schedule::MAN_STEP;
use crate::view::{CELL_MM, FrameQuery, REACH_MM, Sector};

/// The purpose word for bout draws on the combat stream.
pub const PURPOSE_BOUT: u32 = 0;

/// What a man fights with, as far as a bout can tell: fixed at setup, per man.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Kit {
    /// How far he can strike, at most [`REACH_MM`].
    pub reach_mm: i32,
    /// How well his shield and armour keep him from harm, per mille: 1000 is the reference
    /// man, less is better protected.
    pub guard_pm: u32,
    /// His skill, in steps: each step by which a striker out-skills his target multiplies the
    /// chance of harm by [`BoutParams::skill_step_pm`].
    pub skill: i8,
}

impl Kit {
    pub const PLAIN: Kit = Kit {
        reach_mm: 1_500,
        guard_pm: 1_000,
        skill: 0,
    };
}

/// The chances of a bout. Multipliers are per mille and apply to the chance of harm.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoutParams {
    /// Bouts per second for a man who is forward with an enemy in reach. Q16.
    pub rate_q16: u32,
    /// The chance that a strike wounds: the striker forward, the target steady and facing him,
    /// attending to him, both sound and equally skilled, with the reference shield and armour.
    pub harm: Prob,
    /// The target has himself stepped forward.
    pub target_forward_pm: u32,
    /// The target is giving ground.
    pub target_back_pm: u32,
    /// The target has turned his back. This multiplier is the slaughter; when it applies, the
    /// side of approach does not.
    pub target_turned_pm: u32,
    /// The strike comes at the target's left, where his shield is.
    pub shield_side_pm: u32,
    /// The strike comes at his right, the open side.
    pub open_side_pm: u32,
    /// The strike comes from behind a man who has not turned.
    pub behind_pm: u32,
    /// The target is attending to a nearer attacker.
    pub unattended_pm: u32,
    pub target_hampered_pm: u32,
    pub striker_hampered_pm: u32,
    /// The answering strike of a man who did not step forward.
    pub answer_pm: u32,
    pub skill_step_pm: u32,
    /// Of strikes that wound, the shares per mille that disable and that kill. The rest hamper.
    pub disable_pm: u32,
    pub kill_pm: u32,
    /// The chance that a strike which does not wound drives its target back a step.
    pub drive_back: Prob,
}

impl BoutParams {
    pub const PLACEHOLDER: BoutParams = BoutParams {
        // One bout in ten seconds.
        rate_q16: 6_554,
        harm: Prob::from_ratio(1, 20),
        target_forward_pm: 1_500,
        target_back_pm: 1_000,
        target_turned_pm: 8_000,
        shield_side_pm: 1_000,
        open_side_pm: 2_000,
        behind_pm: 4_000,
        unattended_pm: 2_000,
        target_hampered_pm: 1_500,
        striker_hampered_pm: 600,
        answer_pm: 700,
        skill_step_pm: 1_250,
        disable_pm: 350,
        kill_pm: 150,
        drive_back: Prob::from_ratio(3, 10),
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Engaged {
    defender: ManId,
    dist_sq: i64,
    attacker: ManId,
}

/// The bout resolver. Its scratch lists carry nothing from one step to the next.
#[derive(Clone, Debug)]
pub struct Bouts {
    pub params: BoutParams,
    kits: Vec<Kit>,
    cells: Vec<(i64, u32)>,
    engaged: Vec<Engaged>,
    took: Vec<(ManId, BoutOutcome)>,
}

const ONE: u128 = 1 << 32;

fn cell_key(cx: i32, cy: i32) -> i64 {
    ((cy as i64) << 32) | cx as u32 as i64
}

fn scale(p: u128, per_mille: u32) -> u128 {
    p * per_mille as u128 / 1000
}

impl Bouts {
    /// One kit per man, indexed by `ManId`.
    pub fn new(params: BoutParams, kits: Vec<Kit>) -> Self {
        assert!(
            kits.iter()
                .all(|k| (0..=REACH_MM).contains(&(k.reach_mm as i64))),
            "a reach beyond what perception looks for"
        );
        assert!(params.disable_pm + params.kill_pm <= 1000);
        Bouts {
            params,
            kits,
            cells: Vec::new(),
            engaged: Vec::new(),
            took: Vec::new(),
        }
    }

    fn index(&mut self, bodies: &Bodies) {
        self.cells.clear();
        for (i, p) in bodies.pos.iter().enumerate() {
            if bodies.posture[i] != Posture::Down {
                let key = cell_key(p.x.div_euclid(CELL_MM), p.y.div_euclid(CELL_MM));
                self.cells.push((key, i as u32));
            }
        }
        self.cells.sort_unstable();
    }

    /// The nearest standing enemy within `a`'s reach, ties to the lower id.
    fn opponent(&self, bodies: &Bodies, a: usize) -> Option<(i64, ManId)> {
        let p = bodies.pos[a];
        let reach = self.kits[a].reach_mm as i64;
        let (cx, cy) = (p.x.div_euclid(CELL_MM), p.y.div_euclid(CELL_MM));
        let span = (reach as i32).div_euclid(CELL_MM) + 1;
        let mut best = None;
        for dy in -span..=span {
            for dx in -span..=span {
                let key = cell_key(cx + dx, cy + dy);
                let lo = self.cells.partition_point(|c| c.0 < key);
                for &(_, j) in self.cells[lo..].iter().take_while(|c| c.0 == key) {
                    let j = j as usize;
                    if bodies.side[j] == bodies.side[a] {
                        continue;
                    }
                    let d = p.dist_sq(bodies.pos[j]);
                    if d <= reach * reach {
                        let cand = (d, ManId(j as u32));
                        if best.is_none_or(|b| cand < b) {
                            best = Some(cand);
                        }
                    }
                }
            }
        }
        best
    }

    /// Whether `d` can answer `a`: he stands, `a` is in front of him, and within his own reach.
    fn can_answer(&self, bodies: &Bodies, d: usize, a: usize, dist_sq: i64) -> bool {
        let reach = self.kits[d].reach_mm as i64;
        matches!(
            bodies.posture[d],
            Posture::Steady | Posture::Forward | Posture::Back
        ) && bodies.wound[d] < Wound::Disabled
            && dist_sq <= reach * reach
            && Turn::of(bodies.pos[a] - bodies.pos[d])
                .is_none_or(|dir| Sector::of(bodies.facing[d], dir) == Sector::Front)
    }

    /// What one strike does to its target, from one uniform draw.
    fn strike(
        &self,
        bodies: &Bodies,
        striker: usize,
        target: usize,
        attended: bool,
        draw: u32,
    ) -> BoutOutcome {
        let q = &self.params;
        let mut p = q.harm.q32() as u128;
        match bodies.posture[target] {
            Posture::Turned => p = scale(p, q.target_turned_pm),
            posture => {
                p = scale(
                    p,
                    match posture {
                        Posture::Forward => q.target_forward_pm,
                        Posture::Back => q.target_back_pm,
                        _ => 1000,
                    },
                );
                let from = Turn::of(bodies.pos[striker] - bodies.pos[target])
                    .map_or(Sector::Front, |dir| Sector::of(bodies.facing[target], dir));
                p = scale(
                    p,
                    match from {
                        Sector::Front => 1000,
                        Sector::Left => q.shield_side_pm,
                        Sector::Right => q.open_side_pm,
                        Sector::Rear => q.behind_pm,
                    },
                );
            }
        }
        if !attended {
            p = scale(p, q.unattended_pm);
        }
        if bodies.wound[target] == Wound::Hampered {
            p = scale(p, q.target_hampered_pm);
        }
        if bodies.wound[striker] == Wound::Hampered {
            p = scale(p, q.striker_hampered_pm);
        }
        if bodies.posture[striker] != Posture::Forward {
            p = scale(p, q.answer_pm);
        }
        p = scale(p, self.kits[target].guard_pm);
        let edge = (self.kits[striker].skill as i32 - self.kits[target].skill as i32).clamp(-8, 8);
        for _ in 0..edge.abs() {
            p = if edge > 0 {
                scale(p, q.skill_step_pm)
            } else {
                p * 1000 / q.skill_step_pm.max(1) as u128
            };
        }
        let p = p.min(ONE);
        let u = draw as u128;
        if u < p {
            // Where the draw fell inside the wounding share decides how bad the wound is.
            let x = (u * 1000 / p) as u32;
            return if x < q.kill_pm {
                BoutOutcome::Killed
            } else if x < q.kill_pm + q.disable_pm {
                BoutOutcome::Disabled
            } else {
                BoutOutcome::Hampered
            };
        }
        let driven = ((ONE - p) * q.drive_back.q32() as u128) >> 32;
        let can_give_way = matches!(bodies.posture[target], Posture::Steady | Posture::Forward);
        if can_give_way && u - p < driven {
            BoutOutcome::DrivenBack
        } else {
            BoutOutcome::Nothing
        }
    }
}

impl Resolver for Bouts {
    fn resolve<F: FrameQuery>(
        &mut self,
        bodies: &Bodies,
        _frames: &F,
        tick: u32,
        rng: &Rng,
        out: &mut Vec<Event>,
    ) {
        if !bodies.posture.contains(&Posture::Forward) {
            return;
        }
        self.index(bodies);
        self.engaged.clear();
        for a in 0..bodies.len() {
            if bodies.posture[a] == Posture::Forward
                && bodies.wound[a] < Wound::Disabled
                && let Some((dist_sq, defender)) = self.opponent(bodies, a)
            {
                self.engaged.push(Engaged {
                    defender,
                    dist_sq,
                    attacker: ManId(a as u32),
                });
            }
        }
        // By defender, then nearest first: a defender attends to the first man in his list, and
        // each attacker's draw is addressed by his rank in it.
        self.engaged.sort_unstable();
        let occurs = Prob::from_rate(self.params.rate_q16, MAN_STEP);
        self.took.clear();
        let mut rank = 0;
        for i in 0..self.engaged.len() {
            let e = self.engaged[i];
            rank = if i > 0 && self.engaged[i - 1].defender == e.defender {
                rank + 1
            } else {
                0
            };
            let block =
                rng.block(Key::new(Stream::Combat, e.defender.0, tick, PURPOSE_BOUT).at(rank));
            if !occurs.hits(block[0]) {
                continue;
            }
            let (a, d) = (e.attacker.0 as usize, e.defender.0 as usize);
            let attended = rank == 0;
            let defender = self.strike(bodies, a, d, attended, block[1]);
            let attacker_took = if attended && self.can_answer(bodies, d, a, e.dist_sq) {
                self.strike(bodies, d, a, true, block[2])
            } else {
                BoutOutcome::Nothing
            };
            out.push(Event {
                tick,
                man: e.defender,
                pos: bodies.pos[d],
                kind: EventKind::Bout {
                    attacker: e.attacker,
                    defender,
                    attacker_took,
                },
                causes: [Cause::Bout, Cause::None, Cause::None],
            });
            self.took.push((e.defender, defender));
            self.took.push((e.attacker, attacker_took));
        }
        // Every bout was settled from the same snapshot. A man struck more than once takes the
        // worst of it.
        self.took.sort_unstable();
        for i in 0..self.took.len() {
            let (man, worst) = self.took[i];
            if self.took.get(i + 1).is_some_and(|next| next.0 == man) {
                continue;
            }
            let m = man.0 as usize;
            let event = |kind| Event {
                tick,
                man,
                pos: bodies.pos[m],
                kind,
                causes: [Cause::Bout, Cause::None, Cause::None],
            };
            let from = bodies.posture[m];
            let wound = match worst {
                BoutOutcome::Hampered => Wound::Hampered,
                BoutOutcome::Disabled => Wound::Disabled,
                BoutOutcome::Killed => Wound::Dead,
                BoutOutcome::Nothing | BoutOutcome::DrivenBack => Wound::Sound,
            };
            if wound > bodies.wound[m] {
                out.push(event(EventKind::Wound { to: wound }));
            }
            let to = if wound >= Wound::Disabled {
                Posture::Down
            } else if worst == BoutOutcome::DrivenBack && from == Posture::Forward {
                Posture::Steady
            } else if worst == BoutOutcome::DrivenBack && from == Posture::Steady {
                Posture::Back
            } else {
                from
            };
            if to != from {
                // A fall is logged as a posture change too: that is what other men see.
                out.push(event(EventKind::Posture { from, to }));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::observe::{Observed, Side};
    use crate::units::Vec2;
    use crate::view::NoFrames;

    fn put(b: &mut Bodies, x: i32, y: i32, side: Side, posture: Posture, facing: u16) -> ManId {
        b.push(Observed {
            pos: Vec2::new(x, y),
            side,
            posture,
            facing: Turn(facing),
            ..Default::default()
        })
    }

    const EAST: u16 = 0;
    const WEST: u16 = Turn::HALF;

    /// Runs the same snapshot for `steps` man steps and returns every event.
    fn run(bouts: &mut Bouts, b: &Bodies, steps: u32) -> Vec<Event> {
        let rng = Rng::new(7);
        let mut out = Vec::new();
        for s in 0..steps {
            bouts.resolve(b, &NoFrames, s * 5, &rng, &mut out);
        }
        out
    }

    fn bouts_of(events: &[Event]) -> Vec<(ManId, ManId, BoutOutcome, BoutOutcome)> {
        events
            .iter()
            .filter_map(|e| match e.kind {
                EventKind::Bout {
                    attacker,
                    defender,
                    attacker_took,
                } => Some((attacker, e.man, defender, attacker_took)),
                _ => None,
            })
            .collect()
    }

    fn plain(n: usize) -> Bouts {
        Bouts::new(BoutParams::PLACEHOLDER, vec![Kit::PLAIN; n])
    }

    fn harmed(o: BoutOutcome) -> bool {
        o >= BoutOutcome::Hampered
    }

    #[test]
    fn steady_lines_do_no_harm() {
        let mut b = Bodies::default();
        put(&mut b, 0, 0, Side::A, Posture::Steady, EAST);
        put(&mut b, 1_000, 0, Side::B, Posture::Steady, WEST);
        assert!(run(&mut plain(2), &b, 4_000).is_empty());
        // Nor does a man who is forward with no one in reach.
        b.posture[0] = Posture::Forward;
        b.pos[1] = Vec2::new(1_600, 0);
        assert!(run(&mut plain(2), &b, 4_000).is_empty());
    }

    #[test]
    fn bouts_come_at_the_rate_and_harm_at_the_named_chances() {
        let mut b = Bodies::default();
        let a = put(&mut b, 0, 0, Side::A, Posture::Forward, EAST);
        let d = put(&mut b, 1_000, 0, Side::B, Posture::Steady, WEST);
        let steps = 400_000;
        let ev = run(&mut plain(2), &b, steps);
        let bouts = bouts_of(&ev);
        assert!(bouts.iter().all(|x| x.0 == a && x.1 == d));
        // One bout in ten seconds, at four steps a second.
        let n = bouts.len() as u32;
        let expect = steps / 40;
        assert!(n.abs_diff(expect) < expect / 20, "{n} bouts");
        // The defender is hurt one time in twenty.
        let hurt = bouts.iter().filter(|x| harmed(x.2)).count() as u32;
        assert!(hurt.abs_diff(n / 20) < n / 200, "{hurt} of {n}");
        // Half of those wounds only hamper; 15 % kill.
        let killed = bouts.iter().filter(|x| x.2 == BoutOutcome::Killed).count() as u32;
        assert!(killed.abs_diff(hurt * 15 / 100) < hurt / 20);
        // Of the rest, three in ten drive him back.
        let driven = bouts
            .iter()
            .filter(|x| x.2 == BoutOutcome::DrivenBack)
            .count() as u32;
        assert!(driven.abs_diff((n - hurt) * 3 / 10) < n / 50);
        // His answer is weaker than the attack (x 0.7) but the attacker has stepped into it
        // (x 1.5): 0.05 x 1.05.
        let answered = bouts.iter().filter(|x| harmed(x.3)).count() as u32;
        assert!(
            answered.abs_diff(n * 525 / 10_000) < n / 200,
            "{answered} of {n}"
        );
    }

    #[test]
    fn a_turned_back_is_struck_often_and_does_not_answer() {
        let mut b = Bodies::default();
        put(&mut b, 0, 0, Side::A, Posture::Forward, EAST);
        put(&mut b, 1_000, 0, Side::B, Posture::Turned, EAST);
        let bouts = bouts_of(&run(&mut plain(2), &b, 200_000));
        let n = bouts.len() as u32;
        let hurt = bouts.iter().filter(|x| harmed(x.2)).count() as u32;
        // 0.05 x 8.
        assert!(hurt.abs_diff(n * 4 / 10) < n / 50, "{hurt} of {n}");
        assert!(bouts.iter().all(|x| x.3 == BoutOutcome::Nothing));
        assert!(bouts.iter().all(|x| x.2 != BoutOutcome::DrivenBack));
    }

    #[test]
    fn the_second_of_two_attackers_strikes_unanswered_at_a_busy_man() {
        let mut b = Bodies::default();
        let near = put(&mut b, 100, 0, Side::A, Posture::Forward, EAST);
        let far = put(&mut b, 0, 600, Side::A, Posture::Forward, EAST);
        put(&mut b, 1_000, 0, Side::B, Posture::Steady, WEST);
        let bouts = bouts_of(&run(&mut plain(3), &b, 400_000));
        let of = |who: ManId| -> Vec<_> { bouts.iter().filter(|x| x.0 == who).collect() };
        let (first, second) = (of(near), of(far));
        assert!(second.iter().all(|x| x.3 == BoutOutcome::Nothing));
        assert!(first.iter().any(|x| harmed(x.3)));
        let share = |v: &[&(ManId, ManId, BoutOutcome, BoutOutcome)]| {
            v.iter().filter(|x| harmed(x.2)).count() as u32 * 1000 / v.len() as u32
        };
        // The second man's strike is twice as likely to wound.
        assert!((45..56).contains(&share(&first)), "{}", share(&first));
        assert!((90..111).contains(&share(&second)), "{}", share(&second));
    }

    #[test]
    fn skill_guard_wounds_and_the_open_side_all_count() {
        let hurt_share = |edit: &dyn Fn(&mut Bodies, &mut Vec<Kit>)| {
            let mut b = Bodies::default();
            put(&mut b, 0, 0, Side::A, Posture::Forward, EAST);
            put(&mut b, 1_000, 0, Side::B, Posture::Steady, WEST);
            let mut kits = vec![Kit::PLAIN; 2];
            edit(&mut b, &mut kits);
            let bouts = bouts_of(&run(
                &mut Bouts::new(BoutParams::PLACEHOLDER, kits),
                &b,
                400_000,
            ));
            bouts.iter().filter(|x| harmed(x.2)).count() as u32 * 10_000 / bouts.len() as u32
        };
        let base = hurt_share(&|_, _| {});
        assert!((470..530).contains(&base), "{base}");
        // Two steps of skill: x 1.25 twice.
        let skilled = hurt_share(&|_, k| k[0].skill = 2);
        assert!((740..820).contains(&skilled), "{skilled}");
        let outclassed = hurt_share(&|_, k| k[0].skill = -2);
        assert!((290..350).contains(&outclassed), "{outclassed}");
        // A better shield halves it.
        let guarded = hurt_share(&|_, k| k[1].guard_pm = 500);
        assert!((225..275).contains(&guarded), "{guarded}");
        // A hampered target, x 1.5; a hampered striker, x 0.6.
        let lame = hurt_share(&|b, _| b.wound[1] = Wound::Hampered);
        assert!((700..800).contains(&lame), "{lame}");
        let weak = hurt_share(&|b, _| b.wound[0] = Wound::Hampered);
        assert!((270..330).contains(&weak), "{weak}");
        // The defender faces north, so the attack comes at his left, where the shield is; he
        // faces south, and it comes at his open side; he faces east, and it comes from behind.
        let shield = hurt_share(&|b, _| b.facing[1] = Turn(Turn::QUARTER));
        assert!((470..530).contains(&shield), "{shield}");
        let open = hurt_share(&|b, _| b.facing[1] = Turn(3 * Turn::QUARTER));
        assert!((950..1_050).contains(&open), "{open}");
        let behind = hurt_share(&|b, _| b.facing[1] = Turn(EAST));
        assert!((1_900..2_100).contains(&behind), "{behind}");
    }

    #[test]
    fn a_short_weapon_cannot_answer_a_long_one() {
        let mut b = Bodies::default();
        put(&mut b, 0, 0, Side::A, Posture::Forward, EAST);
        put(&mut b, 2_000, 0, Side::B, Posture::Steady, WEST);
        let mut kits = vec![Kit::PLAIN; 2];
        kits[0].reach_mm = 2_500;
        let bouts = bouts_of(&run(
            &mut Bouts::new(BoutParams::PLACEHOLDER, kits),
            &b,
            100_000,
        ));
        assert!(!bouts.is_empty());
        assert!(bouts.iter().all(|x| x.3 == BoutOutcome::Nothing));
    }

    #[test]
    fn wounds_and_falls_are_logged_as_their_own_events() {
        let mut b = Bodies::default();
        let a = put(&mut b, 0, 0, Side::A, Posture::Forward, EAST);
        let d = put(&mut b, 1_000, 0, Side::B, Posture::Steady, WEST);
        let ev = run(&mut plain(2), &b, 100_000);
        let after = |i: usize| -> Vec<EventKind> {
            ev[i + 1..]
                .iter()
                .take_while(|e| e.tick == ev[i].tick)
                .map(|e| e.kind)
                .collect()
        };
        let mut seen = [false; 4];
        for (i, e) in ev.iter().enumerate() {
            let EventKind::Bout {
                defender,
                attacker_took,
                ..
            } = e.kind
            else {
                continue;
            };
            assert_eq!(e.man, d);
            let rest = after(i);
            let mut want = Vec::new();
            // Events follow in order of man: the attacker is man 0.
            match attacker_took {
                BoutOutcome::Nothing => {}
                BoutOutcome::DrivenBack => want.push(EventKind::Posture {
                    from: Posture::Forward,
                    to: Posture::Steady,
                }),
                BoutOutcome::Hampered => want.push(EventKind::Wound {
                    to: Wound::Hampered,
                }),
                o => {
                    want.push(EventKind::Wound {
                        to: if o == BoutOutcome::Killed {
                            Wound::Dead
                        } else {
                            Wound::Disabled
                        },
                    });
                    want.push(EventKind::Posture {
                        from: Posture::Forward,
                        to: Posture::Down,
                    });
                }
            }
            match defender {
                BoutOutcome::Nothing => {}
                BoutOutcome::DrivenBack => {
                    seen[0] = true;
                    want.push(EventKind::Posture {
                        from: Posture::Steady,
                        to: Posture::Back,
                    })
                }
                BoutOutcome::Hampered => {
                    seen[1] = true;
                    want.push(EventKind::Wound {
                        to: Wound::Hampered,
                    })
                }
                o => {
                    seen[if o == BoutOutcome::Killed { 3 } else { 2 }] = true;
                    want.push(EventKind::Wound {
                        to: if o == BoutOutcome::Killed {
                            Wound::Dead
                        } else {
                            Wound::Disabled
                        },
                    });
                    want.push(EventKind::Posture {
                        from: Posture::Steady,
                        to: Posture::Down,
                    });
                }
            }
            assert_eq!(rest, want, "after {:?}", e.kind);
        }
        assert_eq!(seen, [true; 4]);
        assert!(ev.iter().all(|e| e.man == a || e.man == d));
    }

    #[test]
    fn a_bout_does_not_depend_on_who_else_is_fighting() {
        let mut b = Bodies::default();
        put(&mut b, 0, 0, Side::A, Posture::Forward, EAST);
        let d = put(&mut b, 1_000, 0, Side::B, Posture::Steady, WEST);
        let alone = run(&mut plain(2), &b, 20_000);
        // Another pair, forty metres away, and a bystander beside the first.
        put(&mut b, 40_000, 0, Side::A, Posture::Forward, EAST);
        put(&mut b, 41_000, 0, Side::B, Posture::Forward, WEST);
        put(&mut b, 0, 1_800, Side::A, Posture::Steady, EAST);
        let crowd = run(&mut plain(5), &b, 20_000);
        let mine = |ev: &[Event]| -> Vec<Event> {
            ev.iter()
                .filter(|e| e.man.0 < 2)
                .filter(|e| !matches!(e.kind, EventKind::Bout { attacker, .. } if attacker.0 > 1))
                .copied()
                .collect()
        };
        assert_eq!(mine(&alone), mine(&crowd));
        assert!(alone.iter().any(|e| e.man == d));
        assert!(crowd.len() > alone.len());
    }
}
