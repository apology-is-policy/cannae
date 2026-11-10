# Slice 0: Pre-registration (draft)

**Status:** a draft by Paullus, 2026-11-10. It must be agreed with Varro and frozen before any
scored run. Freezing means recording the commit and the SHA-256 of
[`scenarios/prereg.toml`](../../scenarios/prereg.toml) and of both priors files in
[STATUS](../STATUS.md). After that, changing a band or a definition voids the runs scored under
it.

The point is that neither of us can tune a pass band to his own engine after seeing results.
That's the same reason the sealed battles are hash-committed.

## What is fixed before any scored run

- **The scenarios:** the patch and its variants (below), and the seeds.
- **The metrics code:** [`metrics.rs`](../../crates/cannae-harness/src/metrics.rs) at the frozen
  commit.
- **The pass bands:** [`scenarios/prereg.toml`](../../scenarios/prereg.toml).
- **The priors:** each builder lists every free parameter of his engine with a prior, in
  `docs/slice0/priors-pairwise.md` and `docs/slice0/priors-field.md`, counted line by line. The
  two builders swap lists and read them before the freeze.
- **The lethality bounds** from the S0.4 literature search.

## The procedure

For each cell of the experiment (each rung of the memory ladder on each substrate):

1. Draw 200 parameter sets from the builder's priors.
2. Run each set on the same 10 seeds, for each scenario variant.
3. Compute a run summary per run, from observable state and the event log only.
4. Pool each per-run statistic over a parameter set's seeds by its median, and compare it with
   the band.

**Report:**

- the number of free parameters;
- the share of parameter sets that pass all eight patterns;
- which patterns fail most often, and how.

**The knife-edge rule.** A cell passes only if at least 10 % of its parameter sets pass all
eight patterns. A model that passes only on a knife-edge has failed, whichever engine it is
(vision 04 §5.3). The 10 % is proposed, to be agreed.

**The memory ladder.** Within each substrate, the lowest rung that passes is kept.

## Shared definitions

- **Broken.** A body breaks when at least 40 % of its standing men (men not down) have turned.
  A side breaks when 40 % of its standing men, across all its bodies, have turned. The loser is
  the first side to break.
- **Contact** is the first bout, or the first man step at which the gap between the fronts is
  3 m or less.
- **The front** of side A is the 90th percentile of its facing men's positions along the axis
  from A's centroid to B's at setup. B's front is the 10th percentile. The **gap** is the
  distance between the two fronts.
- **Formed phase:** from contact to the loser's break, or to the end of the run.
- **A hit** is a man's first wound that disables or kills him.

## The eight patterns

| # | Pattern | Statistic | Proposed band |
|---|---|---|---|
| 1 | A stable standoff | The formed phase's length; the median gap while formed | At least 300 s; at least 1.0 m |
| 2 | Pulses and lulls | Burstiness of bouts in 10 s bins while formed; the share of bins at 20 % of the busiest or less | At least 0.2; 0.2-0.9 |
| 3 | Rear-first collapse | The mean setup depth (0 front, 1 rear) of the first tenth of each broken body to turn | At least 0.6 |
| 4 | Lateral spread | Spearman's rho between a 2 m bin's distance from the first break and its first turn; the time for 80 % of the line's bins to turn | At least 0.3; 10-900 s |
| 5 | Casualty asymmetry | The share of the loser's hits after the break | At least 0.6 |
| 6 | Holds, then breaks | Across seeds: the share that break within 5 minutes of contact; the share still unbroken two hours after contact | At most 10 %; at most 25 % |
| 7 | The casualty clock | Hits per front file per hour while formed, each side against its own first-line files; lethality per bout | 0.1-2.0; 0.005-0.15 (proposed) |
| 8 | The centurion gradient | Leaders' share of hits while formed, against after the break, pooled over seeds | Higher while formed |

**The reasons for the bands:**

- **Pattern 1:** lines face each other just out of reach (Sabin; rival 02 §7). Five minutes is
  short against a battle of hours, but long enough to rule out lines that close straight to
  the death.
- **Pattern 2:** burstiness is 0 for bouts at random and negative for a steady beat. Pulses
  need bursts, and lulls need quiet bins, but not only quiet bins.
