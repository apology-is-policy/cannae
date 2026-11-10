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
   experimental archaeology and scholarship. As many as possible are fixed from independent
   evidence: physiology, load-carriage studies, reconstruction trials. Sensitivity analysis
   finds the few that matter, and history matching rules out implausible values for those.
   Cannae is the development scenario, so it cannot be the proof. The proof is a set of sealed
   battles the model was never tuned on (A3).
5. **The model is independent of rendering.** The simulation is a headless library. Viewers and
   batch runners are thin clients.
6. **Deterministic.** A seed plus a scenario reproduces a battle bit for bit. That gives us
   replays, regression tests, and a way to find behavior bugs by bisection.
7. **A historical hypothesis testbed, within limits.** Contested mechanics are parameters, not
   hard-coded assumptions: frontage per man, maniple intervals, the strength of *othismos*
   (shoving). One line-relief mechanism is built first, with a seam for the others. The
   simulator can show what is mechanically feasible (does a line passage jam, and when?) and
   how much an outcome depends on each assumption. Fitting battle outcomes cannot settle these
   debates on its own, and we do not claim it can.
8. **Parsimony.** A mechanism enters the model only when a named historical pattern cannot be
   reproduced without it. Many plausible mechanisms explain the same few patterns, and the
   corpus is a few dozen battles. Every extra knob makes a good fit mean less.

## The player

You set up a battle, historical or invented (now and then something a little crazy, just to see
how it plays out), and watch it unfold. At any moment you can pause and take over commanders,
from the consul down to a tribune. You give them orders, or small visual scripts: "when half of
these maniples are past this line, send the right-wing cavalry against their flank".

There are two modes:

- **Experimenter.** You see the truth, and your orders take effect at once.
- **Role-player.** You see only your officer's beliefs, and your orders travel by voice, signal
  and messenger, with the same delays and losses as everyone else's.

Without you, the simulation runs fully autonomously, so batches of seeds can run for testing.
Your inputs are recorded in the replay. Intent, plans and the command AI are described in
[09-intent-and-plans.md](09-intent-and-plans.md).

## Acceptance tests

Cannae is the **development scenario**. We will watch it constantly while building, so passing
it is a requirement, not evidence. The evidence comes from battles the model was never tuned on
(A3).

**A1 — Cannae.** Run with the historical setup, a high (but not certain) share of seeds must end
the way the battle did. "The way it did" means what survives every reading of the sources, not
one reconstruction:

- Hasdrubal's Iberian and Gallic heavy horse breaks the Roman citizen cavalry on the river flank.
- The Numidians pin the allied cavalry on the other wing. That cavalry breaks when Hasdrubal
  arrives in its rear.
- The Roman infantry pushes into the Carthaginian centre, crowds together and loses its order.
- The Libyans, posted half at each end of the line (Polybius 3.113), strike both of its flanks.
  Cavalry closes the rear.
- No formed Roman body of several thousand men gets out. The encircled mass is compressed and
  destroyed.

The Gallo-Iberian centre may hold or break. Polybius (3.115) says it held for a time, then gave
way; Livy (22.47) has the Romans pressing on through a column of fleeing men until no one
resisted them. The trap closed anyway. Whether a centre that breaks still serves the trap is a
real historical question. The simulator reports the answer; it does not presume one.

The Libyans' formation is not recorded. The deep columns of many reconstructions are a scenario
variant.

**Outcome classes**, computed from the event log by tested classifier code:

| Class | Definition |
|---|---|
| **(a) Encirclement** (historical) | The structure above. Sub-classes: **(a1)** the centre holds until the trap closes; **(a2)** the centre breaks, and the trap closes anyway. |
| **(b) Victory without a pocket** | The Romans lose and flee, but the rear is never closed and no formed body gets out |
| **(c) Breakout** | A formed Roman body of several thousand leaves the field in order, as 10,000 men did through the Carthaginian centre at the Trebia (Polybius 3.74) |
| **(d) Other** | A Roman victory, a draw, anything else |

