# Status

**Phase:** design. No code yet.
**Working mode:** guided (see [CLAUDE.md](../CLAUDE.md)).
**Last updated:** 2026-11-10.

## Current arc: V -- revise the vision after review 1

Approved by Michal on 2026-11-09. This arc responds to
[design review 1](reviews/2026-11-09-fable-5.2-vision-review.md), by Fable 5.2. Each chunk is
one commit.

| Chunk | Content | State |
|---|---|---|
| C0 | Continuity contract: `CLAUDE.md`, self-compaction tooling, this file, the review record | landed |
| V1 | The 13 factual corrections from the review | landed |
| V2 | README: Cannae outcome classes (centre may break), indeterminacy test redefined | landed |
| V3 | 07 validation: Cannae as development scenario, sealed battles, script tiers, variance decomposition, contrasts, sensitivity analysis and history matching | landed |
| V4 | 04 psyche: excitable-system dynamics, contagion on observable acts, parsimony | next |
| V5 | 06 architecture: fanless 8 GB Air, revised performance, intent pass, observables lane, keyframes | queued |
| V6 | 03 formations: line relief, R2 with a seam, feasibility instead of discrimination | queued |
| V7 | 08 roadmap: vertical slices, demand-driven knowledge base, review cadence | queued |
| V8 | New 09: intent, plans and command AI | queued |
| V9 | New 10: the contact zone, combat and wounds | queued |

**V1 note.** Each correction was checked against the primary text before it was applied. One
half-item was overturned: Polybius 6.23 gives the hastatus one greave, not greaves (the Greek
word is singular). This is recorded in the review's disposition table.

## Next arc (approved in principle, starts after V)

**Slice 0, two spikes, about 1-2 weeks:**
1. A one-dimensional behavior toy of two short lines. Does a stable standoff, pulsing combat,
   rear-first collapse and the casualty asymmetry appear over a non-trivial parameter region?
2. A performance spike: 150,000 agents, collision, a dummy think with realistic memory traffic.
   Measure sustained cycles per agent-tick on the Air, and check the hash is identical on 1, 4
   and 8 threads.

## Open decisions

None blocking. Michal's answers so far:
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
