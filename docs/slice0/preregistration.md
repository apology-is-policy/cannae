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
- **The priors,** in three files, counted line by line:
  - `docs/slice0/priors-shared.md`: every parameter of the shared core that both engines run.
    That covers the frames' ties, the bout resolver, and a troop table (reach, guard and skill;
    critical power and W' per troop type). Varro writes the ties, bouts and kits, Paullus the
    stamina, power and gait. Each signs the other's.
  - `docs/slice0/priors-field.md` (Varro) and `docs/slice0/priors-pairwise.md` (Paullus):
    only what is the engine's own. That means the substrate, and the man's rungs: what a troop
    is like inside, as opposed to what it fights with.

  If each builder drew the shared parameters from his own prior, the experiment would compare
  priors as much as engines.
- **The lethality bounds** from the S0.4 literature search.

## The procedure

For each cell of the experiment (each rung of the memory ladder on each substrate):

1. Draw 200 parameter sets: the shared parameters from the shared priors, the rest from the
   engine's own.
2. Run each set on the same 10 seeds, for each scenario variant.
3. Compute a run summary per run, from observable state and the event log only. After the
   first side breaks, the rout is scored for 15 minutes and no longer, so a run can stop there
   and score as if it had run on ([scored-runs-cost.md](scored-runs-cost.md); proposed by
   Paullus, agreed by Varro).
   - **The audit, reported and not banded** (Varro): for one parameter set in twenty (draws
     0, 20, 40 and so on), the seeds are also run to the end. The report says how often
     anything happens after the window: a bout, a rally, a second break.
   - **What counts as happening after the window,** among front men as scored: a bout
     between two of them; a man who turns from flight back to face the enemy (a rally); a
     side that had not broken by the window's close breaking after it (a second break). The
     audit also checks that each audited seed's stopped run and whole run summarise the
     same.
4. Pool each per-run statistic over a parameter set's seeds by its median, and compare it with
   the band.

**Scoring a parameter set** (agreed in call 0001, turns 43-44; rival 06 §2):

- **The eight bands are scored on `base`,** per parameter set, over its 10 seeds.
- **Pattern 6 passes only if** its band passes on `base` **and** all three dose-response
  statistics below reach 0.6 for the same parameter set. The report gives pattern 6 both with
  and without them, so a reader sees which half failed.
- **A set passes** when all eight patterns pass, pattern 6 as above.
- **`shock` and `volleys` are reported only** (below).

**Code:** `crates/cannae-score` (`cell` writes each run to disk, `score` scores a cell),
with the bands and statistics in `crates/cannae-harness/src/metrics.rs`.

**Report:**

- the number of free parameters;
- the share of parameter sets that pass all eight patterns;
- which patterns fail most often, and how.

**The knife-edge rule.** A cell passes only if at least 10 % of its parameter sets pass all
eight patterns. A model that passes only on a knife-edge has failed, whichever engine it is
(vision 04 §5.3). The 10 % is agreed (call 0001, turn 44). **It is applied to the observed
share as it comes.** The report gives each cell's share with its 95 % binomial (Wilson)
interval. At 200 sets, a true 10 % is observed anywhere from about 6 % to 14 %. Both sides
sign now that nobody argues from the interval afterwards.

**Common draws.** Draw *i* uses the same shared values in every cell, from a seed fixed at
the freeze, so cells do not differ by the noise of the shared priors. **The freeze seed** is
the first 8 bytes, read big-endian, of the SHA-256 over `scenarios/prereg.toml`,
`priors-shared.md`, `priors-pairwise.md` and `priors-field.md`, concatenated in that order as
frozen (agreed in call 0001, turns 35-36). Neither side can choose it without changing what
both signed. A drawn value depends only on that seed, the name of its line and the draw's
number (`crates/cannae-score/src/draw.rs`). The ladder is nested in
the same way. What a man sees, and how much each thing he sees weighs (the alarm and urge sums
of the visible layer), is one set of weights for every rung. On one substrate, draw *i* of
a rung is draw *i* of the rung below plus that rung's new parameters. So rung 3 (the
continuous gauges) adds memory on top of the same perception weights as rungs 0-2, and nothing
else changes. Rung 3 keeps rung 1's *has fled* and replaces rung 2's *shaken*, so with its
gauges switched off it is rung 1, event for event (tested).

