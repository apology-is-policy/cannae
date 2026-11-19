# 5. The relief

**Draft by Paullus, 2026-11-19.** From the relief blocks of the scorer's outputs
([score-field-r0.txt](../scored/score-field-r0.txt) and the other seven),
[reliefs.txt](../scored/reliefs.txt), the drill's two files in
[replays/](../scored/replays/) and the last lines of each engine in
[rungs.txt](../scored/rungs.txt). What is shown, and how it is read, was fixed in
[the outline](../report-outline.md) ("5. The relief") before any record of `relief` was read.
The relief is reported for each cell and never scored for a set.

## The drill hardly came

**A relief can come only where a line lasts.** The script allows none before twenty minutes
([relief.toml](../../../scenarios/patch/relief.toml), `not_before_s = 1200`), and only in a
lull of twenty seconds. A run is stopped fifteen minutes after a side breaks (section 3). So
a run is relieved only if no side broke in its first five minutes, and section 1 found that
nearly every seed breaks within five minutes.

| Engine | Rung | Every fourth set: runs with a relief ordered | With both ordered | Top-up: sets | Runs with a relief ordered | With both ordered |
|---|---|---|---|---|---|---|
| Field | 0 | 22 of 500 (4 %) | 20 | 25 | 98 of 250 (39 %) | 91 |
| Field | 1 | 17 of 500 (3 %) | 15 | 25 | 99 of 250 (40 %) | 93 |
| Field | 2 | 13 of 500 (3 %) | 11 | 11 | 35 of 110 (32 %) | 34 |
| Field | 3 | 13 of 500 (3 %) | 10 | 15 | 54 of 150 (36 %) | 51 |
| Pairwise | 0 | 6 of 500 (1 %) | 5 | 5 | 17 of 50 (34 %) | 17 |
| Pairwise | 1 | 6 of 500 (1 %) | 5 | 5 | 17 of 50 (34 %) | 17 |
| Pairwise | 2 | 7 of 500 (1 %) | 6 | 4 | 11 of 40 (28 %) | 11 |
| Pairwise | 3 | 7 of 500 (1 %) | 6 | 4 | 14 of 40 (35 %) | 14 |

**A run with no relief is its `backed` twin, man for man**, so on the sets of the stride
(0, 4, 8 and so on) nearly every run of `relief` is the same battle as its twin.

**The top-up was run.** It is `relief` and `backed` on up to 25 more sets a cell, those where
`base` was still being fought twenty minutes in, in at least 2 seeds of 10. Whether to run it
was decided on the machine time alone: the figure, about 4 hours of the Pi's time, is in the
pre-registration's table (2026-11-18 19:58), and Michal said yes on 2026-11-19, before any
relief statistic was read. It ran on the Pi that morning, 05:09 to 08:40 UTC. Its sets are
named on the line "the top-up, if it is run" of each score file. The field's cells at rungs
0 and 1 reached the cap of 25. On the top-up's sets about a third of the runs were relieved.

## The drill alone, and the whole second line

| Engine | Rung | The drill alone (`backed` against `relief`): mean over the 50 sets (95 % interval) | Sets above 0.5 | Rank correlation with the Weibull shape | The whole second line (`base` against `relief`): median (quartiles) |
|---|---|---|---|---|---|
| Field | 0 | 0.500 (0.499-0.501) | 5 | -0.20 | 0.57 (0.49-0.65) |
| Field | 1 | 0.499 (0.498-0.500) | 2 | 0.12 | 0.57 (0.50-0.65) |
| Field | 2 | 0.500 (0.499-0.500) | 2 | 0.17 | 0.58 (0.48-0.66) |
| Field | 3 | 0.500 (0.499-0.500) | 2 | 0.07 | 0.57 (0.49-0.67) |
| Pairwise | 0 | 0.500 (0.500-0.500) | 1 | 0.15 | 0.61 (0.53-0.73) |
| Pairwise | 1 | 0.500 (0.500-0.500) | 1 | 0.01 | 0.61 (0.54-0.73) |
| Pairwise | 2 | 0.500 (0.500-0.500) | 2 | -0.13 | 0.61 (0.54-0.71) |
| Pairwise | 3 | 0.500 (0.500-0.500) | 0 | 0.23 | 0.62 (0.54-0.72) |

Each is the probability, over pairs of seeds, that the Roman line fares worse in the first
variant than in the second.

- **The drill alone is 0.500 because the drill hardly came,** and says nothing else about
  it. Its rank correlation with the shape, expected positive, runs from -0.20 to 0.23, and it
  ranks a statistic that is 0.5 in most sets.
- **The whole second line helps,** at 0.57 to 0.58 on the field engine and 0.61 to 0.62 on
  the pairwise. The pre-registration said it would by arithmetic: with a second line the
  Roman side has twice the men, so the rout of one century no longer breaks it, whether the
  second line ever moves or not.

