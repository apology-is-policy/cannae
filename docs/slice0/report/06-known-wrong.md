# 6. What is known to be wrong in the scored code

**Draft by Paullus, 2026-11-19.** From Addendum 4 of the
[pre-registration](../preregistration.md), [the QC page](../qc/README.md),
[STATUS](../../STATUS.md), [leaders.txt](../scored/leaders.txt) and
[open-flank.txt](../scored/open-flank.txt). One thing more was found while this report was
being written, after the scoring; it comes first, and the text says how it was counted.
Nothing here is corrected, and nothing is scored again.

## Found after the scoring: in `flank`, the band starts the clock

**What `flank` was meant to be:** "a small third body arrives on one flank of the Roman line
after contact" (the pre-registration's table of variants). In
[flank.toml](../../../scenarios/patch/flank.toml) a band of 36 stands off the Roman left from
the start, and is ordered to charge at ten minutes.

**What the scorer counts.** Contact is the first bout, or the first moment the gap between
the two sides' fronts is 3 m or less, and the time to break is counted from contact
(`cannae-harness`, `metrics.rs`). A body is left out of a side's front if the scenario marks
it `front = false`, as `shock` marks its rear band, or if it stands wholly behind another body
of its side. The flank band is neither: it stands beside the Roman line, level with it, and
the scorer takes its men as part of the warband side's front. **So in `flank` it finds contact
about a second in, earlier than in `base` in every run,** before the warband has reached the
Romans. The band also counts among the warband side's men when the scorer tests whether that
side broke. **And it turns the scorer's axis** (found by design review 2): the axis runs
between the centroids of the two sides' front men, band included (`axis_of` in
`metrics.rs`), so in `flank` every gap and every lateral bin is measured on a turned axis.
The reviewer works the turn out at about 26 degrees; I have not checked that figure. Marking
the band `front = false` mends both.

**The counts.** These were read from the records by a short script written for this
section, not by a tool fixed beforehand:

| Engine | Rung | Runs with contact in both `base` and `flank` | Contact earlier in `flank` | Contact, median: `base`, `flank` | Runs where the Roman side broke at the same tick in both |
|---|---|---|---|---|---|
| Field | 0 | 1,997 | 1,997 | 5.75 s, 1.00 s | 1,590 of 2,000 |
| Field | 1 | 1,997 | 1,997 | 5.75 s, 1.00 s | 1,601 of 2,000 |
| Field | 2 | 1,997 | 1,997 | 5.75 s, 1.00 s | 1,655 of 2,000 |
| Field | 3 | 1,997 | 1,997 | 5.75 s, 1.00 s | 1,646 of 2,000 |
| Pairwise | 0 | 1,996 | 1,996 | 6.00 s, 1.25 s | 1,741 of 2,000 |
| Pairwise | 1 | 1,996 | 1,996 | 6.00 s, 1.25 s | 1,737 of 2,000 |
| Pairwise | 2 | 1,996 | 1,996 | 6.00 s, 1.25 s | 1,731 of 2,000 |
| Pairwise | 3 | 1,995 | 1,995 | 6.00 s, 1.25 s | 1,736 of 2,000 |

**What it does to the flank leg.** In most runs the Roman side broke at the very tick it broke
in `base`. But the `flank` run's clock started earlier, so its Roman line's time to break is
longer. The leg compares two clocks:

- **Its value says nothing about a flank attack.** Section 3 gives it as a median of 0.32 to
  0.39, and the same on the break time alone. Sections 1 and 3 read that as "the wrong way
  round"; this is why.
- **It cannot change the verdict.** Pattern 6 fails on its band in all but at most one set of
  each cell (section 1). With pattern 6 counted by its band alone, no set of the field's new
  batch passes all eight (section 2), and no set of the pairwise engine passes pattern 3 in
  any cell. No reading of the flank leg could make a set pass.
- **It goes on the list for a second round** (26 in [STATUS](../../STATUS.md)). Under this
  pre-registration it is reported and not corrected.

## Found by design review 2: two of the scorer's statistics

Neither is counted here. Both are the reviewer's readings of the code, checked against it by
me, with the reviewer's estimates marked as such.

