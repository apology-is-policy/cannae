# 01 — The Man

The vision of record makes each soldier a small dynamical system. The rival makes him a short
record of what a witness could see, and a short list of what he is like. This document says
what is in that record, what moves a man from one posture to another, and what does the work
that a morale variable does elsewhere.

## 1. State

**Dispositions.** Five numbers, drawn once at setup from the troop type's distributions, and
never changed during a battle:

| Disposition | What it sets |
|---|---|
| **Nerve** | How much alarm he bears before he gives ground, and before he turns |
| **Ardour** | How readily he steps forward to strike, and how readily he follows another who has |
| **Skill** | His odds in a bout; how well he reads what he sees |
| **Physique** | Critical power and W′ capacity |
| **Drill** | How stiffly he keeps his place, how fast he performs a drill, how far an order outweighs his own inclination |

**What a witness could see.**

| State | Values |
|---|---|
| **Place** | His body's carrier, his slot in its frame, and whether he is in it ([02](02-order-and-contact.md) §2) |
| **Position and facing** | Fixed-point coordinates |
| **Posture** | Steady, forward, back, turned, down (§2) |
| **Wound class** | Sound, hampered, disabled, dead |
| **Breath** | W′, as in the vision of record: published physiology, not a tuned number |
| **Marks** | *Has fled* and *shaken* (§4, §9). His role: soldier, file leader, closer, standard-bearer, centurion |

That is about 24 bytes of changing state per man, against about 100. Later features may add a
mark or a counter when a named pattern needs one (heat, hunger, a stressor he has never met).
They do not add a continuous inner variable.

## 2. Five postures

| Posture | What a witness sees |
|---|---|
| **Steady** | In his place, facing the enemy, on guard. The default. |
| **Forward** | Stepped out to strike or to throw; committed. Against turned backs, this is pursuit. |
| **Back** | Giving ground, still facing the enemy |
| **Turned** | His back to the enemy, running |
| **Down** | Disabled or dead |

Marching, wheeling and waiting are orders to a body, not postures of a man. A recoil (back) and
a rout (turned) are different acts from the start. Review 1 found that the first psyche draft
could not tell them apart; here they cannot be confused.

## 3. What moves a man

Every change of posture is an **event**. Each possible change has a hazard: a chance per
second, computed from the man's dispositions, his body, his body's order, and the visible acts
of the men around him. Nothing else enters. What he can see is counted, not sensed: how many of
his own side nearby are steady, forward, back, turned or down; whether the slot beside him is
empty; whether his leader and his standard are up; how many enemies are in reach, on his flank
or behind him, and what they are doing.

| Change | Raised by | Lowered by |
|---|---|---|
| Steady → forward | Ardour. A leader forward in view. Neighbours forward. Enemy backs. The order to charge. | Spent breath. Wounds. A steady enemy in numbers. His own side going back. |
| Forward → steady | Spent breath. A hit. Few neighbours forward with him. | Enemy backs in view |
| Steady → back | **Alarm above nerve.** Alarm counts turned backs of his own side, an empty slot beside him, enemies on flank or rear, his leader or standard down, pressure from the front. | **Support:** steady neighbours, ranks behind him, a leader, the standard, a fresh line behind. An order to retire makes going back the expected act, and then it alarms no one. |
| Back → turned | Alarm still above nerve. Room to run. | A closer behind him: the optio's staff. |
| Turned → steady (rally) | No pursuer in view. A standing carrier in view, with steady men around it. Drill. | The *has fled* mark |
| Any → down | A bout, a missile, a crush ([02](02-order-and-contact.md)) | Shield, armour, room to move |

"Alarm above nerve" is a threshold with a little noise: the hazard is near zero below it and
high above it. Thresholds differ from man to man, so a collapse runs through a body as an
avalanche, exactly as the vision of record's 04 §5.5 describes. The rival takes that mechanism
as the whole of the psyche, not as one layer of four.

**Rear-first collapse** needs no rumour field. The men at the rear and on the edges have room
to run, no fight to hold their eyes, and wounded men coming back past them. They leave first
unless a closer stands behind them, which is what the Roman optio is for.

## 4. Nothing stores morale

No number in the model is a man's morale, and none is a unit's. Five things do the work that
the resolve variable R does in the vision of record. The first three are plain to see. The
last two are marks of memory, and each must earn its place:

1. **Breath.** W′ drains in a charge, a pursuit and a press, and refills in a lull.
2. **The stock of the bold.** The men who step forward are the men who get hit. Hour by hour a
   line loses the men who led its bouts and anchored its ranks. Nothing in the model counts
   them; the population simply thins. Relief by a fresh line restores it. A body with no
   relief cannot.
3. **What accumulates in view.** Gaps, bodies, fallen leaders, a standard that went down.
4. **The mark.** A man who has fled once and rallied carries *has fled*, which lowers his nerve
   for the rest of the day. Paullus proposed it. Rallied troops were less reliable than fresh
   ones, and the cavalry that Caesar's fourth line broke at Pharsalus did not come back but
   made for the high hills (*BC* 3.93). It is an event remembered, not a gain to tune, and it
   stays only if an ablation shows a pattern needs it.
5. **Shaken.** A small integer that visible shocks raise and minutes wear away (§9). Paullus
   proposed this too. It is the least memory of fear a man can have, and it is admitted on
   the same terms as the first mark.

Breath is a reservoir, and Paullus is right to say so. It is kept because it is physiology,
measured in laboratories, and because a witness can see a winded man.

**This is the rival's central bet.** A line must hold for an hour and then break, with these
things and no slow reservoir of resolve behind them. The experiment climbs a ladder: no marks,
then *has fled*, then *shaken*, and keeps the lowest rung that passes. If the top rung fails,
the rival is wrong ([06](06-experiment.md)).

## 5. What Michal asked for, in these terms

| He asked for | Here |
|---|---|
| **Fear and morale** | Nerve against alarm, per man. A unit's morale is measured, as in the vision of record: the share of its men steady, forward, back and turned, and the share in their places. The viewer shows both. |
| **Experience** (Legio X against recruits) | Higher nerve and skill. A truer reading of what he sees: a veteran does not count an ordered step back as alarm, and a recruit may. Pacing: a veteran goes forward when he has breath. |
| **Discipline** | Drill: he keeps his place, obeys the signal, and does not chase turned backs unless ordered |
| **Auxiliaries that differ** | Different distributions of the same five dispositions. No culture switch. |
| **A unit's history** | Shifts its men's nerve at setup: a habit of victory, the memory of a defeat |

A warband differs from a maniple in four ways, all of them inputs a source can speak to. More
of its ardour stands in front: nobles and champions. Less drill. No appointed closers. No
relief. The Gallic pattern, a fierce first charge and then collapse, should follow: the bold go
in together, are spent together, and nothing replaces them.

## 6. Leaders

A leader is a man whose acts weigh more in other men's counts. A centurion going forward is
seen by eighty men, and counts in their hazards as several neighbours would. A leader has high
ardour and nerve because he was chosen for them. There is no morale field around him. He works
by being seen.

- **The centurion in front** starts bouts. He is also hit more often than his men
  ([04](04-acceptance.md) §5).
- **The closer behind** blocks the step from back to turned for the rear rank.
- **The standard-bearer** carries the body's frame ([02](02-order-and-contact.md) §1).
- **Succession is a rule:** when a centurion falls, the optio takes his place. A warband has no
  such rule, and its bold men are not replaced.

## 7. What is deliberately absent

| Absent | Why, and what happens to its patterns |
|---|---|
| The fast state x; fear F; fury Φ | Postures and their hazards do this work. Fury is ardour meeting enemy backs. |
| Resolve R | §4 and §9 |
| Helplessness H | Volleys a man cannot answer raise *shaken* (§9). Carrhae is not on the path to Cannae; when it arrives, it tests whether that is enough. |
| Shame S | A closer or an officer in view already lowers the hazard of going back |
| Beliefs and rumours | A soldier acts on what he sees and on the signals he hears. News travels as visible acts, which is how a redeployment could be read as flight at Munda. Officers keep their beliefs ([05](../vision/05-command.md)). |
| The habituation vector | A per-stressor shift in nerve, added when elephants are |

## 8. Every rout has a genealogy

Each event records its causes: the acts that contributed most to the hazard, and the men who
performed them. So a collapse is a tree: who turned first, who followed whom, where a closer or
a centurion stopped a branch. Three things read that tree:

- the **inspector**, to answer "why did this man run?" in one line;
- the **outcome classifiers**, which then need no thresholds on inner variables;
- **A2**, whose "identifiable events" are the roots of the large trees.

