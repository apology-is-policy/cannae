# 04 — The Psyche: Body, Mind, Discipline, Experience

This is the heart of the project. Existing games, such as Total War and Field of Glory, model
morale as a **unit-level bar** that drains with casualties and flanking until it crosses a
threshold. We model **each soldier** as a small dynamical system. Soldiers are coupled through
what they perceive, through their social ties and through their leaders. Unit morale is never
stored anywhere. It is something you *measure* by looking at the men.

Three ideas carry the design. Each comes from outside game design:

1. **Approach and avoidance form an excitable system.** Each soldier has a fast approach–avoid
   state and two slow recovery variables: stamina (W′) and resolve (R). Like the FitzHugh–Nagumo
   model of a nerve cell, such a system rests until pushed hard enough, then fires a pulse and is
   refractory while it recovers. In battle the pulse is a surge into contact, a clash, a recoil
   and a lull. Zeeman's catastrophe model of aggression (1976) inspired the first draft; design
   review 1 showed why it had to change (§5).
2. **Rout cascades are avalanches in a disordered, coupled system.** This is the random-field
   Ising model with hysteresis (Sethna et al.). Soldiers with heterogeneous thresholds influence
   their neighbours through what they visibly do. The *spread* of individual courage within a
   unit shapes whether it crumbles gradually or **snaps** all at once. That is a testable
   prediction, though small formations blur it.
3. **Stamina follows exercise physiology.** The critical-power / W′ model: an anaerobic reserve
   that drains above a sustainable power and refills below it. Fighting at full intensity
   exhausts a man in minutes, and recovery takes several more. Clashes, lulls and the value of
   line relief emerge from this. They do not have to be scripted.

One rule governs all three: **parsimony**. A mechanism enters the model only when a named
historical pattern cannot be reproduced without it (§5.6).

## 1. Agent pipeline (per soldier)

```
          ┌────────────── orders / signals / leader presence ───────────────┐
          ▼                                                                 │
Perception ──► Appraisal ──► Excitable psyche (x,R) ──► Arbitration ──► Action ──► Motor/Combat
   ▲  (sight, sound,   (threat, support,   (x: approach↔avoid;   (drive vs order,   (steer, strike,
   │   touch, rumors)   helplessness...)     F, R, fury, shame)    via discipline)    throw, flee...)
   └──────────────────────────── body state (stamina, wounds, pressure) ◄───────────┘
```

Each stage runs at its own rate. Motor and combat run every tick (20 Hz). Perception, appraisal
and the psyche run at 2–5 Hz, staggered across agents. Expensive long-range perception samples
coarse fields (see [06](06-architecture.md)). The think rate sets how fast panic can travel, so
reaction time is a model parameter, not an engine detail, and each agent's phase is assigned by
hash.

## 2. Per-soldier state

### Static traits

Traits are drawn at creation from the troop profile's distributions:

| Trait | Meaning |
|---|---|
| Temperament threshold | Baseline courage: the man's individual random field |
| Aggression | Baseline fury gain: how readily he closes and strikes |
| Strength, endurance | Critical power (CP), W′ capacity, carrying capacity |
| Weapon skills | Offence and defence per weapon class |
| Age and experience | Battles survived. A **habituation vector** per stressor class (elephants, cavalry charges, horse archers, chariots, encirclement, night, Gallic noise...). |
| Social ties | Tent-group (*contubernium*, 8 men), century, maniple or cohort, legion. Or clan, retinue, kin. Ties are strengthened by time served together. |
| Role | Soldier, file leader, file closer, optio, signifer, centurion, musician, ... |

### Dynamic body

