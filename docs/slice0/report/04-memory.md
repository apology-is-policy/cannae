# 4. The two memory variants

**Draft by Paullus, 2026-11-19.** From the scorer's outputs and
[rungs.txt](../scored/rungs.txt), counted as [the outline](../report-outline.md) fixed
beforehand ("One rung against another: the count").

**The statistic:** for each set, the probability over seed pairs that the Roman line fares
worse in the variant than in `base`. Rung 0, which has no memory, is the control.

## The statistic by rung

Median over the 200 sets (quartiles):

| Engine | Rung | `shock` | `volleys` |
|---|---|---|---|
| Field | 0 | 0.74 (0.59-0.88) | 0.51 (0.39-0.59) |
| Field | 1 | 0.76 (0.62-0.88) | 0.50 (0.40-0.59) |
| Field | 2 | 0.83 (0.72-0.92) | 0.55 (0.44-0.66) |
| Field | 3 | 0.76 (0.63-0.86) | 0.49 (0.40-0.59) |
| Pairwise | 0 | 0.56 (0.46-0.65) | 0.52 (0.41-0.59) |
| Pairwise | 1 | 0.56 (0.46-0.65) | 0.52 (0.41-0.59) |
| Pairwise | 2 | 0.54 (0.45-0.64) | 0.58 (0.47-0.71) |
| Pairwise | 3 | 0.55 (0.47-0.65) | 0.52 (0.41-0.60) |

## One rung against another, set by set

Each set's statistic at the higher rung against the lower, read by the two-sided sign line
over the sets not tied (it lets a fair coin past at about 2 % each way).

| Engine, variant | 1 vs 0 | 2 vs 0 | 3 vs 0 | 2 vs 1 | 3 vs 2 |
|---|---|---|---|---|---|
| Field, `shock` | 73 / 56, not past | 145 / 48, **more** | 100 / 77, not past | 141 / 51, **more** | 44 / 147, **less** |
| Field, `volleys` | 55 / 40, not past | 145 / 47, **more** | 78 / 78, not past | 145 / 46, **more** | 43 / 151, **less** |
| Pairwise, `shock` | 33 / 11, **more** | 91 / 88, not past | 77 / 79, not past | 81 / 92, not past | 101 / 84, not past |
| Pairwise, `volleys` | 46 / 8, **more** | 155 / 38, **more** | 87 / 65, not past | 155 / 37, **more** | 40 / 156, **less** |

Each cell gives the sets where the higher rung's statistic is larger / smaller; ties are
in [rungs.txt](../scored/rungs.txt). **More** and **less** are past the line.

## Against what was expected beforehand

- **"Rung 1 shows more than rung 0 only where men fled from the shock and rallied."** On the
  pairwise engine it shows more in both variants, though most sets tie (156 and 146 of 200,
  many of them the same run for run). On the field engine it is not past the line.
- **"Rung 2 shows more than rung 1 in about half the draws, those where *shaken* wears off
  more slowly than the four minutes between the shock and the front."** Split at that cut:
  - on the field engine rung 2 shows more in **both** halves, in both variants (`shock`: 84 /
    31 of the slow sets and 57 / 20 of the others; `volleys`: 99 / 18 and 46 / 28), so the
    cut does not separate them;
  - on the pairwise engine `volleys` shows the same (104 / 12 and 51 / 25), and `shock`
    shows nothing in either half.
- **"Rung 3 shows more than rung 2."** It shows **less**, past the line, in three of the four
  engine-variants, and nothing in the fourth. Against rung 0 it is not past the line
  anywhere. So the gauges of rung 3 give back about what rung 0 gives.

**The number of looks:** five pairs of rungs, two variants and two engines are twenty; with
the eight counts of the split and the relief's two (section 5) the outline counted thirty.
Eleven of the twenty main comparisons are past the line, where the outline expected about
0.8 by chance either way. But the pattern they make is not the ladder the design expected:
rung 2's *shaken* moves the statistic, and rung 3's gauges move it back.

## What `volleys` cannot test

Fear of the fire itself. Men see only the falls it causes, and in these runs the Roman front
lost a median of 3 men to it before contact (section 3).

## What it means

- **Memory changes how a line takes a blow:** rung 2 makes a Roman line fare worse than it
  does without memory, after a shock or a volley on the field engine and after a volley on
  the pairwise.
- **But no rung passes the eight patterns** (section 1), so none of this rescues the verdict.
  The ladder's question was which memory is the least that reproduces the patterns, and the
  answer is that none of the four does.
