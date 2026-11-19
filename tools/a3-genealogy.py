#!/usr/bin/env python3
"""Counts over the genealogy of the collapses: the analysis arc's chunk A3.

    tools/a3-genealogy.py --table docs/slice0/table-of-record.csv \\
        genealogy-pairwise-r0.jsonl ... genealogy-field-r3.jsonl
    tools/a3-genealogy.py --self-test

Reads the lines `cannae-replay genealogy` wrote, one file per cell (named
`genealogy-<engine>-r<rung>.jsonl`), and prints, cell by cell: the times from contact to the
first bout, the loser's first man back, first knock-back, first turn and break; how each
side's men gave ground before the break (driven back in a bout, or from fear, and by which
cause); how the loser's men turned; the first man to turn; the same where the loser had
lost nobody; pattern 3 with its ties shared; the stragglers; the lines that held five minutes
and then broke; and the standoff against each time.

It runs nothing. **Exploratory:** none of this was registered before the batch, and none of
it changes a verdict.

The sample is `base`, sets 0, 10, .. 190, all ten seeds (`"sample": true`); every other run
in which a line held five minutes and then broke is replayed too, and counted only where the
held runs are asked about.
"""

import argparse
import csv
import json
import math
import os
import re
import sys

CELL_ORDER = [(e, r) for e in ("pairwise", "field") for r in range(4)]
NAME = {"pairwise": "Pairwise", "field": "Field"}
KINDS = ["after_knock_back", "after_fear_back", "from_place"]
KIND_WORDS = {
    "after_knock_back": "after a knock-back",
    "after_fear_back": "after giving ground from fear",
    "from_place": "from his place",
    "turned_at_or_after_break": "at or after the break",
}
PHASES = ["first_bout_s", "first_back_s", "first_knock_back_s", "first_turn_s", "break_s"]


