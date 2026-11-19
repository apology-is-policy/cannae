# 1. The verdict

**Draft by Paullus, 2026-11-19.** Every figure here is copied from the outputs in
[../scored/](../scored/), which the commands fixed beforehand in
[the outline](../report-outline.md) printed ("The order at the batch's end", step 6). Nothing
is computed in this text.

## Every cell fails

**In all eight cells, not one of the 200 parameter sets passes all eight patterns.** The
knife-edge rule needed at least 10 % of the sets, on the observed share.

| Engine | Rung | Sets passing all eight | Share | 95 % interval | The rule |
|---|---|---|---|---|---|
| Field | 0 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |
| Field | 1 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |
| Field | 2 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |
| Field | 3 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |
| Pairwise | 0 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |
| Pairwise | 1 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |
| Pairwise | 2 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |
| Pairwise | 3 | 0 of 200 | 0.0 % | 0.0 to 1.9 % | fail |

**No rung passed, on either engine.** The base gate of Addendum 1 had said the same of every
cell on `base` alone ("cannot pass"), before any other variant ran.

**The rungs cannot be ranked.** The rule ranks them first by the share that passes all
eight, then by the median number of patterns passed. Within each engine all four rungs tie
on both: 0 % and a median of 2 patterns on the field engine, 0 % and a median of 1 on the
pairwise engine.

**Nothing is retuned.** No prior is narrowed and nothing is run again under this
pre-registration. A second round would be a new one.

## What the runs used

- **The code:** the re-frozen commit 40b8dcd (Addendum 4). The batch ran from a checkout at
  e7636bf and the top-up from one at 724ec29. Each script checked when it started that
  `git diff 40b8dcd -- crates scenarios Cargo.toml Cargo.lock` was empty.
- **The draws:** freeze seed 9464880715404986219, 200 sets a cell, 10 seeds a set, the same
  draws at every rung.
- **The free parameters:** the table of record has 101 drawn lines. The field engine reads
  at most 97 of them (all but the pairwise engine's 4), and the pairwise engine at most 93
  (all but the field's 8). The 5 gauge lines are read at rung 3 only, and a few others only
  under one rung's marks or one contact rule. A count line by line for each cell is not
  made here.

## Pattern by pattern

Sets that pass each pattern on `base`, of 200:

| Engine | Rung | 1 | 2 | 3 | 4 | 5 | 6 (band) | 7 | 8 | 6 with dose-response | Patterns passed, median (quartiles) |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Field | 0 | 18 | 9 | 0 | 83 | 116 | 1 | 42 | 122 | 0 | 2 (1-3) |
| Field | 1 | 17 | 8 | 1 | 84 | 114 | 1 | 39 | 124 | 0 | 2 (1-3) |
| Field | 2 | 5 | 4 | 0 | 63 | 105 | 1 | 33 | 119 | 1 | 2 (1-2) |
| Field | 3 | 6 | 5 | 0 | 79 | 115 | 0 | 33 | 120 | 0 | 2 (1-2) |
| Pairwise | 0 | 2 | 2 | 0 | 43 | 87 | 1 | 19 | 136 | 1 | 1 (1-2) |
| Pairwise | 1 | 2 | 2 | 0 | 44 | 86 | 1 | 19 | 133 | 1 | 1 (1-2) |
| Pairwise | 2 | 1 | 1 | 0 | 28 | 87 | 0 | 18 | 132 | 0 | 1 (1-2) |
| Pairwise | 3 | 2 | 2 | 0 | 43 | 79 | 1 | 21 | 127 | 1 | 1 (1-2) |

The patterns: 1 standoff, 2 pulses and lulls, 3 rear-first collapse, 4 lateral spread, 5
casualty asymmetry, 6 holds then breaks, 7 casualty clock, 8 centurion gradient.

**Pattern 6, by its two halves.** By its band alone it passes in at most 1 set of 200 in
any cell. So the dose-response legs are not what fails it: the band already does. The legs
themselves are in the scorer's outputs; on the field's rung 0, recruits fared worse than
veterans in 194 sets of 200 by the rule's measure, and a flank attack worse than none in 5
(a figure that says nothing about a flank attack: the scorer started the clock early in
`flank`, section 6).

## How the patterns fail

[fails.txt](../scored/fails.txt) gives every failing set by the part of its band that
failed, and the quartiles of each pooled value. Across the eight cells the failures have one
shape: **the lines meet and break within seconds to minutes, from the front.**

| What the band asks | Field, median over sets (rungs 0-3) | Pairwise, median over sets (rungs 0-3) | Band |
|---|---|---|---|
| The formed phase before a break, in s (pattern 1) | 21.5 to 27.1 | 18.4 to 19.5 | at least 300 |
| Share of seeds that break within 300 s (pattern 6) | 1.00 in every cell | 1.00 in every cell | at most 0.10 |
| Depth in the files of the first men to turn, 0 front to 1 rear (pattern 3) | 0.25 to 0.26 | 0.18 | at least 0.60 |
| Burstiness of the killing (pattern 2) | -0.04 to -0.03 | -0.05 | at least 0.20 |

- **Pattern 1 fails on its first part in nearly every set** (field rung 0: 182 of 182 failing
  sets fail on the formed phase; 20 also on the gap).
- **Pattern 6 fails because nearly every seed breaks early** (field rung 0: 198 of the 199
  failing sets have too many seeds breaking early; 5 have too many that never break).
- **Pattern 3 fails in every set on its only part:** the first men to turn stand near the
  front, a quarter of the way back on the field engine and less on the pairwise.
- **Pattern 2 fails mostly on burstiness** (field rung 0: 178 of 180), and the killing has
  no pulses: its burstiness is below zero, more even than chance.
- **Patterns 4, 5, 7 and 8 pass in a share of sets.** Where 4 fails, it fails because the
  spread of the break along the line takes less than its band's 10 s (field rung 0: all 117
  failing sets), which is the same short fight again.

## The first batch, on the frozen code

The field engine's first batch, on the code as first frozen (before the two fixes of
Addendum 4), has `base` only. Counted with pattern 6 by its band (Varro's count in
[two.txt](../scored/two.txt)), it also passes all eight in **0 of 200** sets in each of its
four cells. The two fixes did not change the verdict. Section 2 sets the two batches side by
side.

## What this does not say

- **It does not say which mechanism is wrong.** Both engines share the man, his fear and his
  ties, and differ in the substrate. Both fail the same way. That points at what they
  share, and the report's later sections give the counts that bear on it; it does not prove
  it.
- **It does not say the priors were too narrow.** The 200 sets span the priors as frozen.
  Whether some region outside them would pass is a question for a second round, under a new
  pre-registration.
