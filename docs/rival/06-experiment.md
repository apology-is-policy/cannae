# 06 — The Experiment

The two designs disagree about two things: the man, and the substrate he moves on. Both can be
settled cheaply, inside the two weeks the vision of record already gives to Slice 0. The form
below was agreed with Paullus on yip call 0001. Whether it runs, and who builds what, is
Michal's decision.

## 1. One harness, two engines

**Shared by both engines:**

- the scenario files;
- the metrics, computed by the harness from observable state only: positions, postures, wound
  classes and logged events;
- the RNG streams;
- W′ and its constants.

**Different:** the model of the man, and the substrate.

| | Engine A: the vision of record | Engine B: the rival |
|---|---|---|
| The man | The excitable psyche of 04 §5, with its three kernels | Dispositions, postures and hazards ([01](01-the-man.md)) |
| The substrate | Soft-body collision at 20 Hz, threaded | Field and events at 4 Hz, one core, with contact where men are crowded and moving ([03](03-engine.md)) |

**Fairness.**

- Each engine's parameter priors are written down before any scored run, by its builder, and
  read by the other.
- The pass bands for every pattern are fixed before any scored run.
- Neither engine may special-case the patch.
- Both get the same time box for tuning and optimisation.

## 2. The patch: behaviour

A thin slice of a battle in two dimensions: two centuries side by side, twenty files by eight
ranks, against a warband of equal numbers on open ground. About 400 men. It is wide enough for
a break to spread along the line and deep enough to have rear ranks, which the one-dimensional
toy could not offer together.

**Variants:** veterans against recruits; a small third body arriving on a flank; a second
Roman line behind the first, to test relief.

**Memory variants,** for the question of whether fear must outlast its cause
([01](01-the-man.md) §9): volleys that cannot be answered and kill few, then a charge; a shock
from behind that is driven off, then pressure from the front. Does the earlier fright change
what happens next, and should it?

**Patterns scored:**

| # | Pattern | What is measured |
|---|---|---|
| 1 | A stable standoff | Two lines can face each other out of reach without closing to the death |
| 2 | Pulses and lulls | Clash and lull structure in the contact-intensity series, with no scripted rhythm |
| 3 | Rear-first collapse | Where the first men to leave a breaking body stood |
| 4 | Lateral spread | A break travels along the line |
| 5 | Casualty asymmetry | The loser's losses fall mostly after the break |
| 6 | **Holds, then breaks** | The time to the first break must lie away from both zero and never, and must move the right way, by a credible amount, for veterans against recruits, a flank arrival, and relief. Its absolute value is close to an input in both designs. |
| 7 | **The casualty clock** | Disabling hits per front position per hour inside the band, with lethality per bout inside its bounds ([02](02-order-and-contact.md) §9) |
| 8 | The centurion gradient | The centurions' share of losses is higher in a formed fight than in a rout |

Patterns 1, 2, 3 and 5 are the four the vision of record set for its toy. Pattern 4 comes with
the second dimension. Patterns 6 and 7 are the ones each side expects the other to fail.

**Procedure.** For each engine, draw the same number of parameter sets from its priors, run
each on a fixed set of seeds, and score every pattern. Report:

- the number of free parameters;
- the share of prior draws that pass every pattern;
- which patterns fail most often, and how.

**One ladder for the man.** Paullus proposed, and I accept, that the two men are rungs of one
ladder and not rivals:

| Rung | The man |
|---|---|
| 0 | No marks |
| 1 | *Has fled* |
| 2 | *Has fled* and *shaken* |
| 3 | Continuous fear and resolve, the vision of record's F and R, as gauges in place of *shaken* |
| 4 | The full excitable psyche of 04 §5 |

Every rung shares the visible layer: five postures, hazards read off counted acts, and events
with logged causes. The vision of record turns x into acts anyway. The lowest rung that passes
is kept. The question is then not which man, but where on one ladder the patterns stop
failing. That is the parsimony pillar applied to the mind.

