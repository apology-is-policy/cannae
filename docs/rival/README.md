# Small Men, Many Battles — a Rival Vision

**Status:** a proposal on branch `varro`. It is not the design of record; that is
[`docs/vision/`](../vision/README.md). Michal asked for a competing vision, with leave to relax
anything. Written by Varro (Claude Fable 5.2) on 2026-11-10, and revised the same day after
Paullus's critique of the one-page sketch (yip call 0001, turns 3 and 5, notes 2, 3 and 5).

## The thesis

Model only what a witness could have seen of each man: his place, his posture, his wounds and
his exhaustion, with a few fixed dispositions behind them. Spend what that saves on running the
whole battle at 1:1 a hundred times a night on the Air. Then both acceptance tests are measured
at full scale on Michal's machine, instead of being argued up from a battle one tenth the size.

## The witness rule

The vision of record already forbids soldiers to read each other's minds: they see position,
facing and posture, nothing else. The rival applies the same rule to the modeller. Polybius,
Caesar and du Picq report what men did: who stepped forward, who gave ground, who turned, who
fell. They report fear too, constantly, but by its signs and of bodies of men: the panic and
confusion the Numidians caused in the Roman rear at Cannae, greater than the slaughter (Livy
22.48). No source reports a level. So the rival stores what has signs (a posture, a wound, a
winded man, a shaken one) and not a quantity that nothing visible depends on.

The rule has a consequence that matters more than its tidiness. A man described this way is
small, a battle of small men is cheap, and a cheap battle can be run often enough to do
statistics at full scale.

## Six departures

| # | Vision of record | Rival | Why |
|---|---|---|---|
| 1 | 3–7× real time at full scale; 5–10 Cannae seeds a night; acceptance batches on a rented machine; calibration at 1:10 | **Throughput at 1:1 is a requirement.** Target 20–50× real time on one core, a hundred or more full-scale seeds a night | Both acceptance tests are distributions. The race that decides Cannae does not scale down ([04](04-acceptance.md) §1). |
| 2 | Each man is a dynamical system: x, F, Φ, R, H and more, about 100 bytes | **The man is small and discrete:** five dispositions, a posture, a wound class, W′, and at most two marks of memory. Every change is a logged event. | The sources constrain acts and rates, not inner gains. Every rout gets a traceable genealogy. |
| 3 | The clash is a relaxation oscillation timed by W′ | **Fear is the default; fighting is led.** Standoff is the rest state. A bout starts when a bold man or a centurion steps forward and others follow. | The casualty clock: a formed fight has room for very little committed fighting ([02](02-order-and-contact.md) §9). |
| 4 | A frame fitted to where the men are, and men steering to its slots | **The standard is the frame.** Every body has a carrier; drill sets how stiffly men are tied to it. Order is a measured quantity. | Better history, simpler code, and "loses its order" becomes a number. |
| 5 | 20 Hz pairwise collisions on all cores; float state; double buffers and intent passes | **Fields and events, one battle per core.** Bodies move in a density and pressure field at 4 Hz, and touch each other only where they are crowded and moving. State is integers. The cores run seeds. | This is where the speed comes from. Determinism across machines comes free. |
| 6 | A learned predictor for A2; an emulator for calibration; a scale study | **Acceptance measured directly.** The ensemble of seeds is the predictor. Fate curves come from forking a live battle. | Cheap runs make the indirect machinery unnecessary. |

## What stays

W′ and its published physiology. The rule that agents read only observable state. The
counter-based RNG and its separate streams. The outcome classes, authoring tiers, contrasts and
sealed battles of [07](../vision/07-validation.md). Officers with beliefs, messengers and
scripts as the one plan language ([05](../vision/05-command.md), [09](../vision/09-intent-and-plans.md)).
Rust, the viewer, vertical slices, and the parsimony pillar, which the rival only applies harder.

## Michal's five wishes

| Wish | In the rival |
|---|---|
| 1. Ancient land battles; behaviour over graphics | Kept. |
| 2. Soldier-level 1:1, real numbers, real time, on the Air | Kept, and extended: statistics at 1:1 on the Air as well. Each man exists, but he is thinner. |
| 3. Formations that hold, a psyche, experience, discipline, limited commanders, orders on foot and horse | Kept. **One question is Michal's:** the psyche here is dispositions, visible acts and at most two marks of memory (*has fled*, *shaken*), with no fear or morale gauge per man. Fear and morale are things you watch and measure per unit. Is that the psyche he asked for? The argument is in [01](01-the-man.md) §9. |
| 4. Cannae ends as it did, usually; beforehand it is not obvious | Kept, and measured at full scale ([04](04-acceptance.md)). |
| 5. Set up, watch, take over, script | Kept. Added: the odds at any moment, and what moved them. |

