# Slice 0: what the visual QC pass found

**Status:** Paullus, 2026-11-11. The pairwise engine's list is complete. Varro has read it and
agrees with every bin (call 0001, note 61). His list for the field engine is
[qc-results-field.md](qc-results-field.md). The rules of the pass, and Michal's decisions on
what it found, are in [qc-pass.md](qc-pass.md).

- **The page for Michal:** [qc/README.md](qc/README.md), one picture for each gap.
- **Every pairwise flag with its picture:** [qc/pairwise.md](qc/pairwise.md).

## Verdict so far

- **No defect on the pairwise engine.** Of the 75 flags the rule picks there, none shows the
  code contradicting its own text. 37 are fine and 38 are gaps: the code does what was written
  and it looks wrong.
- **The fix for the step back works.** On the candidate the standard-bearer walks up when his
  file closes, and the frame stays where it was (picture `15-standard-front-pairwise-front-rank-1`,
  at 29.25 s, the second Michal saw the century step back).
- **A smaller step back remains, and it is by design.** A holding century still steps back as
  one, by 0.65 m, when its standard-bearer himself gives ground (gap A below).
- **Six detectors found nothing on either engine:** 8 `down-moves`, 11 `bout-reach`,
  12 `bout-state`, 23 `slot-twice`, 24 `down-rises`, 27 `over-packed`. On the pairwise engine
  also 4, 5, 6 and 7. Both dormancy checks pass in all 320 battles.

## What was run

- **The code:** the re-freeze candidate, `refreeze-candidate` at 9d96089.
- **The battles:** 320 of the nominal set, on Michal's Raspberry Pi, in 1,560 s. `flank` was
  run again for 1,200 s ([qc-pass.md](qc-pass.md), the lines after the scan).
- **The control:** on the frozen code, pairwise, rung 0, `base`, seed 1, detector 1 flags the
  known step back at 29.25 s (70 of 77 slots moved 1,800 mm back).
- **The tool's tests** on the candidate tree: 38 detector tests and 6 same-battle tests pass.
- **Flags:** 151,258. **Picked by the rule:** 170, of them 75 on the pairwise engine.

## The counts

Flags by variant; in brackets, the battles with at least one. The nominal set is never scored.

**The pairwise engine, 160 battles.**

| # | Detector | base | veterans | recruits | flank | shock | volleys | backed | relief | All |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `slot-jump` | 79 | 54 | 63 | 157 | 442 | 86 | 128 | 136 | 1145 (143) |
| 2 | `halted-body-walks` | 55 | 55 | 40 | 69 | 36 | 48 | 104 | 104 | 511 (136) |
| 3 | `pinned-runner` | 127 | 141 | 116 | 130 | 157 | 172 | 218 | 218 | 1279 (151) |
| 9 | `facing-in-place` |  | 29 |  | 7 | 332 |  |  |  | 368 (25) |
| 10 | `flight-toward` | 139 | 80 | 156 | 228 | 1049 | 169 | 140 | 140 | 2101 (130) |
| 13 | `bout-through` | 44 | 278 | 4 | 62 | 70 | 3 | 44 | 44 | 549 (53) |
| 14 | `flicker` | 1 | 8 |  | 1 |  | 2 | 1 | 1 | 14 (9) |
| 15 | `standard-front` | 33 | 26 | 19 | 37 | 18 | 22 | 42 | 42 | 239 (121) |
| 16 | `out-of-place` | 516 | 178 | 230 | 912 | 821 | 251 | 651 | 651 | 4210 (147) |
| 17 | `forward-idle` | 388 | 644 | 268 | 1174 | 1118 | 236 | 397 | 397 | 4622 (156) |
| 18 | `gap` | 16 | 60 | 6 | 25 | 21 | 14 | 26 | 26 | 194 (87) |
| 19 | `leaderless` | 1 |  | 3 | 1 | 1 | 3 | 1 | 1 | 11 (11) |
| 20 | `far` | 2317 | 1030 | 3117 | 3680 | 897 | 134 | 2327 | 2651 | 16153 (145) |
| 25 | `slot-back` | 111 | 223 | 103 | 246 | 438 | 92 | 195 | 199 | 1607 (160) |
| 26 | `stalled` |  |  |  |  |  |  | 1 | 1 | 2 (2) |
| 28 | `sudden-flight` | 1868 | 1262 | 1963 | 2031 | 1655 | 1507 | 3694 | 3694 | 17674 (152) |

