# Slice 0: Priors for the pairwise engine

**Status:** by Paullus, 2026-11-10, read by Varro. Frozen with the pre-registration
([preregistration.md](preregistration.md), Sign-off).

This file holds only what belongs to the vision of record's engine: the pairwise substrate,
and the top two rungs of the memory ladder. The shared core (frames, bouts, the troop table,
stamina, power and gait) is drawn from [priors-shared.md](priors-shared.md) and is the same in
every cell. Rungs 3 and 4 also read the visible layer's perception weights unchanged
([priors-field.md](priors-field.md)), so they add only their own parameters
(preregistration.md, "Common draws").

Each line gives one free parameter, the placeholder now in the code, the prior, and the source
or the reason. Sources are in [crowd-physics.md](crowd-physics.md), tagged there as primary,
secondary or unconfirmed.

## The substrate (`crates/cannae-pairwise`, `Params`)

**Drawn parameters: 4.** One is fixed. Four physical numbers that both substrates have (crush
onset density, time to disablement, turn rate, and how fast a full press gains ground) are drawn
once for both, in [priors-shared.md](priors-shared.md), at Varro's proposal (call 0001, turn
18). The derivations of `crush_mm` and the crush rate from them, in this substrate's terms, are
kept there.

| Parameter | Placeholder | Prior | Source or reason |
|---|---|---|---|
| `radius_mm`: a man's disc | 280 | uniform 280-300 (narrowed from 240-300 by the bottleneck calibration, [passage-calibration.md](passage-calibration.md): walking flows match Seyfried et al. 2009 only at 280-300 mm) | ANSUR II men: bideltoid breadth 510 mm mean, 567 mm at the 95th percentile, chest depth 254 mm [primary, computed from the public data]. Fruin's body ellipse is 600 by 450 mm [secondary]. A disc with the same area has radius 260 mm; one with the same width, 300 mm. Ancient men were smaller than modern American soldiers but carried a shield. Check: discs of radius 240-300 touch at 5.0-3.2 men per m², bracketing Fruin's "when crowd density equals the plan area of the human body, individual control is lost" [primary]. |
| `brace_mm_s`: how fast a standing man regains his spot after a shove | 600 | uniform 300-1,200 | A reason, not a source: a man shoved half a metre regains it in about a second. Checked with `press_mm_s` in the passage study (below). |
| `core_mm`: overlap with one neighbour beyond which bodies give no more | 150 | uniform 100-200 | Chest depth of ANSUR II men is 254 mm on average [primary, computed]. Bodies and kit yield some way under crowd pressure, and then not at all. Check: at r = 280 and a core of 150 mm, men pressed to the bone stand 410 mm apart, about 6.9 per m², near Fruin's "almost a fluid mass" at about 7 per m² [primary]. Across the box, 5-15 per m²; the Hajj measurements found local densities above 10 [primary]. |
| `stiffness_q8`: share of an overlap removed per pass, out of 256 | 128 | uniform 64-192 | Numerical: how soft the discs are. Together with `press_mm_s` it sets how far a pressed crowd compresses. Its effect on each pattern is reported, so that no verdict hinges on it alone. |
| `iterations`: relaxation passes per base tick | 2 | fixed at 2 | Numerical, not drawn. A convergence check at 4 is reported for the base patch. |

**Two rules with no parameter** (S0.7, after the sustained pocket run showed a ring of men
walking straight through a standing crowd to 64 men per m²):
- A man cannot walk into the men he touches ahead of him: his step loses its component
  toward them, so he slides along them or stands.
- His lean fades as his summed overlap with the men ahead, in the direction he faces, rises
  toward `core_mm`: he cannot push further than bodies give.

Crushing is drawn at the man's step (4 Hz), over the whole step.

**Known simplification.** Crushing is read from the summed overlap, whatever its direction. In
crowd deaths, a load front to back stops a man breathing and a load side to side does not
(Gill and Landi 2004 [primary]). A variant weighting front and back contacts is a candidate if
crushing turns out to matter in a scored pattern. It is not drawn now.

