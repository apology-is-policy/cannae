# 04 — Acceptance

The rival keeps both acceptance tests and the sealed battles. It changes where they are
measured and how much machinery stands between a run and a verdict. It also has to be honest
about a cost: a smaller model writes more of its behaviour in directly, so fewer patterns count
as evidence (§4).

## 1. A1 at full scale

A1's definition is unchanged: the outcome classes, the sequence markers, the authoring tiers,
the Dertosa and Trebia contrasts, the stopping rule and the casualty mapping are those of
[07](../vision/07-validation.md) §6. What changes is that A1 is measured at 1:1, on the Air,
over hundreds of seeds.

**Why scale matters: the race.** Class (a) needs the trap to close before the Roman infantry
gets out of it. Two clocks run against each other:

- **The cavalry's clock.** Hasdrubal must break the Roman horse, ride behind the Roman army to
  the other wing, break the allied horse, and return on the infantry's rear. Those distances
  grow with the length of the battle line.
- **The centre's clock.** How long the Gauls and Iberians give ground before they are driven
  in, and how long a formed Roman body needs to force its way out. That depends on depth,
  breath and the stock of bold men. It does not depend on the length of the line.

A battle at 1:10 must shrink something:

| How Little Cannae shrinks | What it does to the race |
|---|---|
| Narrow the front tenfold, keep the depth | The ride shrinks tenfold and the centre's endurance does not. A ride of minutes becomes well under a minute. The trap closes far too easily. |
| Shrink front and depth by about three each | The ride shrinks threefold and the centre thins threefold. In the pocket, the share of men on the perimeter triples, so many more of the encircled can fight and the crush is weaker. |

Neither preserves the race, and no rescaling of time constants repairs both clocks at once.
The vision of record's scale study (1:20, 1:10, 1:5) would probably detect the drift, but it
cannot remove it, and the cure is the rented machine. Paullus called this the sharpest point
in the sketch and conceded it.

**Classifier terms get simpler.** A "formed body" is a body whose share of men in place is
above a tolerance. "Loses its order" is that share falling ([02](02-order-and-contact.md) §2).

**Dose-response at full scale.** Five levels of the centre's quality at a hundred seeds each
is two or three nights on the Air.

## 2. A2 from the ensemble

Michal's wish: looking at the armies beforehand, it should not be obvious how the battle ends.
The vision of record measures this with a learned predictor trained on thousands of small
battles. That predictor estimates one thing: the probability of each outcome given what can be
seen before the battle. With cheap runs we do not need to estimate it. For a scenario s, run
many seeds and count:

> p(s) = the share of seeds that side A wins

The ensemble is the ideal observer. It knows everything observable beforehand and nothing that
is hidden; the hidden traits drawn at setup are one of the RNG streams it averages over.
Paullus agreed this is simpler and stronger.

**The tests:**

1. **Indeterminate where it should be.** On scenarios a generator makes balanced, p(s) spreads
   through the middle of the range. The band is fixed before the runs.
2. **Not noise.** On lopsided scenarios p(s) sits near 0 or 1. The variance decomposition by
   RNG stream is kept as designed: how much of the outcome was fixed by hidden traits, how
   much by combat dice, how much by command friction.
3. **Decided by events.** A battle's fate curve ([03](03-engine.md) §6) should move in steps.
   The measure: the share of the curve's total movement that falls in short windows around
   logged events, which are the roots of the large rout genealogies, commanders' falls, and
   reserves committed.
4. **Blind trials,** as designed. Michal predicts with probabilities and we track his score.

The ensemble covers what an ideal observer could know. Michal is not an ideal observer, and
his wish is about what *he* can foresee. That is why test 4 stays.

## 3. Sealed battles and calibration

**Sealed battles are unchanged:** dossiers from a fixed rubric, hash-committed, run once,
scored by log-likelihood against baselines. They remain the only strong evidence either design
can offer. The rival runs them at full scale on the Air.

