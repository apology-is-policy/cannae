# 07 — Validation

## 1. Philosophy

- **Pattern-oriented modelling, for design and for validation** (Grimm et al.). Validate against
  many patterns at several scales at once. That means individual patterns (fatigue, the length
  of a duel), unit patterns (how long a maniple holds against a warband) and battle patterns
  (outcome, casualty ratio, duration, the *sequence* of events). A model that hits one number
  by tuning is worthless. A model that reproduces many independent patterns with one parameter
  set is credible. The same discipline governs design: a mechanism enters the model only when a
  named pattern cannot be reproduced without it (pillar 8 in the [README](README.md)).
- **Development, calibration and sealed sets.** Cannae is the **development scenario**. Its
  mechanism list was built from it, and we will watch it constantly, so a Cannae pass is a
  requirement, not evidence. Most of the corpus is calibration material. Three to five battles
  are **sealed** (§5): their dossiers are fixed and hash-committed before the engine can run
  them, and then they are run once.
- **Every scenario has free inputs of its own:** troop ratings, commander profiles, the plan.
  Adjusting them after seeing results is fitting, whatever the parameter count says. Ratings and
  profiles come from a fixed rubric, and plans are capped by the authoring tiers (§6.3).
- **Verification is not validation.** Some checks test a rule that [04](04-psyche.md) writes in
  directly: horses refuse a dense, steady wall; experienced men pace their own effort and form
  up without orders; only trained units can feign flight. Those checks are unit tests. They
  must pass, but they prove nothing about the theory. Only patterns the model was not told to
  produce count as evidence. The tables below mark which is which.
- **Distributions, contrasts and dose-response, not single runs.** Every check is a Monte Carlo
  band over many seeds. Where possible a check compares two conditions on paired seeds (common
  random numbers), because the *direction* of an effect needs far fewer seeds than a
  probability does. How many seeds we can afford depends on scale (§9).
- **Input uncertainty is sampled, not enumerated.** Instead of three discrete source variants
  (Polybius' numbers, Livy's, a Delbrück-style reduction), army sizes, troop quality and other
  inputs are drawn from continuous distributions that span the sources. Genuinely discrete
  alternatives stay discrete: the site of Cannae and the course of the Aufidus are debated. A
  robust result holds across the input distribution. When it does not, that is a finding worth
  reporting.
- **Contested mechanics are parameters, within limits.** Frontage per man, maniple intervals and
  the strength of *othismos* are parameters. One line-relief mechanism (R2) is built, with a
  seam for the others (see [03](03-formations-tactics.md)). The simulator can show what is
  mechanically feasible (does a line passage jam, and when?) and how sensitive outcomes are to
  each assumption. Fitting outcomes cannot pick the true relief mechanism: the model's premise
  is Sabin's pulsing combat, so R1 would lose by construction.

## 2. Stylized facts (must hold across the corpus)

| ID | Fact | Measurable target (initial) | Kind |
|---|---|---|---|
| S1 | **Casualty asymmetry.** The losing side suffers most of the casualties, mostly after it breaks. | Winners ≤ about 5–15 %; losers about 20–60 %+ in decisive battles. Most of the loser's dead fall after the first major rout. | Evidence |
| S2 | **Duration and pulsing.** Large battles last hours. Melee comes in pulses of minutes, separated by lulls. | Clash and lull structure visible in the contact-intensity time series | Evidence |
| S3 | Flank and rear attacks decide out of all proportion to the kills they cause | The rout probability from a flank charge is much higher than from a frontal charge of equal force | Evidence |
| S4 | Fresh reserves committed at the right moment are decisive | Varying the timing of reserve commitment shifts the outcome distribution | Evidence |
| S5 | Light troops rarely decide alone, except against troops that cannot respond, or in broken terrain | Carrhae passes; generic skirmish-only fights stay indecisive | Evidence |
| S6 | Cavalry wins wings and does the pursuing. It rarely breaks steady, formed heavy infantry head-on. | Horses refuse; a frontal cavalry charge into steady infantry fails | Unit test (horse refusal is written into 04) |
| S7 | The phalanx is invincible frontally on flat ground and collapses when disordered or flanked | Micro-test M3 | Evidence |
| S8 | Elephants cut both ways | Micro-test M6, Zama, Magnesia, Panormus | Mixed (elephant fear is written in) |
| S9 | Veterans beat recruits, even when outnumbered | Pharsalus (22,000 against 45,000) and the micro-tests | Evidence |
| S10 | Encircled armies are compressed, lose the ability to fight, and are slaughtered or surrender | Cannae, Teutoburg | Evidence |
| S11 | The death or flight of a commander is frequently decisive | Pydna, Metaurus, Philippi | Evidence |
| S12 | Disciplined units can retire in order or break out even in defeat | The Trebia breakout | Evidence |

