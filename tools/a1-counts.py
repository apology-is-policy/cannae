#!/usr/bin/env python3
"""Counts from the Slice 0 batch's records: the analysis arc's chunk A1.

    tools/a1-counts.py --records DIR --table docs/slice0/table-of-record.csv \\
        --sets docs/slice0/analysis/sets.csv [--perms 1000]
    tools/a1-counts.py --self-test

Reads the records of `base` for the eight cells (`<engine>-r<rung>-base.jsonl` in DIR), the
table of record, and the scorer's verdicts set by set (`cannae-extras sets`). Prints, cell by
cell: who lost; the men each side lost before the break; the spread of break times within a
set; the runs and sets in which a line held five minutes, and what they pass; and a rank
screening of the 101 drawn lines against the time to break and against who lost.

It runs nothing and changes nothing. **Exploratory:** none of these counts was registered
before the batch, and none of them changes a verdict.

Definitions (each from crates/cannae-harness/src/metrics.rs at the frozen commit):
- The time to break is the loser's break less the contact, `time_to_break_s`; a run with no
  break has none, and counts as longer than any break.
- A line held when its run had no break, or a time to break of 300 s or more: pattern 6's
  "early" is a break within 300 s.
- Losses are men disabled or killed (a man's first wound at `Disabled` or worse). Before the
  break means before contact (`losses_before_contact`) plus while formed, from contact to the
  loser's break. Each side's count while formed is recovered from its hits per file per hour,
  times its files (20 a side in `base`), times the formed hours; the two must add up to
  `hits_formed`, and the tool stops if they do not.
- Spearman's rho is Pearson's r on average ranks. The family-wise line is the 95th
  percentile of the largest |rho| over the 101 lines, with the outcome shuffled over the sets
  (a fixed seed, so the output is the same each time).
"""

import argparse
import csv
import json
import math
import operator
import os
import random
import sys

BATCH = 9_464_880_715_404_986_219
FILES = (20, 20)  # front files of the Romans and of the warband in scenarios/patch/base.toml
RANKED = 160  # men in ranks on each side in base: 2 x 10 x 8 Romans, 20 x 8 warband
HELD_S = 300.0
CELLS = [(e, r) for e in ("pairwise", "field") for r in range(4)]
ENGINE_NAME = {"pairwise": "Pairwise", "field": "Field"}

# Lines a cell's `base` never reads (crates/cannae-score/src/draw.rs and lib.rs at the frozen
# commit): the other engine's own lines; the gauges below rung 3; the troops `base` does not
# name; the has-fled weight at rung 0 (rung 3 keeps the has-fled mark); the shaken weights
# except at rung 2.
NOT_READ_ALWAYS = {
    "troop.veterans.nerve_above",
    "troop.veterans.spread_pm",
    "troop.recruits.nerve_below",
}


def not_read(engine, rung, names):
    out = set(NOT_READ_ALWAYS)
    other = "field." if engine == "pairwise" else "pairwise."
    out |= {n for n in names if n.startswith(other)}
    if rung < 3:
        out |= {n for n in names if n.startswith("gauges.")}
    if rung == 0:
        out.add("man.has_fled")
    if rung != 2:
        out |= {"man.shaken", "man.shaken_wear_ticks"}
    missing = out - set(names)
    if missing:
        raise SystemExit(f"lines not in the table: {sorted(missing)}")
    return out


# ---------------------------------------------------------------- small statistics