- **Pattern 3 gives ties to the front rank.** The first tenth of a body's men to turn is taken
  after sorting by tick and then by man number (`metrics.rs`), and men are numbered rank by
  rank from the front. Men who turn at the same man step share a tick, so a tie goes to the
  lower number, nearer the front. The reviewer estimates the bias at 0.05 to 0.1, unmeasured; it cannot carry the
  measured 0.18 to 0.26 to the band's 0.6. The reviewer adds that the statistic asks who
  begins the cascade, where vision 03 has rear men slip away first, before it.
- **Pattern 8 passes by geometry.** By the reviewer's arithmetic, three leaders among 40
  front-rank men are 0.075 of them, above the formed share observed (0.054 to 0.055), and
  three among 322 men about 0.009, below the rout share (about 0.02). So any
  front-rank man would pass it. The leaders' count below compares a leader with a file
  leader, which this pattern does not.

## D1, the defect left in by decision

**On the field engine, two enemies on their feet can stand nearer than the guard's distance**
when both count as in their places. Varro found it in his own code in the QC pass. Addendum 4
gives the counts, from the 160 unscored field battles of the pass:

- **123 flags under 300 mm, in 27 battles:** `shock` 108 in 20 battles, `veterans` 12 in 5,
  `flank` 3 in 2, none in the other five variants;
- **the nearest pair stood 11 mm apart, and 119 of the 123 flags last one man step.**

**Michal decided on 2026-11-11 that it is recorded and left in,** on both our
recommendations: a fix would have changed the field engine's contact rule for every pair at
the line, and its owner would have written it knowing that engine's result on `base`. **The
scored runs do not count it,** so how often it acted in them is not known.

## The gaps of the QC pass

**38 on the pairwise list and 50 on the field's:** things that look wrong although the code
does what its text says. By the pass's own rule they stay in; the batch measures the model as
it is. [The page](../qc/README.md) shows each kind in a picture.

**First among them, A: a formation follows its standard-bearer when he himself gives
ground,** by 0.65 m at a time on the pairwise engine and by as much as 15.6 m in twelve
seconds on the field engine. It is the nearest to the defect fixed in Addendum 3. Michal
decided that it stays in the batch and goes first on the list.

## The list for a second round

**Items 1 to 25 are in [STATUS](../../STATUS.md), and none is decided;** 26 is the flank
clock above, and 27 a candidate registered before the freeze and left off the list until
design review 2 found it: a blow that drives a calm man back puts him in the same posture,
with the same ways out, as a man who gave ground from fear. The batch bears on four of
them:

- **1, a fallen leader weighs too much:** the first count below.
- **2, nothing limits how fast turning spreads:** where pattern 4 fails, it fails because the
  break spreads along the line in less than its band's 10 s (section 1).
- **3, bodies never exploit a gap:** the second count below.
- **10, the first to turn are front-rank men:** pattern 3 fails in every set but one of the
  1,600, and the first men to turn stand about a quarter of the way back or less (section 1).

## What Michal saw, counted

At a mid-range set, on one seed, a century broke three seconds after its centurion fell, and
the warband then stood beside an open Roman flank. STATUS said the batch would show how often
the drawn sets do this. Both counts were defined before any record of the new batch was read.

### A century and its centurion

**The sample:** `base`, sets 0, 10, .. 190, all ten seeds, in every cell, replayed with the
event log. A leader's fall is the first time the leader of a front body goes down before his
body broke; a file leader's fall, for comparison, is every fall of a man who is a file leader
at that tick. 52 of the 1,600 replayed runs were not counted, all because the battle the
replay tool stepped had other events than the scorer's (8, 8, 6 and 4 on the field's rungs 0
to 3; 7, 7, 6 and 6 on the pairwise).

For the Roman bodies: how many falls were followed by the body's break within 5 s, within
30 s, and before a successor could have the place by the set's own delay (2 to 15 s, drawn for
each set):