## 3. Micro-scenario suite (fast, run daily)

| ID | Scenario | Expected band (to refine from sources and experimental archaeology) | Kind |
|---|---|---|---|
| M1 | Duel matrix: legionary, Gaul, Iberian, phalangite, Celtiberian | Sanity-checks the weapon and armour numbers. No duel is a foregone conclusion. | Unit test |
| M2 | Century or maniple against an equal Gallic warband on open ground | Romans win most runs by endurance and line relief. The Gauls win a meaningful share when their first rush breaks the line. | Evidence |
| M3 | Phalanx syntagma(s) against maniples: flat ground, then rough ground | Flat: the phalanx wins frontally. Rough ground or a flank opportunity: the maniples win. | Evidence |
| M4 | Cavalry against infantry: steady and frontal, disordered, flanked | Steady infantry holds. Disordered or flanked infantry routs. | Mixed (steady: unit test) |
| M5 | Horse archers against an infantry square: with and without arrow resupply, with and without slingers, high ground | Resupply means slow destruction. Slingers or high ground turn it around. | Evidence |
| M6 | Elephants against green and habituated infantry, with and without lanes and trumpets | Green troops panic. Lanes and skirmishers neutralise the elephants. Some charges rebound into their own side. | Mixed |
| M7 | Flank charge on a recruit cohort and on a veteran cohort | Veterans often turn to face. Recruits often rout. | Evidence |
| M8 | Triplex against simplex acies (equal numbers) facing a persistent enemy | The triplex outlasts, given fatigue and morale. If not, the model is wrong. | Evidence |
| M9 | Pursuit: disciplined against undisciplined pursuers | Kill rates in pursuit. How often pursuers rally. How exposed they are to a counter-charge. | Evidence |
| M10 | The Pharsalus halt | Veterans self-pace the charge. Recruits do not. | Unit test |
| M11 | The Sabis: surprise attack on men building camp | Veterans self-organize into a line. Recruits need officers. | Unit test |
| M12 | Gergovia: an eager assault, then a recall sounded from across a valley (*BG* 7.47) | Units in the valley's sound shadow do not hear it. Eager units outrun officers who try to hold them back. | Evidence |
| M13 | Feigned flight: trained against untrained cavalry | Trained cavalry turns back. Untrained cavalry routs for real. Pursuers get disordered. | Unit test (the feint); evidence (the pursuers) |
| M14 | Compression: an encircled block | The share of men able to fight collapses. Casualties accelerate. | Evidence |
| M15 | Lulls emerge | Contact intensity pulses without any scripted lull | Evidence |
| M16 | Crumble or snap: the same unit with narrow and wide threshold spreads | Compare the distributions of avalanche sizes (how many men break together), not a snap-or-crumble label. Formations are small and thin, so the transition is blurred. | Evidence |

## 4. Battle corpus

**Roles:** **D** development (Cannae and its two contrasts); **S?** candidate for sealing (§5);
**C** calibration; **Q** qualitative only, because the surviving narrative is too thin to score.