**A suggestion for both designs.** Sabin's *Lost Battles* (2007) applies one rule set to some
three dozen ancient battles, with troop classes and quality grades. It would be a much stronger
baseline than a Lanchester model. If its grades can serve as the rating rubric, the dossiers
lose a degree of freedom, because someone who never saw our engine wrote them. This is from
memory and must be checked before anyone relies on it.

**Calibration.**

- **Fewer parameters.** My count is 25–35, against the 50–100 the vision of record estimates
  for psyche and combat. Counting both exactly is part of the experiment.
- **Fixed from independent evidence:** W′ and its recovery; walking speed against density;
  the bounds on lethality per bout.
- **The casualty clock** ([02](02-order-and-contact.md) §9) is a hard constraint, applied
  before anything else.
- **History matching without an emulator.** The method stays: rule out the regions of
  parameter space whose outputs are implausibly far from the patterns, and report the share of
  the surviving region in which each test passes. With cheap runs the simulator can be sampled
  directly.

## 4. What is evidence and what is input

| Pattern | Status in the rival |
|---|---|
| Low losses while the lines are formed | **Input.** The clock sets the bout rate. |
| Battles last hours | **Partly input.** The clock assumes a duration. That a line breaks, and when, is not written in. |
| The loser's losses fall mostly after the break | **Mixed.** That turned backs are deadly is written in. The size of the asymmetry is not. |
| Pulses and lulls | **Evidence.** Bouts are led, but nothing sets their rhythm. |
| Rear-first collapse | **Evidence, weakly.** Room to run and closers are written in; that they add up to a collapse from the rear is not. |
| Snap or crumble | **Evidence.** |
| The triplex outlasts a single line | **Evidence.** |
| Veterans beat recruits; a flank charge breaks recruits more often | **Written in for direction, evidence for size.** |
| The Gallic pattern | **Evidence.** There is no culture switch. |
| The centurion share | **A shared calibration target** (§5) |
| Sealed battles | **Evidence.** |

The vision of record has the same problem in a different place: its low losses come from a
defensive stance that is also a modelling choice. But it does not fix them from an assumed
clock, and the rival does. I count that against the rival.

## 5. The centurion share

Caesar reports centurion losses separately, which gives a rare number about who was hit.

| Action | Centurions lost | Other losses | Share | Kind of fight |
|---|---|---|---|---|
| Pharsalus (*BC* 3.99) | About 30 | Not more than 200 | About 13 % | A formed fight, won |
| Gergovia (*BG* 7.51) | 46 | Just under 700 | About 6 % | An assault thrown back |
| Dyrrachium (*BC* 3.71) | 32, counted with the tribunes | 960 | About 3 % | A panic; many crushed without a wound |

Centurions were 1.25 % of a legion on paper, and about 2 % of Caesar's depleted legions at
Pharsalus (*BC* 3.89).

**Caveats.** These are Caesar's own figures. For Pharsalus another tradition gives 1,200 dead
(Appian, from memory), which would bring the share down to about 2 %.

**It does not separate the two designs.** Paullus's objection is right. A centurion stands in
the front rank, where nearly all losses in a formed fight fall. If a century fights n files
wide and losses fall evenly along its front, his share of them is at most 1/n, and it drops as
losses mount, because the man who replaces him is not a centurion. For a full century ten
files wide that ceiling is 10 %. For Caesar's centuries at Pharsalus, under fifty men each, it
is higher, and 13 % does not clear it. "Leaders lead the bouts" and "leaders stand in front"
predict the same thing.

**What it is good for.** The gradient: high in a formed fight, lower in a repulse, lowest in a
panic. Both engines should reproduce it without being told to. A share above 1/n with n known
would separate the two readings, but I know of no source that gives one.

## 6. Seeds and machines

| Scale | Seeds | Where |
|---|---|---|
| The patch and the micro-tests | Thousands; seconds each | The Air |
| Full-scale Cannae | A hundred or more a night (target) | The Air |
| A1 dose-response, five levels | Two or three nights | The Air |
| Sealed battles | Hundreds of seeds each | The Air |
| A rented machine | Only if the speed target is missed | Michal's approval, as now |
