# 11. After the report

**Draft by Paullus, 2026-11-19.** Not results, and none of them mine to decide.

## What the outline fixed beforehand

- **Michal reads the report and decides what is kept.**
- **I propose design review 2.** By [CLAUDE.md](../../../CLAUDE.md) a review is due at the
  close of every slice, and the merged hypothesis failed the test it was built to pass (the
  vision's own psyche, rung 4, was not in it). **Michal asked for it on 2026-11-19, and it has
  reported:** [its report](../../reviews/2026-11-19-fable-5.2-slice0-review.md), with a
  disposition table.
- **`docs/vision/` is revised to the results,** with Michal's approval.

## Questions for Michal

- **Rung 4.** Michal decided on 2026-11-10 that rung 4 is built only if rungs 0 to 3 all fail.
  They have, on both engines (section 1), so the condition holds whether "all fail" means one
  engine or both.
- **What follows from the speed** (section 8), and when the Mac can be given its forty quiet
  minutes for the second reading.
- **The list for a second round:** 26 candidates in [STATUS](../../STATUS.md), none decided.
  One came from this report: the flank band that starts the clock (26, section 6); and 27,
  registered before the freeze, was found missing by design review 2.

## Faults to mend that change no result

- **The replay tool's stepping:** in 59 replays its battle differed from the scorer's, and in 8
  it missed a relief ordered in a run's last second (sections 5 and 6). The records are sound:
  the scorer's own run gives each one back. **Mended on 2026-11-20** (b8ae6af): the tool
  stopped up to 19 ticks early, because the record keeps the scorer's last tick in whole
  seconds. Run again, every replay counts, and sections 5 and 6 give the recount.
- **The flank band's marking,** if `flank` is run again (section 6).
