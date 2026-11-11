# Slice 0: The passage study (S0.8)

**Status:** Paullus, 2026-11-10. The bench is built, and both engines have run on it: the
pairwise substrate here, and Varro's field in [passage-field.md](passage-field.md), compared
below. Run:
`cargo run --release -p cannae-harness --bin passage -- --substrate pairwise --radius 300`
(add `--spread` for the second steering, below).
Code: `crates/cannae-harness/src/passage.rs`. The calibration against measured flows is in
[passage-calibration.md](passage-calibration.md).

## What it runs

Two passages, from [rival 06](../rival/06-experiment.md) §2. Every substrate gets the same
bodies and the same steering at every man step, so only the physics differs between runs.
There is no man model.

- **The interval.** A maniple of 120 (12 files by 10 ranks, a metre apart) stands 3 m in front
  of the line. It falls back at a walk, facing the enemy, through the interval between two
  maniples of the same shape. A man is through once he is past the line's rear rank.
  - *Pressed:* an enemy body, 16 files by 6 ranks, in contact with the maniple's front, follows
    it up and shoves. It stops 1.5 m short of the line.
- **The gate.** 600 men in a loose mass (about 0.7 per m²), 10 m out, make for a gate in a
  rampart. The gate's passage is 3 m deep.
  - *Pressed:* they run (4 m/s) and shove, each facing where he is going. Otherwise they walk.
- **Steering:** the nearest point of the mouth, 0.4 m in from its edges; then straight through;
  then straight on.
