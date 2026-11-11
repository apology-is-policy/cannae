# Slice 0: the visual QC pass

**Status:** Paullus, 2026-11-11. The detectors and their thresholds below are fixed in this
commit, before the tool exists and before any battle it flags has been looked at. Varro reviews
the list first; his changes are dated lines at the end.

## Why

Michal watched a handful of battles in the viewer and found three faults that no test had
caught: a century stepping back a rank as one, a runner pinned behind his own men, and a man
running half a second after he was steady. The first is a defect in the frozen code and is in
two scored runs of three. Michal asked for a regular pass of the same kind: look at what the
men do, and flag what looks wrong.

The batch is restarted once, with fixes ([preregistration](preregistration.md), Addendum 3 and
the one to come). This pass runs **before** that re-freeze, because no restart comes after it.
It runs again at every freeze and at the close of each slice.

## What was already seen when this list was written

- **Battles looked at closely:** the nominal set on the pairwise engine, `base`, seed 1, at
  rungs 0 and 2 (positions of every man for the first 42 s), and the same fight on the test
  physics. Detectors 1 to 3 come from them.
- **Outcomes known:** both of us know that on the frozen code the field engine fails the 10 %
  rule on `base` at rungs 0 to 3 (the pre-registration's table). So this list is written, and
  every flag is judged, with that knowledge. The safeguards are the thresholds fixed here, and
  the rule under "How a flag is judged".

## The battles

- **The nominal set only,** which is never scored. No drawn set is looked at.
- Both engines, rungs 0 to 3, all eight variants, seeds 1 to 5: **320 battles.**
- 600 simulated seconds each; 1,800 for `relief` and `backed`, whose second line acts late.

## The detectors

A man step is 250 ms. A man is **up** if he is not down, and **standing** if he is steady,
forward or giving ground. Distances are between centres.

| # | Name | It flags | Why it would be wrong |
|---|---|---|---|
| 1 | `slot-jump` | At one man step, the slots of at least half of a body's slotted men move more than 1,200 mm, by the same vector to within 100 mm | A led frame moves a walking or running step. A jump is a frame re-formed |
| 2 | `halted-body-walks` | While a body's order is to hold, at least half of its standing men move more than 600 mm over 2 s, within 30 degrees of one direction | A halted body should stand |
| 3 | `pinned-runner` | A man in flight and up moves less than 500 mm in 2 s. Reported with how long it lasts | A man who runs should get away, or be cut down |
| 4 | `overlap-enemies` | Two up men of different sides closer than 300 mm | Both engines keep enemies apart |
| 5 | `overlap-own` | Two up men of one side closer than 300 mm. A flag on the pairwise engine, a count on the field, where men of one side are not solid | Two bodies in one place |
| 6 | `crowd` | 24 or more up men in one 2 m cell (6 per m²) | Past the density at which crushing begins |
| 7 | `jump` | An up man moves more than 1,500 mm in one man step (6 m/s) | Faster than a man runs |
| 8 | `down-moves` | A man who is down is more than 100 mm from where he fell | The fallen lie still |
| 9 | `facing-in-place` | A standing man in his place faces more than 90 degrees off his slot's facing for 1 s or more | A man in line faces as his body does |
| 10 | `flight-toward` | A man in flight moves faster than 500 mm/s, within 45 degrees of the direction to the nearest standing enemy within 10 m, for 1 s or more | He is running at the enemy |
| 11 | `bout-reach` | A bout between men more than 3,000 mm apart | Out of any weapon's reach |
| 12 | `bout-state` | A bout whose attacker is in flight or down, whose defender was already down, or whose two men are of one side | No such bout should be drawn |
| 13 | `bout-through` | A bout with a third up man within 250 mm of the line between the two, in the middle three fifths of it | A strike through a man |
| 14 | `flicker` | Six or more changes of posture by one man within 5 s | A man does not change his mind every half second |
| 15 | `standard-front` | A formed body's standard is carried by a man with no slot ahead of him. A count, with the first time | Reported, since it follows from files closing; judged by eye |
| 16 | `out-of-place` | A steady man with a slot who is not in his place for 10 s or more | He should reach his slot or leave it |
| 17 | `forward-idle` | A man forward for 20 s or more with no bout of his in that time | He stepped out to strike |
| 18 | `gap` | A standing front-rank man whose slot beside him exists and stays unfilled for more than 10 s, the leader's slot during a succession apart | A drilled file closes in two seconds |
| 19 | `leaderless` | A body with a living closer whose leader has been down for more than 20 s | The closer succeeds in five |
| 20 | `far` | A standing man more than 300 m from the field's centre, or any coordinate beyond 5 km | Walked off, or a number gone wrong |

**Two checks that are not visual,** run on the same battles:

| # | Name | It checks |
|---|---|---|
| 21 | `dormancy-check` | The first 300 s again with the runner's dormancy check on: a man the runner skips must have every hazard at zero. A failed assertion is a flag |
| 22 | `dormancy-equal` | The first 120 s again deciding for every man: the hash after each man step must equal the normal run's |

## How a flag is looked at

- **The tool prints counts** by detector, engine and variant, and writes each flag with its
  battle, time and men, and the positions of the men within 15 m for 3 s either side.
- **For each detector and engine, the first three flags** in the order of variant, rung, seed
  and time are rendered as frames and looked at. More are looked at when the three differ in
  kind, or when a count is far larger in one variant.
- **Paullus looks at the pairwise engine's flags, Varro at the field's.** Each brings the other
  every candidate with its frames.
- **Michal gets a page** with a picture and a caption for each flag looked at, and a link that
  opens the live viewer at that battle and time.

## How a flag is judged

Each flag looked at goes into one of three bins.

- **A defect:** the code does not do what its own comment, its document or an agreed rule says.
  It is judged against that text, and never against a pass band or an outcome. Both of us must
  agree, and Michal decides whether it goes into the re-freeze.
- **A gap:** the code does what was written, and it looks wrong. It goes on the list of
  candidates for a second round in [STATUS](../STATUS.md). Nothing is changed.
- **Fine:** the detector's threshold caught something ordinary. The threshold is not changed
  during the pass; the flag is recorded as fine with the reason.

**Each defect that goes into the re-freeze** is listed in the addendum as found after the
gate's verdict was known, with who found it and who wrote the fix. The fix is written by the
code's owner, reviewed by the other, checked by mutation, and carries a counter where one is
possible, as the standard-bearer fix does.

## What the pass cannot do

A detector finds only what someone thought to define. Michal's own watching found all three
faults so far, and it stays the part that finds new kinds. A new kind becomes a detector.

## Changes after review

**2026-11-11, Varro** (call 0001, turn 84), before the tool has been run on any battle. I know
the outcome named above, so these lines are written with that knowledge. Detectors 25 to 27
look for faults in code of mine: the frames, the fix ed6d363, and the field.

- **The code under the pass** is the re-freeze candidate: the frozen code with ed6d363 and the
  runner's fix. Detector 1 is also run once on the frozen code (pairwise, rung 0, `base`,
  seed 1), where it must flag the step back at about 29 s. A detector that misses the known
  fault is not one.