## 9. Should fear outlast its cause?

After the sketch, Michal said that the vision of record's fear model "makes sense humanly" and
the rival's "makes sense computationally", that he was not sure, and that Paullus and I should
argue it out. Paullus opened on yip call 0001 (note 2). This is my side.

**It is not about compute.** Paullus is right. A fear level per man would cost this engine a
byte and no measurable time. The speed comes from the substrate ([03](03-engine.md)), not from
the thin man. The case for the thin man is the witness rule: what can be checked, and what can
be explained.

**What each choice really buys and costs.**

| | A fear level in each man | Hazards from what he sees |
|---|---|---|
| Buys | Fear that lingers after its cause has gone; dread that builds | Every act has reasons you can read. Nothing to tune that no source constrains. |
| Costs | Gains, time constants and decay rates that a small corpus cannot pin down. An inspector that says "fear 0.73". | Fear ends the moment its cause leaves his sight |

**Both are human.** People run because they see others run. Fear as a reading of the situation
is as human as fear as a feeling. But a man who has just been struck from behind does not
become calm when the attacker is driven off. The sketch had no place for that. It was a real
gap, and Paullus found it.

**The sources report fear by its signs, and sometimes as lasting.**

- At Cannae, some five hundred Numidians feigned desertion, were placed in the Roman rear, and
  then fell on the line from behind. They made "a great slaughter, and a good deal greater
  panic and confusion": *stragem ingentem ac maiorem aliquanto pavorem ac tumultum* (Livy
  22.48). The fear was larger than the killing.
- At Sentinum, Fabius held that in a long fight the Samnites' spirits sink little by little,
  *sensim residere Samnitium animos*, while the Gauls' bodies as well give out under toil and
  heat (Livy 10.28). The vision of record cites the second half for W′. The first half
  separates spirit from body, and it describes a slow change of mind. It is a general's belief,
  not a measurement, but it is a pattern a model should be able to produce.

**The middle: *shaken*.** Paullus proposed one small integer per man. Visible shocks raise it:
a neighbour killed beside him, enemies behind, the standard falling, a volley he cannot answer.
It wears away over minutes, sooner with a leader in view. While it is raised, his nerve is
lower. I accept it as the first candidate for a memory of fear, on three conditions that keep
it inside the witness rule:

1. **Events set it and clear it,** and each is logged with its cause.
2. **It has signs.** A shaken body can be seen, as a winded one can, and the share of a unit's
   men who are shaken is shown.
3. **It earns its place by ablation.** The engine runs with no marks, with *has fled*, and
   with *shaken* added, and keeps the lowest rung that passes ([06](06-experiment.md) §2).

The rule that results: **a man may carry marks of what has happened to him. He does not carry
gauges.** A mark is discrete, set by an event, and explained in the log.

**What still differs from the vision of record,** even with *shaken*:

- no fast inner variables. Postures and hazards replace x, F and Φ;
- one small mark where the vision of record has four slow variables (R, H, S, B);
- bouts are led, not oscillated ([02](02-order-and-contact.md) §7).

**The first difference may be smaller than it looks.** In the vision of record x, F and Φ have
time constants of about a second. A variable that follows what a man sees with a second's lag
is hard to tell from a hazard read off what he sees, with reaction time as a parameter. If
that is right, what really divides the two men is the slow part (how much memory) and what
sets the rhythm of a clash. Both are on the experiment's list.

**What Michal would see.** Click a man: "Steady. Shaken, 2 of 3, since the standard fell four
minutes ago. Chance to give ground: 4 % a second, because three neighbours have turned, the
left is open, and no closer stands behind him." Click a unit: the shares steady, forward, back
and turned; the share in their places; the share shaken. That panel is the unit's morale as a
thing to watch. It is made of counts, and nothing stores it.

**Where I stand.** With *shaken*, the rival's man has a temperament, a memory and a state of
mind with visible signs. I think that is a psyche in the sense Michal asked for, and one he can
inspect more honestly than a gauge. Without any mark he is closer to a reflex, and I no longer
argue for that version as the default: it is the bottom rung of the ladder, there to be beaten.
I expect the engine to need *shaken*. Paullus and I both lean to this middle. We differ on what
surrounds it, and the patterns should settle that. The choice of what the vision says is
Michal's.
