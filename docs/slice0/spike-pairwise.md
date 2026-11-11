# Slice 0: The speed spike for the pairwise engine (S0.7)

**Status:** Paullus, 2026-11-11. **The engine is under its speed line as read: 2.67x against
3x over a whole battle,** sustained on eight threads, before the man is added (the next
section). The readings below that cover the substrate alone, with no man model. The kill line
for this engine is 3x real time sustained at full scale, with the man included
([rival-comparison.md](../rival-comparison.md)).

## The sustained readings of record (2026-11-11)

Taken by the rule in [spike-field.md](spike-field.md) ("How a sustained reading is read"), with
`tools/s07-readings-pairwise.sh` on the spike at 91185c4: 150,000 men, eight threads, Varro's
perception, ten minutes each. The raw reports are in
[readings/pairwise-2026-11-11](readings/pairwise-2026-11-11/).

| Run | Second half, the figure of record | Whole run | Slowest report | Load before, after |
|---|---|---|---|---|
| **Pocket,** 08:39:25 to 08:49:25 UTC | **1.59x** | 2.00x | 1.34x, the last | 1.89, 5.65 |
| **Line,** 08:52:06 to 09:02:06 UTC | **8.42x** | 8.57x | 6.24x, the last | 1.97, 7.37 |

- **Whole battle, 2 / (1/line + 1/pocket): 2.67x.** The line is 3x.
- **Neither run is void:** the load after each is under 10.
- **The man is not in it,** and he only adds cost. So this engine does not reach its line on
  the Air as measured. The pocket alone is also under the vision's "toward 2x".
- **The pocket slows as it fills:** 3.19x in the first half minute, 1.34x in the last, with
  the men touching rising from 116,000 to 147,000.
- **Two thirds of the pocket's time is the perception.** The spike runs Varro's perception on
  one thread, as his own spike does, and times it. In the pocket's last five minutes it took
  67 % of the wall time, the views alone 64 %; in the line's, 46 %. Without it the same runs
  would read about 4.8x and 15.7x, and a whole battle about 7.4x. In Varro's pocket the
  perception took 85 %. So one shared component, run on one thread in both spikes, is the
  larger part of what puts each engine under its line. The rule reads the figure with the
  perception in it, and so the figure stands; what to do about the component is a separate
  question, and Michal should see this before deciding anything about either engine.

**What could have touched the two runs.** Logged, not pleaded:

- **Something held the Mac's load near 1.7 to 1.9 with nothing of ours running:** 2.20 at
  08:37:50 UTC, 1.89 at 08:39:25, and 1.97 at 08:52:06 after 161 s without a run. At 08:34 I
  had seen the system's `dasd` near a full core. I did not record what it was during the runs.
- **The line's last minute read 7.30x and 6.24x** after 510 s at 8.7x to 9.0x. I do not know
  why. Without that minute the line is about 8.9x and the whole battle 2.70x.
- **Mine during the runs:** ssh and scp to the Pi, and file edits. No build and no test.

**What did not run.** The Mac went down at about 09:13 UTC, in the third run, and was back at
12:14. The rout was cut off after eight minutes, and the four-thread pocket and the pocket with
the reference perception never started. None of them enters the figure.

**A second reading would be a second line.** The rule says nothing is read again as the same
line. Varro and I put one joint question to Michal, with both verdicts side by side, and no
second reading of either engine is taken before they answer. If one is allowed, its conditions
and which figure decides are written before it is taken, and both figures are reported.

## Verdict so far

| Setup | 1 core | 4 cores, 10 min sustained | 8 cores, 3 min, machine hot |
|---|---|---|---|
| **Line:** two armies of 75,000, 1,000 files by 75 ranks, pressed together along 1.5 km | 8.0x | **12.1x** | 14.8x |
| **Surge:** the same line, every man re-targeted 3 m ahead at each man step | 4.3x | **5.3x** | 7.3x |
| **Pocket:** 60,000 men in a disc, 90,000 around them walking in and leaning with full weight | 1.9x | **2.3x** | 3.8x |

