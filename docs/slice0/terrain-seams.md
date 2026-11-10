# Slice 0: Where terrain plugs in later

**Status:** Paullus, 2026-11-10, at Michal's request. Nothing here is built, and nothing in
the vision changes. The point is that when a slice asks for high ground, a river or a marsh,
neither engine has to be reworked to take it. The terrain of record is
[vision 06](../vision/06-architecture.md) §7: a height map at 1-2 m, ground types (firm,
rough, marsh, sand, wood, river and fords), obstacles as lines, and slope changing speed,
fatigue, impetus and missile range.

## The one rule that keeps it cheap

**Ground never moves, so every part gets it when it is built, never through a per-tick trait.**
Walls already work this way: the scenario lists them (`[[wall]]`), each substrate is handed
them at construction (`Pairwise.walls`, `Field::with_walls`), and the passage bench builds any
substrate from them (`run(&case, |walls| ...)`). Height and ground types will travel the same
road, as one `Ground` value in the core that holds the walls, their index, a height map and a
ground-type layer. Each trait (`Substrate`, `Perceive`, `ManModel`, `Resolver`) stays as it
is. Changing `&[Wall]` to `&Ground` at the few constructors is a small change, made when the
first height arrives and not before: an empty stub now would be a mechanism without a pattern.

## Where each effect would go

| Effect | Where | What it would need |
|---|---|---|
| Walls, ramparts, gates, ditches, palisades | Both substrates | **Built** (S0.8), in both engines, with a coarse index so a man checks only the walls near him |
| Rivers and fords | Both substrates | A river is walls along its banks with gaps at the fords; a ford slows like rough ground |
| Ground type (rough, marsh, sand, wood) | Both substrates | A speed factor per type, read where a man stands. The pairwise substrate scales the step; the field scales its flow. One shared prior per type |
| Slope and speed | Both substrates | The slope along a man's way, from the height map. Uphill slower, steep downhill slower too |
| Slope and fatigue | The man's breath (`stamina::step`) | Climbing costs more of his critical power. Measured costs of walking on gradients exist in the physiology literature, to be sourced when built. The man reads the slope under his own feet, which is the ground and not anyone's inner state, so the rule that men read only what is observable holds |
| Slope and impetus | The shared bout resolver | An edge for the man above, as a function of the height difference between the two men. One shared prior, and a historical check: the uphill fights in [vision 07](../vision/07-validation.md) (Bibracte, Zela, Aquae Sextiae) |
| Slope and missile range | Missiles (not built yet) | Range and drop from the height difference |
| Sight from height, ridges that hide | Perception | Eye height above the ground, and a line-of-sight test against the height map for far things (standards, leaders, a body coming over a rise). The 3 m window of the View hardly changes |
| A hill or camp as refuge, a hill behind as support | The man (visible layer) | A refuge is a place fugitives make for (vision 03, "Refuges"); it can be read off the ground like a standard |

## What already fits, and one thing to keep

- Positions are 2D in mm, and height is a field over that plane, which is how battlefield
  terrain behaves (a "2.5D" map). No coordinate changes.
- Both engines bin men in grids, and both can read a per-cell value (height, ground type) for
  no more than a lookup.
- Everything is integer, so height in mm and slope as a ratio keep runs bit for bit.
- **To keep:** the effects of ground belong in the physics (the substrates and the bout
  resolver), and in the man only through what he can see and feel underfoot. A man should
  never be handed a hidden terrain bonus.

## When

By the parsimony rule, a terrain effect is built when a battle the slice runs needs it. Cannae
was fought on a plain by the Aufidus, so Slices 1-2 need at most the river as walls. Slope
comes with the first uphill battle in the corpus, or a sealed battle that needs it.