**The passage.** One more scenario, for the substrate: a maniple falling back through an
interval under pressure, and a crowd leaving through a gate. It is run three ways: field
alone; field with contact where men are crowded and moving; contact everywhere. Each is scored
against measured flows through bottlenecks from pedestrian studies (flow against gap width,
and when clogging starts) and against the others. Those experiments are not armoured men under
attack, but they are a fair check on any model of crowding. The lowest rung that matches is
kept. Paullus proposed this item. The data still has to be found and read.

A model that passes only on a knife-edge has failed, whichever engine it is. The vision of
record already says so (04 §5.3).

**One thing must be found first:** bounds on lethality per bout from duel experiments and
reenactment. Neither of us has a source in hand. Without them pattern 7 has no teeth.

## 3. The spike: speed

150,000 bodies in each substrate, with realistic memory traffic, in three configurations:

- **a line battle:** two armies in contact along 1.5 km;
- **a pocket:** 60,000 men encircled and compressed;
- **a rout through gaps:** 20,000 men leaving through camp gates, the worst case for contact.

Measured after ten minutes of sustained load on the fanless Air, never at burst:

- simulated seconds per wall second: engine B on one core; engine A on all cores as designed,
  reported in total and per core;
- for B, an identical state hash across repeated runs; for A, across 1, 4 and 8 threads;
- the peak density reached in the pocket, which must stay believable
  ([02](02-order-and-contact.md) §5).

## 4. Kill criteria, both ways

**The rival dies if:**

- it runs below 10× real time per core in the pocket; or
- it cannot hold a line for an hour and then break with no more memory than the two marks
  (pattern 6). Then the answer lies higher on the ladder (§2), on whichever substrate survives.

**The vision of record's engine dies if:**

- it cannot meet the casualty clock without making the clash nearly harmless (pattern 7); or
- it cannot reach 3× real time sustained at full scale on the Air. Below that, every A1 batch
  needs a rented machine.

**Tie-breakers, if both live:** fewer free parameters; a larger share of the prior passing;
the centurion gradient.

## 5. Four outcomes

The man and the substrate are separable, so the experiment has four results, not two.

| | Substrate B is fast enough | Substrate B is not |
|---|---|---|
| **Man B passes** | The rival stands | The small man on the vision of record's engine: a cheaper psyche, same physics |
| **Man B fails** | The dynamical psyche on fields and events: costlier per man than B, perhaps far cheaper than pairwise collision | The vision of record stands, with the lessons of [05](05-costs-and-critique.md) §7 |

The two mixed results are worth as much as the pure ones, and the harness should be built so
that a man and a substrate can be combined freely.

## 6. Scope, order and who builds what

Two complete engines do not fit in two weeks, and they are not needed. Paullus's split, which
I accept:

- **The man** is tested on the 400-man patch: every rung of the ladder, on both substrates.
  At 400 men the substrate costs nothing either way.
- **The substrate** is tested by the 150,000-body spike, each running its real arithmetic per
  man with no behaviour tuning, and by the passage.
- **Order:** the harness and metrics; the patch on the field; the patch on pairwise contact;
  the spikes; the passage. If time runs short, the threaded pairwise engine at full scale is
  what gets measured in parts. Paullus has said that shortfall counts against his side.

**Each piece is built by its believer.** Paullus builds the harness, the metrics, the pairwise
substrate and rungs 3 and 4. Varro builds the field substrate and rungs 0 to 2. Each reads the
other's parameter list before any scored run. That is my reading of Paullus's proposal, and
the decision is Michal's.

## 7. Cost

Two weeks, on the Air, with no money spent. That is the budget the vision of record gives Slice 0, which
still waits for Michal's go-ahead. It replaces that slice's toy and spike with comparative
versions of both.

## 8. What Michal decides

1. **Whether Slice 0 runs in this form.**
2. **Who builds what** (§6).
3. **Whether dispositions, visible acts and at most two marks are the psyche he asked for,**
   with fear and morale as things he watches and measures per unit, or whether he wants
   gauges in each man ([01](01-the-man.md) §9). The memory ladder (§2) informs that choice. It
   does not make it.
4. **When a design review runs.** CLAUDE.md proposes one after a major revision of the vision.
   A rival vision is at least that. My suggestion is to review after the experiment, when its
   results have made half the questions moot. That is a suggestion, not a decision.
