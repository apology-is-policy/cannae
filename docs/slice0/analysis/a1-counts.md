# A1: counts from the records of `base`

Printed by `tools/a1-counts.py` from the scored batch's records (freeze seed 9464880715404986219), the table of record and `cannae-extras sets`. **Exploratory:** none of these counts was registered before the batch; none changes a verdict.

| File | Stopped runs | Sets | Whole runs | Whole runs whose summary equals the stopped run's |
|---|---|---|---|---|
| pairwise-r0-base.jsonl | 2000 | 200 | 100 | 100 |
| pairwise-r1-base.jsonl | 2000 | 200 | 100 | 100 |
| pairwise-r2-base.jsonl | 2000 | 200 | 100 | 100 |
| pairwise-r3-base.jsonl | 2000 | 200 | 100 | 100 |
| field-r0-base.jsonl | 2000 | 200 | 100 | 100 |
| field-r1-base.jsonl | 2000 | 200 | 100 | 100 |
| field-r2-base.jsonl | 2000 | 200 | 100 | 100 |
| field-r3-base.jsonl | 2000 | 200 | 100 | 100 |

## 1. Who lost

By run: whose line broke first. By set: in how many of its ten seeds the Romans' did.

| Cell | Runs | Romans | Warband | Neither | Sets: Romans in all 10 | 6-9 | 5 | 1-4 | 0 | Same side in all 10 |
|---|---|---|---|---|---|---|---|---|---|---|
| Pairwise rung 0 | 2000 | 1829 (91.5 %) | 167 (8.3 %) | 4 (0.2 %) | 90 | 109 | 1 | 0 | 0 | 90 |
| Pairwise rung 1 | 2000 | 1828 (91.4 %) | 168 (8.4 %) | 4 (0.2 %) | 90 | 109 | 1 | 0 | 0 | 90 |
| Pairwise rung 2 | 2000 | 1830 (91.5 %) | 168 (8.4 %) | 2 (0.1 %) | 92 | 107 | 1 | 0 | 0 | 92 |
| Pairwise rung 3 | 2000 | 1834 (91.7 %) | 165 (8.2 %) | 1 (0.1 %) | 94 | 105 | 1 | 0 | 0 | 94 |
| Field rung 0 | 2000 | 1764 (88.2 %) | 192 (9.6 %) | 44 (2.2 %) | 81 | 113 | 4 | 2 | 0 | 81 |
| Field rung 1 | 2000 | 1766 (88.3 %) | 188 (9.4 %) | 46 (2.3 %) | 84 | 109 | 4 | 3 | 0 | 84 |
| Field rung 2 | 2000 | 1811 (90.5 %) | 179 (8.9 %) | 10 (0.5 %) | 88 | 111 | 1 | 0 | 0 | 88 |
| Field rung 3 | 2000 | 1797 (89.8 %) | 171 (8.6 %) | 32 (1.6 %) | 90 | 108 | 1 | 1 | 0 | 90 |

## 2. Men lost before the break

Runs with a break. Each side has 160 men in ranks. Median (quartiles) over runs; the last column is the runs in which the loser broke with no man of its own down.

| Cell | Runs with a break | Loser's losses | as a share of its ranks | Winner's losses | Broke before contact | Loser lost none |
|---|---|---|---|---|---|---|
| Pairwise rung 0 | 1996 | 1 (0-3) | 0.006 (0.000-0.019) | 0 (0-0) | 8 (0.4 %) | 741 (37.1 %) |
| Pairwise rung 1 | 1996 | 1 (0-3) | 0.006 (0.000-0.019) | 0 (0-0) | 8 (0.4 %) | 743 (37.2 %) |
| Pairwise rung 2 | 1998 | 1 (0-2) | 0.006 (0.000-0.013) | 0 (0-0) | 8 (0.4 %) | 742 (37.1 %) |
| Pairwise rung 3 | 1999 | 1 (0-3) | 0.006 (0.000-0.019) | 0 (0-0) | 9 (0.5 %) | 785 (39.3 %) |
| Field rung 0 | 1956 | 1 (0-3) | 0.006 (0.000-0.019) | 0 (0-1) | 3 (0.2 %) | 729 (37.3 %) |
| Field rung 1 | 1954 | 1 (0-3) | 0.006 (0.000-0.019) | 0 (0-1) | 3 (0.2 %) | 736 (37.7 %) |
| Field rung 2 | 1990 | 1 (0-2) | 0.006 (0.000-0.013) | 0 (0-1) | 3 (0.2 %) | 730 (36.7 %) |
| Field rung 3 | 1968 | 1 (0-3) | 0.006 (0.000-0.019) | 0 (0-1) | 3 (0.2 %) | 800 (40.7 %) |

