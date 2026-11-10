# Cannae — Vision

A real-time, soldier-level simulator of land battles from the First Punic War (264 BC) to the
death of Augustus (AD 14). Graphics are deliberately minimal: top-down circles. The investment
goes into behavior. Every soldier is an agent with a body, equipment, a psyche and a place in a
chain of command, and the battle's outcome emerges from those agents.

## The thesis

Ancient battles were decided by psychology, cohesion and command much more than by kill rates.

- Ardant du Picq: the victors lost few men, and the vanquished lost most of theirs in flight.
  Hand-to-hand fighting in a formed line killed relatively few. The slaughter began when one side
  turned its back.
- Philip Sabin and Adrian Goldsworthy: Roman battles lasted hours. They did not grind
  continuously. They pulsed: short, intense clashes, then lulls with the lines a few metres apart
  trading missiles and insults, until one side surged or gave way.
- The Roman *triplex acies* exists because of these facts. It only makes sense if men tire, lose
  heart and need relief. A simulator that has no fatigue or morale cannot show why the Roman
  system worked.

So the core of the system is not a combat formula. It is:

1. a **body**: stamina, wounds, encumbrance, crowd pressure;
2. a **psyche**: fear, resolve, trust in comrades and leaders, shame, fury;
3. **discipline and experience**: how strongly training and orders override individual drives,
   and how accurately a man reads danger;
4. **command and control**: officers who see only what is in front of them, and orders that
   travel by voice, horn, standard, runner and rider, with delay, loss and misreading.

Physical combat (weapons, armour, missiles, shoving) is the substrate everything else runs on.

## Design pillars

1. **Emergence over scripting.** There is no "units rout at 30 % casualties" rule. A rout is a
   cascade of individual decisions: the rear ranks start to slip away, the front ranks lose the
   support behind them, a standard goes down, someone shouts that the enemy is behind.
2. **Every soldier is an agent, and agents are cheap.** We simulate 1:1, including about 136,000
   men at Cannae. This needs hierarchical decisions (soldier, file, century, maniple or cohort,
   legion, wing, army), staggered thinking rates, and multi-resolution perception.
3. **Information is local and imperfect.** No one sees the whole battle, not even the
   commander. Commanders act on beliefs that are partial, stale and sometimes wrong. At Philippi,
   Cassius killed himself because he misread a dust-obscured scene.
4. **Historically calibrated, not historically scripted.** Parameters come from the sources,
   experimental archaeology and scholarship. They are validated against a corpus of battles in
   Monte Carlo runs. Calibration and test battles are kept apart so we do not overfit to Cannae.
5. **The model is independent of rendering.** The simulation is a headless library. Viewers and
   batch runners are thin clients.
6. **Deterministic.** A seed plus a scenario reproduces a battle bit for bit. That gives us
   replays, regression tests, and a way to find behavior bugs by bisection.
7. **A historical hypothesis testbed.** Contested mechanics are switchable modes, not hard-coded
   assumptions: how line relief worked, whether there were maniple intervals, *othismos*
   (shoving), frontage per man. The simulator can then help argue those questions.

## Acceptance tests

**A1 — Cannae.** Run with the historical setup, a high (but not certain) share of seeds must
reproduce the battle's structure:

- Hasdrubal's Iberian and Gallic heavy horse breaks the Roman citizen cavalry on the river flank.
- The Numidians pin the allied cavalry on the other wing. That cavalry breaks when Hasdrubal
  arrives in its rear.
- The Gallo-Iberian crescent holds for a time, then gives way under the deep Roman mass and is
  driven in (Polybius 3.115; Livy 22.47). The Roman maniples crowd into the gap and lose their
  order.
- The Libyans, posted half at each end of the line and held back as a reserve (Polybius 3.113),
  face inwards and attack the Roman flanks once the Romans are between them. Their formation is
  not recorded; the deep columns of many reconstructions are a scenario variant.
- Hasdrubal's cavalry closes the rear. The Roman mass is compressed and destroyed.
- Casualties fall within the range the sources give. Polybius: about 70,000 Roman dead and
  10,000 captured, against 5,700 Carthaginian dead, 4,000 of them Gauls. Livy: 48,200 Roman dead
  and 19,300 captured, against about 8,000 Carthaginian dead.

A minority of seeds must go differently, for the right reasons. The Gauls give way too early and
the Romans break through, as the Roman centre did at Trebia. Or Hasdrubal's horsemen over-pursue.
Hannibal's plan was a gamble, and the simulation must treat it as one.

**A2 — Pre-battle indeterminacy.** Looking at two armies before the battle, it should not be
obvious how it will end. This needs to be measurable:

- In balanced scenarios, the side favored by a naive predictor (numbers × troop quality) should
  win a clearly non-certain share of seeds.
- The spread of outcomes comes from mechanisms you can name: a wing collapsing, the timing of a
  reserve, a commander's death, a rumor. It should not be noise.
- Blind prediction trials: you look at a deployment, predict the result, and we track how well
  calibrated your predictions are.

**A3 — The battle corpus.** A suite of battles chosen because different factors decided each
one: terrain, weather, surprise, elephants, horse archers, the experience gap, command failure.
Each battle has expected outcome bands (see [07-validation.md](07-validation.md)). The Trebia,
Lake Trasimene, Zama, Cynoscephalae, Pydna, Magnesia, Carrhae, Sabis, Pharsalus, Munda and
Teutoburg are among them. Caesar alone supplies a dozen test cases.

## Scope

- **Period:** 264 BC to AD 14. Pyrrhus (280–275 BC) and Idistaviso (AD 16) sit just outside the
  window and are candidates for calibration or stretch scenarios.
- **In scope:** field battles, ambushes, marching columns caught by an enemy, river crossings,
  field fortifications (ditches, stakes, camp ramparts as refuges).
- **Out of scope for now:** naval battles, siege warfare, and campaign-level strategy. The
  campaign does enter as initial conditions: fatigue, hunger, supply, numbers.

## Document map

| Doc | Contents |
|---|---|
| [01-armies.md](01-armies.md) | Every army and troop type in scope: organization, role, fighting style, discipline profile |
| [02-equipment.md](02-equipment.md) | Weapons, armour, shields, mounts and elephants, as physical properties the simulator uses |
| [03-formations-tactics.md](03-formations-tactics.md) | Formations, drills, battle mechanics and tactical patterns, including contested ones |
| [04-psyche.md](04-psyche.md) | The behavioral core: body, psyche, discipline, experience, contagion, rout, rally |
| [05-command.md](05-command.md) | Officers, beliefs, orders and signals, commander personalities |
| [06-architecture.md](06-architecture.md) | Tech stack, data layout, tick structure, performance budget on an M2 |
| [07-validation.md](07-validation.md) | Validation method, stylized facts, battle corpus, statistical criteria |
| [08-roadmap.md](08-roadmap.md) | Phases, the knowledge base and sources, open questions |