def median(v):
    s = sorted(v)
    n = len(s)
    if n == 0:
        return None
    return s[n // 2] if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2


def quantile(v, q):
    """Linear interpolation between order statistics (R's type 7)."""
    s = sorted(v)
    if not s:
        return None
    h = (len(s) - 1) * q
    lo = math.floor(h)
    hi = min(lo + 1, len(s) - 1)
    if s[lo] == s[hi]:
        return s[lo]
    return s[lo] + (h - lo) * (s[hi] - s[lo])


def ranks(v):
    """Average ranks from 1; equal values share their mean rank."""
    order = sorted(range(len(v)), key=lambda i: v[i])
    r = [0.0] * len(v)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and v[order[j + 1]] == v[order[i]]:
            j += 1
        mean = (i + j) / 2 + 1
        for k in range(i, j + 1):
            r[order[k]] = mean
        i = j + 1
    return r


def unit(v):
    """Centred and scaled to length one, so a dot product of two is Pearson's r."""
    m = sum(v) / len(v)
    c = [x - m for x in v]
    norm = math.sqrt(sum(x * x for x in c))
    return [x / norm for x in c] if norm > 0 else None


def dot(a, b):
    return sum(map(operator.mul, a, b))


def spearman(x, y):
    ux, uy = unit(ranks(x)), unit(ranks(y))
    return None if ux is None or uy is None else dot(ux, uy)


def within_share(groups):
    """Share of the pooled values' sum of squares that lies within groups, and the pooled
    within-group standard deviation."""
    allv = [x for g in groups for x in g]
    n, k = len(allv), len(groups)
    grand = sum(allv) / n
    total = sum((x - grand) ** 2 for x in allv)
    within = sum(sum((x - sum(g) / len(g)) ** 2 for x in g) for g in groups)
    return within / total, math.sqrt(within / (n - k))


def fmt(x, d=2):
    if x is None:
        return "-"
    if x == math.inf:
        return "none"
    return f"{x:.{d}f}"


def q3(v, d=2):
    if not v:
        return "-"
    return f"{fmt(median(v), d)} ({fmt(quantile(v, 0.25), d)}-{fmt(quantile(v, 0.75), d)})"


def pct(k, n):
    return f"{k} ({100 * k / n:.1f} %)" if n else "0"


# ---------------------------------------------------------------- the records


def side_losses(s):
    """Men each side lost before the loser's break: before contact, plus while formed."""
    formed = [0, 0]
    hours = s["formed_s"] / 3600.0
    for side in (0, 1):
        h = s["hits_per_file_hour"][side]
        if h is not None:
            x = h * FILES[side] * hours
            formed[side] = round(x)
            if abs(x - formed[side]) > 1e-6:
                raise SystemExit(f"hits per file hour {h} gives {x} men, not a whole number")
    if hours > 0 and sum(formed) != s["hits_formed"]:
        raise SystemExit(f"side counts {formed} do not add up to hits_formed {s['hits_formed']}")
    return [s["losses_before_contact"][i] + formed[i] for i in (0, 1)]


def load(records, engine, rung):
    """The stopped run of each (set, seed) of `base`, and how many whole runs agree with it."""
    name = f"{engine}-r{rung}-base.jsonl"
    stopped, whole = {}, {}
    with open(os.path.join(records, name)) as f:
        for line in f:
            r = json.loads(line)
            if (
                r["engine"] != ENGINE_NAME[engine]
                or r["rung"] != rung
                or r["variant"] != "base"
                or r["freeze_seed"] != BATCH
            ):
                raise SystemExit(f"{name}: a record of another cell or freeze")
            key = (r["draw"], r["seed"])
            (whole if r["whole"] else stopped)[key] = r
    agree = sum(1 for k, w in whole.items() if k in stopped and stopped[k]["summary"] == w["summary"])
    return name, stopped, len(whole), agree


class Run:
    def __init__(self, rec):
        s = rec["summary"]
        self.set = rec["draw"]
        self.s = s
        self.ttb = s["time_to_break_s"]
        self.loser = s["loser"]
        self.held = self.ttb is None or self.ttb >= HELD_S
        self.losses = side_losses(s)

    def ttb_or_inf(self):
        return math.inf if self.ttb is None else self.ttb


# ---------------------------------------------------------------- the sections


def who_lost(cells):
    print("## 1. Who lost\n")
    print("By run: whose line broke first. By set: in how many of its ten seeds the Romans' did.\n")
    print("| Cell | Runs | Romans | Warband | Neither | Sets: Romans in all 10 | 6-9 | 5 | 1-4 | 0 | Same side in all 10 |")
    print("|---|---|---|---|---|---|---|---|---|---|---|")
    for cell, runs, bysets in cells:
        n = len(runs)
        lo = [sum(1 for r in runs if r.loser == x) for x in (0, 1, None)]
        romans = [sum(1 for r in v if r.loser == 0) for v in bysets.values()]
        warband = [sum(1 for r in v if r.loser == 1) for v in bysets.values()]
        bands = [
            sum(1 for k in romans if k == 10),
            sum(1 for k in romans if 6 <= k <= 9),
            sum(1 for k in romans if k == 5),
            sum(1 for k in romans if 1 <= k <= 4),
            sum(1 for k in romans if k == 0),
        ]
        same = sum(1 for a, b in zip(romans, warband) if a == 10 or b == 10)
        print(
            f"| {cell} | {n} | {pct(lo[0], n)} | {pct(lo[1], n)} | {pct(lo[2], n)} | "
            + " | ".join(str(b) for b in bands)
            + f" | {same} |"
        )
    print()


def losses(cells):
    print("## 2. Men lost before the break\n")
    print(
        f"Runs with a break. Each side has {RANKED} men in ranks. Median (quartiles) over runs; "
        "the last column is the runs in which the loser broke with no man of its own down.\n"
    )
    print("| Cell | Runs with a break | Loser's losses | as a share of its ranks | Winner's losses | Broke before contact | Loser lost none |")
    print("|---|---|---|---|---|---|---|")
    for cell, runs, _ in cells:
        b = [r for r in runs if r.loser is not None]
        lose = [r.losses[r.loser] for r in b]
        win = [r.losses[1 - r.loser] for r in b]
        early = sum(1 for r in b if r.ttb == 0)
        none = sum(1 for x in lose if x == 0)
        print(
            f"| {cell} | {len(b)} | {q3(lose, 0)} | {q3([x / RANKED for x in lose], 3)} | "
            f"{q3(win, 0)} | {pct(early, len(b))} | {pct(none, len(b))} |"
        )
    print()


def spread(cells):
    print("## 3. The spread of break times within a set\n")
    print(
        "Sets whose ten seeds all broke after contact. Coefficient of variation and the longest "
        "break over the shortest are per set (median and quartiles over sets). The sum of "
        "squares of the logarithm of the time to break is split into within and between sets; for "
        "reference, a constant hazard (exponential times) gives a standard deviation of "
        f"{math.pi / math.sqrt(6):.2f} within a set, a Weibull of shape k gives that over k.\n"
    )
    print("| Cell | Sets used | Sets' median time to break, s | CV within a set | Longest / shortest | Share of log sum of squares within sets | SD of log within sets | Weibull shape it implies |")
    print("|---|---|---|---|---|---|---|---|")
    for cell, _, bysets in cells:
        groups, cvs, ratios, meds = [], [], [], []
        for v in bysets.values():
            t = [r.ttb for r in v]
            if any(x is None or x <= 0 for x in t):
                continue
            groups.append([math.log(x) for x in t])
            m = sum(t) / len(t)
            sd = math.sqrt(sum((x - m) ** 2 for x in t) / (len(t) - 1))
            cvs.append(sd / m)
            ratios.append(max(t) / min(t))
            meds.append(median(t))
        share, sdw = within_share(groups)
        print(
            f"| {cell} | {len(groups)} | {q3(meds, 1)} | {q3(cvs)} | {q3(ratios, 1)} | "
            f"{share:.2f} | {sdw:.2f} | {math.pi / math.sqrt(6) / sdw:.2f} |"
        )
    print()


PATTERN_KEYS = ["p1", "p2", "p3", "p4", "p5", "p6", "p7", "p8"]
RUN_METRICS = [
    ("formed phase, s", lambda s: s["formed_s"]),
    ("median gap, m", lambda s: s["median_gap_m"]),
    ("burstiness", lambda s: s["burstiness"]),
    ("depth of the first to turn", lambda s: s["first_turn_depth"]),
    ("spread, s", lambda s: s["spread_s"]),
    ("loser's share of hits after the break", lambda s: s["loser_after_break"]),
]


def held(cells, verdicts):
    print("## 4. Lines that held five minutes\n")
    print(
        f"A run held when no side broke, or its loser broke {HELD_S:.0f} s or more after contact. "
        "Pattern 6's band asks that at most one seed in ten break early, so a set needs nine or "
        "ten held seeds to pass it.\n"
    )
    print("| Cell | Held runs | of which no break | Sets with 0 held | 1-4 | 5-8 | 9-10 |")
    print("|---|---|---|---|---|---|---|")
    for cell, runs, bysets in cells:
        h = [r for r in runs if r.held]
        nob = sum(1 for r in h if r.ttb is None)
        k = [sum(1 for r in v if r.held) for v in bysets.values()]
        print(
            f"| {cell} | {pct(len(h), len(runs))} | {nob} | {sum(1 for x in k if x == 0)} | "
            f"{sum(1 for x in k if 1 <= x <= 4)} | {sum(1 for x in k if 5 <= x <= 8)} | "
            f"{sum(1 for x in k if x >= 9)} |"
        )
    print()
    print("### Sets with five or more held seeds, and the scorer's verdicts on them\n")
    print("Patterns passed, 1 to 8, pattern 6 by its band alone.\n")
    print("| Cell | Set | Held seeds | Romans lost | Patterns passed | Could not be scored |")
    print("|---|---|---|---|---|---|")
    anyrow = False
    for cell, _, bysets in cells:
        for d, v in sorted(bysets.items()):
            k = sum(1 for r in v if r.held)
            if k < 5:
                continue
            anyrow = True
            row = verdicts[(cell, d)]
            passed = [str(i + 1) for i, p in enumerate(PATTERN_KEYS) if row[p] == "1"]
            unscored = [str(i + 1) for i, p in enumerate(PATTERN_KEYS) if row[p] == ""]
            romans = sum(1 for r in v if r.loser == 0)
            print(
                f"| {cell} | {d} | {k} | {romans} | {', '.join(passed) or 'none'} | "
                f"{', '.join(unscored) or '-'} |"
            )
    if not anyrow:
        print("| (none) | | | | | |")
    print()
    print("### How often each pattern passes: those sets, and all sets\n")
    print("| Cell | Sets | " + " | ".join(str(i + 1) for i in range(8)) + " |")
    print("|---|---|" + "---|" * 8)
    for cell, _, bysets in cells:
        five = [d for d, v in bysets.items() if sum(1 for r in v if r.held) >= 5]
        for label, ds in (("five or more held", five), ("all", sorted(bysets))):
            counts = [sum(1 for d in ds if verdicts[(cell, d)][p] == "1") for p in PATTERN_KEYS]
            print(f"| {cell}, {label} | {len(ds)} | " + " | ".join(str(c) for c in counts) + " |")
    print()
    print("### Held runs beside the rest, run by run\n")
    print("Median of each run's own value; the bands are for sets, so this only compares.\n")
    head = " | ".join(m for m, _ in RUN_METRICS)
    print(f"| Cell | Runs | {head} | Romans lost | Loser's losses |")
    print("|---|---|" + "---|" * len(RUN_METRICS) + "---|---|")
    for cell, runs, _ in cells:
        for label, group in (("held", [r for r in runs if r.held]), ("broke early", [r for r in runs if not r.held])):
            if not group:
                print(f"| {cell} {label} | 0 |" + " - |" * (len(RUN_METRICS) + 2))
                continue
            vals = []
            for _, f in RUN_METRICS:
                v = [f(r.s) for r in group if f(r.s) is not None]
                vals.append(fmt(median(v), 3) if v else "-")
            broke = [r for r in group if r.loser is not None]
            romans = sum(1 for r in broke if r.loser == 0)
            ll = [r.losses[r.loser] for r in broke]
            print(
                f"| {cell} {label} | {len(group)} | " + " | ".join(vals)
                + f" | {pct(romans, len(broke))} | {fmt(median(ll), 0) if ll else '-'} |"
            )
    print()


def screening(cells, table, perms):
    names = [n for n in table[0].keys() if n != "draw"]
    sets = sorted(int(r["draw"]) for r in table)
    values = {int(r["draw"]): r for r in table}
    xs = {n: unit(ranks([float(values[d][n]) for d in sets])) for n in names}
    print("## 5. A rank screening of the 101 drawn lines\n")
    print(
        "Spearman's rho over the 200 sets between each line's drawn value and two outcomes: the "
        "set's median time to break (a run with no break counts as longer than any), and the "
        "share of its seeds in which the Romans' line broke first. `FW` is the family-wise 95 % "
        f"line ({perms} shuffles of the outcome over the sets); `not read` is the largest |rho| "
        "among the lines this cell never reads, a control that should sit under the line.\n"
    )
    found_t, found_r = {}, {}
    rhos = {}
    for idx, (cell, _, bysets) in enumerate(cells):
        engine, rung = cell.split()[0].lower(), int(cell.split()[-1])
        unread = not_read(engine, rung, names)
        if sorted(bysets) != sets:
            raise SystemExit(f"{cell}: sets in the records differ from the table's")
        yt = [median([r.ttb_or_inf() for r in bysets[d]]) for d in sets]
        yr = [sum(1 for r in bysets[d] if r.loser == 0) / len(bysets[d]) for d in sets]
        out = {}
        for label, y, found in (("time", yt, found_t), ("romans", yr, found_r)):
            uy = unit(ranks(y))
            rho = {n: dot(xs[n], uy) for n in names}
            rng = random.Random(BATCH + 17 * idx + (label == "romans"))
            yp = list(uy)
            mx = []
            xl = [xs[n] for n in names]
            for _ in range(perms):
                rng.shuffle(yp)
                mx.append(max(abs(dot(x, yp)) for x in xl))
            fw = quantile(mx, 0.95)
            ctrl = max(unread, key=lambda n: abs(rho[n]))
            out[label] = (rho, fw, ctrl)
            for n in names:
                if n not in unread and abs(rho[n]) > fw:
                    found.setdefault(n, set()).add(cell)
        rhos[cell] = out
        rt, fwt, ct = out["time"]
        rr, fwr, cr = out["romans"]
        print(f"### {cell}\n")
        print(
            f"Time to break: FW line {fwt:.3f}; not read ({len(unread)} lines): {abs(rt[ct]):.3f} "
            f"({ct}). Romans lost: FW line {fwr:.3f}; not read: {abs(rr[cr]):.3f} ({cr}).\n"
        )
        top = sorted((n for n in names if n not in unread), key=lambda n: -abs(rt[n]))
        shown = [n for n in top if abs(rt[n]) > fwt] or top[:5]
        extra = [n for n in names if n not in unread and abs(rr[n]) > fwr and n not in shown]
        print("| Line | rho, time to break | rho, Romans lost |")
        print("|---|---|---|")
        for n in shown + extra:
            mt = " *" if abs(rt[n]) > fwt else ""
            mr = " *" if abs(rr[n]) > fwr else ""
            print(f"| {n} | {rt[n]:+.3f}{mt} | {rr[n]:+.3f}{mr} |")
        print("\n`*`: past the cell's family-wise line.\n")

    for title, found, key in (
        ("time to break", found_t, "time"),
        ("Romans lost", found_r, "romans"),
    ):
        print(f"### Across the cells: the lines past the family-wise line for {title}\n")
        heads = " | ".join(c for c, _, _ in cells)
        print(f"| Line | Cells past it | {heads} |")
        print("|---|---|" + "---|" * len(cells))
        order = sorted(found, key=lambda n: (-len(found[n]), -sum(abs(rhos[c][key][0][n]) for c, _, _ in cells)))
        for n in order:
            row = []
            for c, _, _ in cells:
                rho, fw, _ = rhos[c][key]
                row.append(f"{rho[n]:+.2f}{'*' if abs(rho[n]) > fw and c in found[n] else ''}")
            print(f"| {n} | {len(found[n])} | " + " | ".join(row) + " |")
        print()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--records")
    ap.add_argument("--table")
    ap.add_argument("--sets")
    ap.add_argument("--perms", type=int, default=1000)
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        return self_test()
    if not (a.records and a.table and a.sets):
        ap.error("needs --records, --table and --sets")
    with open(a.table) as f:
        table = list(csv.DictReader(f))
    if len(table) != 200 or len(table[0]) != 102:
        raise SystemExit("the table of record should be 200 sets by 101 lines")
    verdicts = {}
    with open(a.sets) as f:
        for row in csv.DictReader(f):
            verdicts[(f"{ENGINE_NAME[row['engine']]} rung {row['rung']}", int(row["set"]))] = row
    cells = []
    print("# A1: counts from the records of `base`\n")
    print(
        "Printed by `tools/a1-counts.py` from the scored batch's records (freeze seed "
        f"{BATCH}), the table of record and `cannae-extras sets`. **Exploratory:** none of "
        "these counts was registered before the batch; none changes a verdict.\n"
    )
    print("| File | Stopped runs | Sets | Whole runs | Whole runs whose summary equals the stopped run's |")
    print("|---|---|---|---|---|")
    for engine, rung in CELLS:
        name, stopped, nwhole, agree = load(a.records, engine, rung)
        cell = f"{ENGINE_NAME[engine]} rung {rung}"
        runs = [Run(stopped[k]) for k in sorted(stopped)]
        bysets = {}
        for r in runs:
            bysets.setdefault(r.set, []).append(r)
        if any(len(v) != 10 for v in bysets.values()):
            raise SystemExit(f"{name}: a set without ten seeds")
        cells.append((cell, runs, bysets))
        print(f"| {name} | {len(runs)} | {len(bysets)} | {nwhole} | {agree} |")
    print()
    who_lost(cells)
    losses(cells)
    spread(cells)
    held(cells, verdicts)
    screening(cells, table, a.perms)
    return 0


def self_test():
    assert ranks([3, 1, 2, 2]) == [4, 1, 2.5, 2.5]
    assert ranks([math.inf, 1, math.inf]) == [2.5, 1, 2.5]
    assert abs(spearman([1, 2, 3, 4], [10, 20, 30, 45]) - 1) < 1e-12
    assert abs(spearman([1, 2, 3, 4], [4, 3, 2, 1]) + 1) < 1e-12
    assert spearman([1, 1, 1], [1, 2, 3]) is None
    # A worked value: x = 1..5, y = 2,1,4,3,5 gives sum d^2 = 4, rho = 1 - 6*4/120 = 0.8.
    assert abs(spearman([1, 2, 3, 4, 5], [2, 1, 4, 3, 5]) - 0.8) < 1e-12
    assert median([3, 1, 2]) == 2 and median([4, 1, 2, 3]) == 2.5 and median([]) is None
    assert quantile([1, 2, 3, 4], 0.25) == 1.75 and quantile([5], 0.9) == 5
    share, sd = within_share([[1.0, 1.0], [3.0, 3.0]])
    assert share == 0 and sd == 0
    share, sd = within_share([[0.0, 2.0], [0.0, 2.0]])
    assert abs(share - 1.0) < 1e-12 and abs(sd - math.sqrt(2)) < 1e-12
    # Hits per file per hour back to men: 3 Romans and 5 warriors down in 90 s formed.
    s = {
        "formed_s": 90.0,
        "hits_per_file_hour": [3 / 20 / (90 / 3600), 5 / 20 / (90 / 3600)],
        "hits_formed": 8,
        "losses_before_contact": [1, 0],
    }
    assert side_losses(s) == [4, 5]
    s["hits_formed"] = 9
    try:
        side_losses(s)
        raise AssertionError("a mismatch should stop it")
    except SystemExit:
        pass
    s0 = {"formed_s": 0.0, "hits_per_file_hour": [None, None], "hits_formed": 0, "losses_before_contact": [2, 3]}
    assert side_losses(s0) == [2, 3]
    names = [f"x.{i}" for i in range(3)] + sorted(NOT_READ_ALWAYS) + [
        "pairwise.radius_mm", "field.jam_q8", "gauges.keep_pm", "man.has_fled", "man.shaken",
        "man.shaken_wear_ticks",
    ]
    nr = not_read("field", 0, names)
    assert "pairwise.radius_mm" in nr and "field.jam_q8" not in nr and "man.has_fled" in nr
    nr = not_read("pairwise", 3, names)
    assert "field.jam_q8" in nr and "gauges.keep_pm" not in nr and "man.has_fled" not in nr
    assert "man.shaken" in nr and "man.shaken" not in not_read("pairwise", 2, names)
    print("self-test: all passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
