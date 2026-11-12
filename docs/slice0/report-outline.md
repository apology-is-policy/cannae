# Slice 0: what the report owes

**Status:** draft by Paullus, 2026-11-11, written before the new batch has made a record.
Varro read it on the same day, also before any record, and added four things from the
documents outside the pre-registration. Each is marked "added by Varro". One of them left a
definition to me: how the discs' passage is shown in the relief. It is in section 5, proposed
on 2026-11-11 at 18:48 UTC, while the new batch ran its first step, and agreed with Varro's
changes by 19:06 UTC. Section 6 has the counts for what Michal saw at the mid-range set,
agreed in the same turns.

**Four more parts were fixed the same evening** (proposed by Paullus at 21:23 UTC, read and
corrected by Varro at 21:29 UTC, call 0001, turn 126; the new batch was still in its first
step): how a pattern's failures are counted (section 1), the field engine's two results
side by side (section 2), how stragglers and tempo are given (section 3), and how one rung
is counted against another (section 4). Each fixes how a promised figure is counted, before
a record is read.

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
4. **The pairs the standard-bearer fix did not act in** (section 2, agreed in call 0001,
   turn 126): each of the field engine's runs of `base` whose count of held frames is zero,
   beside the first batch's run of the same set and seed, as equal or unequal and nothing
   else. If one pair is unequal, we stop and tell Michal before anything is scored. What is
   then looked at to find the fault is agreed with them first.
5. **Michal has decided the top-up of the relief** (section 5), on the machine time alone,
   before any relief statistic is read.
   - **One figure is printed for that decision** (Varro, call 0001, turn 132): the top-up's
     machine time summed over the eight cells. How many sets each cell would add is an
     outcome of `base`, so it stays unread until the scoring. The total is entered in the
     table of outcomes seen, since it is not nothing.
   - **How the figure is made** (`cannae-extras topup`, written on 2026-11-12 with `base`
     in and nothing of it read for this): for each cell, the time `base` took on the
     top-up's sets, times what `relief` and `backed` cost beside `base` on the stride's sets
     that were as long fought, or on all of the stride's sets where none was. The sum over
     the eight cells is divided by four, since the batch runs four battles at a time. It is
     an estimate: a relieved line may fight longer on the top-up's sets than on the
     stride's.
     - **What it reads of `relief` and `backed`:** the wall time of their runs and nothing
       else. It prints the one total, rounded to an hour, and names no cell, in the figure
       or in a refusal.
     - **Checked:** on the eight `base` steps, the runs' own times added up and divided by
       four give 0.999 of each step's length. On made-up records the total equals a count
       made apart in Python, among them a cell with no set for the top-up, one over the cap
       of 25 and one with no long-fought set on the stride. Its sets are held to the
       scorer's (`report::topup`), and it stops if they differ.
     - **It is run once,** when the batch has finished and Varro has read it.

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

