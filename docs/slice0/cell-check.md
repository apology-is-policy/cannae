# The field's cell at half its size: the check

**Status:** conditions only. Written by Varro on 2026-11-11, before the check was built or
run. Paullus reads them before it is run. The results are entered at the end, under their own
date.

## What was promised

The field engine's cell is 1.024 m. It is fixed, and it is not one of the drawn parameters.
[priors-field.md](priors-field.md), "Fixed, and not drawn", says of it:

> Structure. A check at 0.512 m will be reported with the results: the base patch re-run at
> the centre, and how far each of the eight measures moves.

The [report's outline](report-outline.md), section 7, lists the check as owed by me.

## What I knew when I wrote this

- **One outcome of the scored code.** On the frozen code, fewer than 10 % of the sets pass
  all eight bands on `base` in each of the field engine's four cells
  ([preregistration.md](preregistration.md), "Outcomes seen during the batch", 2026-11-11
  06:47). I know no count.
- **Battles of the nominal set at 1.024 m.** The QC pass ran on the nominal set, and I traced
  its flags man by man. I have never tabled its eight statistics.
- **Nothing at 0.512 m.** I know of no run of the patch at that cell. The field's own tests
  use it twice, for two men who meet.
- **No record of the new batch.** It was in its first step.

## What is changed

**One line:** the centre's `cell_shift`, 10 to 9 (`crates/cannae-field/src/field.rs`). The
build is commit 9241361 on the branch `cell-check`: the re-frozen code (40b8dcd) with that
line. It is never merged, and no scored run uses it.

**Nothing is changed to suit the smaller cell.** Every density, speed and distance of the
centre stays as it is. If one of them only makes sense at the larger cell, this check is where
that shows.

What the line carries with it, by the code:

| What | At 1.024 m | At 0.512 m |
|---|---|---|
| How far a man's presence reaches. It is shared among the four cells around him, by how near he is to each | One cell: 1.024 m | 0.512 m |
| The most one man adds to the density at a point (a man does not read his own share) | 0.95 per m² | 3.8 per m² |
| How far ahead a walker reads the density | 1.024 m, or one step's travel if that is farther, or his target if it is nearer | 0.512 m, with the same two exceptions |
| What a walker reads of himself there. The look ahead reads the whole field, his own share included, and a cell ahead still holds up to about a quarter of him | Up to about 0.24 per m² | Up to about 0.95 per m². The jam is at 5.4 |
| The densities of the parameters | Men per m², converted to the cell by the code | The same numbers |
| Spreading | mm/s for each man per m² of difference across a metre, read across one cell | The same law, read across half the distance |
| Who may touch whom in the position solve | Distances in millimetres | The same. Two of the field's tests cover a half-metre cell |
| Tiles of 32 by 32 cells | 32.8 m | 16.4 m. Bookkeeping: it changes speed and no result |

**What does not change:** the perception, which has a cell of its own (2 m, in
`cannae-core`); the frames, the man, the gauges and the bouts; the scenario; the scorer and
the bands.

## The runs

- **The set:** the nominal set (`cell --nominal yes`), with every parameter at the centre of
  its prior or at its placeholder. It is not a draw and is never scored.
- **The variant:** `base`.
- **The cells of the experiment:** the field engine at rungs 0 to 3. A rung is a choice and
  has no centre, so all four are run.
- **The seeds:** 1 to 30, at each cell size. That is 240 runs, each stopped by the rule,
  15 minutes after a side breaks.
- **Why 30 seeds and not the 10 of a scored set:** "how far a measure moves" can only be read
  against how far it moves from seed to seed. Thirty runs a side give the comparison below a
  standard error of about 0.075.
- **Both builds are made the same way,** by `tools/cell-check.sh`, on the Mac. Nothing runs
  on the Pi while the batch does. The records do not depend on the machine or on its load, so
  I take no lock for it; I do not run it while someone holds the Mac for a timing.

## What is reported

For each rung, at each cell size, by `tools/cell-check.py`:

1. **The statistics of the eight patterns,** pooled over the thirty runs as the scorer pools
   a set's ten (`metrics::verdicts`), each with its band:

   | Pattern | Statistic | Pooled as |
   |---|---|---|
   | 1 | The formed phase's length; the median gap while formed | Median over runs |
   | 2 | Burstiness; the share of quiet bins | Median over runs |
   | 3 | The setup depth of the first to turn | Median over runs |
   | 4 | Spearman's rho of the spread; the time for 80 % of bins to turn | Median over runs |
   | 5 | The share of the loser's hits after the break | Median over runs |
   | 6 | The share of runs that break within 5 minutes of contact; the share unbroken two hours after it | Counts over runs |
   | 7 | Hits per front file per hour; lethality per bout | Median over runs and sides; median over runs |
   | 8 | Leaders' share of hits while formed, and after the break | Pooled counts |

2. **Each pattern's verdict** by its band, pattern 6 by its band alone. The two dose-response
   legs need other variants and are not part of this check.
3. **How the runs of the two cell sizes compare,** for each of the eleven statistics that a
   single run has a value of: the eight of patterns 1 to 5, the two of pattern 7, and the
   time to break, with "never" later than any time. The comparison is A, the chance that a
   run at 0.512 m gives a larger value than a run at 1.024 m, ties counted half: the
   pre-registration's probability of superiority. The median and the quartiles at each size
   are given with it. A statistic that fewer than ten runs have at either size is not
   compared, and the report says so.
4. **Beside the promise, as context:** which side broke first, in how many runs; the machine
   time of the runs at each size; and the outcome of the field's own tests on the half-cell
   build, by name where one fails.

**A check on the script.** `score` is run on the same records and prints the scorer's own
verdict for each pattern. All 64 must be the script's. If one is not, the script is wrong:
nothing is read from it until it agrees, and the correction is said.

## How it is read, written beforehand

- **A statistic has moved** on a rung if A is 0.35 or less, or 0.65 or more. With nothing
  different between the sizes, about one comparison in twenty crosses those lines. There are
  44 comparisons, so about two will, by chance.
- **A verdict has changed** if a pattern passes at one size and fails or cannot be scored at
  the other.
- **The cell matters at the centre** if any verdict changes on any rung, or if five or more
  of the 44 comparisons move. The report then says so in those words, says that the field
  engine's results hold for a cell of 1.024 m, and the cell goes on the list for a second
  round as a number to draw or to justify.
- **Otherwise** the report says that at the centre no verdict changed and that the measures
  moved no more than seeds move them, with the count.
- **Either way the tables are given whole.**

**What I expect.** Most of the eight should not move. A man in his place is moved by his tie
alone and reads no density, so the cell acts only on men out of their places: those who step
forward, give ground, flee or pursue. A walker among them reads more of himself ahead at the
smaller cell (the table above), so he may walk a little slower. If anything moves, I expect
it in what flight and pursuit decide: the loser's hits after the break (pattern 5) and the
spread of the collapse (pattern 4). The report says whether that held.

**What the check cannot say:**

- **Anything about the drawn sets away from the centre.** It is one set.
- **Anything about the other variants.**
- **Anything about the passages.** The field was calibrated on measured flows at 1.024 m
  ([passage-field.md](passage-field.md)). The bench is not run again here. The field's own
  tests are, and their outcome is reported.

## One run

- **It is run once.** Nothing is tuned and run again, and no seed is added afterwards.
- **Nothing in the scored code changes because of it,** whatever it shows. The cell stays at
  1.024 m, and no prior and no band moves.
- **If the half-cell build fails** (a run that panics, a test that does not pass), that is
  reported as it is, with the message. The runs that did finish are still tabled and marked
  incomplete. Any repair of the check's build is a new decision: Paullus and Michal hear of
  it before it is made, and it never touches the scored code.
- **The result is an outcome seen before the scoring.** It is not a scored run. It is entered
  in the pre-registration's table of outcomes seen during the batch all the same, with its
  date.

## Results

*Not run yet.*