- **Widths:** intervals of 3-12 m, at one shape of maniple. Frontage and intervals are
  parameters (Michal's ruling). Gates of 3-12 m: no source gives a gate's width.
  Pseudo-Hyginus 49 puts the *titulum* sixty feet out, "per latitudinem portarum", but gives no
  figure for the gate itself (Latin checked).

## The pairwise substrate's results

Disc radius 300 mm, every other parameter at its placeholder (the crush parameters now at
their priors' centres, below).

**The interval** (all 120 pass in every run):

| Width | Pressed | Js | Nine in ten through | Longest wait | Densest at the mouth | Line pushed back |
|---|---|---|---|---|---|---|
| 3 m | no | 2.62 | 24.1 s | 0.5 s | 2.9 per m² | 0.59 m |
| 3 m | yes | 1.49 | 33.1 s | **6.9 s** | 3.2 per m² | 0.51 m |
| 6 m | no | 2.11 | 18.8 s | 0.5 s | 2.9 per m² | 0.38 m |
| 6 m | yes | 2.10 | 18.8 s | 0.5 s | 2.9 per m² | 0.37 m |
| 9 m | no | 1.72 | 17.4 s | 0.6 s | 1.9 per m² | 0.05 m |
| 9 m | yes | 1.72 | 17.4 s | 0.6 s | 1.9 per m² | 0.21 m |
| 12 m | no | 1.48 | 16.6 s | 0.8 s | 1.6 per m² | 0 |
| 12 m | yes | 1.48 | 16.6 s | 0.8 s | 1.6 per m² | 0.21 m |

**The gate** (all 600 pass in every run):

| Width | Walking: Js | Walking: nine in ten through | Running and shoving: Js | Running: nine in ten through |
|---|---|---|---|---|
| 3 m | 2.91 | 72.7 s | 6.27 | 31.9 s |
| 4.5 m | 2.94 | 52.7 s | 6.35 | 22.4 s |
| 6 m | 2.64 | 46.0 s | 6.05 | 18.6 s |
| 9 m | 1.86 | 43.2 s | 4.75 | 16.1 s |
| 12 m | 1.51 | 40.1 s | 3.78 | 15.2 s |

Js is persons per metre of width per second, counted from the tenth to the ninetieth man in a
hundred. It measures capacity only where a jam forms at the mouth; a wide gap passes what
arrives. No one was crushed in any run.

## What it shows

1. **A maniple passes an interval in under a minute,** 17-33 s at every width from 3 to 12 m.
   At a walk, a relief fits inside a lull of a few minutes. The maniple's backward walk at
   1.3 m/s is fast for men facing an enemy, so these times are a lower bound.
2. **Faster is slower, at the narrowest interval only.** At 3 m the enemy's shove jams the
   maniple against the line. The longest wait between two men rises from 0.5 to 6.9 s, and the
   time for nine in ten to pass from 24 to 33 s. This is the clogging Pastor et al. measured at
   doors, and here it comes out of the physics with no rule for it. At 6 m and wider the shove
   changes nothing.
3. **The line beside the interval is shoved** up to 0.6 m by men squeezing through a narrow
   gap. At any width, men pushed against it by the enemy's shove move it 0.2-0.5 m. Each
   man braces back to his slot afterwards.
4. **In a jam, the flow runs about 40-50 % high on this steering.** Walking through a 3-6 m
   gate, the discs pass 2.6-2.9 persons per metre per second, against about 1.9 measured. Js
   also falls with width here, where Liao et al. found it constant. Most of both turned out
   to be the steering, not the discs (below): steered across the whole mouth, the discs pass
   1.8-2.0.
5. **Running and shoving more than doubles the flow** at the gate, to 6.0-6.4 at 3-6 m. There
   are no data for a crowd running through a wide gate. Garcimartín et al.'s pushing crowd,
   starting from standing at narrow doors, reached about 4.0. Discs running at 4 m/s in step
   are the same limit at speed: a real crowd at 4 per m² cannot run.
6. **No one is crushed at the priors' centres.** The running crowd peaks at 4.8 per m² at the
   mouth, where it is still flowing, below the onset of 6.5. Pressed against a wall instead
   (`--example press_wall`, 90 s), the crowd reaches 5.4-8.0 per m² across the corners of the
   priors. The summed overlap there matches what hexagonal packing predicts at the measured
   density to within about 7 % (549-798 mm against 588-805). So deriving `crush_mm` from a
   density holds in this substrate. Crushing begins only in the lower part of the onset prior.

## Fixed on the way

- **The crush rate's unit could not hold its prior.** The rate was in Q16 per mm per second.
  The prior's centre is 1.5 units in Q16 and its slow end 0.3, so most draws would have rounded
  to 0 or 1. It is now `crush_rate_q32_per_mm`. The placeholders are now the priors' centres:
  `crush_mm` 831 (onset 6.5 per m² at r = 280) and a rate of 100,830 (a mean of 134 s to
  disablement at 8.5 per m²). The old placeholder, 250 mm and 0.01 per s per mm, was about 400
  times the prior's centre rate. At a 3 m gate it disabled half the running crowd within five
  minutes.
- **Running men faced straight at the rampart,** so the shove of every man beside the gate went
  into the wall. Each now faces where he is going.
- **A man leaning with no one ahead of him walked faster than his gait,** because his lean was
  added to his step. Now he leans only on someone he touches ahead (found when Varro's press
  rule gave every man closing on an enemy a lean). The tables above are after the fix. Only
  the pushing rows moved, by up to a tenth.

## Steering across the whole mouth

Varro found that in his engine most of the fall of flow with width came from the steering.
Every man from the wings makes for one of the two edge points of the mouth, so the wings pile
into its edges. With `--spread`, each man makes for the point of the mouth as far across it as
he stands across the crowd, so the whole mouth is used. The gate, Js, on both steerings:

| Width | Discs, nearest point | Discs, whole mouth | Field, nearest point | Field, whole mouth |
|---|---|---|---|---|
| **Walking** | | | | |
| 3 m | 2.91 | 2.01 | 1.81 | 1.60 |
| 4.5 m | 2.94 | 1.89 | 1.70 | 1.61 |
| 6 m | 2.64 | 1.79 | 1.43 | 1.27 |
| 9 m | 1.86 | 1.57 | 0.91 | 0.93 |
| 12 m | 1.51 | 1.32 | 0.87 | 0.79 |
| **Running and shoving** | | | | |
| 3 m | 6.27 | 5.60 | 5.70 | 5.11 |
| 4.5 m | 6.35 | 5.38 | 4.73 | 4.17 |
| 6 m | 6.05 | 5.13 | 3.89 | 3.33 |
| 9 m | 4.75 | 4.44 | 2.60 | 2.48 |
| 12 m | 3.78 | 3.60 | 2.42 | 2.10 |

- **Steered across the whole mouth, both engines walk through a 3-4.5 m gate inside the
  measured range** (about 1.9, 1.6-2.3): the discs at 1.9-2.0, the field at 1.6. On Liao et
  al.'s own geometry the discs then pass 2.06-2.31, nearly constant across width
  ([passage-calibration.md](passage-calibration.md)).
- **The steering moves the discs by up to a third and the field by about a tenth.** A crowd of
  discs keeps its walking speed until it touches, so how men converge on the mouth decides how
  densely they arrive at it.
- **At 9-12 m this loose crowd cannot keep the gate full,** so Js there measures what arrives,
  not what the gate can pass.
- **Neither steering was measured.** People in a waiting area do spread across a bottleneck,
  so the second is the nearer. In the scored runs, steering comes from the man model.

## A candidate remedy, not built

What discs lack is the relation between speed and density, the *fundamental diagram* of
pedestrian research. Through these gaps it costs little once men use the whole mouth: 10-20 %
high on Liao's geometry. One way to add it is a rule that a man walks no faster than the gap
ahead of him allows, over a headway time. Single-file experiments find the space a walker needs grows
with his speed. That rule costs one parameter and a look-ahead beyond touching: up to about
2 m at a walk, more at a run. That means a neighbour search several times larger, against a
pocket already at 2.3x where the kill line is 3x.

It is not built now. Passage capacity enters the scored runs only through the relief variant,
where an optimistic capacity makes relief easier, and that effect will be reported (how:
[report-outline.md](report-outline.md), "How the discs' passage is shown in the relief",
written on 2026-11-11 before any relief statistic was read). The field
has a speed-density relation by construction. If it matches the measured flows on this bench
and the discs do not, that is a point to the field, and this experiment exists to find exactly
that.

## The field on the same bench

Varro's engine ran the same passages with every prior at its centre
([passage-field.md](passage-field.md)). Against the measurements and against the discs:

| | Measured | Field | Discs (300 mm) |
|---|---|---|---|
| Walking through a 3 m gate, Js | about 1.9 (1.6-2.3) | 1.81 (1.17 before calibration) | 2.91 |
| Running and shoving through a 3 m gate, Js | about 4.0, at 0.7 m doors | 5.70 | 6.27 |
| A maniple through a 3 m interval: nine in ten through | no data | 27 s | 24 s |
| The same, with the enemy following | no data | 1-2 of 120 still not through after 5 min | 33 s |
| Seyfried's 0.8-1.2 m corridor | 1.6-2.3 | cannot run: narrower than a cell | 1.7-2.4 |

The field's figures are at its calibrated centre (below), and after both engines' lean fix: a
lean now pushes only on men in the way.

What it shows, as it stands:
- **Before calibration, the two engines missed the walking flow in opposite directions.** The
  field passed the capacity of its walking curve, which was fitted to people walking in the
  open. The discs keep full speed until they touch.
- **The field's three ways of touching give the same flow,** within 6 % at every width.
  Contact inside a crowd buys nothing at a gate of 3 m or more. It already fails on speed in
  the pocket, so it has no case left where it earns its place.
- **Narrow gaps belong to the discs, and military widths may belong to the field.** The field
  cannot resolve a gap narrower than a cell, about a metre. Intervals and camp gates are wider
  than that. If it matches there, it will be by calibration: before it, the field was 40 % low,
  and the report gives both numbers.
- **The field is now calibrated, and the discs are not.** Varro made the walking curve's
  capacity a drawn parameter, U[1.6, 2.3] persons per metre per second, on the bottleneck
  measurements and on no pattern, as the discs' radius was calibrated (F8 in
  [priors-field.md](priors-field.md)). He checked it first on Liao et al.'s geometry: the flow
  scales with the capacity at every width from 2.4 to 5 m. At its centre the field walks 1.81
  through a 3 m gate. On the bench's first steering the discs stay 30-50 % high; steered
  across the whole mouth they are in the measured range too (above).
- **Running and shoving, both engines run high** where data exist only for narrow doors: the
  field 5.70, the discs 6.27, against about 4.0 at 0.7 m.
- **Much of the discs' excess was the bench's steering,** as Varro suspected from his engine.
  Checked: see "Steering across the whole mouth" above.

## Waiting

- **The man model's version (Varro):** the frames steering a body through an interval, and a
  turned man's flight aiming at a mouth before the ground beyond it.
