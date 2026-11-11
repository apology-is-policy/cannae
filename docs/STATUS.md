# Status

**Phase:** Slice 0, the experiment between the two designs. The shared core and harness run end to end with a toy man; the engines are next.
**Working mode:** guided (see [CLAUDE.md](../CLAUDE.md)).
**Last updated:** 2026-11-11, morning.

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
| S0.3 | Metrics and pre-registration: the eight patterns computed from observable state; pass bands and both sides' parameter priors written down before any scored run | 0fbcc74 metrics and draft. **Frozen text at dd9d90a**, signed by Paullus; Varro checks every number and signs ([sign-off](slice0/preregistration.md#sign-off)) |
| S0.4 | Lethality bounds: literature search on skeletal trauma and experimental archaeology (Varro searches line-fighting ethnography and Collins) | f8cb2a6 Paullus's half; band proposed |
| S0.5 | The pairwise substrate, where every body touches its neighbours, at patch scale; a perception conformance check for Varro | 14e919b |
| S0.6 | The top two rungs of the memory ladder: continuous fear and resolve, then the full excitable psyche | Rung 3 built and plugged into Varro's men through his `Memory` hook ([design](slice0/rungs-3-4.md)). Its dormancy is exact through a fight and a rout; with its gauges switched off it replays rung 1's battle event for event. Rung 4 is built only if rungs 0-3 all fail (Michal) |
| S0.7 | The speed spikes: 150,000 bodies in a line battle, a pocket and a rout through camp gates, on both substrates | Pairwise done for the line, surge and pocket ([report](slice0/spike-pairwise.md)); the rout through camp gates is built (with walls indexed on a coarse grid, so a man checks only the walls near him) and waits for an idle machine to be timed; Varro's field spike is his |
| S0.8 | The passage study, against measured crowd-flow data | Walls (`cannae_core::Wall`, `[[wall]]` in scenarios) and the pairwise calibration done ([report](slice0/passage-calibration.md)). The bench for the two passages is built, and both engines have run on it ([report](slice0/passage.md), [field](slice0/passage-field.md)) |
| S0.9 | Scored runs and the report; then design review 2 | The batch runner is built (`cannae-score`), and it runs drawn parameter sets. Costed ([report](slice0/scored-runs-cost.md)): every run to the end would take about eight days of the Air. With Varro we agreed to stop scoring 15 minutes after the break, which brings it to one to three nights, with an audit of whole runs. The sampler is complete: 101 drawn lines, mine and Varro's. `shock` is written; `volleys` and `relief` are Varro's. How the two memory variants are scored is agreed and built: only the fronts are scored, so a band that strikes the rear starts no contact. The freeze seed is fixed by the files we sign. `cell` now writes each run to disk and picks up where it stopped, and `score` scores a cell. Writing the scorer turned up a gap in the pre-registration: it never said which variant the eight patterns are scored on, nor how the dose-response enters. Agreed with Varro: the eight on `base`; pattern 6 also needs the Roman line to fare worse with recruits, in `flank` and without relief. "Fares worse" is judged by who broke first and when, because a time to break alone would have scored a warband breaking sooner as the Romans breaking sooner. The knife-edge share of 10 % is agreed. `volleys` cannot test fear of the fire itself, only of the falls men see, and the pre-registration says so. `volleys` and the relief drill are built by Varro and merged. The relief is reported for each cell rather than passed or failed for each parameter set: against the base patch it would pass by arithmetic (a second line doubles the men a side needs to lose before it counts as broken), and against a line that is merely backed, ten seeds cannot tell a real effect from chance. A second line now no longer counts toward where a side's front is |

**Speed (pairwise substrate only, no man model yet; sustained readings on the fanless Air,
[report](slice0/spike-pairwise.md)):** 150,000 bodies on 4 cores for 10 minutes ran at 12.1x
real time in a line battle, 5.3x when every man keeps pushing, and 2.3x in a pocket (3.8x on 8
cores). The kill line is 3x with the man included, so **the pocket is this engine's main risk**.
The worst case was measured: everyone touching, a ring pressing forever, and no one ever
falling. The sustained run also found a physics bug: men could walk through a standing crowd,
up to 64 men per m². Now a man cannot walk into the men ahead of him, his lean fades as he
presses to the bone, and bodies have a hard core. The pressed edge settles at about 5 per m²,
the crowd-safety literature's onset of danger.

**The passage study.** A bench runs the same scripted passages on
any physics engine: a maniple of 120 falling back through an interval in the line behind it,
with and without the enemy shoving it, and a crowd of 600 making for a camp gate, walking or
running and shoving ([report](slice0/passage.md)). On my engine:

- **A maniple passes an interval in under a minute,** 17-33 s at every width from 3 to 12 m.
- **At a 3 m interval, the enemy's shove jams it.** The longest wait between two men rises
  from half a second to seven. This is the clogging measured at doors in experiments, and here
  it comes out of the physics, with no rule for it.
- **Walking flows match with discs of 300 mm; pushing through a door does not.** Walking
  without pushing matches the measured flows through narrow corridors and wide bottlenecks
  ([calibration](slice0/passage-calibration.md)). I first reported the wide gaps 30-50 % too
  fast, as a limit of discs. Varro's check showed most of that was the bench's steering: when
  men use the whole mouth, the discs come within 10-20 % of the measured flow, nearly constant
  across width. People pushing through a 0.7 m door pass about 40 % faster than discs can,
  because people turn their shoulders. That limit stands, reported and not fixed.
- **Varro's engine on the same bench** first passed too few walking men through a gate: 1.17
  per metre per second against about 1.9 measured, where mine passes 2.91. His walking curve
  was fitted to people walking in the open. He has now calibrated it on the bottleneck
  measurements, as my disc's radius was, and at its centre it passes 1.81. Steered across
  the whole mouth, both engines walk through a 3 m gate inside the measured range. His contact inside crowds changes the flow by under 6 % at any gate, so it buys
  nothing there, and it already fails on speed in the pocket.
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
| Walls on the field, the passage bench on it, three fixes from review, the press rule, the lean fix, the walking curve's capacity as a calibrated prior | 12b227c to 3516da4, reviewed and merged |
| The sampler's skeleton (`cannae_score::draw`) | 2aeec18, reviewed and merged |
| A scene of first contact and examples that trace the early breaks | 712022c, reviewed and merged. He found a fault in his rule for an enemy on the flank: it counted the men fighting a neighbour, against his own written design. The fix is agreed, and it adds a field to the shared view. The rest of the early breaks is his table doing what he wrote; no prior moves |
| His 81 lines of the sampler, the table of record (`--bin table`) and a nominal set; the flank fix (`NearEnemy::strikes_another`, mirroring the resolver's choice of opponent) | c49201d, 0c78ffb, reviewed and merged. After the fix a century still breaks about a minute in at the centres, as expected; no prior moved |
| `volleys` (missiles on an area, resolved beside the bouts) and the relief drill (a century relieved by the one behind it in a lull, in the shared frames) | 79957e5, e187489, ca38f3a, reviewed and merged |
| `backed`, and the relief from twenty minutes | b00dcf1, reviewed and merged |
| The rule for reading sustained figures, fixed before any is taken | b373249, merged (c9379c0) |
| His sustained speed readings | waiting for a quiet window on the Mac |

**The freeze (2026-11-10).** The pre-registration's text is final at commit dd9d90a. The record
is in its [sign-off](slice0/preregistration.md#sign-off): the SHA-256 of `prereg.toml` and of
the three priors files, the freeze seed (**9464880715404986219**, from the first 8 bytes of the
SHA-256 over the four), and the hash of the [table of record](slice0/table-of-record.csv), the
200 drawn parameter sets. Before signing, Varro had all 101 drawn lines checked against the
sampler by an independent reader: every range agrees, and five sentences were corrected (two of
them mine). Both have signed: Varro recomputed every number himself before signing. Michal accepted
the pass rules.

**The scored batch is running** on Michal's Raspberry Pi 400 (4 × Cortex-A72), so that the
Mac stays free: started 2026-11-10 at 17:49 UTC by `tools/s09-batch.sh`, at the frozen code.
Before the batch, the same 32 runs were made on the Pi and the Mac, and every result agreed
bit for bit.

**How long it will take.** Measured on its first cell: 12 runs a minute, about 3 hours for each
cell and variant of the field engine. The pairwise engine costs about twice as much. That makes
**about 11 days on the Pi alone** (it could be 9-15). Michal chose that over splitting the
batch with the Mac or renting a machine. `base` runs first on both engines, so the eight
patterns' verdicts for all eight cells come in **about 1.6 days**.

**Progress (2026-11-11, 06:40 UTC): the batch will be restarted.** Two things happened
overnight.

- **Michal decided on one restart** with Varro's fix for the standard-bearer defect, after his
  test showed the defect in two runs of three while the body was still formed. The decision was
  asked at 23:25 UTC and answered at about 06:38.
- **The pairwise half of the batch cannot run as built.** The field engine's `base` finished in
  all four of its cells (8,400 runs, 11.7 hours). Every pairwise step then failed within a
  second: a stack overflow in the batch runner. The runner hands battles to a pool of worker
  threads, and each pairwise battle hands its physics to a small pool of its own. A worker
  waiting on the inner pool picks up another battle meanwhile, and that one another, until the
  stack is full: about 2,100 battles deep for a whole cell. It changes no result. It never
  showed before because no test ran a whole pairwise cell; the runs made by every fourth set
  nest 550 deep and fit. The fix is in the runner only and goes into the same re-freeze.
- **A verdict line now exists, and I know it by inference.** With the pairwise steps failed,
  the script reached its gate early and judged the four field cells. I have not read the lines,
  but the order of the steps shows all four were put last: on the frozen code, with the defect
  in it, **fewer than 10 % of the parameter sets pass all eight patterns on the field engine at
  any rung**. I learned this after the restart was decided, and after Varro's fix was
  committed. It is in the pre-registration's record. The counts stay unread.
- **The re-freeze is therefore pinned:** the frozen code, Varro's fix exactly as committed
  before the gate ran, and the runner's fix, which must be shown to change no record. Nothing
  else.

**The crosswise check of the two perceptions passed** (finished 00:04 UTC): all 160 runs gave
the same end state and the same event log through either perception, and the 80 that the batch
had also made agree with its records, field for field.

**Rechecked on the Mac.** For every cell and variant, two parameter sets are run again on the
Mac and compared with the Pi's records, field for field (`tools/s09-recheck.py`). The freeze
seed fixes which sets, so nobody picks them. It runs as the batch goes. So far: the first
cell's 20 records are all equal. The tool was tested on a copy with one altered record, and it
caught it.

**Two addenda to the pre-registration** (agreed with Varro, written before any record was
scored, [addenda](slice0/preregistration.md#addendum-1-2026-11-10-cells-that-base-rules-out-run-their-variants-last)):

1. **A cell ruled out on `base` runs its variants last.** Such a cell has fewer than 10 % of
   its sets passing the eight bands on `base`, so it cannot pass whatever its variants do. Its
   variants run after those of the cells that can still pass. This changes the order only:
   everything still runs, and nothing is scored differently.
2. **A crosswise check of the two perceptions.** The two engines see through different code,
   held to the same view by tests but never compared on a scored battle. A sample of 160 runs
   is redone with the perceptions swapped, and every run must end in the same state with the
   same event log. If any run differs, the scoring stops and Michal is told. It has run, and
   every run agreed (below).

**A live viewer (asked for by Michal, 2026-11-10).** `cd viewer && cargo run --release`, then
open http://127.0.0.1:8377/. It runs one battle of the patch on either engine, at any memory
rung and scenario variant, and shows it as it goes: play, pause, step, speed, pan and zoom, and
a click on a man says who he is and what he is doing. It uses the mid-range parameter set,
which is never scored; the batch's drawn sets are not offered, because watching one before it
is scored would be reading its outcome. The viewer is a package of its own outside `crates/`,
so the frozen code is untouched and no simulation crate depends on it. `tools/replay.py` makes
a replay page from the harness's still frames. The viewer also says why: a log of every man
who gives ground, turns or goes down, with the model's own reasons, and a tally per side.

**What watching it showed (Michal, 2026-11-10).** At mid-range settings, on one seed, a century
of 81 broke three seconds after its centurion was killed, and the warband then stood formed
beside an open Roman flank. Both are the model doing what its rules say, and both look wrong.
Nothing is retuned under this pre-registration; the batch shows how often the 200 drawn sets
do this. Candidates for a second round, none decided (5 to 10 came from Michal's later questions):

1. **A fallen leader weighs too much** against a replacement that takes five seconds. Varro,
   who wrote the rule, thinks so too.
2. **Nothing limits how fast turning spreads.** Each man who turns alarms all who see him at
   the next quarter-second.
3. **Bodies never exploit a gap.** A body follows its one scripted order; the command layer is
   a later slice.
4. **A line cannot give ground and hold.** A man who steps back alarms his neighbours and is
   already nearer to running, and nothing makes the men beside him step back to keep the line.
   Cannae needs a centre that is pushed back a long way without dissolving.
5. **A man in flight cannot get past his own men.** He keeps the heading he chose when he
   turned and never steps round the man behind him, and a steady man never makes way. On my
   engine one runner stood pinned against the man behind him for over twenty seconds. On
   Varro's he would pass through, because bodies of one side are not solid there.
6. **No man is relieved by himself.** A whole century can be relieved in a lull, and files
   close over the fallen, but a tired or wounded man cannot be passed back. The sources give
   relief by lines and units (Livy 8.8; Caesar, Civil War 1.45 and 3.94) and room between ranks
   to step forward and back (Polybius 18.30.8; Vegetius 3.14); none describes a drill for
   swapping single men.
7. **A runner who cannot get out keeps frightening the men near him.** On my engine the first
   century's first runner stood pinned 1.3 m behind his place for 16 seconds, and all that
   time he counted as a turned comrade for everyone within about 5 m.
8. **Nothing separates giving ground from running.** The second century's first runner went
   from steady to giving ground to running in half a second, unwounded, with one man of his
   century down. The model's reasons: an enemy on his flank, three Gauls within 1.2 m, and that
   pinned runner in view.
9. **No man holds back from being the first to run,** and neither shame nor punishment is an
   input (Polybius 6.37-38). Support is a count of the men beside and behind him, his leader
   and the rear officer.
10. **The first to turn are front-rank men.** The sources and our own pre-registered pattern
   say a collapse starts at the rear. The batch scores how often it does.

**A defect found by watching (Michal, 2026-11-10).** When the man in front of a century's
standard-bearer falls, the whole century steps back one rank, 1.8 m, two seconds later. A
formation's place is fixed on its standard-bearer. When his file closes over the fallen man he
should step up; instead the formation's place moves back to where he stands, and every other
man follows it. Seen at about 30 s on my engine at rung 0, seed 1, and confirmed from the men's
positions.
- **It is in the frozen shared code,** so it is in every scored run, on both engines alike. The
  comparison between cells stands. It may touch the absolute bands a little: a body that gives
  1.8 m breaks contact for a second or two.
- **The batch goes on and nothing is changed.** It is written into the pre-registration's
  record of things seen during the batch. The report will count how often it happened in the
  scored runs, and whether those runs differ. The fix waits for a second round.
- **Michal decided: carry on,** over stopping, fixing, freezing again and restarting. After the
  batch, `base` is repeated in all eight cells with only this defect fixed, and the report
  gives both verdicts. The verdict of record stays the frozen one
  ([Addendum 3](slice0/preregistration.md#addendum-3-2026-11-10-a-defect-in-the-frames-and-base-repeated-with-it-fixed)).

**The replay page ran on the test physics,** where men walk through each other. The live viewer
runs the real engines and marks the leaders; use it and not the replay.

**Speed readings.** These need about two quiet hours on the Mac, through the other project's
lock. My spike now times the perception as Varro's does. Testing it showed that the core's
reference perception, which my engine uses, is very slow in the pocket. In a short test on a
loaded Mac, my engine ran the pocket ten times slower with it than with Varro's. His perception depends only on the core and is held to the same view, so either
engine can use it. Agreed with Varro: the perception is reported apart from the substrate, and
my kill line is read with his perception. A sustained figure is read by one rule for both, fixed
before any is taken ([spike-field.md](slice0/spike-field.md)).

**My line is read on all eight cores, corrected 2026-11-10.** I had set my readings to four
threads out of habit. The experiment's plan says "engine A on all cores as designed", and the
estimate behind the line counts every core. The correction is in my engine's favour, and I had
seen numbers on both sides of it; Varro read the texts and agreed
([spike-pairwise.md](slice0/spike-pairwise.md)).

**My 3x is read over a whole battle (Michal, 2026-11-10).** The criterion names no scene, and
the vision's estimate behind it reads "3-7x over a whole battle, falling toward 2x in the
encirclement". I had been reading it scene by scene, where a pocket under 3x kills the engine.
The whole-battle reading is in my engine's favour, so I put both to Michal with Varro's view.
Varro wrote the criterion's sentence and reads it as a whole battle. The figure of record: the
line scene and the pocket at equal shares of simulated time, with the pocket's own figure
printed beside it. Varro's line still names the pocket.

**The full-scale bench with the real man is designed** and agreed with Varro: 150,000 men,
real formations and fighting, either engine, each phase timed, in a package of its own outside
the frozen code. Two scenes, a line and a pocket. What counts as a real crush in the pocket is
written down before any run. Reading the code for it turned up a scan in the shared formation
code that looks at every man for every body under orders; Varro is replacing it with an indexed
one, bit for bit the same, on a branch that merges after the batch.

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

- **Speed line and the frames defect (2026-11-10):** the pairwise engine's 3x is read over a
  whole battle (the line scene and the pocket at equal shares), not scene by scene. The batch
  carries on with the frames defect in it; `base` is repeated afterwards with the defect fixed,
  and both verdicts are reported.
- **Slice 0 (2026-11-10):** only men out of their places press, on both engines. Whether a
  formed line shoves (othismos) is put off to Slice 1, not settled, and the report says so.
  Rung 4 is built only if rungs 0-3 all fail.
- **Terrain (2026-11-10):** no change to the vision now, but neither engine should be caught
  out when a slice asks for high ground, a river or a marsh. Where each effect would plug in,
  and the one rule that keeps it cheap (ground is handed to each part when it is built, as
  walls already are): [terrain-seams.md](slice0/terrain-seams.md).

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
