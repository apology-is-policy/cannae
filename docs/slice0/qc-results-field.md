# Slice 0: the visual QC pass, the field engine's list

**Status:** Varro, 2026-11-11. All 95 flags the rule picks on the field engine are binned below.
**One defect, in my own field code** (four flags). 41 flags are fine and 50 are gaps.
The rules of the pass are in [qc-pass.md](qc-pass.md), and the pairwise engine's list is in
[qc-results.md](qc-results.md).

**Closed on 2026-11-11.** Paullus read the list and would move no bin; he checked D1 against
the text and the code and agrees that it is a defect (call 0001, turn 95). He did not look at
my pictures. Michal decided at about 15:40 UTC, on both our recommendations, that D1 is
recorded and left in: the re-frozen code does not fix it (preregistration, Addendum 4). I read
Paullus's 75 rows and agree with every bin. Below, "candidate defect" is how I wrote it before
those two steps.

**Corrected on 2026-11-11, after the first commit of this list (18e4015):** gap Q is withdrawn.
I had listed it without tracing it and pointed at Paullus's bout resolver. Replayed, its three
flags are the ordinary wait for a bout followed by gap G. The counts above do not change.
I then replayed the two other gaps I had listed as not traced, P and R. Both stay gaps. P is
larger than I had written, and it is on both engines.

**What I knew while judging.** On the frozen code the field engine fails the 10 % rule on
`base` at rungs 0 to 3, and it has failed its speed line
([spike-field.md](spike-field.md)). The candidate defect is in that engine and was found by its
owner, so read my recommendation on it with that in mind.

**The order of things.** I formed every bin before I read Paullus's list or his note 60. I
wrote this document after reading both, and I use his letters for the gaps we share. Where we
found the same thing separately, the table says "also Paullus's".

## Verdict

- **One candidate defect, D1:** two enemies on their feet come nearer than the guard when
  neither of them is out of his place. 123 flags in 27 of the 160 battles, none in `base`. It
  needs Paullus's agreement, and Michal decides. I recommend that it does not go into the
  re-freeze; the reasons are under D1.
- **The fix for the step back works on this engine too.** No flag of detectors 1 and 25 has
  slots moving a Roman rank back under an order to hold. The steps back that remain are 650 mm
  (gap A), and re-forms 7 m or more off where a standard-bearer fled and stood again.
- **Both `stalled` flags are fine.** The warband was halted at its standoff before one standing
  Roman, who fell in the last quarter second of the detector's window (gap M).
- **The closer never failed to succeed.** In every `leaderless` case I replayed, he fled before
  his five seconds were up.
- **Nine detectors found nothing on this engine:** 8, 11, 12, 23, 24 and 27, as on the pairwise
  engine, and both dormancy checks pass in all 160 battles.

## How I looked

- **The scan** is Paullus's: `cannae-qc` on the candidate (9d96089), 160 field battles,
  100,579 flags, copied to the Mac with the pictures.
- **Pictures:** I looked at 76 of the 95. Eleven flags are marked "=": their picture is the
  same drawing as another flag's, compared file to file with the variant, rung and seed taken
  out. Eight of those I did not look at separately. Two I
  judged by a replay of the battle without its picture. Nine I judged from their notes alone;
  they are marked "note", and each is the next flag of a kind in a battle I had looked at.
- **Replays:** `cannae-qc clip`, built on the Mac from the candidate at 9d96089, for eight
  moments. They are named in the table.
- **Every flag of detectors 1, 4, 6, 7, 19, 25 and 26,** not only the picks, sorted by what
  its note says: order, distance, body, and whether the standard changed hands.

## D1, a candidate defect: enemies nearer than the guard

**What the text says.**

- `crates/cannae-field/src/field.rs`, the module's third step: "Any free man with an enemy
  near is handled by positions: no two enemies on their feet may stand closer than a guard's
  distance."
- `crates/cannae-field/src/lib.rs`: "enemies are kept apart by their exact positions, never by
  cells".
- [priors-field.md](priors-field.md), F6: "Guard: the nearest two enemies on their feet can
  stand, centre to centre", 600 mm in the nominal set.

**What the code does.** The pass over exact positions runs over free men only. A man is free
when the frames last saw him out of his place. A pair of enemies of whom neither is free is not
looked at, so nothing keeps the two apart. A man's place is wide: within 600 mm of his slot in
a century, within 1,200 mm in a warband.

**What was seen.** 123 flags of detector 4, which asks for less than 300 mm, half the guard.
The nearest pair stood 11 mm apart. 119 of them last one man step.