## Each relieved run beside its twin

A run of `relief` and the run of `backed` with the same set and seed are the same battle
until the relief is ordered. The counts are how the relieved line fared against its twin:

| Engine | Rung | Every fourth set: better, worse, the same | Top-up: better, worse, the same |
|---|---|---|---|
| Field | 0 | 8, 5, 9 | 28, 29, 41 |
| Field | 1 | 2, 5, 10 | 28, 29, 42 |
| Field | 2 | 2, 5, 6 | 13, 14, 8 |
| Field | 3 | 2, 3, 8 | 15, 20, 19 |
| Pairwise | 0 | 1, 2, 3 | 8, 7, 2 |
| Pairwise | 1 | 1, 1, 4 | 8, 7, 2 |
| Pairwise | 2 | 2, 1, 4 | 5, 4, 2 |
| Pairwise | 3 | 0, 1, 6 | 9, 4, 1 |

**Better and worse are close in every cell.** Where the twins are most numerous, the field's
top-up at rungs 0 and 1, it is 28 better against 29 worse at both. The most one-sided count
is the pairwise top-up at rung 3, 9 against 4 in 14 runs. These counts show no effect of the
drill on a line that has lasted twenty minutes.

## Against what was expected beforehand

**"Rung 3 shows the drill's effect more than rung 2."** It cannot be told. On the sets of the
stride, set by set:

- **Field:** larger at rung 3 in 3 sets, smaller in 3, the same in 44 of 50 (37 of them with
  no relief ordered in any run at either rung). Not past the two-sided line, which is 6 of 6.
- **Pairwise:** larger in 0, smaller in 2, the same in 48 of 50 (43 with no relief at either
  rung). With 2 sets not tied it cannot be told.

These are the relief's two looks of section 4's count. The top-up's sets differ from rung to
rung, and no comparison of its twins across rungs was fixed beforehand, so none is made here.

## What the discs' passage does to the relief

[passage.md](../passage.md) promised that the effect of the discs' passage on the relief
would be reported: they pass men through a gap 10 to 20 % too freely, which makes a relief
easier on the pairwise engine. The outline fixed how it is shown and how it is read.

### The drill alone, at the nominal set

The relief's own test fixture, with its relief allowed from half a minute, men whom nothing
moves and nobody fighting (rung 2, seed 3; a man walks 1,300 mm/s):

| Engine | The fresh century has the front after (a free walk) | The tired one halts after that (a free walk) | Men out of their places at the halt | Nine in ten of the tired century back after the halt | Nearest two men of the two centuries | Jammed |
|---|---|---|---|---|---|---|
| Field | 18.75 s (18.25 s) | 18.75 s (18.50 s) | 0 of 80 and 80 | 0.05 s | 900 mm | no |
| Pairwise | 18.75 s (18.45 s) | 18.75 s (18.70 s) | 0 of 80 and 80 | 0.05 s | 900 mm | no |

Both reliefs of each engine give the same line. **On both engines the drill takes within
half a second of a free walk over the same ground, and no man is pushed out of his place.**

### The reliefs of the scored runs, replayed

For each cell, the relieved runs of `relief` were replayed, lowest set first and then lowest
seed, until 100 reliefs were seen or the runs were used up; then the top-up's, apart, up to
100 reliefs of their own. A replay changes no record. It counts only if its battle is the
record's own; fifteen were not counted, and they are listed below.

**The times are the drill's own.** In every cell, in both parts, the median relief put the
fresh century in front 18.75 s after the order and halted the tired one 18.75 s after that,
and nine in ten of the tired century's men were back in their places 0.05 s after the halt.
The median number of men out of their places at the halt is 0. The longest:

| Engine | Rung | Part | Reliefs seen (runs) | Halted | Front taken, longest | Halted after that, longest | Nine in ten back, longest | Out of place at the halt, most |
|---|---|---|---|---|---|---|---|---|
| Field | 0 | first 50 sets | 38 (20) | 35 | 73.00 s | 73.00 s | 0.05 s | 0 |
| Field | 0 | top-up | 98 (50) | 92 | 1,194.75 s | 122.00 s | 400.80 s | 107 |
| Field | 1 | first 50 sets | 28 (15) | 25 | 174.00 s | 174.00 s | 0.05 s | 2 |
| Field | 1 | top-up | 100 (51) | 92 | 886.50 s | 122.00 s | 0.05 s | 1 |
| Field | 2 | first 50 sets | 20 (11) | 17 | 73.00 s | 73.00 s | 0.05 s | 0 |
| Field | 2 | top-up | 67 (34) | 65 | 322.25 s | 322.25 s | 0.05 s | 8 |
| Field | 3 | first 50 sets | 21 (12) | 18 | 25.00 s | 25.00 s | 0.05 s | 0 |
| Field | 3 | top-up | 99 (50) | 98 | 341.00 s | 18.75 s | 0.05 s | 2 |
| Pairwise | 0 | first 50 sets | 9 (5) | 6 | 77.50 s | 29.25 s | 0.05 s | 27 |
| Pairwise | 0 | top-up | 34 (17) | 34 | 18.75 s | 18.75 s | 0.05 s | 0 |
| Pairwise | 1 | first 50 sets | 9 (5) | 6 | 77.50 s | 29.25 s | 0.05 s | 27 |
| Pairwise | 1 | top-up | 34 (17) | 34 | 18.75 s | 18.75 s | 0.05 s | 0 |
| Pairwise | 2 | first 50 sets | 11 (6) | 8 | 137.25 s | 89.75 s | 1.05 s | 32 |
| Pairwise | 2 | top-up | 22 (11) | 22 | 18.75 s | 18.75 s | 0.05 s | 0 |
| Pairwise | 3 | first 50 sets | 11 (6) | 8 | 25.50 s | 25.50 s | 1.05 s | 8 |
| Pairwise | 3 | top-up | 28 (14) | 28 | 18.75 s | 18.75 s | 0.05 s | 0 |

- **Every relieved run of the first 50 sets was replayed,** but for those not counted: no
  cell came near 100 reliefs there. The field's top-up at rungs 0, 1 and 3 stopped at about
  100 reliefs, before its relieved runs were used up.
- **The reliefs that did not halt were unfinished:** the run ended before the tired century
  halted, or a century broke first.

**The three marks.** Each is taken between the order and the tired century's halt:
**pressed**, a bout began with a man of either century; **jammed**, some man of the two
centuries had eight or more others on their feet within a metre, one of them of the other
century; **unfinished**, by reason.

| Engine | Rung | Part | Pressed and jammed | Pressed, not jammed | Jammed, not pressed | Neither | Unfinished: a century broke, the run ended |
|---|---|---|---|---|---|---|---|
| Field | 0 | first 50 sets | 0 | 2 | 0 | 36 | 0, 3 |
| Field | 0 | top-up | 2 | 11 | 0 | 85 | 1, 5 |
| Field | 1 | first 50 sets | 0 | 2 | 0 | 26 | 0, 3 |
| Field | 1 | top-up | 1 | 5 | 0 | 94 | 1, 7 |
| Field | 2 | first 50 sets | 0 | 0 | 0 | 20 | 0, 3 |
| Field | 2 | top-up | 0 | 2 | 0 | 65 | 0, 2 |
| Field | 3 | first 50 sets | 0 | 0 | 0 | 21 | 0, 3 |
| Field | 3 | top-up | 0 | 8 | 0 | 91 | 0, 1 |
| Pairwise | 0 | first 50 sets | 1 | 0 | 0 | 8 | 0, 3 |
| Pairwise | 0 | top-up | 0 | 2 | 0 | 32 | 0, 0 |
| Pairwise | 1 | first 50 sets | 1 | 0 | 0 | 8 | 0, 3 |
| Pairwise | 1 | top-up | 0 | 2 | 0 | 32 | 0, 0 |
| Pairwise | 2 | first 50 sets | 0 | 1 | 0 | 10 | 0, 3 |
| Pairwise | 2 | top-up | 0 | 2 | 0 | 20 | 0, 0 |
| Pairwise | 3 | first 50 sets | 0 | 0 | 0 | 11 | 0, 3 |
| Pairwise | 3 | top-up | 0 | 2 | 0 | 26 | 0, 0 |

**The three unfinished reliefs of the first 50 sets are the same in every cell:** sets 16
and 56, seed 6, ordered at 1,200 s, and the run ended before the halt. Read from those
runs' records, not by a tool fixed beforehand: in both, on both engines, the warband broke
just after five minutes, so the run's window closed at about twenty minutes, the first moment
the script allows a relief. The Roman line had won, stood in a lull, and was relieved as the
run ended.

### The runs beside their twins, by mark

How the relieved line fared against its `backed` twin (better, worse, the same). No run
lacked its twin on disk.

