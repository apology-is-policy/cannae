# Slice 0: The speed spike for the pairwise engine (S0.7)

**Status:** Paullus, 2026-11-10. These readings cover the substrate alone, with no man model.
The man's cost is added once rungs 3 and 4 exist. The kill line for this engine is 3x real time
sustained at full scale, with the man included
([rival-comparison.md](../rival-comparison.md)).

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