| Variant | Flags | Battles |
|---|---|---|
| `shock` | 108 | 20 |
| `veterans` | 12 | 5 |
| `flank` | 3 | 2 |
| the other five | 0 | 0 |

**Three replays, two ways it happens.**

- **A slot that lies on an enemy** (`veterans`, rung 0, seed 5, 549 s). A Gaul, steady, walks
  to his slot. It lies 200 mm from a Roman who stands steady in his own slot. They are 783,
  470 and then 200 mm apart, both in their places. At the next step the frame has moved, the
  Gaul is out of his place, and he stands 601 mm off.
- **The first step of a flight** (`veterans`, rung 0, seed 1, 50.5 s; `shock`, rung 0, seed 1,
  66.5 s). A Roman turns and runs a metre into a Gaul: 149 mm and 109 mm. For that one step he
  still counts as in his place, because the frames looked before he ran.

**How the frames come to overlap.** By gap A, when a warband's frame re-forms on a chieftain
who has gone into the Roman line. And by a charge: its frame halts when the fronts are within
the contact distance, which it finds a run step late.

**Is it a defect?** I think so: F6 and the sentence in `field.rs` state a rule without
exception, and the code keeps it only where one of the two men is free. The other reading is
in the same paragraph: the rule is given for "any free man", and "a man in his place moves by
his tie to his slot, and nothing else". Paullus judges which reading holds.

**What is not counted.** Pairs between 300 and 600 mm apart. The detector's threshold was fixed
before the scan and is not changed.

**My recommendation: record it, and leave it out of the re-freeze.**

- I know this engine's outcome on `base`. A change to its contact code written by me now is
  what the pre-registration exists to prevent.
- The engine has failed its speed line.
- In the nominal set it does not occur in `base`, which the gate reads.
- The report names it as a known defect of the field engine in the scored runs.

## The gaps

A to M are Paullus's, from [qc-results.md](qc-results.md); only those the field's pictures
show are repeated. N, O, P and R are new here; Q is withdrawn.

| | What looks wrong on the field engine | What the code says | Best picture |
|---|---|---|---|
| **A** | **A frame follows its standard-bearer wherever his own posture takes him.** A century of 72 walks back 15.6 m in twelve seconds, steady, behind a standard-bearer who gives ground (`veterans`, rung 0, seed 1, from 54.5 s). A warband's 157 slots jump 2 m every half second behind a chieftain who runs on in pursuit. A frame re-forms 1.3 km and 1.6 km off where a standard-bearer who fled stood again, and a steady man walks toward it for seventeen minutes | `frames.rs`: "a body whose standard-bearer steps out follows its standard". On this engine the walk back is longer than on the pairwise one (24 steps in a row against 5), because here he walks back through his own ranks | `01-slot-jump-field-other-3`, `25-slot-back-field-other-2` |
| **B** | The standard-bearer moves up into the front rank and fights there. 212 times in 130 battles | Files close over the fallen | `15-standard-front-field-front-rank-1` |
| **D** | A man runs half a second after he was steady. 16,876 times | Candidate 8 in [STATUS](../STATUS.md) | `28-sudden-flight-field-1` |
| **E** | A runner runs straight to the rear whoever is there. In `shock` the Roman rear rank flees into the band that attacks it | `cannae-man`: "Turned, he runs the way his body's back was" | `03-pinned-runner-field-4`, `10-flight-toward-field-1` |
| **G** | Pursuit never ends. The chieftain is among the runners 300 m from the field at 115 s | No rule ends a pursuit | `20-far-field-standing-1` |
| **I** | A strike passes through a comrade. Here the comrade often stands on the same spot | `bouts.rs`, `opponent`; and men of one side are not solid on this engine | `13-bout-through-field-past-a-comrade-1` |
| **K** | A file that is lost is never closed from the side. One survivor stands at his slot beside the warband for a minute | Files close forward only | `18-gap-field-right-1` |
| **M** | A body halts at its standoff before a single man. 160 Gauls stand for 67 s before one Roman standard-bearer, 1.3 km from the field, while one of them fights him | `frames.rs`, `gap_ahead` | `26-stalled-field-1` |
| **N** | **Pursuers stand on one another.** 24 to 30 Gauls stand in one 2 m cell around one runner | Men of one side are not solid on this engine. Detector 5 counts it: 50,089 flags of two comrades nearer than 300 mm | `06-crowd-field-1` |
| **O** | **A man at the edge of his alarm goes from steady to back and again, six times in five seconds.** When he carries the standard, the century steps back with him each time | Giving ground and standing again are two thresholds with a small margin between them | `14-flicker-field-gives-way-1` |
| **P** | **A body ordered to retire walks off for good, and its men never regain their slots.** The rear band of `shock` is ordered off at 180 s with 29 of its 36 men out of their places in pursuit. The frame walks back at 1.3 m/s to the run's end, 550 m in seven minutes, and the men walk after their slots at the same pace: from 190 s to 599 s the median man is 25 m from his slot and one is 156 m from it, 35 of them steady. More than ten of the 36 are flagged out of place from the order to the run's end in 15 of the 20 `shock` battles on this engine, and in 5 of the 20 on the pairwise one | Traced by replay (rung 0, seed 1). `frames.rs`: a man "has a slot in the frame and walks to it", and "a retirement leads backwards". No order ends a retirement | `16-out-of-place-field-4` |
| **Q** | *Withdrawn.* I had written: "A man stands forward at the enemy's line for one to two minutes with no bout", 91 s and 131 s in the two picks, not traced. Replayed (`base`, rung 0, seed 1, men 170 and 180), each man's time is two things already on the lists: 33 to 35 s at the Roman line with a standing Roman a metre ahead and no bout, and then 55 s and 94 s forward behind a runner who is never nearer than 1.75 m | The first is the ordinary tail of a bout rate of one in ten seconds (`bouts.rs`, `rate_q16`), as in Paullus's `17` rows, which are 24 to 34 s. The second is gap G: a man's reach is 1.5 m, and pursuer and runner move at the same 4 m/s. The bout resolver does what its text says | `17-forward-idle-field-1` |
| **R** | **The second line runs within seconds of the first, untouched.** Beside the flag, in one battle (`backed`, rung 0, seed 4), by replay: 19 men of the first line are in flight at 74 s and 101 at 76 s. At 76 s both centuries of the second line, 24 m behind, are steady to a man. One of them has 75 of 81 steady at 77 s, 6 at 79 s, and every man in flight at 81.5 s, and no enemy came nearer to it than 9 m. The other is all in flight at 82.5 s, with no enemy nearer than 5.5 m | `cannae-man`, `seen`: the men of his own side in flight in his view count toward a man's alarm, up to eight of them, whichever body they belong to. The first line's runners come through the second. I did not isolate which term of the alarm carried it | none |