| Engine | Rung | Leaders' falls | Broke within 5 s | Within 30 s | Before a successor | File leaders' falls | Broke within 5 s | Within 30 s | Before a successor |
|---|---|---|---|---|---|---|---|---|---|
| Field | 0 | 40 | 15 (38 %) | 24 (60 %) | 11 (28 %) | 373 | 55 (15 %) | 186 (50 %) | 59 (16 %) |
| Field | 1 | 40 | 16 (40 %) | 26 (65 %) | 13 (32 %) | 363 | 56 (15 %) | 190 (52 %) | 59 (16 %) |
| Field | 2 | 35 | 24 (69 %) | 29 (83 %) | 21 (60 %) | 250 | 69 (28 %) | 190 (76 %) | 72 (29 %) |
| Field | 3 | 38 | 16 (42 %) | 25 (66 %) | 13 (34 %) | 335 | 64 (19 %) | 199 (59 %) | 69 (21 %) |
| Pairwise | 0 | 33 | 15 (45 %) | 24 (73 %) | 14 (42 %) | 422 | 206 (49 %) | 335 (79 %) | 193 (46 %) |
| Pairwise | 1 | 33 | 15 (45 %) | 24 (73 %) | 14 (42 %) | 423 | 207 (49 %) | 337 (80 %) | 195 (46 %) |
| Pairwise | 2 | 28 | 20 (71 %) | 25 (89 %) | 18 (64 %) | 329 | 205 (62 %) | 301 (91 %) | 193 (59 %) |
| Pairwise | 3 | 30 | 13 (43 %) | 24 (80 %) | 12 (40 %) | 403 | 193 (48 %) | 329 (82 %) | 184 (46 %) |

**The body was in a like state at both kinds of fall:** the median share of its standing men
in flight was 0.00 at a leader's fall in every cell, and 0.00 (field) or 0.01 (pairwise) at a
file leader's.

**The warband,** which has no succession: 3 to 11 leaders' falls a cell, followed by the
break within 5 s in none but two cells (4 of 8 on the field's rung 2, 2 of 4 on the pairwise
rung 2); and 0 to 6 % of its file leaders' falls.

**How it reads, by the rule fixed beforehand** (if a centurion's fall is followed by the break
within 5 s much more often than a file leader's, with the body in a like state, the rule does
what Michal saw):

- **On the field engine it does.** A century broke within 5 s of its centurion's fall more
  than twice as often as after a file leader's, in every rung: in 38 to 69 % of the
  centurions' falls, by rung, against 15 to 28 %.
- **On the pairwise engine the count does not single out the centurion.** A body broke within
  5 s of nearly half of all its file leaders' falls too (48 to 62 %), against 43 to 71 % of
  its centurions'. With a formed fight of about 19 s (section 1), many falls come near the
  break, whoever falls.
- **What it does not say** (Varro): the count pools twenty sets, and the fear a fallen leader
  adds is drawn for each set. It says how often the drawn sets do this, and not how strong
  the rule is in any one.

### The warband beside an open flank

**No rule lets a body turn on a flank:** a body follows its one scripted order, in every set.
What can be counted is how often the opening arises. For each cell, on `base`, the runs in
which one Roman century broke and, a minute later, the other century and the warband both
stood (counted from the scored records by `tools/s09-open-flank.py`):

| Engine | Rung 0 | Rung 1 | Rung 2 | Rung 3 |
|---|---|---|---|---|
| Field | 211 of 2,000 (10.6 %) | 191 (9.6 %) | 156 (7.8 %) | 173 (8.7 %) |
| Pairwise | 51 of 2,000 (2.5 %) | 48 (2.4 %) | 20 (1.0 %) | 25 (1.2 %) |

No run ended inside the minute, and no line had a problem. **The opening arises in 7.8 to
10.6 % of the runs on the field engine, and in 1.0 to 2.5 % on the pairwise.** In every one
of them the warband, as a body, went on with its order. The count is of the opening, not of
what the warband's men did at it: men near the gap still go after runners one by one (Varro).

## What it means

- **The flank leg was measured with the wrong clock.** It is no evidence either way about a
  flank attack, and it could not have changed the verdict.
- **The centurion's fall weighs heavily on the field engine:** a century breaks within
  seconds of it far more often than after a file leader's fall. On the pairwise engine
  the fight is too short for the count to tell.
- **The open flank Michal saw is common on the field engine** and rare on the pairwise, and no
  body in the model can use it.
- **D1 and the QC gaps stay as recorded.** None was counted in the scored runs.
