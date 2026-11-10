# 05 — Costs and Critique

What the rival costs, where I think the vision of record is wrong, overbuilt or timid, where
the rival is weak, and what should cross from one design to the other whichever wins.

## 1. What the rival costs

**Compute.** A target of 20–50× real time on one core, from arithmetic that nobody has
measured ([03](03-engine.md) §2).

**Implementation effort.** My judgment is that the engine is about half the work of the one in
[06](../vision/06-architecture.md). It has no collision solver, no threads inside a battle, no
integrator for the psyche, no second model at the level of bodies, no learned predictor and no
emulator. It has hard parts of its own:

- a fixed-point pressure field that stays stable;
- the bookkeeping that decides which men can be affected this step;
- carriers that keep station on each other without oscillating;
- hazard tables that are small enough to reason about.

That estimate is a judgment, not a measurement. The troop research, the command layer and the
validation regime cost the same in both designs.

**What Michal loses.**

- **The inner life.** No fear, fury or resolve gauges to inspect in a man. He gets
  dispositions, at most two marks, a posture history, and the cause of each change by name.
- **Wounds by body zone and the blood pool.** Four classes, and flavour drawn afterwards.
- **Some of the jostle.** Formed men tied to their slots will look more regular than soft
  bodies shoving. Where order has gone, bodies touch as they do in the vision of record. The
  viewer may still need to add visual life that is not state.
- **Perhaps part of the line-relief study,** if contact only where men are crowded misses
  something that contact everywhere catches. The experiment checks.
- **Some evidence** ([04](04-acceptance.md) §4).

## 2. Where the vision of record is wrong

1. **Little Cannae distorts the race A1 is about** (07 §9; 08, Slice 2). The cavalry's ride
   scales with the front and the centre's endurance does not ([04](04-acceptance.md) §1).
   Paullus conceded this.
2. **A clash is defined by how fast W′ drains** (04 §5.2). It has to be defined by what men
   do. The casualty clock leaves room for very little committed fighting, so minutes of full
   intensity must be nearly harmless or rare ([02](02-order-and-contact.md) §9).
3. **A frame fitted to men who steer toward it is circular** (03 §1). Each follows the other.
   The sources say what the men followed: the standards.
4. **The engine is dearest where the men are most helpless.** 06 §4 concedes that activity
   levels save little in the encirclement, where speed falls toward 2×. The pocket's interior
   is 60,000 men who cannot reach an enemy.
5. **The Slice 0 toy cannot show what it is for.** In one dimension it has either rear ranks or
   a line along which a break can spread, not both. Paullus agreed.

## 3. Where it is overbuilt

1. **Seven changing variables in each man's mind,** while its own 04 §5.6 doubts whether the
   individual needs to be bistable at all.
2. **Three perception grids** with sound, dust and rumour fields, and a vocabulary of rumours
   that distort.
3. **A five-stage combat chain, body zones and a blood pool** for 136,000 men, validated
   against casualty totals and nothing finer.
4. **Two models that must agree:** the soldier engine and the aggregate crate.
5. **The apparatus that expensive runs force:** a learned predictor, an emulator, a scale
   study.
6. **Bit-exact floats across machines and thread counts.** That is needed only because battles
   are threaded inside and acceptance runs on a rented machine.

The vision of record marks much of this as later features, and its parsimony pillar is sound.
My objection is that the architecture is sized for the whole list, and pays for it from the
first slice.

## 4. Where it is timid

1. **It accepts that its own acceptance test cannot run on the machine it is built for.**
   Michal asked for his machine.
2. **The toy asks which kernel, not whether a man needs a dynamical system.**
3. **Parsimony is applied to mechanisms and never to the substrate.** Pairwise collision for
   every body at 20 Hz entered without a named pattern that needs it everywhere. Paullus has
   since named where it is needed: gaps, gates and banks, where crowds clog. That is a few per
   cent of the men.

## 5. Where the rival is weak

**Paullus's objections, and what I did with them.**

| Objection | Disposition |
|---|---|
| The casualty clock does not force "fighting is led" | **Accepted.** It is a test on both engines. |
| The rival already has reservoirs, and needs a memory of having fled | **Accepted.** W′ is kept as physiology. The *has fled* mark is added, with an ablation. |
| There is nowhere for fear to outlast its cause | **Accepted as a candidate.** *Shaken*: one small integer, raised by visible shocks, fading in minutes, kept only if the ablation needs it ([01](01-the-man.md) §9). |
| Lattices and loose crowds are two representations | **Accepted.** One mechanism: a carrier and a tie. |
| A horse moves 3.5 m in a step at 4 Hz | **Accepted.** Substeps, and the charge decided before contact. |
| Line relief becomes an input; a field smooths away what is granular; a metre cell cannot resolve a three-metre gap | **Accepted.** Bodies touch where men are crowded and moving, so a jam is a result again ([02](02-order-and-contact.md) §5). Three rungs go into the experiment. |
| The speed is arithmetic | **Accepted.** The spike decides. |
| The centurion share does not discriminate | **Accepted** ([04](04-acceptance.md) §5). |

**My own doubts.**

1. **The central bet.** A line must hold for an hour and then break with no stored morale. A
   threshold model can easily do one of two wrong things instead: stand frozen for ever, or
   collapse at the first alarm. The pulsing regime may be a knife-edge here too.
2. **Hidden parameters.** The weights that turn visible acts into alarm and support are free
   parameters. Paullus's rough count of the hazard table is 30–40, above my first estimate for
   the whole man. The line-by-line count is owed before any scored run.
3. **Rhythm.** Led bouts may look like history, or like noise.
4. **A thin man may be a shallow one.** Experience becomes four numbers and two rules.
   Michal's Legio X must still feel different from recruits in ways he can see.
5. **What I cannot see from here.** I have read the vision of record for a morning. Paullus
   has lived with it. Some of what I call overbuilt may answer a problem I have not met.

## 6. Alternatives considered and rejected

| Alternative | Why not |
|---|---|
| Units as the entity, soldiers as decoration | It deletes emergence, which is Michal's first wish |
| Fat agents of five to ten men | It preserves the geometry of the race but gives up 1:1 |
| A continuum of two fluids | No individuals at all |
| Keep the dynamical psyche and optimise harder | It might reach 10–15×. It leaves the inner variables as unconstrained as before. |
| Nerve held by the tent-group, not the man | Rear-first collapse needs men who differ by rank |

## 7. What should cross over

**If the rival loses,** the vision of record should still take:

- the standard as the frame. Paullus would adopt it whichever engine wins;
- the casualty clock as a constraint, and the two-dimensional patch;
- A2 from the ensemble wherever runs are cheap enough, and fate curves by forking;
- the *has fled* mark, and the centurion gradient as a target;
- the lesson of the race: A1 needs 1:1, so the vision of record needs either more speed or the
  rented machine sooner than planned.

**If the rival wins,** it takes from the vision of record nearly everything that is not the
engine or the mind: the armies and equipment ([01](../vision/01-armies.md),
[02](../vision/02-equipment.md)), command and plans ([05](../vision/05-command.md),
[09](../vision/09-intent-and-plans.md)), the validation regime
([07](../vision/07-validation.md)), and the research on the contact zone
([10](../vision/10-combat-and-wounds.md) §1 and §3) as the basis of the bout tables.

The two designs share more than they dispute. What they dispute is the man and the substrate,
and an experiment can settle both ([06](06-experiment.md)).
