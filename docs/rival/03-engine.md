# 03 — The Engine

The vision of record plans a classical agent engine: every body collides pairwise at 20 Hz on
all cores, and every man integrates his psyche several times a second. The rival's engine does
less per man and almost nothing for a man to whom nothing is happening. This document gives the
target, the arithmetic behind it, and what the design drops and keeps from
[06](../vision/06-architecture.md).

## 1. The target

- **Full-scale Cannae in under ten minutes on one core:** 20–50× real time.
- **A hundred or more full-scale seeds a night** on the fanless Air.
- **The live viewer on one core** at 1–4×, with the other cores free to fork (§6).

The vision of record expects 3–7× on all cores together, 5–10 seeds a night, and a rented
machine for acceptance. The rival aims to run full-scale Cannae about as often as that plan
runs Little Cannae at 1:10.

## 2. The arithmetic

About 150,000 bodies, moved at 4 Hz: 600,000 body-steps per simulated second.

| Work per simulated second | Count | Cycles each | Total |
|---|---|---|---|
| Field: deposit, gather, move | 600,000 body-steps | about 100 | 60 M |
| Cell aggregates | 70,000 occupied cells × 4 | about 50 | 14 M |
| Hazards, for men to whom something can happen | 40,000 | about 400 | 16 M |
| Bouts | 1,200 | about 2,000 | 2 M |
| Men with an enemy within three metres: exact positions | 6,000 × 4 | about 500 | 12 M |
| Carriers and officers | 2,000 × 2 | about 2,000 | 8 M |
| **Total** | | | **about 110 M** |

One performance core sustains about 3 G cycles a second, so a little under 30× real time. Paullus checked
it from the other side: 30× at 4 Hz allows about 190 cycles per body-step, everything included.

A four-hour battle then takes about nine minutes on one core. The Air sustains roughly four
performance-core equivalents (the vision of record's own 17 G cycles a second, less 15–25 %
without a fan). That is about 27 battles an hour, over 200 in a night. At 10× per core, the line
below which I call the field dead, it is 80 a night.

**Contact is not in the table.** Bodies that are crowded and moving also touch each other
([02](02-order-and-contact.md) §5). If a tenth of the men are in that state it adds about
40 M cycles; if a whole pocket churned at once it could more than double the total. With
contact the honest range is nearer 12–30× per core. The kill line stays at 10×.

**Every cycle count in the table is a guess.** The Slice 0 spike measures them
([06](06-experiment.md)).

**Why the pocket is cheap.** In the encirclement the vision of record expects nearly every man
to be active, for hours, and its speed falls toward 2×. Here the 60,000 men in the pocket's
interior are bodies in a pressure field. They cannot reach an enemy, so they have no bouts.
The current design is most expensive exactly where the men can do least.

## 3. Two clocks

- **The field step,** 4 Hz. Every body deposits onto the grid and moves ([02](02-order-and-contact.md)
  §5). Fast bodies take substeps. A body that is in its place and standing still can be
  deposited once as a block.
- **Events.** Posture changes, bouts, volleys, signals, orders. Each step, hazards are
  evaluated only for men to whom something can happen: an enemy within reach or view, alarm in
  a nearby cell, an order in force, or a man out of his place. A hazard is compared with a draw
  from the counter-based RNG.

**What a man sees is kept as counts.** Each cell holds, per side, how many men are steady,
forward, back, turned and down. Postures change only by events, so an event updates one cell's
counts and nothing else. A man's alarm and support are sums over his own and neighbouring
cells, plus his slot neighbours. A cell whose counts changed is marked hot, and only men near
hot cells are evaluated.

As in the vision of record, the step sets how fast panic can travel, so reaction time is a
model parameter, not an engine detail.

## 4. Integer state

Positions are fixed-point. Dispositions and counts are small integers. W′ is joules in an
integer. Hazards are fixed-point probabilities read from tables. The field's arithmetic is
fixed-point too.

