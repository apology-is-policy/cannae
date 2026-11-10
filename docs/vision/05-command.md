# 05 — Command and Control

Officers are agents with bodies, eyes, horses and voices. No one has a god's-eye view. A
commander acts on a **belief state** built from what he and his messengers have seen. That belief
is partial, stale and sometimes wrong. Orders are physical messages that travel at the speed of a
runner, a rider or a sound, and they can be lost or misread.

## 1. Hierarchy (configurable per army)

**Roman, mid-Republic:**

```
Consul (or praetor, dictator, proconsul)
 ├─ wing commanders (the other consul or proconsul, legates)
 │   ├─ tribuni militum (6 per legion)        ├─ praefecti sociorum (allied alae)
 │   │   └─ centurions (two per maniple)  →  optio, signifer, cornicen
 │   │        └─ files (file leader … file closer)
 │   └─ praefectus equitum / decurions (cavalry)
 └─ staff: friends, lictors, couriers, scouts (exploratores, speculatores)
```

- **Late Republic.** Legates command legions or groups of legions. The primus pilus and the
  centurion hierarchy carry most of the tactical weight. Antesignani and cavalry prefects command
  the auxiliary contingents.
- **Carthage.** Hannibal and his brothers, wing commanders (Hasdrubal, Hanno or Maharbal),
  national contingent leaders with interpreters, Numidian chieftains.
- **Hellenistic.** The king and his guard, strategoi, phalanx commanders, cavalry ilarchs.
- **Gauls and Germans.** War leader, chieftains, clan heads, retinues. Authority is persuasive
  rather than coercive. Orders spread by chieftains' example and by shouting.

Cannae, for example:

- **Romans.** Paullus on the right with the citizen cavalry. Varro on the left with the allied
  cavalry. Servilius and Atilius in the center (Polybius).
- **Carthaginians.** Hasdrubal on the left. Hanno (Polybius) or Maharbal (Livy) on the right.
  Hannibal and Mago in the center.

## 2. The officer agent

| Component | Contents |
|---|---|
| **Perception** | Same model as soldiers, with better position: mounted (eye height about 2.4 m versus 1.6 m), often on rising ground, able to ride to look. Dust, fog and terrain still occlude. |
| **Belief map** | Estimates of friendly and enemy units: position, state, intent, each with a confidence and a timestamp. Updated from his own sight, from reports and from inference ("cavalry dust beyond the hill"). Stale beliefs decay toward uncertainty. |
| **Plan / intent** | Received from his superior (*mission intent* plus specific tasks), plus his own sub-plan |
| **Personality** | Aggression, caution, initiative (how far he deviates from orders when he sees an opportunity), competence (accuracy of his situation assessment, quality of his timing), charisma (morale field strength and radius), personal risk-taking, communication style (micromanaging versus mission command), reserve doctrine |
| **Body** | Can be wounded or killed. Can panic. A fleeing commander is a strategic event. |

**The decision rhythm is hierarchical.** Centurions act continuously. Tribunes and wing commanders
re-evaluate every few seconds to tens of seconds. The commander re-evaluates every 30–120 seconds,
or immediately on key events: a report arrives, a wing breaks.

## 3. Orders and their transport

An order is a structured message. Examples:

```
{ to: LegioV/cohorts 1-4, verb: ADVANCE_TO, arg: position, trigger: "when cavalry engaged",
  from: legate, issued_at: t, fidelity: f }
```

Verbs:

- **Movement:** advance, halt, hold, retire.
- **Line relief:** relieve, pass lines.
- **Orientation:** wheel, face left/right/rear.
- **Formation:** form orbis, form testudo, form line, form column.
- **Action:** charge, throw pila, skirmish, screen, pursue, do-not-pursue, rally, retire to camp.
- **Cavalry and trickery:** envelop, feign flight, ambush-hold, ambush-spring.
- **Reporting:** report status.

| Channel | Speed / range | Failure modes |
|---|---|---|
| **Voice** | About 10–30 m in battle din | Masked by noise. Garbled. Unheard by distant files. |
| **Horn signals** (cornu, tuba, bucina; Celtic carnyx) | Several hundred metres; near-instant | A small vocabulary (advance, halt, retire, attack). Masked by din. Shadowed by terrain: at Gergovia a wide valley kept most legions from hearing the recall (*BG* 7.47). Ambiguous about who it is meant for. Can be forced: at Thapsus the soldiers made a trumpeter sound the charge. |
| **Standards** (signum movements) | Line of sight | Need visibility. Dust. Misread. |
| **Runners** | About 3–5 m/s | Killed, lost, delayed by crowds. Arrive with a stale order. |
| **Riders / aides / tribunes** | About 6–12 m/s | Same failures, faster. The tribune may interpret the order himself (good or bad). |
| **Commander in person** | His riding speed | The highest fidelity and morale effect, but his attention leaves the rest of the field, and he risks death |
| **Pre-arranged triggers** | Instant, local | "When the Gauls fall back to the line of the Libyans, turn inward." Executed by the subordinate's *own perception* of the trigger, which may come early, late or never. |

