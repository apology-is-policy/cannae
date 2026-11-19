# 3. Reported, not banded

**Draft by Paullus, 2026-11-19.** From the scorer's outputs ([score-field-r0.txt](../scored/score-field-r0.txt)
and the other seven) and [fails.txt](../scored/fails.txt). No set fails on anything in this
section.

## The audit

Sets 0, 20, 40 and so on were also run to the end on every seed, in every variant, and each
such run recorded what happened after the scored window closed.

- **Every audited run's stopped and whole summaries are equal:** 64 audit lines (eight cells,
  eight variants), 100 of 100 in each.
- **After the window, on `base`:** a bout in 13 to 21 of 100 whole runs, a rally in 8 to 16,
  and a second break in 0 to 2, by cell.
- **Over all eight variants of a cell** (800 whole runs): a bout after the window in 108 to
  161, a rally in 52 to 136, a second break in 5 to 12.

So stopping a run 15 minutes after its break hides a little fighting and some rallies, and
almost never a second break.

## Stragglers

The loser's men who turned before the collapse, over the runs of `base` in which a side
broke:

| Engine | Rung | Runs with a side broken | With one or more stragglers | The most in one run |
|---|---|---|---|---|
| Field | 0 | 1,956 | 230 (11.8 %) | 19 |
| Field | 1 | 1,954 | 210 (10.7 %) | 30 |
| Field | 2 | 1,990 | 130 (6.5 %) | 16 |
| Field | 3 | 1,968 | 168 (8.5 %) | 18 |
| Pairwise | 0 | 1,996 | 63 (3.2 %) | 8 |
| Pairwise | 1 | 1,996 | 61 (3.1 %) | 8 |
| Pairwise | 2 | 1,998 | 26 (1.3 %) | 3 |
| Pairwise | 3 | 1,999 | 51 (2.6 %) | 4 |

The median count is 0 in every cell. Nearly every run has a side broken, and in most of them
no man turned before the collapse: the line went all at once.

## Tempo

Each set's median over its seeds, then the quartiles over the 200 sets. The only figures
there are (5 to 25 % of the time, 10 to 25 % of the men) are no band.

| Engine | Front-rank man-time spent forward, median (quartiles) | Sets inside 5-25 % | Front-rank men who start any bout, median (quartiles) | Sets inside 10-25 % |
|---|---|---|---|---|
| Field, rungs 0-3 | 0.474 to 0.480 (0.41 to 0.49) | 11 to 20 | 0.387 to 0.425 (0.31 to 0.50) | 18 to 29 |
| Pairwise, rungs 0-3 | 0.485 to 0.486 (0.44 to 0.49) | 11 to 13 | 0.375 to 0.400 (0.29 to 0.48) | 28 to 31 |

The front rank spends about half its time forward, twice the top of the figures, and about
two men in five start a bout. The fight is pressed harder than the sources describe, and it
is short.

## The shape of the time to break

The median Weibull shape over sets: on the field engine 1.02 to 1.42 by rung, on the
pairwise 1.52 to 1.67, with quartiles from about 0.7 to 2.8. No source gives this, so it is
reported only.

## The dose-response legs on the break time alone

Beside each leg's statistic, the same on the Roman line's break time alone, so that a reader
sees whether skill at arms carries a leg (the loophole the pre-registration names: veterans
who hold no longer than recruits but beat the warband sooner would pass on skill alone).

| Engine | Recruits worse than veterans: how the line fared, median; sets at or above 0.6 | The same on break time alone | Flank worse than none: how the line fared, median; sets at or above 0.6 | The same on break time alone |
|---|---|---|---|---|
| Field, rung 0 | 0.89; 194 | 0.89; 194 | 0.39; 5 | 0.38; 4 |
| Field, rung 1 | 0.88; 194 | 0.88; 193 | 0.39; 4 | 0.39; 5 |
| Field, rung 2 | 0.83; 192 | 0.84; 192 | 0.37; 1 | 0.36; 1 |
| Field, rung 3 | 0.88; 193 | 0.88; 192 | 0.38; 3 | 0.37; 3 |
| Pairwise, rung 0 | 0.84; 191 | 0.85; 191 | 0.34; 1 | 0.33; 1 |
| Pairwise, rung 1 | 0.84; 191 | 0.86; 191 | 0.34; 1 | 0.33; 1 |
| Pairwise, rung 2 | 0.81; 191 | 0.81; 191 | 0.32; 0 | 0.32; 0 |
| Pairwise, rung 3 | 0.84; 189 | 0.85; 190 | 0.33; 1 | 0.32; 1 |

- **The loophole carried nothing:** the two measures agree in every cell.
- **The recruits leg passes in nearly every set.**
- **The flank leg fails in nearly every set, and the wrong way round:** its median is below
  0.5 everywhere, as if a Roman line attacked in the flank fared *better* than one that was
  not. That is not a finding about a flank attack. Section 6 shows why: in `flank` the scorer
  starts the clock about five seconds early, and the time to break is counted from it.
- **In no cell did the Roman line break in fewer than 2 seeds of `base` in any set.**

## Front losses before contact

The median per run, on each side's front, before the lines met: in `shock`, 23 to 25 Romans on
the field engine and 3 to 4 on the pairwise; in `volleys`, 3 Romans on both. The warband lost
none before contact in either.