- **Every flag looked at is listed with its bin and the reason,** those judged fine too. Each
  of us reads the other's list before the pass closes.

| # | Name | It flags | Why it would be wrong |
|---|---|---|---|
| 23 | `slot-twice` | Two up men of one body hold the same slot at one man step | A slot is one man's place |
| 24 | `down-rises` | A man who was down is not down at a later man step | The fallen stay down |
| 25 | `slot-back` | At one man step, the slots of at least half of a body's slotted men move by the same vector to within 100 mm, and either the body's order is to hold and they move more than 300 mm, or its order is to advance, charge or pursue and they move more than 300 mm against its facing. A count, with the first three looked at | A frame gives ground, or shifts while halted, only when its standard-bearer is driven from his place (`frames.rs`, `anchor`). This counts how often, and catches a step back smaller than detector 1's |
| 26 | `stalled` | A body whose order is to advance, charge or pursue, with at least half of its men standing and no up enemy within 10 m of any of them, whose slots move less than 500 mm in 10 s | A led frame walks until the fronts are 4 m apart. With ed6d363 it stands while the standard-bearer steps up to a slot that moved under him, which should take about two seconds |
| 27 | `over-packed` | 40 or more up men in one 2 m cell (10 per m²) | The field says a press gains no ground past the packing density, 9 per m² in the nominal set ([priors-field](priors-field.md), F2). The margin of 1 allows for counting centres in a box |

**2026-11-11, Paullus** (call 0001, turn 85), before the tool has been run on any battle, and
with the same knowledge. Varro asked which detector would have found Michal's third fault, the
man who ran half a second after he was steady. None of 1 to 27 would: detector 14 counts six
changes of posture, and he made two. So it gets one of its own.

| # | Name | It flags | Why it would be wrong |
|---|---|---|---|
| 28 | `sudden-flight` | A man turns to flight less than 1 s after he was steady. A count, with the first three looked at | The code has no rule against it: giving ground and running are two transitions with nothing between them ([STATUS](../STATUS.md), candidate 8). So every flag is a gap and none a defect; the count says how common the thing Michal saw is |

## How the tool reads the list (Paullus, 2026-11-11)

Written with the tool (`viewer/src/detect.rs`, `cannae-qc`), before it was run on any battle. The
table above is not changed. These are the readings a table leaves open, and Varro reviews them
with the list.

