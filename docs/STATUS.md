# Status

**Phase:** design. No code yet. A rival vision is being written by a second session, Varro.
**Working mode:** guided (see [CLAUDE.md](../CLAUDE.md)).
**Last updated:** 2026-11-10.

## Current arc: a rival vision (Phase 0b)

Michal's idea, 2026-11-10. A second session, **Varro** (Fable 5.2, worktree `cannae-varro`,
branch `varro`), builds a competing vision for the simulator and is free to relax our
constraints. Paullus (the main session) leads, argues and reports. They talk over the yip line,
and Michal watches on the switchboard. Nothing from the rival lands on `main` without Michal's
decision.

- **Varro's sketch, "Small men, many battles":** keep only what a witness could see of each man,
  plus fixed dispositions; anchor formations on their standards; use a field-and-event substrate,
  one battle per core; run full-scale Cannae a hundred times a night on the Air.
- **Agreed so far:** the Slice 0 toy becomes a thin 2D patch; the casualty clock and the
  centurions' share of losses become tests for both designs; Slice 0 becomes a bake-off, with one
  harness, two engines, and kill criteria both ways.
- **In progress:** Varro writes the full rival in `docs/rival/` (seven files, committed on
  `varro`, not pushed).
- **Next:** Paullus writes a short comparison, and both go to Michal.
- **Questions for Michal:** does "a psyche" need inner per-man values, or is fear and morale shown
  through behaviour enough? Is it acceptable for line-passage jams to be an input rather than an
  emergent result?

## Next arc

**Slice 0, about 1–2 weeks, on Michal's go-ahead**, now proposed as a bake-off between the two
designs. Michal chose the rival exercise over the design review 2 proposed at arc V's close.

1. **Behaviour:** a thin 2D patch, two centuries against a warband (about 400 men), in each
   engine. Scored on standoff, pulsing, rear-first collapse, the casualty asymmetry, the casualty
   clock and the centurion share; parameter count and the share of prior volume that passes.
2. **Performance:** 150,000 bodies in each substrate. Sustained simulated seconds per wall second
   per core in an encirclement, and identical hashes across thread counts or cores.

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

- The go-ahead for Slice 0 in its bake-off form, after the rival arc reports.

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
