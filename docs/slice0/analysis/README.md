# Slice 0 analysis: why the line breaks

**Exploratory.** None of this was registered before the batch, and nothing here changes a
verdict or a mechanism. The arc and its chunks are in [STATUS](../../STATUS.md) ("Current
arc"); it follows [design review 2](../../reviews/2026-11-19-fable-5.2-slice0-review.md)'s
advice to analyse what exists before anything is built.

| Chunk | Output | State |
|---|---|---|
| A1. Counts from the records | [a1-counts.md](a1-counts.md), with the scorer's verdicts set by set in [sets.csv](sets.csv) | done, 2026-11-19 |
| A2. Mend the replay tool | | next |
| A3. The genealogy of the collapses, from replays | | |
| A4. The diagnosis, and a proposed status table for `docs/vision/` | | |

## A1: what the records say

**How it was counted.** `tools/a1-counts.py` reads the records of `base` in the eight cells,
2,000 runs a cell, with the table of record and the scorer's own verdicts set by set. The
verdicts come from the frozen scorer through `cannae-extras sets`, and they agree with
[fails.txt](../scored/fails.txt) in all 192 counts of pass, fail and cannot be scored. The
tool states its definitions at its head and tests its arithmetic (`--self-test`). Every figure
below is printed in a1-counts.md.

```
cannae-extras sets --records DIR > docs/slice0/analysis/sets.csv
tools/a1-counts.py --records DIR --table docs/slice0/table-of-record.csv \
    --sets docs/slice0/analysis/sets.csv > docs/slice0/analysis/a1-counts.md
```

`DIR` is the batch's records, a copy byte for byte of the Pi's.

### 1. The Romans' line is the one that breaks

**It breaks first in 88 to 92 % of runs in every cell.** In 81 to 94 of each cell's 200 sets
it broke first in all ten seeds. In no set did the warband's line break first in all ten. Who
loses is decided by the troops' ardour. The lines past the family-wise line for it are:

- the warband's front-rank ardour, in all eight cells;
- the hastati leaders' ardour, in six;
- the warband's ardour, in five;
- the bout rate, in four.

### 2. A line breaks having lost almost nobody

**The loser lost a median of one man before its break,** 0.6 % of its 160 men in ranks. In 37
to 41 % of runs it lost none. The winner lost a median of none. (A loss is a man disabled or
killed, before contact or while formed.)

So whatever turns the first men, it is not their losses. A fright spreads through a line that
nobody has hurt. That is what design review 2's reading predicts: a blow that is parried
drives a man back, in the same posture as a man giving ground, and the sight alarms his
neighbours. It is also what any fear that outruns harm would do. A3 counts the causes.

### 3. The seed matters as much as the parameter set

**About half the sum of squares of the log of the time to break lies within sets,** between
the ten seeds of one set (0.49 to 0.57).

- **Within a set, the log time has a standard deviation of 0.82 to 0.97.** That implies a
  Weibull shape of 1.3 to 1.6: a hazard that rises a little with time, not a clock. A
  constant hazard would give shape 1. These shapes are of the same order as section 3's.
- **The sets' median times to break are 18.6 to 22.9 s.**

### 4. A line that held did not bring the other patterns with it

**How often a line held for five minutes:**

- 2.4 to 3.9 % of runs on the pairwise engine;
- 5.5 to 11.7 % of runs on the field engine.

Pattern 6 needs nine held seeds in ten. One or two sets per cell reached that. Set 94 did in
seven of the eight cells, and set 178 at the field's rungs 0 and 1.

**Held runs show pulses and lulls.** Their median burstiness is 0.42 to 0.52, against −0.04
to −0.06 where the line broke early; the band is at least 0.2.

**They still break from the front.**

- **The first to turn stand at a depth of 0.21 to 0.41,** against the band's 0.6.
- **The loser takes most of its losses before its break, not after.** Its share after the
  break is 0 to 0.33, against the band's at least 0.6.
- **Pattern 3 passes in none of the 58 sets with five or more held seeds, and pattern 5 in
  two.**

Over all sets, pattern 5 passes in 79 to 116 per cell. Those are the quick breaks: the loser
has lost almost nobody before them (point 2), so most of its losses come after.

**So a line that lasts longer does not, by lasting, collapse from the rear or take its losses
in the rout.** A slower clock alone would not pass patterns 3 and 5.

### 5. The rank screening: the same few lines in every cell

**The screen behaves.** In each cell, the lines that cell never reads are a control. Their
largest |rho| is 0.09 to 0.14, under family-wise lines of 0.24 to 0.25.

**For the time to break, the lines past the family-wise line are the same on both engines and
at every rung:**

| Line | What it is | rho | Cells past the line |
|---|---|---|---|
| `tie.standoff_mm` | how far from the enemy the warband's frame halts, 3.0 to 4.5 m | +0.42 to +0.49 | 8 |
| `man.beside` | the support a man takes from each man of his side standing beside or ahead of him (it also scales a leader's) | +0.27 to +0.30 | 8 |
| `man.enemy_front` | the alarm of an enemy who has stepped forward at him, within reach | −0.24 to −0.29 | 6 |
| `bout.rate_q16` | how often bouts come | −0.21 to −0.27 | 4, the field's |
| `bout.harm_ppm` | how much harm a bout does | −0.15 to −0.35 | 2 |
| `bout.drive_back_pm` | the chance a blow that does not wound drives a steady man back | −0.16 to −0.26 | 1 |

**No memory line passes in any cell:** not the has-fled mark, not *shaken*, not the gauges.

What this says:

- **The time to break is set by what both engines share:** the frame's halt, the man's fast
  weights and the bout. No memory line moves it. That agrees with section 1 and with the
  review.
- **The knock-back pulls the same way in all eight cells,** but over its prior's range it is a
  weaker lever than the standoff. The screen does not single it out, and it cannot rule it
  out: a marginal rank correlation over 101 lines measures how much a line's range moves the
  outcome, not whether its mechanism is right.
- **The strongest lever on how long a line lasts is how far apart the frames halt,** not
  anything in a man's psyche. Why is a question for A3.

**The screen's limits.** It sees each line alone, so it misses interactions, and it covers one
scenario and 200 sets. A line whose prior is narrow can matter without showing.

### What A1 hands to A3

1. **When nobody is hurt, what turns the first men?** Count the causes the event log gives for
   the first men of each collapse to step back and to turn. Design review 2's knock-back is
   one candidate, the alarm of an enemy in front another.
2. **Why does a wider standoff lengthen the time from contact to the break?**
3. **In the runs that held, what finally broke the line, and why from the front?**
