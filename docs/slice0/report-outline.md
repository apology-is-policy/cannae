# Slice 0: what the report owes

**Status:** draft by Paullus, 2026-11-11, written before the new batch has made a record.
Varro read it on the same day, also before any record, and added four things from the
documents outside the pre-registration. Each is marked "added by Varro". One of them left a
definition to me: how the discs' passage is shown in the relief. It is in section 5, proposed
on 2026-11-11 at 18:48 UTC, while the new batch ran its first step, and agreed with Varro's
changes by 19:06 UTC. Section 6 has the counts for what Michal saw at the mid-range set,
agreed in the same turns.

**Four parts are proposed and not yet agreed** (Paullus, 2026-11-11, 21:23 UTC, with the new
batch still in its first step): how a pattern's failures are counted (section 1), the
field engine's two results side by side (section 2), how stragglers and tempo are given
(section 3), and how one rung is counted against another (section 4). Each fixes how a
promised figure is counted, before a record is read. Each says so at its head until Varro
has read it.

This is a list, with no result in it. Everything below was promised before the runs: in the
[pre-registration](preregistration.md) and its addenda, in [STATUS](../STATUS.md), or to
Michal. It is collected here so that the report leaves nothing out, and so that nothing gets
in because of how the results look. Where the list adds something nobody promised, it says so.

**What was known when it was written:** the outcomes in the pre-registration's table of
outcomes seen during the batch, both speed readings, and the QC pass. Nothing else about a
scored run.

## Before anything is scored

Each of these is done, and the report says so with its counts:

1. **The batch has finished:** every step has run and none has failed.
2. **The recheck on the Mac** (`tools/s09-recheck.py`): two sets of every cell and variant,
   run again and compared with the Pi's records, all fields but the wall time.
3. **The crosswise check of the two perceptions,** again on the re-frozen code, on the same
   sample of 160 runs each way (Addenda 2 and 4). If any run differs, we stop and tell Michal
   before anything is scored.
4. **Michal has decided the top-up of the relief** (section 5), on the machine time alone,
   before any relief statistic is read.

Only then are the gate's lines and the records read.

## 1. The verdict

For each of the eight cells (two engines, rungs 0 to 3), on the re-frozen code:

| The report gives | Promised in |
|---|---|
| The commit the runs used, and that its code is the re-frozen one | Sign-off |
| The number of free parameters | The procedure |
| The share of the 200 sets that pass all eight patterns, with its 95 % Wilson interval | The knife-edge rule |
| Pass or fail: at least 10 % of the sets, on the observed share as it comes. Nobody argues from the interval | The knife-edge rule |
| Which patterns fail most often, and how | The procedure |
| Pattern 6 with and without its two dose-response legs, so that a reader sees which half failed | Scoring a parameter set |

### How a pattern fails: the count

