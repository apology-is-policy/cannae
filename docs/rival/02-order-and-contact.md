# 02 — Order and Contact

Two things happen to formed men in a battle: they keep or lose their order, and they meet the
enemy. This document covers both. Part A is how bodies of men hold a shape and move. Part B is
what happens where the lines touch.

## Part A — Order

### 1. Every body has a carrier

A body of men is the men who follow one carrier: a maniple and its standard, a warband and its
chieftain, a turma and its decurion. **The body's frame is the carrier's position and facing,**
steadied by its file leaders. It is not fitted to where the men are, and it is not imposed from
outside. The carrier is a man: he walks, decides, is hit, and falls.

This is how the armies worked. At the Sabis, surprised while building camp, each man fell in at
"whatever standards he first caught sight of, so as not to lose fighting time looking for his
own": *quaeque prima signa conspexit, ad haec constitit, ne in quaerendis suis pugnandi tempus
dimitteret* (*BG* 2.21). Paullus supplied the reference.

**On the march the frame goes where the carrier goes. Halted, it stands.** A halted body's
carrier has a place in the frame like any other man, and the frame moves only when he is driven
further from that place than his tolerance. Then it re-forms on him. So a pressed body braces
on a fixed line and gives ground a step at a time, and a body whose carrier steps out follows
him. Paullus caught the first build, which re-anchored on the carrier at every step and so gave
a held line no fixed point.

**When the standard goes down,** the frame freezes where it was, and the ties to it weaken.
Every man who sees it fall counts it as alarm ([01](01-the-man.md) §3). A bold man nearby may
pick it up, and the frame is restored. A unit that loses its standard and no one lifts it is a
crowd.

### 2. In his place, or out of it

Each man has a slot in his carrier's frame and a **tie** to it. Drill sets the tie: how stiffly
he holds the slot, and how quickly he follows when the frame moves. A man within tolerance of
his slot is **in his place**. Leaving it is an event, whether he steps out to fight, is pushed
out, or runs.

One mechanism covers every kind of body. Paullus pressed for this, and it is better than the
sketch's split between lattices and loose crowds:

| Body | Tie | Result |
|---|---|---|
| Maniple, cohort, phalanx | Stiff, quick | Ranks and files that hold under pressure |
| Warband | Loose, slow, wide tolerance | A mass around its chieftain that bunches when pressed |
| Turma, Numidian band | Loose, fast | A body that can scatter and re-form on its leader |

**Order is a measured quantity:** the share of a body's men in their places, and how far the
rest have strayed. A1 says the Roman infantry "loses its order" in the Carthaginian centre.
Here that phrase is a number the classifier reads, not a judgment about spacing.

### 3. The army is a flock of carriers

Carriers keep station on each other: a standard dresses on the standards beside it and on its
line's reference. An army's shape is therefore the motion of about a thousand carriers, and the
136,000 men follow by their ties. A general advance, a refused flank, a wheel inward: all are
movements of carriers, ordered by officers. This is also most of what the vision of record
calls the middle layer. A centurion's decision moves a carrier, and the men follow.

### 4. Drill and line relief

A drill is a change of slot map, with a duration and an error rate set by the body's drill:
close up, open out, face about, wheel, form a ring.

**Line relief** is built as R2, as in the vision of record: in a lull, the tired body's carrier
steps back through the interval, the fresh body's carrier steps up, and the men follow. The men
passing each other are crowded and moving, so their bodies touch (§5). Whether a passage jams
is then a result, as it is in the vision of record, and it can be checked against measured
flows through bottlenecks ([06](06-experiment.md) §2).

### 5. The field

*As proposed. What was built differs in several places, and one claim below proved wrong:
see [08](08-the-field-as-built.md).*

Each man deposits his presence on a grid of cells about a metre wide, spread smoothly over the
cells around him, which gives a **density** and a **pressure** for each side. That field
carries everything at long range. Bodies touch each other only where it matters (below).

- A man moves toward where he wants to be: his slot if he is in a body, away from the enemy if
  he has turned. His speed falls as density rises, following the walking-speed curves of
  pedestrian studies (Weidmann's fit, checked against a secondary source; see
  [08](08-the-field-as-built.md)).
- Pressure pushes from dense cells toward sparse ones. Both sides count, so two masses cannot
  pass through each other.