| Variable | Dynamics |
|---|---|
| W′ balance (anaerobic reserve) | Drains when power demand > CP (fighting, running, carrying a heavy shield up): about 1–3 minutes of full-intensity fighting empties it. Refills below CP, with a time constant of roughly 5–10 minutes at rest (Skiba et al. 2012). |
| Glycogen fatigue | Slow. Lowers CP over hours. Restored only by food and long rest, i.e. not within one battle. |
| Heat load and hydration | Sun, armour, exertion. Vercellae, Carrhae. |
| Wounds | Per body zone: severity, bleeding rate, functional impairment (weapon arm, legs, vision). Pain and shock. |
| Crowd pressure | From collisions. Support when it comes from behind, crush when it comes from all sides. Inability to raise weapons. |
| Load state | Pila left, shield intact or pinned or dropped, javelins, arrows |

Glycogen, heat and hydration are later features. They enter when a slice needs their pattern.

### Dynamic mind

| Variable | Timescale | Role |
|---|---|---|
| **x**: approach–avoid state | Seconds (τx ≈ 0.5–1 s) | The fast variable. Strongly positive means attack or pursue. Moderately positive means hold and fight. Near zero means hesitate. Moderately negative means give ground, still facing the enemy. Strongly negative means turn and flee. Comrades never see x, only the acts it produces. |
| **F**: fear (acute threat) | Seconds | Fast arousal from perceived threats. Pushes x toward avoidance. Decays when the threat ends. |
| **Φ**: fury (rage, bloodlust) | Seconds to minutes | Fuelled by contact, by success, by comrades killed, by the war cry. Pushes x toward approach and spends W′ fast. Drives the charge and the pursuit. Overrides restraint, which is how men end up looting. |
| **R**: resolve (morale reserve) | Minutes | The slow psychological recovery variable. Drained by sustained fear, comrades seen falling and helplessness. Refilled only in safety, faster with leaders and standards in view; relief by a fresh line provides the safety. Low R holds x on the avoid side. |
| **H**: helplessness | Minutes | Builds under fire you cannot answer: Carrhae, Panormus. Drains R quickly. |
| **S**: honor / shame pressure | Event-driven | Whether officers and comrades can see him, the standard at risk, rivalry with neighbouring units. |
| **B**: local beliefs | Event-driven | "Flanked left", "the centurion is down", "the enemy is breaking", "the camp is taken". Set by his own perception or by rumors, with a confidence level. |

Integrator state is f32, or 16-bit fixed point where bandwidth matters (§5.1). About 100 bytes
per soldier is 15 MB for 150,000 men, so memory is not the constraint.

## 3. Perception

Information inside a formation is local too:

- **Sight.** A forward cone. Range is limited by dust, fog, light and terrain. It is occluded by
  bodies: a man in the 6th rank sees the backs of his comrades, not the enemy. Elevation helps
  (officers on horseback, men on a hill). Helmet types narrow the cone.
- **Sound.** A coarse acoustic field: the intensity and direction of clamor, screams and fighting
  noise. The commands of a nearby centurion (about 10–30 m in battle din). Horn signals (cornu,
  tuba) carry hundreds of metres, but only a few signals are distinguishable. Din masks them and
  terrain shadows them: at Gergovia most legions did not hear the recall because a wide valley lay
  between them and Caesar (*BG* 7.47). **Fighting heard behind you** is the strongest fear input
  there is.
- **Touch.** Crowd pressure: pushed from behind, which is support, or squeezed from the sides,
  which means compression and encirclement.
- **Proprioception of neighbours.** Are my left, right and rear neighbours present and steady?
  "Am I alone?" is the core cohesion signal.
- **Rumors.** Diffusing fields with a fixed vocabulary: "pushed back", "broken", "the centurion
  is down", "the enemy is behind us", "the camp is taken", "the enemy is breaking". A rumor
  enters where someone perceives or believes the event, spreads at the speed of shouting through
  occupied ground, and fades. A man hears an item when its intensity passes his threshold.
  Distortion drifts toward the more alarming neighbour in the vocabulary: "the left is pushed
  back" becomes "the left is broken". Veterans discount rumors and look for themselves. Recruits
  amplify them.