**Proposed by Paullus on 2026-11-11 at 21:23 UTC,** while the new batch ran its first step
(`base` on the field's rung 0). Varro has not read it yet. Neither of us has read a record
of the new batch.

The scorer prints, for each pattern, how many sets pass, how many fail and how many cannot
be scored. The table above also promises how they fail. A pattern is a band on one or two
values pooled over a set's seeds, so a set fails it on one part or another. For each cell,
on `base`, the report gives the failing sets by the part that failed:

| Pattern | The parts a set can fail on |
|---|---|
| 1 Standoff | the formed phase under its minimum; the gap under its minimum |
| 2 Pulses and lulls | burstiness under its minimum; the share of quiet bins under its band, or over it |
| 3 Rear-first collapse | the depth of the first to turn under its minimum |
| 4 Lateral spread | the correlation under its minimum; the time of the spread under its band, or over it |
| 5 Casualty asymmetry | the loser's share of hits after the break under its minimum |
| 6 Holds, then breaks | too many seeds break early; too many never break. The two dose-response legs are in the scorer's own table |
| 7 Casualty clock | hits per file-hour under the band, or over it; harm per bout under its band, or over it |
| 8 Centurion gradient | the leaders' share of hits in the formed fight not above their share in the rout |

- **A set that fails on two parts is counted under each.**
- **A set that cannot be scored on a pattern** is counted apart, as the scorer counts it.
- **Beside the counts:** the quartiles over sets of each pooled value, with its band, so
  that a reader sees how far off the sets lie.
- **The tool must agree with the scorer.** It computes each verdict again from the parts and
  compares it with the scorer's, set by set. If one differs, it prints nothing.

Then, for each engine:

- **The lowest rung that passes is kept.**
- **If no rung passes,** the report says plainly "no rung passed". The rungs are still
  ranked: first by the share of sets that pass all eight, then by the median number of
  patterns passed. Nobody narrows a prior and reruns. A second round is a new
  pre-registration.
- **Rung 4** is built only if rungs 0 to 3 all fail (Michal, 2026-11-10), and is then
  pre-registered by itself. The text does not say whether "all fail" means on one engine or
  on both. If it comes to that, it is Michal's to say.

## 2. The field engine has two results

From Addendum 4, and from the conditions of Addendum 3 that still stand:

- **The first batch's `base` on the frozen code:** the 8,400 records of the field engine's
  four cells, kept as they are and scored with the batch. It is the pre-registered verdict of
  the frozen code for those cells.
- **Each of those runs beside its pair in the new batch.**
- **How often the standard-bearer defect could act:** the fixed build counts the frames it
  held (`held`). The report gives, for each cell, in how many scored runs the count is one
  or more, and whether those runs differ from the rest on any pattern.
- **A run whose count is zero is unchanged by the fix:** the report states it for the pairs,
  field for field, all but the wall time and the count itself.
- **The engines are compared on the re-frozen code only.** Wherever the report compares
  them, it says that the field engine has two results and the pairwise engine one: its half
  could not run on the frozen code, through a fault in my batch runner.
- **Which cells the gate put last** (Addendum 1). It changes the order of the runs only.

### The two results side by side: the counts

**Proposed by Paullus on 2026-11-11 at 21:23 UTC,** while the new batch ran its first step.
Varro has not read it yet. It is his engine and the conditions are his, so his reading may
change it. Neither of us has read a record of the new batch.

**What the first batch can give.** It ran `base` alone on the frozen code. The two
dose-response legs of pattern 6 need `veterans`, `recruits` and `flank`, and those never ran
on that code. So the frozen code's result is seven patterns and the band of the sixth. Where
the report counts "all eight" for it, pattern 6 is taken by its band alone, and the new
batch's count taken the same way stands beside it. The new batch's full verdict is
section 1's.

For each of the field engine's four cells, on `base`:

1. **The pairs.** Each record of the first batch beside the new batch's record of the same
   set and seed, a stopped run with a stopped run and a whole run with a whole one. The
   report gives:
   - the number of pairs, and of records with no partner in the other batch;
   - of the pairs whose count of held frames is zero: how many are equal field for field,
     all but the wall time and the count. Addendum 3 says every one must be. One that is
     not is a fault in the fix, and the report says so before anything else about the cell;
   - of the pairs whose count is one or more: how many differ in their summary.
2. **Where the defect could act:** the runs of `base` whose count is one or more, and the
   sets with such a run among their ten seeds.
3. **The verdicts of the two batches, set by set.** For each pattern, the sixth by its band:
   the sets that pass in both batches, in the first alone, in the new one alone, and in
   neither. The same for all eight taken that way.
4. **Whether those runs differ from the rest on any pattern** (Addendum 3's words), two
   ways, on the eleven values a run gives to the patterns (the list of
   [cell-check.md](cell-check.md)):
   - **Within the pairs whose count is one or more:** for each value, the pairs in which
     the first batch's run has the larger value, the smaller, the same, and those where one
     run has none. Here each run is its own control: the same set and seed, and the same
     battle until the first held frame.
   - **Against the rest of the cell,** in the new batch: for each value, the median of the
     runs whose count is one or more and of the others, and the probability that a run of
     the first group has the larger value.

**How it is read, said now:**

- **Item 3 is what bears on the verdict:** whether the defect changed the verdict of any
  set, and of the cell.
- **The pairs show what the defect did.** The comparison against the rest cannot: a set
  whose standard-bearer is driven back differs from the others in more than that. It can
  show that such runs are different battles, and no more.

**One count before the scoring, proposed:** the equal and unequal pairs among those whose
count is zero (item 1), as counts and nothing else, taken with the recheck and the crosswise
check. An unequal pair would mean the fix does more than its text says, and that should be
known before the verdicts are. It also tells how many runs have a count of one or more,
which is no outcome of a pattern.

## 3. Reported, not banded

| The report gives | For | Promised in |
|---|---|---|
| The audit: how often anything happens after the 15 minutes scored past the break (a bout, a rally, a second break), and that each audited seed's stopped run and whole run summarise the same | Each cell | The procedure |
| Stragglers: men who turned before the collapse | Each run | Patterns 3 and 4 |
| Tempo: the share of the first line's front-rank man-time spent forward while formed, and the share of those men who start any bout | Each cell | Pattern 7 |
| The shape of the time to break: the median Weibull shape over sets, and its spread | Each cell | The shape of the time to break |
| The two dose-response statistics on the Roman side's break time alone, and the number of sets where the Roman line broke in fewer than 2 seeds of `base` | Each cell | The dose-response |
| The loophole, by name: veterans who hold no longer than recruits but beat the warband sooner pass on skill at arms alone | Once | The dose-response |
| Each side's front losses before contact | `shock`, `volleys` | The variants |

**How two of these are given** (proposed by Paullus on 2026-11-11 at 21:23 UTC, not yet read
by Varro; no record of the new batch had been read):

- **Stragglers** are counted for each run, and a cell has 2,000 runs. The report gives, over
  the runs of `base` in which a side broke: the share with one or more stragglers, the
  quartiles of the count, and the largest.
- **Tempo** is pooled as the banded values are: each set's median over its seeds, then the
  quartiles over sets, for each of the two shares. Beside them, the number of sets inside
  the only figures there are (5 to 25 % of the time, 10 to 25 % of the men). Those figures
  are no band and no set fails on them.

## 4. The two memory variants

- **The statistic:** for each cell and set, the probability over seed pairs that the Roman
  line fares worse in `shock` than in `base`; the same for `volleys`. The report gives its
  distribution over sets, rung by rung.
- **Rung 0 is the control.** A memory shows as a difference larger than rung 0's.
- **Against what was expected beforehand:**
  - rung 1 shows more than rung 0 only where men fled from the shock and rallied;
  - rung 2 shows more than rung 1 in about half the draws, those where *shaken* wears off
    more slowly than the four minutes between the shock and the front;
  - rung 3 shows more than rung 2.
- **What `volleys` cannot test:** fear of the fire itself. Men see only the falls it causes.

### One rung against another: the count

**Proposed by Paullus on 2026-11-11 at 21:23 UTC,** while the new batch ran its first step.
Varro has not read it yet, and the expectations above are mostly his. Neither of us has
read a record of the new batch.

The expectations say that one rung "shows more" than another, and nothing yet says how that
is counted. Set *i* has the same shared values and the same weights at every rung, by
construction (`crates/cannae-score/src/draw.rs`). So set *i* at one rung can be put beside
set *i* at another. For each engine, and for `shock` and `volleys` apart:

- **Each rung against rung 0, and each against the rung below it:** over the sets scored at
  both, the median of the difference in the statistic, and the number of sets in which the
  higher rung's statistic is larger, smaller, and the same.
- **"Shows more", fixed now:** the higher rung's statistic is the larger in more sets than
  a fair coin gives 95 times in 100, ties left out. With 200 sets and no tie that is 115
  sets or more.
  - **Why a count over sets:** one set's statistic rests on ten seeds a side, so its
    difference between two rungs is mostly chance. Over 200 sets the chance cancels.
  - **There are twenty such comparisons** (five pairs of rungs, two variants, two engines),
    so one of them is expected past the line by chance alone. The report gives all twenty.
- **Rung 2 against rung 1 is also given in two groups,** by the wear of *shaken* drawn for
  the set: more than four minutes for one level to wear off, or not. The expectation is
  that the difference shows in the first group only.
- **Rung 1 against rung 0 cannot be given in the groups its expectation names.** The record
  does not hold whether men fled from the shock and rallied. It is given whole, and the
  report says so.

The relief's expectation of section 5, rung 3 against rung 2, is counted the same way on
the drill's statistic, over the sets that ran `relief`: with 50 sets and no tie the line is
33 sets or more.

## 5. The relief

Reported for each cell, never scored for a set:

- **The drill alone:** the mean over sets of the probability that the Roman line fares worse
  backed than relieved, with its 95 % interval; the share of sets above 0.5; and its rank
  correlation with each set's Weibull shape, expected positive.
- **The whole effect of a second line:** the same statistic for `base` against `relief`,
  arithmetic and all.
- **Whether the drill happened:** the share of `relief` runs in which one relief was ordered,
  and in which both were.
- **Each relieved run beside its twin** in `backed`: how many fared better, how many worse,
  how many the same.
- **Against what was expected beforehand:** rung 3 shows the drill's effect more than rung 2.
- **The top-up:** `relief` and `backed` on up to 25 more sets a cell, those where `base` was
  still being fought twenty minutes in. It runs only if Michal gives the machine time. That
  is decided on the machine time alone, before any relief statistic of the first 50 sets is
  read, and the report says which way. Its twins are reported apart from the first 50 sets'.
- **What the discs' passage does to the relief** (added by Varro, from
  [passage.md](passage.md), "A candidate remedy, not built"): passage capacity enters the
  scored runs only through `relief`. The discs pass men through a gap 10 to 20 % too freely,
  which makes a relief easier on the pairwise engine. That document promises that the effect
  is reported. How it is shown is defined below, before a relief statistic is read.

### How the discs' passage is shown in the relief

**By Paullus and Varro, 2026-11-11** (call 0001, turns 103 to 106): proposed by Paullus at
18:48 UTC, changed to Varro's reading of it at 18:58 UTC, and agreed by Varro at 19:04 UTC
with one change more, which is in. The new batch was in its first step then (`base`
on the field's rung 0). Neither batch has made a record of `relief`, and neither of us has
read a record of the new batch.

**The promise was written before the drill.** [passage.md](passage.md) made it on 2026-11-10
at 13:37 UTC, with a maniple falling back through an interval in mind. The relief was
scripted three hours later, and it has no interval:

- **Files pass through files.** The fresh century walks up the lanes between the tired
  century's files, and the tired one then falls back down the lanes of the fresh one. A lane
  is 1.8 m from file to file and carries one file, a man every 1.8 m.
- **The two centuries together are 160 men on 18 m by 14.4 m:** 0.62 men per m². The bench
  measured the discs' excess where a crowd jams at a mouth, and its jams stood at 2.9 to
  4.8 men per m² there.
- **In the lanes the two engines do not differ in kind.** On the field a man in his place is
  moved by his tie alone and reads no density; its walking curve is read in `relief` only by
  men out of their places (Varro). On the pairwise engine a man walks to his slot at his
  pace until he touches someone.
- **So the discs' fault can act only in a jam:** men out of their places and crowded, because
  an enemy came on while the centuries passed, or because men in flight ran through the
  lanes.

**Three marks, fixed now.** Each is taken between the relief's order and the tired century's
halt, or the run's end if it never halts:

- **Pressed:** a bout began with a man of either century. It is what the enemy did.
- **Jammed:** at some man step, some man of the two centuries had eight or more others on
  their feet within a metre of him, at least one of them a man of the other century of the
  two. With himself that is nine men on 3.14 m², 2.9 per m²: the bench's own measure of
  density, and the lowest at which it jammed. It is counted on positions, the same way on
  both engines (Varro).
  - **Why a man of the other century must be among them** (Varro): a front in close fight
    is that dense too, one century and the enemy at about half a metre a man, and nobody
    passes through anybody there. The promise is about men passing through men.
- **Unfinished, by reason** (Varro): one of the two centuries broke before the halt; or
  neither did and the run ended, which is a drill that stuck.

**What the report gives.** None of it changes a statistic of section 5, and no correction is
applied to anything.

1. **The drill alone, on both engines.** The fixture of the relief's own test
   (`crates/cannae-score/tests/relief.rs`: the relief patch with its relief allowed from
   half a minute, men whom nothing moves, nobody fights), at the nominal set. For each
   engine:
   - the seconds from the order until the fresh century has the front, and from then until
     the tired one halts where the fresh one stood. Both are events of the frames;
   - the same two times for a free walk over the same ground at a man's pace;
   - **when the men are back** (Varro): of the men of the two centuries who were in their
     places when the relief was ordered, how many are out of them when the tired century
     halts; and the seconds from the halt until nine in ten of the tired century's men who
     were in their places at the order are in them again. Men who have fallen since are left
     out. A frame halts wherever its men are, so the frames' times alone could not show a
     jam;
     - **Why the men at the order** (Varro, turn 114, on reading the tool): a man who fled
       earlier is still of his body and out of his place as long as he is adrift. Counted
       over every man on his feet, a thinned century would never be "back", with no jam
       anywhere.
   - the nearest that a man of one century comes to a man of the other, and whether the
     drill was jammed by the mark above.
2. **The reliefs of the scored runs, replayed.** A replay changes no record: it runs the
   same set and seed again and must end in the record's own summary. One that does not is
   not counted, and the report says how many did not. For each cell, the relieved runs of
   `relief` are replayed, lowest set first and then lowest seed, until 100 reliefs have been
   seen or the runs are used up. For each relief: the times and the count of item 1, and the
   three marks.

   The report gives, for each cell: the reliefs seen; the median and the longest of each
   time; pressed against jammed as a two-by-two count; and the unfinished by reason.
3. **The runs beside their twins.** For the runs with a jammed relief, for those with a
   pressed relief and none jammed, and for the rest: how many fared better than their
   `backed` twin, how many worse and how many the same.

**How it is read, said now:**

- **The jammed count is how often the fault could act.** The bench's figures apply to a
  jammed relief: there the discs pass 10 to 20 % too freely when men use the whole mouth,
  and 30 to 50 % on the bench's first steering. The report says how many reliefs that is on
  each engine, and what became of them.
- **A pressed relief is where a jam is likely.** The two-by-two count says how often it
  came, and how often one came with no enemy in it.
- **A relief neither pressed nor jammed** is the drill of item 1 inside a battle.
- **If no relief of a cell was jammed,** the report says that the effect the promise named
  had nothing to act on in that cell, as far as the replayed reliefs show.

**What it cannot show:**

- **How the relief would have fared with the capacity right.** The remedy was not built, so
  that battle cannot be run. The report gives how often the fault could act and not what it
  did.
- **Nothing is read from the difference between the engines.** They differ in everything
  else too, so a larger effect of the drill on one of them is not the passage's doing, and
  the report does not argue that it is.

**What it needs:** a small replay tool outside the scored code, written and checked on
unscored sets before it is pointed at a scored run. Varro reads it before it is used. It is
run after the scoring, on the Pi.

## 6. What is known to be wrong in the scored code

- **D1, a defect left in by Michal's decision** (Addendum 4): on the field engine, two
  enemies on their feet can stand nearer than the guard's distance. The report names it with
  the counts from the QC pass. The scored runs do not count it.
- **The gaps of the QC pass:** 38 flags on the pairwise list and 50 on the field's, things
  that look wrong although the code does what its text says
  ([the page](qc/README.md)). First among them: a formation follows its standard-bearer when
  he himself gives ground.
- **The list for a second round,** 1 to 25 in [STATUS](../STATUS.md), none decided.
- **What Michal saw at the mid-range set** (a century that broke three seconds after its
  centurion fell; a warband standing beside an open flank). STATUS says the batch shows how
  often the drawn sets do this. The two counts are defined below.

### A century and its centurion: the count

**By Paullus and Varro, 2026-11-11** (call 0001, turns 103 to 106): proposed by Paullus,
corrected by Varro, and agreed by him at 19:04 UTC. Neither of us had read a record of the
new batch.

**The scored record cannot count it.** It holds each body's break and the pooled hits on
leaders, and not when a leader fell. A replay with the event log does. A replay changes no
record, and counts only if it ends in the record's own summary.

- **The sample:** `base`, sets 0, 10, .. 190, all ten seeds, in every cell: 200 runs a cell.
- **A leader's fall:** the first time the leader of a front body goes down, if his body has
  not broken by then.
- **A file leader's fall,** for comparison: every fall of a man who is a file leader at the
  tick he goes down, in a front body not yet broken. Files close up and renumber, so it is
  his role at that tick and not his rank at the start (Varro).
- **For each fall:** the seconds from it to his body's break, or "never"; and the share of
  the body's standing men who are in flight at that tick.

**The report gives,** for each cell and each side, for the leaders and for the file leaders:

- the number of falls;
- how many were followed by the body's break within 5 s, and within 30 s, and the median
  time of the rest;
- the median share of the body in flight at the fall (Varro). A centurion may fall at a
  different point of the fight than a file leader, and this shows whether he does;
- **for the Roman bodies,** in both groups: how many falls were followed by the break
  before a successor could have the leader's place, by each set's own delay (Varro). The
  delay is drawn for each set, 2 to 15 s. A file leader's fall gets the same window, so that
  the leaders' figure has one beside it. The warband has no succession: its men see their
  leader down for the rest of the battle.

**The windows are fixed now:** 5 s and 30 s. What Michal saw was three seconds.

**How it is read, said now:** if a centurion's fall is followed by the break within 5 s much
more often than a file leader's, with the body in a like state, then the rule does what
Michal saw, and the report says in what share of cases. The count pools twenty sets, and the
fear a fallen leader adds is drawn for each. So it says how often the drawn sets do this,
and not how strong the rule is in any one (Varro).

### The warband beside an open flank: the count

**Proposed by Paullus on 2026-11-11 at 18:58 UTC, with Varro's additions of 19:04 UTC** (call
0001, turns 105 and 106).

No rule lets a body turn on a flank: a body follows its one scripted order, in every set. So
there is nothing to count about the rule. What can be counted is how often the situation
arises, and the scored record holds it:

- **For each cell, on `base`:** the share of runs in which one Roman century broke and, a
  minute later, the other century and the warband were both unbroken. A warband that had
  broken by then stands beside nothing (Varro).
- **The minute is a choice of ours:** long enough for a commander to act, and short against
  the fifteen minutes the run goes on.
- **What the count is not** (Varro): it counts the opening, and not what the warband's men
  did at it. Men near the gap still go after runners one by one; the body as a body follows
  its order.
- **The script:** `tools/s09-open-flank.py`. It reads the scored records alone and is run
  after the scoring. A run that ends inside the minute with both still standing is counted
  apart, as neither; the share is given over all the runs, and the three counts with it.
  Varro read it (call 0001, turn 122).

## 7. Checks on the code and the runs

- **The two fixes in the re-frozen code,** each with how it was shown: the standard-bearer
  fix by its tests, its 20 mutations and the unscored runs with a count of zero; the runner
  fix by records equal to the first batch's, in my check and in Varro's own.
- **What was chosen with the verdict in hand:** the runner fix, and leaving D1 in.
- **The pre-flight:** every one of the 64 steps started on the machine that ran the batch.
- **The same results on the Mac and the Pi,** bit for bit.
- **The QC pass:** how many battles, flags and pictures, and that both dormancy checks pass.
- **The field's cell size** (added by Varro, from [priors-field.md](priors-field.md), "Fixed,
  and not drawn"): the cell is 1.024 m by structure and not drawn. The priors promise a check
  at 0.512 m with the results: the base patch at the centre run again, and how far each of
  the eight measures moves. Varro owes that run. It is made on a build of its own and never
  on the scored code, and its conditions are written before it is run.
  - **Run on 2026-11-11** ([cell-check.md](cell-check.md)). The report gives all three
    things that document sets side by side: what the line fixed beforehand says, what kind
    of battle it was read on, and how the field's own tests fare on the half-cell build.
- **Every outcome either of us saw before the scoring,** from the pre-registration's table.

## 8. Speed

- **Each engine against its own line, as first read** ([spike-pairwise.md](spike-pairwise.md),
  [spike-field.md](spike-field.md)). Those readings are the verdict on the engines as built.
- **The share of the shared perception** in each reading.
- **The second line** Michal chose, with the perception on all threads, beside the first and
  never in its place. Its conditions were written before it was read.
- **What the figures leave out:** the man is not in the pairwise figure, nor in the field's
  (added by Varro: [spike-field.md](spike-field.md), "the man only adds cost").
- **The field engine is not read a second time** (added by Varro, 2026-11-11): its line is
  for one core, and its pocket was under 10x with the perception's time taken out.

What follows from the speed is Michal's to decide with this report.

## 9. The passages

- **Both engines on the same bench** ([passage.md](passage.md),
  [passage-field.md](passage-field.md)), and how each was calibrated on measured flows
  ([passage-calibration.md](passage-calibration.md)).
- **The field's flow before its calibration beside the calibrated one** (added by Varro, from
  [passage.md](passage.md)): before it the field was 40 % low, and that document promises
  both numbers.
- **The one difference in the press, as it comes:** on the pairwise engine a loose man who
  presses shoves a formed man; on the field a man in his place is moved by his tie alone. The
  interval passage shows it.
- **The limit that stands:** people push through a 0.7 m door about 40 % faster than discs.

## 10. What Slice 0 does not answer

- **Whether a formed line shoves.** A man in his place does not press in Slice 0, on either
  engine. That is put off to Slice 1, not settled.
- **Fear of missiles that fell nobody.**
- **The shape of the hazard of breaking.** No source gives it, so it is reported and no rung
  is failed on it.
- **Any second round.** Nothing is retuned under this pre-registration.

## 11. After the report

Not results, and none of them mine to decide:

- Michal reads the report and decides what is kept.
- I propose design review 2.
- `docs/vision/` is revised to the results, with Michal's approval.
