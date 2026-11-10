# Slice 0: Priors for the pairwise engine (draft)

**Status:** a draft by Paullus, 2026-11-10. It is frozen with the pre-registration
([preregistration.md](preregistration.md)). Varro reads it before the freeze.

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

**Drawn parameters: 8.** One is fixed.

| Parameter | Placeholder | Prior | Source or reason |
|---|---|---|---|
| `radius_mm`: a man's disc | 280 | uniform 240-300 | ANSUR II men: bideltoid breadth 510 mm mean, 567 mm at the 95th percentile, chest depth 254 mm [primary, computed from the public data]. Fruin's body ellipse is 600 by 450 mm [secondary]. A disc with the same area has radius 260 mm; one with the same width, 300 mm. Ancient men were smaller than modern American soldiers but carried a shield. Check: discs of radius 240-300 touch at 5.0-3.2 men per m², bracketing Fruin's "when crowd density equals the plan area of the human body, individual control is lost" [primary]. |
| Crush onset density ρc (sets `crush_mm`) | `crush_mm` 250, which is about 4.4 men per m² at r = 280 | uniform 5-8 men per m² | Helbing and Mukerji 2012: above 4-5 per m² congestion builds and falls become likely [primary]. Helbing et al. 2007 (Hajj, measured): flow collapses above 6 per m² local, with local peaks above 10 [primary]. Fruin 1993: at about 7 per m² the crowd becomes "almost a fluid mass" [primary]. `crush_mm` is derived as six contacts' overlap at ρc in hexagonal packing: 6 (2r − s), with s = √(2 / (√3 ρc)). At r = 280 that is 477-1080 mm. The placeholder crushes too early. |
| Time to be disabled when crushed (sets `crush_rate_q16_per_mm`) | 0.01 per s per mm | log-uniform 30-600 s, as the mean time to disablement at a density 2 per m² above ρc | No measurement found. Expert estimates in press interviews put loss of consciousness at about 30 s and irreversible harm at 3-6 minutes [secondary, unconfirmed]. The rate per mm is derived from this time and the extra overlap at ρc + 2. The weakest line in the file. |
| `turn_per_tick`: fastest turn | 1,638 units (180° per s) | uniform 120-200° per s | Mason et al. 2022: turning in place peaks at 190 ± 27° per s and averages 124° per s; a 180° turn while walking peaks at 179-182° per s [primary]. The pace instruction is unconfirmed, so these are typical rates, not maximal ones. |
| `press_mm_s`: how fast a man at full press edges forward | 300 | uniform 200-700 | No direct measurement. The nearest is Pastor et al. 2015: a crowd approaching a door advanced at 0.41 m/s when told to avoid contact and 0.72 m/s when pushing was allowed [primary]. |
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

## Rung 3: continuous fear and resolve (S0.6)

To be written with the rung, before the freeze. This rung adds two gauges per man on top of
the visible layer. Fear rises with the alarm sum and decays; resolve is spent by fear and
renewed by the urge sum. Each gauge's gain and time constant is drawn here.

## Rung 4: the full excitable psyche (S0.6)

To be written with the rung, before the freeze. This rung adds the approach-avoid state with
hysteresis, and the W' cycle, which times a man's going forward and falling back. Its
parameters are drawn here, on top of rung 3's.

## Count

| Part | Drawn | Fixed |
|---|---|---|
| Substrate | 8 | 1 |
| Rung 3 | (to come) | |
| Rung 4 | (to come) | |
