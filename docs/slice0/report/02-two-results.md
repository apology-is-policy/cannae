# 2. The field engine has two results

**Draft by Paullus, 2026-11-19.** Every figure is copied from
[two.txt](../scored/two.txt) and the first batch's scores
([first-score-field-r0.txt](../scored/first-score-field-r0.txt) and the next three), as the
outline's commands printed them. How each count is read was fixed in
[the outline](../report-outline.md) ("The two results side by side: the counts") before any
record of the new batch was read.

## The verdict is the same on both codes

| Field engine, `base` | The first batch (frozen code) | The new batch (re-frozen code) |
|---|---|---|
| Sets passing all eight, pattern 6 by its band | 0 of 200 in each of the four cells | 0 of 200 in each of the four cells |

The first batch ran `base` alone, so it can give pattern 6 by its band only, and the new
batch's count is taken the same way beside it. (The scorer's own line "All eight" reads
nothing passed for the first batch by construction, since it needs the dose-response
variants; it is not quoted for the frozen code.) The new batch's full verdict is section
1's.

**The engines are compared on the re-frozen code only.** The field engine has two results
and the pairwise engine one: the pairwise engine's half could not run on the frozen code,
through a fault in my batch runner.

## 1. The pairs

Each run of the first batch beside the new batch's run of the same set and seed.

| Rung | Pairs | No partner | Count of held frames zero | Of those, equal | Count one or more | Of those, differing in their summary |
|---|---|---|---|---|---|---|
| 0 | 2,100 | 0 | 711 | 711 | 1,389 | 1,250 |
| 1 | 2,100 | 0 | 664 | 664 | 1,436 | 1,288 |
| 2 | 2,100 | 0 | 816 | 816 | 1,284 | 1,102 |
| 3 | 2,100 | 0 | 702 | 702 | 1,398 | 1,213 |

**Every run the fix did not act in is unchanged,** field for field but for the wall time and
the count: 2,893 of 2,893. Addendum 3 required every one; this was checked before anything
was scored.

## 2. Where the defect could act

| Rung | Scored runs with a count of one or more | Sets with such a run | The count over all 2,000 scored runs |
|---|---|---|---|
| 0 | 1,324 of 2,000 | 195 of 200 | median 1, quartiles 0 to 1, at most 6 |
| 1 | 1,369 of 2,000 | 196 of 200 | median 1, quartiles 0 to 1, at most 6 |
| 2 | 1,222 of 2,000 | 195 of 200 | median 1, quartiles 0 to 1, at most 4 |
| 3 | 1,331 of 2,000 | 197 of 200 | median 1, quartiles 0 to 1, at most 6 |

So the defect could act in about two runs in three, and in nearly every set.

## 3. The verdicts, set by set

For each pattern, the sets that pass in both batches, in the first alone, in the new one
alone, and in neither. It is read on the two middle counts, by the two-sided sign line.

- **One comparison of the 36 passed the line:** pattern 5 at rung 1, which 5 sets pass in
  the first batch alone and 15 in the new one alone (the line was 15 of 20).
- **Every other count was within it, or too small to tell.** "All eight" passes in no set
  of either batch. Patterns 3 and 6 pass in at most one set in either batch, and part in at
  most one. Patterns 1 and 2 have at most five sets that part.

The full table, cell by cell, is in [two.txt](../scored/two.txt).

## 4. Whether those runs differ

**Within the pairs, set by set** (each set's ten seeds pooled, so that a set is one look).
The outline counted 44 looks over the four cells and said about two would pass the line by
chance. Seven passed it:

| Rung | Value | Larger in the first batch | Smaller | The same | Direction |
|---|---|---|---|---|---|
| 0 | median gap while formed | 57 | 35 | 103 | larger in the first batch |
| 1 | median gap while formed | 59 | 36 | 101 | larger in the first batch |
| 2 | median gap while formed | 60 | 29 | 106 | larger in the first batch |
| 3 | median gap while formed | 57 | 31 | 109 | larger in the first batch |
| 1 | loser's share of hits after the break | 48 | 72 | 72 | larger in the new batch |
| 3 | spread, Spearman's rho | 55 | 80 | 62 | larger in the new batch |
| 3 | harm per bout | 62 | 41 | 94 | larger in the first batch |

**The one thing the fix plainly did:** the gap between the two lines while they were formed
is larger in the first batch, in all four cells. That is what the defect did: a century
stepped back a whole rank when its standard changed hands. The other three pass the line
once each, in one cell, about as many as chance gives.

**Against the rest of the cell,** in the new batch, the runs the fix acted in are different
battles from the others, and the outline said beforehand that this comparison shows no more
than that. On the field's rung 0 they are longer before a break (median 31.0 s against
25.9 s) and their spread takes longer (9.0 s against 5.0 s). The full lists are in
[two.txt](../scored/two.txt).

**Pattern 8, over those pairs:** the leaders' share of the hits in the formed fight and in
the rout is nearly the same in both batches in every cell (rung 0: 0.054 and 0.023 in the
first, 0.055 and 0.019 in the new).

## What it means

- **The fix changed the battles it acted in, as it should,** and left every other run as
  it was.
- **It did not change the verdict.** On both codes the field engine passes all eight in no
  set of any cell, and the shape of the failure of section 1 is the same: a fight of
  seconds, broken from the front.