- The line and the surge leave room for the man.
- The pocket does not, on 4 cores. It is the worst case: every man touches someone, the ring
  presses forever, and nobody ever leaves, because the spike does not apply crush events. In a
  battle, crushed and fallen men drop out of the contact search. Even so, the pocket is the
  engine's main risk against its kill line, and the place to spend effort next (below).
- The burst readings at 6 s are much higher (line 14x, surge 11x and pocket 5x, on 4 cores).
  The fanless Air throttles within a few minutes, and the sustained figures are the ones that
  count.

## Which thread count the line is read on (corrected 2026-11-10)

**The line is read on all eight of the Air's cores.** I had been treating the four-thread column
above as the one that counts, and I had set my next readings and the full-scale bench to four
threads. That was habit, not the line.

- **The kill criterion names no thread count:** "below 3x sustained at full scale"
  ([rival-comparison.md](../rival-comparison.md), the kill criteria).
- **The experiment's plan says how it is measured:** "engine B on one core; engine A on all
  cores as designed, reported in total and per core" ([rival 06](../rival/06-experiment.md),
  section 3). Engine A is this one.
- **The estimate the line was cut from counts every core.** The comparison says "We expect
  3-7x on all cores, and our kill line is 3x", and [06](../vision/06-architecture.md) derives
  the 3-7x from the cycles of the performance and the efficiency cores. All three were written
  before anything was measured.
- **What I had seen when I corrected it.** The table above: the pocket at 2.3x on four threads
  over ten minutes, and at 3.8x on eight threads over three minutes with the machine hot. So
  the correction is in this engine's favour, and I made it knowing numbers on both sides of
  it. I told Varro so. He read the texts himself and agreed that eight threads is the line as
  written (call 0001, turns 69-70).
- **From now on:** the figure of record is taken on eight threads, sustained, by the rule in
  [spike-field.md](spike-field.md) ("How a sustained reading is read"). Four threads are read
  beside it and printed. A run is void if the load after it is 10 or more on eight threads, or
  6 or more on four: the threads plus 2.
- **No sustained eight-thread reading exists yet.** The 3.8x above ran for three minutes, not
  ten, and without perception. It decides nothing.

## What the line is read over: a whole battle (Michal, 2026-11-10)

**The 3x is read over a whole battle, not scene by scene.** Michal decided it, with Varro's
reading and mine put side by side, before any reading of record was taken.

- **Why it was a question.** The criterion names no scene: "it cannot reach 3x real time
  sustained at full scale on the Air. Below that, every A1 batch needs a rented machine"
  ([rival 06](../rival/06-experiment.md), section 4). The estimate it was cut from is a figure
  over a whole battle, and it already has the pocket near 2x: "About 3-7x real time over a whole
  battle at full Cannae scale, falling toward 2x in the encirclement"
  ([06](../vision/06-architecture.md), the performance table). Varro's line names the pocket;
  this one does not.
- **What I had been doing.** Reading it scene by scene: the verdict above calls the pocket "the
  engine's main risk against its kill line", and I had proposed to Varro that either scene
  under 3x be final. I found the vision's sentence after that.
- **Whose favour it is in.** Mine, and I had seen the pocket at 2.3x and 3.8x. So I did not
  settle it. Varro, who wrote the criterion's sentence, read it as a whole battle and proposed
  the weighting below, knowing the same figures. The other choices put to Michal were scene by
  scene, and a whole battle with a floor of 2x on the pocket.
- **The figure of record.** No whole battle exists at full scale in Slice 0. So the line scene
  and the pocket count at equal shares of simulated time:

      2 / (1/line + 1/pocket)

  Each scene's figure is its sustained mean by the reading rule, on eight threads, with
  Varro's perception. Deployment and the approach are left out, which counts against this
  engine. The pocket's own figure is printed beside it, against the vision's "toward 2x".
- **In the full-scale bench** the figure of record is the same mean over the two scenes, each
  from its first bout to its end, and the slowest simulated minute is printed beside it.
  Varro's line stays the pocket's slowest minute, as his line names the pocket.

## Method

