#!/usr/bin/env python3
"""The field's cell check: the eight patterns' statistics of the nominal set on `base`, at the
scored cell (1.024 m) and at half of it (0.512 m). Its conditions are in
docs/slice0/cell-check.md, written before this script read a record.

    tools/cell-check.py DIR_1024 DIR_512 [--seeds 30] [--score-dir DIR]

Each directory holds `field-r<rung>-base.jsonl` for rungs 0 to 3, as
`cell --nominal yes --out DIR` wrote them. For each rung this prints every statistic the
scorer pools for a set (`metrics::verdicts` in cannae-harness), at both cells, with the
band's verdict on each, and A: the chance that a run at 0.512 m gives a larger value than a
run at 1.024 m, ties counted half.

With `--score-dir`, the eight verdicts are compared with the scorer's own, read from
`DIR/score-<cell>-r<rung>.txt`: the output of `score` on the same records.
"""

import argparse
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# `draw` of the nominal set (`draw::nominal`).
NOMINAL = 2**32 - 1
RUNGS = (0, 1, 2, 3)
CELLS = ("1024", "512")
# A statistic has moved when A is this far from 0.5 or farther, with at least FEWEST values
# at each cell. Both are fixed in the conditions.
MOVED = 0.15
FEWEST = 10


def median(values):
    """The scorer's median: the mean of the middle two for an even count."""
    v = sorted(values)
    if not v:
        return None
    m = len(v) // 2
    return v[m] if len(v) % 2 else (v[m - 1] + v[m]) / 2


def quartiles(values):
    """For display only."""
    v = sorted(values)

    def at(q):
        x = q * (len(v) - 1)
        i = int(x)
        if i + 1 >= len(v) or x == i or v[i] == v[i + 1]:
            return v[i]
        # "Never" is infinite: a quartile that reaches into it is never.
        return v[i + 1] if v[i + 1] == float("inf") else v[i] + (x - i) * (v[i + 1] - v[i])

    return at(0.25), at(0.75)


def larger(coarse, fine):
    """The chance that a value of `fine` is larger than a value of `coarse`, ties half."""
    if len(coarse) < FEWEST or len(fine) < FEWEST:
        return None
    wins = sum(1.0 if y > x else 0.5 if y == x else 0.0 for x in coarse for y in fine)
    return wins / (len(coarse) * len(fine))


def opt(x):
    return [] if x is None else [x]


def at_least(key):
    return lambda x, b: x >= b[key]


def within(key):
    return lambda x, b: b[key][0] <= x <= b[key][1]


# Pattern, name, one run's values, the band as text, the band's test of the pooled value.
PER_RUN = [
    (1, "formed phase, s", lambda r: [r["formed_s"]],
     lambda b: f">= {b['standoff_min_formed_s']:g}", at_least("standoff_min_formed_s")),
    (1, "median gap while formed, m", lambda r: opt(r["median_gap_m"]),
     lambda b: f">= {b['standoff_min_median_gap_m']:g}", at_least("standoff_min_median_gap_m")),
    (2, "burstiness", lambda r: opt(r["burstiness"]),
     lambda b: f">= {b['pulses_min_burstiness']:g}", at_least("pulses_min_burstiness")),
    (2, "share of quiet bins", lambda r: opt(r["lull_share"]),
     lambda b: "{:g} to {:g}".format(*b["pulses_lull_share"]), within("pulses_lull_share")),
    (3, "depth of the first to turn", lambda r: opt(r["first_turn_depth"]),
     lambda b: f">= {b['rear_first_min_depth']:g}", at_least("rear_first_min_depth")),
    (4, "spread, Spearman's rho", lambda r: opt(r["spread_rho"]),
     lambda b: f">= {b['spread_min_rho']:g}", at_least("spread_min_rho")),
    (4, "spread, s for 80 % of bins", lambda r: opt(r["spread_s"]),
     lambda b: "{:g} to {:g}".format(*b["spread_s"]), within("spread_s")),
    (5, "loser's hits after the break", lambda r: opt(r["loser_after_break"]),
     lambda b: f">= {b['asymmetry_min_after_break']:g}", at_least("asymmetry_min_after_break")),
    (6, "time to break, s (never: last)",
     lambda r: [float("inf") if r["time_to_break_s"] is None else r["time_to_break_s"]],
     lambda b: "not banded", None),
    (7, "hits per front file per hour",
     lambda r: [x for x in r["hits_per_file_hour"] if x is not None],
     lambda b: "{:g} to {:g}".format(*b["clock_hits_per_file_hour"]),
     within("clock_hits_per_file_hour")),
    (7, "lethality per bout", lambda r: opt(r["lethality_per_bout"]),
     lambda b: "{:g} to {:g}".format(*b["clock_lethality_per_bout"]),
     within("clock_lethality_per_bout")),
]
PATTERNS = [
    "1 standoff", "2 pulses and lulls", "3 rear-first collapse", "4 lateral spread",
    "5 casualty asymmetry", "6 holds, then breaks (band only)", "7 casualty clock",
    "8 centurion gradient",
]


