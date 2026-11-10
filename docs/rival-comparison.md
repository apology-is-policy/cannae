# Two Visions Compared

**Phase 0b report,** 2026-11-10. Written by Paullus for Michal's decision.

The rival design, *Small men, many battles*, is by Varro (Fable 5.2): seven files in
[`docs/rival/` on branch `varro`](https://github.com/apology-is-policy/cannae/tree/varro/docs/rival).
The argument behind this report is on yip call 0001, turns 1-6 and notes 1-5.

## In short

**Varro's thesis:** model only what a witness could have seen of each man. Spend what that saves
on running full-scale Cannae a hundred times a night on the Air.

**Where we ended up:** over six turns, most of our disagreement turned into one common design.
Three questions are left: two about the man and one about speed. The Slice 0 experiment can
settle all three in two weeks, on the Air, with no money spent.

**My recommendation: merge.**

1. Take the common design below as the working hypothesis.
2. Run Slice 0 as the experiment.
3. Revise `docs/vision/` to whatever wins.

## Where each design was stronger

**The rival was right about:**

- **Scale.** A 1:10 Cannae shrinks the cavalry's ride but not the centre's endurance. That
  distorts the very race A1 is about, so acceptance has to be measured at 1:1. Our plan needed
  a rented machine for that; his aims to run on the Air.
- **Statistics.** When runs are cheap, the whole set of seeded runs is the predictor for A2.
  Forking a live battle gives the odds at any moment, and the events that moved them. That
  replaces our learned predictor, our emulator and our scale study.
- **The frame.** Bodies of men formed on their standards (at the Sabis, *BG* 2.21), not on a
  frame fitted to wherever the men happen to be. If the standard is lost, the frame is lost.
- **The casualty clock.** Known losses over known hours leave room for very little committed
  fighting. It is now a test on both designs.
- **Explanations.** Every change of posture is an event, logged with its causes. So every rout
  has a family tree: who turned first, who followed whom, and where a centurion stopped it.
- **Engineering.** Integer state and one battle per core give bit-exact replay for free. They
  drop a whole layer of our threading machinery.
- **Our overbuilding.** From the first slice, our architecture paid for features we only plan
  later: three perception grids, wounds by body zone, and a second, aggregate model.

**Ours was right about the following, and Varro has taken each in:**

- **Memory.** Fear must be able to outlast its cause. A man struck from behind does not calm
  down the moment the attacker is driven off. The *has fled* and *shaken* marks come from this.
- **One representation.** Every body of men, legion or warband, is a carrier and the men tied
  to it. Drill sets how stiffly they are tied.
- **Bodies where it matters.** A pure density field smooths away jams at gaps, crushes, and the
  man trampled because of where he stood. In the common design, bodies touch wherever men are
  crowded and moving.
- **A clash is not the same as killing.** A clash can be hard work with little killing. So the
  casualty clock does not refute our stamina-timed clashes; it tests them.
- **Honest scoring.** In both designs, how long a battle lasts is close to an assumed input. So
  "holds, then breaks" is scored by whether the time to break moves the right way (veterans
  against recruits, a flank arrival, relief), not by its absolute value.

## The common design

- **Scale and statistics:** 1:1 always. A2 from the set of seeded runs. Fate curves by forking.
- **The man:**
  - five fixed dispositions: nerve, ardour, skill, physique, drill;
  - a posture: steady, forward, back, turned or down;
  - a wound class;
  - stamina (W').

  His chance of changing posture is read off what he can see, with a reaction delay. His memory
  is marks, as few as the patterns allow (see the ladder below).
- **Bodies of men:** a carrier (standard, chieftain or decurion) and men tied to their slots.
  Drill sets the ties.
- **Order decides the physics:**
  - a man in his place follows his slot;
  - a man out of his place moves in a density field;
  - a man who is out of place, crowded and moving also touches the men around him.
- **Contact:** bouts started by bold men and centurions, or a charge on a signal. The casualty
  clock bounds them.
- **Engine:** integer state, one battle per core, and no second aggregate model, provided the
  speed holds.
- **Kept from `docs/vision/`:** the armies and equipment, command and plans, the validation
  regime (outcome classes, tiers, contrasts, sealed battles), and the research on the contact
  zone.

## What is still open

1. **How much memory a man needs.** One ladder spans both designs:
   - no marks;
   - *has fled*;
   - *shaken*;
   - our continuous fear and resolve;
   - our full psyche.

   The lowest rung that reproduces the patterns is kept.
2. **What times a clash.** Varro says bold men lead short bouts. We say clashes end when
   stamina runs out. The pulse-and-lull pattern and the casualty clock decide.
3. **Whether the speed is real.** Varro estimates 12-30x real time per core with contact, and
   his kill line is 10x. We expect 3-7x on all cores, and our kill line is 3x. Neither figure
   has been measured.
4. **Whether a soft pressure field with local contact keeps a pocket at a believable density.**
   The pocket test measures it.

## Slice 0 as the experiment

Two weeks, on the Air, with no money spent.

1. **Harness and metrics** come first. Both engines share them: the scenarios, metrics computed
   from observable state only, the random streams, and stamina.
2. **The man:** a patch of 400 men, two centuries against a warband. Every rung of the ladder
   runs on both substrates. Eight patterns are scored, including "holds, then breaks" and the
   casualty clock. Pass bands and parameter priors are written down before any scored run.
3. **The substrate:** 150,000 bodies in three set-ups: a line battle, a pocket, and a rout
   through camp gates. Measured after ten minutes of sustained load.
4. **The passage:** a maniple falling back through an interval under pressure, and a crowd
   leaving through a gate. Each runs three ways: field alone, field with local contact, and
   contact everywhere. All are scored against measured crowd-flow data.

**Kill criteria, both ways.**

- **The rival dies if** it runs below 10x per core in the pocket, or if no rung up to *shaken*
  holds a line and then breaks it.
- **Ours dies if** it meets the casualty clock only with a near-harmless clash, or if it runs
  below 3x sustained at full scale.

**Who builds what:** each design is built by its believer.

- **Paullus:** the harness, the metrics, the substrate where every body touches every
  neighbour, and the top two rungs.
- **Varro:** the field substrate and the bottom three rungs.

**Before any scored run:**

- a line-by-line parameter count for each design, swapped between us;
- bounds on how lethal one bout is, from a literature search. The sources are likely to be
  weak, and we will say so.

## The options

- **Keep ours.** Take Varro's crossovers: the standard as frame, the clock, the 2D patch, the
  seed ensemble and *has fled*. Acceptance at 1:1 then needs a rented machine.
- **Adopt the rival.** Its central bet is unproven: that a line can hold for an hour and then
  break with only marks of memory.
- **Merge (recommended).** The common design is the hypothesis, and Slice 0 settles what is
  open. Then the vision is revised, followed by design review 2.

## What Michal decides

1. Which option.
2. The go-ahead for Slice 0 in this form, and who builds what.
3. **The psyche.** The ladder will show how much inner state the patterns need. Do you also
   want fear gauges in each man for the inspector, even if the patterns do not need them?
4. Whether `docs/rival/` lands on `main` next to `docs/vision/` as a proposal, so both are in
   one place.
5. When design review 2 runs. We both suggest after the experiment.

**Your earlier question about line-passage jams is settled.** Bodies now touch where men are
crowded and moving, so whether a passage jams is a result again, not an input.