- **Machine:** MacBook Air M2 (Mac14,2), 8 GB, fanless. Four performance and four efficiency
  cores.
- **Build:** release with debug info (`[profile.release] debug = 1`).
- **Runs:** `crates/cannae-pairwise/examples/spike.rs`. Each setup ran 10 minutes of wall time
  on 4 threads, back to back, then 3 minutes each on 1 and on 8 threads with the machine hot.
  Rates were logged every minute. 20 Hz base tick, two relaxation passes per tick, placeholder
  parameters.
- **Disturbance:** Varro's session ran one one-second build at about 11:22 UTC, in the surge's
  third minute on 4 threads. That minute may read low; the 10-minute average barely moves.
- **Determinism:** every rewrite below was checked hash for hash against the version before
  it, and across 1, 4 and 8 threads, until the physics changed on purpose (see "Physics fixed").

## What it took

| Version | Line, 1 core (burst) | Pocket, 1 core (burst) | What changed |
|---|---|---|---|
| Spatial hash | 0.45x | | Hash map from cell to men |
| Dense grid | 2.5x | 2.4x | A sorted, row-major grid with a half-stencil (1cce715) |
| Sorted list | 9.9x | 3.8x | No per-cell array, a radix sort, then re-sorting the previous order by insertion; even and odd runs writing into one correction list; a persistent rayon pool (e6d2195) |

**Where the time went.** Phase timings, taken with temporary instrumentation, overturned the
first profile. In the dense grid, building the grid cost more than the contact search. That
meant clearing, copying and prefix-summing an array of 855,000 cells for 150,000 men, twice a
tick. The contact search itself spent most of its time walking empty cells. With 8 threads,
summing per-thread buffers cost as much as the work they held. Spawning threads for every
pass cost another 15-20 %.

## Physics fixed by the sustained run

The first 10-minute pocket run fell from 1.75x to 0.8x. A density probe showed why: the ring did
not compress the inner crowd, it walked through it. The inner disc stayed at 2.8 men per m²
while the ring piled up at its centre at 64 men per m², with a median summed overlap of 5 m.
That is impossible. Two relaxation passes per tick cannot hold back a crowd walking 65 mm a
tick into men who are only bracing.

Three rules fixed it. All are physical, and all are in `crates/cannae-pairwise`:

1. **A man cannot walk into the men he touches ahead of him.** His step loses its component
   toward them (their summed direction, from the last pass), so he slides along them or stands.
2. **His lean fades as he presses to the bone.** The lean shrinks as his summed overlap with
   the men ahead, in the direction he faces, approaches `core_mm`.
3. **Bodies have a hard core.** Overlap with one neighbour beyond `core_mm` is removed in full
   at each pass, not by the stiffness share.

After the fix the pressed edge of the pocket settles at about 5 men per m², the onset of danger
in the crowd-safety literature. The outer radius stops shrinking, and overlaps stay physical.
A test pins it: a ring leaning with full weight on a standing crowd may squeeze it toward 6.9
men per m², but cannot get into its middle (`a_pressing_ring_cannot_walk_through_a_standing_crowd`).
The crush draw also moved to the man's 4 Hz step, over the whole step. That is the same hazard
at a fifth of the cost.

## Not measured yet

- **The man model.** Rungs 3 and 4 (S0.6) on Varro's visible layer. Most men are dormant
  most of the time, but the pocket is where they are not.
- **The rout through camp gates.** It needs walls, which come with the passage study (S0.8).
- **Varro's field substrate.** That is his spike, against his kill line of 10x per core in the
  pocket.

## Next for the pocket, if it decides the verdict

- **Re-sort.** In the pocket the insertion sort falls back to a full sort once a tick, because
  ring men cross rows as they creep in. Merging the few men who moved into the sorted rest
  would be linear.
- **Sleep.** Men whose contacts and positions have not changed for a few ticks need not be
  relaxed. Physics engines call these sleeping islands, and their wake rule would have to be
  exact, as dormancy is for the man.
- **Apply crush events.** That is not a speed-up but the battle as it would be: crushed men go
  down, and the pocket thins.