## 3. The spread of break times within a set

Sets whose ten seeds all broke after contact. Coefficient of variation and the longest break over the shortest are per set (median and quartiles over sets). The sum of squares of the logarithm of the time to break is split into within and between sets; for reference, a constant hazard (exponential times) gives a standard deviation of 1.28 within a set, a Weibull of shape k gives that over k.

| Cell | Sets used | Sets' median time to break, s | CV within a set | Longest / shortest | Share of log sum of squares within sets | SD of log within sets | Weibull shape it implies |
|---|---|---|---|---|---|---|---|
| Pairwise rung 0 | 191 | 19.5 (12.9-41.8) | 0.74 (0.40-1.34) | 7.3 (3.4-20.2) | 0.52 | 0.85 | 1.51 |
| Pairwise rung 1 | 191 | 19.5 (12.9-41.8) | 0.73 (0.40-1.33) | 7.2 (3.4-19.4) | 0.52 | 0.84 | 1.52 |
| Pairwise rung 2 | 193 | 18.6 (12.0-32.4) | 0.66 (0.40-1.30) | 6.5 (3.4-17.7) | 0.57 | 0.82 | 1.56 |
| Pairwise rung 3 | 193 | 19.5 (12.9-37.2) | 0.73 (0.40-1.30) | 6.9 (3.4-18.0) | 0.54 | 0.83 | 1.54 |
| Field rung 0 | 170 | 21.7 (13.4-43.2) | 1.04 (0.56-1.50) | 12.1 (5.2-32.2) | 0.53 | 0.97 | 1.32 |
| Field rung 1 | 171 | 21.9 (13.4-42.6) | 0.98 (0.52-1.47) | 11.3 (4.5-28.7) | 0.49 | 0.94 | 1.37 |
| Field rung 2 | 190 | 21.0 (12.9-40.2) | 0.76 (0.51-1.40) | 8.9 (4.4-25.4) | 0.54 | 0.91 | 1.41 |
| Field rung 3 | 179 | 22.9 (13.8-46.2) | 0.88 (0.46-1.43) | 9.6 (3.9-26.0) | 0.51 | 0.91 | 1.42 |

## 4. Lines that held five minutes

A run held when no side broke, or its loser broke 300 s or more after contact. Pattern 6's band asks that at most one seed in ten break early, so a set needs nine or ten held seeds to pass it.

| Cell | Held runs | of which no break | Sets with 0 held | 1-4 | 5-8 | 9-10 |
|---|---|---|---|---|---|---|
| Pairwise rung 0 | 77 (3.9 %) | 4 | 159 | 39 | 1 | 1 |
| Pairwise rung 1 | 77 (3.9 %) | 4 | 159 | 39 | 1 | 1 |
| Pairwise rung 2 | 48 (2.4 %) | 2 | 172 | 27 | 1 | 0 |
| Pairwise rung 3 | 63 (3.1 %) | 1 | 164 | 34 | 1 | 1 |
| Field rung 0 | 233 (11.7 %) | 44 | 117 | 63 | 18 | 2 |
| Field rung 1 | 230 (11.5 %) | 46 | 119 | 61 | 18 | 2 |
| Field rung 2 | 111 (5.5 %) | 10 | 142 | 53 | 4 | 1 |
| Field rung 3 | 151 (7.5 %) | 32 | 135 | 59 | 5 | 1 |

### Sets with five or more held seeds, and the scorer's verdicts on them

Patterns passed, 1 to 8, pattern 6 by its band alone.

