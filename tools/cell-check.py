#!/usr/bin/env python3
"""The field's cell check: the eight patterns' statistics of the nominal set on `base`, at the
scored cell (1.024 m) and at half of it (0.512 m). Its conditions are in
docs/slice0/cell-check.md, written before this script read a record.

    tools/cell-check.py DIR_1024 DIR_512 DIR_1024B [--seeds 30] [--score-dir DIR]

DIR_1024 and DIR_512 hold `field-r<rung>-base.jsonl` for rungs 0 to 3 with seeds 1 to 30, as
`cell --nominal yes --out DIR` wrote them. DIR_1024B holds seeds 31 to 60 at 1.024 m: the
control, in which nothing differs but the seeds.

For each rung this prints every statistic the scorer pools for a set (`metrics::verdicts` in
cannae-harness) for both groups of runs, with the band's verdict on each, and A: the chance
that a run of the second group gives a larger value than a run of the first, ties counted
half. It does so three times: for the half cell against the control's seeds at the scored
cell, on which the line is read; for the half cell against the same seeds at the scored cell;
and for the control.

Then it splits the sixty runs at 1.024 m into thirty and thirty at random, 1,000 times, the
four rungs of a seed kept together, and counts how often seeds alone move a statistic as far
over the four rungs, and change as many verdicts, as the half cell did.

With `--score-dir`, the verdicts are compared with the scorer's own, read from
`DIR/score-<group>-r<rung>.txt`: the output of `score` on the same records.
"""

import argparse
import bisect
import json
import random
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# `draw` of the nominal set (`draw::nominal`).
NOMINAL = 2**32 - 1
RUNGS = (0, 1, 2, 3)
# A statistic has moved when A is this far from 0.5 or farther, with at least FEWEST values
# in each group. Both are fixed in the conditions, as are the splits and their line.
MOVED = 0.15
FEWEST = 10
SPLITS = 1000
SPLIT_SEED = 20261111
RARE = 25


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


def larger(first, second):
    """The chance that a value of `second` is larger than a value of `first`, ties half."""
    if len(first) < FEWEST or len(second) < FEWEST:
        return None
    v = sorted(first)
    wins = 0.0
    for y in second:
        below = bisect.bisect_left(v, y)
        wins += below + 0.5 * (bisect.bisect_right(v, y) - below)
    return wins / (len(first) * len(second))


def has_moved(a):
    return a is not None and abs(a - 0.5) >= MOVED - 1e-12


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


def values_of(runs):
    return [[x for r in runs for x in get(r)] for _, _, get, _, _ in PER_RUN]


def verdicts(runs, b, values=None):
    """As `metrics::verdicts`: None where a pattern cannot be scored."""
    values = values_of(runs) if values is None else values
    pooled = [median(v) for v in values]
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


def load(directory, rung, first, last):
    """By seed: the records of seeds `first` to `last`; and the problems found."""
    path = Path(directory) / f"field-r{rung}-base.jsonl"
    if not path.exists():
        return {}, [f"{path}: no such file"]
    runs, problems = {}, []
    for n, line in enumerate(path.read_text().splitlines(), 1):
        try:
            r = json.loads(line)
        except json.JSONDecodeError:
            problems.append(f"{path}:{n}: not a record")
            continue
        ours = (r["engine"] == "Field" and r["rung"] == rung and r["variant"] == "base"
                and r["freeze_seed"] is None and r["draw"] == NOMINAL and not r["whole"])
        if not ours or not first <= r["seed"] <= last or r["seed"] in runs:
            problems.append(f"{path}:{n}: not one stopped run of the nominal set on base, "
                            f"seeds {first} to {last}")
            continue
        runs[r["seed"]] = r
    missing = [s for s in range(first, last + 1) if s not in runs]
    if missing:
        problems.append(f"{path}: INCOMPLETE, no run of seeds {missing}")
    return runs, problems


def num(x):
    if x is None:
        return "none"
    if x == float("inf"):
        return "never"
    return f"{x:.4g}"


def group_text(values, pooled, ok):
    if not values:
        return f"{0:>3} {'none':>8} {'':<20} {'':<4}"
    q = "[{}, {}]".format(*map(num, quartiles(values)))
    mark = "" if ok is None else "pass" if ok else "FAIL"
    return f"{len(values):>3} {num(pooled):>8} {q:<20} {mark:<4}"


def word(v):
    return "unscorable" if v is None else "pass" if v else "FAIL"


def share(k, m):
    return "none" if m == 0 else f"{k} of {m} ({100 * k / m:.1f} %)"


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