| Engine | Rung | Part | With a jammed relief | With a pressed relief, none jammed | The rest |
|---|---|---|---|---|---|
| Field | 0 | first 50 sets | 0, 0, 0 | 1, 1, 0 | 6, 4, 8 |
| Field | 0 | top-up | 1, 1, 0 | 3, 3, 1 | 8, 16, 17 |
| Field | 1 | first 50 sets | 0, 0, 0 | 1, 1, 0 | 1, 3, 9 |
| Field | 1 | top-up | 0, 1, 0 | 0, 3, 1 | 12, 16, 18 |
| Field | 2 | first 50 sets | 0, 0, 0 | 0, 0, 0 | 2, 4, 5 |
| Field | 2 | top-up | 0, 0, 0 | 1, 0, 0 | 11, 14, 8 |
| Field | 3 | first 50 sets | 0, 0, 0 | 0, 0, 0 | 2, 3, 7 |
| Field | 3 | top-up | 0, 0, 0 | 3, 1, 2 | 12, 17, 15 |
| Pairwise | 0 | first 50 sets | 1, 0, 0 | 0, 0, 0 | 0, 2, 2 |
| Pairwise | 0 | top-up | 0, 0, 0 | 0, 1, 0 | 8, 6, 2 |
| Pairwise | 1 | first 50 sets | 1, 0, 0 | 0, 0, 0 | 0, 1, 3 |
| Pairwise | 1 | top-up | 0, 0, 0 | 0, 1, 0 | 8, 6, 2 |
| Pairwise | 2 | first 50 sets | 0, 0, 0 | 1, 0, 0 | 1, 1, 3 |
| Pairwise | 2 | top-up | 0, 0, 0 | 1, 0, 0 | 4, 4, 2 |
| Pairwise | 3 | first 50 sets | 0, 0, 0 | 0, 0, 0 | 0, 1, 5 |
| Pairwise | 3 | top-up | 0, 0, 0 | 1, 0, 0 | 8, 4, 1 |

### How it reads, by the rules fixed beforehand

- **The jammed count is how often the fault could act.** On the pairwise engine, where the
  discs pass men too freely, one relief was jammed at rung 0 and one at rung 1, both in the
  first 50 sets (1 of 9 reliefs seen there), and none at rungs 2 and 3 or in any top-up. Each
  of the two runs fared better than its twin. On the field engine, two reliefs were jammed in
  the top-up at rung 0 and one at rung 1, and none elsewhere.
- **No relief was jammed without an enemy in it.** Every jammed relief was also pressed, and
  most pressed reliefs were not jammed. Nothing jammed on the passage alone.
- **Most reliefs were neither pressed nor jammed:** the drill of the fixture inside a battle,
  with the fixture's own times.
- **In four cells no relief was jammed:** the field's rungs 2 and 3 and the pairwise rungs 2
  and 3. There the effect the promise named had nothing to act on, as far as the replayed
  reliefs show.

**What it cannot show.** How the relief would have fared with the passage right: the remedy
was not built, so that battle cannot be run. These counts give how often the fault could act,
not what it did. And nothing is read from the difference between the engines, which differ in
everything else too.

## The fifteen replays not counted

| Engine | Rung | Set, seed | Part | Why |
|---|---|---|---|---|
| Field | 0 | 8, 5 | first 50 sets | the battle stepped by the replay is not the scorer's |
| Field | 0 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |
| Field | 0 | 2, 6 | top-up | the battle stepped by the replay is not the scorer's |
| Field | 1 | 8, 5 | first 50 sets | the battle stepped by the replay is not the scorer's |
| Field | 1 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |
| Field | 1 | 93, 5 | top-up | the battle stepped by the replay is not the scorer's |
| Field | 2 | 8, 5 | first 50 sets | the battle stepped by the replay is not the scorer's |
| Field | 2 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |
| Field | 2 | 150, 3 | top-up | the battle stepped by the replay is not the scorer's |
| Field | 3 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |
| Field | 3 | 17, 7 | top-up | the battle stepped by the replay is not the scorer's |
| Pairwise | 0 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |
| Pairwise | 1 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |
| Pairwise | 2 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |
| Pairwise | 3 | 72, 6 | first 50 sets | the record has both reliefs ordered at 1,200 s; the replay saw none |

- **Set 72, seed 6, is one run in all eight cells,** of the same kind as sets 16 and 56
  above: its record ends at 1,200 s, and both reliefs were ordered in that last second. The
  replay saw no relief.
- **The other seven are the field engine's:** the battle the replay tool stepped had other
  events than the scorer's.
- **The records are sound.** In every one of the fifteen, the scorer's own run of that set
  and seed gave the record back, all but the wall time; the replay tool checks that first.
  The fault is in the replay tool, and it is on the list for later (section 11).

## What it means

- **The relief was not tested by the scored runs.** The lines broke before the drill was
  allowed, so it came in 1 to 4 % of the stride's runs, and the drill's statistic sits at
  0.5.
- **Where it came, mostly on the top-up's long-fought sets, it neither helped nor harmed** a
  line that had lasted twenty minutes, as far as the twins show.
- **The drill itself works on both engines:** in the fixture and in most replayed reliefs it
  takes the time of a free walk and pushes no man out of his place.
- **The discs' fault had almost nothing to act on:** on the pairwise engine one jammed relief
  at rung 0, one at rung 1, and none at rungs 2 and 3.
