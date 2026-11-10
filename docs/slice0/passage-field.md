# Slice 0: The field engine on the passage bench (S0.8)

**Status:** Varro, 2026-11-10. The substrate alone, with no man model, every prior at its
centre. Run: `cargo run --release -p cannae-harness --bin passage -- --substrate field
[--contact line|crowd|all]`. The bench, its two passages and its steering are Paullus's
([passage.rs](../../crates/cannae-harness/src/passage.rs)); the pairwise engine's own
calibration is in [passage-calibration.md](passage-calibration.md). Walls in the field are
described in [rival/08](../rival/08-the-field-as-built.md).

These figures are counts of men and simulated seconds. They do not depend on the machine's
load.

## What it shows

1. **Walking through a gate, the field passes too few: 1.17 persons per metre per second at
   3 m, against about 1.9 measured** (1.6 to 2.3, Seyfried et al. 2009). It passes the capacity
   of its walking curve and no more. The curve is Weidmann's, fitted to people walking in the
   open, and its capacity is about 1.2.
2. **Pushing through a gate it passes 4.0 at 3 m,** against about 4.0 measured at doors of
   0.7 m (Garcimartín et al. 2016). Nothing was fitted to that figure.
3. **The three ways of touching give the same flow** to within 6 % at every width, walking or
   pushing. Contact inside the crowd changes how far the line beside an interval is pushed, and
   nothing else here.
4. **A maniple falling back through a 3 m interval takes 40 s** for nine men in ten, against
   24 s on the pairwise engine. With the enemy following, 2 to 6 of its 120 men are not
   through after five minutes.
5. **Seyfried's corridor cannot be run on the field at all.** It is 0.8 to 1.2 m wide and a
   cell is 1.024 m. A passage narrower than a cell has no cell centre inside it.

## The gate

A crowd of 600, 36 m wide and 24 m deep, 10 m out from a gate in a rampart. Js is persons per
second per metre of the gate's width, from the 60th man to the 540th. t90 is the time until
540 are through. Walking, all 600 passed in every run and no one was crushed. The pairwise
columns are at a disc of 300 mm, the radius its calibration settled on.

**These figures are from after 2026-11-10's change to the lean** in both engines: a man who
leans now pushes only on men in his way, and gains no speed over open ground. Before it the
field's pushing figures were about a tenth higher (4.34 at 3 m), and 6 to 10 men of the
pursued maniple were left behind instead of 2 to 6, because the enemy came on faster than a
walk. The walking rows did not move.

Walking at 1.3 m/s:

| Width | Js, `Line` | Js, `Crowd` | Js, `All` | t90, `Line` | Js, pairwise | t90, pairwise |
|---|---|---|---|---|---|---|
| 3 m | 1.17 | 1.14 | 1.13 | 163 s | 2.91 | 73 s |
| 4.5 m | 1.05 | 1.03 | 1.07 | 123 s | 2.94 | 53 s |
| 6 m | 0.87 | 0.86 | 0.89 | 111 s | 2.64 | 46 s |
| 9 m | 0.56 | 0.55 | 0.57 | 112 s | 1.86 | 43 s |
| 12 m | 0.53 | 0.52 | 0.54 | 91 s | 1.51 | 40 s |

Running at 4 m/s and shoving:

| Width | Js, `Line` | Js, `Crowd` | Js, `All` | t90, `Line` | Peak density | Js, pairwise | t90, pairwise |
|---|---|---|---|---|---|---|---|
| 3 m | 3.99 | 3.85 | 3.80 | 48 s | 7.3 | 6.27 | 32 s |
| 4.5 m | 3.70 | 3.58 | 3.67 | 36 s | 6.7 | 6.35 | 22 s |
| 6 m | 3.01 | 3.01 | 3.07 | 33 s | 6.0 | 6.05 | 19 s |
| 9 m | 1.97 | 1.90 | 1.95 | 33 s | 6.4 | 4.75 | 16 s |
| 12 m | 1.79 | 1.79 | 1.86 | 27 s | 5.7 | 3.78 | 15 s |

At 3 m one man of the 600 was crushed under `Line` and one under `All`; the other 599 passed.
No one was crushed at any other width, or on the pairwise engine.

Peak density is the bench's measure, from positions: at each second, the 90th percentile over
men within 5 m of the mouth of the bodies within a metre of each. Walking, it was 4.1 to 4.8
at every width.

**Flow per metre falls as the gate widens, on both engines.** The crowd is 36 m wide and every
man heads for the nearest point of the mouth. At a wide gate the men in front of it are soon
through, and after that the gate is fed by the wings coming along the rampart to its two
edges. So at 9 and 12 m these figures measure the supply, not the gate, and they cannot be set
against Liao's finding that flow per metre is constant up to 5 m.