def table(title, names, groups, bands):
    """Prints one comparison in full: `groups[k][rung]` is the records of group k by seed.
    Returns how many verdicts changed, how many statistics moved, and each group's verdicts."""
    print(f"\n{'=' * 100}\n{title}\nA: the chance that a run of the second group gives a "
          "larger value than a run of the first, ties half.")
    moved, compared, changed, all_verdicts = [], 0, [], {}
    by_stat = [[] for _ in PER_RUN]
    for rung in RUNGS:
        records = [list(g[rung].values()) for g in groups]
        runs = [[r["summary"] for r in rs] for rs in records]
        vals = [values_of(rs) for rs in runs]
        got = [verdicts(rs, bands, v) for rs, v in zip(runs, vals)]
        all_verdicts[rung] = [g[0] for g in got]
        print(f"\nRung {rung}: {len(runs[0])} runs and {len(runs[1])} runs")
        for k in (0, 1):
            first = [r["loser"] for r in runs[k]]
            wall = sum(r["wall_ms"] for r in records[k]) / 1000
            sim = sum(r["sim_s"] for r in records[k])
            print(f"  {names[k]}: the Romans broke first in {first.count(0)}, the warband in "
                  f"{first.count(1)}, neither broke in {first.count(None)}; "
                  f"{wall:.0f} s of machine time for {sim} simulated s")
        head = f"{'n':>3} {'median':>8} {'[quartiles]':<20} {'ok':<4}"
        print(f"\n  {'statistic':<32} {'band':<12} {'first: ' + head}   "
              f"{'second: ' + head}   {'A':>5}")
        for i, (pattern, name, _, band, _) in enumerate(PER_RUN):
            a = larger(vals[0][i], vals[1][i])
            mark = "too few" if a is None else ""
            if a is not None:
                compared += 1
                by_stat[i].append(a)
                if has_moved(a):
                    mark = "MOVED"
                    moved.append(f"rung {rung}, {name} (A {a:.3f})")
            texts = [group_text(vals[k][i], got[k][1][i], got[k][2][i]) for k in (0, 1)]
            print(f"  {f'{pattern} {name}':<32} {band(bands):<12} {'':<7}{texts[0]}   "
                  f"{'':<8}{texts[1]}   {'' if a is None else f'{a:5.3f}'} {mark}")
        for k in (0, 1):
            (early, never, n), (lf, f, lr, ro) = got[k][3], got[k][4]
            print(f"  {names[k]}: 6 broke within {bands['holds_early_s']:g} s of contact "
                  f"{share(early, n)}, at most {100 * bands['holds_max_early']:g} %; "
                  f"unbroken {bands['holds_never_s']:g} s after it {share(never, n)}, "
                  f"at most {100 * bands['holds_max_never']:g} %")
            print(f"  {names[k]}: 8 leaders' share of hits while formed {share(lf, f)}, "
                  f"after the break {share(lr, ro)}")
        print(f"\n  {'pattern':<34} {'first':<11} {'second':<11}")
        for k, name in enumerate(PATTERNS):
            v = [g[0][k] for g in got]
            flag = "" if v[0] == v[1] else "CHANGED"
            if flag:
                changed.append(f"rung {rung}, {name}: {word(v[0])} to {word(v[1])}")
            print(f"  {name:<34} {word(v[0]):<11} {word(v[1]):<11} {flag}")
    print(f"\nMoved: {len(moved)} of {compared} comparisons.")
    for m in moved:
        print(f"  {m}")
    print(f"Verdicts changed: {len(changed)} of {8 * len(RUNGS)}.")
    for m in changed:
        print(f"  {m}")
    return len(changed), len(moved), all_verdicts, [mean_move(a) for a in by_stat]


def mean_move(a_by_rung):
    """How far a statistic's A lies from 0.5, as a mean over the rungs that have it."""
    return sum(a - 0.5 for a in a_by_rung) / len(a_by_rung) if a_by_rung else 0.0


