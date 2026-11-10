# Slice 0: Priors for the Field Engine and Rungs 0 to 2

**Status:** a draft by Varro, 2026-11-10, for Paullus to read and attack. It is frozen with the
[pre-registration](preregistration.md), by commit and SHA-256, once both have signed.

This file lists every free parameter that is this engine's own, with the distribution it is
drawn from and the reason. Parameters of the shared core (the tie, bouts, kit, breath, power,
gait) are in [priors-shared.md](priors-shared.md) and are drawn once for both engines. What the
man does with these numbers is in [rival/07](../rival/07-hazards.md).

**What I had seen when I wrote this.** No run of the patch with these men. The only runs made
before this file was written were the unit tests of `cannae-man` (one man, one hand-built view
at a time) and a three-minute clash of 24 men a side whose troops were invented for the test.
The centres below come from the arithmetic in the "Reason" column, and from nothing else.
After it was written I ran the first 30 seconds of the base patch, and 5 seconds of each
variant, to check that the files load and the men march. I looked at the event count and the
hash, and at nothing else.

## How a parameter set is drawn

- **U[a, b]** is uniform between a and b. **L[a, b]** is log-uniform: the logarithm is uniform.
- Every draw is independent of the others unless the line says "derived".
- Weights are in units (§1 of rival/07) and are stored in Q8, rounded to the nearest 1/256.
- Draw *i* of the 200 uses the same values on every rung: rung 1 is rung 0 plus line 30, and
  rung 2 is rung 1 plus lines 31 and 32. Paullus's rungs 3 and 4 read lines 1 to 29 unchanged.

## 1. The man

Alarm is in turned men: a man of his own side, turned, in view, is 1.

| # | Field | What it is | Prior | Centre | Reason |
|---|---|---|---|---|---|
| 1 | `back` | A man of his side giving ground | U[0.2, 0.5] | 0.35 | A backward step is a weaker sign than a turned back, but the same kind. No source gives the ratio. |
| 2 | `fall` | A man of his side going down, for the one step in which it happens | U[1, 3] | 2 | Worth more than a man who runs, but it lasts a quarter of a second, so it only moves men already near the edge |
| 3 | `enemy_flank` | An enemy in reach beside him, facing him | U[1, 2.5] | 1.5 | His shield cannot face two ways |
| 4 | `enemy_rear` | An enemy in reach behind him | U[2, 4] | 3 | Attack from behind is what the sources say breaks bodies outright (Livy 22.48). With a nerve of 2, one such enemy should put a supported man near the edge and an unsupported one over it. |
| 5 | `enemy_front` | An enemy in reach in front who has stepped forward | U[0.25, 0.75] | 0.5 | Facing an attack is what a line is for. Three attackers together should not move a man of average nerve who has his rank beside him. |
| 6 | `gap` | An empty slot beside him | U[0.25, 1] | 0.5 | His open side is uncovered until the file closes |
| 7 | `standard_down` | His standard down or carried off | U[1, 3] | 2 | *Signa* are what a body is ([rival/02](../rival/02-order-and-contact.md) §1) |
| 8 | `leader_down` | His leader down within 20 m | U[0.75, 2.5] | 1.5 | Caesar's centurion losses and what followed them (*BC* 3.71, *BG* 7.51); less than the standard, because a drilled body replaces him |
| 9 | `beside` | Each man standing beside or ahead of him, to 6 | U[0.15, 0.35] | 0.25 | Six such men are then worth 0.9 to 2.1 turned men |
| 10 | `behind` | Each man standing behind him, to 6 | U[0.15, 0.35] | 0.25 | Depth holds a line; the same range as rank mates, for want of anything better |
| 11 | `leader` | How many men his leader counts for, in what he is seen to do | U[2, 5] | 3 | One man who is watched by eighty |
| 12 | `hampered` | Nerve lost to a wound that hampers him | U[0.5, 1.5] | 1 | |
| 13 | `k_alarm` | Doublings of the hazard per unit of alarm | U[2, 3] | 2.5 | One more turned man in view should multiply the hazard of giving ground by 4 to 8: a threshold with a little chance around it, not a slope |
| 14 | `steady_again` | How far under the edge his alarm must be before a man giving ground stands again | U[3, 5] | 4 | A steady interior man sits about 5.75 under: he recovers at once when all is calm, and slowly while anything is wrong |
| 15 | `turn` | How much more alarm it takes to turn than to give ground | U[−2, 0] | −1 | The backward step is the beginning of flight (du Picq, from memory) |
| 16 | `room` | What open ground behind him is worth to a man about to turn | U[0.75, 2.5] | 1.5 | Rear-first collapse: [rival/01](../rival/01-the-man.md) §3 |
| 17 | `closer` | What a closer standing behind him is worth against turning | U[0.75, 2.5] | 1.5 | The optio's staff. Centred to cancel `room`. |
| 18 | `rally` | How far under the edge before a turned man stops | U[2, 4] | 3 | Alone, with a nerve of 2, he does not stop; among six men who stand, he does |

