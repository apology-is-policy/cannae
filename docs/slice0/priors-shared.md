# Slice 0: Priors for the shared core (draft)

**Status:** a draft, 2026-11-10. Paullus writes stamina, power and gait; Varro writes the
frames' ties, the bout resolver and the kits. Each signs the other's lines. Frozen with the
pre-registration ([preregistration.md](preregistration.md)).

Every cell of the experiment draws these from the same priors, and draw *i* takes the same
values in every cell, from a seed fixed at the freeze (preregistration.md, "Common draws").
What a troop fights with and how much wind it has are the same men in both engines, so they
are here. What a troop is like inside belongs to each engine's own file
([priors-pairwise.md](priors-pairwise.md), [priors-field.md](priors-field.md)).

Sources are in [physiology.md](physiology.md), each tagged there as primary, secondary,
unconfirmed or derived.

## Stamina, power and gait (Paullus)

**The units change: power as a multiple of critical power.** The core's placeholders mixed two
scales. Critical power was 250 W, a cycling figure for the work done at the crank. The power
table read like metabolic watts, about five times larger at a gross efficiency near 20 %
(Hopker et al. 2010 [primary]). The literature agrees on ratios more than on absolute watts,
so the core will express every activity as a multiple of the man's own critical power (CP),
and W' in CP-seconds, i.e. W'/CP. Working at 2 CP, a man spends it in W'/CP seconds. (The code counts it in CP-microseconds.) The engines
read breath only through `stamina::step` and the power table, so the seam does not change.

Skiba's recovery constant, τ = 546 exp(−0.01 D_CP) + 316 s, was fitted in mechanical watts on
subjects whose CP was about 240 W (physiology.md §4, [secondary], consistent with the 377 s
reported at 20 W of recovery). In the new units it becomes τ = k (546 exp(−2.4 f) + 316) s,
where f = (CP − P) / CP is the margin below CP. At rest (f ≈ 0.86) that gives 385 s. Ferguson
et al. 2010 found a recovery half-time of about 234 s after exhaustion, so τ ≈ 340 s
[secondary].

**Drawn parameters: 7.** Three are fixed. The values are the same for every troop type unless
a line says otherwise.