def verdicts(runs, b):
    """As `metrics::verdicts`: None where a pattern cannot be scored."""
    pooled = [median([x for r in runs for x in get(r)]) for _, _, get, _, _ in PER_RUN]
    ok = [None if x is None or test is None else test(x, b)
          for x, (_, _, _, _, test) in zip(pooled, PER_RUN)]

    def both(i, j):
        return None if ok[i] is None or ok[j] is None else ok[i] and ok[j]

    n = len(runs)
    times = [r["time_to_break_s"] for r in runs]
    early = sum(1 for t in times if t is not None and t < b["holds_early_s"])
    never = sum(1 for t in times if t is None or t > b["holds_never_s"])
    p6 = None if n == 0 else (early / n <= b["holds_max_early"]
                              and never / n <= b["holds_max_never"])
    # The clock needs a pooled rate; a lethality that cannot be pooled then fails it.
    p7 = None if ok[9] is None else ok[9] and ok[10] is True
    lf, f, lr, ro = (sum(r[k] for r in runs) for k in
                     ("leader_hits_formed", "hits_formed", "leader_hits_rout", "hits_rout"))
    p8 = lf / f > lr / ro if f > 0 and ro > 0 else None
    return ([both(0, 1), both(2, 3), ok[4], both(5, 6), ok[7], p6, p7, p8],
            pooled, ok, (early, never, n), (lf, f, lr, ro))


def load(directory, rung, seeds):
    """The summaries of seeds 1 to `seeds`, the problems found, and the wall and simulated s."""
    path = Path(directory) / f"field-r{rung}-base.jsonl"
    if not path.exists():
        return [], [f"{path}: no such file"], (0.0, 0)
    runs, problems = {}, []
    for n, line in enumerate(path.read_text().splitlines(), 1):
        try:
            r = json.loads(line)
        except json.JSONDecodeError:
            problems.append(f"{path}:{n}: not a record")
            continue
        ours = (r["engine"] == "Field" and r["rung"] == rung and r["variant"] == "base"
                and r["freeze_seed"] is None and r["draw"] == NOMINAL and not r["whole"])
        if not ours or not 1 <= r["seed"] <= seeds or r["seed"] in runs:
            problems.append(f"{path}:{n}: not one stopped run of the nominal set on base, "
                            f"seeds 1 to {seeds}")
            continue
        runs[r["seed"]] = r
    missing = [s for s in range(1, seeds + 1) if s not in runs]
    if missing:
        problems.append(f"{path}: INCOMPLETE, no run of seeds {missing}")
    kept = [runs[s] for s in sorted(runs)]
    cost = (sum(r["wall_ms"] for r in kept) / 1000, sum(r["sim_s"] for r in kept))
    return [r["summary"] for r in kept], problems, cost


def num(x):
    if x is None:
        return "none"
    if x == float("inf"):
        return "never"
    return f"{x:.4g}"


def cell_text(values, pooled, ok):
    if not values:
        return f"{0:>3} {'none':>8} {'':<20} {'':<4}"
    q = "[{}, {}]".format(*map(num, quartiles(values)))
    mark = "" if ok is None else "pass" if ok else "FAIL"
    return f"{len(values):>3} {num(pooled):>8} {q:<20} {mark:<4}"


def word(v):
    return "unscorable" if v is None else "pass" if v else "FAIL"