**For every detector:**

- **A sample** is the state after a man step; samples are 250 ms apart. "Over 2 s" compares a
  sample with the one eight before it. A state that must last "D s or more" must hold at
  consecutive samples spanning D, and "more than D s" at samples spanning more than D. A
  movement that must last a second must be made in four consecutive steps.
- **A flag is an episode:** a condition holding for one subject (a man, a pair of men, a body, a
  2 m cell) at consecutive samples. It is written once, with its first and last time. A
  condition that breaks for one sample and returns is two flags.
- **A kind** is a mechanical sub-class of a flag, named below. It is never a judgement. The
  flags looked at are the first three of each kind for each detector and engine, which
  includes the first three of each detector and engine.
- **Positions are made when a flag is looked at,** by running its battle again
  (`cannae-qc clip`), and are not kept for every flag: a battle is the same battle every time.
- **The variants are taken in the batch's order:** `base`, `veterans`, `recruits`, `flank`,
  `shock`, `volleys`, `backed`, `relief`.

**For single detectors:**

| # | Reading | Kinds |
|---|---|---|
| 1 | A slot's position is the target the frames give the man, which includes the distance the frame leads its men while the body moves. The slotted men are those with a slot at that step, a fallen man whose file has not closed yet among them. The common vector is the median of the moved slots' vectors | `by-the-lead`: the slots moved straight ahead or back by the tie's lead, to within 100 mm, which is the lead coming on or going off. `other`: anything else |
| 2 | The order was to hold at every sample of the 2 s. "One direction" is any arc of 60 degrees | |
| 3 | In flight at every sample of the 2 s; the distance is between where he was and where he is | |
| 5 | | `both-standing`, `one-in-flight` |
| 6 | The cells are the squares of a 2 m grid through the origin | |
| 8 | Where he fell is where he lay at the first sample at which he was down | |
| 10 | The direction is to the nearest standing enemy within 10 m as both stood before the step | |
| 12 | The postures are those the bout phase read, before its own outcomes were applied. A wrapper around the resolver notes them; it passes everything through, and a test holds the battle to the scorer's own | `same-side`, `defender-down`, `attacker-in-flight`, `attacker-down` |
| 13 | The third man was up when the bout phase read him. The nearest such man is named | `past-a-comrade`, `past-an-enemy`, by the side of the attacker |
| 14 | Every change of posture counts, whatever caused it | `steady-and-forward`: every change in the 5 s was between steady and forward, the pulse of the fight. `gives-way`: any other |
| 15 | Formed: the standard is carried by a man who is standing | `front-rank`, `the-closer` |
| 16 | In his place as the frames say: within the tie's tolerance of his slot | |
| 17 | The 20 s count from his last bout, as attacker or defender, if he had one while forward | |
| 18 | A front-rank man is a man in a file with no slot ahead of him. The leader's slot during a succession: the empty slot is that of the body's leader, who is down and still kept in his file | `left`, `right` |
| 19 | The closer is the man posted behind the rear rank at the start. Living: not down | `closer-standing`, `closer-in-flight` |
| 20 | The field's centre is the mean of all the men's positions at the start | `standing`, `coordinate` |
| 21 | The run with the check on must also give the normal run's state after every man step | |
| 23 | The same slot: the two men's targets are equal to the millimetre | |
| 25 | As 1: a slot's position is its target. The vector is the one half the body's slots share, the larger if two halves each share one. Under an order to retire nothing is flagged | `by-the-lead`: as in 1, which here is the lead going off when a body that was advancing halts. `other`: anything else |
| 26 | The order was to advance, charge or pursue at every sample of the 10 s. The slots' movement is the median, over the men slotted at both ends, of how far each man's target moved. Half of its men: of the men it was set up with. The enemy within 10 m is of any of its standing men, at the end of the 10 s | |
| 27 | The same cells as 6 | |
| 28 | In flight at this sample and not at the last, and steady at one of the three samples before | |

**The tool's own tests** are on hand-made scenes: for each detector a case that must be flagged
and a case just under its threshold.

**2026-11-11, Varro, a second line** (call 0001, turn 86), still before the tool has been run
on any battle:

- **Detectors 25 and 26 in `backed` and `relief`:** the first flag of each kind in each of
  these two variants is looked at as well, on both engines. The relief is the one place that
  reads another body's lead, and ed6d363 adds a way for a body not to lead.
- **Detector 1 cannot see a warband step back one rank:** its ranks are 1,200 mm apart, and the
  detector asks for more than 1,200. Detector 25 covers that. No threshold is changed.

**2026-11-11, Paullus, after the scan** (call 0001, turn 89). Written with the table of counts
by detector, engine and variant seen, and nothing else: no flag's line and no picture.

