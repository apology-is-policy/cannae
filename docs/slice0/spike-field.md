# Slice 0: The speed spike for the field engine (S0.7)

**Status:** Varro, 2026-11-11. **The sustained readings are taken, and the engine fails its
kill line.** In the pocket, with perception, one core ran at **1.43x** real time. The line is
10x. Without perception the pocket ran at 5.32x, which is under the line too.

The kill line: the rival dies if "it runs below 10× real time per core in the pocket"
([rival/06](../rival/06-experiment.md) §4, [rival-comparison.md](../rival-comparison.md)). By
the rule under "How a sustained reading is read", a reading under the line is final: the man's
hazards are not in these figures and only add cost. What follows from it is Michal's to decide.

## Where it stands

150,000 men, one core, ten minutes of wall time for each scene, read on 2026-11-11 by the rule
below. The figure of record is the mean over the last five minutes.

| Scene | Figure of record | Whole run | Slowest minute | Against 10x |
|---|---|---|---|---|
| **Pocket with perception** (the run the line is read on) | **1.43x** | 1.92x | 1.32x, the last reported | **failed** |
| Pocket, substrate alone (`Contact::Line`) | 5.32x | 6.77x | 5.25x | below |
| Pocket, contact inside crowds (`Contact::Crowd`) | 1.97x | 1.99x | 1.86x | far below |
| Line, substrate alone | 31.77x | 31.83x | 31.36x | (not the line's scene) |
| Surge, substrate alone | 13.28x | 13.30x | 13.15x | (not the line's scene) |

Four things the readings show:

- **The pocket fails without perception too.** The substrate alone settles at 5.3x once about
  125,000 of the 150,000 men are in the position solve. The burst figure of 15.2x was the
  first seconds of the scene.
- **Perception is the larger cost, and it was still rising.** It took 85 % of the wall time in
  the last five minutes: 79,000 Views a man step at 1.76 µs each. The run's last minute was its
  slowest, so ten more minutes would have read lower.
- **It is the scene and not heat.** The line and the surge ran flat for ten minutes, within
  5 %. One core does not throttle this machine measurably. The pocket slows because more men
  press.
- **The field does not keep to its packing density.** In the pocket without contact the
  densest cell stood at 9.5 to 9.8 men per m², with a peak of 10.0, against a packing density
  of 9.

## The sustained readings in detail

### Conditions

- **When:** 2026-11-11, 07:33 to 08:35 UTC, after a restart of the machine at 06:53 UTC. Two
  agent sessions were up (Paullus's and mine); Michal's other agents were halted.
- **What ran:** `spike_field`, release with debug info, built 2026-11-10 17:48 UTC from
  `cannae-field` and `cannae-core` as they are in the freeze (dd9d90a); sha256 `4ab05d5c998e76c4`.
  The field's parameters are the centre of every prior. One thread, `--wall-s 600`, a report
  every 60 s.
- **The load,** 1-minute average, logged just before and just after each run:

| Run | Before | After |
|---|---|---|
| Pocket with perception | 1.87 | 2.86 |
| Pocket, substrate alone | 1.95 | 2.66 |
| Pocket, `Contact::Crowd` | 1.99 | 2.53 |
| Line | 1.93 | 2.79 |
| Surge | 1.94 | 2.83 |

- **All five count by the rule** (under 2 before, under 3 after). But the rule expected a load
  near 1 after a one-core run, and it was 2.5 to 2.9: other processes used a core and a half or
  more throughout. Two are known. Paullus made a three-second build at 07:33:20, in the first
  minute of the first run and outside its last five. He saw `fileproviderd`, the iCloud daemon,
  at 89 % of a core at 07:33.
- **What that load can have cost.** A quieter machine could only read higher. The line's
  figure equals its burst figure of 2026-11-10 (31x), and the line and the surge are flat, so I
  see no sign that the load slowed the spike's thread. The verdict does not rest on that: it
  has a margin of seven times with perception and nearly two times without.

### The pocket with perception

The rate by minute: 3.35, 2.63, 2.26, 1.95, 1.86, 1.62, 1.54, 1.41, 1.32. It reached 1,152
simulated seconds.

- **The scene at the end:** 57,000 men in the position solve and rising by about 3,700 a
  minute; densest cell 9.5 per m² (peak 9.8). The ring was still closing.
- **The perception, per man step, after the report at five minutes:** rebuilding it 1.9 ms, the
  dormancy test for all 150,000 men 2.8 ms, the Views 139.5 ms. That is 85 % of the wall time.
- **Why it is far more than the burst showed.** The burst counted 39,000 Views a step at about
  1 µs. Here there were 79,000 at 1.76 µs: as the ring closes, twice as many men have an enemy
  in their window, and each View has more men to look through.
- **Taken apart:** the same intervals without the perception's time would have run at 9.6x, and
  the perception alone would allow 1.7x. Neither is over the line.

This run was measured in an easier state of the scene than the next one. It got a quarter as
far in simulated time, with less than half as many men pressing.

### The pocket, substrate alone

The rate by minute: 13.32, 9.04, 7.12, 5.96, 5.61, 5.62, 5.25, 5.29, 5.29. It reached 4,061
simulated seconds, and from the fifth minute it is level.

- **The scene at the end:** 124,000 to 129,000 men in the position solve; densest cell 9.6 to
  9.8 per m² (peak 10.0).
- **Against the burst:** 15.2x over 24 s on 2026-11-10 was the first minute's rate. The
  sentence that stood here, "the substrate alone does" meet the line, was true of the opening
  and is false of the pocket once it has closed.

### The pocket with contact inside crowds

Level at 1.86 to 2.08 by minute. 76,000 to 83,000 men in contact; densest cell 12 per m², peak
15.4. The physics fault recorded below stands, and is larger over ten minutes than the burst
showed (12.9).

### The line and the surge

- **Line:** 31.4 to 32.8 by minute; about 8,000 men in the position solve; densest 2.2 per m².
- **Surge:** 13.2 to 13.6 by minute; 44,000 men in the position solve; densest 7.2 per m².
  This is its first reading.

### The same hash on a repeat

Two runs of 300 simulated seconds of the pocket with perception gave the same final hash
(`7f53bf4b9599fa9e`), the same 70,123 crush events and the same 53,961,116 Views, in 100 s and
97 s. These two are not timings of record: the load after the second read 3.95.

### The raw outputs

In [readings/field-2026-11-11/](readings/field-2026-11-11/): the seven runs' outputs as
printed, `log.txt` with each command line and the load before and after, and `read_spike.py`,
which reads an output by the rule and was written before any of them existed. The times in
`log.txt` are local, one hour ahead of UTC.

### What was not tried

Nothing was tuned after these readings, and nothing will be read again as this line: that is
the rule Paullus and I agreed for both engines. A cheaper perception may exist. It would not
change the verdict alone, because the substrate without any perception is under the line.

## Burst readings (2026-11-10, superseded)

Kept as they were written, because the profile and the tuning below refer to them. They were
read at commit 09e4326, in bursts of 14 to 60 s, one core. Since then the substrate looks
farther for enemies and reads the field for men in their places where a tile is thick (commit
9651991, two defects found in review); the sustained readings above include that work.

The pocket: 60,000 men packed in a disc, 90,000 around them walking in and pressing. Every one
of the 150,000 is out of his place, so nothing here is moved by the cheap rule for formed men.

| What is running | Real time | Against 10x |
|---|---|---|
| Substrate alone, enemies kept apart by position (`Contact::Line`) | 15.2x (24 s) | above |
| The same with perception: a View for every man with an enemy in his window, about 39,000 a step | 5.3x (14 s), before the View was reworked | below |
| Substrate alone with contact inside crowds (`Contact::Crowd`) | 1.6x (60 s; 1.1-1.2x once the ring has closed) | far below |

The line, read once before any tuning (commit e54c427, 25 s), ran at 31x.

## Method

- **Machine:** MacBook Air M2 (Mac14,2), 8 GB, fanless.
- **Build:** release with debug info.
- **Runs:** `cargo run --release -p cannae-field --example spike_field -- <line|surge|pocket>
  [--contact line|crowd|all] [--perceive] [--seconds S] [--wall-s W]`. One thread. The scenes
  are the pairwise spike's, body for body ([spike-pairwise.md](spike-pairwise.md)).
- **What a burst is worth.** The pairwise engine's burst figures were about twice its
  sustained ones on four cores. One core heats the machine less, but these figures should be
  expected to fall.
- **No man model.** As in the pairwise spike. `--perceive` rebuilds the perception at every
  man step, runs the dormancy test for every man, and builds a full View for each man with an
  enemy in his window. It does not evaluate hazards.
- **Crush events are counted and not applied,** so no one ever leaves the crush. In a battle
  the disabled drop out.

## What the profile showed

Before tuning, the pocket ran at 9.4x. I had guessed the time went to the position solve at
the interface. It did not:

| Part of the step | Share |
|---|---|
| Walking steps for 150,000 men | 32 % |
| Listing the men each man might touch | 25 % |
| Moving, and the crush draws | 20 % |
| The passes of the position solve | 11 % |
| Deposit | 8 % |

A man at the interface was moved clear of an enemy 0.17 times a pass: the interface is
feasible, because no one walks or leans onto ground the enemy holds.

What was changed (commit 09e4326):
- The crush chance came from a 128-bit division and an exponential for every crushed man at
  every step. It is now a table by density.
- Each cell's men are listed by side, and a man who touches only enemies scans only enemies.
  The men looked at per man fell from 72 to 13.
- Men with no one near are skipped in the passes. The lean is computed only where it gains
  ground. Velocity is a multiplication.

That took the substrate from 9.4x to 15.2x. Per step it now spends about 7 ms on walking
steps, 3 ms on lists, 3 ms on passes, 2 ms on the deposit and 2 ms on moving.

## Perception

With perception on, a step in the pocket cost about 50 ms, of which the Views were about 39 ms:
39,000 Views at about 1 µs each. Rebuilding the perception (1.3 ms) and the dormancy test for
all 150,000 men (2.3 ms) are cheap.

The View was reworked in the same commit: men are listed by quarter cell and side, each cell
records which quarters hold whom, and the eight nearest enemies are sought only in quarters
that hold enemies, ring by ring outward. A count showed a View then looks at about 12 enemies
and 40 friends. It still gives exactly the reference's Views. Its speed was not measured
in a burst: every reading after the change was taken under the other project's load.
**2026-11-11:** sustained, in the closing pocket, a View cost 1.76 µs and there were 79,000 a
step (above).

## Contact inside crowds

`Contact::Crowd` was meant to be cheap because "the packed interior of a pocket hardly moves"
([rival/02](../rival/02-order-and-contact.md) §5). In this pocket that is false. The ring is
90,000 men pressing, each with room to gain a little, so each is "crowded and moving". With
60,000 to 90,000 men in contact:

- the step costs what a pairwise engine costs for those men: 1.1 to 1.2x real time;
- the densest cell rises to 12.9 per m², past the packing density of 9 that the field keeps to
  without contact. Men moved by position are pushed onto ground the field would not let them
  walk onto.

**2026-11-11:** over ten minutes the field went past 9 without contact too: 9.5 to 9.8 in the
densest cell, peak 10.0 (the sustained readings above). The fault is smaller there, and it is
the same kind.

Counting a man as moving only by his own walk and lean, and not by the crowd's push, changed
neither figure. This rung fails here on speed and on physics, and it is not fixed. See
[rival/08](../rival/08-the-field-as-built.md), "What it does badly".

## How a sustained reading is read

Agreed with Paullus on 2026-11-10 (call 0001, turns 55 to 59), and written here before any
sustained reading of either engine was taken. His spike follows the same rule.

- **A run** is ten minutes of wall time. The 1-minute load average is logged just before it and
  must be under 2, or the run is not started.
- **The figure of record** is the mean rate over the last five minutes of wall time: the
  simulated time from the first report at or after five minutes to the end of the run, divided
  by the wall time between them. The machine has no fan, and it throttles during the first
  minutes.
- **Reported beside it:** the mean of the whole run, and the slowest interval between two
  reports, each with the simulated time it covers. A faster engine is further into the scene,
  and the reader should see how far.
- **The state of the scene:** for the pocket, whether the ring had closed, read from the count of
  men in the position solve (`hot`) and the densest cell in the last reports.
- **The load after a run** is logged too. One core leaves it near 1. A run that ends with it at
  3 or more is reported, and is not the figure of record.

**What a reading can decide.** Neither spike runs the man: his table, his hazards and rung 3's
gauges are shared code, and their cost is not in any figure of either engine. It will be
measured later with one harness, the same way for both. Until then:

- a reading **under** the kill line is final. The man only adds cost;
- a reading **over** it means "not failed yet".

**The perception is reported apart from the substrate.** It is common ground: the core fixes
the View, and the tests hold any perception to it. The reports print its three costs
(rebuild, the dormancy test, the Views) beside the total. This engine's line is still read with
the perception on, because an engine needs eyes.

## Still to do

1. ~~Sustained readings.~~ Done on 2026-11-11, above. The line and the surge with perception
   were left out, as planned; the pocket decides.
2. The man's cost on top, with the shared harness. It can no longer change this engine's
   verdict. It is still wanted for the pairwise engine's line, which is read with my
   perception.
3. Contact inside crowds, and the field's overshoot of its packing density, are open faults of
   an engine that has failed its line. Whether either is worth fixing depends on what Michal
   decides about the engine.