No floating point touches authoritative state. Bit-exact replay on any machine then needs no
rules about fused multiply-add, no pure-Rust `libm`, and no fixed-order reductions. The
counter-based RNG and its separate streams are kept exactly as designed.

## 5. One battle, one core

A battle runs on one thread. Events are applied in a fixed order: step, then man. A bout
changes both men at once.

This removes a layer of the vision of record's engine: the double-buffered observables lane,
the intent and resolve passes for pairwise events, work split into fixed-size blocks, padded
accumulators, and the test that the hash matches on 1, 4 and 8 threads. None of it is needed
when the unit of parallel work is a whole battle. Seeds scale perfectly across cores.

The observables-only rule stays, enforced by types as before: a hazard may read counts and
slot neighbours, never another man's dispositions.

## 6. Forking

The whole changing state is about 5 MB. Copying it takes milliseconds. A **fork** is a copy
that continues with fresh draws on the chosen RNG streams.

- **Fate curves.** Fork a battle at a moment, run the copies to the end, and the share that end
  each way is the odds at that moment. Steps in the curve mark the events that decided the
  battle ([04](04-acceptance.md) §2).
- **Live,** the idle cores fork the battle you are watching. Expect a coarse running estimate:
  a new finished fork every minute or two. A smooth curve for a recorded battle is a job of
  hours, not seconds. I will not promise more until the spike has run.
- **Counterfactuals.** Fork, change one thing, and compare the two ensembles on paired seeds.

## 7. What officers see

Bodies are first-class objects here: a carrier and the men tied to it. The scene graph of
[09](../vision/09-intent-and-plans.md) §3 needs no clustering of men into bodies. A body's
state (formed, disordered, routing) is read from its share of men in place and its posture
counts. Men who have left their bodies are grouped by cell. Each officer's view by occlusion,
his belief map, orders, messengers and scripts are as in the vision of record.

Horn signals and shouted orders are events with a range and a terrain shadow. That is enough
for the recall unheard across the valley at Gergovia. Dust and rumour fields are not built
until a slice needs their pattern.

## 8. Viewer and inspector

The rendering plan is unchanged: one instanced draw of circles. The viewer interpolates
between field steps.

- **Inspector.** A man's five dispositions, his marks, his posture history, and the causes of
  his last change, by name: "turned at 14:32; shaken since the standard fell at 14:30; the two
  men on his right had turned, and no closer stood behind him".
- **Overlays.** Posture shares per body, order, the share shaken, pressure, breath, and the
  bold men still standing.
- **The genealogy** of a rout, drawn as a tree over the field ([01](01-the-man.md) §8).
- **The fate curve,** with its steps labelled by event.

## 9. Dropped and kept from 06

| Dropped | Why |
|---|---|
| Pairwise soft-body collision for every body at 20 Hz | The field, with contact only where men are crowded and moving ([02](02-order-and-contact.md) §5) |
| Threads inside a battle, and everything that makes them deterministic | §5 |
| Floating-point rules | §4 |
| Three perception grids with sound, dust and rumour fields | One grid of counts (§3) |
| Activity levels of detail | Idle men already cost almost nothing |
| The split between keyframes and a quantised stream | The full state is small enough to keyframe often |
| The aggregate crate | The full engine is fast enough to be the officers' test bed. One model, not two that must agree. |

**Kept:** Rust; a simulation crate with no renderer dependency; the data, command, metrics, CLI
and viewer crates; the snapshot interface; the event log; Parquet batch outputs; light builds
for an 8 GB machine.

## 10. Risks

1. **The bookkeeping may cost more than the hazards.** Hot cells and eligibility lists are the
   kind of code that eats a budget quietly.
2. **Cavalry fights** are many fast, free bodies with bouts: the engine's worst case per body.
3. **One thread is a ceiling.** If some mechanism turns out expensive, there is no parallel
   escape inside a battle short of splitting the field into tiles later.
4. **Fixed-point pressure** may be fiddly to get stable.
5. **Coarse steps hide things.** Anything that needs a twentieth of a second is invisible at 4
   Hz unless it is made an event.
