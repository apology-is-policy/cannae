# 8. Speed

**Draft by Paullus, 2026-11-19.** From [spike-pairwise.md](../spike-pairwise.md) and Varro's
[spike-field.md](../spike-field.md). Both readings were taken on 2026-11-11 by the rule the
two of us signed ("How a sustained reading is read", in spike-field.md): 150,000 men on the
fanless Air, ten minutes a scene, the mean of the last five minutes as the figure of record.

## Each engine against its own line, as first read

**Both engines miss their speed lines.** A reading under the line is final, since the man is
not in either figure and he only adds cost.

| Engine | The line | The scene it is read on | Figure of record | Verdict |
|---|---|---|---|---|
| Pairwise | 3x over a whole battle, on the Air's eight threads | `2 / (1/line + 1/pocket)`: line 8.42x, pocket 1.59x | **2.67x** | under |
| Field | 10x on one core | the pocket with perception | **1.43x** | under |

- **Pairwise:** the pocket slows as it fills, from 3.19x in its first half minute to 1.34x in
  its last, as the men touching rise from 116,000 to 147,000. Something held the Mac's load
  near 1.7 to 1.9 with nothing of ours running; it is logged, and the rule's test for a
  disturbed run did not trigger. Three side runs were lost when the Mac went down and enter
  no figure.
- **Field:** the pocket's rate fell every minute, from 3.35x to 1.32x, so ten more minutes
  would have read lower. Its substrate alone settles at 5.3x, under the line without the
  perception too. The line and the surge, substrate alone and not the line's scene, read
  31.8x and 13.3x.

These readings are the verdict on the engines as built.

## The share of the shared perception

**One component is the larger part of both failures.** Both spikes run Varro's perception on
one thread.

- **Pairwise:** in the pocket's last five minutes it took 67 % of the wall time, and 46 % in
  the line's. Without it the same runs would read about 4.8x and 15.7x, and a whole battle
  about 7.4x.
- **Field:** in the pocket it took 85 %. Without its time the same intervals would read 9.6x,
  still under 10x, and the perception alone would allow 1.7x. The 9.6x is higher than the
  substrate-alone run's 5.3x because that run reached a harder state of the scene: four times
  as far in simulated time, with more than twice as many men pressing.

The figures stand as read, with the perception in them.

## The second line: the perception on all threads

**Michal decided on 2026-11-11** that the perception is put on all threads and the speed read
again as a new line. The first readings stay as the verdict on the engines as built.

- **Its conditions were written and agreed before anything was read** (spike-pairwise.md): the
  change is in the pairwise spike alone, 29 lines added and 14 removed; the same rule, scenes
  and figure; one reading, against the same 3x.
- **The change computes the same thing:** ten short runs gave the same counts of Views, the
  same check and the same final hash as the spike as built, on the Pi and again on the Mac.
- **It has not been read.** The one attempt, on 2026-11-11 from 18:33 to 18:57 UTC, never got
  the load under 2 that the rule asks for: Apple's own work after that day's system update
  held it between 2.9 and 17. The script was stopped before any timed run started. **The
  reading waits until Michal can give the Mac about forty minutes with nothing else running.**
- **What I expected, unmeasured, when it was decided:** about 5x over a whole battle, if the
  perception runs four times faster on eight cores.

**The field engine is not read a second time** (Varro): its line is for one core, and its
pocket was under 10x even with the perception's time taken out.

## What the figures leave out

**The man is in neither figure.** The spikes run the substrate and the perception; the man's
own decisions, his fear and his ties, add cost on top. So a figure under the line is final,
and a figure over it would mean only "not failed yet".

## By the kill criteria agreed beforehand

[rival 06](../../rival/06-experiment.md) §4 set them on 2026-11-10, before either engine was
measured. **By them both designs die on speed** (pointed out by design review 2): the rival's
below 10x per core in the pocket, the vision's engine below 3x sustained at full scale. The
rival's second criterion, a line that holds for an hour and then breaks, fails too (section
1). With no survivor, the agreed procedure does not choose between the two. The reviewer
would not call the pairwise reading final, from one run under an unexplained load; by the
rule both signed it is.

## What follows

What follows from the speed is Michal's to decide with this report. Both engines are slower
than their lines on the Air, and on both the shared perception is the larger part of it.