**What the passage study may narrow, before the freeze.** S0.8 runs a passage on all three
physics models (field, field with contact, pairwise) against measured bottleneck flows:
- walkers told not to push pass at about 1.9 persons per metre of width per second, rising
  linearly with width (Seyfried et al. 2009 [primary]);
- people allowed to push pass at about 4.0 (Garcimartín et al. 2016 [primary]).

If the pairwise engine matches those only in part of the box above (for `press_mm_s`,
`brace_mm_s`, `stiffness_q8` and `radius_mm`), the prior is narrowed to that part and the run
that justified it is recorded here. This uses outside data, not the eight patterns, so it is
calibration, not tuning.

**Done (S0.8, [passage-calibration.md](passage-calibration.md)).** The radius is narrowed to
280-300 mm: smaller discs walk through Seyfried's corridors far too fast. Nothing else is
narrowed. The pushing flow through narrow doors is about 40 % low at every radius that walks
right, and walking flow through wide gaps 30-50 % high at every radius in the prior. Both are
structural, so no part of the other boxes fixes them.

**In the sampler** (`crates/cannae-score/src/draw.rs`): `pairwise.radius_mm`,
`pairwise.brace_mm_s`, `pairwise.core_mm` and `pairwise.stiffness_q8`, in the units above.
Relaxation passes are fixed at 2.

## Rung 3: continuous fear and resolve (S0.6)

`crates/cannae-gauges`, `Params`; the design is [rungs-3-4.md](rungs-3-4.md). Two gauges per
man, on top of the visible layer's perception weights, which are drawn once and shared by every
rung. Fear rises with the frightening acts he sees and decays; resolve is spent by fear while
he is awake to something, and comes back by itself. Their alarm term is fear plus a share of
his nerve for the resolve he has spent. Nothing here draws at runtime.

| Parameter | Placeholder | Prior | Source or reason |
|---|---|---|---|
| τ_F, `fear_tau_ms`: how long fear lasts | 3 min | log-uniform 1-10 min | "Minutes": the same scale as Varro's `shaken_wear` (2-10 min), so that rungs 2 and 3 differ in form, not in time scale |
| K, `keep_pm`: how much of a fright's weight he keeps carrying | 0.625 | uniform 0.25-1 | At 1, a man who has watched a threat for several τ_F carries it in full after it has gone; at a quarter, a remnant |
| T_R, `spend_ms`: how long a fear of one alarm unit takes to empty full resolve | 30 min | log-uniform 10-60 min | Formed fights lasted hours, and lines were relieved (the triplex acies; vision 03, R2): resolve is spent over tens of minutes, not seconds |
| τ_R, `resolve_tau_ms`: how long resolve takes to come back | 30 min | log-uniform 10-60 min | A relieved line rested long enough to go back in (vision 03, R2) |
| w_R, `empty_q8`: alarm at empty resolve | 2 alarm units | uniform 1-3 | About a man's whole nerve (2, priors-field.md T1): a man with no resolve left has no nerve left |

**In the sampler:** `gauges.fear_tau_ms`, `gauges.keep_pm`, `gauges.spend_ms`,
`gauges.resolve_tau_ms` and `gauges.empty_q8`, in the stored units of the fields named above.
Rung 3 reads them on both engines, so draw *i* has the same gauges in both.

What each should do is written down beforehand in [rungs-3-4.md](rungs-3-4.md): a hazard of
breaking that rises over a long fight (a Weibull shape above rung 2's), a fright that lingers
after a shock is driven off, and volleys that kill few but wear men down.

## Rung 4: the full excitable psyche (S0.6)

Not drawn unless it is built. Michal decided (2026-11-10) that it is built only if rungs 0-3
all fail, since what it alone would add (graded intensity inside *steady*, and refractoriness
beyond breath) is tied to no named pattern ([rungs-3-4.md](rungs-3-4.md)). If it is built, its
parameters are written here under a pre-registration of its own.

## Count

| Part | Drawn | Fixed |
|---|---|---|
| Substrate | 4 (plus 4 shared) | 1 |
| Rung 3 | 5 | 0 |
| Rung 4 | only if built | |