**The field engine, 160 battles.**

| # | Detector | base | veterans | recruits | flank | shock | volleys | backed | relief | All |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | `slot-jump` | 75 | 151 | 90 | 152 | 247 | 76 | 109 | 113 | 1013 (144) |
| 2 | `halted-body-walks` | 56 | 68 | 60 | 75 | 40 | 46 | 116 | 116 | 577 (144) |
| 3 | `pinned-runner` | 12 | 12 | 3 | 79 | 216 | 8 | 12 | 12 | 354 (79) |
| 4 | `overlap-enemies` |  | 12 |  | 3 | 108 |  |  |  | 123 (27) |
| 5 | `overlap-own` | 4523 | 8036 | 3870 | 7533 | 8423 | 1782 | 7961 | 7961 | 50089 (160) |
| 6 | `crowd` |  | 3 |  |  |  |  |  |  | 3 (1) |
| 7 | `jump` |  |  |  |  | 3 |  |  |  | 3 (3) |
| 9 | `facing-in-place` | 1 | 1 |  | 18 | 154 |  | 1 | 1 | 176 (34) |
| 10 | `flight-toward` | 200 | 132 | 201 | 278 | 644 | 126 | 201 | 201 | 1983 (128) |
| 13 | `bout-through` | 5 | 109 |  | 50 | 66 |  | 5 | 5 | 240 (53) |
| 14 | `flicker` |  | 6 |  | 2 | 2 |  |  |  | 10 (4) |
| 15 | `standard-front` | 28 | 25 | 33 | 30 | 17 | 15 | 32 | 32 | 212 (130) |
| 16 | `out-of-place` | 741 | 696 | 385 | 844 | 861 | 364 | 732 | 732 | 5355 (156) |
| 17 | `forward-idle` | 456 | 903 | 417 | 1249 | 1119 | 327 | 456 | 456 | 5383 (156) |
| 18 | `gap` | 25 | 15 | 35 | 33 | 38 | 26 | 37 | 37 | 246 (128) |
| 19 | `leaderless` | 1 |  | 4 | 8 | 4 | 3 | 1 | 1 | 22 (22) |
| 20 | `far` | 2444 | 799 | 2719 | 3805 | 753 | 140 | 2451 | 2613 | 15724 (144) |
| 25 | `slot-back` | 92 | 440 | 132 | 128 | 265 | 107 | 511 | 513 | 2188 (160) |
| 26 | `stalled` |  |  |  |  |  |  | 1 | 1 | 2 (2) |
| 28 | `sudden-flight` | 1771 | 887 | 2026 | 1933 | 1386 | 1443 | 3715 | 3715 | 16876 (150) |

## The gaps the pairwise pictures show

Each is named once here and referred to by its letter in the list below. "Beside the flag"
means the picture shows it although no detector was written for it.

