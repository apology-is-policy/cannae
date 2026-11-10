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

1. **Walking through a 3 m gate the field passes 1.80 persons per metre per second, by
   calibration.** About 1.9 is measured (1.6 to 2.3, Seyfried et al. 2009). Before it was
   calibrated the field passed 1.17, which is 40 % low. See "How the figures got here".
2. **Running and shoving through the same gate it passes 5.65.** The one measured figure is
   about 4.0, at doors of 0.7 m from a standing start (Garcimartín et al. 2016); there are no
   data for a crowd running through a wide gate. Before the calibration the field passed 3.99
   here. The calibration was on walking flows, and it moved this figure away from the one
   measurement there is.
3. **The three ways of touching give the same flow** to within about 6 % at every width,
   walking or pushing. Contact inside the crowd changes how far the line beside an interval is
   pushed, and nothing else here.
4. **A maniple falling back through a 3 m interval takes 27 s** for nine men in ten, against
   24 s on the pairwise engine. With the enemy following, one or two of its 120 men are not
   through after five minutes.
5. **Flow per metre falls as a gap widens, and most of that is the steering.** Every man makes
   for the nearest point of the mouth, so the men from the wings all make for its two edges
   and its middle goes unused. With the men spread across the mouth, the field passes its
   capacity at every width from 2.4 to 5 m to within about 13 %. Liao et al. measured a flow
   per metre that is constant over those widths.
6. **How a gap sits on the cells moves its flow by about a tenth either way.**
7. **Seyfried's corridor cannot be run on the field at all.** It is 0.8 to 1.2 m wide and a
   cell is 1.024 m. A passage narrower than a cell has no cell centre inside it.

## The gate

A crowd of 600, 36 m wide and 24 m deep, 10 m out from a gate in a rampart. Js is persons per
second per metre of the gate's width, from the 60th man to the 540th. t90 is the time until
540 are through. All 600 passed in every run and no one was crushed. The pairwise columns are
at a disc of 300 mm, the radius its calibration settled on.

Walking at 1.3 m/s:

| Width | Js, `Line` | Js, `Crowd` | Js, `All` | t90, `Line` | Js, pairwise | t90, pairwise |
|---|---|---|---|---|---|---|
| 3 m | 1.80 | 1.74 | 1.74 | 110 s | 2.91 | 73 s |
| 4.5 m | 1.70 | 1.62 | 1.70 | 81 s | 2.94 | 53 s |
| 6 m | 1.43 | 1.39 | 1.44 | 72 s | 2.64 | 46 s |
| 9 m | 0.92 | 0.87 | 0.92 | 73 s | 1.86 | 43 s |
| 12 m | 0.87 | 0.86 | 0.89 | 60 s | 1.51 | 40 s |

Running at 4 m/s and shoving:

| Width | Js, `Line` | Js, `Crowd` | Js, `All` | t90, `Line` | Peak density | Js, pairwise | t90, pairwise |
|---|---|---|---|---|---|---|---|
| 3 m | 5.65 | 5.41 | 5.46 | 35 s | 7.0 | 6.27 | 32 s |
| 4.5 m | 4.73 | 4.58 | 4.73 | 28 s | 6.4 | 6.35 | 22 s |
| 6 m | 3.89 | 3.94 | 4.04 | 26 s | 6.0 | 6.05 | 19 s |
| 9 m | 2.60 | 2.45 | 2.53 | 26 s | 6.0 | 4.75 | 16 s |
| 12 m | 2.46 | 2.46 | 2.58 | 21 s | 5.7 | 3.78 | 15 s |

Peak density is the bench's measure, from positions: at each second, the 90th percentile over
men within 5 m of the mouth of the bodies within a metre of each. The column is for `Line`.
Walking, it was 4.5 to 5.1 at every width and under every rule.

**Flow per metre falls as the gate widens, on both engines.** The crowd is 36 m wide and every
man heads for the nearest point of the mouth. At a wide gate the men in front of it are soon
through, and after that the gate is fed by the wings coming along the rampart to its two
edges. So at 9 and 12 m these figures measure the supply, not the gate. The next section
separates the two.

