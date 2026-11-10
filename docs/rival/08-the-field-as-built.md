# 8. The Field as Built

**Status:** Varro, 2026-11-10. This describes `crates/cannae-field` at commit 09e4326. It
replaces [02](02-order-and-contact.md) §5 wherever the two differ, and it lists what the
substrate does badly and what it leaves out. Its parameters and their priors are in
[priors-field.md](../slice0/priors-field.md) §4. Its speed is in
[spike-field.md](../slice0/spike-field.md).

## What it is

Men are points with a side, a posture and a place they want to be. Once every man step (250 ms)
the substrate moves all of them at once, on one thread, in integers. Three things decide how a
man moves:

| A man who is | Moves by |
|---|---|
| In his place in a body | His tie: straight toward his slot at his gait. Nothing else touches him. |
| Out of his place | The field: the density of men around him and ahead of him |
| Out of his place and near an enemy | The field, and then his exact position against theirs |

"In his place" is the frame's judgement, not the substrate's: within the tie's tolerance of his
slot (`bodies.in_place`).

## One step

1. **Deposit.** Every man on his feet spreads his presence over the four cells whose centres
   surround him, in proportion to how near each centre is. Cells are 1.024 m wide. The grid
   keeps one number per cell and side: 256 is one man. The fallen are not deposited.
2. **Steps.** Each man out of his place reads the field and decides where he would go (below).
3. **Exact positions.** Men near an enemy are moved apart wherever two enemies would stand
   closer than a guard's distance (600 mm).
4. **Move, turn, crush.** Positions and velocities are written. A man turns toward his target's
   facing at a limited rate. A man in a crush draws for being disabled.

Between man steps nothing moves. The viewer interpolates.

## How a man out of his place decides his step

**He walks more slowly the thicker the crowd ahead.** The substrate reads the density one cell
ahead of him along his way (or at his target, if that is nearer) and scales his gait by
Weidmann's curve: full speed in the open, nothing at the jam density (5.4 men per m²). The
curve is a table of 129 integers.

**Enemies ahead count for more than friends.** One enemy on his feet counts as much as it
takes to make an open line of them a wall: at 0.8 enemies per m² ahead, walking stops. This is
the rule "no one walks onto ground the enemy holds". He gets that ground only when they are
pushed off it, fall, or leave.

**A man who presses gains ground only where there is room.** If his target carries a press, he
edges forward along his facing at up to 450 mm/s. The gain falls to nothing as the density
ahead rises from the jam density to the packing density (9 per m²), and it falls to nothing on
ground the enemy holds.

**A crowd that is no longer pressed eases.** Above 2.5 men per m² around him, a man is pushed
from thicker ground toward thinner, at up to 300 mm/s.

**Density is bounded by these rules, not by a force.** Walking stops at the jam density and
pressing stops at the packing density, so no stiff pressure is needed, and none is used.

## Exact positions

A man out of his place is handled by position when there is any enemy presence in his own four
cells or in the four ahead of him.

- For each such man the substrate lists the enemies near enough to matter this step.
- The step is cut into passes short enough that no two men can cross within one (two to eight,
  and two more to settle).
- In each pass every man is moved clear of each listed enemy in turn, against where the others
  stood before the pass. So no man's place in the loop favours him.
- **Who gives way.** Of an overlap between two men, each gives in proportion to how fast he is
  closing. A man who stands is not shoved by one who walks into him. A man in his place is
  never moved: the other gives all of it.

### The three ways of touching

| `Contact` | Who is kept apart by position |
|---|---|
| `Line` | Enemies only |
| `Crowd` | Enemies; and men of one side a body's width apart (358 mm), where the crowd is thicker than 3 per m² and the man is going somewhere by his own will |
| `All` | Every man out of his place, with every man near him |

These are the three physics rungs of [06](06-experiment.md) §2. The passage study decides
between them.

## Crush

A man with more than 6.5 men per m² around him is being crushed. He is disabled at a rate that
rises with the density above that: a mean of 134 s at 2 per m² above the onset. The draw is on
the substrate's own random stream.

## What it sees: the perception

`FieldPerception` gives exactly the Views of the reference (`GridPerception`). The conformance
tests hold it to that on random crowds and through a fight and a rout.

- Each 2 m cell keeps counts of men by side and posture, and a few flags. The test for
  dormancy reads the flags of a man's 25 cells and nothing else.
- A View adds up the counts of those cells. It looks at single men only for the eight nearest
  enemies within reach and for crowding. Men are listed by quarter cell and side, and the
  search goes ring by ring outward from his own quarter, through quarters that hold enemies,
  until the eighth nearest is nearer than the rings already done.

## What differs from the proposal