| Battle | Year | Role | What it tests |
|---|---|---|---|
| Bagradas (Tunis) | 255 BC | C | Elephants and cavalry against a legion on flat ground (Xanthippus) |
| Panormus | 250 BC | C | Elephants panicked by missiles from a ditch, stampeding into their own side |
| Trebia | 218 BC | D | Cold, hunger and wet. Mago's ambush. Cavalry superiority. 10,000 Romans breaking out through the center (Polybius 3.74). **Contrast for A1:** a formed body gets out. |
| Lake Trasimene | 217 BC | C | Ambush of a column in fog. No deployment. Death of the commander. |
| **Cannae** | 216 BC | **D** | **Acceptance test A1** |
| Dertosa (Ibera) | 215 BC | D | **Contrast for A1.** A similar plan fails because the Iberian center gives way at the first clash (Livy 23.29). The same plan with unreliable troops must fail. |
| Metaurus | 207 BC | C | Nero's flank march behind his own line on his own initiative. Hasdrubal's death. |
| Ilipa | 206 BC | C | Reverse Cannae. Deception in deployment. An enemy fighting unfed. |
| Great Plains | 203 BC | Q | The wings collapse while the Celtiberians fight to the death in the center |
| Zama | 202 BC | C | Elephants against lanes. Three lines on each side. Cavalry returning on the rear. |
| Cynoscephalae | 197 BC | C | Fog, a half-formed phalanx, a tribune's attack on the rear |
| Magnesia | 190 BC | C | Chariots stampeded. Elephants panicking in the phalanx intervals. Antiochus over-pursues. |
| Pydna | 168 BC | C | Phalanx disorder over rough ground, maniples exploiting the gaps |
| Arausio | 105 BC | Q | Divided command, uncoordinated attacks. A catastrophe. |
| Aquae Sextiae | 102 BC | C | Uphill defence, Marcellus' ambush in the rear |
| Vercellae | 101 BC | Q | Heat, dust, sun. Habituated Roman troops. |
| Chaeronea / Orchomenus | 86–85 BC | C | Field works against chariots and phalanx. Sulla rallying his men in person. |
| Tigranocerta | 69 BC | C | A small army breaks a huge host by hitting its cataphracts in flank and rear from a hill |
| Carrhae | 53 BC | C | Horse archers with resupply, cataphracts, helplessness, Publius' sortie |
| Bibracte | 58 BC | S? | Pila pinning shields, an uphill fight, the Boii and Tulingi flank attack, the third line facing about |
| Vosges (Ariovistus) | 58 BC | S? | Young Crassus commits the third line on his own initiative |
| Sabis | 57 BC | S? | Surprise during camp building. Self-organizing veterans. Caesar's personal intervention. |
| Gergovia | 52 BC | C | A recall unheard across a valley. Over-eager assault. Roman defeat. |
| Bagradas (Curio) | 49 BC | C | Feigned retreat and cavalry envelopment by Juba's general Saburra |
| Dyrrachium | 48 BC | C | Panic at the fortification line. Pompey fails to pursue. |
| **Pharsalus** | 48 BC | S? | Experience against numbers. The hidden fourth line against cavalry. The self-paced charge. |
| Zela | 47 BC | C | An enemy attacking uphill into men building camp. Scythed chariots. |
| Ruspina | 46 BC | C | Encirclement by light troops and horse. Orbis. Alternating facings to break out. |
| Thapsus | 46 BC | C | Elephants panicked by missiles. Soldiers forcing the attack signal. Massacre. |
| Munda | 45 BC | C | A redeployment misread as flight starts a collapse. A desperate fight. |
| Forum Gallorum | 43 BC | S? | Veterans against veterans fighting in silence, recruits held back |
| Philippi I and II | 42 BC | C | Cassius' fatal misreading of the battle. Two wings with opposite results. |
| Teutoburg | AD 9 | C | A column ambushed in forest and rain over several days (the field part only) |

Stretch scenarios: Pyrrhus at Heraclea, Asculum and Beneventum (280–275 BC); Raphia (217 BC);
Idistaviso (AD 16).

**Caveats.**

- **The corpus over-samples upsets.** Famous battles are famous partly because they were
  surprising. Their outcome frequencies are not base rates, which is why A2 uses a scenario
  generator (§7).
