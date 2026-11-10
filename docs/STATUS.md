# Status

**Phase:** design. No code yet. The rival-vision arc has reported; Michal decides next.
**Working mode:** guided (see [CLAUDE.md](../CLAUDE.md)).
**Last updated:** 2026-11-10.

## Current arc: a rival vision (Phase 0b) -- reported, waiting for Michal

Michal's idea, 2026-11-10. A second session, **Varro** (Fable 5.2, worktree `cannae-varro`,
branch `varro`), built a competing vision and was free to relax our constraints. Paullus led,
argued and reported. They talked over the yip line, and Michal watched on the switchboard.
Nothing from the rival lands on `main` without Michal's decision.

- **The rival:** *Small men, many battles*, seven files in `docs/rival/` on branch `varro`
  (pushed as a branch, not merged).
- **The report:** [rival-comparison.md](rival-comparison.md). It covers where each design was
  stronger, the common design both sessions now hold, what is still open, Slice 0 as the
  experiment, the options and the decisions.
- **Converged:**
  - Cannae is measured at 1:1, with A2 taken from the set of seeded runs.
  - Each body of men forms on its standard.
  - Each man has postures, chances to change them read off what he can see, and marks of
    memory.
  - Order decides the physics: bodies touch only where men are crowded and moving.
  - Integer state, one battle per core.
  - The casualty clock is a test on both designs.
- **Open, for the experiment:**
  - how much memory a man needs (one ladder spanning both designs);
  - what times a clash;
  - whether the speed is real;
  - whether a crowd stays at a believable density.
- **Recommendation:** merge. Take the common design as the hypothesis, run Slice 0 as the
  experiment, then revise `docs/vision/`.

## Next arc

**Slice 0 as the experiment, about two weeks, on Michal's go-ahead.** The full plan is in
[rival-comparison.md](rival-comparison.md).

1. **Harness and metrics**, shared by both engines.
2. **The man:** a 400-man patch (two centuries against a warband). Every rung of the memory
   ladder runs on both substrates, and eight patterns are scored.
3. **The substrate:** 150,000 bodies in a line battle, a pocket, and a rout through camp gates.
   Speed is measured as sustained simulated seconds per wall second.
4. **The passage:** a maniple falling back through an interval, and a crowd leaving through a
   gate, both checked against measured crowd-flow data.

Kill criteria run both ways. Each engine is built by its believer (proposed).

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

- Which option: keep ours, adopt the rival, or merge ([rival-comparison.md](rival-comparison.md)).
- The go-ahead for Slice 0 as the experiment, and who builds what.
- Whether each man should also carry fear gauges for the inspector, even if the patterns don't need them.
- Whether `docs/rival/` lands on `main` as a proposal.

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