Urge is in forward men: a man of his own side, forward, in view, is 1.

| # | Field | What it is | Prior | Centre | Reason |
|---|---|---|---|---|---|
| 19 | `enemy_turned` | An enemy ahead with his back turned, to 6 | U[1, 2.5] | 1.5 | Forward panic ([rival/02](../rival/02-order-and-contact.md) §10): three backs should bring an ordinary man out |
| 20 | `enemy_back` | An enemy ahead giving ground, to 6 | U[0.25, 0.75] | 0.5 | |
| 21 | `charge` | An order to charge or pursue | U[5, 8] | 6 | Ordered, a man of average ardour should go within a second or two |
| 22 | `own_unsteady` | Each man of his side giving ground or turned, to 6, against going forward | U[0.5, 1.5] | 1 | |
| 23 | `breath` | Each level of being winded, against going forward | U[1, 3] | 2 | With half his breath gone, a man in a flurry of five behind his leader should be at the threshold of stepping back. How many seconds that takes is the shared cost of fighting. |
| 24 | `k_urge` | Doublings of the hazard per unit of urge | U[2, 4] | 3 | When a leader goes, the men who see him should follow in seconds |
| 25 | `go` | The urge at which a steady man steps out at the edge rate | derived | 4 | **Derived from an interval.** Draw T from L[300 s, 1200 s]: how often a centurion of the hastati steps out unprompted with the enemy standing before him. Then `go` = his ardour (line T5 below) + log2(T / 16 s) / `k_urge`. A formed fight has room for a flurry per stretch of front every 10 to 40 minutes. |
| 26 | `stay` | How far under `go` his urge must fall before a forward man steps back | U[0.5, 1.5] | 1 | |
| 27 | `react` | The fastest a man changes posture | L[1, 4] per second | 2 | Reaction time of a quarter of a second to a second |

How far he moves:

| # | Field | What it is | Prior | Centre | Reason |
|---|---|---|---|---|---|
| 28 | `step_back_mm` | How far behind his slot a man giving ground stands | U[0.6, 1.5] m | 1.0 | A pace or two |
| 29 | `stand_off_mm` | How near a forward man closes | U[0.8, 1.2] m | 1.0 | Inside a sword's reach of 1.5 m |

The marks:

| # | Field | Rung | What it is | Prior | Centre | Reason |
|---|---|---|---|---|---|---|
| 30 | `has_fled` | 1, 2 | Nerve lost for good by a man who has turned his back | U[0.5, 2] | 1 | Rallied troops are less reliable than fresh ones. No source gives a size. |
| 31 | `shaken` | 2 | Nerve lost per level of *shaken* (three levels at most) | U[0.4, 1.2] | 0.75 | At three levels a supported man of average nerve should be within reach of the edge |
| 32 | `shaken_wear` | 2 | How long one level takes to wear off | L[2, 10] min | 5 | "Minutes" ([rival/01](../rival/01-the-man.md) §9). Shorter than a lull and it does nothing; longer than the fight and it is a gauge. |

**Count: 29 at rung 0, 30 at rung 1, 32 at rung 2.**

## 2. Troop inputs

A troop's values say who came to the battle. They are inputs, and the dose-response tests check
that they act in the expected direction and size, not that the model discovers them.

