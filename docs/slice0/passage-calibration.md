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
- the least steering a person does: head for the nearest point of the mouth, 0.4 m in from its
  edges, then straight down the corridor;
- walking at 1.3 m/s;
- flow measured from the 6th to the 54th man out.

Garcimartín et al. 2016 used doors of 0.69 and 0.75 m with pushing allowed. They found about
4.0 persons per metre per second at every level of competitiveness.

Liao et al. (arXiv 1506.02433, experiment AO in their Table 2) ran wide bottlenecks: 350 people
at 3.0 per m² in a waiting area 18 m wide, through a bottleneck 1.0 m long and 2.4-5.0 m wide.
They report that "the flow per unit width is constant as bottleneck width changes". Their
values are only in a figure, so the comparison here uses Seyfried's 1.9 and that constancy.
Here: 360 men, 30 across by 12 deep.

## Result

Specific flow in persons per metre of width per second. All passed in every run.

**Seyfried's corridor** (60 men):

| Width | Press | r = 240 mm | r = 280 mm | r = 300 mm | Measured |
|---|---|---|---|---|---|
| 0.8 m | none | 3.10 | 2.19 | 1.77 | 1.6-1.9 |
| 0.9 m | none | 3.50 | 2.31 | 1.95 | 1.9-2.1 |
| 1.0 m | none | 3.90 | 2.34 | 1.73 | 1.9-2.2 |
| 1.1 m | none | 3.75 | 2.94 | 2.38 | 1.8-1.9 |
| 1.2 m | none | 4.08 | 2.94 | 2.37 | 1.8-2.3 |
| 0.69 m | half | 3.48 | 2.16 | 1.58 | about 4.0 |
| 0.75 m | half | 3.61 | 2.32 | 1.98 | about 4.0 |
| 0.69 m | full | 3.75 | 2.44 | 1.99 | about 4.0 |
| 0.75 m | full | 4.27 | 2.79 | 2.11 | about 4.0 |

**Liao's wide bottlenecks** (360 men, no pushing):

| Width | r = 240 mm | r = 280 mm | r = 300 mm | Measured |
|---|---|---|---|---|
| 2.4 m | 4.44 | 3.30 | 2.87 | constant, about 1.9 |
| 3.0 m | 4.29 | 3.39 | 2.94 | |
| 3.6 m | 3.84 | 3.30 | 2.89 | |
| 4.4 m | 3.19 | 2.85 | 2.56 | |
| 5.0 m | 2.98 | 2.69 | 2.42 | |

**Walking through a narrow corridor matches best at r = 300 mm.** It is inside Seyfried's
measured range at 0.8 and 0.9 m, 9 % under it at 1.0 m, and over it by 3-23 % at 1.1-1.2 m.
At 280 mm it is 15-50 % high. Smaller discs pass far too fast. The prior on the radius stays
at **280-300 mm**, narrowed from 240-300. A first version of this table aimed every man at the
middle of the mouth. Aiming at its nearest point, as here, moves the flow by up to a tenth,
and leaves r = 300 still the best fit.

**Walking through a wide bottleneck runs 30-50 % high at every radius in the prior.** At
2.4-5.0 m, discs of 300 mm pass 2.4-2.9 persons per metre per second, against a constant 1.9
or so. Real walkers slow down as a crowd thickens. A disc keeps full walking speed until it
touches the one ahead, so a packed column streams through at full speed. In a narrow corridor
of one or two lanes this hardly shows. In a wide one it does. This is the substrate's second
structural limit. The remedy, a speed that depends on the gap ahead, is discussed in
[passage.md](passage.md) and not built.

**Pushing through a narrow door does not match at any radius that also matches walking.** The
measured 4.0 needs discs of about 240 mm, which walk through an open bottleneck nearly twice
too fast. People pushing through a gap turn their shoulders, and are about 0.3 m from front
to back but 0.5 m across. A disc is as long as it is wide, so in single file it caps a 0.7 m
door at about 1.5-2 persons a second, against the 2.8 measured.

This is the first structural limit: the pairwise engine underestimates the flow of a pushing
crowd through gaps of one or two bodies' width by about 40 %. Military passages are wider,
where shoulders matter less but the second limit applies instead. Bodies that turn (ellipses,
or a radius that shrinks under press) are possible remedies, each with a cost, and none is
built.

**Who was measured.** Seyfried's and Liao's people carried no shields. A man with a scutum is
wider than a man in a shirt, so this calibration, and the field's walking curve calibrated on
the same data, may both run high for armed men. Neither engine has data for that, and neither
guesses a correction (Varro, call 0001 turn 30).

**Also seen:** between 1.0 and 1.1 m the flow jumps, as two discs begin to fit side
by side. Seyfried found the growth continuous, with lanes whose spacing grows with width.
Equal hard discs make steps; drawing the radius from a prior smears them across draws.

**And a trap found on the way.** Men who aimed past the mouth from beside it walked into the
door frame, and its corner turned them away along its edge. They never got through. The cure
was steering to the mouth, which a person does and the man model's intents must do too.
