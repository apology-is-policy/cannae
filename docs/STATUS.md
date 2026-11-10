# Status

**Phase:** Slice 0, the experiment between the two designs. The shared core and harness run end to end with a toy man; the engines are next.
**Working mode:** guided (see [CLAUDE.md](../CLAUDE.md)).
**Last updated:** 2026-11-10.

## Current arc: Slice 0 -- the experiment (approved 2026-11-10)

Michal approved Slice 0 as the experiment between the vision of record and Varro's rival design.
It takes about two weeks, on the Air, with no money spent. The plan is in
[rival-comparison.md](rival-comparison.md) and [rival 06](rival/06-experiment.md). Each design is
built by its believer. Paullus builds on `main`; Varro builds on branch `varro`, and Paullus
reviews his work and merges it.

**Paullus's chunks:**

| Chunk | Content | Commit |
|---|---|---|
| S0.0 | Land the rival on `main` as a proposal; point the vision docs at the merge; open this arc | c475976 |
| S0.1 | Workspace and shared core: integer units and angles, Philox random streams with fork branches, W', observable state, state hash | 7ae78f3 |
| S0.2a | The seam, in the core: tick and phases, the View and reference grid perception, dormancy, events and causes, competing hazards, intents and targets, the power table, the traits. Agreed with Varro on the call | d419574 |
| S0.2b | The harness: TOML scenarios (the patch), the runner, a kinematic test substrate, a toy man, SVG frame dumps, determinism and dormancy conformance tests | c196e28 |
| S0.3 | Metrics and pre-registration: the eight patterns computed from observable state; pass bands and both sides' parameter priors written down before any scored run | 0fbcc74 metrics and draft; freeze after Varro agrees |
| S0.4 | Lethality bounds: literature search on skeletal trauma and experimental archaeology (Varro searches line-fighting ethnography and Collins) | f8cb2a6 Paullus's half; band proposed |
| S0.5 | The pairwise substrate, where every body touches its neighbours, at patch scale; a perception conformance check for Varro | 14e919b |
| S0.6 | The top two rungs of the memory ladder: continuous fear and resolve, then the full excitable psyche | Rung 3 built and plugged into Varro's men through his `Memory` hook ([design](slice0/rungs-3-4.md)). Its dormancy is exact through a fight and a rout; with its gauges switched off it replays rung 1's battle event for event. Rung 4 is built only if rungs 0-3 all fail (Michal) |
| S0.7 | The speed spikes: 150,000 bodies in a line battle, a pocket and a rout through camp gates, on both substrates | Pairwise done for the line, surge and pocket ([report](slice0/spike-pairwise.md)); the rout through camp gates is built (with walls indexed on a coarse grid, so a man checks only the walls near him) and waits for an idle machine to be timed; Varro's field spike is his |
| S0.8 | The passage study, against measured crowd-flow data | Walls (`cannae_core::Wall`, `[[wall]]` in scenarios) and the pairwise calibration done ([report](slice0/passage-calibration.md)). The bench for the two passages is built, and the pairwise substrate has run on it ([report](slice0/passage.md)); Varro's field runs on the same bench when it lands |
| S0.9 | Scored runs and the report; then design review 2 | |

**Speed (pairwise substrate only, no man model yet; sustained readings on the fanless Air,
[report](slice0/spike-pairwise.md)):** 150,000 bodies on 4 cores for 10 minutes ran at 12.1x
real time in a line battle, 5.3x when every man keeps pushing, and 2.3x in a pocket (3.8x on 8
cores). The kill line is 3x with the man included, so **the pocket is this engine's main risk**.
The worst case was measured: everyone touching, a ring pressing forever, and no one ever
falling. The sustained run also found a physics bug: men could walk through a standing crowd,
up to 64 men per m². Now a man cannot walk into the men ahead of him, his lean fades as he
presses to the bone, and bodies have a hard core. The pressed edge settles at about 5 per m²,
the crowd-safety literature's onset of danger.

**The passage study (pairwise substrate so far).** A bench runs the same scripted passages on
any physics engine: a maniple of 120 falling back through an interval in the line behind it,
with and without the enemy shoving it, and a crowd of 600 making for a camp gate, walking or
running and shoving ([report](slice0/passage.md)). On my engine:

- **A maniple passes an interval in under a minute,** 17-31 s at every width from 3 to 12 m.
- **At a 3 m interval, the enemy's shove jams it.** The longest wait between two men rises
  from half a second to six. This is the clogging measured at doors in experiments, and here
  it comes out of the physics, with no rule for it.
- **Two structural limits, reported, not fixed.** Walking without pushing matches measured
  flows through narrow corridors when a man's disc has a radius of 300 mm
  ([calibration](slice0/passage-calibration.md)). Through wide gaps it runs 30-50 % too fast,
  because real walkers slow down as a crowd thickens and discs do not until they touch. And
  people pushing through a 0.7 m door pass about 40 % faster than discs can, because people
  turn their shoulders.
