# 04 — The Psyche: Body, Mind, Discipline, Experience

This is the heart of the project. Existing games, such as Total War and Field of Glory, model
morale as a **unit-level bar** that drains with casualties and flanking until it crosses a
threshold. We model **each soldier** as a small dynamical system. Soldiers are coupled through
what they perceive, through their social ties and through their leaders. Unit morale is never
stored anywhere. It is something you *measure* by looking at the men.

Three ideas carry the design. Each comes from outside game design:

1. **Fight or flight is a cusp catastrophe.** Zeeman (1976) modelled aggression as a cusp. Fear
   and rage are the control parameters, and behavior jumps suddenly between attack and flight,
   with hysteresis. An aroused soldier is *bistable*: small changes can flip him. A calm soldier
   responds smoothly.
2. **Rout cascades are avalanches in a disordered, coupled system.** This is the random-field
   Ising model with hysteresis (Sethna et al.). Soldiers with heterogeneous thresholds influence
   their neighbors. The *spread* of individual courage within a unit decides whether it crumbles
   gradually or **snaps** all at once. This gives us a theory-backed knob, and a testable
   prediction.
3. **Stamina follows exercise physiology.** The critical-power / W′ model: an anaerobic reserve
   that drains above a sustainable power and refills below it. Fighting at full intensity
   exhausts a man in minutes. Lulls and line relief emerge from this. They do not have to be
   scripted.

## 1. Agent pipeline (per soldier)

```
          ┌────────────── orders / signals / leader presence ───────────────┐
          ▼                                                                 │
Perception ──► Appraisal ──► Psyche dynamics (cusp) ──► Arbitration ──► Action ──► Motor/Combat
   ▲  (sight, sound,   (threat, support,   (x: approach↔avoid;   (drive vs order,   (steer, strike,
   │   touch, rumors)   helplessness...)     F, R, fury, shame)    via discipline)    throw, flee...)
   └──────────────────────────── body state (stamina, wounds, pressure) ◄───────────┘
```

Each stage runs at its own rate. Motor and combat run every tick (20 Hz). Perception, appraisal
and the psyche run at 2–5 Hz, staggered across agents. Expensive long-range perception samples
coarse fields (see [06](06-architecture.md)).

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
| W′ balance (anaerobic reserve) | Drains when power demand > CP (fighting, running, carrying a heavy shield up); refills below CP. Time constant about 1–3 minutes of full-intensity fighting. |
| Glycogen fatigue | Slow. Lowers CP over hours. Restored only by food and long rest, i.e. not within one battle. |
| Heat load and hydration | Sun, armour, exertion. Vercellae, Carrhae. |
| Wounds | Per body zone: severity, bleeding rate, functional impairment (weapon arm, legs, vision). Pain and shock. |
| Crowd pressure | From collisions. Support when it comes from behind, crush when it comes from all sides. Inability to raise weapons. |
| Load state | Pila left, shield intact or pinned or dropped, javelins, arrows |

### Dynamic mind

| Variable | Timescale | Role |
|---|---|---|
| **x**: behavioral state | Seconds | Position on the cusp surface. Strongly positive means attack or pursue. Moderately positive means hold and fight. Near zero means hesitate or give ground. Negative means withdraw. Strongly negative means rout. |
| **F**: fear (acute threat) | Seconds | Fast arousal from perceived threats. Decays when the threat ends. |
| **Φ**: fury (rage, bloodlust) | Seconds to minutes | Fuelled by contact, by success, by comrades killed, by the war cry. Drives the charge and the pursuit. Overrides restraint, which is how men end up looting. |
| **R**: resolve (morale reserve) | Minutes | Slow capital. Drained by sustained fear, casualties seen, helplessness and exhaustion. Refilled by rest, success, leaders and line relief. |
| **H**: helplessness | Minutes | Builds under fire you cannot answer: Carrhae, Panormus. Drains R quickly. |
| **S**: honor / shame pressure | Event-driven | Whether officers and comrades can see him, the standard at risk, rivalry with neighbouring units. |
| **B**: local beliefs | Event-driven | "Flanked left", "the centurion is down", "the enemy is breaking", "the camp is taken". Set by his own perception or by rumors, with a confidence level. |