## Every field flag looked at

"=" marks a picture that is the same drawing as the one named. "note" marks a flag judged from
its note. "clip" marks a replay.

| Flag | Where | Bin | Reason |
|---|---|---|---|
| `01` by-the-lead 1, 2, 3 | base r0 s2, 190 s, 198.25 s, 214.5 s | fine | The warband's lead comes on as the century ahead gives way |
| `01` by-the-lead 4 | shock r0 s1, 60 s | fine | The rear band is ordered to charge and its lead comes on |
| `01` other 1, 2, 3 | base r0 s1, 33 s, 34 s, 38.75 s | gap A | The halted warband's frame re-forms on its chieftain, who has stepped out more than 1,200 mm; at 38.75 s it moves 1.1 m sideways with him |
| `01` other 4 | veterans r0 s1, 44.25 s | gap A | The same. Clip of the same battle at 95 to 99 s: he runs 1,000 mm a step in pursuit and the frame re-forms every second step |
| `02` 1, 2, 3; 4 = 1 | base r0 s1, 37.5 s to 44.75 s | gap A | The men walk back to slots that stepped back 650 mm with their standard-bearer, and then the century breaks |
| `03` 1, 2, 3 | base r0 s1, 44.5 s, 45 s; base r0 s3, 86.5 s | fine | A runner ringed by two to five pursuers cannot pass them, and stands 2.5 to 4.25 s until he is cut down |
| `03` 4 | shock r0 s1, 66.75 s | gap E | A Roman of the rear rank flees into the rear band and is held among its men |
| `04` 1 | veterans r0 s1, 50.5 s | **defect D1** | Clip: a Roman's first step of flight ends 149 mm from a Gaul; both count as in their places |
| `04` 2 | veterans r0 s5, 549 s | **defect D1** | Clip: a Gaul walks to a slot that lies 200 mm from a standing Roman |
| `04` 3 | veterans r0 s5, 553 s | **defect D1** | The same place four seconds later, another pair at 276 mm for two steps. The warband's ranks stand inside the century's |
| `04` 4 | shock r0 s1, 66.5 s | **defect D1** | Clip: a Roman flees a metre into a charging Gaul, 109 mm |
| `05` both-standing 1, 2, 3, 4 | base r0 s1, 8.75 s, 12 s, 17.25 s; shock r0 s1, 60.25 s | fine | Men of one side are not solid on this engine; the list makes it a count |
| `05` one-in-flight 1, 2, 3; 4 = 1 | base r0 s1, 14.75 s, 16 s, 16.5 s | fine | A runner passes through the men behind him, as the engine is written |
| `06` 1, 2, 3 | veterans r1 s1, 113.5 s to 119.5 s | gap N | 24 to 30 pursuers in one cell around one runner. It is under the packing density of 9 per m² |
| `07` 1; 2 = 1; 3 | shock r0, r1, r3 s2, 78.5 s | fine | One event at three rungs. A runner in a knot of enemies is pushed a guard's distance clear in the step he runs a metre: 1,596 mm against 1,500 |
| `09` 1, 2, 3, 4 | base r2 s1, 60.5 s; veterans r3 s1, 49.5 s; flank r0 s1, 616.5 s; shock r0 s1, 67.25 s | fine | A Gaul who is forward faces the man he strikes. He counts as in his place because a warband's place is 1,200 mm wide |
| `10` 1, 2, 3 | base r0 s2, 175.25 s, 224.5 s, 225 s | gap E | Runners run on for a minute or two with a pursuer level with them or ahead |
| `10` 4 | shock r0 s1, 82.5 s | gap E | A Roman runs for 100 s behind a Gaul who chases others |
| `13` 1, 2; 3 = 1; 4 | base r0 s1, 36 s; base r0 s2, 194 s; veterans r0 s1, 52 s | gap I | A Gaul strikes with a comrade 46 to 147 mm from the line of the strike |
| `14` gives-way 1, 2 | veterans r1 s5, 580.25 s, 582.5 s | gap O | The first century's standard-bearer goes from steady to back and again, six times in five seconds |
| `14` steady-and-forward 1, 2, 3 | veterans r1 s5, 580.5 s to 583 s | fine | Two Gauls step out and stand again: the pulse of the fight, as the kind says |
| `15` 1 | base r0 s1, 29.25 s | gap B | The standard-bearer walks up into the front rank when his file closes. The frame stays: the fix at work |
| `15` 2 | base r0 s1, 31 s | gap B | The same in the warband |
| `15` 3, 4 | base r0 s2, 218.75 s; recruits r0 s1, 15.75 s | gap B | The same, in a century that is breaking |
| `16` 1 | base r0 s1, 16.5 s | fine | The closer walks 16 m through his century to the fallen leader's slot: the succession |
| `16` 2, 3 | base r0 s1, 40.75 s | gap A | Gauls stay steady and 2 m from their slots to the end: the frame runs on behind the chieftain |
| `16` 4 | shock r0 s1, 169.25 s | gap P | A Gaul who chased a runner follows his retiring band at its own pace |
| `17` 1; 4 = 1 | base r0 s1, 3.75 s to 95 s | gap G | By replay, man 170: from 5.5 s a standing Roman a metre ahead for 35 s and no bout (the tail, as `17` 2); from 40.5 s he is forward for 55 s behind runners who stay out of his reach |
| `17` 2 | base r0 s1, 5.75 s to 30.75 s | fine | 25 s: the ordinary tail of the bout rate, as on the pairwise engine |
| `17` 3 | base r0 s1, 7 s to 138.5 s | gap G | By replay, man 180: his last bout is at 7 s; 33 s more at the line with no other; from 44.25 s he is forward for 94 s behind a runner 1.8 to 2.6 m ahead at his own pace |
| `18` left 1, 2, 3; 4 (note) | base r0 s1, 42.5 s; base r0 s2, 189.25 s, 214.5 s; backed r0 s1 | gap K | A front-rank man stands beside a file that is empty to the end |
| `18` right 1, 2; 3, 4 (note) | base r0 s2, 216.25 s; veterans r0 s1, 103 s; veterans r1 s1; recruits r0 s1 | gap K | The same. In `veterans` the warband walks past 14 steady Romans 5 m off |
| `19` closer-in-flight 1; 2 = 1; 3 (note) | flank r0, r1 s1; flank r2 s3, 617.5 s | fine | The century is in flight, the closer with it. The rule gives a closer in flight no succession |
| `19` closer-standing 1 | base r2 s1, 11.25 s | fine | Clip: the leader falls at 11.25 s and the closer flees at 14.75 s, before his five seconds |
| `19` closer-standing 2 (clip); 3 = 2 | recruits r0, r1 s1, 11.25 s | fine | Clip of rungs 0 and 3: the same. Also `volleys` rungs 0 and 3: he flees 4.75 s after the fall |
| `20` 1; 2, 3 (note) | base r0 s1, 115 s | gap G | The chieftain and the men beside him, among the runners 300 m out |
| `20` 4 | flank r0 s1, 115.25 s | gap G | The same moment in a variant watched longer |
| `25` by-the-lead 1; 2 (note) | flank r2 s2, 608.25 s; flank r2 s5, 614.25 s | fine | The flank band's lead goes off as it reaches its contact distance |
| `25` by-the-lead 3 | shock r0 s1, 71.25 s | fine | The rear band's lead goes off |
| `25` other 1; 4 = 1; 5 = 1 | base, backed, relief r0 s1, 2.75 s | fine | The warband halts at its standoff: the lead goes off as its bearer walks his last step, 1,175 mm back |
| `25` other 2; 3 (note) | base r0 s1, 39.25 s, 39.75 s | gap A | The first century steps back 650 mm, twice in half a second |
| `26` 1; 2 = 1 | backed, relief r0 s4, 1,168.5 s | fine | The detector reads the enemy only at the end of its ten seconds. The warband stood at its standoff from 1,101.5 s; the Roman fell at 1,168.25 s; its lead came on at 1,168.75 s. It shows gap M. Also Paullus's |
| `28` 1, 2; 3 (note); 4 = 1 | base r0 s1, 38.25 s to 39 s | gap D | Romans of the front rank turn within half a second of standing steady |