- **Pattern 3:** rear-first means the first to go stood behind the middle of the body.
- **Pattern 4:** a break that spreads takes longer to reach far bins than near ones. A break
  that happens everywhere at once gives a rho near 0.
- **Pattern 5:** the victors lose few men, and the vanquished lose most of theirs in flight
  (du Picq).
- **Pattern 6:** the time to break must lie well away from both zero and never. This is the
  pattern each side expects the other to fail. Varro showed that the first draft (unbroken
  within one hour, at most 20 %) could not be passed by a line whose chance of breaking is the
  same every minute: at most 10 % broken by 5 minutes needs a mean time to break of at least
  47 minutes, and at most 20 % unbroken at an hour needs at most 37. The band would have
  assumed a hazard that rises with time. That may be true, but no source gives its shape, and
  it is what separates the rungs of the memory ladder. So the horizon is two hours, at which a
  constant hazard passes for a mean of 47-87 minutes, and the shape is reported, not banded
  (below).
- **Pattern 7:** the casualty clock (rival 02 §9) gives one disabling hit per front position
  every 1.4-2.8 hours, at 1.8 and 0.9 m per file. Plausible inputs move that by a factor of
  two or three, hence 0.1-2.0.
- **Pattern 7, lethality per bout (proposed: 0.005-0.15).** This is the chance that either
  man is disabled or killed in one committed bout. The skeletal and experimental half of the
  search is in [lethality-skeletal.md](lethality-skeletal.md). Its central range, 0.01-0.07,
  comes mostly from the casualty clock plus an assumed bout rate. Using it as the band would
  test the clock twice, so the band uses only the evidence that stands on its own:
  - **Upper end, 0.15.** Gladiator fights went to a decision in perhaps 0.05-0.2 of exchanges,
    between men with little armour and no neighbours. Common healed wounds and veterans of
    several battles argue against much more than 0.1 for formed shield infantry.
  - **Lower end, 0.005.** This is the weakest number in the file. It rests only on "several
    times below gladiators". It is also what decides the vision of record's kill criterion ("a
    near-harmless clash"). Paullus proposes it and builds that engine, so Varro should set or
    accept it.
  - Varro's half of the search (ethnography of line fighting, Collins) may narrow either end.
- **Pattern 8:** Caesar's centurion losses are high in a formed fight and lower in a repulse
  or a panic (rival 04 §5).

**Dose-response, for pattern 6, across variants, with the casualty clock held fixed.** Each
statistic below is the probability of superiority over seeds, with "never broke" counted as
later than any time:

- recruits break sooner than veterans: at least 0.6;
- a line attacked in the flank breaks sooner than one not attacked: at least 0.6;
- a line relieved by a second line breaks later than one without relief: at least 0.6.

The absolute time to break is close to an input in both designs, so it is scored by direction
and size, not by value (rival 04 §4).

**Reported, not banded: the shape of the time to break.** For each cell, a Weibull fitted by
maximum likelihood to each parameter set's times to break over its seeds, with seeds unbroken
at two hours censored there (`weibull_shape` in the metrics). The report gives the median shape
over parameter sets and its spread. A shape near 1 is a line whose chance of breaking does not
change with time; above 1, a line that grows brittle. Patch scenarios run 7,800 s, so that
contact up to 10 minutes in still leaves two hours to watch.

## The variants

| Variant | Change from the base patch |
|---|---|
| `base` | Two centuries side by side (10 files by 8 ranks each) against a warband of equal numbers (20 by 8) |
| `veterans` / `recruits` | The Roman troop type: veterans and recruits against the same warband |
| `flank` | A small third body arrives on one flank of the Roman line after contact |
| `relief` | A second Roman line stands behind the first and relieves it in a lull |
| `volleys` | Missiles that cannot be answered and kill few, then a charge |
| `shock` | An attack from behind that is driven off, then pressure from the front |

The last two test whether an earlier fright should change what happens next (rival 01 §9).

## Sign-off

| | Agreed | Commit | SHA-256 of `prereg.toml` |
|---|---|---|---|
| Paullus | pending | | |
| Varro | pending | | |