**What passes:**

- **Class (a) is the most common outcome, but not a certainty.** The design target is
  0.6 ≤ P(a) ≤ 0.95. That is a target, not a measurement: history gives one draw.
- **The minority outcomes respond to the right causes.** A rare outcome cannot be pinned down
  with a few hundred seeds: with 100 seeds, a 3 % outcome fails to appear about one time in
  twenty. So A1 tests how the classes move when a cause is varied:
  - P(c) rises as the Carthaginian centre gets weaker. Dertosa (215 BC) is the contrast: there
    the Iberian centre gave way at the first clash, the pila barely thrown, and a similar plan
    failed (Livy 23.29).
  - P(c) rises with the Roman infantry's cohesion in defeat. The Trebia is the contrast.
  - P(rear closed) falls as Hasdrubal's cavalry advantage shrinks, or as his horsemen become
    likelier to over-pursue.
- **The authored plan does not carry the result.** Polybius records no orders for Cannae, and a
  scripted "Hannibal plan" would write the outcome into the input. A1 runs at three authoring
  tiers, and all three are reported: **tier 0** is deployment plus generic doctrine; **tier 1**
  adds each commander's stated purpose; **tier 2** adds explicit triggers. The claim is made at
  tier 0 or 1.
- **Casualties fall within the sources' envelope.** They are compared only after a fixed
  stopping rule and a fixed mapping from simulated states (dead, wounded, captured, fled) to the
  sources' categories, because the source totals include the pursuit and the capture of the
  camps. Polybius: about 70,000 Roman dead and 10,000 captured, against 5,700 Carthaginian dead,
  4,000 of them Gauls. Livy: 48,200 Roman dead and 19,300 captured, against about 8,000
  Carthaginian dead.

**A2 — Pre-battle indeterminacy.** Looking at two armies before a battle, it should not be
obvious how it will end. The uncertainty must come from the battle, not from noise.

- **A strong predictor.** We train a predictor on thousands of small simulated battles, using
  only what is observable before the battle. A2 is its residual uncertainty on balanced
  scenarios. On lopsided scenarios it must be confident and right: indeterminacy must not mean
  noise.
- **Variance decomposition.** Each kind of randomness has its own RNG stream: hidden traits
  drawn at setup, combat dice, perception noise, command and messengers. Re-randomising one
  stream with the others frozen tells us how much of the outcome was fixed at setup, how much
  is dice, and how much is command friction. "Not noise" becomes a number.
- **Predictability over time.** Re-run during the battle, the predictor should grow more
  certain in steps at identifiable events (a wing breaking, a reserve committed, a commander
  falling), rather than drift smoothly.
- **Balanced scenarios from a generator.** Historical battles are ones both commanders chose to
  accept, which is a selection effect. "Balanced" is defined by a scenario generator instead.
- **Blind trials.** You look at deployments and predict outcomes with probabilities, and we
  track your calibration. If you can predict perfectly, A2 fails.

**A3 — Sealed battles and the corpus.** A corpus of battles, chosen because different factors
decided each one: terrain, weather, surprise, elephants, horse archers, the experience gap,
command failure. The Trebia, Lake Trasimene, Zama, Cynoscephalae, Pydna, Magnesia, Carrhae,
Sabis, Pharsalus, Munda and Teutoburg are among them; Caesar alone supplies a dozen. Most of the
corpus is calibration material. Three to five battles are **sealed**: their dossiers (numbers,
deployment, troop ratings, commander profiles) are written from a fixed rubric and
hash-committed before the engine can run them, then run once, as pre-registered. They are scored
by the log-likelihood of the historical outcome against simple baselines. Details are in
[07-validation.md](07-validation.md).

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
| [08-roadmap.md](08-roadmap.md) | Vertical slices, what waits, the knowledge base, design reviews, open questions |