| | What looks wrong | What the code says | Best picture |
|---|---|---|---|
| **A** | **A body's frame follows its standard-bearer wherever his own posture takes him.** A holding century steps back 0.65 m as one, unengaged files included, when he gives ground. A halted warband's 158 slots jump 1.3 m forward, twice in two seconds, when he steps out to fight. When he flees and stops again, the frame re-forms where he stands, 16 m off, and the men still steady walk out of the line to it | `frames.rs`, `anchor`: "A halted frame stays put until he is out of his place in it." Nothing tells a bearer who is driven from one who goes | `25-slot-back-pairwise-other-2`, `01-slot-jump-pairwise-other-1`, `15-standard-front-pairwise-front-rank-3` |
| **B** | **The standard-bearer moves up into the front rank and fights there.** He starts in the second rank. When the man ahead of him falls, the file closes and he becomes a front-rank fighter; a dozen Gauls gather round him. Gap A starts here | Files close over the fallen, and the bearer is a man of his file like any other. The list made it a count: 239 times in 121 of 160 battles | `15-standard-front-pairwise-front-rank-1`, `-2` |
| **C** | **A man in flight in the front rank cannot get through his own files.** He stays there, in flight, for up to 21 s. The man behind him stands a rank short of his slot for as long, because the runner is in it | Known: Michal's second fault, a second-round candidate in [STATUS](../STATUS.md). 1,279 times, in 151 of 160 battles | `03-pinned-runner-pairwise-1`, `16-out-of-place-pairwise-1` |
| **D** | **A man runs half a second after he was steady** | Known: Michal's third fault, candidate 8. 17,674 times on this engine, 16,876 on the field | `28-sudden-flight-pairwise-1` |
| **E** | **A runner runs straight to the rear whoever is there, and a pursuer ignores the runner behind him.** A fleeing Roman runs for 82 s 2.4 m behind a Gaul who chases others at the same pace | Nothing in the man's flight reads where the enemy is, and a pursuer strikes only at what is ahead of him | `10-flight-toward-pairwise-1` |
| **F** | **Runners keep their file lines.** A broken century runs off in rows of five to nine men, nose to tail at half a metre, and is still in rows 100 s later | They all run the same way at the same pace, and bodies are solid | `10-flight-toward-pairwise-2`, `20-far-pairwise-standing-1` |
| **G** | **Pursuit never ends.** Pursuers and runners move at the same 4 m/s. Nobody is caught and nobody gives up: ten Gauls are still among the runners 300 m from the field, and go on | No rule ends a pursuit. The scored runs stop at the rout's end, so no score sees it | `20-far-pairwise-standing-1` |
| **H** | **The two fronts pack into one dense band.** After two to three minutes each side's front three ranks have gone forward and stand two or three deep at about half a metre, along 25 m. The Romans' 1.8 m a man is gone. Seen in `base` and `veterans`, seed 2 | Any man may step forward, and nothing keeps fighting room. No density detector fires: it stays under 6 men per m² | `09-facing-in-place-pairwise-1`, `14-flicker-pairwise-gives-way-1` |
| **I** | **A strike passes through a comrade.** A man strikes an enemy 1.4 m off with one of his own standing on the line between them | `bouts.rs`, `opponent`: the nearest enemy within reach and not behind him. Nothing asks whether the way is clear. 549 times, half of them in the band of H | `13-bout-through-pairwise-past-a-comrade-1` |
| **J** | **Men attacked from behind rarely face about.** In `shock`, 25 s after the rear band arrives, 29 Romans are within 3 m of a band man: 20 are in flight, and of the 9 who stand 7 have their backs or sides to him. The rear ranks stay steady, facing the front, with the band behind them | Beside the flag. I counted it for one battle (pairwise, rung 0, seed 1) | `25-slot-back-pairwise-other-6` |
| **K** | **A file that is lost is never closed from the side.** The frame keeps the empty column to the end. The 30 men left of a century stand in a staircase; one survivor stands alone at his slot for nine minutes | Files close forward only. A loose file closes after 10 s, a drilled one after 2 | `18-gap-pairwise-left-2`, `-4` |
| **L** | **After one century breaks, the warband and the other century stand corner to corner, steady, to the end of the ten minutes.** Neither turns on the other | Beside the flag. Slice 0 has no command above the body, and a man steps forward only at what is ahead of him | `18-gap-pairwise-left-2` |
| **M** | **A body halts at its standoff before a single man.** 138 Gauls wait 4 m from one Roman standard-bearer who stands alone, and send men out one at a time | `frames.rs`, `gap_ahead`: the nearest enemy who stands within the body's frontage | `26-stalled-pairwise-1` |

## Every pairwise flag looked at

The id names the picture; all of them are in [qc/pairwise.md](qc/pairwise.md). "=" marks a flag
whose picture is the same drawing as another's, compared file to file: the same moment in a
variant that shares its start with `base`.