So panic can start in the **rear ranks**, from noise and rumor, while the front is holding. Du
Picq observed exactly this: the rear ranks are the first to slip away.

## 4. Appraisal

Perceptions become a handful of scalar signals. **Experience modulates how accurate they are**:
a recruit's threat estimate is noisy and biased upward by noise and spectacle, while a veteran
reads the situation closer to the truth.

- **Threat in front.** Enemy density in reach, their aggression (whether they are advancing),
  weapon reach (a sarissa wall), cavalry approaching at speed, elephants.
- **Threat on the flanks and rear.** Enemies or fighting perceived outside the forward arc. Heavy
  weight.
- **Support.** Neighbours present and steady, the depth behind me, a fresh line nearby, a leader
  visible or audible, a standard nearby, a fortified camp or hill behind.
- **Attrition.** Comrades falling nearby (seen), my own wounds, my own exhaustion.
- **Success.** Enemies falling back or falling. The enemy line wavering.
- **Unanswerable fire.** Being hit while unable to respond. This feeds H.
- **Comrades' acts.** Stepping back, turning away, running, dropping shields, falling, surging
  forward, each read against the unit's current order (§5.4).
- **Habituation.** Novel stressors are amplified, and familiar ones are damped per stressor class.
  The Romans facing elephants at Heraclea versus at Zama. Marius making his recruits watch the
  Teutones for days before letting them fight.

Appraisal reuses the neighbour scan of the collision pass, so local perception costs little extra
(see [06](06-architecture.md)).

## 5. Psyche dynamics: an excitable system

The first draft used Zeeman's cusp. Design review 1 showed that its algebra did not do what the
prose claimed: resolve appeared in no equation, fear and fury were interchangeable, high arousal
made men *more* stubborn rather than more brittle, and routs healed themselves once arousal
decayed. The repaired model is an excitable system, the structure that produces pulses separated
by refractory lulls. The Slice 0 toy settles the details (§5.6).

### 5.1 The equations

Each think tick, integrated with a fixed step and an explicit noise term:

```
fast:   τx · dx/dt = K(x, d − w) + σ · ξ(t)
drive:  d = gΦ · Φ − gF · F + support + social
            + shame + success
slow:   w = cW · (1 − W′/W′max) + cR · (1 − R)
```

- **x** is the fast approach–avoid state, with τx about 0.5–1 s. **K** is the individual kernel
  (§5.6). With the cubic kernel, K = b·x − x³ + (d − w), and b is the same for every culture
  unless evidence forces otherwise.
- **The drive d has a direction.** Fury pushes toward approach and fear toward avoidance.
  Support (neighbours, depth, leaders, a camp behind), the social term built from comrades' acts
  (§5.4), shame and success add to it.
- **w gathers the two slow variables.** Spent stamina and depleted resolve both push x toward
  avoidance, but they recover very differently:

```
τF · dF/dt = threat − F                    seconds
τΦ · dΦ/dt = fury inputs − Φ               tens of seconds
τH · dH/dt = unanswerable fire − H         minutes
dW′/dt     = −(P − CP)             if P > CP
           = (W′max − W′) / τW′    if P < CP
τR · dR/dt = r · (1 − R) − (δF·F + δL·L + δH·H) · R
```

- **W′** drains while the power demand P of his action (fighting, running, carrying a shield
  uphill) exceeds his critical power CP. About 1–3 minutes of full-intensity fighting empties
  it. Below CP it refills, with a time constant of roughly 5–10 minutes when resting in a lull
  (Skiba et al. 2012). These are published physiology, not tuned numbers.
- **R** lies between 0 and 1. It drains with sustained fear F, comrades seen falling L and
  helplessness H. Its recovery rate r is zero while a threat is within reach or while the man is
  fleeing, and rises with leaders and standards in view. τR is minutes to tens of minutes.
- **Noise** σ·ξ comes from the counter-based RNG, so runs stay deterministic. Integrator state
  is f32, or 16-bit fixed point where bandwidth matters. 8-bit quantisation stalls slow
  dynamics, because an increment smaller than one step per tick vanishes. Memory is not the
  constraint.

