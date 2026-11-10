# Slice 0: Rungs 3 and 4 of the memory ladder (design, S0.6)

**Status:** a design by Paullus, 2026-11-10, written before any code. Rungs 0-2 are Varro's
(`crates/cannae-man`, [rival/07](../rival/07-hazards.md)). The ladder is nested: a rung adds
memory to the one below and changes nothing else, so the perception weights (alarm and urge
sums) are the same on every rung, and draw *i* of a rung is draw *i* of the rung below plus
its own new parameters ([preregistration.md](preregistration.md), "Common draws").

The hook is a `Memory` trait in cannae-man, which Varro is writing. A memory supplies one term
to a man's alarm and one to his urge, each read in closed form at the tick, and absorbs what he
saw at each step he is awake. Three rules come from dormancy:
- a term cannot wake a man;
- anything that decays is stored as (value, tick) and read forward;
- each part of a term names its cause.

## Rung 3: fear and resolve

What rungs 0-2 cannot do is let a fright outlast its cause, or let a long fight wear a man down.
At rung 2, *shaken* goes up a level at each fall a man sees and wears off a level at a time. It
is a step count, and nothing drains. Rung 3 replaces it with two continuous gauges.

**Fear, F** (in alarm units). It rises while he sees frightening acts and decays when he does
not:
- At each step he is awake, `absorb` first reads F forward to now, then adds g · acts · Δ.
  Here acts is the visible layer's sum of frightening acts and Δ is the man step (250 ms).
- Read at tick t: F(t) = F₀ exp(−(t − t₀)/τ_F).
- Under acts held at a constant A, F settles at K·A, where K = g·τ_F. So K is how much of a
  fright's weight he keeps carrying.

**Resolve, R** (per mille of full). It is what a long fight spends:
- At each step he is awake, R loses F/T_R · Δ (floored at zero). Under a fear of one alarm
  unit, full resolve empties in T_R.
- Read at tick t, it recovers toward full: R(t) = 1000 − (1000 − R₀) exp(−(t − t₀)/τ_R).
- Resolve is spent only while he is awake to something. A dormant man's fear still decays,
  but spending it while he sees nothing would couple the two gauges in a way that has no
  simple closed form. Dormant men see nothing frightening, so the error is small, and it is
  named here.

**The alarm term:** F + w_R (1000 − R)/1000, in two parts, named `Cause::Fear` and
`Cause::Resolve`. The urge term is zero. Rung 3 replaces rung 2's *shaken*, but keeps rung 1's
*has fled*, since the ladder is nested.

**What it should do, written beforehand:**
- **A hazard that rises over a long fight.** Resolve drains while the lines are formed, so the
  time to break should have a Weibull shape above 1, and above rung 2's. If the shapes match,
  the gauges have not earned their place.
- **A fright that lingers.** After a shock that is driven off (the `shock` variant), men
  should stay nearer the edge for minutes. *Shaken* does that in steps, so rung 3 must do it
  measurably better, or the ladder keeps rung 2.
- **Volleys that kill few but wear men down** (the `volleys` variant): fear accumulates, and
  resolve drains, under fire that kills nobody.

**Priors** (to go into [priors-pairwise.md](priors-pairwise.md) before the freeze):

| Parameter | Prior | Reason |
|---|---|---|
| τ_F: how long fear lasts | L[1, 10] min | "Minutes", the same scale as Varro's `shaken_wear` (2-10 min), so the rungs differ in form, not in time scale |
| K: how much of a fright's weight he keeps carrying | U[0.25, 1] | At 1, a man who has watched a threat for several τ_F carries it in full even after it has gone; at a quarter, a remnant |
| T_R: how long a fear of one alarm unit takes to empty full resolve | L[10, 60] min | Formed fights lasted hours, and lines were relieved (the triplex acies; vision 03, R2). Resolve is spent over tens of minutes, not seconds |
| τ_R: how long resolve takes to come back | L[10, 60] min | A relieved line rested long enough to go back in (vision 03, R2) |
| w_R: alarm at empty resolve | U[1, 3] | About a man's whole nerve (2, priors-field.md T1): a man with no resolve left has no nerve left |

**Count: 5.** They draw nothing at runtime.

## Rung 4: the full excitable psyche

The vision of record (04) gives the man a fast approach-avoid state with a threshold and
refractoriness, plus slow stamina and resolve. On the call I conceded that the fast layer is
hard to tell from a hazard read off what he sees with a reaction-time lag. Varro's postures
already have hysteresis, and his urge already reads breath (`winded`), so the W' cycle already
checks going forward on every rung. Rung 3 adds the slow memory. What is left that rung 4 alone
would add:

1. **Graded intensity inside "steady":** how hard a man leans into the press, and how close he
   stands at the standoff. In the vision this comes out of the approach-avoid balance.
2. **Refractoriness beyond breath:** after a forward excursion, a spell in which he will not go
   again, even with breath left.

Neither is tied to a named pattern that rungs 0-3 obviously fail. By the parsimony invariant, a
mechanism enters only when a named pattern cannot be reproduced without it. **Michal decided
(2026-11-10): rung 4 is built only if rungs 0-3 all fail.**

## The press, decided

Varro found that in the field a man in his place moves by his tie alone, so a press for formed
men would act on one substrate and not the other. We recommended, and **Michal decided
(2026-11-10), that in Slice 0 only men out of their places press**, on both substrates.
`Intent::Close` carries a press for that; `HoldSlot`'s stays zero. Whether a formed line shoves
is put off to Slice 1, when the field gets a push at the level of the body. The rule and its
shared prior, which includes zero, go in the visible layer (Varro's crate). See
[preregistration.md](preregistration.md), "The press".
