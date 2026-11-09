# 03 — Formations, Drill and Tactics

A formation is a **geometry**: slots in a local frame. It is also a **drill**: the transitions a
unit has been trained to perform between geometries, at a known speed and with a known error
rate. Soldiers steer toward their slots, but the psyche bends that steering: fearful men lag and
bunch, eager men surge. The formation the viewer sees emerges from soldiers trying to hold slots
under stress. It is never a rigid block.

## 1. Geometry parameters (per unit, per moment)

| Parameter | Typical values *(many debated)* |
|---|---|
| Frontage per man | Phalanx *synaspismos* (shields locked) about 0.45 m; *pyknosis* (close order) about 0.9 m. Roman close order about 0.9 m. Roman fighting order about 1.8 m per Polybius 18.30 *(debated)*. Barbarian masses are variable and crowd under pressure. |
| Depth | Phalanx 16 (up to 32 at Magnesia, 50 at Leuctra). Roman maniple about 6–10 ranks *(debated)*. At Cannae the Roman maniples were formed "many times deeper than broad" (Polybius 3.113). Germanic wedges are deep. |
| Interval between maniples or cohorts | Equal to the unit's frontage (the checkerboard / *quincunx*) or much smaller *(debated)*. A switchable mode. |
| Distance between lines | About 75–150 m *(estimated)* |
| File structure | A file leader and a file closer (Roman optio at the rear; Greek *ouragos*). Rear-rank officers matter for stopping men from slipping away. |
| Facing | Each soldier and each unit frame has one. Turning a formation to face a flank is a drill whose execution time and failure rate depend on training. |

## 2. Formation catalog

### Infantry

- **Triplex acies.** Manipular (hastati, principes, triarii) or cohortal (4–3–3 cohorts). The
  backbone of the Roman system.
- **Duplex and simplex acies.** Caesar used double lines and, when outnumbered, single lines.
  Ruspina began in a single line.
- **Quadruplex acies.** At Pharsalus, Caesar hid a fourth line of 6 cohorts behind his right to
  meet Pompey's cavalry, with orders to thrust their pila at the riders' faces.
- **Phalanx** (16 deep), with *synaspismos* for defence and the *epikampios* (a refused flank
  bent back).
- **Orbis** (a ring). Sabinus and Cotta (*BG* 5.33). Ruspina. A last resort that trades mobility
  for all-round defence.
- **Testudo.** Overlapped shields over the head and on the sides. Slow. For assaults on walls,
  and against Parthian arrows (Antony, 36 BC). Falls apart under shock or fatigue.
- **Cuneus / wedge.** The Roman *cuneus* is probably a dense assault column *(debated)*. The
  Germanic "boar's head" is a deep wedge.
- **Forfex** (pincers, against a wedge) and **serra** (saw: advance and retire). Both come from
  Vegetius, a late source; use with care.
- **Agmen quadratum.** A hollow square on the march with the baggage inside. Crassus at Carrhae.
- **Celtic or Germanic shield-wall mass.** The Helvetian "phalanx", with overlapping shields.
- **Open skirmish order.** Velites and light troops. Loose clouds that advance, throw, and fall
  back.

### Cavalry

- **Line**, a few ranks deep (Greek and Roman).
- **Wedge** (Scythian and Macedonian) and **rhombus** (Thessalian). Both are built for turning
  and penetrating.
- **Numidian swarm.** Dispersed, harass, flee, return.
- **Parthian combination.** Horse archers fix the enemy and wear him down. Cataphracts strike
  once he is disordered or crowded.
- **Cantabrian circle.** Javelin riders wheeling in a continuous loop past the enemy front.
- ***Hamippoi* mixing.** Light infantry running among the horsemen (Germans; Caesar at Pharsalus
  with the antesignani).
- Dismounted cavalry: the Roman equites at Cannae, Iberian horsemen.

## 3. Line relief: one mechanic, several hypotheses

The *triplex acies* is the point of the Roman army. How it actually worked is unknown, so the
simulator provides switchable mechanisms and lets the corpus discriminate between them:

| Mode | Mechanism | Proponents |
|---|---|---|
| **R1: line passage** | The whole first line withdraws through the intervals of the second, which advances into contact | Traditional reading of Livy 8.8 |
| **R2: pulsing combat with relief in the lulls** | Combat comes in short clashes. In the lulls, men drift back and the next line's maniples fill gaps or take over the front. | Sabin, Goldsworthy |
| **R3: rank rotation within the maniple** | Fresh rear ranks replace exhausted front-rankers inside each unit | Various |
| **R4: gap-filling** | The second line advances *into the intervals* of the first, making a continuous front, and the first line's maniples then retire | Variant readings |

The simulator does not have to *choose* among these. Each is a set of drill transitions plus
psyche thresholds. The validation suite tells us which mode, or which mix, reproduces the
historical patterns: duration, casualty asymmetry, how the Romans recover from local setbacks.
This is research value nobody else offers.

## 4. Battle mechanics the simulator must produce

1. **Deployment takes time.** A column becoming a line takes from tens of minutes to hours for
   large armies. An army caught deploying is vulnerable: the Trebia, Lake Trasimene, the Sabis,
   Teutoburg.
2. **Skirmish phase.** Light troops duel in front of the lines, then retire through gaps or
   around the flanks. Elephants and chariots are usually met here.
3. **The advance.** A steady pace to keep order. At some range, the charge at a run. Breath and
   order trade off against impetus: Pompey's men stood to receive Caesar's charge, and Caesar's
   veterans *halted on their own* halfway to recover (*BC* 3.92–93).
