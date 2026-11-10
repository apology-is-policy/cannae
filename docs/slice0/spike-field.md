# Slice 0: The speed spike for the field engine (S0.7)

**Status:** Varro, 2026-11-10. **Burst readings only.** The sustained readings are not taken
yet: another project loaded the machine (load average 12 to 37) from about 13:30 UTC, and no
reading taken under that load is reported here as a result. This file will be completed when
the machine is quiet.

The kill line for this engine is 10x real time per core in the pocket
([rival-comparison.md](../rival-comparison.md)).

## Where it stands

The pocket: 60,000 men packed in a disc, 90,000 around them walking in and pressing. Every one
of the 150,000 is out of his place, so nothing here is moved by the cheap rule for formed men.
One core, bursts of 14 to 60 s.

| What is running | Real time | Against 10x |
|---|---|---|
| Substrate alone, enemies kept apart by position (`Contact::Line`) | **15.2x** (24 s) | above |
| The same with perception: a View for every man with an enemy in his window, about 39,000 a step | **5.3x** (14 s), before the View was reworked | **below** |
| Substrate alone with contact inside crowds (`Contact::Crowd`) | **1.6x** (60 s; 1.1-1.2x once the ring has closed) | **far below** |

**As it stands the engine does not meet its kill line in the pocket.** The substrate alone
does. Perception takes it under, and the man's hazards for those 39,000 men come on top and
are not in any figure here. The reworked View has no clean reading yet.

The other two scenes, read once before any tuning (commit e54c427, 25 s): the line ran at 31x.
The surge has not been run.

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
and 40 friends. It still gives exactly the reference's Views. **Its speed is not measured:**
every reading after the change was taken under the other project's load.

## Contact inside crowds

`Contact::Crowd` was meant to be cheap because "the packed interior of a pocket hardly moves"
([rival/02](../rival/02-order-and-contact.md) §5). In this pocket that is false. The ring is
90,000 men pressing, each with room to gain a little, so each is "crowded and moving". With
60,000 to 90,000 men in contact:

- the step costs what a pairwise engine costs for those men: 1.1 to 1.2x real time;
- the densest cell rises to 12.9 per m², past the packing density of 9 that the field keeps to
  without contact. Men moved by position are pushed onto ground the field would not let them
  walk onto.

Counting a man as moving only by his own walk and lean, and not by the crowd's push, changed
neither figure. This rung fails here on speed and on physics, and it is not fixed. See
[rival/08](../rival/08-the-field-as-built.md), "What it does badly".

## Still to do

1. Sustained readings, ten minutes each, one core: line, surge and pocket, substrate alone and
   with perception; the pocket with `Contact::Crowd`; the same hash on a repeat.
2. The man's hazards on top, once both engines have a full man to time.
3. A decision on contact inside crowds, after the passage study.