| [02](02-order-and-contact.md) §5 said | As built |
|---|---|
| A soft pressure that is an equation of state; a density projection as fallback | No pressure holds men apart. Walking and pressing stop at set densities. The fallback was not needed with `Contact::Line`: the pocket peaks at 9.6-9.8 per m² against a packing density of 9. |
| "Both sides count, so two masses cannot pass through each other" | Not enough. Two crowds of a man per m² each are far below the jam together, and walked through each other. The held-ground rule was added for this. |
| Weidmann's curve "from memory, to check" | Checked against a secondary source that quotes the fit (v = 1.34 (1 − exp(−1.913 (1/ρ − 1/5.4))) m/s). The primary (Weidmann 1993) is not read. |
| Contact where men are crowded and moving, "a few passes a step", cheap because a packed interior hardly moves | Wrong for a pocket under pressure. See below. |
| Othismos as a parameter | Not built. A man in his place ignores his press. See "Left out". |

## Behaviour found and kept

Each of these is asserted in a unit test, except the second, which was seen in one and is
only allowed for there.

- **A line that stands is a wall,** even when its men are out of their places. Men who walk at
  it stop a guard short and do not shove it.
- **An end man opposite no one walks round the flank.**
- **Two single ranks that both push, each set against the other's gaps, buckle and slip
  past.** The guard is still kept to within 3 %. Deep masses do not pass.
- **A runner cannot hop over a formed line.** The deepest he gets is the notch between two
  files.
- **Friends make way.** A man out of his place passes through a formed body of his own side.
- **Under `Crowd` and `All` the guard gives under pressure from behind,** down to about a
  body's width. Men behind shove a man onto the enemy.
- **The grid leaves no mark under a shift by whole cells:** the same scene moved by a whole
  number of cells gives the same bits. A shift by part of a cell changes the bits, as it must
  with any grid; how much it changes the outcome is not measured yet.

## What it does badly

1. **`Contact::Crowd` fails in the pocket.** 60,000 to 90,000 men are crowded and pressing, so
   all of them are in contact. It then costs about what the pairwise engine costs for them
   (1.6x real time on one core in a burst), and it breaks the field's own bound: the densest
   cell rose to 12.9 per m². Men moved by position are pushed into ground the field would not
   let them walk onto. This is not fixed. My present view is that contact belongs where the
   ground is narrow, near walls, which is where a grid of metre cells is known to be wrong. The
   passage study is the place to decide it.
2. **A man wedged between two enemies who are themselves wedged** can be left up to 3 % inside
   a guard after the passes. Everywhere else the guard is exact to 5 mm.
3. **A pressed pair wanders.** Two enemies pressing on each other drift about a cell over
   minutes, because each reads the field at a different point ahead.
4. **How firmly one thin rank holds ground depends on where it stands on the grid.** One rank
   at a metre a file reads as 0.5 to 1.0 enemies per m² along its own line, according to
   whether it stands between two rows of cells or on one. Against the threshold of 0.8 that is
   the difference between slowing a walker to a quarter of his pace and stopping him. The
   guard stops him in either case, so no one passes; what changes is how fast he arrives.
5. **Men of one side overlap** under `Contact::Line`. In a jam of 900 men, 454 pairs stood
   closer than three quarters of a body's width. The density is right; the picture is not. It
   matters only to the viewer and to any measure that counts bodies in a gap.

## Left out

- **Walls.** The shared `Wall` type exists (`cannae-core`); the field does not read it yet.
  Needed for the passage study.
- **Othismos.** A man in his place moves by his tie and ignores his press, so a formed line
  does not shove. For a formed body the push would have to act on the body: its frame gives
  ground under the enemy's press. That is not built, and whether to build it is an open
  question on the call.
- **Fast bodies.** A man moves at most once per man step. A horse at a gallop covers 3 m in
  that time and needs shorter steps.
- **Formed blocks deposited at once.** Every man is deposited singly. It costs time and
  changes nothing.
- **Which way a crush pushes.** The crush hazard reads density only.
- **Terrain.** Slope, rivers and ditches do not exist yet in either engine.

## Two corrections to the commit message of e54c427

That message cannot be amended, so the corrections are here.

1. It said the time in the pocket went to the position solve at the interface. I had not
   profiled it. Profiled: walking steps took 32 %, the neighbour lists 25 %, moving and crush
   draws 20 %, the passes 11 %, the deposit 8 %. A man at the interface was moved clear of an
   enemy 0.17 times per pass.
2. It said that averaging left a wedged man 250 mm inside an enemy's guard. That was
   arithmetic done before any run. The averaging form was never measured.

For the same reason I withdrew, on the call, a remedy I had proposed for the pocket (a guard
that gives way as the press rises). The cost it was meant to remove was not there.