- **Crush.** Above a pressure threshold a man cannot use his weapons, his breath drains, and at
  the extreme he is injured. This happened. Of Caesar's losses at Dyrrachium, "a great part
  perished without any wound, crushed in the ditches, the works and the river banks in the
  terror and flight of their own side": *pars magna in fossis munitionibusque et fluminis ripis
  oppressa suorum in terrore ac fuga sine ullo vulnere interiit* (*BC* 3.71).
- **Othismos** stays a parameter: how much of the rear ranks' push reaches the front.

**Which pressure.** Paullus asked, and it matters. The field's pressure is an equation of
state: a function of density, zero below about two or three men to the square metre and rising
above. It is deliberately soft. It limits speed, spreads a crowd gently, and reports crush. It
does not have to hold men apart, because contact does that wherever they are crowded and
moving (below). A stiff pressure would be unstable at 4 Hz. A projection that enforces a
density limit, as continuum crowd models do (Treuille et al. 2006; Narain et al. 2009:
Paullus's references, from memory), is a solve every step. If the soft field with contact lets
a pocket pack beyond a believable density, the fallback is that projection, run only on the
dense cells with a fixed number of sweeps. The spike's pocket measures peak density for this
reason ([06](06-experiment.md) §3).

**At the contact line, positions and not cells.** Lines in contact stand about a cell apart,
so the grid cannot carry what happens there. Any man with an enemy within about three metres
is handled by exact positions: who is in reach, on which side, at his flank or his back. At
the densities of a fighting line the field's pressure is zero, so it does not push the two
fronts apart. Counts by cell serve the men further back.

**Where bodies touch.** Michal's read of the two substrates was that bodies feel more human
and the field more mathematically pretty, and he asked Paullus and me to argue toward common
ground. Paullus's case for bodies (call 0001, note 3) is that a field smooths away what is
granular. Men arch and clog at a gap. Force runs in chains through a packed crowd. One man
goes down and is trampled because of where he happened to stand. And a cell a metre wide
cannot say how many men pass a gap three metres wide: there the grid would set the answer, not
the men. He is right on each count.

His middle is the one astrophysics uses: a mesh carries the long range, and a pairwise
correction runs at short range where it matters. I accept it in this form. **Order decides the
physics:**

| A man who is | Moves by |
|---|---|
| In his place | His tie to his slot. Formed men do not shove each other. In both designs their jostle is mostly for the eye. |
| Out of his place, with room | The field: he steers, and density slows him |
| Out of his place, crowded and moving | **Contact with the bodies around him,** on top of the field: at a gap, a gate, a river bank, the churning edge of a pocket |

The packed interior of a pocket is crowded but hardly moves, so it stays on the field. Contact
is solved by position, a few passes a step, which stays stable at 4 Hz.

This is the engine's own principle applied to physics: spend the work where something is
happening. It also follows the sources, which keep formed bodies and crowds apart. Three
rungs go into the experiment against measured bottleneck flows: field alone; field with
contact where men are crowded and moving; contact everywhere. The lowest that passes is kept
([06](06-experiment.md) §2). I expect the middle rung to be needed wherever men pass a gap.

### 6. Fast bodies

A galloping horse covers 3.5 m in a quarter of a second, so fast bodies move in substeps. More
important, **a charge is decided before contact.** When the gap closes to a few tens of
metres, an event settles it from what each side can see: infantry that is steady and in its
places, or not; horses that will not run onto a steady wall. One side gives way, or the
charge pulls up. That rule is written in, as it is in the vision of record, so it is a unit
test and not evidence.

## Part B — Contact

### 7. Standoff is the rest state

Two formed lines stop just out of reach of each other, and stay there. Stepping into reach is
the *forward* act, and it needs ardour or a leader ([01](01-the-man.md) §3). Most men, most of
the time, do not have enough of either. The lull is not a recovery phase between clashes. It
is what a battle line does unless something starts a fight.

Sabin argued this for Roman battle (*JRS* 2000; from memory). Collins's study of modern
violence gives the same picture from direct observation: confrontation produces tension and
fear, most people are bad at violence, a small number do most of it, and the worst killing
comes when one side turns away (*Violence*, 2008). Du Picq said much of it first.

### 8. A bout

When a man who is forward has an enemy in reach, a **bout** may occur. It is one event, drawn
at a rate and resolved in one step from the situation:

- **reach and room:** his weapon's class against the density around him. A long cutting sword
  needs space; a short thrusting sword does not;
- **the side he is attacked from:** shield side, open side, behind;
- **numbers:** two on one is deadly;
- **skill, breath, and the class of shield and armour.**

The outcomes: nothing; the enemy driven back; hampered; disabled; dead. There is no chain of
blows and no body zones. A turned back multiplies the hazard many times over, and that is the
slaughter.

**Bouts are led.** A centurion or a bold man goes forward. Neighbours with enough ardour
follow. There is a flurry of bouts along a few metres of front. It ends when the leaders are
hit, winded, or left unsupported, and the line settles back into standoff. A drilled body can
also go forward together on a signal, which is a charge.

### 9. The casualty clock

Battle length and losses fix how much fighting there can have been:

> disabling hits per front position per hour
> = (share lost × men) ÷ (files × hours)

Take 40,000 infantry who lose 3 % in two hours of formed fighting on a 1.5 km front. At 0.9 m
per file that is one disabling hit per front position every 2.8 hours. At 1.8 m per file it is
one every 1.4 hours. Other plausible inputs move the answer by a factor of two or three, not
ten.

If a committed bout disables a man about one time in ten and lasts ten or twenty seconds,
there is room for a handful of bouts per front position per hour. **Committed fighting fills a
few per cent of the time at the front.**

The vision of record lets W′ time the clash: one to three minutes at full intensity, then
five to ten of recovery (04 §5.2). Paullus's answer is fair: a clash can be hard work without
being lethal, with shoving, shield work and wary fencing. So the clock does not refute the
oscillator. It constrains both models, and we have agreed to use it as a test: each engine must
meet the clock with a lethality per bout inside bounds fixed beforehand from duel experiments
([06](06-experiment.md)). I have no source for those bounds in hand yet.

One cost is real. If the clock sets the bout rate, then "low losses while the lines are
formed" is an input here, not a finding ([04](04-acceptance.md) §4).

### 10. Forward panic

Enemy backs in view raise the hazard of going forward and remove what holds a man back. That
is pursuit. Drill restrains it; an order can release it. The same act explains the Roman
disaster at Cannae: men who see the Gauls give way press in after them, leave their places,
and crowd. The vision of record says the same (09 §1). Here it needs no extra mechanism.

### 11. Missiles

A volley is an event from a body onto an area. It yields hits at a rate set by range, the
target's density, and its class of shield and armour. A hit may kill, disable, hamper, or pin a
shield, which hampers. Missiles are not flown one by one.

### 12. Wounds

Four classes: sound, hampered, disabled, dead. A share of the disabled die within days, which
feeds A1's casualty mapping. Where the wound was and what made it are not state. For the
inspector and the chronicle they can be drawn afterwards from tables consistent with the
outcome. That is flavour, and the model says so.

### 13. How each pattern arises

| Pattern | Mechanism here |
|---|---|
| Standoff | The rest state (§7) |
| Pulse and lull | Led bouts that end when the leaders are hit, winded or unsupported (§8) |
| Rear-first collapse | Rear and edge men have room to run and no closer behind them ([01](01-the-man.md) §3) |
| Snap or crumble | The spread of nerve within a body |
| A rout that does not heal | A turned man rallies only with no pursuer in view and a standing carrier to rally on |
| Dread under fire that cannot be answered | Volleys with no enemy in reach raise *shaken* ([01](01-the-man.md) §9). A candidate, to be tested. |
| Slaughter in flight | Bouts against turned backs; forward panic (§10) |
| The Gallic pattern | The bold stand in front, go in together, are spent together, and are not replaced |
| Roman staying power | Leaders appointed and succeeded by rule; closers behind; relief that brings fresh breath and fresh bold men |
| Compression | Pressure above the weapon-use threshold (§5) |
| Clogging at a gate or a gap | Contact between crowded, moving bodies (§5) |
| The inward surge at Cannae | Forward panic into a yielding centre; order lost; then compression |
| Envelopment | Carriers moved by officers on their own reading of the field ([05](../vision/05-command.md), [09](../vision/09-intent-and-plans.md)) |
