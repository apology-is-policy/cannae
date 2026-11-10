# 07 — The Man, As Built

[01](01-the-man.md) says what moves a man. This document says exactly how, as it is written in
`crates/cannae-man`: the sums, the thresholds, what he remembers, and what was left out. It
covers the three lower rungs of the memory ladder. Every number has a prior in
[priors-field.md](../slice0/priors-field.md); the values quoted here are the centres.

## 1. Two sums and one curve

A man has five postures: steady, forward, back, turned, down. At every man step (four times a
second) each way out of his posture has a **hazard**, a chance per second. Every hazard comes
from one of two sums:

- **Alarm:** what he sees that pushes him toward flight, less what he sees that holds him, less
  his nerve.
- **Urge:** what sends him forward at the enemy.

Each hazard is the same curve of a **margin**, which is a sum set against a threshold:

> hazard = (1 in 16 seconds) × 2^(k × margin), and never faster than his reaction time

At a margin of zero he goes in about sixteen seconds. Each unit of margin doubles the hazard k
times. A few units below zero the hazard is nothing; a unit or two above, he goes at once. So
each man has a threshold with a little chance around it, and thresholds differ from man to man
because nerve and ardour do. That is all the randomness there is in a man.

**The units.** Alarm is counted in turned men: one man of his own side, turned, in his view,
is one unit. Urge is counted in forward men: one man of his own side, forward, in his view, is
one unit. Every other weight says how many such men a thing is worth.

## 2. Alarm

"In view" means in his window: the five-by-five block of 2 m cells around him. "In reach" means
within 3 m.

**Acts** (what others are doing):

| He sees | Counts | Worth |
|---|---|---|
| A man of his side turned | each, up to 8 | 1 |
| A man of his side giving ground | each, up to 8 | 0.35 |
| A man of his side going down, at this step | each, up to 3 | 2 |
| An enemy in reach on his left or right, who faces him | each, up to 3 | 1.5 |
| An enemy in reach behind him | each, up to 3 | 3 |
| An enemy in reach in front who has stepped forward | each, up to 3 | 0.5 |
| His leader, within 20 m, turned | | 3 more |
| His leader, within 20 m, giving ground | | 3 × 0.35 more |

An enemy counts only while he stands and faces the fight. An enemy beside him who is fighting
his neighbour is his neighbour's business.

**Conditions** (how things stand):

| | Worth |
|---|---|
| A slot beside his own that exists and is empty | 0.5 each |
| His standard is down, or is being carried off by a man in flight | 2 |
| His leader is down, within 20 m | 1.5 |

**Support** (subtracted):

| | Counts | Worth |
|---|---|---|
| Men of his side standing beside him and ahead of him | each, up to 6 | 0.25 |
| Men of his side standing behind him | each, up to 6 | 0.25 |
| His leader standing, within 20 m | | 3 × 0.25 |

**His nerve** (subtracted): a disposition drawn at setup, about 2 units for a legionary of the
first line. It is lowered by:

- a wound that hampers him: 1;
- from rung 1 up, having fled before: 1;
- at rung 2, each level of *shaken*: 0.75.

A man in the middle of a steady body, with nothing wrong, sits about 5.75 units below the edge.
A rear-rank man has no one behind him, so he sits 1.5 units nearer to it. So does a man at the
end of a line, for want of men beside him.

**Conditions alone move no one.** A steady man has no hazard at all unless something he sees
is an act: an enemy in his window, a shock, a man of his side wavering, or an order to charge.
A body whose standard is down stands, unhappily, until something happens near it. Then the
conditions count in full. This is also what lets the engine skip men who have nothing to react
to, without changing a single outcome.

## 3. Urge

| | Counts | Worth |
|---|---|---|
| His ardour | a disposition | about 0 for a soldier, 2 for a centurion |
| A man of his side forward | each, up to 8 | 1 |
| His leader forward, within 20 m | | 3 more |
| An enemy ahead who has turned his back | each, up to 6 | 1.5 |
| An enemy ahead giving ground | each, up to 6 | 0.5 |
| An order to charge or pursue | | 6 |
| A man of his side giving ground or turned | each, up to 6 | −1 |
| Being winded | each full quarter of his breath gone, up to 3 | −2 |

## 4. The six ways to change

| From → to | Margin | Only if |
|---|---|---|
| Steady → forward | urge − 4 | an enemy is in view, or he is ordered to charge; and he has no hampering wound |
| Forward → steady | 3 − urge | (at once if no enemy is left in view, or if he is hampered) |
| Steady → back | alarm | |
| Back → steady | −alarm − 4 | |
| Back → turned | alarm + 1, + 1.5 with room behind him, − 1.5 with a closer behind him | |
| Turned → steady | −alarm − 3 | no enemy in view; his standard within 60 m, up, and not being carried off |

The alarm exits double 2.5 times per unit. The urge exits double 3 times per unit.

What these numbers mean in practice, at the centres:

- **Who leads.** With the enemy standing in front and no one forward, an ordinary soldier steps
  out about once in eighteen hours. A centurion does it about once in seventeen minutes.
- **Who follows.** When the centurion goes, each soldier who sees him is at the edge and
  follows in seconds. Each follower pulls the next. That is a led bout.
- **Why it ends.** At the placeholder cost of fighting, a man's breath lasts a little over two
  minutes. Once a quarter of it is gone he is two units less eager; once half is gone, four. As
  men drop out, the others have fewer to follow. Breath comes back in five to ten minutes of
  standing. The cost of fighting is a shared prior, and a wide one: no study measures it.
