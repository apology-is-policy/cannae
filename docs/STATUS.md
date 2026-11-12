# Status

**Phase:** Slice 0, the experiment between the two designs. Both engines are built and
frozen. The scored batch is being restarted once, on a code with two fixes; the newest
blocks are under "Since then" below.
**Working mode:** guided (see [CLAUDE.md](../CLAUDE.md)).
**Last updated:** 2026-11-12, 14:40 UTC.

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
| His sustained speed readings | done 2026-11-11; his write-up follows |
| His review of the runner's fix (449dcd6) and of the QC list; five detectors of his own | 3151e50, merged |

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

**Since then (2026-11-11, 07:30 UTC):**

- **The runner is fixed** on branch `runner-threads` (449dcd6): battles run on plain threads.
  It changes no record: 90 field records made with it on the Mac equal the batch's own from
  the Pi, and 80 pairwise records equal the frozen runner's. Varro reviews it. A pre-flight
  script will start every one of the batch's 64 steps once before the re-freeze, so that the
  batch runs nothing for the first time.
- **A visual QC pass comes before the re-freeze** (Michal's request): twenty detectors for
  things that look wrong, over 320 battles at the mid-range settings, with the thresholds fixed
  before any flag is looked at ([qc-pass.md](slice0/qc-pass.md)). It costs about a day, and no
  restart comes after it.
- **The Mac was restarted at 06:53 UTC** for the speed readings, which still wait for its
  background work to settle.
- **Nothing can be pushed at the moment:** the GitHub login on the Mac has been invalid since
  the restart, and Michal has to renew it. Commits wait locally.

**Since then (2026-11-11, 07:50 UTC):**

- **Varro reviewed the runner's fix and the QC list, and found nothing against either.** He
  added five detectors, three of them aimed at his own code, and two conditions: the pass runs
  on the fixed code, and every flag looked at is listed with its verdict, the harmless ones
  too. I added one more for the third thing Michal saw, a man who ran half a second after he
  was steady: it counts how often that happens.
- **The QC tool is written and not yet run:** 26 detectors and the two dormancy checks, a
  renderer that draws the men around each flag at four moments, and a link from each picture
  into the live viewer at that battle and second. Its own tests are on hand-made scenes. It
  runs on no battle before those tests pass.
- **The fixed code exists as a branch** (`refreeze-candidate`): the frozen code with Varro's
  fix and the runner's fix, checked file by file to hold nothing else.
- **[Addendum 4](slice0/preregistration.md#addendum-4-2026-11-11-the-batch-is-restarted-once-on-a-re-frozen-code)
  is drafted, unsigned:** the restart, what goes into the fixed code, and the record of who
  knew what and when. Michal told Varro they had seen none of the Pi's output before deciding.
- **Varro's speed readings are running** (since 07:33 UTC, about an hour). Mine follow at
  once, about another hour. Nothing is compiled on the Mac meanwhile.
- **The Pi cannot be reached at the moment:** since its restart the Mac is on a phone hotspot,
  not on the Pi's network. The old batch runs on there. The pre-flight and the new batch wait
  until the Mac is back on that network.

**Since then (2026-11-11, 08:40 UTC):**

- **The Pi is reachable again and GitHub works;** everything is pushed.
- **The old batch was stopped at 08:25 UTC.** Its folder is untouched and stays as the first
  batch: 9,401 records, 8,400 of them the field engine's `base`.
- **The QC tool is tested and committed.** Its 38 tests on hand-made scenes pass, and six more
  hold the battles it builds to the scorer's own, event for event. It was compiled and tested
  on the Pi, so the Mac stayed free.
- **The control passed:** on the frozen code the tool finds the step back Michal saw, at
  29.25 s: 70 of 77 slots moved back by exactly one rank.
- **The pass itself is running on the Pi,** on the fixed code: 320 battles.
- **Varro's speed readings are done, and he reports that his engine fails its line.** The
  pocket with perception ran at 1.43x sustained on one core, against 10x. His table goes into
  [spike-field.md](slice0/spike-field.md). By the rule both signed, a reading under the line
  is final, since the man only adds cost. My own readings start when he hands the Mac over.

**Since then (2026-11-11, 12:40 UTC):**

- **My engine is under its own speed line too: 2.67x against 3x over a whole battle.** The
  pocket ran at 1.59x sustained on eight threads and the line at 8.42x, both valid by the rule,
  and the man is not in it yet ([spike-pairwise.md](slice0/spike-pairwise.md)). So both
  engines miss their speed lines on the Air. What follows from that is Michal's to decide
  with the Slice 0 report.
- **The Mac went down at about 09:13 UTC,** during my third run, and was back at 12:14. The two runs that set the
  figure were complete; three side runs are missing. Something unknown had held the Mac's load
  near 1.8 during my runs; it is logged, and the rule's own test for a disturbed run did not
  trigger. A second reading on the restarted Mac would stand beside the first, not replace it.
  Varro and I put that to Michal as one joint question; none is taken before they answer.
- **One shared component is the larger part of both speed failures.** Both spikes run Varro's
  perception on one thread. It took 67 % of my pocket's last five minutes and 85 % of his.
  Without it my runs would read about 7.4x over a whole battle. The figures stand as read;
  what to do about the component is for Michal to see before deciding about either engine.
- **The QC pass ran:** 320 battles on the fixed code, 151,258 flags, 170 picked by the rule
  and drawn. Six detectors found nothing on either engine, and both dormancy checks pass
  everywhere.
- **The pairwise half is looked at** ([qc-results.md](slice0/qc-results.md)): 75 flags, **no
  defect,** 37 fine, 38 gaps. The fix for the step back works. Thirteen things look wrong
  although the code does what it says. The ones Michal has not seen yet: a century still
  steps back as one, by 0.65 m, when its standard-bearer himself gives ground; the
  standard-bearer moves up into the front rank and fights; the fronts pack into a dense band
  after two minutes; runners keep their file lines; pursuit never ends; men attacked from
  behind rarely face about.
- **Still to come:** Varro's list for the field engine, each reading the other's, the page
  with pictures for Michal, then Addendum 4, the re-freeze and the new batch. **Nothing runs
  on the Pi meanwhile.**

**Since then (2026-11-11, 15:45 UTC):**

- **The page with pictures is there: [slice0/qc/README.md](slice0/qc/README.md).** One
  picture and a few lines for each of the thirteen things that look wrong on the pairwise
  engine, made to be read on a phone. The one to look at first is A: a century still steps
  back as one, by 0.65 m, when its standard-bearer gives ground.
- **Every pairwise flag looked at, with its four-panel picture, bin and reason:**
  [slice0/qc/pairwise.md](slice0/qc/pairwise.md). 75 flags, 63 distinct pictures.
- **The page's links open the live viewer on the Mac.** It runs there now, built from the
  fixed code, on 127.0.0.1 only. It is stopped before anyone reads a speed again.
- **The draft of Addendum 4 has the four changes Varro asked for** (call 0001, note 59): which
  batch pairs with which, the runner fix named as chosen after the verdict was known, both
  speed verdicts known before the re-freeze, and a place for his own check of the runner fix.
  It is still unsigned.
- **Varro's list for the field engine is in**
  ([qc-results-field.md](slice0/qc-results-field.md)): 95 flags, 41 fine, 50 gaps, and
  **one candidate defect, in his own field code** (4 flags; 123 of its kind in 27 of 160
  battles, none in `base`). Two enemies on their feet can come nearer than the guard's
  distance when both count as in their places; the nearest pair stood 11 mm apart. I read
  the text and the code and agree that it is a defect. Both of us recommend that it is
  recorded and left out of the re-freeze.
- **Each of us has read the other's list.** Varro agrees with all 75 pairwise bins. The step
  back that remains (A on the page) is larger on his engine: a century of 72 walks back
  15.6 m in twelve seconds behind a standard-bearer who gives ground.
- **Four more things that look wrong, from his list:** pursuers of one side stand on one
  another (the field engine only); a man at the edge of his alarm flickers between steady
  and giving ground; a body ordered to retire walks off for good (both engines); the second
  line runs within seconds of the first, untouched.

**Michal's three decisions (2026-11-11, about 15:40 UTC),** each on the recommendation of both
of us:

1. **The field engine's defect is recorded and left in.** The re-frozen code is the frozen
   code with the two fixes and nothing more. The report names the defect with its counts.
2. **The step back that remains stays in the new batch** and goes first on the list for the
   second round (11 below). The code does it on purpose: a formation follows its standard.
3. **Speed: the shared perception is put on all threads and the speed is read again as a new
   line,** conditions written first. The first readings stay in the report as the verdict on
   the engines as built. It does not hold up the batch.

**Since then (2026-11-11, 16:15 UTC):**

- **The fixed code passes its tests:** 217 of 217 on the Pi, and the QC tool's own 44. A
  marching and a charging body were traced step by step on both engines: every slot moves
  alike, and none by a rank.
- **The picture page has its field half:** a section from Varro's list on
  [the page](slice0/qc/README.md), and every field flag with its picture in
  [slice0/qc/field.md](slice0/qc/field.md).
- **Varro's own check of the runner fix is nearly done:** 76 runs were left at 16:05 UTC,
  and every record compared so far is equal but for the wall time. His line and signature
  on Addendum 4 follow it.
- **The speed work Michal chose is prepared, and nothing is read.** Its conditions are
  written ([spike-pairwise.md](slice0/spike-pairwise.md)). The change is small and in the
  speed test alone.

**Since then (2026-11-11, 16:35 UTC): the code is frozen again.**

- **Addendum 4 is signed by both of us and approved by Michal** (at about 16:31 UTC;
  [the text](slice0/preregistration.md#addendum-4-2026-11-11-the-batch-is-restarted-once-on-a-re-frozen-code)).
- **The re-frozen code is commit 40b8dcd:** the frozen code with the two fixes and nothing
  more, five files under `crates`. The files the freeze hashed are unchanged, so the freeze
  seed and the 200 parameter sets are the same.
- **Varro's own check of the runner fix found no record that differs:** 640 records made by
  the fixed runner against the frozen runner's, on both engines, every cell and every variant.
- **The batch's pre-flight runs on the Pi since 16:12 UTC** and ends at about 18:25. It
  starts every one of the 64 steps for two minutes and throws the runs away. At 16:33, 9
  steps had run and none had died.
- **One thing is still to build before the batch starts: the gate's scorer.** The batch asks
  a scorer which cells `base` has ruled out, so that their variants run last (Addendum 1).
  That scorer prints one line more than the frozen one and lives on a branch. Addendum 4
  says it is built from the re-frozen code, so it is made again while the pre-flight runs.
- **Still to come:** the new batch on the Pi, into a new folder, when the pre-flight has
  ended with no failed step.

**Since then (2026-11-11, 18:40 UTC): the new batch runs.**

- **The new batch started on the Pi at 18:39 UTC,** on the re-frozen code, into a folder of
  its own. `base` runs first in all eight cells. How long it takes I say when the first cell
  is done.
- **Its pre-flight passed:** all 64 steps started, 460 runs, none failed.
- **The gate's scorer was made again on the re-frozen code** and checked: its records and
  its output equal the re-frozen build's, but for the one gate line and two added hashes.
- **The two engines' men still see the same thing on the re-frozen code:** all 160 runs of
  the crosswise check end in the same state with the same event log through either
  perception.
- **Varro recomputed the record of the re-freeze himself,** and it holds.
- **The second speed reading is being taken on the Mac** (since 18:33 UTC). Varro holds the
  other project's lock so that the Mac is quiet. The live viewer is stopped meanwhile.

**Since then (2026-11-11, 19:00 UTC): the speed reading is put off, and two definitions are
written.**

- **The second speed reading was not taken.** The Mac was busy with Apple's own work after
  the day's system update, mostly the analysis of the photo library, and its load never came
  under the 2 the rule asks for. I stopped the script at 18:57 UTC before any timed run had
  started, so nothing was read. It is started again when the Mac is quiet
  ([spike-pairwise.md](slice0/spike-pairwise.md)). Its check that the changed speed test
  computes the same thing passed on the Mac: all ten short runs gave the recorded figures.
- **The batch:** 216 runs of its first step were done at 18:59 UTC, twenty minutes in, and no
  step has failed. At that rate the first of its 64 steps takes about three hours. The steps
  differ in length, so I give the whole batch's time when the first cell is done.
- **How the discs' too-easy passage is shown in the relief is now defined,** before any
  record of the relief exists ([report-outline.md](slice0/report-outline.md), section 5).
  The relief as scripted has no gap to squeeze through: files pass through files, at a fifth
  of the density at which the bench's crowds jammed. So the report counts how often a relief
  was jammed or pressed by the enemy, and says that it cannot give what a correct passage
  would have changed.
- **The two things Michal saw at the mid-range set now have their counts** (section 6): how
  often a body breaks within seconds of its leader's fall, against the fall of any file
  leader; and how often one Roman century stands on after the other has broken.
- **Varro corrected two facts about his engine in these texts and added three things.** All
  are in.
- **A small replay tool for these counts is written** (branch `replay-tool`). It lives
  outside the scored code and is pointed at scored runs only after the scoring.

**Since then (2026-11-11, 19:20 UTC):**

- **Both definitions and both counts are agreed** and stand in the outline over both our
  names, with four more small changes from Varro.
- **The replay tool builds and passed its check on sets that are not scored** (drawn with
  another seed). Run again by the tool, every one of 16 battles was the battle of its
  record, on both engines. Varro reads it before it goes onto main.
- **The relief alone, with no fighting, is a free walk on both engines:** the fresh century
  is up in 18.75 s and the tired one back 18.75 s later, and no two men of the two centuries
  come nearer than 0.9 m. The two men out of their places at the end are the two closers,
  who stand aside while the centuries pass.
- **Outcomes of four unscored relieved battles were read in that check** and are entered in
  the pre-registration's table of outcomes seen.
- **Varro checks the field engine's cell size now, on the Mac:** the mid-range set at the
  scored cell and at half of it, 360 runs, with rules for reading it that were written and
  read before any run ([cell-check.md](slice0/cell-check.md)). It changes nothing in the
  scored code.
- **The speed reading waits for the Mac.** The other project holds it for two to three
  hours, so the reading comes at about 21:30 UTC or later.

**Since then (2026-11-11, 19:25 UTC): the replay tool is on main.**

- **Varro read it whole and found one fault,** which is fixed: two of its counts took in men
  who had fled long before the relief, so a thinned century would have looked out of place
  with no jam anywhere. They are now asked only of the men who were in their places when the
  relief was ordered. The outline's sentence is corrected to match.
- **With that he agreed to its use,** and it is merged
  (`viewer/src/bin/cannae-replay.rs`). It is pointed at scored runs only after the scoring.
- **In the drill alone, no man who was in his place at the order is out of it at the end.**
  The two men I named above, the closers, have no slot of their own and are outside the
  count.

**Since then (2026-11-11, 19:31 UTC): the field engine's cell check is run.**

Varro's check, on the mid-range set, which is never scored
([cell-check.md](slice0/cell-check.md)). The scored code is untouched by it.

- **By the rule written before the run: no.** With the cell at half its size, the eight
  measures moved and the verdicts changed no more than they do between two groups of seeds
  at the same cell.
- **That is weak evidence, and Varro says so.** At the mid-range set the battle is over in
  about half a minute on the field engine, at every rung, and such a short fight gives a
  crowd little to do.
- **Eight of the field engine's own 44 tests fail at the half cell.** The one that counts: a
  man walking through a crowd covers a sixth of the distance the calibrated curve gives. So
  the cell is not a neutral choice of structure, as the priors called it. The crowd rules
  were calibrated at this cell and hold at this cell.
- **Varro recommends that the cell go on the list for a second round.** It is there as
  candidate 25, undecided like the rest.
- **Both of us have now read this outcome of the field engine at the mid-range set:** it
  fails six of the eight patterns there and the Roman line breaks first in nearly every
  run. It is entered in the table of outcomes seen. The 200 drawn sets are what is scored.

**Since then (2026-11-11, 21:14 UTC): the speed reading is put off until the Mac can be
quiet.**

- **The lock came to Varro at 21:12 UTC and he gave it back within a minute.** The photo
  analysis was running again, and so was a build in a third project's session. A lock on the
  other project's line does not still either.
- **We stop trying by turns.** A run started in a gap between two builds and then slowed
  would count by the rule, and that would be a wrong figure. The reading waits for a window
  Michal gives ("Open decisions" below).
- **The three counts the report owes now have their tools,** each read and agreed by Varro
  and each run only after the scoring: the reliefs and the leaders' falls by replay
  (`cannae-replay`), and the open flank from the records (`tools/s09-open-flank.py`).
- **The batch:** its first step is still running and no step has failed.

**Since then (2026-11-11, 21:38 UTC): the batch's first step is done, and four more counts
are fixed.**

- **The first of the batch's 64 steps finished at 21:32 UTC** with no failure: 2,100 runs in
  2 hours 53 minutes. That is the pace of the first batch, so the estimate given then
  stands:
  - the eight patterns' verdicts for all eight cells in about 1.6 days, on about the
    morning of 13 November (UTC);
  - the whole batch in about 11 days (it could be 9 to 15), on about 22 November.
  - The estimate takes the pairwise engine at twice the field's cost. Its first step
    starts at about 06:15 UTC on 12 November and shows whether that holds.
- **That step's records are rechecked.** The Mac made two of its sets again and all 20
  records are equal to the Pi's. The crosswise check holds too: all 160 runs agree through
  either perception, and the 21 of them the batch has made so far agree with its records.
- **Four more of the report's figures now have a fixed way of being counted,** written
  before any record was read and agreed with Varro
  ([report-outline.md](slice0/report-outline.md), sections 1 to 4):
  - on which part of its band a set fails a pattern;
  - the field engine's first batch beside the new one, run by run and set by set;
  - how stragglers and tempo are given;
  - when one rung of memory "shows more" than another: by a count over the sets, against
    the line a fair coin passes once in forty times.
- **Varro changed one reading rule in them, and it matters.** I had the two batches of the
  field engine compared by whether any set's verdict changed. In the unscored test the
  standard-bearer fix acted in 141 of 144 runs. So the two runs of nearly every pair are
  different battles from that point on, and verdicts flip both ways by chance. The report
  now reads whether they flip one way more than the other.
- **One more check comes before the scoring:** every run the fix did not act in must equal
  its run in the first batch. If one does not, we stop and tell Michal before anything is
  scored.
- **The tool for these counts is written and on main** (`cannae-extras`, outside the scored
  code). Checked on sets that are not scored, in all eight cells: it rebuilds every verdict
  from its parts and gets the scorer's each time, and its counts between rungs equal counts
  made apart from the records, 30 of 30. Varro reads it before it is pointed at a scored
  run.
- **My watcher on the batch had a fault** and would never have reported the step's end. I
  found it at 21:17 UTC and replaced it; nothing was missed.

**Since then (2026-11-11, 21:51 UTC): the first check of the standard-bearer fix on scored
runs passed.**

- **Varro read the new tool whole and found no fault.** He added a test that tries every
  edge of every band, and a guard against naming the wrong folder as the first batch. Both
  are merged.
- **Where the fix did not act, it changed nothing.** On the field engine's rung 0, each of
  the new batch's 2,100 runs of `base` was put beside the first batch's run of the same set
  and seed. In 711 of them the fix never acted, and all 711 are equal to the first batch's,
  field for field. None is unequal.
- **In the other 1,389 it acted,** about two runs in three. That is fewer than in the
  unscored test (141 of 144), and it is no outcome of a pattern.
- **This check is taken for each field cell as its `base` finishes,** and before anything
  is scored. It is entered in the pre-registration's table.

**Since then (2026-11-12, 00:28 UTC): the second step is done and checked.**

- **The batch's second step finished at 00:25 UTC** with no failure: the field engine's
  rung 1, `base`, 2,100 runs in 2 hours 53 minutes, as the first. It is in its third step.
- **Rechecked on the Mac:** two sets of the step made again, 20 records, all equal to the
  Pi's. With the first step that is 40 of 40.
- **The crosswise check:** 160 of 160 through either perception; the 41 of those runs the
  batch has made so far agree with its records.
- **The standard-bearer fix, where it did not act, changed nothing here either.** Of the
  2,100 runs, 664 never met it, and all 664 equal the first batch's. In the other 1,436 it
  acted.

| Field engine, `base` | Runs the fix never acted in | Equal to the first batch's | Unequal | Runs it acted in |
|---|---|---|---|---|
| Rung 0 | 711 | 711 | 0 | 1,389 |
| Rung 1 | 664 | 664 | 0 | 1,436 |

**Since then (2026-11-12, 06:47 UTC): the field engine's `base` is done in all four cells,
and every check on it passed.**

- **Four of the batch's 64 steps are done, none failed.** The field's rungs 2 and 3 finished
  at 03:04 and 05:54 UTC, in 2 hours 39 minutes and 2 hours 50 minutes.
- **Rechecked on the Mac:** two sets of each step made again, 80 records in all, every one
  equal to the Pi's.
- **The crosswise check:** 160 of 160 through either perception; the 89 of those runs the
  batch has made so far agree with its records.
- **The standard-bearer fix changed nothing where it did not act, in any of the four
  cells.** This was the check agreed for before the scoring, and it is complete:

| Field engine, `base` | Runs the fix never acted in | Equal to the first batch's | Unequal | Runs it acted in |
|---|---|---|---|---|
| Rung 0 | 711 | 711 | 0 | 1,389 |
| Rung 1 | 664 | 664 | 0 | 1,436 |
| Rung 2 | 816 | 816 | 0 | 1,284 |
| Rung 3 | 702 | 702 | 0 | 1,398 |
| All four | 2,893 | 2,893 | 0 | 5,507 |

- **The pairwise engine's first step started at 05:54 UTC** and had 843 of its 2,100 runs
  after 51 minutes. So far that is faster for each run than the field, where the estimate
  of 11 days took it at twice the field's cost. I give a new estimate when the step is
  done.
- **I looked late.** My watcher reported the third step's end at 03:08 UTC and this session
  woke only at 06:45 UTC, so the checks of rungs 2 and 3 came three and a half hours after
  they could have. Nothing was missed by it.

**Since then (2026-11-12, 08:07 UTC): the pairwise engine's first step is done, and the
batch will be shorter than I said.**

- **Five of the 64 steps are done, none failed.** The pairwise engine's rung 0, `base`,
  finished at 08:04 UTC: 2,100 runs in 2 hours 10 minutes. The field's four steps took
  2 hours 49 minutes on average.
- **This is the first step the pairwise engine has ever finished in a batch,** and it is
  checked like the others: two of its sets made again on the Mac, 20 records, all equal to
  the Pi's (100 of 100 over the five steps). The crosswise check stands at 160 of 160, and
  the 101 of those runs the batch has made agree with its records.
- **A new rough estimate: five and a half to seven days from the start,** so the batch is
  done on about 17 to 18 November. The estimate of 11 days took the pairwise engine at
  twice the field's cost, and on its first step it cost less than the field.
  - **What the estimate assumes:** every step of a cell runs at the pace of its `base`,
    and the pairwise engine's rungs 1 to 3 cost between what rung 0 costs and twice that.
    The costing before the batch had rung 3 at twice rung 0. The next three steps show it.
  - **`base` for all eight cells** is then done between about 14:30 and 21:00 UTC today.

**Since then (2026-11-12, 09:01 UTC): the live viewer runs again,** at Michal's asking, so
that they can look again. It had been stopped since the evening's attempt at the speed
reading.

- **It is built from the re-frozen code,** the code the batch runs. So it has the one fix
  to the model that code has: the century no longer steps back a whole rank when its
  standard changes hands (the moment Michal saw at 29.25 s).
- **Everything else Michal saw is still there, by their decisions.** The smaller step back
  of 0.65 m when the standard-bearer himself gives ground stays (candidate 11); so do the
  century that breaks seconds after its centurion falls and the warband beside an open
  flank (candidates 1 and 3). Nothing is retuned under this pre-registration.
- **It listens on 127.0.0.1 only, port 8377,** and shows the mid-range set only. The links
  on [the QC page](slice0/qc/README.md) open it at the moments named there.
- **It is stopped before a speed is read.**

**Since then (2026-11-12, 10:16 UTC): six of the 64 steps are done, none failed.**

- **The pairwise engine's rung 1, `base`, finished at 10:14 UTC** in 2 hours 10 minutes, the
  same as its rung 0. So rung 1 costs no more than rung 0, and the batch's time looks
  nearer the five and a half days than the seven.
- **Rechecked on the Mac:** two of its sets made again, 20 records, all equal to the Pi's
  (120 of 120 over the six steps). The crosswise check: 160 of 160, and the 121 of those
  runs the batch has made agree with its records.

**Since then (2026-11-12, 12:26 UTC): seven of the 64 steps are done, none failed.**

- **The pairwise engine's rung 2, `base`, finished at 12:22 UTC** in 2 hours 8 minutes. The
  last `base` step, its rung 3, is running and should end at about 14:30 UTC.
- **Rechecked on the Mac:** two of its sets made again, 20 records, all equal to the Pi's
  (140 of 140 over the seven steps). The crosswise check: 160 of 160, and the 141 of those
  runs the batch has made agree with its records.

**Since then (2026-11-12, 14:40 UTC): `base` is done in all eight cells, and the batch is on
to the other variants.**

- **Eight of the 64 steps are done, none failed.** The pairwise engine's rung 3 finished at
  14:35 UTC in 2 hours 13 minutes. Its four `base` steps took 2 hours 8 to 13 minutes each,
  so its higher rungs cost no more than rung 0, and the batch should be done on about
  17 November.
- **Rechecked on the Mac:** two sets of every `base` step made again, 160 records in all,
  every one equal to the Pi's.
- **The crosswise check is complete:** 160 of 160 runs agree through either perception, and
  all 160 agree with the batch's own records.
- **Something leaks from here on, and I have closed what I can.** After `base` the batch
  runs the variants of the cells that can still pass first. So the names of its steps tell
  what the gate decided, and neither of us had counted that as a way of seeing an outcome.
  - My watcher printed the name of the first step after `base`. What that tells, and what
    it does not, is in the pre-registration's table of outcomes seen (row of 12 November,
    14:37 UTC).
  - Since 14:39 UTC my watcher and my checks print counts and times only: steps done and
    failed, and records equal and unequal. From here this page gives the batch's progress
    as a number of steps, with no names.
  - I asked Varro to look at the Pi the same way, and offered him the stop before the row.

**What the report owes is listed:** [slice0/report-outline.md](slice0/report-outline.md), a
draft with no result in it, written before the new batch has made a record. It collects
everything the pre-registration, its addenda and this page promised the report would give.
One decision in it will be Michal's before any relief statistic is read: whether the relief
gets its extra runs (the "top-up"), on the machine time alone.

**The crosswise check of the two perceptions passed** on the frozen code (finished 00:04 UTC):
all 160 runs gave the same end state and the same event log through either perception, and
the 80 that the first batch had also made agree with its records, field for field. It was
run again on the re-frozen code and passed again (the blocks of 18:40 and 21:38 UTC above).

**Rechecked on the Mac.** For every cell and variant, two parameter sets are run again on the
Mac and compared with the Pi's records, field for field (`tools/s09-recheck.py`). The freeze
seed fixes which sets, so nobody picks them. It runs as the batch goes. So far, on the new
batch: the 160 records of its eight `base` steps are all equal (2026-11-12, 14:40 UTC). The tool was
tested on a copy with one altered record, and it caught it.

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

**11 to 24 came from the QC pass (2026-11-11).** The letters are those of
[qc-results.md](slice0/qc-results.md) and [qc-results-field.md](slice0/qc-results-field.md);
the pictures are on [the page](slice0/qc/README.md). C is 5 above, D is 8, and L is 3.

11. **A: a formation follows its standard-bearer wherever his own posture takes him.** A
    century steps back as one when he gives ground: 0.65 m at a time on my engine, 15.6 m in
    twelve seconds on Varro's. **Michal: it stays in the batch and is first here.**
12. **B: the standard-bearer moves up into the front rank and fights.** A starts there.
13. **E: a runner does not read where the enemy is,** and a pursuer ignores a runner behind
    him.
14. **F: runners keep their file lines,** nose to tail.
15. **G: pursuit never ends.** Nobody is caught and nobody gives up.
16. **H: the two fronts pack into one dense band,** and the Romans' fighting room is gone.
17. **I: a strike passes through a comrade.**
18. **J: men attacked from behind rarely face about.**
19. **K: a lost file is never closed from the side.**
20. **M: a body halts at its standoff before a single man.**
21. **N: pursuers of one side stand on one another** (the field engine only).
22. **O: a man at the edge of his alarm flickers** between steady and giving ground, six times
    in five seconds. When he carries the standard, the century steps with him each time.
23. **P: a body ordered to retire walks off for good,** and its men never regain their slots
    (both engines).
24. **R: the second line runs within seconds of the first, untouched.** A man's alarm counts
    his side's runners, not his body's.
25. **The field engine's cell is a parameter of its crowd rules** (Varro, 2026-11-11, from his
    cell check; recommended with the result in hand, and not decided). At half the cell,
    eight of the field's own 44 tests fail, and a walker in a crowd covers a sixth of the
    calibrated distance. Either the density is spread over a fixed width in metres whatever
    the cell, or the cell is named as a calibrated constant with the walking curve.

**A defect left in by decision: D1, in the field engine.** Two enemies on their feet can stand
nearer than the guard's distance (Michal's first decision above).

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

**Waiting on Michal, and holding up one thing (2026-11-11, 21:14 UTC): a quiet Mac for the
second speed reading.** The reading needs about forty minutes in which the Mac runs nothing
heavy. Twice today it had none. Three things have to rest at once, and only Michal can
arrange any of them:

- the photo library's analysis (`mediaanalysisd`), which came back at 21:12 UTC;
- a build in another project's session on the Mac (not ours, and not the Thylacine line's);
- the Thylacine sessions, which the lock stills.

Until then the reading is put off. Nothing was read, and the batch and the report's other
work do not wait on it.

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