| Parameter | Placeholder now | Prior | Source or reason |
|---|---|---|---|
| W'/CP: seconds of work at CP held in reserve | 80 s (20 kJ at 250 W) | uniform 50-100 s | Cycling: 68-82 s (Black et al. 2015, via Jones and Vanhatalo 2017 [secondary]). Team-sport men running: 70 s (Vassallo et al. 2020 [primary]). The ratio is the same in mechanical and metabolic units. Widened for men fit from marching but not athletes. |
| k: scale on Skiba's τ | 1 | uniform 0.7-1.4 | Covers Skiba's 377 s and Ferguson's τ ≈ 340 s at rest, and the simplification of applying τ step by step to the current power rather than to the session's mean (stamina.rs). |
| Rest | 80 W | fixed at 0.09 CP | 1 MET is about 80 W for 70 kg, a convention [unconfirmed], over a CP near 900 W metabolic (75 % of a VO2max of 50 ml/kg/min [primary ranges]). It only enters recovery, where it barely matters. |
| Stand, in kit | 120 W | fixed at 0.14 CP | Pandolf: standing with 25 kg costs 129 W for a 70 kg man [secondary equation, derived]. |
| Walk at 1.3 m/s, in kit | 200 W | uniform 0.40-0.60 CP | Pandolf: 370 W with 25 kg, i.e. 0.41 CP. The equation under-predicts military loads by 12-33 % [secondary], which gives 0.55. Mail and a shield load the limbs as well as the trunk, and armour on the limbs costs more than the same weight carried (Askew et al. 2012 [primary, abstract]). |
| Run at the run gait | 450 W | uniform 1.2-1.6 CP | Level running costs about 1 kcal/kg/km, ~1,250 W gross at 4 m/s for 70 kg, i.e. 1.4 CP [secondary, derived]. 4 m/s is about the speed at VO2max for fit team-sport men (Vassallo et al. 2020 [primary]). Kit adds to it. |
| Fight: a man forward, striking and warding | 500 W | uniform 1.1-2.0 CP | No study measures the total cost of weapon fighting [unconfirmed]. Boxing sparring draws about 70 % of VO2peak aerobically (≈ 0.9-1.0 CP), with blood lactate of 6-9 mmol/L, so a large anaerobic share on top (Arseneau et al. 2011 [primary]). Armoured combat drills, three 90 s rounds, reached near-maximal heart rate and 8.2 mmol/L (Mohammadalizadeh et al. 2024, n = 1 [primary]). With W'/CP = 70 s, the prior empties W' in 70-700 s of unbroken fighting. That brackets "W' spent in roughly 1-5 minutes". |
| Press: leaning into the men in front | 400 W | uniform 0.9-1.8 CP | No metabolic data [unconfirmed]. Scrum studies: a man sustains 1-2 kN against a machine, tests last 5-10 s, and five 5 s pushes lose 23 % of force (Martin and Beckham 2020 review [primary]). A moving shove does work F·v far above CP; a static lean does little external work. The weakest line here. |
| Walk speed | 1.3 m/s | fixed at 1.3 m/s | Vegetius 1.9: "militari ergo gradu XX milia passuum horis quinque dumtaxat aestiuis conficienda sunt", i.e. 20 Roman miles in five summer hours. That is 29.6 km in about 6¼ hours, 1.32 m/s, checked in the Latin (thelatinlibrary.com, edition not named). The full step, "qui citatior est", is 24 miles: 1.58 m/s. |
| Run speed | 4.0 m/s | uniform 3.0-5.0 m/s | Vegetius 1.9: "quicquid addideris, iam cursus est, cuius spatium non potest definiri": anything faster than the full step is running, and its distance cannot be fixed. 4 m/s is about the speed at VO2max for fit men, unloaded [primary]. A charge over a short distance may go faster, and a shield and kit slow it. |
| Troops differ in wind | none | not drawn: the same for every troop | Livy (10.28) blames the Gauls' fading at Sentinum partly on bodies that cannot bear toil and heat. That is a candidate variant, not a prior: building it in would hand one side a result the patterns are meant to test. |