## What the flags of detectors 1 and 25 are on this engine, by count

Sorted from the notes of all 3,201 flags of the two detectors. Paullus sorted the pairwise
engine's the same way and found the same three mechanisms.

| What moved | Flags | Mechanism |
|---|---|---|
| A century's slots 650 mm back under an order to hold | 1,449 of detector 25, and one of 722 mm | Gap A. 658 of them in a body of more than 50 men, 668 in one of five or fewer. Every one is 650 mm exactly and straight back: two steps of a standard-bearer who gives ground of himself. None is a shove |
| The warband's slots 1,175 mm back under an order to advance | 371 of detector 25 | A walking body halts |
| The warband's slots 1,200 to 3,500 mm, any direction | 219 of detector 1, 12 of detector 25 | Gap A: the frame re-forms on the chieftain |
| A century's slots one to three ranks ahead under an order to hold | 179 of each | Files closing together, mostly in a body of two to four men. The frame does not move |
| A century's slots 650 mm ahead, or once 920 mm back, under an order to hold | 10 of detector 25 | Gap A: the frame follows a standard-bearer who steps forward |
| Slots moving 1,500 mm | 503 of detector 1, 21 of detector 25 | The lead coming on or going off |
| A band's slots, any direction, under an order to charge or as it is ordered off | 91 of detector 1, 124 of detector 25 | A running body: it follows its bearer at every step, and halts 500 mm back when the lead goes off in a run step |
| A century's slots 7 m to 1.6 km | 21 | Gap A: the frame re-forms where a standard-bearer who fled stands again. The sideways 9,000 mm in several is his new file: he falls in at the rear of the shortest file, which is an empty one at the end of the line |

## Where the tool read badly on this engine

None changes a bin, and no threshold is touched.

- **Detector 4's 300 mm is half this engine's guard.** It counts the worst of D1 and not all
  of it.
- **Detectors 1 and 25 cannot tell the lead from the lead and a step.** A halt is 1,175 mm and
  the kind `by-the-lead` asks for 1,500. Also Paullus's.
- **Detector 9 reads "in his place" as the frames do,** so a warrior fighting within 1,200 mm
  of his slot is flagged for facing the man he strikes.
- **Detector 16 flags the closer on his way to the leader's slot.**
- **Detector 19's kind** names how the closer stood when the leader fell. Also Paullus's.
- **Detector 26** reads the enemy only at the end of its ten seconds. Also Paullus's.

## What came of it

- Paullus read this list and judged D1 a defect, as I had.
- Michal decided that D1 is recorded and left in. The report names it as a known defect of the
  field engine in the scored runs.
- The gaps N, O, P and R are on the second-round list in [STATUS](../STATUS.md), with gap A
  first on it by Michal's decision.
- The pictures of every flag, with these rows beside them, are in [qc/field.md](qc/field.md).