Nerve is in alarm units, ardour in urge units.

| # | Input | Hastati | Warband | Reason |
|---|---|---|---|---|
| T1 | Nerve, mean | U[1.5, 2.5] | U[1.25, 2.25] | The scale of everything else in §1 is set against a nerve of 2 |
| T2 | Nerve, spread between men (SD) | U[0.5, 1] | U[0.75, 1.25] | A levy of clans is more uneven than a drilled levy |
| T3 | Ardour, mean | 0 (fixed: it is the zero of the scale) | U[0, 1] | The fury of the first onset (Livy 10.28) |
| T4 | Ardour, spread between men (SD) | U[0.3, 0.7] | U[0.5, 1] | |
| T5 | Ardour added to the leader | U[1.5, 2.5] | U[1.5, 2.5] | A centurion is chosen to go first |
| T6 | Ardour added to the front rank | 0 (fixed) | U[0.5, 1.5] | Champions fight in front of a warband; a century's front rank is its ordinary men |
| T7 | Nerve added to leader, closer and standard-bearer | U[1, 2] | U[0.5, 1.5] | Picked men |

Variants of the hastati, drawn as differences so that the direction is built in:

| # | Input | Prior | Reason |
|---|---|---|---|
| T8 | Veterans: nerve above the hastati's mean | U[0.5, 1.5] | |
| T9 | Veterans: spread, as a share of the hastati's | U[0.5, 0.8] | Survivors are alike |
| T10 | Recruits: nerve below the hastati's mean | U[0.25, 1.25] | |

**Count: 5 for the hastati, 7 for the warband, 3 for the variants.** Skill at arms, reach,
guard and breath per troop are shared and are in priors-shared.md.

## 3. Fixed, and not drawn

These are choices too. They are fixed because they are structure, not because they are known.

| What | Value | Why fixed |
|---|---|---|
| His window; reach | 5 × 5 cells of 2 m; 3 m | The shared perception (`cannae-core`) |
| Caps on counts | 8 turned or back; 6 standing in each group; 6 enemy backs; 3 falls; 3 enemies of each kind in reach | Past these a man is not counting |
| The edge rate | 1 in 16 s | It only sets where zero is on the scale of nerve and of `go` |
| Sight of his leader's acts | 20 m | About a century's frontage |
| Sight of his standard, to rally | 60 m | Standards are made to be seen; beyond this a runner is taken to be gone |
| Levels of *shaken* | 0 to 3 | |
| Levels of being winded | One for each full quarter of his breath gone, to three | |
| Forward step with no enemy in reach | 2.25 m | It must carry a man from any standoff the tie allows (at most 4.5 m) to within reach |
| Man step | 250 ms | The shared schedule |

## 4. The field substrate

To be added with `cannae-field`: the equation of state, the contact rule, the cell size and the
fallback. The freeze waits for these lines.

## 5. What I expect, written beforehand

- **Rungs 0 and 1 have one slow clock, and it is the leader.** Nothing a man carries at those
  rungs changes over an hour except his breath and his wounds. What changes in view is who
  has fallen. A warband does not replace its chieftain or lift its standard, and he is its
  boldest man, so he is the likeliest to be hit. I expect the warband's time to break to be set
  mainly by when he or its standard falls.
- **I expect the Roman line not to break at all in the base patch** at rungs 0 and 1, and I am
  unsure at rung 2. If that holds, pattern 6 fails for the Romans as "never", and the rungs
  pass or fail on the warband alone.
- **Rear-first collapse is the pattern I am least sure of.** A man sees 4 to 6 m ahead, so the
  rear ranks of a body eight deep do not see its front. Alarm must travel back rank by rank as
  men give ground. If it does not, the first to turn will be in the front half.
- **Rung 2 should break sooner than rung 1 and with a hazard that rises faster,** because falls
  in a flurry come together and stack. If the two rungs are indistinguishable, *shaken* has
  not earned its place.
- **The knife-edge rule may well fail every rung.** There are 29 to 32 independent draws, most
  with ranges of a factor of two. I have not narrowed any range to help.