- **The crush settings were unusable, now fixed.** The old stand-in values disabled half a
  running crowd at a gate within five minutes, and the rate's units could not even hold the
  agreed prior. At the prior's centres, no one is crushed in a passage. A crowd shoved against
  a wall reaches 5.4-8 men per m², in line with the formula behind the crush prior.

**Priors (drafts, frozen with the pre-registration):** three files, so that both engines draw everything they share from one prior, and draw *i* is the same in every cell.

- [priors-shared.md](slice0/priors-shared.md): stamina, power and gait (Paullus); ties, bouts and the troop table (Varro); four crowd-physics numbers. Power is a multiple of each man's critical power, because the placeholders mixed cycling watts with metabolic ones. The Gaulish guard (S23) is now checked in Polybius's Greek: the shield "could not cover the man all round" (2.30.3), and the comparison of shields at 2.30.8 has a gap in the text.
- [priors-pairwise.md](slice0/priors-pairwise.md): the pairwise substrate (four drawn parameters, plus the shared ones) and rung 3's five (how long fear lasts, how much of it a man carries, how fast it spends his resolve, how fast resolve returns, and what empty resolve costs). Rung 4's are written only if it is built.
- [priors-field.md](slice0/priors-field.md): Varro's, 29-32 drawn per rung plus troop inputs, with what he expects written down before any run.

The sources behind them are in [crowd-physics.md](slice0/crowd-physics.md) and [physiology.md](slice0/physiology.md). The march speed is checked against Vegetius 1.9 in the Latin: 20 Roman miles in five summer hours, about 1.3 m/s.

**Varro's work:**

| Piece | Status |
|---|---|
| Frames (standards, slots, ties, drills) and the shared bout resolver, in the core | 84dce11, reviewed and merged to `main` |
| `cannae-man` with rungs 0-2, his priors file, the behaviour table (rival 07), his lines of priors-shared.md, his half of the lethality search | 9b29064, reviewed and merged |
| The field substrate (its own perception, and contact either along the line or where men are crowded), a first speed pass, and the `Memory` hook that rung 3 plugs into | e54c427, 09e4326, 756334c, reviewed and merged |
| Walls on the field, its spike report, the rest of his priors | in progress |

**Pre-registration: five defects found and fixed so far,** four of them by Varro. All were found
before any scored run, which is the point of writing it down first.

1. **Holding then breaking:** the pass mark couldn't be met by a line whose chance of breaking
   stays the same every minute. The horizon is now two hours, and the shape of the time to
   break is reported, not banded.
2. **Pulses and lulls:** "burstiness" was counted per time bin, where random fighting passes.
   It is now measured on the gaps between bouts, per 9 m stretch of front.
3. **Rear-first collapse and lateral spread:** one early runaway could set both statistics.
   They are now measured over the collapse only, and runaways are counted separately.
4. **No rule if no rung passes:** now written down. The rungs are still ranked, the report
   says so plainly, and nobody narrows a prior and reruns.
5. **Lethality per exchange:** the lower end of the band is now 0.002, set by Varro from his
   half of the search, since it decides my engine's kill line.

Fighting tempo is reported but not banded. The sources are too thin to fail an engine on.

**Kill criteria, both ways.**

- **The rival dies if** it runs below 10x per core in the pocket, or if no rung up to *shaken*
  holds a line and then breaks it.
- **Ours dies if** it meets the casualty clock only with a near-harmless clash, or if it runs
  below 3x sustained at full scale.

**Where both stand on speed today: both are at risk in the pocket.** Mine runs the pocket at
2.3x sustained on 4 cores with no man model yet, against a line of 3x with him. Varro's own
readings (burst, one core, 150,000 men in the pocket):
- his substrate alone, with contact along the line only: 15.2x, up from 9.4x;
- with contact wherever men are crowded: 1.6x, because in a pocket 60,000-90,000 men are
  crowded and pressing, and contact costs there what it costs in mine;
- with perception on: 5.3x, before he rebuilt the search for nearby enemies. About 38,000 men
  have an enemy in view at every step.

So today the rival does not meet its own line of 10x per core in the pocket once perception is
counted, and the man's hazards come on top. That is Varro's statement, and it may change after
his sustained readings, which wait for the machine to be idle.

**Deviation from the vision of record.** Slice 0 replaces the toy and spike described in
[08](vision/08-roadmap.md) §1, with Michal's approval. The vision docs carry a pointer to say so.
They are revised to the experiment's results after it reports.

## Last arc: a rival vision (Phase 0b, closed 2026-11-10)