def counts(first, second, bands):
    """Between two groups of summaries by rung: verdicts changed, statistics moved, and each
    statistic's mean move over the rungs."""
    changed = moved = 0
    by_stat = [[] for _ in PER_RUN]
    for rung in RUNGS:
        va, vb = values_of(first[rung]), values_of(second[rung])
        a, b = verdicts(first[rung], bands, va)[0], verdicts(second[rung], bands, vb)[0]
        changed += sum(1 for x, y in zip(a, b) if x != y)
        for i, (x, y) in enumerate(zip(va, vb)):
            sup = larger(x, y)
            if sup is not None:
                by_stat[i].append(sup)
                moved += has_moved(sup)
    return changed, moved, [mean_move(a) for a in by_stat]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("dir_1024")
    ap.add_argument("dir_512")
    ap.add_argument("dir_1024b")
    ap.add_argument("--seeds", type=int, default=30)
    ap.add_argument("--score-dir")
    args = ap.parse_args()
    n = args.seeds
    bands = tomllib.loads((ROOT / "scenarios/prereg.toml").read_text())
    problems = []
    groups = {}
    for name, directory, first in (("1024", args.dir_1024, 1), ("512", args.dir_512, 1),
                                   ("1024b", args.dir_1024b, n + 1)):
        groups[name] = {}
        for rung in RUNGS:
            groups[name][rung], found = load(directory, rung, first, first + n - 1)
            problems += found

    print(f"The field's cell check: the nominal set on base, {n} runs a group.")
    print(f"Moved: A at or below {0.5 - MOVED:.2f}, or at or above {0.5 + MOVED:.2f}, with "
          f"at least {FEWEST} values in each group.")
    half = table(f"THE HALF CELL: 0.512 m, seeds 1 to {n} (second) against 1.024 m, seeds "
                 f"{n + 1} to {2 * n} (first). The line is read on this one.",
                 ("1.024 m", "0.512 m"), (groups["1024b"], groups["512"]), bands)
    same = table(f"THE SAME SEEDS: 0.512 m (second) against 1.024 m (first), seeds 1 to {n}. "
                 "Beside the line, not the line.",
                 ("1.024 m", "0.512 m"), (groups["1024"], groups["512"]), bands)
    control = table(f"THE CONTROL: 1.024 m, seeds {n + 1} to {2 * n} (second) against "
                    f"seeds 1 to {n} (first)",
                    (f"seeds 1 to {n}", f"seeds {n + 1} to {2 * n}"),
                    (groups["1024"], groups["1024b"]), bands)
    shown = (("half cell", half), ("same seeds", same), ("control", control))

    print(f"\n{'=' * 100}\nSEEDS ALONE: {SPLITS} random splits of the {2 * n} runs at "
          f"1.024 m into {n} and {n}, the four rungs of a seed kept together")
    pool = {r: {**groups["1024"][r], **groups["1024b"][r]} for r in RUNGS}
    seeds = sorted(pool[0])
    if any(sorted(pool[r]) != list(range(1, 2 * n + 1)) for r in RUNGS):
        print("  not made: the runs at 1.024 m are incomplete")
    else:
        rng = random.Random(SPLIT_SEED)
        got = []
        for _ in range(SPLITS):
            rng.shuffle(seeds)
            a = {r: [pool[r][s]["summary"] for s in seeds[:n]] for r in RUNGS}
            b = {r: [pool[r][s]["summary"] for s in seeds[n:]] for r in RUNGS}
            got.append(counts(a, b, bands))
        print("\n  Each statistic over the four rungs: the mean of A less 0.5; and the splits "
              "in which seeds alone\n  move it as far as the half cell did or farther, either "
              "way.")
        print(f"  {'statistic':<32} {'half cell':>10} {'same seeds':>11} {'control':>9} "
              f"{'splits':>14}")
        for i, (pattern, name, _, _, _) in enumerate(PER_RUN):
            reach = sum(1 for g in got if abs(g[2][i]) >= abs(half[3][i]) - 1e-12)
            print(f"  {f'{pattern} {name}':<32} {half[3][i]:>+10.3f} {same[3][i]:>+11.3f} "
                  f"{control[3][i]:>+9.3f} {reach:>6} of {SPLITS}")

        def reached(values, mine):
            return sum(1 for x in values if x >= mine - 1e-12)

        def line(what, values, pick, form):
            v = sorted(values)
            mine = [pick(c) for _, c in shown]
            print(f"\n  {what}: seeds alone give a median of {form(v[SPLITS // 2])}, and at "
                  f"most {form(v[SPLITS * 95 // 100 - 1])} in 95 splits of 100")
            for (label, _), x in zip(shown, mine):
                print(f"    {label:<11} {form(x):>6}; splits that reach it: "
                      f"{reached(v, x)} of {SPLITS}")
            return reached(v, mine[0])

        reach_move = line("The largest of the eleven moves",
                          [max(abs(m) for m in g[2]) for g in got],
                          lambda c: max(abs(m) for m in c[3]), lambda x: f"{x:.3f}")
        reach_verdicts = line("Verdicts changed", [g[0] for g in got],
                              lambda c: c[0], str)
        line("Comparisons marked MOVED (no line)", [g[1] for g in got], lambda c: c[1], str)
        rare = reach_move < RARE or reach_verdicts < RARE
        print(f"\n  The line: fewer than {RARE} of the {SPLITS} splits reach the half cell's "
              "largest move, or its count of changed verdicts.\n  THE CELL MATTERS AT THE "
              f"CENTRE: {'YES' if rare else 'NO'}")

    if args.score_dir:
        disagree, checked = [], 0
        for name, ours, k in (("1024", control, 0), ("512", half, 1), ("1024b", control, 1)):
            for rung in RUNGS:
                mine = ours[2][rung][k]
                path = Path(args.score_dir) / f"score-{name}-r{rung}.txt"
                theirs = scorer_verdicts(path) if path.exists() else ["missing"] * 8
                for k in range(8):
                    checked += 1
                    if theirs[k] != mine[k]:
                        disagree.append(f"rung {rung}, group {name}, {PATTERNS[k]}: the "
                                        f"scorer says {theirs[k]}, this script {mine[k]}")
        print(f"\nThe scorer's own verdicts: {checked - len(disagree)} of {checked} are "
              "this script's.")
        for m in disagree:
            print(f"  {m}")
        problems += disagree
    if problems:
        print(f"\nPROBLEMS: {len(problems)}")
        for p in problems:
            print(f"  {p}")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