| Cell | Set | Held seeds | Romans lost | Patterns passed | Could not be scored |
|---|---|---|---|---|---|
| Pairwise rung 0 | 49 | 5 | 7 | 1, 2, 7, 8 | - |
| Pairwise rung 0 | 94 | 9 | 10 | 1, 2, 4, 6, 8 | - |
| Pairwise rung 1 | 49 | 5 | 7 | 1, 2, 7, 8 | - |
| Pairwise rung 1 | 94 | 9 | 10 | 1, 2, 4, 6, 8 | - |
| Pairwise rung 2 | 94 | 6 | 10 | 1, 2, 4, 7, 8 | - |
| Pairwise rung 3 | 49 | 5 | 9 | 1, 2, 7, 8 | - |
| Pairwise rung 3 | 94 | 9 | 10 | 1, 2, 4, 6, 7 | - |
| Field rung 0 | 3 | 5 | 7 | 1, 4, 7, 8 | - |
| Field rung 0 | 8 | 5 | 8 | 1, 2, 4, 7, 8 | - |
| Field rung 0 | 9 | 6 | 7 | 1, 7, 8 | - |
| Field rung 0 | 14 | 6 | 6 | 1, 2, 7, 8 | - |
| Field rung 0 | 17 | 6 | 8 | 1, 2, 4, 7, 8 | - |
| Field rung 0 | 19 | 5 | 5 | 1, 2, 7, 8 | - |
| Field rung 0 | 49 | 6 | 4 | 1, 5, 8 | - |
| Field rung 0 | 74 | 6 | 7 | 1, 2, 4, 7, 8 | - |
| Field rung 0 | 91 | 5 | 5 | 7, 8 | - |
| Field rung 0 | 93 | 7 | 6 | 1, 2, 4, 5, 7, 8 | - |
| Field rung 0 | 94 | 10 | 5 | 1, 2, 7, 8 | - |
| Field rung 0 | 105 | 5 | 7 | 1, 4, 7, 8 | - |
| Field rung 0 | 140 | 7 | 6 | 1, 8 | - |
| Field rung 0 | 142 | 5 | 5 | 4, 8 | - |
| Field rung 0 | 150 | 5 | 8 | 1, 4, 7, 8 | - |
| Field rung 0 | 155 | 6 | 6 | 1, 2, 4, 7, 8 | - |
| Field rung 0 | 156 | 7 | 9 | 1, 4, 8 | - |
| Field rung 0 | 159 | 5 | 9 | 1, 4, 7, 8 | - |
| Field rung 0 | 178 | 9 | 9 | 1, 4, 6, 7, 8 | - |
| Field rung 0 | 195 | 6 | 4 | 1, 2, 4, 7, 8 | - |
| Field rung 1 | 3 | 5 | 7 | 1, 4, 7, 8 | - |
| Field rung 1 | 8 | 5 | 8 | 1, 2, 4, 7, 8 | - |
| Field rung 1 | 9 | 6 | 7 | 1, 7 | - |
| Field rung 1 | 14 | 6 | 6 | 1, 2, 7, 8 | - |
| Field rung 1 | 17 | 6 | 9 | 1, 2, 4, 7, 8 | - |
| Field rung 1 | 19 | 5 | 5 | 1, 2, 7 | - |
| Field rung 1 | 49 | 6 | 3 | 1, 8 | - |
| Field rung 1 | 91 | 5 | 5 | 7, 8 | - |
| Field rung 1 | 93 | 7 | 5 | 1, 2, 4, 7, 8 | - |
| Field rung 1 | 94 | 10 | 4 | 1, 4, 7, 8 | - |
| Field rung 1 | 105 | 5 | 7 | 1, 4, 7, 8 | - |
| Field rung 1 | 107 | 5 | 9 | 4, 7, 8 | - |
| Field rung 1 | 140 | 7 | 6 | 1, 4, 8 | - |
| Field rung 1 | 142 | 5 | 4 | 3, 4, 8 | - |
| Field rung 1 | 150 | 5 | 9 | 1, 4, 7, 8 | - |
| Field rung 1 | 155 | 7 | 5 | 1, 2, 4, 7, 8 | - |
| Field rung 1 | 156 | 7 | 8 | 1, 4, 7, 8 | - |
| Field rung 1 | 159 | 6 | 9 | 1, 4, 7, 8 | - |
| Field rung 1 | 178 | 9 | 10 | 1, 2, 4, 6, 7, 8 | - |
| Field rung 1 | 195 | 6 | 6 | 1, 2, 4, 7, 8 | - |
| Field rung 2 | 8 | 6 | 8 | 1, 2, 4, 7, 8 | - |
| Field rung 2 | 94 | 9 | 7 | 1, 2, 6, 7, 8 | - |
| Field rung 2 | 156 | 6 | 9 | 1, 4, 7, 8 | - |
| Field rung 2 | 178 | 5 | 10 | 1, 4, 7, 8 | - |
| Field rung 2 | 195 | 5 | 7 | 1, 2, 7 | - |
| Field rung 3 | 17 | 7 | 7 | 1, 2, 4, 7, 8 | - |
| Field rung 3 | 49 | 6 | 5 | 1, 2, 8 | - |
| Field rung 3 | 94 | 10 | 3 | 1, 4, 7, 8 | - |
| Field rung 3 | 155 | 6 | 7 | 1, 2, 4, 7, 8 | - |
| Field rung 3 | 156 | 7 | 9 | 1, 4, 7, 8 | - |
| Field rung 3 | 178 | 5 | 10 | 1, 4, 7, 8 | - |