| Flag | Where | Bin | Reason |
|---|---|---|---|
| `01` by-the-lead 1 | base r0 s1, 40.5 s | fine | The warband's lead comes on as the century ahead breaks |
| `01` by-the-lead 2 | base r0 s1, 48.75 s | fine | The same, a second time |
| `01` by-the-lead 3 | base r0 s4, 98 s | fine | The same, another seed |
| `01` by-the-lead 4 | shock r0 s1, 60 s | fine | The rear band's lead comes on with its order to charge |
| `01` other 1 | base r0 s1, 42.5 s | gap A | The halted warband's frame is re-formed on its bearer, who stepped out to fight: 158 slots move 1.3 m |
| `01` other 2 | base r0 s1, 43.5 s | gap A | The same, a second later |
| `01` other 3 | base r0 s1, 46.75 s | fine | 1 of 1 slots: the last slotted man of a broken century, his file closing |
| `01` other 4 | shock r0 s1, 72.5 s | gap A | The charging band's frame slides 1.2 m in a step with its bearer; its men stand in knots 2 to 5 m from their slots |
| `02` 1 | base r0 s1, 39.25 to 43.25 s | gap A | The first century steps back 0.65 m after its bearer gave ground, then breaks |
| `02` 2 | base r0 s1, 41.75 to 44.5 s | fine | 2 of 3 standing men: the remnant of a broken century |
| `02` 3 | base r0 s1, 50.5 to 65.75 s | gap A | The second century's 12 standing men walk 16 m to a frame re-formed on their bearer, who fled and stood again |
| `02` 4 | backed r0 s1, 39.25 s | gap A | = `02` 1 |
| `03` 1 | base r0 s1, 13 to 28.75 s, man 8 | gap C | A front-rank Roman in flight for 15.75 s between his own file and the Gauls |
| `03` 2 | base r0 s1, 19.25 to 40.5 s, man 82 | gap C | The same, 21 s |
| `03` 3 | base r0 s1, 38 to 41.75 s, man 14 | gap C | The same, until the century breaks |
| `03` 4 | backed r0 s1, 13 s, man 8 | gap C | = `03` 1 |
| `09` 1 | veterans r0 s2, 190 to 197 s, man 162 | fine | A Gaul at the warband's corner faces the Romans who came round it. Beside it: H |
| `09` 2 | veterans r0 s2, 198 to 200.5 s, man 86 | fine | A Roman at the band's end faces the enemy beside him |
| `09` 3 | veterans r0 s2, 198.5 to 200.75 s, man 237 | fine | The same, a Gaul |
| `09` 4 | shock r0 s1, 67.25 to 68.75 s, man 329 | fine | A man of the charging band, near a slot of a frame that moves with its bearer, fights facing sideways. All 332 such flags in `shock` are the band's men |
| `10` 1 | base r0 s1, 59.75 to 142.25 s, men 89 and 204 | gap E | A runner 2.4 m behind a pursuer for 82 s |
| `10` 2 | base r0 s1, 81.5 to 140.25 s, men 83 and 172 | gap E | The same, 59 s. Beside it: F |
| `10` 3 | base r0 s1, 140.75 to 144.75 s, men 2 and 198 | gap E | The same |
| `10` 4 | shock r0 s1, 78.75 to 180.75 s, men 155 and 355 | gap E | The same behind a man of the band, 102 s |
| `13` 1 | base r0 s1, 50 s | gap I | A strike over 1.46 m with a comrade 0.18 m from the line, in the knot round the standard-bearer |
| `13` 2 | base r0 s2, 176 s | gap I | The same, in the band of H |
| `13` 3 | base r0 s2, 176.25 s | gap I | The same, the comrade 9 mm from the line |
| `13` 4 | veterans r0 s2, 176 s | gap I | The same men and second as `13` 2 |
| `14` gives-way 1 | veterans r0 s2, 192 to 193.75 s, man 16 | fine | A Roman in the band: steady, forward, steady, back, steady, forward in 1.75 s. Rare: 14 flickers in 160 battles |
| `14` gives-way 2 | veterans r1 s2, 192 s | fine | = gives-way 1 |
| `14` steady-and-forward 1 | base r2 s5, 34 to 34.5 s, man 184 | fine | A front-rank Gaul toggles after the Romans have fled. Rare |
| `14` steady-and-forward 2 | veterans r0 s2, 210 to 212.5 s, man 113 | fine | A Roman in the band. Rare |
| `14` steady-and-forward 3 | veterans r0 s2, 234.75 to 237.25 s, man 16 | fine | The same |
| `15` 1 | base r0 s1, 29.25 to 40.75 s, man 15 | gap B | The bearer walks up into the front rank (the frame stays: the fix works) and fights there for 11.5 s |
| `15` 2 | base r0 s1, 46 to 48.25 s, man 96 | gap B | The second century's bearer in the front rank at its corner; a dozen Gauls gather round him |
| `15` 3 | base r0 s1, 52 to 55 s, man 96 | gap A | He fled 5 m and stood again: the frame snaps to him, and the 14 men left get slots more than 10 m away |
| `15` 4 | backed r0 s1, 29.25 s | gap B | = `15` 1 |
| `16` 1 | base r0 s1, 15.25 to 29.75 s, man 18 | gap C | Steady, 1.85 m short of his slot for 14.5 s: pinned runner 8 stands in it |
| `16` 2 | base r0 s1, 16.5 to 31.25 s, man 80 | fine | The closer takes the fallen leader's place, as the rule says, and needs 14.75 s to walk 16.5 m through his own files |
| `16` 3 | base r0 s1, 21.25 to 40 s, man 92 | gap C | As `16` 1, behind pinned runner 82 |
| `16` 4 | flank r0 s1, 15.25 s | gap C | = `16` 1 |
| `17` 1 | base r0 s1, 3.5 to 36.5 s, man 173 | fine | Bouts come once in ten seconds in the nominal set, so one forward man in seven waits 20 s. My threshold caught the ordinary |
| `17` 2 | base r0 s1, 4.5 to 38.25 s, man 167 | fine | The same |
| `17` 3 | base r0 s1, 7 to 30.75 s, man 170 | fine | The same |
| `17` 4 | flank r0 s1, 3.5 s | fine | = `17` 1 |
| `18` left 1 | base r0 s1, 54.75 s to the end, man 90 | gap K | Two men walk to the frame of gap A and stand there alone to the end |
| `18` left 2 | base r0 s3, 61.75 s to the end, man 86 | gap K | The 30 men left stand in a staircase. Beside it: L |
| `18` left 3 | base r0 s5, 31.25 s to the end, man 84 | gap K | The same, 25 men |
| `18` left 4 | veterans r0 s1, 60 s to the end, man 90 | gap K | One man stands alone at his slot for nine minutes |
| `18` right 1 | base r0 s2, 410.5 to 436.5 s, man 15 | fine | A runner stands again, alone, for 26 s: a lone man has no file beside him |
| `18` right 2 | base r0 s2, 436.75 to 448.5 s, man 19 | fine | The same, 12 s |
| `18` right 3 | base r0 s2, 448.75 to 463.25 s, man 16 | fine | The same, 14.5 s |
| `18` right 4 | veterans r0 s2, 184.75 to 197.5 s, man 219 | fine | A warband file closes after 10 s, and the detector asks for more than 10: 12.75 s |
| `19` 1 | base r2 s1, 11.25 s to the end | fine | The leader fell at 11.25 s and the closer was in flight by 15.25 s, before the five seconds were up. The century was gone by 26 s |
| `19` 2 | recruits r0 s4, 66 s to the end | fine | The same: the closer fled 3.5 s after the leader fell |
| `19` 3 | recruits r1 s4, 66 s | fine | = `19` 2 |
| `20` 1 | base r0 s1, 115.5 to 220.25 s, man 171 | gap G | A Gaul among the runners 300 m from the field, and still there 105 s later |
| `20` 2 | base r0 s1, 115.75 s, man 172 | gap G | The man beside him |
| `20` 3 | base r0 s1, 116 s, man 173 | gap G | The next |
| `20` 4 | flank r0 s1, 115.75 s, man 171 | gap G | The same scene as `20` 1 |
| `25` by-the-lead 1 | shock r0 s5, 68.75 s | fine | The band's lead goes off at contact |
| `25` by-the-lead 2 | shock r1 s5, 68.75 s | fine | = by-the-lead 1 |
| `25` by-the-lead 3 | shock r2 s4, 78.5 s | fine | The same |
| `25` other 1 | base r0 s1, 2.75 s | fine | The warband halts at its standoff: the lead goes off (1,500 mm back) while the bearer walks a step (325 mm on) |
| `25` other 2 | base r0 s1, 40.75 s | gap A | The first century's 73 slots move 0.65 m back at one step; half a second later every man stands on them |
| `25` other 3 | base r0 s1, 41.25 s | fine | The warband's lead goes off three steps after it came on |
| `25` other 4 | backed r0 s1, 2.75 s | fine | = other 1 |
| `25` other 5 | relief r0 s1, 2.75 s | fine | = other 1 |
| `25` other 6 | shock r0 s1, 66.75 s | fine | A frame on the march follows its bearer, who is checked at the Roman rear rank |
| `26` 1 | backed r0 s2, 1,648.75 to 1,658.75 s | fine | The warband stood at its standoff before one Roman, who fell at the window's end; the detector reads the enemy only then. Beside it: M |
| `26` 2 | relief r0 s2, 1,648.75 s | fine | = `26` 1 |
| `28` 1 | base r0 s1, 19 s, man 82 | gap D | Steady at 18.5 s, giving ground at 18.75, in flight at 19 |
| `28` 2 | base r0 s1, 38.75 s, man 28 | gap D | The same |
| `28` 3 | base r0 s1, 39.25 s, man 38 | gap D | The same, a man of the third rank |
| `28` 4 | backed r0 s1, 19 s | gap D | = `28` 1 |

