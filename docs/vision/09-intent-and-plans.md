# 09 — Intent, Plans and the Command AI

Intent is the hardest part of command to model. An officer has to know what his superior wants,
see what is in front of him, notice when the plan no longer fits, and do something sensible
about it. It is tempting to think only a large language model could do that. This document
argues that an algorithmic AI can, because ancient command was low-bandwidth and
recognition-based. It then describes the pieces: what the player does, what an officer sees, how
plans are written, how officers decide, where an LLM fits (outside the loop), how time works,
and how officers are tested.

The officer agent itself (perception, belief map, personality, orders and channels) is in
[05](05-command.md).

## 1. What ancient command asked of a general

- **Low bandwidth at the top.** After contact, a general made a handful of decisions: commit the
  reserve, send the cavalry, ride to a sector, retreat. Orders took minutes to arrive. With that
  much delay, plausibility matters more than brilliance.
- **The famous improvisations were recognitions.** At Cynoscephalae an unnamed tribune took
  "not more than twenty maniples", judged "on the spur of the moment what ought to be done", and
  fell on the Macedonian rear (Polybius 18.26). At the Metaurus, Claudius Nero took cohorts from
  his idle right wing, led them behind his own line and fell on the enemy's flank (Livy 27.48).
  Young Crassus committed the third line at the Vosges (*BG* 1.52). Each spotted a standard
  opportunity: an unengaged force within reach of an enemy flank or rear.
- **The hard layer is the middle one.** Low bandwidth is true of generals. It is not true of the
  thousand-plus centurion-level controllers who decide when to charge, relieve, face a flank or
  close a gap, nor of the unit frame beneath them ([03](03-formations-tactics.md) §1). Slice 1
  builds that layer first ([08](08-roadmap.md)).
- **The Roman failure at Cannae was mostly not a command failure.** It was soldier-level (the
  inward surge into the pocket) and structural (no reserve). Roman officers should try to react
  and go unheard.

## 2. The player

You set up a battle and watch. At any moment you can pause and take over commanders, from the
consul down to a tribune, and give orders or small visual scripts: "when half of these maniples
are past this line, send the right-wing cavalry against their flank". Scripts compile to the same
plan representation the AI uses (§4).

- **Experimenter mode.** You see the truth, your orders take effect at once, and a script's
  trigger is evaluated on the truth.
- **Role-player mode.** You see only your officer's beliefs. Your orders travel by voice, signal
  and messenger, with the same delays and losses as everyone else's, and a trigger is evaluated
  on what the assignee can perceive, so it may fire early, late or never.

Player inputs (pauses, takeovers, orders, scripts) are recorded in the replay, so a run with a
human in it replays exactly ([06](06-architecture.md) §9).

## 3. The scene graph: what an officer sees

An officer does not see 50,000 men. He sees **bodies**: a formed maniple, a mass of Gauls, a
cloud of horsemen.

- **True bodies are computed once per side,** by grouping men from their observable state:
  position, spacing, facing and motion. Each body has a centroid, a facing, a frontage axis, a
  depth, a composition, and a state: formed, disordered or routing, read from the irregularity of
  its spacing and from backward motion.
- **Front, flank and rear** come from geometry. Each boundary segment of a body is classified by
  its outward normal against the body's facing: front, flank (about ±90°), or rear. A segment is
  **exposed** when no friendly body or terrain anchor (a river, a slope, a wood) covers it.
  Facing coherence tells a mob from an orbis: an orbis is all front and has no flank.
- **Each officer's view is computed from the true bodies,** by angular occlusion from his eye
  height (about 1.6 m on foot, 2.4 m mounted) over the terrain. From horseback on a plain he sees
  front edges. Depth, and anything behind the first line, is largely unknown: Caesar hid a fourth
  line at Pharsalus. Bodies are not re-clustered per officer, which keeps views cheap and
  consistent.
- **Friends come through the order of battle.** An officer knows which units are his, but their
  state only from what he sees and what reports tell him.
- **The belief map** ([05](05-command.md) §2) is built from these views, with a timestamp and a
  confidence on each entry. Stale beliefs decay toward uncertainty.

Some of history's deceptions then emerge on their own. At Cannae, Roman officers saw the Gauls
falling back and could read it as winning.

## 4. The plan representation

**One small core representation, three users.** The AI generates and repairs it. A text
authoring syntax and the visual scripts both compile to it. Keeping the core small is what lets
the AI repair a plan in flight.

| Element | What it is |
|---|---|
| **Task** | A verb ([05](05-command.md) §3) assigned to a force: advance, hold, pin, envelop, relieve, pursue... |
| **Target** | A late-bound query over the officer's *own* scene graph, resolved when the task runs: `flank_of(enemy.main_infantry)`, `enemy.cavalry nearest river` |
| **Trigger** | When to start, in coarse terms the assignee can perceive (few, half, most), or a time, or a signal |
| **Purpose** | Why: the intent a subordinate falls back on when the task no longer fits |
| **Constraint** | Limits: do not pursue beyond the stream; keep contact with the legion on your left |
| **Contingency** | What to do if: the centre breaks, the camp is threatened, the commander falls |

Michal's example as a script:

```
task     right_wing_cavalry: attack(flank_of(enemy.main_infantry))
when     half(selected_maniples) beyond line(L1)
purpose  "stop their infantry turning on our centre"
```

### The script budget: authoring tiers