- **`flank` is watched for 1,200 s.** Its band is ordered in at 600 s (`flank.toml`), the
  second the pass stopped watching. So the first scan saw the band stand, and every count of
  `flank` equalled `base`, which is how it was found. The 40 battles were run again for
  1,200 s and their flags replace the first run's. "600 simulated seconds each" was my
  oversight.
- **The picker also takes the first flag in the variant that has the most of a kind,** for
  every detector and engine. It is the mechanical reading of "more are looked at ... when a
  count is far larger in one variant". Without it nearly every pick is `base`, rung 0, seed 1.
- **Pictures made on the Pi** are turned into PNGs by a headless Chromium, from the same SVGs.

**2026-11-11, Varro** (call 0001, turn 90): yes to both changes. Neither touches a threshold,
and both were made on the counts alone.

**2026-11-11, Paullus, while looking.** I also looked at flags the rule does not pick, to see
what the large classes of detectors 1 and 25 are: I sorted their flags by order, distance and
body size, and traced a standard-bearer step by step in three battles. `tools/qc-render.py
draw --also FILE` draws such a flag. What that showed is in
[qc-results.md](qc-results.md).

**2026-11-11, Paullus, the page.**

- **The page the rule promises Michal is two pages.** [qc/pairwise.md](qc/pairwise.md) has
  every pairwise flag looked at with its four-panel picture, what the detector measured, the
  bin and the reason; `tools/qc-page.py` writes it from the table in
  [qc-results.md](qc-results.md). [qc/README.md](qc/README.md) shows one picture for each
  gap, cut down to one panel so that it reads on a phone (`tools/qc-crop.py`).
- **The pictures in the repository were drawn again after the looking.** The tool's legend
  ran under its scale bar; it is now on a row of its own. All 75 have the same panels as the
  ones I looked at, compared line for line.
- **The page's own comparison of the drawings** finds the same twelve repeats that my list
  marks with "=".

**2026-11-11, Paullus, on D1** (call 0001, turn 95). Varro's list has one candidate defect, in
the field engine: two enemies on their feet nearer than the guard when both count as in their
places ([qc-results-field.md](qc-results-field.md)). I read `field.rs`, `lib.rs`, F6 and the
step's code, and I agree that it is a defect.

- **The text states the guard without exception:** F6, "the nearest two enemies on their feet
  can stand", and `field.rs`, "no two enemies on their feet may stand closer".
- **The code skips the pair:** a man in his place leaves the step before the pass over
  positions, so a pair of whom both are in their places is never looked at.
- **The other reading does not hold.** "Any free man ... is handled by positions" says who is
  processed. It rests on two men in their places never being within a guard of each other,
  and the code lets that fail.
- **The first step of a flight is a defect under either reading.** The man is not moving by
  his tie to his slot; he counts as in his place only because the flag is a step old.

**2026-11-11, Michal's decisions, at about 15:40 UTC,** asked by Paullus with both
recommendations:

- **D1 is recorded and left out of the re-freeze.** Both recommended that.
- **Gap A stays in the new batch** and goes first on the second round's list. It was put to
  Michal by itself because it is the nearest to the defect they saw. Both recommended that.
- The speed question was put with these two; its answer is in
  [spike-pairwise.md](spike-pairwise.md).

**2026-11-11, Paullus, the candidate before the re-freeze** (`refreeze-candidate` at d35684c:
main merged in; its code is that of 9d96089, which the pass ran on).

- **Tests,** on the Pi: `cargo test --workspace` passes, 217 of 217. The QC tool's own 38
  detector tests and 6 same-battle tests pass.
- **A body on the march, traced step by step** (Varro asked for it; `cannae-qc clip`, the
  slots of every man of the body at each man step, on both engines, rung 0, seed 1):
  - **The warband's advance in `base`,** 0 to 2.75 s: all 160 slots move 325 mm at every
    step, the same for each, and every man counts as in his place throughout. At the halt
    they move 1,175 mm back once (the lead less a step), and the men stand on them.
  - **The flank band's charge in `flank`,** from 600 s: all 36 slots move 1,000 mm at every
    step, the same for each. 25 or 26 of the 36 men count as in their places while they
    run; the slowest runs 4 m behind his slot and keeps that distance.
  - **No step moves a slot by a rank,** and no slot moves apart from the others.

**2026-11-11, Paullus, the field half of the page.** [qc/field.md](qc/field.md) has every one
of the 95 field flags with its picture, Varro's bin and his reason, copied from his table by
the same tool. Its pictures were drawn again with the legend corrected; all 95 have the same
panels as the ones he looked at. The eleven flags his table marks as the same drawing as
another are the eleven the files show. The phone page has a section from his list: D1 and N
with a cut picture each, and A as it is on his engine, O, P and R in words.
