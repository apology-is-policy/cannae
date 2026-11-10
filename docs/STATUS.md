# Status

**Phase:** design revised after review 1. No code yet. Waiting for Michal's go-ahead.
**Working mode:** guided (see [CLAUDE.md](../CLAUDE.md)).
**Last updated:** 2026-11-10.

## Next step: waiting for Michal

1. **Proposed: design review 2 by Fable**, of the revised vision, before any code. CLAUDE.md
   calls for a review after a major revision of `docs/vision/`. Not run.
2. **Next arc, approved in principle: Slice 0, two spikes, about 1–2 weeks.** Starts on Michal's
   go-ahead.
   1. **A behaviour toy:** two short lines in one dimension, with the excitable psyche of
      [04](vision/04-psyche.md) §5 and W′. Do a stable standoff, pulsing combat, rear-first
      collapse and the casualty asymmetry appear over a non-trivial parameter region? Which of
      the three kernels (smooth, two-threshold, cusp) is the simplest that does?
   2. **A performance spike:** 150,000 agents, collision, a dummy think with realistic memory
      traffic. Measure sustained cycles per agent-tick on the Air, and check that the hash is
      identical on 1, 4 and 8 threads.

## Last arc: V -- revise the vision after review 1 (closed 2026-11-10)

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

**Blocking the next step:**

- The go-ahead for Slice 0, and whether design review 2 runs first.

**Not blocking:**

- The time window: Pyrrhus (280–275 BC) and Idistaviso (AD 16) as stretch scenarios only?
- Out of scope: naval warfare, sieges and the campaign layer, except as initial conditions?
- Which three to five battles to seal. Needed before Slice 2 ([07](vision/07-validation.md) §5).

**Michal's answers so far:**

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