### 5.2 Pulses, lulls and routs

- **A pulse.** A surge carries a man into contact. Fighting spends W′, which raises w until x
  drops, and he steps back, still facing the enemy. In the lull W′ refills, w falls and x climbs
  until he surges again. This is a relaxation oscillation, like the spike and refractory period
  of a nerve cell in the FitzHugh–Nagumo model. Its clash and lull lengths come from W′'s
  physiology. Coupling between neighbours (§5.4) synchronises the men's cycles into the unit's
  clashes and lulls.
- **A rout.** Sustained fear and losses drain R over minutes to hours. Low R raises the floor of
  w, so x stays on the avoid side even after W′ has refilled. A man who turns and runs changes
  what he perceives: he no longer sees the enemy's front, but he hears fighting behind him, the
  strongest fear input there is. That keeps F high and holds R's recovery at zero. A rout
  therefore does not heal by itself. It ends when the man reaches safety and R recovers (§10).
- **A recoil is not a rout.** Stepping back a few paces while facing the enemy, as a lull
  begins, and turning to flee are different acts. Comrades see the acts, never x (§5.4).

### 5.3 The contact-line loop

Behaviour at the line is a closed loop. x sets movement; movement sets distance and contact;
distance and contact set threat, exertion and success; and those set x. The loop has three
regimes:

| Regime | What it looks like | Where it belongs |
|---|---|---|
| **Standoff** | A stable gap of a few metres; missiles and insults | Lines that will not close; the lulls themselves |
| **Pulsing** | Surge, clash, recoil, lull, surge | The normal infantry fight (S2, M15) |
| **Lock-in** | Continuous contact until one side's R collapses | Men who cannot step back: encircled, compressed, or pinned against a river |

Which regime appears depends on the fear and fury gains, on W′ capacity and recovery, and on
coupling. The Slice 0 toy maps the regions. The model is acceptable only if historical troop
profiles fall in the pulsing region over a non-trivial range of parameters, not on a knife-edge.

### 5.4 Contagion: observable acts, read against the order

A soldier never reads a comrade's x. Agents read only each other's observable state (see
[06](06-architecture.md)), so the social term is built from **acts**: a comrade stepping back
while facing the enemy, turning his back, running, dropping his shield, falling, or surging
forward. Tent-mates weigh more than strangers.

Each act is read against the unit's **current order**. Under "retire" or "relieve", stepping
back is expected and alarms no one. Under "hold" it is mildly alarming. A turned back is
alarming under any order, except a feigned flight the men have been told about, which only
trained units can perform. This is how disciplined units can retire under pressure without
reading their own withdrawal as flight. Undrilled men lack the shared frame, so any backward
step can look like the start of a rout.

### 5.5 Avalanches: snap or crumble

Thresholds vary by temperament, wounds, position in the formation and leader proximity, so a
collapse spreads like an avalanche through a disordered medium (the random-field Ising model,
Sethna et al.). That theory predicts that a **narrow** spread of thresholds (uniform troops)
makes a unit **snap** in one system-wide avalanche, while a **wide** spread (a few heroes and
many followers) makes it **crumble** in many small breaks that officers can patch. Formations
are small and thin, so the transition is blurred. M16 ([07](07-validation.md)) therefore
compares avalanche-size distributions instead of labelling units.

The classical Gallic pattern, a tremendous first charge and then collapse, is *not* written in
through a culture parameter. Livy's Fabius, at Sentinum, says the Gauls' bodies cannot bear toil
and heat, so their first fights are "more than men's, their last less than women's" (10.28), a
Roman stereotype. That is an explanation by endurance. A fury-driven charge spends W′ fast, and
a line without relief spends it all at once, so the whole line is exhausted together. The Roman
line rotates, so its men are never all spent at the same moment.

### 5.6 Parsimony: one signature, four mechanisms