**The memory ladder.** Within each substrate, the lowest rung that passes is kept. The ladder
scored is rungs 0-3. **Rung 4** (the full excitable psyche) **is built only if rungs 0-3 all
fail** (Michal, 2026-11-10). It would then be pre-registered on its own, with its own freeze.

**The press** (Michal, 2026-11-10). Only men out of their places press: a man closing on an
enemy (`Intent::Close`), at a gate, in the churn at a pocket's edge. A man in his place does not
press in Slice 0 (`HoldSlot` press is zero), on either substrate. **The shared prior on press is
a prior about men out of their places only. Slice 0 does not answer whether a formed line
shoves: that is put off to Slice 1, not settled**, and the report says so. One difference
remains, and it is the substrate question itself, not a gap: in the pairwise engine a loose
man who presses shoves a formed man, while in the field a man in his place is moved by his tie
alone. The interval passage shows it, and it is reported as it comes.

**If no rung passes** (written before any run, at Varro's proposal):
- the rungs are still ranked, first by the share of parameter sets passing all eight patterns,
  then by the median number of patterns passed;
- the report says plainly "no rung passed";
- nobody narrows a prior and reruns under this pre-registration. A second round is a new
  pre-registration, with its own freeze.

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
| 2 | Pulses and lulls | Burstiness of the gaps between successive bouts while formed, per 9 m sector of front (by the defender's place at setup), median over sectors with at least three gaps; the share of 10 s bins at 20 % of the busiest or less | At least 0.2; 0.2-0.9 |
| 3 | Rear-first collapse | The mean setup depth (0 front, 1 rear) of the first tenth of each broken body to turn within its collapse | At least 0.6 |
| 4 | Lateral spread | Spearman's rho between a 2 m bin's distance from the first break and its first turn within the side's collapse; the time for 80 % of the line's bins to turn | At least 0.3; 10-900 s |
| 5 | Casualty asymmetry | The share of the loser's hits after the break | At least 0.6 |
| 6 | Holds, then breaks | Across seeds: the share that break within 5 minutes of contact; the share still unbroken two hours after contact | At most 10 %; at most 25 % |
| 7 | The casualty clock | Hits per front file per hour while formed, each side against its own first-line files; lethality per bout | 0.1-2.0; 0.002-0.15 (proposed) |
| 8 | The centurion gradient | Leaders' share of hits while formed, against after the break, pooled over seeds | Higher while formed |

**The reasons for the bands:**

- **Pattern 1:** lines face each other just out of reach (Sabin; rival 02 §7). Five minutes is
  short against a battle of hours, but long enough to rule out lines that close straight to
  the death. Since both engines halt an advance at the shared standoff (priors-shared.md,
  S5), the gap is close to an input. It still fails if more than a tenth of a body is forward
  most of the time, so it tests that fighting is the exception (Varro).
- **Pattern 2:** burstiness, after Goh and Barabási, is computed on the gaps between bouts,
  where a random process scores 0 at any rate. The first draft computed it on counts per bin,
  where random fighting slower than one bout in 23 s across the front would have scored above
  0.2 (Varro's arithmetic: (1 - √m) / (1 + √m) for a mean of m per bin). Flurries in
  different stretches of the front, pooled, add up toward random, so it is taken per 9 m
  sector. Pulses need bursts, and lulls need quiet bins, but not only quiet bins.
- **Patterns 3 and 4: the collapse.** A body's collapse runs from the last sample at which
  under a quarter of the break share of its standing men (10 %) had turned, to its break. It
  reaches back from there through any earlier turns of the body's men that come in an unbroken
  chain, each within 30 s of the next, so that the men who begin a cascade belong to it.
  Pattern 4 does the same for the losing side as a whole. Only turns inside the collapse count,
  so one man who ran an hour early cannot set either statistic. Men who turned before the
  collapse are reported as stragglers, per run, and not scored. (Varro proposed the 10 %
  start; the 30 s chain is Paullus's, after a test showed the bare threshold dropped the first
  tenth of a cascade, the very men pattern 3 is about.)
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
- **Pattern 7, lethality per bout (proposed: 0.002-0.15).** This is the chance that either
  man is disabled or killed in one committed bout. The skeletal and experimental half of the
  search is in [lethality-skeletal.md](lethality-skeletal.md). Its central range, 0.01-0.07,
  comes mostly from the casualty clock plus an assumed bout rate. Using it as the band would
  test the clock twice, so the band uses only the evidence that stands on its own:
  - **Upper end, 0.15.** Gladiator fights went to a decision in perhaps 0.05-0.2 of exchanges,
    between men with little armour and no neighbours. Common healed wounds and veterans of
    several battles argue against much more than 0.1 for formed shield infantry.
  - **Lower end, 0.002.** Still the weakest number in the file. Paullus proposed 0.005 and
    builds the engine it decides ("a near-harmless clash" is the vision of record's kill
    criterion), so he asked Varro to set it. Varro's half of the search
    ([lethality-line-fighting.md](lethality-line-fighting.md)) found nothing that measures harm
    per exchange between armoured men. Two weak and independent lines of arithmetic, from
    Italian duel statistics and from the worst Maring case, give 0.0013-0.038 and
    0.0002-0.011. The band is an outer envelope, so Varro moved the end down to 0.002: toward
    the engine with more fighting and less harm in each bout, which is not his.
  - **Since the engines share one bout resolver, harm per strike is a shared draw**
    (priors-shared.md, S8), inside this band. So the band is close to an input for both. What
    pattern 7 tests is whether each engine's tempo, at the harm it is dealt, lands on the
    clock.
  - **Tempo, reported and not banded:** the share of the first line's front-rank man-time
    spent forward while formed, and the share of those men who start any bout. The only
    figures (5-25 % of the time, 10-25 % of the men) rest on Dani skirmishing and an inference
    (lethality-line-fighting.md): too little to fail an engine on.
- **Pattern 8:** Caesar's centurion losses are high in a formed fight and lower in a repulse
  or a panic (rival 04 §5).

**Dose-response, for pattern 6, across variants, with the casualty clock held fixed.** All
three variants change the Roman side, so each statistic compares **how the Roman line fared**
(agreed in call 0001, turns 43-44). Each run gets one value, ordered worst for the Romans
first:

1. the Roman side broke first, at time *t* after contact: the earlier, the worse;
2. neither side broke;
3. the warband broke first, at time *t* after contact: the earlier, the better.

A Roman line broken before the fronts met has the worst value there is. If both sides break
at the same tick, the Romans count as broken first, the conservative side. The statistic is
the probability of superiority on that value over the 10 × 10 pairs of a parameter set's
seeds, ties counted half (`fortune` and `superiority_of` in the metrics):

- the Roman line fares worse with recruits than with veterans: at least 0.6;
- it fares worse attacked in the flank (`flank`) than not (`base`): at least 0.6;
- it fares worse without relief (`base`) than with it (`relief`): at least 0.6.

**Why not the time to break alone.** The first break of either side, as a time, would score
a warband that breaks sooner against recruits as recruits breaking sooner. The Roman side's
break time alone, with "never" tied, gives a variant no room to respond in a set where the
Romans usually win: `relief` could not reach 0.6 unless the unrelieved line broke in at least
2 seeds of 10. That would fail a set for having no room to respond, not for responding
wrongly. The value above lets one thing through, which the report names: veterans who hold
no longer than recruits but beat the warband sooner pass on skill at arms alone.

**Reported beside it:** the same three statistics on the Roman side's break time alone, with
"never" later than any time, and the number of sets where the Roman line broke in fewer than
2 seeds of `base`. A reader then sees whether any verdict hangs on the definition.

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
| `volleys` | Missiles that cannot be answered and kill few, then pressure from the front |
| `shock` | An attack from behind that is driven off, then pressure from the front |

The last two test whether an earlier fright should change what happens next (rival 01 §9).
`shock` is written (`scenarios/patch/shock.toml`): a band of 36 charges the Roman rear at one
minute and is ordered off at three, and the warband comes on from about 58 m at six. A test
checks that the band strikes and goes before the front fights.

`volleys` is written (`scenarios/patch/volleys.toml`) on the same clock, so that the two differ
only in the fright. Missiles fall on the Roman line from one minute to three, with no enemy in
reach to answer, and the warband comes on at six by the base patch's order.

- **The fall is scripted and nobody throws it,** as the band is ordered off and not beaten off:
  every run has the same fire to remember.
- **The dose,** of every thousand men who stand under it throughout: 75 hampered, 20 disabled,
  5 killed. That is about 4 men felled and 12 hampered of 162.
  - Its shape is that of a fight with missiles: 10 to 30 wounded for each man killed, most of
    them lightly ([lethality-line-fighting.md](lethality-line-fighting.md), A4).
  - Its size is Varro's choice of "few" and has no source. It is the script's, not a prior:
    both engines and every rung get the same fire, draw for draw.
- **A test checks** that the fire falls on the Romans alone and only while it lasts, that
  nobody fights before the front comes on, and that what it cost is reported as the Romans'
  losses before contact.

`relief` is written (`scenarios/patch/relief.toml`): the base patch unchanged, and behind each
century a second century of the same men, 24 m back and half a file to one side.

- **The drill** is in the shared frames ([rival 02](../rival/02-order-and-contact.md) §4, as
  built). Each century is relieved once, by the one behind it, in its own lull, and not before
  ten minutes. The fresh century comes up through the tired one's files and takes the front;
  only then does the tired one fall back, to where the fresh one stood.
- **A lull** is twenty seconds in which no man of the tired century is forward, giving ground
  or in flight, and no enemy before it has stepped forward.
- **The 24 m, the ten minutes and the twenty seconds are Varro's choices and have no source.**
  How relief worked is not known (vision 03 §3). They are the script's, not priors.
- **Until a man turns his back, a run of `relief` is the run of `base` with the same seed,
  event for event** (tested on both engines). The second line stands out of every window, and
  it comes last in the file, so every man of the base patch keeps his number.
- **Tests check** the drill with men placed by hand, and the passage on both engines with
  men who never fight: nobody is hurt, the enemy gains no ground, and every man ends in his
  place.

**How the two memory variants are scored** (agreed in call 0001, turns 37-38):

- **Only the fronts are scored.** A body marked `front = false` in the scenario, such as
  `shock`'s band, starts no contact, breaks no side, and a hit on one of its men counts in no
  pattern. Contact is the first bout between two men of front bodies, or a gap of 3 m or
  less between the front bodies.
- **A second line is not the front** (call 0001, turn 46). A side's front, for the gap, is
  made by the facing men of its front bodies that are not in reserve. A body is in reserve
  while it is wholly behind another body of its side that covers at least half its width.
  Without this, a second line's men, counted as a share of the side, would pull the front
  back a rank. Before a relief, `relief`'s first line then meets the enemy exactly as
  `base`'s does. 40 run summaries over five variants, both engines and two rungs came out
  the same before and after. The formed phase and the casualty clock start there. For the
  variants with no such body nothing changes: 24 run summaries over three variants, both
  engines and two rungs came out the same before and after.
- **Reported:** each side's front losses before contact, so that a shock that killed men can
  be told from one that frightened them.
- **A line that breaks before the fronts meet** breaks at once, as far as the comparison goes:
  the shock alone broke it.
- **The statistic, reported and not banded:** for each cell and parameter set, the probability
  of superiority over seed pairs that the Roman line fares worse in `shock` than in `base`,
  by the same value as the dose-response (agreed in call 0001, turn 44). The same for
  `volleys` against `base`. The report gives its distribution over parameter sets, rung by
  rung.
- **Rung 0 is the control.** It has no memory, so its difference from `base` is only what the
  shock left behind: losses, disorder, breath, the longer approach. A memory shows itself as a
  difference larger than rung 0's.
- **Expected, written beforehand:**
  - Rung 1 shows more than rung 0 only where men fled from the shock and rallied, since *has
    fled* is all it carries (Varro).
  - Rung 2 shows more than rung 1 only in draws where *shaken* wears off more slowly than the
    four minutes between the shock and the front. The prior on the wear is 2-10 minutes, so
    Varro expects that in about half the draws.
  - Rung 3 shows more than rung 2 ([rungs-3-4.md](rungs-3-4.md)).
- `volleys` has no body in reach: its missiles start no bout, so contact is untouched. Any
  skirmishers it gets are `front = false`.
- **What `volleys` cannot test** (call 0001, turns 42-43). Men see only the falls a volley
  causes: the visible layer, shared by every rung, has no row for missiles that fell nobody.
  So Slice 0 does not test fear of the fire itself, only what seen falls and losses leave
  behind. Rung 3's expectation is amended to match ([rungs-3-4.md](rungs-3-4.md)). A row for
  being under fire waits for a slice whose named pattern needs it.

## Sign-off

| | Agreed | Commit | SHA-256 of `prereg.toml` |
|---|---|---|---|
| Paullus | pending | | |
| Varro | pending | | |