### How often each pattern passes: those sets, and all sets

| Cell | Sets | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 |
|---|---|---|---|---|---|---|---|---|---|
| Pairwise rung 0, five or more held | 2 | 2 | 2 | 0 | 1 | 0 | 1 | 1 | 2 |
| Pairwise rung 0, all | 200 | 2 | 2 | 0 | 43 | 87 | 1 | 19 | 136 |
| Pairwise rung 1, five or more held | 2 | 2 | 2 | 0 | 1 | 0 | 1 | 1 | 2 |
| Pairwise rung 1, all | 200 | 2 | 2 | 0 | 44 | 86 | 1 | 19 | 133 |
| Pairwise rung 2, five or more held | 1 | 1 | 1 | 0 | 1 | 0 | 0 | 1 | 1 |
| Pairwise rung 2, all | 200 | 1 | 1 | 0 | 28 | 87 | 0 | 18 | 132 |
| Pairwise rung 3, five or more held | 2 | 2 | 2 | 0 | 1 | 0 | 1 | 2 | 1 |
| Pairwise rung 3, all | 200 | 2 | 2 | 0 | 43 | 79 | 1 | 21 | 127 |
| Field rung 0, five or more held | 20 | 18 | 9 | 0 | 13 | 2 | 1 | 16 | 20 |
| Field rung 0, all | 200 | 18 | 9 | 0 | 83 | 116 | 1 | 42 | 122 |
| Field rung 1, five or more held | 20 | 17 | 8 | 1 | 15 | 0 | 1 | 17 | 18 |
| Field rung 1, all | 200 | 17 | 8 | 1 | 84 | 114 | 1 | 39 | 124 |
| Field rung 2, five or more held | 5 | 5 | 3 | 0 | 3 | 0 | 1 | 5 | 4 |
| Field rung 2, all | 200 | 5 | 4 | 0 | 63 | 105 | 1 | 33 | 119 |
| Field rung 3, five or more held | 6 | 6 | 3 | 0 | 5 | 0 | 0 | 5 | 6 |
| Field rung 3, all | 200 | 6 | 5 | 0 | 79 | 115 | 0 | 33 | 120 |

### Held runs beside the rest, run by run

Median of each run's own value; the bands are for sets, so this only compares.

| Cell | Runs | formed phase, s | median gap, m | burstiness | depth of the first to turn | spread, s | loser's share of hits after the break | Romans lost | Loser's losses |
|---|---|---|---|---|---|---|---|---|---|
| Pairwise rung 0 held | 77 | 946.000 | 3.600 | 0.479 | 0.246 | 7.750 | 0.167 | 52 (71.2 %) | 4 |
| Pairwise rung 0 broke early | 1923 | 20.750 | 1.293 | -0.056 | 0.175 | 6.000 | 0.600 | 1777 (92.4 %) | 1 |
| Pairwise rung 1 held | 77 | 928.500 | 3.600 | 0.479 | 0.250 | 7.750 | 0.125 | 52 (71.2 %) | 4 |
| Pairwise rung 1 broke early | 1923 | 20.750 | 1.295 | -0.056 | 0.175 | 6.000 | 0.600 | 1776 (92.4 %) | 1 |
| Pairwise rung 2 held | 48 | 982.750 | 3.600 | 0.416 | 0.206 | 5.500 | 0.333 | 33 (71.7 %) | 2 |
| Pairwise rung 2 broke early | 1952 | 17.750 | 1.260 | -0.060 | 0.175 | 5.250 | 0.587 | 1797 (92.1 %) | 1 |
| Pairwise rung 3 held | 63 | 849.000 | 3.600 | 0.470 | 0.222 | 8.500 | 0.200 | 46 (74.2 %) | 3 |
| Pairwise rung 3 broke early | 1937 | 20.000 | 1.286 | -0.058 | 0.175 | 5.750 | 0.571 | 1788 (92.3 %) | 1 |
| Field rung 0 held | 233 | 1087.500 | 3.925 | 0.505 | 0.389 | 17.000 | 0.118 | 137 (72.5 %) | 5 |
| Field rung 0 broke early | 1767 | 23.750 | 1.438 | -0.039 | 0.254 | 6.750 | 0.778 | 1627 (92.1 %) | 1 |
| Field rung 1 held | 230 | 1077.625 | 3.925 | 0.521 | 0.405 | 18.750 | 0.111 | 134 (72.8 %) | 5 |
| Field rung 1 broke early | 1770 | 23.750 | 1.440 | -0.039 | 0.254 | 6.750 | 0.750 | 1632 (92.2 %) | 1 |
| Field rung 2 held | 111 | 953.000 | 3.600 | 0.490 | 0.317 | 8.000 | 0.000 | 72 (71.3 %) | 2 |
| Field rung 2 broke early | 1889 | 20.500 | 1.377 | -0.049 | 0.238 | 6.000 | 0.714 | 1739 (92.1 %) | 1 |
| Field rung 3 held | 151 | 1272.000 | 3.925 | 0.503 | 0.341 | 9.750 | 0.128 | 88 (73.9 %) | 4 |
| Field rung 3 broke early | 1849 | 24.000 | 1.473 | -0.043 | 0.246 | 7.000 | 0.750 | 1709 (92.4 %) | 1 |