## What the flags of detectors 1 and 25 are, by count

I traced the standard-bearer step by step in three battles and then sorted all 2,752 pairwise
flags of the two detectors by order, distance and body size. Three mechanisms cover the large
classes:

- **The lead coming on or going off** (`frames.rs`, `anchor`): 1,500 mm exactly when the frame
  was halted, and 1,175 mm back when a walking body halts.
- **The frame following its bearer** (gap A): at every step on the march, and when he is more
  than the tolerance from his place while it is halted (600 mm for a century, 1,200 mm for a
  warband). On the pairwise engine 420 flags are a holding century's step back of 650 mm.
- **Files closing together:** when a rank falls or flees as one, its files close at the same
  step two seconds later, and half the body's slots move up a rank (1,800 mm ahead under an
  order to hold: 220 flags; two ranks: 42). The frame does not move. The picture I checked it
  on is `base`, rung 0, seed 3 at 57 s.

**No flag is the old defect,** on either engine: under an order to hold no flag of the two
detectors has slots moving a Roman rank back (1,700 to 1,900 mm), and under an order to
advance every flag between 1,100 and 1,300 mm back is 1,175 exactly, the halt above.

## Where the tool read badly

None changes a bin. The thresholds are not touched during the pass.

- **Detector 1 cannot tell files closing together from a frame that moved.** It asks for half
  the slots moving alike.
- **Detector 25's kind `by-the-lead` misses a walking body that halts** by the bearer's step
  (1,175 mm, not 1,500): 310 of its 1,607 pairwise flags.
- **Detector 17's 20 s is the ordinary tail** of a bout rate of one in ten seconds.
- **Detector 19's kind** names how the closer stood when the leader fell. In all four distinct
  cases on this engine he was in flight within five seconds.
- **Detector 26 reads the enemy only at the end of its ten seconds.**
- **My list watched `flank` for 600 s,** the second its band is ordered in. Found from the
  counts and run again.

## What followed

- **Varro read this list** and agrees with every bin. He checked gaps A, I and J against the
  text, as the three nearest to a defect.
- **His list has one defect, D1, in the field engine.** I agree with it. Michal decided that
  it is recorded and left out of the re-freeze.
- **Michal decided that gap A stays in the new batch** and goes first on the second round's
  list ([STATUS](../STATUS.md), 11 to 24).
- **Still to come:** the field engine's half of the page for Michal.