def scorer_verdicts(path):
    """The eight verdicts `score` printed for the one set in a file of its output."""
    out = {}
    for line in Path(path).read_text().splitlines():
        m = re.match(r"^\s{2}([1-8]) .*?\s+(\d+)\s+(\d+)\s+(\d+)\s*$", line)
        if m:
            counts = tuple(int(m.group(k)) for k in (2, 3, 4))
            out[int(m.group(1))] = {(1, 0, 0): True, (0, 1, 0): False, (0, 0, 1): None}.get(
                counts, "not one set")
    return [out.get(k, "missing") for k in range(1, 9)]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dir_1024")
    ap.add_argument("dir_512")
    ap.add_argument("--seeds", type=int, default=30)
    ap.add_argument("--score-dir")
    args = ap.parse_args()
    bands = tomllib.loads((ROOT / "scenarios/prereg.toml").read_text())
    dirs = dict(zip(CELLS, (args.dir_1024, args.dir_512)))
    problems, moved, compared, changed, disagree, checked = [], [], 0, [], [], 0

    print(f"The field's cell check: the nominal set on base, seeds 1 to {args.seeds}, "
          "at 1.024 m and at 0.512 m.")
    print("A: the chance that a run at 0.512 m gives a larger value than one at 1.024 m, "
          f"ties half.\nMoved: A at or below {0.5 - MOVED:.2f}, or at or above "
          f"{0.5 + MOVED:.2f}, with at least {FEWEST} values at each cell.")
    for rung in RUNGS:
        runs, got = {}, {}
        for c in CELLS:
            runs[c], found, cost = load(dirs[c], rung, args.seeds)
            problems += found
            got[c] = verdicts(runs[c], bands) + (cost,)
        print(f"\nRung {rung}: {len(runs['1024'])} runs at 1.024 m, "
              f"{len(runs['512'])} at 0.512 m")
        for c, label in zip(CELLS, ("1.024 m", "0.512 m")):
            first = [r["loser"] for r in runs[c]]
            wall, sim = got[c][5]
            print(f"  {label}: the Romans broke first in {first.count(0)}, the warband in "
                  f"{first.count(1)}, neither broke in {first.count(None)}; "
                  f"{wall:.0f} s of machine time for {sim} simulated s")
        head = f"{'n':>3} {'median':>8} {'[quartiles]':<20} {'ok':<4}"
        print(f"\n  {'statistic':<32} {'band':<12} {'1.024 m: ' + head}   "
              f"{'0.512 m: ' + head}   {'A':>5}")
        for i, (pattern, name, get, band, _) in enumerate(PER_RUN):
            values = {c: [x for r in runs[c] for x in get(r)] for c in CELLS}
            a = larger(values["1024"], values["512"])
            mark = ""
            if a is None:
                mark = "too few"
            else:
                compared += 1
                if abs(a - 0.5) >= MOVED - 1e-12:
                    mark = "MOVED"
                    moved.append(f"rung {rung}, {name} (A {a:.3f})")
            texts = [cell_text(values[c], got[c][1][i], got[c][2][i]) for c in CELLS]
            print(f"  {f'{pattern} {name}':<32} {band(bands):<12} {'':<9}{texts[0]}   "
                  f"{'':<9}{texts[1]}   {'' if a is None else f'{a:5.3f}'} {mark}")
        for c, label in zip(CELLS, ("1.024 m", "0.512 m")):
            (early, never, n), (lf, f, lr, ro) = got[c][3], got[c][4]
            share = lambda k, m: "none" if m == 0 else f"{k} of {m} ({100 * k / m:.1f} %)"
            print(f"  {label}: 6 broke within {bands['holds_early_s']:g} s of contact "
                  f"{share(early, n)}, at most {100 * bands['holds_max_early']:g} %; "
                  f"unbroken {bands['holds_never_s']:g} s after it {share(never, n)}, "
                  f"at most {100 * bands['holds_max_never']:g} %")
            print(f"  {label}: 8 leaders' share of hits while formed {share(lf, f)}, "
                  f"after the break {share(lr, ro)}")
        print(f"\n  {'pattern':<34} {'1.024 m':<11} {'0.512 m':<11}")
        for k, name in enumerate(PATTERNS):
            v = [got[c][0][k] for c in CELLS]
            flag = "" if v[0] == v[1] else "CHANGED"
            if flag:
                changed.append(f"rung {rung}, {name}: {word(v[0])} to {word(v[1])}")
            print(f"  {name:<34} {word(v[0]):<11} {word(v[1]):<11} {flag}")
        if args.score_dir:
            for c in CELLS:
                path = Path(args.score_dir) / f"score-{c}-r{rung}.txt"
                theirs = scorer_verdicts(path) if path.exists() else ["missing"] * 8
                for k in range(8):
                    checked += 1
                    if theirs[k] != got[c][0][k]:
                        disagree.append(f"rung {rung}, cell {c}, {PATTERNS[k]}: the scorer "
                                        f"says {theirs[k]}, this script {got[c][0][k]}")

    print(f"\nMoved: {len(moved)} of {compared} comparisons.")
    for m in moved:
        print(f"  {m}")
    print(f"Verdicts changed: {len(changed)} of {8 * len(RUNGS)}.")
    for m in changed:
        print(f"  {m}")
    if args.score_dir:
        print(f"The scorer's own verdicts: {checked - len(disagree)} of {checked} are "
              "this script's.")
        for m in disagree:
            print(f"  {m}")
    if problems:
        print(f"\nPROBLEMS WITH THE RECORDS: {len(problems)}")
        for p in problems:
            print(f"  {p}")
    return 1 if problems or disagree else 0


if __name__ == "__main__":
    sys.exit(main())
