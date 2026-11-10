# Slice 0: The pairwise engine against measured bottleneck flows (S0.8, first part)

**Status:** Paullus, 2026-11-10. The substrate alone, with no man model. Run:
`cargo run --release -p cannae-pairwise --example passage`. Sources are in
[crowd-physics.md](crowd-physics.md).

The passage study proper runs a maniple falling back through an interval, and a crowd leaving
through a gate, on all three physics models (field, field with contact, pairwise). This first
part checks the pairwise substrate against the two controlled experiments we have, before the
priors freeze. That is calibration against outside data, not tuning to the eight patterns
([priors-pairwise.md](priors-pairwise.md)).

## The experiment replicated

Seyfried et al. 2009: 60 people start in a waiting area at about 3.3-4 per m², then pass a
corridor 2.8 m long and 0.8-1.2 m wide. They were told not to push. Here:
- the same geometry, walls from the scenario's new `[[wall]]` entries
  (`cannae_core::Wall`);
- the least steering a person does: head for the mouth, then down the corridor;
- walking at 1.3 m/s;
- flow measured from the 6th to the 54th man out.

Garcimartín et al. 2016 used doors of 0.69 and 0.75 m with pushing allowed. They found about
4.0 persons per metre per second at every level of competitiveness.

## Result

Specific flow in persons per metre of width per second. All 60 passed in every run.

| Width | Press | r = 240 mm | r = 280 mm | r = 300 mm | Measured |
|---|---|---|---|---|---|
| 0.8 m | none | 3.10 | 2.19 | 1.77 | 1.6-1.9 |
| 0.9 m | none | 3.45 | 2.30 | 1.81 | 1.9-2.1 |
| 1.0 m | none | 3.79 | 2.15 | 2.10 | 1.9-2.2 |
| 1.1 m | none | 3.53 | 2.65 | 2.24 | 1.8-1.9 |
| 1.2 m | none | 3.90 | 2.78 | 2.19 | 1.8-2.3 |
| 0.69 m | half | 3.48 | 2.16 | 1.58 | about 4.0 |
| 0.75 m | half | 3.61 | 2.32 | 1.98 | about 4.0 |
| 0.69 m | full | 3.75 | 2.44 | 1.99 | about 4.0 |
| 0.75 m | full | 4.27 | 2.79 | 2.11 | about 4.0 |

**Walking without pushing matches at r = 300 mm, and at 280 mm up to 1 m.** At 300 mm the flow
is inside Seyfried's measured range at three widths of five. It is just under it at 0.9 m (1.81
against 1.86-2.06) and a sixth over it at 1.1 m (2.24 against 1.78-1.93), and it rises roughly
linearly with width. Smaller discs pass far too fast. The original prior on the radius (240-300 mm) is narrowed to
**280-300 mm**.

**Pushing through a narrow door does not match at any radius that also matches walking.** The
measured 4.0 needs discs of about 240 mm, which walk through an open bottleneck nearly twice
too fast. People pushing through a gap turn their shoulders, and are about 0.3 m from front
to back but 0.5 m across. A disc is as long as it is wide, so in single file it caps a 0.7 m
door at about 1.5-2 persons a second, against the 2.8 measured.

This is a structural limit of the pairwise engine: it underestimates the flow of a pushing
crowd through gaps of one or two bodies' width by about 40 %. Two things soften it. Military
passages are wider: an interval between maniples, or a camp gate, is several bodies across,
where lanes form and shoulders matter less. And the passage study proper scores the three
physics models against each other at those widths. Bodies that turn (ellipses, or a radius
that shrinks under press) are possible remedies, each with a cost, and none is built.

**Also seen:** between 1.0 and 1.1 m at r = 280 the flow jumps, as two discs begin to fit side
by side. Seyfried found the growth continuous, with lanes whose spacing grows with width.
Equal hard discs make steps; drawing the radius from a prior smears them across draws.

**And a trap found on the way.** Men who aimed past the mouth from beside it walked into the
door frame, and its corner turned them away along its edge. They never got through. The cure
was steering to the mouth, which a person does and the man model's intents must do too.
