# Slice 0: what the report owes

**Status:** draft by Paullus, 2026-11-11, written before the new batch has made a record.
Varro read it on the same day, also before any record, and added four things from the
documents outside the pre-registration. Each is marked "added by Varro".

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
  is reported; how it is shown is Paullus's to say before a relief statistic is read.

## 6. What is known to be wrong in the scored code

- **D1, a defect left in by Michal's decision** (Addendum 4): on the field engine, two
  enemies on their feet can stand nearer than the guard's distance. The report names it with
  the counts from the QC pass. The scored runs do not count it.
- **The gaps of the QC pass:** 38 flags on the pairwise list and 50 on the field's, things
  that look wrong although the code does what its text says
  ([the page](qc/README.md)). First among them: a formation follows its standard-bearer when
  he himself gives ground.
- **The list for a second round,** 1 to 24 in [STATUS](../STATUS.md), none decided.
- **What Michal saw at the mid-range set** (a century that broke three seconds after its
  centurion fell; a warband standing beside an open flank). STATUS says the batch shows how
  often the drawn sets do this. *Not settled, and added by this list:* what the scored
  records can say about it beyond pattern 6's share of early breaks. Varro and I settle that
  before a record is read.

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
