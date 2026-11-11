# The field's cell at half its size: the check

**Status:** run on 2026-11-11; the results are at the end. By the line fixed beforehand, the
half cell moved the measures and changed the verdicts no more than seeds alone do, and the
results say how little that carries. The
conditions were written by Varro on 2026-11-11, before the check was built or
run, and changed twice on the same day after Paullus read them (call 0001, turns 107 and
109): a control, a calibrated line, and a comparison on unrelated seeds. The changes are
listed under "How the conditions changed". The results are entered at the end, under their
own date.

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
- **The half cell:** seeds 1 to 30 at 0.512 m.
- **The scored cell:** seeds 1 to 60 at 1.024 m, in two groups of thirty.
- **The control** (Paullus): seeds 31 to 60 against seeds 1 to 30, both at 1.024 m. Nothing
  differs between them but the seeds.
- **The comparison the line is read on:** the half cell against seeds 31 to 60 at 1.024 m,
  so that the two groups share no seed. The half cell against its own seeds at 1.024 m is
  given beside it. The reason is under "How it is read".
- **In all 360 runs,** each stopped by the rule, 15 minutes after a side breaks. The nominal
  set is not audited (`cell.rs`), so no run is carried to the end. The report gives the
  number of records of each kind in each file.
- **Why 30 seeds a group and not the 10 of a scored set:** "how far a measure moves" can
  only be read against how far it moves from seed to seed.
- **Both builds are made the same way,** by `tools/cell-check.sh`, on the Mac. Nothing runs
  on the Pi while the batch does. The records do not depend on the machine or on its load, so
  I take no lock for it; I do not run it while someone holds the Mac for a timing.

## What is reported

By `tools/cell-check.py`, three times in the same form: for the half cell against unrelated
seeds at the scored cell, for the half cell against its own seeds there, and for the control.

1. **The statistics of the eight patterns,** for each rung and each group, pooled over the
   thirty runs as the scorer pools a set's ten (`metrics::verdicts`), each with its band:

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
3. **How the runs of the two groups compare,** for each of the eleven statistics that a
   single run has a value of: the eight of patterns 1 to 5, the two of pattern 7, and the
   time to break, with "never" later than any time. The comparison is A, the chance that a
   run of the second group gives a larger value than a run of the first, ties counted half:
   the pre-registration's probability of superiority. The median and the quartiles of each
   group are given with it. A statistic that fewer than ten runs have in either group is not
   compared, and the report says so.
4. **Each statistic over the four rungs:** the mean of A over the rungs, less 0.5. This is
   "how far the measure moves". The four rungs share their seeds and differ at the centre
   only by memory, so they are four looks at one move more than four moves.
5. **Seeds alone.** The sixty runs at 1.024 m are split into thirty and thirty at random,
   1,000 times, the four rungs of a seed kept together. Each split is compared as the half
   cell is. The report gives, for each statistic, in how many splits seeds alone move it as
   far as the half cell did; and the same for the largest of the eleven moves and for the
   count of changed verdicts.
6. **Beside the promise, as context:** which side broke first, in how many runs; the machine
   time of the runs at each size; and the outcome of the field's own tests on the half-cell
   build, by name where one fails.

**A check on the script.** `score` is run on each group's records and prints the scorer's
own verdict for each pattern. All 96 must be the script's. If one is not, the script is
wrong: nothing is read from it until it agrees, and the correction is said.

## How it is read, written beforehand

- **A mark for the reader:** a comparison is marked "moved" if A is 0.35 or less, or 0.65 or
  more: two standard errors at thirty runs a group. It decides nothing by itself.
- **A verdict has changed** if a pattern passes in one group and fails or cannot be scored
  in the other.
- **The cell matters at the centre** if either of these holds:
  - fewer than 25 of the 1,000 splits reach the half cell's largest move (the largest, over
    the eleven statistics, of the mean of A less 0.5 over the rungs, either way);
  - fewer than 25 of the 1,000 splits reach the half cell's count of changed verdicts.

  Each is one chance in forty by seeds alone, one in twenty together. The report then says
  so in those words, names the statistics that fewer than 25 splits reach, says that the
  field engine's results hold for a cell of 1.024 m, and the cell goes on the list for a
  second round as a number to draw or to justify.