**By Paullus and Varro, 2026-11-11** (call 0001, turn 126): proposed by Paullus at 21:23
UTC, while the new batch ran its first step (`base` on the field's rung 0), and agreed by
Varro at 21:29 UTC with three additions, which are in. Neither of us had read a record of
the new batch.

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
| 7 Casualty clock | hits per file-hour under the band, or over it; harm per bout under its band, or over it; no seed gives a harm per bout (Varro: the scorer fails such a set) |
| 8 Centurion gradient | the leaders' share of hits in the formed fight not above their share in the rout |

- **A set that fails on two parts is counted under each.**
- **A set that cannot be scored on a pattern** is counted apart, as the scorer counts it,
  and by the value it lacks (Varro). Patterns 1, 2 and 4 cannot be scored when either of
  their two values is missing, even if the other lies outside its band. Pattern 8 needs a
  hit in the formed fight and a hit in the rout. "No seed broke" and "too few bouts to have
  a gap" are different failures, and the pre-registration counts both as failing.
- **Beside the counts:** the quartiles of each pooled value with its band, over the sets
  that have the value, so that a reader sees how far off the sets lie. For pattern 6 the
  pooled values are the two shares of a set's seeds, those that break early and those that
  never break. For pattern 8 they are the leaders' two shares of the hits, each pooled over
  the set's seeds (Varro).
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

**By Paullus and Varro, 2026-11-11** (call 0001, turn 126): proposed by Paullus at 21:23
UTC, while the new batch ran its first step. Varro read it at 21:29 UTC, changed how
item 3 is read, and added to items 2 and 4. All of it is in. Neither of us had read a
record of the new batch.

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
   sets with such a run among their ten seeds; and the quartiles of the count over the runs,
   with its largest value (Varro).
3. **The verdicts of the two batches, set by set.** For each pattern, the sixth by its band:
   the sets that pass in both batches, in the first alone, in the new one alone, and in
   neither. The same for all eight taken that way.
4. **Whether those runs differ from the rest on any pattern** (Addendum 3's words), two
   ways, on the eleven values a run gives to the patterns (the list of
   [cell-check.md](cell-check.md)):
   - **Within the pairs whose count is one or more:** for each value, the pairs in which
     the first batch's run has the larger value, the smaller, the same, and those where one
     run has none. Here each run is its own control: the same set and seed, and the same
     battle until the first held frame. The hits per file-hour are taken for each side
     apart, since a pair has one run on each hand.
     - **Pattern 8 has no value for one run** (Varro). So the report gives the leaders' two
       shares of the hits, pooled over those pairs, for each batch.
     - **The ten runs of a set share their parameters, so 2,000 pairs are not 2,000 looks**
       (Varro). Beside the count over pairs, for each value: the sets whose pooled value is
       larger in the first batch, smaller, and the same. Its line is the two-sided sign
       line of section 4 over the sets not tied. That is 44 looks over the four cells, so
       about two pass it by chance.
   - **Against the rest of the cell,** in the new batch: the sizes of the two groups first.
     Then, for each value, the median of the runs whose count is one or more and of the
     others, and the probability that a run of the first group has the larger value. With
     fewer than ten runs in a group the medians stand alone, as in the cell check (Varro).

**How it is read, said now:**

- **Item 3 is read on its two middle counts** (Varro): the sets that pass in the first
  batch alone against those that pass in the new one alone, by the two-sided sign line of
  section 4 for their sum. An even split says the fix moved no verdict one way. The cell's
  verdict is what each batch gives.
  - **Why not "did any set's verdict change":** in the unscored test of Addendum 3 the frame
    was held in 141 of 144 runs. So nearly every pair parts at its first held frame, and
    from there the two runs are different battles of one set. Verdicts then flip both ways
    by chance, as they do between seeds: in the cell check, seeds alone changed a median of
    3 verdicts in 32 at thirty seeds a group, and a set has ten.
  - **2026-11-11, 21:52 UTC, Varro, after the first cell's count before the scoring:** on
    the field's rung 0 the frame was held in 1,389 of 2,100 scored runs, about two in
    three, and not in nearly all, as the unscored test had led me to say. The rule stays as
    it was fixed before that count was known. Its reason holds in a weaker form: the pairs
    that part are still most of them, and a verdict can still flip by chance through them.
    How those runs lie among the sets is item 2's and is not counted yet. Two things follow
    and change no count: a pair that does not part is one run in both batches and adds
    nothing to a flip; and "the rest" of item 4 is no handful in this cell, so its
    probability will be given.
- **The pairs show what the defect did.** The comparison against the rest cannot: a set
  whose standard-bearer is driven back differs from the others in more than that. It can
  show that such runs are different battles, and no more.

**One count is taken before the scoring** (agreed): the equal and unequal pairs among those
whose count is zero (item 1), as counts and nothing else, with the recheck and the
crosswise check. An unequal pair would mean the fix does more than its text says, and that
should be known before the verdicts are. If there is one, we stop and tell Michal before
anything is scored (Varro). The count also tells how many runs have a count of one or more,
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

**How two of these are given** (proposed by Paullus on 2026-11-11 at 21:23 UTC, agreed by
Varro at 21:29 UTC with two words more, call 0001, turn 126; no record of the new batch had
been read):

- **Stragglers** are counted for each run, and a cell has 2,000 runs. They are the loser's
  men. The report gives, over the runs of `base` in which a side broke: the share with one
  or more stragglers, the quartiles of the count, and the largest.
- **Tempo** is pooled as the banded values are: each set's median over its seeds, then the
  quartiles over the sets that have the value, for each of the two shares. Beside them, the number of sets inside
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

**By Paullus and Varro, 2026-11-11** (call 0001, turn 126): proposed by Paullus at 21:23
UTC, while the new batch ran its first step, and agreed by Varro at 21:29 UTC with six
additions, which are in. The expectations above are mostly his. Neither of us had read a
record of the new batch.

The expectations say that one rung "shows more" than another, and nothing yet says how that
is counted. Set *i* has the same shared values and the same weights at every rung, by
construction (`crates/cannae-score/src/draw.rs`). So set *i* at one rung can be put beside
set *i* at another. For each engine, and for `shock` and `volleys` apart:

- **Each rung against rung 0, and each against the rung below it:** over the sets scored at
  both, the median of the difference in the statistic, and the number of sets in which the
  higher rung's statistic is larger, smaller, and the same.
- **"Shows more", fixed now: the two-sided sign line.** Of the sets not tied, the higher
  rung's statistic is the larger in so many that a fair coin gives as many or more only
  once in forty times. With 200 sets and no tie that is 115 sets or more.
  - **The line follows the number of sets not tied** (Varro). With fewer than six such sets
    no count can reach it, and the report says "cannot be told". Past the line the other
    way is reported as "shows less".
  - **Why a count over sets:** one set's statistic rests on ten seeds a side, so its
    difference between two rungs is mostly chance. Over 200 sets the chance cancels.
- **Sets that are one run at both rungs are counted apart from the other ties** (Varro): the
  sets whose twenty runs, ten of `base` and ten of the variant, fared the same at the two
  rungs, run for run. Ties will be many for rung 1 against rung 0, and they mean something.
  Rung 1's one term is read only for a man who has turned before, and the seeds and the
  shared values are the same at both rungs. So until a man who has turned is judged again,
  the two runs are one run.
  - **For rung 1 against rung 0** that count is the nearest the record has to "where no man
    fled and rallied", the group its expectation names. The record does not hold flights
    before contact, so the comparison is given whole, and the sign count leaves those sets
    out by itself.
- **Rung 2 against rung 1 is also given in two groups,** by the wear of *shaken* drawn for
  the set: more than four minutes (4,800 base ticks) for one level to wear off, or not.
  Each group has the line for its own sets. "Only in the first" means past the line there
  and not in the second (Varro). 57 % of the prior lies above the cut.
  - **A weakness of the expectation, said by Varro before any record:** *shaken* has up to
    three levels, and one level takes the drawn time to wear off. A man with two levels
    still carries one after four minutes, even where one level wears off sooner. If the
    second group shows the difference too, that is the likely reason. The split stays as
    the pre-registration has it.
- **The number of looks** (Varro): five pairs of rungs, two variants and two engines are
  twenty. The line lets a fair coin past at 2 % each way, so by chance 0.4 of the twenty
  pass it upward and 0.8 either way. With the eight counts in groups and the two for the
  relief it is thirty looks. The report gives them all.

**The relief's expectation of section 5,** rung 3 against rung 2, is counted the same way on
the drill's statistic, over the first 50 sets that ran `relief`. The top-up is reported
apart. With 50 sets and no tie the line is 33 sets or more.

- **Where no relief was ordered, `relief` is `backed` run for run** (Varro), and the
  statistic is 0.5 at both rungs. So the count rests on the sets with a relief ordered, and
  the line follows their number. At the mid-range set none was ordered on either engine.

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
- **What the batch's order let through** (Paullus and Varro, 2026-11-12, call 0001, turns
  131 and 132). After `base` the variants of the cells that can still pass run first
  (Addendum 1), so the order of the steps is the gate's verdict. Neither of us had counted
  that as a way of seeing an outcome until the first such step had run.
  - **From 12 November, 14:39 UTC, the batch is looked at by counts only:** steps done and
    failed, twice a day, and the recheck's total. No step's or file's name, no process
    list with its arguments, no time or size of a single step. A failed step or an unequal
    record is the exception, because then the name is needed.
  - **Kept out of first lines too:** commit titles, the heads of STATUS and the yip
    roadmap carry no step's name and nothing inferred from one.
  - **What could not be closed:** a step's length differs by engine, and the top-up's
    total above. The report says so.

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