All of this packs into about **32–48 bytes** per soldier with quantized u8/u16 fields.

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
- **Rumors.** Discrete messages passed between agents within earshot. Each has a content, a
  source and a confidence. They distort probabilistically: "the left is pushed back" becomes "the
  left is broken". Veterans discount rumors and look for themselves. Recruits amplify them.

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
- **Habituation.** Novel stressors are amplified, and familiar ones are damped per stressor class.
  The Romans facing elephants at Heraclea versus at Zama. Marius making his recruits watch the
  Teutones for days before letting them fight.

## 5. Psyche dynamics: the cusp

Each think tick:

```
a = support − threat + order_bias(discipline) + shame_term + success − exhaustion_term − H
b = arousal                     // grows with proximity of combat, noise, F + Φ; damped by drill
dx/dt = −(x³ − b·x − a)         // gradient flow on V(x) = x⁴/4 − b·x²/2 − a·x
```

- **Low arousal (b small).** One equilibrium. x tracks a smoothly. A man in the reserve line
  drifts gradually between calm and uneasy.
- **High arousal (b large, in the clash).** Two stable states, approach and avoid, with a fold
  between them. As a declines, x stays in the "fight" basin until the fold, then **jumps** to
  flight. A sudden break.
- **Hysteresis.** Once he has flipped to flight, a must climb well *above* the level at which he
  broke before he flips back. Rallying is harder than breaking, as it should be.
- **Drill damps b.** Habit makes actions less emotional, so trained men respond more smoothly.
  This yields a striking emergent prediction: **disciplined units degrade gracefully** (fighting
  withdrawal, orderly breakout as at the Trebia), while **fierce but undrilled units are highly
  bistable**: a tremendous charge, then a sudden collapse. This matches the classical pattern for
  Gallic armies without being written into anything.

**Coupling.** A soldier's a includes the observed x of his neighbours: their stepping back,
turning, running, or surging forward. It also includes the states of his tent-mates, weighted
more heavily. Contagion is not a special rule. It is the coupling term.

**Avalanches.** Thresholds vary by temperament, wounds, position in the formation and leader
proximity. So a collapse spreads like an avalanche through a disordered medium. The width of the
threshold distribution decides whether a unit **crumbles** (many small, local breaks that
officers can patch) or **snaps** (one system-wide avalanche). We will test this hypothesis
against the corpus.

## 6. Arbitration: drives, orders, discipline

What a soldier *wants*, x, is combined with what he has been *told*: the unit's current order
and his slot in the formation.

- **Discipline** is the weight of orders against drive. It governs:
  - holding a slot when x would make him surge or lag;
  - not pursuing when the order is to halt;
  - not looting;
  - carrying out unnatural drills: feigned retreat, wheeling under threat, retiring through
    intervals.
- **Discipline** comes from training and culture (Roman *disciplina*: fear of punishment, of
  decimation, of the *fustuarium*), from internalized professionalism, and from the officers
  physically present. The optio's staff at the rear raises the cost of slipping away.
- **The order term enters a** (order_bias) and the action selection. A disciplined man ordered to
  hold holds even when his x drifts toward zero. A disciplined man *in flight* still runs, but
  rallies sooner.

## 7. Four dimensions of quality

These are distinct, because history treats them as distinct:

| Dimension | Level | What it does | Example |
|---|---|---|---|
| **Drill / training** | Unit | Formation precision, transition speed, which complex maneuvers are available and how reliable they are, damping of b | Scipio drilling his army in Spain before Ilipa |
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
- **Rally.** x climbs back through the hysteresis when the threat is gone (distance, no
  pursuers), a leader or standard is visible, neighbours are rallying, and time passes. Drill and
  experience raise the chance. Rallied men form ad-hoc groups around standards.
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
- The crumble-or-snap prediction against the threshold distribution width.