- **Otherwise** the report says that at the centre the half cell moved the measures and
  changed the verdicts no more than seeds alone do, with the counts.
- **Either way the tables are given whole,** the control's beside the half cell's.
- **Why the line is read on unrelated seeds** (after Paullus, turn 109). A split compares two
  thirties that share no seed. Seeds 1 to 30 at the two sizes share their seeds, with the
  same men drawn. If a seed's battle at 0.512 m resembles its battle at 1.024 m at all, those
  two groups differ less than two unrelated thirties would, and the line would be crossed
  less often than one time in forty by seeds alone. That would lean toward finding nothing,
  which is the result that suits my engine. Against seeds 31 to 60 the reference is exact.
  The comparison on the same seeds is still given, with its own counts against the splits,
  and it is not the line.

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

- **The records are made once.** Nothing is tuned and run again, and no seed is added
  afterwards.
- **A fault in the reporting may be repaired** (Paullus): the Python script, or how `score`
  is called. `score` has never read records of the nominal set, and the script has read
  made-up records only. If either fails on the real ones, it is repaired, the repair is said
  here, and the reporting is run again on the same records. The lines above do not change.
- **Nothing in the scored code changes because of it,** whatever it shows. The cell stays at
  1.024 m, and no prior and no band moves.
- **If the half-cell build fails** (a run that panics, a test that does not pass), that is
  reported as it is, with the message. The runs that did finish are still tabled and marked
  incomplete. Any repair of the check's build is a new decision: Paullus and Michal hear of
  it before it is made, and it never touches the scored code.
- **The records stay out of the repository** (Paullus). The sha256 of each records file is
  entered here, and the folder is kept. Anyone can make them again from the two commits.
- **The result is an outcome seen before the scoring.** It is not a scored run. It is entered
  in the pre-registration's table of outcomes seen during the batch all the same, with its
  date.

## How the conditions changed

Before anything was built or run:

| When (UTC) | What |
|---|---|
| 2026-11-11, 19:03 | First written (b59f7a9): 240 runs, and two lines for "the cell matters": any verdict changed on any rung, or five or more of the 44 comparisons moved |
| 2026-11-11, 19:12 | **The control added, and both lines replaced.** Paullus (turn 107): the two lines had nothing beside them. A pooled median near its band's edge flips by seeds alone, and the four rungs share their seeds, so the 44 comparisons are not 44. He asked for seeds 31 to 60 at 1.024 m as a control. I added the 1,000 splits, which say how often seeds alone do as much, and made the lines out of them |
| | **Why the count of moved comparisons is no longer a line.** On made-up records (random numbers in the record's format), a difference built into one statistic, large enough to give A = 0.95 on all four rungs, moved five of the 44; and seeds alone moved five or more in 61 of 1,000 splits. The count could not tell the difference from seeds. The largest mean move could: no split reached it. Seeds alone also changed at least one verdict in 998 of those 1,000 splits, because the made-up medians lay near their bands' edges |
| | Also from Paullus: the reporting may be repaired and run again on the same records; the records stay out of the repository with their hashes here; the kinds of record are counted |
| 2026-11-11, 19:15 | **The line is read on unrelated seeds.** Paullus accepted the lines (turn 109) and remarked that the half cell shared its seeds with the group it was compared to, which a split does not, and that this could only make the check more cautious. He asked for no change. I made one: the line is now read on the half cell against seeds 31 to 60 at 1.024 m. It costs no run |

## Results

**Run on 2026-11-11, 19:19:32 to 19:25:09 UTC, on the Mac** (Apple M2), by
`tools/cell-check.sh` at `varro` 790b3a5, at low priority with two jobs. Entered by Varro on
the same day. The whole report is in
[readings/cell-check-2026-11-11/summary.txt](readings/cell-check-2026-11-11/summary.txt).

### By the line: the half cell moved no more than seeds alone do

| | The half cell, against seeds 31 to 60 | The same seeds | The control | Seeds alone: the median, and the most in 95 splits of 100 |
|---|---|---|---|---|
| The largest of the eleven moves | 0.122 | 0.072 | 0.100 | 0.116; 0.183 |
| Splits that reach it, of 1,000 | **435** | 923 | 678 | |
| Verdicts changed, of 32 | 1 | 1 | 2 | 3; 4 |
| Splits that reach it, of 1,000 | **1,000** | 1,000 | 888 | |
| Comparisons marked "moved", of 44 (no line) | 3 | 0 | 3 | 1; 8 |

The line asked for fewer than 25 splits on either count. So at the centre the half cell
moved the measures and changed the verdicts no more than seeds alone do.

Each statistic over the four rungs, as the mean of A less 0.5:

| Pattern | Statistic | The half cell | The same seeds | The control | Splits in which seeds alone move it as far as the half cell, of 1,000 |
|---|---|---|---|---|---|
| 1 | Formed phase | -0.093 | -0.005 | +0.096 | 194 |
| 1 | Median gap while formed | -0.048 | -0.034 | +0.032 | 522 |
| 2 | Burstiness | +0.106 | +0.011 | -0.098 | 120 |
| 2 | Share of quiet bins | -0.089 | +0.012 | +0.100 | 179 |
| 3 | Depth of the first to turn | -0.122 | -0.014 | +0.098 | 33 |
| 4 | Spread, rho | +0.025 | -0.028 | -0.055 | 722 |
| 4 | Spread, time for 80 % of bins | -0.068 | -0.021 | +0.043 | 298 |
| 5 | Loser's hits after the break | -0.093 | -0.072 | +0.022 | 85 |
| 6 | Time to break | -0.093 | -0.005 | +0.096 | 194 |
| 7 | Hits per front file per hour | +0.038 | -0.002 | -0.038 | 487 |
| 7 | Lethality per bout | +0.036 | +0.018 | -0.017 | 607 |

- **The one changed verdict** is pattern 4 on rung 2: pass at 1.024 m, fail at 0.512 m. The
  time for 80 % of the bins to turn fell from 14 s to 6 s, across the band's lower edge of
  10 s. Seeds alone change the same verdict in the control, on rungs 0 and 2.
- **The formed phase and the time to break are one statistic here.** In every run the formed
  phase ended at the break, so the two rows are the same numbers and the eleven are ten.
- **The half cell's column is nearly the control's with its sign turned** (Paullus, with the
  result in hand; it changes no line). The formed phase is -0.093 and +0.096, burstiness
  +0.106 and -0.098, the depth of the first to turn -0.122 and +0.098. Both columns compare
  with seeds 31 to 60 at 1.024 m, whose battles ran longer than those of seeds 1 to 30 at
  either size (42 s against 36 s on rung 0). On the same seeds the largest move is 0.072. So
  most of the first column is the difference between two groups of seeds, which is what the
  splits measure.
- **What I expected:** that most would not move, and that anything that did would be in
  patterns 4 and 5. Nothing moved beyond seeds. The largest was pattern 3, not 4 or 5.

### What the line cannot carry: at the centre the battle is over within a minute

The nominal set is not scored, and this is an outcome of my engine all the same. At both
sizes and at every rung:

- **It fails patterns 1, 2, 3, 5, 6 and 7 and passes 8.** Pattern 4 passes in nine of the
  twelve groups of thirty.
- **The Roman line broke first in 26 to 29 runs of 30.**
- **87 to 97 % of the runs broke within five minutes of contact,** where the band allows 10 %.

| Rung | Formed phase, median: 1.024 m, seeds 1 to 30 | 1.024 m, seeds 31 to 60 | 0.512 m, seeds 1 to 30 |
|---|---|---|---|
| 0 | 35.8 s | 42.3 s | 36.1 s |
| 1 | 35.8 s | 42.3 s | 34.8 s |
| 2 | 23.3 s | 33.3 s | 21.9 s |
| 3 | 29.6 s | 36.3 s | 28.8 s |

So the check compared half-minute battles. A man in his place reads no density, and a fight
this short gives a crowd little to do. "No more than seeds" is true of this set. It is weak
evidence for a battle that holds.

### The field's own tests: eight of 44 fail at the half cell

On the scored build every test of the field passes (44, 4 and 3). On the half-cell build 36
of the 44 pass and eight fail. `cargo` stops there, so the other seven were not run.

| Test | What it asks at the centre | At 0.512 m |
|---|---|---|
| `walking_slows_as_the_crowd_thickens` | Through a crowd of two per m², a walker covers 1.6 to 2.0 m in the test's two seconds: 0.7 of his free walk | 0.31 m |
| `men_who_only_walk_stop_at_the_jam` | No cell goes far past the jam of 5.4 per m²: a peak of 6.5 at most | 8.7 per m² |
| `men_who_press_pack_tighter_and_stop_at_the_packing_density` | A peak of 10.5 per m² at most, for a packing density of 9 | 11.5 per m² |
| `a_packed_crowd_eases_when_no_one_presses` | A crowd packed at nine per m² eases below 5 in 480 steps | From 10.8 to 8.9 |
| `men_in_their_places_are_crushed_like_anyone_else` | Some man in his place is crushed | None was |
| `men_who_go_for_each_other_are_held_a_guard_apart` | Two enemies who go for each other end within 5 mm of the guard, 600 mm | 794 mm apart |
| `two_thin_lines_that_both_come_on_slip_past_but_no_one_is_stood_upon` | No two enemies nearer than 582 mm | 581 mm |
| `a_crowd_pressed_on_a_wall_is_no_denser_at_the_wall` | Fewer than 100 men in the 10 m² at the wall | 100 |

- **The first is the one that counts.** The walking curve was calibrated on measured flows
  at 1.024 m. With the cell halved and nothing else changed, the same curve gives a walker in
  a crowd of two per m² about a sixth of the calibrated distance.
- **Why, as far as I have read it:** every threshold is in men per m², and at the half cell
  the density is peakier. One man adds up to 3.8 per m² at a point, against 0.95. I have not
  traced the eight one by one.
- **Two of the eight fail by a hair:** one millimetre, and one man.

So "structure" was too easy a word in [priors-field.md](priors-field.md). The cell is a
parameter of the field's crowd: change it and the crowd rules are no longer the calibrated
ones.

### What follows

- **Nothing in the scored code changes,** as the conditions say.
- **The report gives all three:** the line's "no"; that it was read on battles of half a
  minute; and the eight tests.
- **A recommendation made with the result in hand, and outside the line.** The line does not
  put the cell on the list for a second round. I recommend it goes there anyway, on the
  tests: either the density is spread over a fixed width in metres, whatever the cell, or
  the cell is named as a calibrated constant with the walking curve. Paullus and Michal
  decide.
- **The passage bench was not run at the half cell.** The walking test suggests its flows
  would differ.

### The record of the run

- **The records:** 240 at 1.024 m (four files of 60) and 120 at 0.512 m (four of 30), every
  one a stopped run of the nominal set on `base`. No run failed, and the script found no
  other record. The folder is `~/cannae-cell-check` on the Mac.
- **Their hashes** (sha256) are in
  [readings/cell-check-2026-11-11/sha256.txt](readings/cell-check-2026-11-11/sha256.txt).
- **Machine time:** for seeds 1 to 30, the runs at 0.512 m took 1.0 to 1.5 times the runs at
  1.024 m (46, 61, 50 and 50 s against 30, 44, 44 and 49 s, rungs 0 to 3), on a machine that
  was busy with other work. It is a rough figure.
- **A fault in the reporting, repaired.** `cell-check.py` printed the whole report and then
  stopped in its last step, the check against the scorer: a loop reused the name of another
  loop's counter. I renamed it and ran the reporting again on the same records. The 441
  lines before that step are the same in both reports, byte for byte. I had tried the check
  against the scorer on made-up records with the script's first version only, and not again
  after I rewrote it for the control.
- **The check on the script:** the scorer's own verdicts are the script's in 96 of 96.
- **The table of outcomes seen** has this run under my name.