4. **The pilum volley** at about 10–20 m, then the sword charge. The shield-pinning effect, the
   disorder at the moment of contact, the war cry (*clamor*). Romans often advanced in silence
   and shouted at the moment of contact.
5. **The clash** (*concursus*). A short burst of very intense fighting at the contact line. Most
   men in the rear ranks are not fighting. They support, push, wait and fear.
6. **Lulls.** The lines separate by a few metres to tens of metres. Missiles and insults. Men
   recover breath. Both sides must find the will to close again. This has to *emerge* from
   stamina and fear thresholds. It must not be scripted.
7. ***Othismos*** (shoving). In phalanx and dense-mass fighting, rear ranks may physically push
   *(debated: literal mass shove versus metaphor)*. A switchable mode on a crowd pressure model.
8. **Collapse.** It begins at the rear and the edges: men slipping away. Front ranks lose support
   and pressure from behind. A gap opens. A local rout spreads along the line (see
   [04](04-psyche.md)).
9. **Pursuit.** Most killing happens here, done mainly by cavalry. Pursuers are themselves
   disordered and may over-pursue, loot, or be counter-attacked.
10. **Rally and re-tasking.** Disciplined cavalry that wins its wing rallies and turns onto the
    enemy rear. Hasdrubal at Cannae, Laelius and Massinissa at Zama. Undisciplined cavalry rides
    off the map: Antiochus at Magnesia.
11. **Refuges.** A fortified camp, a hill, a river. Fugitives head for them. At Magnesia, the
    tribune at the camp turned the fleeing men around. After Cannae, about 10,000 men were
    captured in the camps.
12. **Surrender.** Encircled men may surrender. The likelihood depends on culture, whether
    quarter is expected, and exhaustion.

## 5. Tactical patterns (emergent from orders and the psyche, never hard-coded)

| Pattern | Historical anchor | What must exist mechanically |
|---|---|---|
| **Double envelopment** | Cannae | A center that yields without breaking (drill + discipline + Hannibal's presence). Wing troops with timed triggers. Cavalry that rallies and re-tasks. The compression of the encircled mass. |
| **Reverse Cannae / echelon** | Ilipa (206 BC) | Weak allies in the center, held back. Strong wings advance and wheel outward. Deployment deception over several days. Timing an early deployment so the enemy fights unfed. |
| **Anti-elephant lanes** | Zama | Maniples aligned in columns rather than a checkerboard. Trumpets. Velites luring elephants down the lanes. |
| **Hidden reserve against cavalry** | Pharsalus | A concealed line, then a well-timed counter-charge into disordered cavalry that is busy flanking |
| **Fix and flank / initiative of subordinates** | Cynoscephalae (an unnamed tribune takes 20 maniples into the Macedonian rear); the Vosges (young Crassus commits the third line, *BG* 1.52); the Metaurus (Claudius Nero marches behind his own army to hit the enemy flank) | Subordinates who perceive opportunities locally and act within their commander's intent |
| **Ambush** | Lake Trasimene (fog, column), the Trebia (Mago in the watercourse), Aquae Sextiae (Marcellus with 3,000 in the rear), Teutoburg | Concealment, observation and perception, surprise as a psyche shock, units caught in column |
| **Feigned flight** | Numidians, Parthians, Viriathus; Juba's Saburra at the Bagradas (49 BC) | Only well-trained units can do it without turning it into a real rout. The pursuer's psyche (fury and greed) draws him into disorder. |
| **Horse-archer attrition** | Carrhae | Ammunition and resupply logistics. The helplessness that drains morale. Formation choice: compression helps cataphracts, dispersal helps archers. The answer: slingers, high ground. |
| **Uphill defence** | Bibracte, Zela, Mons Graupius (later) | Slope affects speed, fatigue, missile range and impetus |
| **Field fortifications** | Sulla at Chaeronea and Orchomenus (trenches against chariots and cavalry); Caesar on the Axona (flank trenches); Alesia | Terrain modifiers that can be created before the battle: ditches, stakes, palisades |
| **Over-pursuit** | Antiochus III at Raphia (217 BC) and again at Magnesia: he won his wing, chased it off the field, and lost the battle. At Zama, Laelius and Massinissa came back from their pursuit only just in time. | Pursuit urge versus discipline. The commander's own temperament when he leads a wing in person. |
| **Breaking out** | 10,000 Romans cut through the Carthaginian center at the Trebia and marched away in order | A disciplined body that keeps cohesion in defeat and moves as one |
| **Morale decapitation** | Flaminius killed at Trasimene; Perseus' early flight at Pydna; Hasdrubal at the Metaurus riding to his death | Visible leaders, rumors of their death that spread, leaders as single points of failure |
| **Misread signal** | Munda: Labienus moved troops to meet a threat, and the Pompeian line read it as flight and collapsed. Gergovia: the recall was not heard over the din. Thapsus: the soldiers forced a trumpeter to sound the attack. | Signals and movements as observed *events* that each agent interprets. Interpretations can be wrong. Signal range is finite. |

## 6. The march and readiness states

Scenarios can begin from any of these states:

- **Marching column**, with baggage, packs and a vanguard and rearguard.
- **Deploying**: columns turning into lines.
- **Deployed**: waiting. Hunger, thirst, heat and boredom accumulate. At Ilipa, Scipio kept the
  Carthaginians standing unfed.
- **Building camp**: tools in hand, armour off (the Sabis).
- **Resting in camp.**

Readiness determines how quickly men can fight and how badly a surprise attack hits them.