**In the sampler** (`crates/cannae-score/src/draw.rs`): each line's name is its address among
the draws, fixed at the freeze, and its values are in the stored unit. `stamina.reserve_us`
(W'/CP in µs), `stamina.tau_scale_pm`, `power.walk_pm`, `power.run_pm`, `power.press_pm` and
`power.fight_pm` (per mille of CP), and `gait.run_mm_s`. Rest, stand and the walk are fixed in
the code. Every troop gets the same drawn wind.

## Crowd physics both substrates have (Paullus, moved from priors-pairwise.md)

Drawn once for both, so that neither substrate can be said to have had the kinder draw. Each
substrate maps them onto its own mechanics. The pairwise engine's mapping is given in each
line; the field engine's goes in [priors-field.md](priors-field.md). Sources are in
[crowd-physics.md](crowd-physics.md).

| Parameter | Placeholder now (pairwise) | Prior | Source or reason |
|---|---|---|---|
| Crush onset density ρc (sets `crush_mm`) | `crush_mm` 831: ρc = 6.5 at r = 280 | uniform 5-8 men per m² | Helbing and Mukerji 2012: above 4-5 per m² congestion builds and falls become likely [primary]. Helbing et al. 2007 (Hajj, measured): flow collapses above 6 per m² local, with local peaks above 10 [primary]. Fruin 1993: at about 7 per m² the crowd becomes "almost a fluid mass" [primary]. `crush_mm` is derived as six contacts' overlap at ρc in hexagonal packing: 6 (2r − s), with s = √(2 / (√3 ρc)). At r = 280 that is 477-1080 mm. Checked in the pairwise substrate: a crowd shoved against a wall for 90 s reaches 5.4-8.0 per m² across the corners of its priors, and its summed overlap is within about 7 % of this formula at the measured density (`--example press_wall`, [passage.md](passage.md)). |
| Time to be disabled when crushed (sets `crush_rate_q32_per_mm`) | 100,830 in Q32, 2.35 × 10⁻⁵ per s per mm: 134 s at ρc + 2, with ρc = 6.5 at r = 280 | log-uniform 30-600 s, as the mean time to disablement at a density 2 per m² above ρc | No measurement found. Expert estimates in press interviews put loss of consciousness at about 30 s and irreversible harm at 3-6 minutes [secondary, unconfirmed]. The rate per mm is derived from this time and the extra overlap at ρc + 2. That density may lie beyond what a draw's crowd can reach; the rate is then a slope, and the crush slower. The weakest line in the file. |
| `turn_per_tick`: fastest turn | 1,638 units (180° per s) | uniform 120-200° per s | Mason et al. 2022: turning in place peaks at 190 ± 27° per s and averages 124° per s; a 180° turn while walking peaks at 179-182° per s [primary]. The pace instruction is unconfirmed, so these are typical rates, not maximal ones. |
| `press_mm_s`: how fast a man at full press edges forward | 300 | uniform 200-700 | No direct measurement. The nearest is Pastor et al. 2015: a crowd approaching a door advanced at 0.41 m/s when told to avoid contact and 0.72 m/s when pushing was allowed [primary]. |

**Count: 4.**

**In the sampler:** `crowd.crush_onset_q8` (men per m² in Q8, 1,280-2,048),
`crowd.crush_time_ms` (log-uniform, 30,000-600,000), `crowd.turn_per_tick` (turn units per base
tick, 1,092-1,820) and `crowd.press_mm_s`. Both engines read the same four lines, so draw *i*
has the same crowd in both. The pairwise engine derives `crush_mm` and `crush_rate_q32_per_mm`
from the first two and its drawn radius, in integers, by the formulas above. At the
placeholders' point they give 831 mm and a rate within 0.5 % of 100,830 (tested). The field
maps them in its own file.

## Ties, bouts and kits (Varro)

**U[a, b]** is uniform; **L[a, b]** is log-uniform. Draws are independent.

### Ties (`frames::Tie`)

How a body keeps its order. A drilled body is a century or a maniple; a loose one is a warband.

| # | Field | What it is | Drilled | Loose | Reason |
|---|---|---|---|---|---|
| S1 | `tolerance_mm` | A man this near his slot is in his place. Also how far a halted body's standard must be driven before the body gives ground. | U[0.4, 0.9] m | U[0.8, 1.8] m | A quarter to a half of the drilled interval of 1.8 m; a warband is twice as slack |
| S2 | `close_after` | How long before a file closes over a man who has fallen or fled | L[1, 5] s | L[5, 30] s | A drilled file steps up at once; a warband fills in as men notice |
| S3 | `lift_after` | How long a fallen standard lies before a man beside it lifts it | L[1, 10] s | never | A rule of the drilled body. A warband's standard, once down, stays down. |
| S4 | `succeed_after` | How long before the closer takes a fallen leader's place | L[2, 15] s | never | The optio succeeds the centurion. A warband has no second. |
| S5 | `standoff_mm` | The gap between fronts at which an advance halts | U[3, 4.5] m, one draw for both | | Just out of reach of a lunge ([rival/02](../rival/02-order-and-contact.md) §7). The upper end is tied to the man's forward step. |
| S6 | `contact_mm` | The same for a charge | U[1.0, 1.4] m, one draw for both | | Inside a sword's reach of 1.5 m |

Fixed: `lead_mm` 1.5 m (it only has to exceed one man step at a run) and `lift_reach_mm` 2.5 m
(the men in the slots around the standard).

**Count: 8** (S1 and S2 twice, S3 to S6 once). An earlier draft of this file said 10, which was
a miscount.

### Bouts (`bouts::BoutParams`)

Multipliers apply to the chance that a strike wounds.

| # | Field | What it is | Prior | Centre | Reason |
|---|---|---|---|---|---|
| S7 | `rate` | Bouts per second for a forward man with an enemy in reach | L[0.05, 0.2] | 0.1 | An exchange every 5 to 20 s between men who are at it. Elite epeeists, who must attack, fence in actions of 18 s ([lethality-line-fighting.md](lethality-line-fighting.md), C3), so this is one to four bouts in such an action. No source counts passes. |
| S8 | `harm` | The chance that one strike wounds: striker forward, target steady and facing him, both sound, equal skill, the reference shield and armour | L[0.003, 0.1] | 0.05 | With the shares below, the chance that a bout disables or kills either man is about equal to `harm` between a forward man and a steady one, and 1.5 times it between two forward men. Pattern 7's band for that chance is 0.002 to 0.15; this range sits inside it. Both halves of the lethality search put the plausible part at 0.005 to 0.07. |
| S9 | `target_forward` | The target has himself stepped forward | U[1, 2] | 1.5 | He has come out from behind his shield |
| S10 | `target_back` | The target is giving ground | U[0.7, 1.3] | 1 | Harder to reach, easier to hurry: unknown sign |
| S11 | `target_turned` | The target has turned his back | U[4, 12] | 8 | Most killing is done in flight (du Picq; Towton, Alken Enge in [lethality-skeletal.md](lethality-skeletal.md)). This is the slaughter. |
| S12 | `shield_side` | The strike comes at his left | U[0.7, 1] | 1 | |
| S13 | `open_side` | The strike comes at his right | U[1.3, 3] | 2 | |
| S14 | `behind` | The strike comes from behind a man who has not turned | U[2, 6] | 4 | |
| S15 | `unattended` | The target is attending to a nearer attacker | U[1.5, 3] | 2 | Two on one |
| S16 | `target_hampered` | The target has a hampering wound | U[1.2, 2] | 1.5 | |
| S17 | `striker_hampered` | The striker has one | U[0.4, 0.8] | 0.6 | |
| S18 | `answer` | The answering strike of a man who did not step forward | U[0.5, 1] | 0.7 | A parry and riposte against a committed attack |
| S19 | `skill_step` | Each step by which the striker out-skills his target | U[1.1, 1.4] | 1.25 | |
| S20 | `disable` | Of wounds, the share that disable | U[0.25, 0.45] | 0.35 | Hampering wounds should be at least as common as disabling ones (healed trauma at Ephesus and Towton) |
| S21 | `kill` | Of wounds, the share that kill outright | U[0.08, 0.25] | 0.15 | 10 to 30 % of disabling outcomes (lethality-skeletal.md) |
| S22 | `drive_back` | The chance that a strike which does not wound drives its target back a step | U[0.1, 0.5] | 0.3 | |

**Count: 16.**

### Kit, per troop (`bouts::Kit`)

| # | Input | Hastati | Veterans | Recruits | Warband | Reason |
|---|---|---|---|---|---|---|
| S23 | Guard: shield and armour, as a multiplier on harm taken | 1 (the reference) | 1 | 1 | U[1, 1.5] | A shield and no body armour against scutum and mail. Polybius on Gallic shields and swords (2.30, 2.33) is from memory, to check. |
| S24 | Skill, in steps | 0 (the reference) | +1 | −1 | 0 | Fixed. The size of a step is S19. |
| S25 | Skill of the leader, in steps above his troop | +2 | +2 | +2 | +2 | Fixed |

Reach is fixed at 1.5 m for every troop in the patch: all fight with swords.

**Count: 1.**

**In the sampler** (`crates/cannae-score/src/draw.rs`, `VARRO_LINES`):

- **Ties:** `tie.drilled.tolerance_mm`, `tie.loose.tolerance_mm`, `tie.drilled.close_after_ms`,
  `tie.loose.close_after_ms`, `tie.drilled.lift_after_ms`, `tie.drilled.succeed_after_ms`,
  `tie.standoff_mm` and `tie.contact_mm`. Times are drawn in ms and stored in base ticks of
  50 ms, to the nearest.
- **Bouts:** `bout.rate_q16` (bouts per second in Q16: 3,277 to 13,107), `bout.harm_ppm`
  (parts per million: 3,000 to 100,000), and the fourteen multipliers and shares S9 to S22 in
  per mille, each named `bout.<field>`.
- **Kit:** `kit.warband.guard_pm`.

## Count

| Part | Drawn | Fixed |
|---|---|---|
| Stamina, power, gait | 7 | 3 |
| Crowd physics | 4 | 0 |
| Ties, bouts, kits | 25 (ties 8, bouts 16, kit 1) | 5 (lead, lift reach, skill steps, leader skill, reach) |
| Volleys | 0 | 3, written in the one scenario that has them: the dose of the `volleys` variant, per thousand men under it (75 hampered, 20 disabled, 5 killed). It is the script's, as the size of the band in `shock` is, and no engine reads it differently. |
| Relief | 0 | 4. Three are written in the one scenario that has it: the distance between the lines (24 m), the earliest time (20 min) and the length of a lull (20 s). One is in the frames: how far off an enemy who steps forward still breaks a lull (20 m). All four are choices with no source ([rival/02](../rival/02-order-and-contact.md) §4, as built). |

## Sign-off

| Lines | Written by | Read and signed by |
|---|---|---|
| Stamina, power, gait | Paullus | Varro: read 2026-11-10, no objection (call 0001 turn 16); signs at the freeze |
| Crowd physics | Paullus (ranges copied by Varro for the field) | Varro: read 2026-11-10, no objection. The field's mapping of each line is in priors-field.md §4. Signs at the freeze |
| Ties, bouts, kits | Varro | Paullus: read 2026-11-10; signs at the freeze. S23 checked in the Greek (below) |

**Paullus on S23 (the warband's guard), 2026-11-10, checked in the Greek** (Büttner-Wobst's
text on Perseus). U[1, 1.5] stands.

- **2.30.3** is about the Gaesatae under the javelins: οὐ γὰρ δυναμένου τοῦ Γαλατικοῦ θυρεοῦ
  τὸν ἄνδρα περισκέπειν, ὅσῳ γυμνὰ καὶ μείζω τὰ σώματ᾽ ἦν, τοσούτῳ συνέβαινε μᾶλλον τὰ βέλη
  πίπτειν ἔνδον: "since the Gallic shield could not cover the man all round, the more naked
  and the bigger their bodies were, the more the missiles fell home". The Loeb's "does not
  cover the whole body" is fair.
- **2.30.8** has a gap in the transmitted text: οἱ μὲν οὖν θυρεοὶ πρὸς ἀσφάλειαν, αἱ δὲ
  μάχαιραι πρὸς πρᾶξιν μεγάλην διαφορὰν [lacuna] ἔχειν: "the shields for safety, and the
  swords for action, [made] a great difference". The Loeb's "far more serviceable for defence"
  translates a supplement. The Greek says only that the shields differed greatly in the
  safety they gave. In context that means the Roman ones gave more.
- **2.28.8:** the Gaesatae threw off their trousers and cloaks and stood in front γυμνοὶ μετ᾽
  αὐτῶν τῶν ὅπλων, "naked, with their arms alone".
- **2.30.2** cuts the other way, for the rest of the warband. Against the same javelins, the
  Celts behind had πολλὴν εὐχρηστίαν, "much good", from their cloaks and trousers. Clothing
  helped them against missiles.

So the evidence is for missiles against naked men, and nothing in 2.28-2.33 mentions Gallic
body armour. A guard worse than the reference fits, and the bottom of the range, 1, covers a
clothed Gaul with a shield. 2.33 is about swords that bend and only cut: it bears on how the
Gauls strike, not on what they take, so it should not be cited for S23.