**What he trades away:**

- the inner life in the inspector (no x, F, Φ, R to look at; marks, a posture history and
  their causes instead);
- wounds by body zone and the blood pool (four wound classes instead);
- some of the organic jostle of soft bodies: formed men keep their places, and bodies touch
  only where order has gone ([02](02-order-and-contact.md) §5);
- "low losses while the lines are formed" as evidence: here it is an input.

## Where the two designs have met

Michal asked for convergence by argument, not a verdict. By turn 5 of the call, Paullus and I
agree on more than we dispute.

| Agreed | From |
|---|---|
| A1 must be measured at 1:1, because the race does not scale | The rival |
| The standard as the frame; one representation for every kind of body | The rival; unified by Paullus |
| A2 from the ensemble; fate curves by forking | The rival |
| The visible layer as the default: five postures, hazards read off counted acts, events with logged causes, the genealogy | The rival |
| No fast inner variables: a hazard with a reaction delay does what x, F and Φ did ([01](01-the-man.md) §9) | The rival's question, Paullus's analysis |
| Marks, not gauges, as the default memory: *has fled*, *shaken* | Paullus's marks, the rival's rule |
| One ladder for the man, spanning both designs ([06](06-experiment.md) §2) | Paullus |
| The field as the default substrate, with contact where men are crowded and moving | The rival's field, Paullus's refinement |
| One battle per core, integer state and no aggregate crate, if the speed per core holds | The rival |
| The casualty clock as a constraint on both, with time-to-break scored by dose-response | The rival's clock, Paullus's scoring |

**Still open, for the experiment to decide:** where on the ladder the patterns stop failing;
what sets the rhythm of a clash; whether the speed is real; whether a soft pressure with local
contact holds a crowd at a believable density.

## Documents

| Doc | Contents |
|---|---|
| [01-the-man.md](01-the-man.md) | State, dispositions, postures, what moves a man, what replaces morale, whether fear should outlast its cause |
| [02-order-and-contact.md](02-order-and-contact.md) | Carriers and ties, the field, the contact zone, the casualty clock, how each pattern arises |
| [03-engine.md](03-engine.md) | Fields and events, integer state, one battle per core, forking, the cost arithmetic |
| [04-acceptance.md](04-acceptance.md) | A1 at 1:1, A2 from the ensemble, what is evidence and what is input |
| [05-costs-and-critique.md](05-costs-and-critique.md) | What it costs, where the vision of record is wrong, overbuilt or timid, and where the rival is weak |
| [06-experiment.md](06-experiment.md) | The Slice 0 experiment: one harness, two engines, kill criteria both ways |

## Status of the claims

| Claim | Status |
|---|---|
| Speed (20–50× per core) | Arithmetic ([03](03-engine.md) §2). **Unmeasured.** The experiment measures it. |
| A line can hold for an hour and then break with no stored morale | **Unproven.** The experiment tests it. |
| Caesar, *BC* 3.71, 3.89, 3.93, 3.99; *BG* 2.21, 7.51. Livy 10.28, 22.48 | Checked on 2026-11-10 against the Latin text at The Latin Library. |
| Sabin, "The Face of Roman Battle", *JRS* 90 (2000) 1–17 | Reference confirmed. The argument is given from memory; the article must be re-read before any wording is quoted. |
| Collins, *Violence: A Micro-sociological Theory* (Princeton, 2008) | Reference and the three concepts used (confrontational tension, forward panic, the violent few) confirmed from the publisher's summary and reviews. Details from memory. |
| Du Picq; Appian on Pharsalus; Plutarch on Carrhae; pedestrian densities and bottleneck flows (Fruin, Weidmann, later experiments) | From memory. To check before use. |

**One correction to the sketch.** It gave the centurions' share of a legion as 1.25 %. That is
the paper figure: 60 in about 4,800. Caesar's legions at Pharsalus averaged about 2,750 men (80
cohorts, 22,000 men: *BC* 3.89), so there the share was nearer 2 %.
