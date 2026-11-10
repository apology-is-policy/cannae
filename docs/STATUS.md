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
| S0.6 | The top two rungs of the memory ladder: continuous fear and resolve, then the full excitable psyche | |
| S0.7 | The speed spikes: 150,000 bodies in a line battle, a pocket and a rout through camp gates, on both substrates | 1cce715, e6d2195 (pairwise 4x faster); sustained readings running |
| S0.8 | The passage study, against measured crowd-flow data | |
| S0.9 | Scored runs and the report; then design review 2 | |

**Speed readings (pairwise substrate only, no man model, 6 s bursts; the sustained 10-minute runs are going now):** 150,000 bodies run at about 9.9x real time on one core in a line battle and 15.5x on four; a pocket, the dense worst case, at 3.8x and 6.3x. The first version managed 2.5x and 2.4x. Its cost was not the contact search but building and summing a grid with a cell for every patch of empty ground. Results are bit-identical across versions and thread counts. The kill line for this engine is 3x sustained on all cores, with the man included.

**Priors (drafts, frozen with the pre-registration):** three files, so that both engines draw everything they share from one prior, and draw *i* is the same in every cell.

- [priors-shared.md](slice0/priors-shared.md): stamina, power and gait (Paullus), plus ties, bouts and the troop table (Varro, to come). Power is now a multiple of each man's critical power, because the placeholders mixed cycling watts with metabolic ones.
- [priors-pairwise.md](slice0/priors-pairwise.md): the pairwise substrate, with seven drawn parameters, and rungs 3-4 to come. The placeholder crush threshold was below the crowd-safety literature's onset of 5-8 men per m².
- `priors-field.md`: Varro's, to come.

The sources behind them are in [crowd-physics.md](slice0/crowd-physics.md) and [physiology.md](slice0/physiology.md). The march speed is checked against Vegetius 1.9 in the Latin: 20 Roman miles in five summer hours, about 1.3 m/s.

**Varro's work:**

| Piece | Status |
|---|---|
| Frames (standards, slots, ties, drills) and the shared bout resolver, in the core | 84dce11, reviewed and merged to `main` |
| `cannae-man` with rungs 0-2 (33 weights, a troop table) | compiles; tests next |
| His priors file and behaviour table; his lines of priors-shared.md | next |
| His half of the lethality search (line-fighting ethnography, Collins) | |
| The field substrate, with contact where men are crowded and moving | |

**Pre-registration, one change agreed:** Varro showed the first draft of pattern 6 could not be
passed by a line whose chance of breaking stays the same every minute, so it quietly assumed
that lines grow brittle. Now a line must not break within 5 minutes of contact in more than
10 % of seeds, and must not still be unbroken two hours after contact in more than 25 %. The
shape of the time to break is reported as a finding, not banded.

**Kill criteria, both ways.**

- **The rival dies if** it runs below 10x per core in the pocket, or if no rung up to *shaken*
  holds a line and then breaks it.
- **Ours dies if** it meets the casualty clock only with a near-harmless clash, or if it runs
  below 3x sustained at full scale.

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