- **Many battles need mechanisms that are deferred** until a slice needs them: elephants
  (Bagradas, Panormus, Metaurus, Ilipa, Zama, Magnesia, Thapsus), chariots (Magnesia, Chaeronea,
  Zela), horse archers (Carrhae), the phalanx (Cynoscephalae, Pydna, Chaeronea, Magnesia), field
  fortifications, and weather such as fog and rain (Trasimene, Cynoscephalae, Teutoburg). See
  [08](08-roadmap.md).

## 5. Sealed battles

The sealed battles are the real test of the model. **Selection criteria:**

1. A narrative good enough to score: a deployment, a sequence of events, and casualty figures.
2. Inside the mechanisms the engine will have by then, so that a failure says something about
   the theory rather than a missing feature.
3. Different deciding factors from each other and from Cannae.
4. Not used in development.

**Candidates:** Bibracte, the Vosges, the Sabis, Pharsalus and Forum Gallorum. Four of the five
come from Caesar, a self-serving eyewitness, so before sealing we add a battle from another
source if the engine's mechanisms allow it (Trasimene or Cynoscephalae, from Polybius). The
final three to five are chosen with Michal before the engine can run full battles (Slice 2 in
[08](08-roadmap.md)).

**Protocol:**

1. **Dossier from a rubric.** Numbers as distributions spanning the sources, deployment,
   terrain, troop ratings and commander profiles, all derived from a fixed rubric written
   before any sealed dossier. The plan is authored at tier 0 or 1 only (§6.3). No LLM writes
   any part of a sealed dossier: an LLM has read about these battles.
2. **Pre-registered analysis.** The dossier also fixes the outcome classes for that battle, its
   sequence markers, the stopping rule, the casualty mapping, the scoring rule and the
   baselines.
3. **Hash-committed.** The dossier is committed to the repository, and its SHA-256 is recorded
   in `docs/STATUS.md`, before the engine can run full battles. Any later change is logged as a
   deviation, and the battle loses its sealed status.
4. **One run.** The engine version and the calibrated parameter region (§8) are frozen and
   hashed. The battle is run once over many seeds, and the result is reported whatever it is.
5. **Scoring.** The log-likelihood of the historical outcome under the simulated distribution,
   compared with simple baselines: a uniform guess over the outcome classes, "the larger army
   wins", and a Lanchester-style numbers-times-quality model.
6. **Afterwards**, a sealed battle that we learn from becomes calibration material, and we say
   so.

## 6. Acceptance test A1: Cannae, in measurable terms

### 6.1 Outcome classes

Computed automatically from the event log by tested classifier code, which lives in the metrics
crate (see [06](06-architecture.md)):

| Class | Definition |
|---|---|
| **(a) Encirclement** (historical) | The Roman infantry pushes into the Carthaginian centre and loses its order. The Libyans engage both of its flanks. Cavalry closes the rear. No formed Roman body of several thousand gets out. |
| (a1) Centre holds | Within (a): the Gallo-Iberian line bends back but stays a line until the trap closes |
| (a2) Centre breaks, trap closes | Within (a): the centre is driven in or locally routed, as Polybius 3.115 and Livy 22.47 describe, and the pocket closes anyway |
| **(b) Victory without a pocket** | The Romans lose and flee, but the rear is never closed and no formed body gets out |
| **(c) Breakout** | A formed Roman body of several thousand leaves the field in order, as at the Trebia |
| **(d) Other** | A Roman victory, a draw, anything else |

**Classifier terms** (initial values, fixed before the first scored run):

- A **formed body** is a connected group of one side's soldiers who hold their formation slots
  within tolerance and share a facing. "Several thousand" means at least 2,000 men.
- **Leaving the field** means crossing the scenario boundary or reaching a refuge (a camp, a
  town) while still formed.
- The **rear is closed** when formed enemy bodies face at least 85 % of the Roman mass's
  perimeter, so that no exit corridor of useful width remains.