## 5. A rank screening of the 101 drawn lines

Spearman's rho over the 200 sets between each line's drawn value and two outcomes: the set's median time to break (a run with no break counts as longer than any), and the share of its seeds in which the Romans' line broke first. `FW` is the family-wise 95 % line (1000 shuffles of the outcome over the sets); `not read` is the largest |rho| among the lines this cell never reads, a control that should sit under the line.

### Pairwise rung 0

Time to break: FW line 0.241; not read (19 lines): 0.112 (gauges.keep_pm). Romans lost: FW line 0.245; not read: 0.107 (gauges.empty_q8).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.481 * | +0.081 |
| man.beside | +0.300 * | +0.102 |
| man.enemy_front | -0.262 * | +0.020 |
| troop.hastati.leader_ardour | +0.085 | -0.331 * |
| troop.warband.ardour | -0.047 | +0.265 * |
| troop.warband.front_ardour | -0.037 | +0.288 * |

`*`: past the cell's family-wise line.

### Pairwise rung 1

Time to break: FW line 0.246; not read (18 lines): 0.112 (gauges.keep_pm). Romans lost: FW line 0.242; not read: 0.144 (field.pack_q8).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.481 * | +0.058 |
| man.beside | +0.299 * | +0.097 |
| man.enemy_front | -0.263 * | -0.013 |
| troop.hastati.leader_ardour | +0.083 | -0.324 * |
| troop.warband.ardour | -0.049 | +0.256 * |
| troop.warband.front_ardour | -0.037 | +0.316 * |

`*`: past the cell's family-wise line.

### Pairwise rung 2

Time to break: FW line 0.246; not read (16 lines): 0.121 (gauges.keep_pm). Romans lost: FW line 0.244; not read: 0.126 (field.pack_q8).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.421 * | +0.115 |
| bout.harm_ppm | -0.296 * | -0.054 |
| man.beside | +0.288 * | +0.117 |
| bout.rate_q16 | -0.246 | -0.333 * |
| troop.hastati.leader_ardour | +0.087 | -0.334 * |
| troop.warband.ardour | -0.055 | +0.257 * |
| troop.warband.front_ardour | -0.017 | +0.364 * |

`*`: past the cell's family-wise line.

### Pairwise rung 3

Time to break: FW line 0.240; not read (13 lines): 0.115 (man.shaken). Romans lost: FW line 0.245; not read: 0.126 (field.pack_q8).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.467 * | +0.100 |
| man.beside | +0.293 * | +0.089 |
| man.enemy_front | -0.264 * | -0.032 |
| bout.rate_q16 | -0.213 | -0.266 * |
| troop.hastati.leader_ardour | +0.082 | -0.316 * |
| troop.warband.front_ardour | -0.050 | +0.317 * |

`*`: past the cell's family-wise line.

### Field rung 0

Time to break: FW line 0.246; not read (15 lines): 0.128 (man.shaken). Romans lost: FW line 0.246; not read: 0.138 (gauges.spend_ms).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.486 * | -0.226 |
| man.beside | +0.269 * | +0.048 |
| man.enemy_front | -0.259 * | +0.084 |
| bout.rate_q16 | -0.253 * | -0.184 |
| troop.warband.front_ardour | -0.029 | +0.255 * |

