# 7. Checks on the code and the runs

**Draft by Paullus, 2026-11-19.** From Addenda 3 and 4 and the table of outcomes seen in the
[pre-registration](../preregistration.md), [STATUS](../../STATUS.md), the QC results
([qc-results.md](../qc-results.md), [qc-results-field.md](../qc-results-field.md)) and
Varro's [cell-check.md](../cell-check.md). The figures are those documents' own, each
entered when its check was made.

## The re-frozen code

**40b8dcd is the first freeze, dd9d90a, with two fixes and nothing more** (Addendum 4). Varro
checked the record of the re-freeze line by line: the five files that differ from dd9d90a are,
blob for blob, those of the two fix commits, and the freeze's hashes, seed and table are the
first freeze's.

- **The standard-bearer fix (ed6d363),** committed by Varro on 2026-11-10 at 23:15 UTC, six
  hours before the gate of the first batch ran. Shown by its tests, its 20 mutations, and the
  runs whose count of held frames is zero: on 144 unscored runs, the 3 with a count of zero
  had equal event logs with and without it, and the 141 others differed. In the scored batch,
  **every field run of the first batch that the fix did not act in is unchanged in the new
  one,** 2,893 of 2,893, field for field but for the wall time and the count (section 2).
- **The runner fix (449dcd6),** mine, after the batch runner overflowed its stack on every
  pairwise step. It runs battles on plain threads. Shown to change no record: 90 field records
  made with it equal the first batch's own from the Pi, and 80 pairwise records equal the
  frozen runner's with one job. Varro's own check, on runs of his choosing: 640 records, 320
  on each engine, equal to the frozen runner's with one job, and the 320 field records also
  equal to the frozen runner's with four jobs. A control with one field changed in one record
  reported that record and no other.

## What was chosen with a verdict in hand

When the code was re-frozen, both of us knew that on the frozen code the field engine's four
cells could not pass on `base` (Addendum 4; the table of outcomes seen, 2026-11-11 06:39 and
06:47). Two things were chosen after that:

- **The runner fix,** written after the gate had run. It is shown above to change no record.
- **Leaving D1 in.** It changes no code. Varro, who knew his engine's result, recommended it,
  I agreed, and Michal decided (section 6).

The standard-bearer fix was not chosen so: it is pinned to the commit made before the gate
ran. Both speed verdicts were also known before the re-freeze (section 8), and they changed
nothing in the batch.

## Before the batch started

- **The pre-flight:** every one of the 64 steps was started on the Pi, in the batch's own
  checkout, on 2026-11-11 from 16:11:51 to 18:22:48 UTC: 460 runs, none failed. The batch ran
  nothing for the first time.
- **The gate's scorer** differs from the re-frozen scorer by the gate line, the crosswise
  switch and two hashes in each record, and Varro read it. Tested on unscored sets: 30 of 30
  records equal on the Mac on each of two cells, and the scorer's output equal but for the
  gate line, on the Mac and on the Pi.
- **The crosswise check of the two perceptions** (Addendum 2), on the re-frozen code: 160 of
  160 runs gave the same end state and the same event log through either perception, and
  all 160 later agreed with the batch's own records. On the frozen code it had passed the
  same way, and the 80 of its runs that the first batch had also made agreed with its
  records.

## The batch, the top-up and the copies

- **The batch** started on 2026-11-11 at 18:39:32 UTC and finished all 64 steps with none
  failed; its log's last line was seen at 19:42 UTC on 18 November.
- **The same results on the Mac and the Pi.** For every cell and variant the Mac made two of
  its sets again, and every record equals the Pi's; all 64 cells and variants are in. The
  top-up got the same recheck: 320 of 320 records equal. The Mac's copy of the records equals
  the Pi's byte for byte.
- **The audit:** in all 64 audit lines (eight cells, eight variants), the stopped and the whole
  summaries of the audited runs are equal, 100 of 100 in each (section 3).
- **The replays** count a run only if the scorer's own run of its set and seed gives the
  record back. It did in every one. The first replays left out 67 for a fault of the replay
  tool, which stopped early; mended after the report and run again on 2026-11-20, every
  replay counts (sections 5 and 6).

## The QC pass

On the re-freeze candidate (9d96089), before the new batch: **320 battles of the nominal set,
160 on each engine,** on the Pi.

- **151,258 flags,** of which the rule picked 170: 75 on the pairwise engine and 95 on the
  field's. Each was looked at in a four-panel picture and binned.