**Fidelity.** Every hop can degrade an order. The chance depends on the order's complexity, the
messenger's and receiver's competence, a language barrier (Hannibal's coalition) and stress. A
degraded order becomes a different verb, a different target, or the wrong timing.

**Reports flow upward through the same channels**, with the same delays. The commander's picture
of a wing is minutes old. He believes it is current.

## 4. Planning before the battle

Scenarios can include a planning phase or a historical plan. A plan is:

1. **A deployment.** Lines, wings, reserves, ambush parties, the commander's own position.
2. **Conditional tasks with triggers.** Examples: Hasdrubal's "break their horse, then ride
   behind the Roman army"; Scipio's "velites lure the elephants into the lanes"; Caesar's "fourth
   line charges when Pompey's horse closes".
3. **Contingencies.** What to do if the center breaks, if the camp is threatened, when to retire.

Commanders with high *initiative* rewrite the plan in flight. Commanders with low initiative
follow it into disaster.

## 5. Commander personalities (presets to calibrate)

| Commander | Style traits |
|---|---|
| **Hannibal** | Deception, ambush, combined arms, precise timing, uses terrain and weather (the ambush in fog at Trasimene; the Romans lured across the icy Trebia unfed). At Cannae the sun troubled neither side (Polybius 3.114; Livy 22.46); the dust-laden wind in the Romans' faces is Livy's, not Polybius'. Positions himself in the critical sector (Cannae center). Very high charisma with mercenaries. |
| **Scipio Africanus** | Innovation, drill, deceptive deployment (Ilipa), adapts to the enemy's system (lanes at Zama). Personal courage. |
| **Caesar** | Speed, very high personal intervention and risk-taking (the Sabis, Munda), exceptional rapport with his soldiers, masterful use of reserves and the hidden line (Pharsalus). Writes his own legend. |
| **Marius** | Training, patience, habituating his troops, choosing the moment and conditions (heat at Vercellae). |
| **Sulla** | Field works, steady control, deception |
| **Pompey** | Methodical, defensive and receiving. Plan built around his cavalry (Pharsalus). |
| **Varro** | Aggressive, relies on depth and mass *(historical reputation debated)* |
| **Aemilius Paullus (216)** | Cautious. Wounded early at Cannae. |
| **L. Aemilius Paullus (168)** | Patient, methodical. Waited for the phalanx to disorder (Pydna). |
| **Flaminius / Sempronius Longus** | Rash, poor reconnaissance (Lake Trasimene, the Trebia) |
| **Crassus** | Poor reconnaissance, indecisive, trusted a treacherous guide |
| **Surena** | A combined-arms system built on horse archers, cataphracts and arrow logistics |
| **Antiochus III** | Leads the cavalry charge in person, over-pursues (Raphia, Magnesia) |
| **Philip V** | Impetuous. Engaged at Cynoscephalae with half his army unformed. |
| **Perseus** | Passive. Fled early at Pydna. |
| **Vercingetorix** | Strategic patience, scorched earth. Forced into decisive battle at Alesia. |
| **Ariovistus** | Bound by omens. Clever at harassing Caesar's supply. |
| **Arminius** | Knows his enemy's system. Ambush, terrain, weather. |
| **Labienus** | Cavalry and light-troop expert (Ruspina). Deserted Caesar to fight against him. |
| **Mark Antony** | Aggressive. At Philippi he stormed Cassius' camp. |
| **Cassius** | At Philippi he misread a dusty scene and the approach of friendly cavalry, believed all was lost, and killed himself. **The canonical test of the belief model.** |
| **Hasdrubal Barca** | Brave, unlucky. At the Metaurus he rode to his death when defeat was certain. |
| **Xanthippus** | A professional drill-master. Chose open ground for his elephants and cavalry. |
| **Viriathus / Jugurtha** | Guerrilla, feigned flight, ambush |
| **Tiberius / Germanicus** | Methodical, careful, logistics-minded (Augustan) |

## 6. Fog of war as a first-class feature

- The **viewer** can show the truth, or any officer's belief map side by side with the truth: a
  fog-of-war overlay. This is debugging and storytelling at once.
- **Misidentification.** Friendly cavalry approaching through dust may be read as enemy
  (Philippi). An enemy redeployment may be read as flight (Munda). A clash heard behind the line
  may be read as encirclement. Agents interpret *events*, so they can misinterpret them.

## 7. Player role (to decide)

The simulation must run fully autonomously for Monte Carlo testing, with every officer an AI. A
human could optionally take one officer's role, from commander down to a single tribune. He would
see only that officer's belief map and issue orders through the same channels with the same
latency. That gives the experience of commanding an ancient army, which no game has offered
faithfully. See the open questions in [08](08-roadmap.md).