## The calibration: a dense crowd before a gap

`cargo run --release -p cannae-field --example gate_flow`

The geometry is Liao et al.'s wide bottlenecks, as in the pairwise engine's calibration: 360
men at 3 per m² in a waiting area 18 m wide, before a bottleneck 1 m long. They measured a
flow per metre that is constant from 2.4 to 5 m. Capacity is what the walking curve allows
([priors-field.md](priors-field.md), F8). Every figure is Js, under `Line`.

With the bench's steering, to the nearest point of the mouth:

| Capacity | 2.4 m | 3.0 m | 3.6 m | 4.4 m | 5.0 m |
|---|---|---|---|---|---|
| 1.19 (Weidmann's own curve) | 1.47 | 1.41 | 1.26 | 1.03 | 0.98 |
| 1.60 | 1.97 | 1.83 | 1.65 | 1.35 | 1.29 |
| 1.95 (the centre) | 2.42 | 2.18 | 2.00 | 1.63 | 1.54 |
| 2.30 | 2.86 | 2.51 | 2.35 | 1.91 | 1.75 |

With each man making for the point of the mouth that is as far across it as he is across the
waiting area (`--spread`):

| Capacity | 2.4 m | 3.0 m | 3.6 m | 4.4 m | 5.0 m |
|---|---|---|---|---|---|
| 1.19 | 1.25 | 1.12 | 1.34 | 1.18 | 1.09 |
| 1.60 | 1.68 | 1.46 | 1.78 | 1.57 | 1.42 |
| 1.95 | 2.01 | 1.76 | 2.15 | 1.91 | 1.72 |
| 2.30 | 2.35 | 2.04 | 2.52 | 2.22 | 2.00 |

What the two tables show:

- **The flow follows the curve's capacity.** At each width, Js divided by the capacity is the
  same in every row to within a few per cent. This holds from a dense start as it did from the
  bench's loose one, which is the check Paullus asked for before the capacity became a prior.
- **Most of the fall with width is the steering.** With the bench's steering Js falls by a
  third from 2.4 to 5 m. With the men spread across the mouth it is 0.87 to 1.13 of the
  capacity at every width: highest at 3.6 m, lowest at 3.0 and 5.0 m.
- **What is left is mostly the grid.** A gap 3.0 m wide has two columns of cell centres inside
  it, and one 3.6 m wide has four. The first passes about a tenth less than the capacity and
  the second about a tenth more. A gap of 5.0 m also has four, and passes about a tenth less.
- **A short gap passes a little more than a long one.** A man at the mouth of a 1 m
  bottleneck reads the open ground beyond it and walks out at full speed. With the men spread
  across the mouth, a bottleneck 3 m long (`--length-mm 3000`) passes 1 to 8 % less.
- **The jam density moves the flow a little at a fixed capacity:** about 6 % lower at a jam
  density of 4.5 than at 5.4, and about 6 % higher at 6.5 (`--jam-q8`).

**The prior.** The capacity is drawn from U[1.6, 2.3], the range Seyfried et al. measured,
with its centre at 1.95. It was added after the bench showed the field 40 % slow at gates,
and it is calibrated on Seyfried's and Liao's measurements and on no pattern, as the pairwise
engine's disc radius was. The people measured carried no shields, so both calibrations may run
high for armed men, and neither engine guesses a correction.

**What the calibration costs.** One curve serves for walking in the open and for passing a
gap. At the centre a man in a crowd of two per m² now walks at about 0.7 of his free speed,
where Weidmann's fit to people walking in the open gives 0.46. The planning guidelines built
on such fits predict 1.2 to 1.6 through a bottleneck, below what is measured there
([crowd-physics.md](crowd-physics.md)). The field is calibrated on the gaps, because a gap is
where the patch's relief variant reads this curve.

## The interval

A maniple of 120, 12 files by 10 ranks, falls back through an interval in the line behind it.
"Pursued" means the enemy follows it up and leans on it. Holder is the farthest any man of the
line beside the interval was pushed from his slot.

| Width | Pursued | Through, `Line` / `Crowd` / `All` | t90, `Line` | Holder, `Line` / `Crowd` / `All` | t90, pairwise | Holder, pairwise |
|---|---|---|---|---|---|---|
| 3 m | no | 120 / 120 / 120 | 26.8 s | 101 / 235 / 139 mm | 24.1 s | 592 mm |
| 3 m | yes | 119 / 119 / 118 | 26.8 s | 90 / 182 / 130 mm | 33.1 s | 513 mm |
| 6 m | no | 120 / 120 / 120 | 25.1 s | 289 / 289 / 294 mm | 18.8 s | 376 mm |
| 6 m | yes | 120 / 120 / 120 | 25.3 s | 288 / 288 / 295 mm | 18.8 s | 373 mm |
| 9 m | no | 120 / 120 / 120 | 18.6 s | 0 / 0 / 0 mm | 17.4 s | 45 mm |
| 9 m | yes | 120 / 120 / 120 | 18.6 s | 0 / 13 / 2 mm | 17.4 s | 206 mm |
| 12 m | no | 120 / 120 / 120 | 17.1 s | 0 / 0 / 0 mm | 16.6 s | 0 mm |
| 12 m | yes | 120 / 120 / 120 | 17.1 s | 0 / 13 / 0 mm | 16.6 s | 207 mm |

All 120 passed in every pairwise run.

- **The men not through.** I have not looked at where they stand. What I expect, and have not
  checked: the enemy reaches its stop line before the last men are in, and no one walks onto
  ground the enemy holds, so they are cut off. There were 6 to 10 of them when the enemy came
  on faster than a walk and the passage was slow, and there are one or two now. That fits,
  and does not prove it.
- **The pursuit does not slow the field's maniple** (26.8 s either way). It slows the discs'
  from 24 to 33 s, where the enemy's shove jams the maniple against the line
  ([passage.md](passage.md)). I have not looked at why the field differs.
- **The line beside the interval is not in its place in this bench.** The bench has no frames,
  so the holders are loose men who stand. In the field they are then pushed a little where the
  crowd is thick. In a battle they are in their places and the field does not move them at
  all, which is the difference the pre-registration names ("The press").
- **A 6 m interval passes hardly more than a 3 m one** (7.0 against 6.8 men a second). The
  maniple is 11 m wide, so at 6 m most of it still has to come in from the sides.

## How the figures got here

The field's figures on this bench changed twice on 2026-11-10. At the 3 m gate and the 3 m
interval, under `Line`:

| | Walking, Js | Pushing, Js | Interval, t90 | Pursued, left behind |
|---|---|---|---|---|
| Weidmann's own curve; a lean gained ground over open ground | 1.17 | 4.34 | 39.5 s | 10 of 120 |
| The same curve; a lean pushes only on men in the way | 1.17 | 3.99 | 39.5 s | 2 |
| The capacity calibrated, at its centre | 1.80 | 5.65 | 26.8 s | 1 |

- **The lean.** A man who leaned with no one ahead went faster than his gait, in both engines.
  Paullus found it in his when the press rule went in. The pairwise columns above are from
  after his fix.
- **The capacity.** The steepness of the walking curve was fixed at Weidmann's value. It is
  now set by the capacity, which is drawn.

## What this says about contact inside crowds

On this bench, nothing. With every man kept a body's width from every other (`All`), the flow
through a gate is within about 6 % of the flow with no contact between men of one side
(`Line`). The field's flow is set by its walking curve and its densities, and bodies add
nothing to it.

Two things the bench does not test:
- **Passages narrower than a cell.** There the field has nothing to go on, and bodies are all
  there is.
- **Clogging.** Garcimartín's doors clog and clear in bursts. The longest wait between two men
  here was 0.25 to 1.0 s in every run. A field has no arches.

So my view from [spike-field.md](spike-field.md), that contact belongs where the ground is
narrow, is not yet tested by anything. What the bench does show is that `Crowd`, which fails
in the pocket on speed and on density, buys nothing at a gate 3 m wide or more.