This was Michal's idea. A second session, **Varro** (Fable 5.2), built a competing vision, free
to relax our constraints. The two sessions argued it over six turns on the yip line while Michal
watched on the switchboard.

- **The rival** is [docs/rival/](rival/README.md), *Small men, many battles*. It landed on
  `main` as a proposal at bf719b3.
- **The report** is [rival-comparison.md](rival-comparison.md) (207c046). It covers where each
  design was stronger, the common design, what is still open, and the options.
- **Michal's decisions:**
  - merge;
  - Slice 0 runs as the experiment;
  - the memory ladder decides whether men carry fear gauges;
  - the rival lands on `main` as a proposal.

## Earlier arc: V -- revise the vision after review 1 (closed 2026-11-10)

Approved by Michal on 2026-11-09. It responded to
[design review 1](reviews/2026-11-09-fable-5.2-vision-review.md), by Fable 5.2. Each chunk was
one commit, pushed.

| Chunk | Content | Commit |
|---|---|---|
| C0 | Continuity contract: `CLAUDE.md`, self-compaction tooling, this file, the review record | e379114 |
| V1 | The 13 factual corrections, each checked against the primary text | b0f5ef8 |
| V2 | README: Cannae outcome classes (the centre may break), A2 redefined, sealed battles, the player | e97d03a |
| V3 | 07 validation: development scenario, sealed battles, authoring tiers, variance decomposition, contrasts, sensitivity analysis and history matching | 5d4c7a1 |
| V4 | 04 psyche: excitable-system dynamics, contagion on observable acts, parsimony | 26b892f |
| V5 | 06 architecture: fanless 8 GB Air, revised performance, observables lane, intent pass, keyframes | ab7fbae |
| V6 | 03 formations: R2 line relief with a seam, feasibility instead of discrimination, the unit frame | 848f40a |
| V7 | 08 roadmap: vertical slices, what waits, demand-driven knowledge base, review cadence | 1d95234 |
| V8 | New 09: intent, plans and the command AI | 3e9e18f |
| V9 | New 10: the contact zone, combat and wounds | c0f9428 |

**What changed, briefly:**

- **Cannae is the development scenario,** not a holdout. The proof is three to five sealed
  battles, hash-committed before the engine can run them.
- **A1** accepts a centre that holds or breaks, adds a breakout class, tests dose-response
  instead of a 3 % minimum, and is run at three authoring tiers. The claim is made at tier 0 or 1.
- **A2** uses a strong learned predictor and splits outcome variance by random-number stream.
- **The psyche** is an excitable system: a fast approach–avoid state plus slow stamina and
  resolve. Contagion works on visible acts, read against the unit's order.
- **Performance** on the fanless Air: about 3–7× real time at full Cannae scale and 5–10 seeds a
  night. Acceptance batches need a rented machine, with Michal's approval.
- **The roadmap** is vertical slices. Two new documents: [09](vision/09-intent-and-plans.md) and
  [10](vision/10-combat-and-wounds.md).

**V1 note.** Each correction was checked against the primary text before it was applied. One
half-item was overturned: Polybius 6.23 gives the hastatus one greave, not greaves (the Greek
word is singular). This is recorded in the review's disposition table.

## Open decisions

**Blocking the next step:** none.

**Not blocking:**

- The time window: Pyrrhus (280–275 BC) and Idistaviso (AD 16) as stretch scenarios only?
- Out of scope: naval warfare, sieges and the campaign layer, except as initial conditions?
- Which three to five battles to seal. Needed before Slice 2 ([07](vision/07-validation.md) §5).

**Michal's answers so far:**

- **Slice 0 (2026-11-10):** only men out of their places press, on both engines. Whether a
  formed line shoves (othismos) is put off to Slice 1, not settled, and the report says so.
  Rung 4 is built only if rungs 0-3 all fail.

- **Phase 0b (2026-11-10):** merge the two designs. Slice 0 is the experiment, each design built
  by its believer. The memory ladder decides whether each man carries fear gauges (no gauges
  for the inspector's sake alone). `docs/rival/` lands on `main` as a proposal.
- **Player role:** sets up battles (historical or experimental), watches, and can pause and take
  over commanders. Two modes: experimenter (truth, instant orders) and role-player (beliefs,
  messengers).
- **Contested mechanics:** frontage, intervals and othismos are parameters. One relief mechanism
  (R2) is built, with a seam for others.
- **Knowledge base or engine first:** neither. Slice 0 comes first, and the knowledge base is
  written as slices need it.

## Reviews

| Date | Reviewer | Scope | Record |
|---|---|---|---|
| 2026-11-09 | Fable 5.2, max effort | Vision docs and command-AI proposal | [review 1](reviews/2026-11-09-fable-5.2-vision-review.md) |