Sudden collapse with hysteresis can come from four places:

1. the individual kernel, if a single man is bistable;
2. neighbour coupling: a coupled population is collectively bistable even when each man
   responds smoothly;
3. the change in what a man perceives once he turns his back;
4. slow resolve R.

The last three are in the model for other reasons: coupling for contagion and rear-first
collapse, the perception change because perception is egocentric anyway, and R because rallying
is rare, slow and needs leaders. Whether the individual also needs bistability is open. The
Slice 0 toy compares three kernels, from simplest to richest: a smooth response, a
two-threshold switch, and the cusp. We keep the simplest that reproduces rear-first collapse,
the casualty asymmetry (S1) and pulsing (S2) over a non-trivial region of parameters.

Every other psyche variable earns its place the same way. Helplessness H exists for Carrhae and
Panormus, and shame S for the velites' observed bravery and the eagle at risk. Each enters the
code only when the slice that needs its pattern arrives (see [08](08-roadmap.md)). How the
surviving parameters are fixed and calibrated is in [07](07-validation.md) §8.

## 6. Arbitration: drives, orders, discipline

What a soldier *wants*, x, is combined with what he has been *told*: the unit's current order
and his slot in the formation.

- **Desire from x.** Strongly positive: charge or pursue. Moderately positive: close and fight.
  Near zero: hold or hesitate. Moderately negative: give ground, still facing the enemy.
  Strongly negative: turn and flee.
- **Discipline** is the weight of orders against that desire. It governs:
  - holding a slot when x would make him surge or lag;
  - not pursuing when the order is to halt;
  - not looting;
  - carrying out unnatural drills: feigned retreat, wheeling under threat, retiring through
    intervals.
- **Discipline** comes from training and culture (Roman *disciplina*: fear of punishment, of
  decimation, of the *fustuarium*), from internalized professionalism, and from the officers
  physically present. The optio's staff at the rear raises the cost of slipping away.
- **The order never pushes x directly.** It acts in arbitration, and it frames how comrades read
  each other's acts (§5.4). So Roman staying power does not rest on one double-counted bias. It
  has several sources, each with a historical anchor: discipline here, support in the drive,
  relief that refills W′, and a reserve of resolve built by the unit's confidence history (§7).
  A disciplined man ordered to hold holds while x is moderately negative. A disciplined man *in
  flight* still runs, but near officers and standards his R recovers faster, so he rallies
  sooner.

## 7. Four dimensions of quality

These are distinct, because history treats them as distinct:

| Dimension | Level | What it does | Example |
|---|---|---|---|
| **Drill / training** | Unit | Formation precision, transition speed, which complex maneuvers are available and how reliable they are, and a shared frame for reading comrades' movements (§5.4) | Scipio drilling his army in Spain before Ilipa |
| **Discipline** | Unit culture and individual | Obedience weight, restraint in pursuit and looting | Roman legions; Hannibal's Africans |
| **Experience** | Individual | Habituation, appraisal accuracy, pacing his own effort, self-organization without orders, skill | Sabis: veterans formed up by themselves. Pharsalus: they halted mid-charge to breathe. |
| **Cohesion** | Social | Strength of ties; loss when units are mixed, understrength or filled with replacements | The tent-group; a Gallic clan |

Plus a unit-level **confidence history**:

- Victories breed a belief in invincibility (Hannibal's veterans).
- Defeats breed fear of a specific enemy. The Romans avoided battle with Hannibal for years after
  Cannae.
- Disgrace can breed shame-driven resolve. The *legiones Cannenses*, Cannae survivors exiled to
  Sicily, fought for Scipio at Zama.
- Too many campaigns breed war-weariness. Caesar's veterans mutinied in 47 BC.

Legio X under Caesar: high in all four, intense trust in the commander, rivalry with other
legions. A newly raised legion: decent drill on paper, low experience, low confidence, cohesion
still forming.

## 8. Leaders inside the psyche

- **The centurion in front.** Example ("he is still fighting"), audible commands. A large local
  support term. His death is a shock to everyone who sees or hears it, and the news then travels
  as a rumor.
- **The optio and file closers at the rear.** They block the "slipping away" channel: physical
  hindrance plus shame.
- **The signifer.** An alignment reference and a rally beacon. The aquilifer of the Tenth leaping
  into the surf in Britain: "Jump, soldiers, unless you wish to betray the eagle!" (*BG* 4.25).
- **Mounted officers and the commander.** A morale field over their visible radius. Caesar at the
  Sabis took a shield from a man in the rear, went to the front, and called the centurions by
  name. That is a huge local morale effect, at real risk to his life. The commander's death or
  flight is a strategic shock: Perseus at Pydna, Crassus' son's head at Carrhae.

## 9. Behavioral repertoire

Hold slot · advance with unit · close and strike · fight defensively (shield up, minimal
exposure) · throw missile · loose missiles (archers, slingers) · step back / give ground ·
retire in order · help wounded comrade to the rear (thins the line) · drop shield / pick up
weapon · close a gap · rally to a standard or officer · freeze · flee · surrender · pursue ·
loot · feign flight (trained units, under order) · relay a rumor or order · shout or chant.

## 10. Rout, pursuit, rally, surrender

- **Rout.** The fleeing man drops his shield with a probability set by culture and panic: faster,
  but defenceless. He flees *away from the threat*, *toward a refuge*, and *with the herd*.
  Herding increases in panic (Helbing's panic model), which produces clogging and crushing at
  bottlenecks: rivers, camp gates, encircled masses.
- **Slaughter.** Fleeing men are struck from behind with no shield and cannot fight back.
  Cavalry kills at speed. Most casualties happen here (stylized fact S1, see
  [07](07-validation.md)).
- **Pursuit.** Pursuers are driven by Φ (fury) and greed and restrained by discipline. They are
  disordered, which makes them vulnerable to a counter-charge. Over-pursuit emerges from this.
- **Rally.** R recovers once the threat is gone (distance, no pursuers), faster when a leader or
  standard is in view. As R recovers, w falls and x climbs back, sooner when neighbours are seen
  rallying (§5.4). Drill and experience raise the recovery rate. Rallied men form ad-hoc groups
  around standards.
- **Surrender.** Encircled, exhausted, no escape, and an *expectation of quarter* that depends on
  culture and enemy. Captives leave the battle as prisoners. About 10,000 of the Cannae captives
  came from the camps.

## 11. Animal minds

- **Horses** refuse to charge into a dense, steady wall. That is why steady infantry usually
  beats cavalry frontally (Pharsalus). They panic at elephants and camels unless habituated.
  They tire, and a wounded horse bolts.
- **Elephants** have their own fear and rage. Pain, fire, trumpets or a dead mahout send them
  into a rampage that tramples friend and foe alike.

## 12. Validation hooks for the psyche

Micro-scenarios with expected outcome bands. Full list in [07](07-validation.md):

- A flank charge on a recruit cohort versus a veteran cohort: the probability of rout, and
  whether the veteran cohort turns to face.
- First exposure to elephants versus habituated troops.
- A feigned flight by Numidians versus by untrained horse: does it stay a feint?
- The Pharsalus halt: do veterans self-pace their charge, and do recruits?
- The Sabis: veterans forming up without orders after a surprise attack.
- Gergovia: an eager assault that does not hear a recall sounded across a valley, and outruns
  the officers who try to hold it back.
- The Trebia breakout: a disciplined mass that keeps its cohesion in defeat.
- Crumble or snap: avalanche-size distributions for narrow and wide threshold spreads.
- The Slice 0 behaviour toy: two short lines in one dimension. Do a stable standoff, pulsing
  combat, rear-first collapse and the casualty asymmetry appear over a non-trivial region of
  parameters? Which kernel is the simplest that does?