- **Pairwise:** no defect; 37 fine and 38 gaps. **Field:** one defect, D1 (four of the
  flags); 41 fine and 50 gaps.
- **Both dormancy checks pass in all 320 battles.** Over the first 300 s, a man the runner
  skips has every hazard at zero; over the first 120 s, deciding for every man gives the same
  hash after every man step as the normal run.
- **The control:** on the frozen code, the pass's first detector flags the step back Michal
  saw, at 29.25 s (70 of 77 slots moved 1,800 mm back).

## The field's cell at half its size

The priors promised a check of the field engine's cell, 1.024 m, at 0.512 m, made on a build
of its own and never on the scored code, its conditions written before it ran. Varro ran it on
2026-11-11; [cell-check.md](../cell-check.md) is his account, and it asks for three things side
by side:

1. **By the line fixed beforehand, the half cell moved the measures and changed the verdicts
   no more than seeds alone do** at the nominal set (435 and 1,000 splits of 1,000 reach its
   moves, where the line asked for fewer than 25).
2. **It was read on half-minute battles.** At both sizes the nominal set's formed phase had a
   median of 22 to 42 s by rung, and 87 to 97 % of runs broke within five minutes of contact.
   "No more than seeds" is weak evidence for a battle that holds.
3. **Eight of the field's 44 tests fail at the half cell.** In a crowd of two per m² a walker
   covers about a sixth of the calibrated distance. So the cell is a parameter of the field's
   crowd rules, not a matter of structure. Varro recommended it for the second round with
   the result in hand; it is item 25 there.

## Every outcome seen before the scoring

The pre-registration's table "Outcomes seen during the batch" has every scored outcome
either of us saw before the scoring, with when, who and how; Addendum 1 adds two it
disclosed itself. They are:

- **four single runs of scored sets:** two that I read while testing the crosswise check
  (Addendum 1), and two of the first batch (field, rung 0, `base`) seen on 2026-11-10 while
  checking that the batch had resumed;
- **the first batch's gate for the four field cells,** inferred by me on 2026-11-11 at 06:39
  from the failed pairwise steps and read by Varro in my turn at 06:47: on the frozen code,
  none of the four could pass on `base`;
- **part of the new batch's gate,** inferred by me on 2026-11-12 at 14:37 from the name of
  the first step after `base`. It left two cases: the field's rung 0 could still pass, or no
  cell of either engine could. **The second held:** the gate said "cannot pass" for all eight
  (section 1);
- **the top-up's one figure,** about 4 hours of the Pi's time, for Michal's decision, on
  2026-11-18.

The table's other rows are checks on unscored sets or of no pattern's outcome: the cell
check, the tools tried on sets drawn with another freeze seed, and the check that the pairs
the fix did not act in are equal.

## What the batch's order and counts let through

**The order was a channel** (Paullus and Varro, 2026-11-12). After `base` the variants of the
cells that can still pass run first (Addendum 1), so the order of the steps is the gate's
verdict. Neither of us had counted that as a way of seeing an outcome until the first such
step had run (the row of 14:37 above). From 14:39 UTC that day the batch was looked at by
counts only.

**Two more channels were found and closed on 15 November:**

- **the recheck's total of records** (mine, closed at 02:53 UTC): each cell and variant adds
  20 records, or 30 when one of its two sets is also one of the audit's, so the totals set
  against the freeze seed's choices would tell which steps had run;
- **the count of steps done at known times** (Varro's, closed at 02:56 UTC): `backed` and
  `relief` steps are about a quarter as long as the others, so where a run of short steps
  comes in the count tells how many cells the gate kept.

**What was published before they were closed:** the steps done at each look (8 at 14:37 UTC
on 12 November, then 11, 16, 21, 25 and 29 at about 02:45 and 14:45 UTC each day to 15
November), the paces and end dates drawn from them, and the recheck's totals (160 to 590), in
STATUS and in my notes; and the same counts and totals in Varro's yip roadmap notes from 13
November 02:45 to 14 November 14:50 UTC and in his summaries to Michal. **Neither of us worked
anything out from them.** From then until the batch's last line every report said "running,
none failed" and nothing more.

**What could not be closed:** a step's length differs by engine, and the top-up's total
figure was published for Michal's decision.

**What the channels held, now that it can be said:** no cell of either engine could pass, so
the variants ran in the usual order, as if there were no gate. What the channels could have
carried is that one fact, which section 1 now gives.