def median(v):
    s = sorted(v)
    n = len(s)
    if n == 0:
        return None
    return s[n // 2] if n % 2 else (s[n // 2 - 1] + s[n // 2]) / 2


def quantile(v, q):
    s = sorted(v)
    if not s:
        return None
    h = (len(s) - 1) * q
    lo = math.floor(h)
    hi = min(lo + 1, len(s) - 1)
    return s[lo] + (h - lo) * (s[hi] - s[lo])


def ranks(v):
    order = sorted(range(len(v)), key=lambda i: v[i])
    r = [0.0] * len(v)
    i = 0
    while i < len(order):
        j = i
        while j + 1 < len(order) and v[order[j + 1]] == v[order[i]]:
            j += 1
        for k in range(i, j + 1):
            r[order[k]] = (i + j) / 2 + 1
        i = j + 1
    return r


def spearman(x, y):
    if len(x) < 3:
        return None
    rx, ry = ranks(x), ranks(y)
    mx, my = sum(rx) / len(rx), sum(ry) / len(ry)
    sxy = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
    sxx = sum((a - mx) ** 2 for a in rx)
    syy = sum((b - my) ** 2 for b in ry)
    return sxy / math.sqrt(sxx * syy) if sxx > 0 and syy > 0 else None


def fmt(x, d=1):
    return "-" if x is None else f"{x:.{d}f}"


def q3(v, d=1):
    if not v:
        return "-"
    return f"{fmt(median(v), d)} ({fmt(quantile(v, 0.25), d)}-{fmt(quantile(v, 0.75), d)})"


def share(k, n):
    return f"{k} ({100 * k / n:.0f} %)" if n else "0"


def top(counts, k=4):
    """The k largest of a cause -> count map, as shares of its total."""
    total = sum(counts.values())
    if not total:
        return "-"
    items = sorted(counts.items(), key=lambda kv: (-kv[1], kv[0]))[:k]
    return ", ".join(f"{c} {100 * n / total:.0f} %" for c, n in items)


def add(into, counts):
    for c, n in counts.items():
        into[c] = into.get(c, 0) + n


def load(path):
    counted, not_counted = [], 0
    with open(path) as f:
        for line in f:
            if not line.strip():
                continue
            j = json.loads(line)
            if "not_counted" in j:
                not_counted += 1
            else:
                counted.append(j)
    return counted, not_counted


def broke(runs):
    return [r for r in runs if r["loser"] is not None]


# ---------------------------------------------------------------- the sections


def phases(cells):
    print("## 1. From contact to the break\n")
    print(
        "Sample runs with a break. Seconds from contact to each of: the first bout; the "
        "loser's first man to give ground, for any reason; its first man driven back in a "
        "bout; its first man to turn; its break. Median (quartiles); a run without one of "
        "them (no knock-back before the break, say) is left out of that column, and the "
        "count of runs that had it follows.\n"
    )
    print("| Cell | Runs | First bout | First man back | First knock-back | First turn | Break |")
    print("|---|---|---|---|---|---|---|")
    for cell, runs in cells:
        b = broke([r for r in runs if r["sample"]])
        cols = []
        for p in PHASES:
            v = [r["contact_to"][p] for r in b if r["contact_to"][p] is not None]
            cols.append(f"{q3(v)}; {len(v)}")
        print(f"| {cell} | {len(b)} | " + " | ".join(cols) + " |")
    print()


def ground(cells):
    print("## 2. How men gave ground before the break\n")
    print(
        "Sample runs with a break; every step back of a front man, up to the first side's "
        "break, summed over the runs. Driven back: a blow that did not wound pushed a steady "
        "man back (cause `Bout`). From fear: his alarm passed its threshold; the largest "
        "cause is the one named first.\n"
    )
    print("| Cell | Loser: driven back | from fear | Winner: driven back | from fear | The loser's steps back from fear, by largest cause |")
    print("|---|---|---|---|---|---|")
    for cell, runs in cells:
        b = broke([r for r in runs if r["sample"]])
        lk = sum(r["backs_knocked"][r["loser"]] for r in b)
        lf = sum(r["backs_fear"][r["loser"]] for r in b)
        wk = sum(r["backs_knocked"][1 - r["loser"]] for r in b)
        wf = sum(r["backs_fear"][1 - r["loser"]] for r in b)
        causes = {}
        for r in b:
            add(causes, r["fear_back_causes"][r["loser"]])
        print(
            f"| {cell} | {share(lk, lk + lf)} | {lf} | {share(wk, wk + wf)} | {wf} | {top(causes)} |"
        )
    print()


def turning(cells):
    print("## 3. How the loser's men turned\n")
    print(
        "Sample runs with a break; each man's first turn before the break. After a knock-back: "
        "he turned from giving ground, and what last put him there was a blow. After fear: "
        "what last put him there was his alarm. From his place: he turned straight from "
        "standing or going forward.\n"
    )
    print("| Cell | Men who turned | after a knock-back | after giving ground from fear | from his place | Turns by largest cause |")
    print("|---|---|---|---|---|---|")
    for cell, runs in cells:
        b = broke([r for r in runs if r["sample"]])
        k = {x: 0 for x in KINDS}
        causes = {}
        for r in b:
            add(k, r["first_turn_kinds"][r["loser"]])
            add(causes, r["turn_causes"][r["loser"]])
        n = sum(k.values())
        print(
            f"| {cell} | {n} | " + " | ".join(share(k[x], n) for x in KINDS) + f" | {top(causes)} |"
        )
    print()


def first_turner_rows(cell, runs):
    ft = [r["first_turner"] for r in runs if r["first_turner"]]
    n = len(ft)
    k = {x: sum(1 for f in ft if f["kind"] == x) for x in KINDS}
    front = sum(1 for f in ft if f["depth"] == 0)
    causes = {}
    for f in ft:
        causes[f["causes"][0]] = causes.get(f["causes"][0], 0) + 1
    back_for = [f["back_for_s"] for f in ft if f["back_for_s"] is not None]
    return (
        f"| {cell} | {n} | " + " | ".join(share(k[x], n) for x in KINDS)
        + f" | {share(front, n)} | {fmt(median([f['depth'] for f in ft]), 2)} | {top(causes, 3)} | {q3(back_for)} |"
    )


FT_HEAD = (
    "| Cell | Runs | after a knock-back | after giving ground from fear | from his place | "
    "in the front rank | median depth | largest cause of his turn | in the back step, s |\n"
    "|---|---|---|---|---|---|---|---|---|"
)


def first_turner(cells):
    print("## 4. The first man to turn\n")
    print(
        "Sample runs with a break: the loser's first man to turn, by how he turned, his depth "
        "at setup (0 the front rank, 1 the rear), the largest cause of his turn, and how long "
        "he had been giving ground when he turned.\n"
    )
    print(FT_HEAD)
    for cell, runs in cells:
        print(first_turner_rows(cell, broke([r for r in runs if r["sample"]])))
    print()
    print("## 5. Where the loser had lost nobody\n")
    print("The same, over the sample runs in which the loser broke with no man of its own down.\n")
    print(FT_HEAD)
    for cell, runs in cells:
        b = [r for r in broke([r for r in runs if r["sample"]]) if r["losses"][r["loser"]] == 0]
        print(first_turner_rows(cell, b))
    print()


def pattern3(cells):
    print("## 6. Pattern 3 again: the first tenth to turn\n")
    print(
        "Sample runs with a break. The depth of the first tenth of each broken body of the "
        "loser, as the scorer takes it (ties at one tick to the lower man number, so to the "
        "front rank) and with the men of a tied tick sharing the places left. The band is at "
        "least 0.6. Then how those men turned.\n"
    )
    print("| Cell | Runs | Depth as scored | With ties shared | Shared less scored, mean | " + " | ".join(KIND_WORDS[k] for k in KIND_WORDS) + " |")
    print("|---|---|---|---|---|" + "---|" * len(KIND_WORDS))
    for cell, runs in cells:
        b = [r for r in broke([r for r in runs if r["sample"]]) if r["pattern3"]]
        ds = [r["pattern3"]["depth"] for r in b]
        dt = [r["pattern3"]["depth_ties_shared"] for r in b]
        diff = sum(t - s for s, t in zip(ds, dt)) / len(b) if b else None
        k = {}
        for r in b:
            add(k, r["pattern3"]["kinds"])
        n = sum(k.values())
        print(
            f"| {cell} | {len(b)} | {q3(ds, 3)} | {q3(dt, 3)} | {fmt(diff, 3)} | "
            + " | ".join(share(k.get(x, 0), n) for x in KIND_WORDS) + " |"
        )
    print()


def stragglers(cells):
    print("## 7. The stragglers\n")
    print(
        "Sample runs with a break: men of the loser who turned before its collapse began, and "
        "how deep they stood.\n"
    )
    print("| Cell | Runs | Runs with stragglers | Stragglers | Their mean depth |")
    print("|---|---|---|---|---|")
    for cell, runs in cells:
        b = [r for r in broke([r for r in runs if r["sample"]]) if r["stragglers"]]
        w = [r for r in b if r["stragglers"]["count"] > 0]
        total = sum(r["stragglers"]["count"] for r in b)
        depth = sum(r["stragglers"]["count"] * r["stragglers"]["mean_depth"] for r in w)
        print(
            f"| {cell} | {len(b)} | {share(len(w), len(b))} | {total} | "
            f"{fmt(depth / total, 2) if total else '-'} |"
        )
    print()


def held(cells):
    print("## 8. Lines that held five minutes, then broke\n")
    print(
        "Every replayed run whose loser broke 300 s or more after contact (from the sample and "
        "beyond it), beside the sample's runs that broke sooner.\n"
    )
    print("### The first man to turn\n")
    print(FT_HEAD)
    for cell, runs in cells:
        h = [r for r in broke(runs) if r["held"]]
        e = [r for r in broke(runs) if r["sample"] and not r["held"]]
        print(first_turner_rows(f"{cell}, held", h))
        print(first_turner_rows(f"{cell}, broke early", e))
    print()
    print("### How the men turned, and pattern 3\n")
    print("| Cell | Runs | Loser's steps back driven by a blow | Turns after a knock-back | after fear | from his place | Pattern 3 depth as scored | with ties shared |")
    print("|---|---|---|---|---|---|---|---|")
    for cell, runs in cells:
        for label, group in (
            ("held", [r for r in broke(runs) if r["held"]]),
            ("broke early", [r for r in broke(runs) if r["sample"] and not r["held"]]),
        ):
            lk = sum(r["backs_knocked"][r["loser"]] for r in group)
            lf = sum(r["backs_fear"][r["loser"]] for r in group)
            k = {x: 0 for x in KINDS}
            for r in group:
                add(k, r["first_turn_kinds"][r["loser"]])
            n = sum(k.values())
            p3 = [r["pattern3"] for r in group if r["pattern3"]]
            print(
                f"| {cell}, {label} | {len(group)} | {share(lk, lk + lf)} | "
                + " | ".join(share(k[x], n) for x in KINDS)
                + f" | {q3([p['depth'] for p in p3], 3)} | {q3([p['depth_ties_shared'] for p in p3], 3)} |"
            )
    print()


SCREENED = ["tie.standoff_mm", "man.beside", "man.enemy_front", "bout.drive_back_pm"]


def screened(cells, table):
    print("## 9. A1's lines against each time\n")
    print(
        "Spearman's rho over the sample runs with a break, run by run (the ten seeds of a set "
        "share its values, so this is a direction, not a test), between the lines A1 found "
        "and the seconds from contact to each event of section 1. A run without the event is "
        "left out of that column.\n"
    )
    print("| Cell | Line | First bout | First man back | First knock-back | First turn | Break |")
    print("|---|---|---|---|---|---|---|")
    for cell, runs in cells:
        b = broke([r for r in runs if r["sample"]])
        for line in SCREENED:
            cols = []
            for p in PHASES:
                pairs = [
                    (float(table[r["draw"]][line]), r["contact_to"][p])
                    for r in b
                    if r["contact_to"][p] is not None
                ]
                rho = spearman([x for x, _ in pairs], [y for _, y in pairs])
                cols.append(f"{rho:+.2f}" if rho is not None else "-")
            print(f"| {cell} | `{line}` | " + " | ".join(cols) + " |")
    print()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--table")
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("files", nargs="*")
    a = ap.parse_args()
    if a.self_test:
        return self_test()
    if not (a.table and a.files):
        ap.error("needs --table and the genealogy files")
    with open(a.table) as f:
        table = {int(r["draw"]): r for r in csv.DictReader(f)}
    found = {}
    for path in a.files:
        m = re.fullmatch(r"genealogy-(pairwise|field)-r([0-3])\.jsonl", os.path.basename(path))
        if not m:
            raise SystemExit(f"{path}: not a genealogy file's name")
        found[(m.group(1), int(m.group(2)))] = path
    print("# A3: the genealogy of the collapses\n")
    print(
        "Printed by `tools/a3-genealogy.py` from `cannae-replay genealogy`. **Exploratory:** "
        "none of this was registered before the batch; none of it changes a verdict. A run "
        "is counted only if the scorer's own run gives its record again, the battle stepped "
        "by the replay has the same events, the samples it takes give the record's whole "
        "summary, and its copy of pattern 3 gives the record's depth and stragglers.\n"
    )
    print("| Cell | Runs replayed | not counted | in the sample | held five minutes, then broke |")
    print("|---|---|---|---|---|")
    cells = []
    for key in CELL_ORDER:
        if key not in found:
            continue
        runs, nc = load(found[key])
        cell = f"{NAME[key[0]]} rung {key[1]}"
        cells.append((cell, runs))
        print(
            f"| {cell} | {len(runs) + nc} | {nc} | {sum(1 for r in runs if r['sample'])} | "
            f"{sum(1 for r in broke(runs) if r['held'])} |"
        )
    print()
    phases(cells)
    ground(cells)
    turning(cells)
    first_turner(cells)
    pattern3(cells)
    stragglers(cells)
    held(cells)
    screened(cells, table)
    return 0


def self_test():
    assert median([1, 3, 2]) == 2 and median([1, 2, 3, 4]) == 2.5
    assert quantile([1, 2, 3, 4], 0.25) == 1.75
    assert abs(spearman([1, 2, 3, 4, 5], [2, 1, 4, 3, 5]) - 0.8) < 1e-12
    assert spearman([1, 2], [1, 2]) is None
    assert top({"A": 3, "B": 1}) == "A 75 %, B 25 %"
    assert top({}) == "-"
    d = {}
    add(d, {"x": 1})
    add(d, {"x": 2, "y": 1})
    assert d == {"x": 3, "y": 1}
    assert share(1, 4) == "1 (25 %)" and share(0, 0) == "0"
    print("self-test: all passed")
    return 0


if __name__ == "__main__":
    sys.exit(main())
