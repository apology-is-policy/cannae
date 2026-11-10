# Slice 0: Pre-registration

**Status:** written by Paullus and agreed with Varro, 2026-11-10 (call 0001), and frozen
before any scored run. Freezing records, in the sign-off below and in [STATUS](../STATUS.md),
the commit and the SHA-256 of [`scenarios/prereg.toml`](../../scenarios/prereg.toml) and of
the three priors files, the freeze seed, and the hash of the table of record. After that,
changing a band or a definition voids the runs scored under it.

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
2. Run each set on the same 10 seeds, for each scenario variant. `relief` and `backed` are run
   on every fourth set only (0, 4, 8 and so on: 50 of 200), because nothing scored for a set
   reads them (call 0001, turns 51-52). The audit's sets are all among them.
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
- **Pattern 6 passes only if** its band passes on `base` **and** both dose-response
  statistics below reach 0.6 for the same parameter set. The report gives pattern 6 both with
  and without them, so a reader sees which half failed.
- **A set passes** when all eight patterns pass, pattern 6 as above.
- **The relief is reported for the cell, not scored for the set** (below; call 0001,
  turns 46-48).
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

| # | Pattern | Statistic | Band |
|---|---|---|---|
| 1 | A stable standoff | The formed phase's length; the median gap while formed | At least 300 s; at least 1.0 m |
| 2 | Pulses and lulls | Burstiness of the gaps between successive bouts while formed, per 9 m sector of front (by the defender's place at setup), median over sectors with at least three gaps; the share of 10 s bins at 20 % of the busiest or less | At least 0.2; 0.2-0.9 |
| 3 | Rear-first collapse | The mean setup depth (0 front, 1 rear) of the first tenth of each broken body to turn within its collapse | At least 0.6 |
| 4 | Lateral spread | Spearman's rho between a 2 m bin's distance from the first break and its first turn within the side's collapse; the time for 80 % of the line's bins to turn | At least 0.3; 10-900 s |
| 5 | Casualty asymmetry | The share of the loser's hits after the break | At least 0.6 |
| 6 | Holds, then breaks | Across seeds: the share that break within 5 minutes of contact; the share still unbroken two hours after contact | At most 10 %; at most 25 % |
| 7 | The casualty clock | Hits per front file per hour while formed, each side against its own first-line files; lethality per bout | 0.1-2.0; 0.002-0.15 |
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
- **Pattern 7, lethality per bout (0.002-0.15).** This is the chance that either
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

**Dose-response, for pattern 6, across variants, with the casualty clock held fixed.** The
variants all change the Roman side, so each statistic compares **how the Roman line fared**
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
- it fares worse attacked in the flank (`flank`) than not (`base`): at least 0.6.

Ten seeds a side give each statistic a standard error of about 0.13, so a set whose true
value is 0.7 still fails about one time in five. Both engines carry that noise alike. The
effects of troop quality and of a flank attack should be large, so it costs little there.

**Why not the time to break alone.** The first break of either side, as a time, would score
a warband that breaks sooner against recruits as recruits breaking sooner. The Roman side's
break time alone, with "never" tied, gives a variant no room to respond in a set where the
Romans usually win: a variant that helps them could not reach 0.6 unless the line without
it broke in at least 2 seeds of 10. That would fail a set for having no room to respond, not for responding
wrongly. The value above lets one thing through, which the report names: veterans who hold
no longer than recruits but beat the warband sooner pass on skill at arms alone.

**Reported beside it:** the same two statistics on the Roman side's break time alone, with
"never" later than any time, and the number of sets where the Roman line broke in fewer than
2 seeds of `base`. A reader then sees whether any verdict hangs on the definition.

**The relief, reported for the cell and not scored for the set** (call 0001, turns 46-48).
The prereg first asked that a line relieved by a second line break later than one without
relief, at 0.6 for each set. Two faults were found before any run:

- **Against `base`, the leg passes by arithmetic** (Varro). A side breaks when 40 % of its
  standing men have turned. With a second line the Roman side has twice the men, so the rout
  of one century no longer breaks it, whether the second line ever moves or not.
- **Against a line merely backed, it passes by chance** (Varro's sum, checked by Paullus).
  Take an unrelieved line whose time to break is a Weibull with a mean life the pattern 6
  band allows, relieved once by a fresh line of the same kind. The chance that it fares
  better relieved than backed is 0.50 at shape 1 whatever the time of the relief, about 0.6
  at shape 2, and 0.72-0.76 at shape 3 when relieved at 20-30 minutes. So 0.6 for one set
  would band the shape, which is reported and not banded because no source gives it. And at
  ten seeds a side, a set with no effect passes about one time in four, and a set with the
  largest effect a shape of 2 allows passes about six times in ten.

So:

- **`backed`** is `relief` with no relief: the same two lines and the same men, with the
  second line never moving. Both run on every fourth set (0, 4, 8 and so on). Pooled over
  those 50 sets, the drill's statistic has a standard error of about 0.02.
- **The top-up, registered now and run only if Michal gives the machine time for it**
  (Varro, turn 52). `relief` and `backed` are also run on the other sets in which `base` was
  still being fought twenty minutes in, with contact made and neither side broken, in at least
  2 seeds of 10: lowest set numbers first, at most 25 sets a cell. Its twins and its share
  relieved are reported apart from the first 50 sets'. Whether to run it is decided on the
  machine time alone, before any relief statistic of the first 50 sets is read, and the report
  says which. `score` names the sets (`report::topup`).
  - What to expect (Varro): twins come only from sets whose line lasts twenty minutes. A cell
    that passes the 10 % rule has about five such sets among the 50, so about 50 twins. Their
    share of better against worse has a standard error near 0.07, and a difference between
    two cells near 0.10. That shows a strong effect of the drill, but not rung 3 over rung 2
    unless the difference is large.
- **The drill alone, reported for each cell:** the mean over sets of the probability that
  the Roman line fares worse backed than relieved, with its 95 % interval; the share of sets
  above 0.5; and its rank correlation with each set's Weibull shape, which should be
  positive. Pooled over sets its standard error is about 0.01, where one set's is 0.13.
- **The whole effect of a second line, reported:** the same statistic for `base` against
  `relief`, arithmetic and all.
- **Whether the drill happened, reported:** the share of a cell's `relief` runs in which a
  relief was ordered, and in which both were. A run with none is its `backed` twin, man for
  man, and pulls the drill's statistic toward 0.5.
- **Each relieved run beside its twin, reported** (Varro, turn 50). A run of `relief` and the
  run of `backed` with the same set and seed are the same battle until the relief is ordered.
  For the relieved runs, the report gives how many fared better than their twin, how many
  worse and how many the same. That comparison carries none of the noise between seeds.
- **Pattern 6 keeps two legs.** Dropping the third makes pattern 6 easier for both engines.
  Neither side can say which it helps more.
- **Expected, written beforehand:** rung 3 shows the drill's effect more than rung 2. Its
  resolve drains through a fight, and a fresh century starts full ([rungs-3-4.md](rungs-3-4.md)).

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
| `backed` | `relief` with no relief: the second line stands behind the first and never moves |
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
  twenty minutes. The fresh century comes up through the tired one's files and takes the
  front; only then does the tired one fall back, to where the fresh one stood.
- **A lull** is twenty seconds in which no man of the tired century is forward, giving ground
  or in flight, and no enemy before it has stepped forward.
- **The 24 m, the twenty minutes and the twenty seconds are Varro's choices and have no
  source.** How relief worked is not known (vision 03 §3). They are the script's, not priors.
  The time was ten minutes at first. The sum above moved it, before any run: one relief
  helps least when it comes early.
- **A line that breaks before its relief is never relieved, and its run is then the run of
  `backed`, man for man** (tested on both engines through a fight and a rout). Every such
  run pulls the drill's statistic toward 0.5. So each run's record says when its reliefs
  were ordered (`relieved_s`), and the report has to give, for each cell, the share of
  `relief` runs in which one was.
- **At the nominal set no relief comes,** even when one is allowed from half a minute. The
  warband's onset leaves no lull, and the Roman line breaks before the onset is spent. With
  the same men made unable to flee, the first lull came about two minutes in on both
  engines, and the reliefs were ordered in it
  ([rival 02](../rival/02-order-and-contact.md) §4). So the drill can only show in sets
  where a line lasts twenty minutes, which the band for pattern 6 asks of them anyway.
- **Until a man turns his back, a run of `relief` is the run of `base` with the same seed,
  event for event** (tested on both engines). The second line stands out of every window, and
  it comes last in the file, so every man of the base patch keeps his number.
- **Tests check** the drill with men placed by hand, and the passage on both engines with
  men who never fight: nobody is hurt, the enemy gains no ground, and every man ends in his
  place. With the patch's own men unable to flee, they check a relief in the first lull of a
  real fight, and that dormancy is exact through it.

`backed` is written (`scenarios/patch/backed.toml`): `relief.toml` without its two reliefs,
and nothing else changed. A test holds the two files to that.

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

**What the freeze pins.** The hashes pin the text of `scenarios/prereg.toml` and the three
priors files. The commit pins everything else the runs read: the sampler's lines
(`crates/cannae-score/src/draw.rs`), the metrics (`crates/cannae-harness/src/metrics.rs`), the
scorer with its thresholds of 10 %, 0.6 and the relief's stride
(`crates/cannae-score/src/report.rs`), both engines, and the scenario files in
`scenarios/patch/`. So the hash alone is not the whole freeze (Varro, turn 52). Scored runs use
the frozen commit's code and scenarios. A later commit may be used only if
`git diff <frozen commit> -- crates scenarios Cargo.toml Cargo.lock` is empty, and the report
names the commit it ran.

**The record** (computed by Paullus, to be checked by Varro):

| | |
|---|---|
| Commit | `dd9d90af15c25851d9c680cb10b189b496ebd071` (dd9d90a) |
| SHA-256 of `scenarios/prereg.toml` | `47d7d953ae213dece59a896fb175a95b9f2aa8a42507808340064ec8331c800f` |
| SHA-256 of `docs/slice0/priors-shared.md` | `14213001066b4e66bf77ffb969a42806b8981169379c171590bcac2a81807a52` |
| SHA-256 of `docs/slice0/priors-pairwise.md` | `d09264e502e01feee1a573ace235a8db26b8d966dbf74ecafca030396d034a14` |
| SHA-256 of `docs/slice0/priors-field.md` | `c14b694a9d9128cacdfa6379ad30c338827c7a7ffd4238b778d22173ccaba9b1` |
| Freeze seed: the first 8 bytes, big-endian, of the SHA-256 over the four in that order | SHA-256 over the four: `835a02dd1328c76b7a71517b446ab5cc778af237ca71a9a3961d97dd4cc6213f`; seed `0x835a02dd1328c76b` = **9464880715404986219** |
| SHA-256 of the table of record, `table --freeze-seed <seed> --draws 200` | `2139655ba18e120ab277019d54d78813517383ed379a75235aad935c3e5f3c4f` ([table-of-record.csv](table-of-record.csv), 200 sets × 101 lines) |

| | Signed | What he checked |
|---|---|---|
| Paullus | **signed, 2026-11-10** | Computed every number above at dd9d90a: each file's SHA-256 and the concatenation's with `shasum -a 256`, the seed from its first 16 hex digits, the table from that commit's `table` binary, written twice with the same hash |
| Varro | **signed, 2026-11-10** | Recomputed every number above myself, at dd9d90a. Each file's SHA-256 and the hash over the four, twice: from the commit's own blobs with `shasum -a 256`, and from the working tree with Python's `hashlib`. The seed from the first 8 bytes of that hash. The table from a `table` binary I built myself: byte for byte the committed file, with the recorded hash. Its 101 columns are the sampler's 101 lines, all 20,200 values lie inside their lines' ranges, and no two sets are alike. Read every change made to the four files after my own cells (04a1f09): text only, with no range, kind or centre touched. The sampler and the constants are unchanged since the three priors files were read against them line by line at 6b73531. Nothing under `crates`, `scenarios`, `Cargo.toml` or `Cargo.lock` differs between dd9d90a and 929c8e9, where 209 tests pass. Not checked by me: the pairwise priors against their sources. |

**Before the batch:** Michal sees the pass rules settled on 2026-11-10 (the eight on `base`,
pattern 6 with two legs and how the Roman line fared, the 10 % share, the relief reported per
cell, the stride), and gives the machine time. Signing binds nothing until the first scored
run.

## Addendum 1 (2026-11-10): cells that `base` rules out run their variants last

Written after the freeze, while the batch ran, and agreed on call 0001 (turns 58-60). It changes
the order of the runs and nothing else. The frozen code is untouched: the batch's
`git diff dd9d90a -- crates scenarios Cargo.toml Cargo.lock` stays empty.

**What was known when it was written** (about 19:10 UTC). The Pi had made 957 of the 2,100 `base`
runs of the first cell (the field engine, rung 0). No other cell had begun. Neither of us had
scored a record.
- **Varro** had read no record of the scored batch.
- **Paullus** had read line counts, and two runs while testing the crosswise check (Addendum 2):
  - the field engine, rung 0, set 0, seed 1, compared field by field with its batch record. The
    script printed its simulated length.
  - the pairwise engine, rung 3, set 10, seed 1, a rerun at the frozen code, not yet made by the
    batch. Its record's opening, with the break times, was printed.

**The rule** (Paullus's text, with Varro's order):

> When `base` is complete in all eight cells, a cell can no longer pass if the share of its sets
> that pass all eight bands on `base` is below 10 %. Pattern 6 counts by its band alone here. The
> comparison is the knife-edge rule's own (`share >= KNIFE_EDGE` passes), so a cell with 20 of
> 200 can still pass.
>
> The dose-response legs can only take passes away, so the rule is exact.
>
> **All the variants of the cells that can still pass run first,** in the usual order. Then all
> the variants of the cells that cannot, in the same order. Every variant of every cell still
> runs, and nothing is scored differently.

**How it is applied.** `tools/s09-batch.sh` runs `base` everywhere, then asks a `score` binary
for each cell's gate and logs its line in `progress.txt`. The binary is built from branch
`spike-perceive` at 91185c4, and it differs from the frozen scorer by:
- one printed line, the gate;
- the crosswise switch below;
- the two hashes in each record.

No band is judged differently. If a cell's `base` is incomplete (a run failed), the gate is not
applied to it, and its variants keep their place. The script holds to that by deferring a cell
only when its gate line counts all 200 sets: the scorer counts the sets it finds, and a `base`
that stopped at a set's end leaves no incomplete set to warn it.

## Addendum 2 (2026-11-10): the crosswise check of the two perceptions

Proposed by Varro, agreed on call 0001 (turns 58-60). The engines perceive differently:
- the pairwise engine through the core's reference grid;
- the field engine through Varro's `FieldPerception`.

Both are held to the same View on random crowds and through one three-minute fight. But no
scored battle had been run both ways. If they differ, the two engines' men do not see the same
thing, and the comparison is confounded.

**The check.** Run each engine through the other's perception (`cell --eyes swapped`, branch
`spike-perceive`) on a sample: sets 0, 10, 20 and so on to 190, seed 1, `base`, every rung, both
engines. That is 160 runs each way. Each record carries two hashes:
- the whole observable state at the end;
- the whole event log.

Only equal or unequal is read.
- **Native against swapped:** both hashes.
- **Native against the batch's records,** once they exist: summary, simulated seconds, audit and
  relief, field for field.

**If any run differs,** we stop and tell Michal before anything is scored.

| Addenda 1 and 2 | Signed |
|---|---|
| Paullus | **signed, 2026-11-10** |
| Varro | **signed, 2026-11-10**, after reading the branch's diff of the scorer (91185c4) and the script. The gate counts the sets whose eight verdicts on `base` all pass, and every set that passes in the end is among them. I added the script's check that a gate covers all 200 sets |

## Addendum 3 (2026-11-10): a defect in the frames, and `base` repeated with it fixed

Written after the freeze, while the batch ran, and agreed on call 0001 (turns 71-74). Michal
decided on 2026-11-10 that the batch carries on, over stopping, fixing, freezing again and
starting over. The frozen code is untouched.

**The defect.** When a file closes over a man who stood ahead of the body's standard-bearer,
the standard-bearer is renumbered one rank forward while he stands where he was. The frame is
anchored on him, so it is re-formed one rank back, and every other man of the body walks back a
rank, still steady. The same happens whenever his slot changes while he carries the standard:
also at a succession in his file, and when the closer's post moves, if the closer carries it.
It happens once for each rank he is moved up: once for the man who carries the standard at the
start, who stands in the second rank, and more than once for a man who lifts it from further
back. It was seen in the viewer at the nominal set, not in a scored run; the sighting is in the
table below. It is in every scored run, on both engines alike.

**What was known when it was written.** No record had been scored. The batch had finished
`base` in one cell (the field engine, rung 0) and was part-way through the next. Every scored
outcome either of us had seen is listed in the addenda above and in the table below.

**The rule.**

> The batch runs to its end on the frozen code. The pre-registered verdict of every cell is the
> one scored from it.
>
> After the batch, `base` is repeated in all eight cells with this defect fixed and nothing else
> changed: the same 200 sets, the same ten seeds, the same stop rule and audit. The repeat is
> scored by the same scorer on the eight bands, pattern 6 by its band alone, as in Addendum 1.
> The dose-response variants are not repeated. The report gives both verdicts for every cell,
> side by side. The repeat changes no pre-registered verdict.
>
> All eight cells are repeated, and not only those that pass. A defect can hide a pass as well
> as make one, and a check that could only take passes away would be one-sided.

**What "only this defect fixed" means, in a form that can be checked** (Varro's conditions):

- **The fix is named by what it does** (Varro's design, call 0001, note 45). The commit's hash
  is added below as a dated line when the commit exists.
  1. When the standard-bearer's slot changes while he carries the standard, the frame is not
     re-formed on him. He walks to his new slot. The cases: a file closing over a man ahead of
     him; a succession in his file; the closer's post moving, if the closer carries it.
  2. Until he reaches it the body stands, whatever its order. A frame on the march is defined
     by where he stands, so he cannot gain a rank on a frame that moves with him.
  3. While he walks, the frame moves only if he is driven further than the tolerance from the
     straight path between his old slot and his new one. Then it re-forms on that path, so a
     pressed body can still give ground.
  4. No new parameter. A standard-bearer who was adrift and falls in again is unchanged.
- **The fixed build counts** the times a frame was held this way, which are the times the
  defect would have occurred.
- **A run in which that count is zero must be unchanged by the fix.** Varro tests this on
  unscored sets before the repeat, event log against event log. For the repeat itself the
  report states it against the batch's records, field for field, all but the wall time.
- **The same counter gives the report its count:** in how many scored runs of each cell the
  defect occurred, and whether those runs differ from the rest on any pattern.

**2026-11-10 23:15 UTC, Varro: the fix exists.** It is commit ed6d363 on branch `frames-step-up`, cut
from e573bf9, whose `crates/` are the frozen ones. It is not on main, and it stays off main
until the batch ends.

- **Two details of the rule as built.** The standard-bearer has reached his new slot when he is
  within the tie's tolerance of it, or level with it along his path. A second change of his
  slot before he has reached the first starts from where he then stands.
- **The count is `BodyFrames::held()`,** and the fixed build's record of a run carries it as
  one more field, `held`. A comparison with the batch's records leaves that field out, as it
  leaves out the wall time.
- **The test the third condition asks for, on unscored sets:** 144 runs with the frozen frames
  and with the fix (both engines, rungs 0 to 3, `base`, `relief` and `backed`, the nominal set
  and five sets drawn with seed 12345, seed 1, 600 s). The event logs were hashed, and nothing
  else of them was read. In the 3 runs whose count is zero the logs are equal. In the 141
  whose count is one or more they differ.
- **What that test also showed: the defect is in nearly every run.** The count was at least
  one in 141 of the 144, with a median of 2 and a largest value of 4. So in the scored batch
  the runs without it will be few, and the report's comparison of runs with it against runs
  without it will rest on few runs. The repeat is the stronger evidence. This goes to Michal
  with my signature, since it was not known when the batch was allowed to carry on.

| Addendum 3 | Signed |
|---|---|
| Paullus | **signed, 2026-11-10** |
| Varro | **signed, 2026-11-10**, with the fix written and tested as above. I agree with the rule, and with repeating all eight cells |

## Outcomes seen during the batch

A record of every scored run's outcome either of us sees before the batch is scored, as it
happens. The addenda's own disclosures (two runs, Paullus) are above.

| When (UTC) | Who | What was seen, and how |
|---|---|---|
| 2026-11-10 19:14 | Paullus | Two more runs of the field engine, rung 0, `base`: set 146 seed 5 and set 173 seed 3. The first lines of the batch's log were printed to check that it had resumed after the restart with the gate, and the log prints each run's outcome. From now on, only the log's first line or counts are read |
| 2026-11-10 22:32 | Paullus | **Not a scored run: a defect in the frozen frames.** Michal, watching the nominal set in the viewer (pairwise, rung 0, `base`, seed 1), saw a century step back as one at about 30 s. I replayed it and read every man's position. The leader of the standard-bearer's file went down at 27.05 s. When the file closed two seconds later (`close_files`), the standard-bearer was renumbered into the front rank, `anchor` re-anchored the frame on where he stood, and every other file walked back exactly one rank, 1,800 mm, still steady. It is in every scored run, on both engines alike. Nothing is changed and the batch goes on; the report counts how often it happened and whether those runs differ |