- The **centre breaks** when a rout cascade takes in at least a quarter of the Gallo-Iberian
  infantry, or Roman formed bodies pass through a gap in their line, before the rear is closed.

### 6.2 Targets

- **Class (a) is the most common outcome, but not a certainty.** The design target is
  0.6 ≤ P(a) ≤ 0.95. It is a target, not a measurement: history gives one draw. The split
  between (a1) and (a2) is reported, never required. Whether a centre that breaks can still
  serve the trap is a real historical question.
- **The minority outcomes respond to the right causes.** A rare outcome cannot be pinned down
  with a few hundred seeds: with 100 seeds, a 3 % outcome fails to appear about one time in
  twenty. So A1 tests dose-response curves and contrasts, under one parameter set:
  - P(centre breaks) and P(c) rise as the quality of the Carthaginian centre falls. Dertosa is
    the contrast.
  - P(c) rises with the Roman infantry's cohesion in defeat. The Trebia is the contrast.
  - P(rear closed) falls as Hasdrubal's cavalry advantage shrinks, or as his horsemen become
    likelier to over-pursue.
- **In class (a):**
  - Roman dead plus captured lie within the source envelope (about 50,000–80,000), after the
    casualty mapping (§6.4).
  - Carthaginian losses are about 4,000–10,000.
  - The Gauls carry the largest share of Carthaginian losses (Polybius 3.117: about 4,000 of
    5,700).

### 6.3 Authoring tiers: the script budget

Polybius records no orders for Cannae. A scripted "Hannibal plan" would write the outcome into
the input, and A1 would become a puppet show. So A1 runs at three authoring tiers, and all three
are reported:

| Tier | What is authored |
|---|---|
| **0** | Deployment plus generic doctrine. Doctrine belongs to an army or culture, not to a battle: for example, "attack an exposed flank within reach", or "after winning the wing, rally and choose the most valuable target", weighted by discipline. |
| **1** | Tier 0 plus each commander's stated purpose, for example "break their horse, then go where you are needed most" |
| **2** | Tier 1 plus explicit triggers and contingencies |

**The claim is made at tier 0 or 1.** Authoring rule at every tier: nothing a commander could
not have said aloud beforehand, and triggers only over what the assignee can perceive, in coarse
terms (few, half, most). Polybius 3.116 credits Hasdrubal's own judgment for leaving the pursuit
to the Numidians and riding to help the Africans. At tier 0 or 1 that choice has to come from
the officer's own reading of the situation. The plan representation is in
[09](09-intent-and-plans.md).

### 6.4 Stopping rule and casualty mapping

The source totals include the pursuit and the capture of the camps, so a fair comparison needs
fixed rules:

- **Stopping rule.** A run ends at the first of: one side has no formed body left on the field
  and a fixed pursuit window has passed (initially 60 simulated minutes); nightfall; a hard cap
  (initially 8 hours).
- **Casualty mapping.** *Dead* are the killed plus the share of the severely wounded who would
  die within days (from the wounds model in [10](10-combat-and-wounds.md)). *Captured* are the
  men who surrendered plus those taken in camps and refuges. The 10,000 men Paullus left to
  guard his camp, who were captured there (Polybius 3.117), enter either through a camp
  sub-scenario or as a fixed term stated in the dossier. *Escaped* are those who left the field.

### 6.5 Sequence markers in class (a)

- The Roman cavalry on the right breaks before the infantry fight is decided.
- The allied cavalry collapses after Hasdrubal arrives in its rear.
- The Libyans turn inward after the Roman mass has pushed deep into the yielding centre.
- The rear is closed by cavalry.
- The share of encircled Romans able to fight collapses (compression).

## 7. Acceptance test A2: indeterminacy

Beating a Lanchester predictor is trivial, and naming a decisive event after the fact is always
possible. As first written, adding noise would have passed A2. The revised test:

1. **Balanced scenarios from a generator.** Historical battles are ones both commanders chose to
   accept, a selection effect that sandbox setups lack. A scenario generator draws armies,
   deployments and terrain. "Balanced" means a naive strength ratio (numbers times fixed
   troop-quality weights) close to one. Historical near-even fights (Munda, Philippi I, Forum
   Gallorum) are added as a check.