- **Giving ground.** One more man turned in view multiplies the hazard of giving ground by
  about 5.7. A man who gives ground is one unit nearer to turning than a steady man is to
  giving ground: the backward step is the beginning of flight.
- **Rear first.** A man with room behind him turns 13 times more readily than a man hemmed in,
  and a rear-rank man also lacks support from behind. A closer standing behind him takes the
  room away.
- **Rally.** A turned man stops only when nothing is after him, his standard stands in sight,
  and enough men around him stand. Among men who are running, he runs.

A man who is forward does not go straight to giving ground. He steps back into line first, and
that hazard rises as soon as his own side wavers.

## 5. What he does in each posture

| Posture | He moves | Gait |
|---|---|---|
| Steady | to his slot | walk |
| Forward | to within 1 m of the nearest enemy in reach who is still up; with none in reach, to 2.25 m ahead of his slot | walk; run under a charge or after a back |
| Back | to 1 m behind his slot, still facing the enemy | walk |
| Turned | away from the way his body faced | run |

A man with no breath left cannot run. Nothing restrains pursuit yet (§8).

## 6. What he carries

Everything that a man carries from one moment to the next:

| | What | Rung |
|---|---|---|
| Posture and wounds | Plain to see; everyone counts them | all |
| Breath | W′: drained by fighting and running, restored by standing. Stored as its value at his last change of posture, and read forward from there. | all |
| *Has fled* | One bit, set when he turns his back. Never cleared. | 1, 2 |
| *Shaken* | A count from 0 to 3. Each fall he sees raises it one; so does the fall of his standard or his leader. One level wears off every five minutes. | 2 |

*Shaken* is raised by rule, not by chance. An earlier design drew a chance for each fall. The
rule has one parameter fewer and needs no random number.

**The three rungs differ in nothing else.** With the same seed, rung 0 and rung 1 are the same
battle, bit for bit, until the first man turns. Rung 1 and rung 2 are the same battle until the
first man is seen to fall. A test holds them to that.

## 7. Bouts

The bout resolver is shared by both engines ([02](02-order-and-contact.md) §8). Three of its
rules shape how this man fights:

- **Only a forward man starts a bout,** with the nearest enemy in his reach and within a
  quarter turn of his facing. Two steady lines do no harm.
- **The rate belongs to the man, not the pair.** Two men who are both forward at each other
  each start bouts, so they exchange twice as often as a forward man does with a steady one.
  Each strike also lands on a man who has stepped into it (×1.5), and each answer is a full
  strike, not a parry and riposte (×0.7). At the placeholder values a mutual pair do each other
  about three times the harm per second of forward against steady.
- **So lethality per bout is a mix.** It is about 1.5 times higher between two forward men
  than between a forward and a steady one. The number the casualty clock tests is the average
  over whoever was forward. Paullus raised this in review.

Breath does not enter a bout's odds. It decides who is forward.

## 8. What is left out

Each of these is in [01](01-the-man.md) and is not built. None enters before a named pattern
fails without it.

| Left out | What stands in for it |
|---|---|
| **Drill as a trait of each man** | Drill lives in the body's tie: tolerance, how fast files close, who lifts the standard, who succeeds the leader ([02](02-order-and-contact.md) §2) |
| **Restraint in pursuit** | None. A forward man chases a back for as long as his urge and his breath last. |
| **Bodies lying in view; wounded men coming back** | Falls count for one step, and at rung 2 as *shaken* |
| **A fresh line behind** | Men standing behind him count, whichever body they belong to |
| **Being outnumbered; threats beyond 3 m** | Only enemies in reach alarm him. Enemies in view let him go forward and keep him awake. |
| **Crowding** | Nothing. The substrate handles the crush. |
| **Leaning into a press** | Every man presses with weight 0 |
| **An ordered retirement that alarms no one** | A body ordered to retire is steady men following a frame that moves back. No one is giving ground, so no one is alarmed. |
| **Turning to face an enemy beside or behind him** | A steady man faces the way his body faces. Only a forward man faces the man he fights. So a body taken in flank is struck without answering, which makes flank attacks strong. |
| **Rallying on another body's standard** | A man rallies only on his own |
| **Fatigue from marching** | A steady man is taken to be standing, even on the march |
| ***Shaken* wearing off sooner with a leader in view** | It wears at one rate |
| **Skill at reading what he sees** | Veterans differ in nerve and in skill at arms only |

Two rules of the frames matter for what breaks a body, and they are not weights of the man:

- A drilled body lifts a fallen standard after a few seconds and replaces a fallen leader with
  its closer. A warband does neither.
- So a warband whose chieftain falls carries 1.5 units of alarm, and loses his support, for the
  rest of the fight. A century carries them for five seconds.

## 9. How many numbers

I said in the first draft that the whole man would take 25 to 35 parameters, and Paullus said
30 to 40 for the visible layer alone. He was nearer.

| | Count |
|---|---|
| Weights and thresholds of the two sums, rung 0 | 27 |
| How far he steps (back, and how near he closes) | 2 |
| *Has fled* (rung 1) | 1 |
| *Shaken* and how fast it wears (rung 2) | 2 |
| **The man: rung 0 / 1 / 2** | **29 / 30 / 32** |
| Troop inputs of this engine (nerve, ardour and their spreads, officers), per troop type | 5 to 7 |
| Shared with the other engine: ties (10), bouts (16), kit (1), and breath, power and gait | in [priors-shared.md](../slice0/priors-shared.md) |

Fixed, and not counted as free: the window and the reach (they are the shared perception), the
caps on each count, the edge rate of one in sixteen seconds, the two sight distances, the top
level of *shaken*, and the forward step of 2.25 m. They are listed in the priors file so that
nothing is hidden in the code.