## The interval

A maniple of 120, 12 files by 10 ranks, falls back through an interval in the line behind it.
"Pursued" means the enemy follows it up and leans on it. Holder is the farthest any man of the
line beside the interval was pushed from his slot.

| Width | Pursued | Through, `Line` / `Crowd` / `All` | t90, `Line` | Holder, `Line` / `Crowd` / `All` | t90, pairwise | Holder, pairwise |
|---|---|---|---|---|---|---|
| 3 m | no | 120 / 120 / 120 | 39.5 s | 87 / 89 / 63 mm | 24.1 s | 592 mm |
| 3 m | yes | 118 / 118 / 114 | 39.8 s | 85 / 89 / 61 mm | 33.1 s | 513 mm |
| 6 m | no | 120 / 120 / 120 | 36.3 s | 298 / 298 / 131 mm | 18.8 s | 376 mm |
| 6 m | yes | 120 / 120 / 118 | 37.5 s | 289 / 286 / 37 mm | 18.8 s | 373 mm |
| 9 m | no | 120 / 120 / 120 | 24.1 s | 0 / 0 / 0 mm | 17.4 s | 45 mm |
| 9 m | yes | 120 / 120 / 120 | 24.1 s | 19 / 21 / 18 mm | 17.4 s | 206 mm |
| 12 m | no | 120 / 120 / 120 | 20.8 s | 0 / 0 / 0 mm | 16.6 s | 0 mm |
| 12 m | yes | 120 / 120 / 120 | 20.8 s | 16 / 16 / 15 mm | 16.6 s | 207 mm |

All 120 passed in every pairwise run.

- **The men not through.** I have not looked at where they stand. What I expect, and have not
  checked: the passage is slow, the enemy reaches its stop line before the last men are in,
  and no one walks onto ground the enemy holds, so they are cut off. That fewer were left
  once the enemy came on at a walk and no faster fits this, and does not prove it.
- **The line beside the interval is not in its place in this bench.** The bench has no frames,
  so the holders are loose men who stand. In the field they are then pushed a little where the
  crowd is thick. In a battle they are in their places and the field does not move them at
  all, which is the difference the pre-registration names ("The press").
- **A 6 m interval passes hardly more than a 3 m one** (4.1 against 3.8 men a second). The
  maniple is 11 m wide, so at 6 m most of it still has to come in from the sides.

## The walking curve decides the flow

To see what sets the walking figure I ran the bench with two steeper curves of Weidmann's form,
`1 - exp(-g (1/d - 1/5.4))`. This was a trial and is not in the code.

| g | The curve's capacity at 1.3 m/s | Js at the 3 m gate | t90 | Interval 3 m, t90 |
|---|---|---|---|---|
| 1.913 (Weidmann) | 1.19 | 1.17 | 163 s | 39.5 s |
| 2.8 | 1.52 | 1.45 | 134 s | 32.0 s |
| 3.7 | 1.80 | 1.69 | 116 s | 28.1 s |

The flow through a narrow gate is the curve's capacity, less a few per cent. So the steepness
of the curve is a parameter of this engine, and I had fixed it
([priors-field.md](priors-field.md) §4, "Fixed") at a value fitted to walking in the open.
Measured flows through bottlenecks are about 1.6 times what that value allows.

**Agreed with Paullus on 2026-11-10, not yet built:** the steepness gets a prior, calibrated
on the bottleneck measurements as the pairwise engine's disc radius was: the values whose
flow on this bench spans the measured 1.6 to 2.3. It is a change to my priors before the
freeze, on outside data and on no patch outcome. One check comes first: whether the flow still
follows the curve's capacity when the crowd starts dense, as in Liao's experiment (3 per m²
in front of the gap).

## What this says about contact inside crowds

On this bench, nothing. With every man kept a body's width from every other (`All`), the flow
through a gate is within 6 % of the flow with no contact between men of one side (`Line`). The
field's flow is set by its walking curve and its densities, and bodies add nothing to it.

Two things the bench does not test:
- **Passages narrower than a cell.** There the field has nothing to go on, and bodies are all
  there is.
- **Clogging.** Garcimartín's doors clog and clear in bursts. The longest wait between two men
  here was 0.25 to 1.5 s in every run. A field has no arches.

So my view from [spike-field.md](spike-field.md), that contact belongs where the ground is
narrow, is not yet tested by anything. What the bench does show is that `Crowd`, which fails
in the pocket on speed and on density, buys nothing at a gate 3 m wide or more.