`*`: past the cell's family-wise line.

### Field rung 1

Time to break: FW line 0.245; not read (14 lines): 0.128 (man.shaken). Romans lost: FW line 0.246; not read: 0.119 (troop.recruits.nerve_below).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.488 * | -0.202 |
| man.beside | +0.272 * | +0.095 |
| man.enemy_front | -0.262 * | +0.069 |
| bout.rate_q16 | -0.254 * | -0.222 |
| troop.warband.ardour | -0.043 | +0.249 * |
| troop.warband.front_ardour | -0.027 | +0.262 * |

`*`: past the cell's family-wise line.

### Field rung 2

Time to break: FW line 0.248; not read (12 lines): 0.140 (pairwise.radius_mm). Romans lost: FW line 0.240; not read: 0.141 (gauges.spend_ms).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.423 * | -0.039 |
| bout.harm_ppm | -0.347 * | -0.002 |
| man.beside | +0.284 * | +0.077 |
| bout.rate_q16 | -0.270 * | -0.321 * |
| troop.hastati.leader_ardour | +0.053 | -0.250 * |
| troop.warband.ardour | -0.072 | +0.327 * |
| troop.warband.front_ardour | +0.012 | +0.383 * |

`*`: past the cell's family-wise line.

### Field rung 3

Time to break: FW line 0.245; not read (9 lines): 0.127 (man.shaken). Romans lost: FW line 0.246; not read: 0.086 (pairwise.core_mm).

| Line | rho, time to break | rho, Romans lost |
|---|---|---|
| tie.standoff_mm | +0.467 * | -0.115 |
| man.enemy_front | -0.289 * | +0.029 |
| man.beside | +0.279 * | +0.121 |
| bout.rate_q16 | -0.264 * | -0.268 * |
| bout.drive_back_pm | -0.256 * | +0.191 |
| troop.hastati.leader_ardour | +0.074 | -0.280 * |
| troop.warband.front_ardour | -0.017 | +0.341 * |

`*`: past the cell's family-wise line.

### Across the cells: the lines past the family-wise line for time to break

| Line | Cells past it | Pairwise rung 0 | Pairwise rung 1 | Pairwise rung 2 | Pairwise rung 3 | Field rung 0 | Field rung 1 | Field rung 2 | Field rung 3 |
|---|---|---|---|---|---|---|---|---|---|
| tie.standoff_mm | 8 | +0.48* | +0.48* | +0.42* | +0.47* | +0.49* | +0.49* | +0.42* | +0.47* |
| man.beside | 8 | +0.30* | +0.30* | +0.29* | +0.29* | +0.27* | +0.27* | +0.28* | +0.28* |
| man.enemy_front | 6 | -0.26* | -0.26* | -0.24 | -0.26* | -0.26* | -0.26* | -0.24 | -0.29* |
| bout.rate_q16 | 4 | -0.22 | -0.22 | -0.25 | -0.21 | -0.25* | -0.25* | -0.27* | -0.26* |
| bout.harm_ppm | 2 | -0.16 | -0.16 | -0.30* | -0.15 | -0.17 | -0.18 | -0.35* | -0.17 |
| bout.drive_back_pm | 1 | -0.21 | -0.21 | -0.16 | -0.23 | -0.24 | -0.24 | -0.16 | -0.26* |

### Across the cells: the lines past the family-wise line for Romans lost

| Line | Cells past it | Pairwise rung 0 | Pairwise rung 1 | Pairwise rung 2 | Pairwise rung 3 | Field rung 0 | Field rung 1 | Field rung 2 | Field rung 3 |
|---|---|---|---|---|---|---|---|---|---|
| troop.warband.front_ardour | 8 | +0.29* | +0.32* | +0.36* | +0.32* | +0.25* | +0.26* | +0.38* | +0.34* |
| troop.hastati.leader_ardour | 6 | -0.33* | -0.32* | -0.33* | -0.32* | -0.24 | -0.22 | -0.25* | -0.28* |
| troop.warband.ardour | 5 | +0.26* | +0.26* | +0.26* | +0.23 | +0.24 | +0.25* | +0.33* | +0.24 |
| bout.rate_q16 | 4 | -0.22 | -0.23 | -0.33* | -0.27* | -0.18 | -0.22 | -0.32* | -0.27* |