The more of a plan is authored, the less a historical result means: a scripted "Hannibal plan"
would write the outcome of Cannae into the input. Every validation scenario is therefore authored
at a declared tier, and A1 is reported at all three ([07](07-validation.md) §6.3):

| Tier | What is authored |
|---|---|
| **0** | Deployment plus generic doctrine, which belongs to an army or culture, not to a battle |
| **1** | Tier 0 plus each commander's stated purpose |
| **2** | Tier 1 plus explicit triggers and contingencies |

**Authoring rule, at every tier:** nothing a commander could not have said aloud beforehand, and
triggers only over what the assignee can perceive, in coarse terms. The claim for A1 is made at
tier 0 or 1. For Cannae:

| Force | Tier 0: doctrine | Tier 1: adds a purpose | Tier 2: adds triggers (illustration only) |
|---|---|---|---|
| Libyans | Attack an exposed flank within reach | Keep the Roman infantry from breaking through the centre | When the Roman mass is between the two Libyan bodies, face inward and attack |
| Hasdrubal's horse | After winning the wing, rally and choose the most valuable target, weighted by discipline | Break their horse, then go where you are most needed | After the Roman horse breaks, ride behind the Roman army to the allied horse, then into the infantry's rear |
| Gauls and Iberians | Advance and engage | Hold them as long as you can | Advance in a crescent; give ground slowly toward the Libyan line |

Polybius 3.116 credits Hasdrubal's own judgment for leaving the pursuit to the Numidians and
riding to support the Africans. At tier 0 or 1 that choice has to come from his reading of the
situation, through the plays below.

## 5. How officers decide

- **Plan monitoring.** Each officer compares the progress he expected with what he perceives. A
  deviation (a wing that should have held is falling back; the enemy horse is not where it was)
  triggers re-evaluation.
- **A library of plays.** About 30–50 hierarchical methods, in the style of hierarchical task
  networks. Each has cues over the scene graph, a decomposition into tasks for subordinates, and
  an expected course. Examples: commit the reserve to a sector; refuse a flank; attack an exposed
  flank; pin and envelop; rally and re-task after winning a wing; relieve the line; retire in
  order; form an orbis; break out; feign flight (trained cavalry only).
- **Initiative plays for subordinates** cover the improvisations of §1. "An unengaged force within
  reach of an exposed flank or rear" is a cue. Whether an officer acts on it depends on his
  initiative, his orders and his reading of his superior's purpose.
- **Recognition-primed decisions** (Klein). An officer matches cues to one play, checks that play
  (can I reach the flank in time? what does it expose?), and accepts it or moves to the next. It
  is not an optimisation over all options. Personality weights the matching: an aggressive
  officer tries attacks first, a cautious one holds. The check is a feasibility check, not a
  simulation of the battle. Look-ahead simulation is deferred: it needs a body-level combat model
  and a model of the enemy's response, and its artefacts would become the AI's habits.
- **The aggregate model** ([06](06-architecture.md) §2) models a battle at the level of formed
  bodies. It is the officer's mental model of forces and timing, a fast test bed for this AI,
  and a cheap rung for calibration. A world-view trait lets the command AI run unchanged against
  the full simulation or against the aggregate model.

## 6. The LLM: an author, never in the loop

No LLM runs inside the simulation. Latency, cost and nondeterminism rule it out, and LLMs reason
poorly about space. The strongest reason is **contamination**: an LLM has read about Cannae and
every other corpus battle. It must never write corpus plans, doctrine or sealed dossiers.

Outside the loop, it is a useful author:

- **Natural language to the plan representation.** You describe a plan; it writes the core
  representation; you review it before it runs.
- **A council of war** for custom battles. The consuls alternated command day by day, and on the
  day of Cannae it was Varro's turn (Polybius 3.113). A council of personalities can argue a plan
  out before the battle.
- **Candidate plans for custom setups,** scored by the simulator against enemy plans *sampled from
  the commander's beliefs*, never against the true enemy plan. Otherwise the commander would
  have rehearsed against the real enemy.
- **Narration** of the event log after the battle ([06](06-architecture.md) §9).

Generated plans are cached and versioned, so runs that use them stay reproducible.

## 7. The time model

- **The simulation runs on a fixed step, decoupled from the wall clock.** Officers decide
  asynchronously, on events plus a periodic re-check that depends on rank ([05](05-command.md)
  §2).
- **Compute budgets are counted in work, never in milliseconds,** so results never depend on the
  machine's speed. Thinking time is simulation time and a personality trait: Caesar decides
  fast. A decision may be computed in the background, but the simulation waits at the tick it is
  due.
- **Presentation:** real time with pause; time compression; auto-pause rules (a rider arrives, a
  unit routs, a trigger fires, the enemy commits his cavalry); and an optional turn-like mode,
  WEGO, that pauses every few simulated minutes for orders.

## 8. Testing the officers

The officers are tested early, before full combat exists ([08](08-roadmap.md), Slices 1–2):

- **Headless batches with expected bands.** An exposed flank 600 m from a cavalry commander: does
  he notice, and how fast? Does reserve timing vary sensibly with personality? Is a dust-obscured
  scene misread, as Cassius misread Philippi? Do different personalities diverge on the same
  scene?
- **A decision debugger.** Step to the next decision; show belief against truth, the candidate
  plays and their match scores, and the orders in transit.
- **Synthetic scenes.** Hand-placed formations on the aggregate model, so the command AI can be
  tuned before soldier-level combat is ready.
- **Experimenter-mode takeover** (Slice 2): you play against the AI, or alongside it, and see
  where it fails.