2. **A strong predictor.** A model trained on thousands of small simulated battles, using only
   pre-battle observables: numbers and types, ratings, deployment geometry, terrain, commander
   profiles. **A2 is its residual uncertainty on balanced scenarios.** On lopsided scenarios it
   must be confident and right, because indeterminacy must not mean noise.
3. **Variance decomposition by RNG stream.** The counter-based RNG gives each kind of randomness
   its own stream: hidden traits drawn at setup, combat dice, perception noise, command and
   messengers. Re-randomising one stream class with the others frozen yields sensitivity
   indices: how much of the outcome was fixed by hidden traits at setup, how much is dice, and
   how much is command friction. "Not noise" becomes a number.
4. **Predictability over time.** The predictor, re-run on the battle state as it unfolds, should
   become more certain in steps at identifiable events (a wing breaking, a reserve committed, a
   commander falling), rather than drift smoothly.
5. **Mechanism attribution** stays, as a diagnostic rather than a test. For every seed the event
   log names the decisive event: the first system-wide rout cascade, a commander's death, the
   timing of a reserve, a misread signal.
6. **Blind trials.** You look at deployments, predict outcomes with probabilities, and we track
   your Brier score. If you can predict perfectly, A2 fails.

## 8. Calibration: sensitivity analysis and history matching

Before troop profiles, the psyche and combat models have roughly 50–100 free parameters. Each
battle in the corpus gives about one bit of outcome and a disputed casualty figure. At that ratio
any outcome can be fitted, so a fit alone shows nothing. The procedure:

1. **Fix what independent evidence can fix:** physiology (W′ and recovery constants), load
   carriage, reconstruction trials, weapon and armour tests. These parameters are not tuned on
   battles.
2. **Screen.** A global sensitivity analysis (Morris screening, then Sobol indices) over the
   micro-suite and small battles finds the ten or so parameters that matter for the patterns.
3. **History matching** (Craig et al.; Vernon et al.). Rather than search for one best fit, rule
   out the regions of parameter space whose outputs are implausibly far from the observed
   patterns, given the uncertainty of the sources, of the model and of the emulator. This is
   done in waves, with a cheap statistical emulator of the simulator.
4. **Report the surviving region.** For each test, report the share of the not-ruled-out region
   in which it passes. A test that passes in only a sliver of the region is fragile, and we say
   so.
5. **Model comparison.** Where mechanisms compete to explain the same patterns (the psyche
   kernels in [04](04-psyche.md), for example), they are compared on the same patterns, and the
   simplest that reproduces them is kept.

## 9. Seeds and machines

The machine is a fanless 8 GB MacBook Air ([06](06-architecture.md)). That sets the budget:

| Scale | Seeds | Where |
|---|---|---|
| Micro-tests and slice battles (hundreds to a few thousand agents) | Thousands; runs take seconds | Locally |
| Little Cannae at 1:10 | A hundred or more overnight | Locally |
| Full-scale Cannae | 5–10 per night | Locally, for development only |
| Acceptance batches at full scale | Hundreds | A rented many-core machine, with Michal's approval, since it costs money |

Calibration happens at small scale, where runs are cheap. A scale study (1:20, 1:10, 1:5)
checks that what was calibrated small still holds as the scale grows.

## 10. Counterfactual and contrast checks

Each check runs on paired seeds, so its direction is measured cheaply.

| Counterfactual | Expected direction |
|---|---|
| Cannae with Libyans in the center instead of Gauls | The center holds better, but there is less force for the envelopment |
| Cannae with Caesar's veterans instead of the 216 BC legions | Higher odds of a breakout or a turn-to-face |
| Zama without lanes | The elephants do much more damage |
| Carrhae with 4,000 slingers on the Roman side | The Parthian advantage drops sharply |
| Pydna on flat ground | The phalanx is much stronger |
| Pharsalus without the fourth line | Pompey's cavalry envelopment succeeds far more often |
