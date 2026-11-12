#!/usr/bin/env python3
"""The replays' lines, counted (docs/slice0/report-outline.md: section 5, "How the discs'
passage is shown in the relief", items 2 and 3; section 6, "A century and its centurion: the
count").

    tools/s09-replays.py reliefs FILE [FILE ..]
    tools/s09-replays.py leaders FILE [FILE ..]
    tools/s09-replays.py reliefs|leaders --format-only FILE [FILE ..]
    tools/s09-replays.py --self-test

Each FILE holds the lines `cannae-replay reliefs` or `cannae-replay leaders` wrote for one
cell. This script reads those lines alone and replays nothing. Every count it prints is defined
in the outline, and the windows are the outline's: 5 s and 30 s.

It prints outcomes of scored runs, so it is run after the scoring. With `--format-only` it
prints how many lines it could read and nothing they hold: that is for trying it on real files
beforehand.
"""
import json
import os
import sys

# `relief` and `backed` run on every fourth set; the top-up's sets are the others, and their
# reliefs are reported apart (cannae_score::report::RELIEF_EVERY).
RELIEF_EVERY = 4
# "Within 5 s" and "within 30 s" of a leader's fall: fixed in the outline before any record
# was read.
WINDOWS = (5.0, 30.0)
# Side A is the Roman side (cannae_score::report::ROMANS).
SIDES = {0: "Romans", 1: "warband"}
GROUPS = {"leader": "leaders", "file": "file leaders"}
TIMES = (
    ("up_after_s", "the fresh century had the front after the order"),
    ("halted_after_s", "the tired century halted after that"),
    ("back_after_s", "nine in ten were back in their places after the halt"),
)


def median(values):
    v = sorted(values)
    if not v:
        return None
    mid = len(v) // 2
    return v[mid] if len(v) % 2 else (v[mid - 1] + v[mid]) / 2


def read(path):
    """The lines of one file: (those counted, runs not counted, lines that cannot be read)."""
    counted, not_counted, unreadable = [], 0, 0
    with open(path) as f:
        for line in f:
            if not line.strip():
                continue
            try:
                r = json.loads(line)
                r["draw"], r["seed"]
            except (ValueError, KeyError, TypeError):
                unreadable += 1
                continue
            if "not_counted" in r:
                not_counted += 1
            else:
                counted.append(r)
    return counted, not_counted, unreadable


def reliefs(lines):
    """One group of reliefs counted: the first 50 sets', or the top-up's."""
    runs = {}
    for r in lines:
        runs.setdefault((r["draw"], r["seed"]), []).append(r)
    out = {"reliefs": len(lines), "runs": len(runs), "times": {}}
    for key, _ in TIMES:
        got = [r[key] for r in lines if r[key] is not None]
        out["times"][key] = (len(got), median(got), max(got, default=None))
    astray = [r["astray_at_halt"] for r in lines if r["astray_at_halt"] is not None]
    out["astray"] = (len(astray), median(astray), max(astray, default=None))
    out["pressed_jammed"] = {
        (p, j): sum(1 for r in lines if r["pressed"] == p and r["jammed"] == j)
        for p in (True, False)
        for j in (True, False)
    }
    out["unfinished"] = {
        why: sum(1 for r in lines if r["unfinished"] == why) for why in ("broke", "ended")
    }
    # A run with a jammed relief; one with a pressed relief and none jammed; the rest. A run's
    # lines all carry the same word for its twin.
    twins = {g: {t: 0 for t in ("better", "worse", "same", "none")}
             for g in ("jammed", "pressed", "rest")}
    for run in runs.values():
        group = ("jammed" if any(r["jammed"] for r in run)
                 else "pressed" if any(r["pressed"] for r in run) else "rest")
        twins[group][run[0]["twin"]] += 1
    out["twins"] = twins
    return out


def leaders(lines):
    """The falls counted, for each side and for leaders and file leaders apart."""
    out = {}
    for side in SIDES:
        for group in GROUPS:
            falls = [r for r in lines if r["side"] == side and r["group"] == group]
            after = [r["break_after_s"] for r in falls]
            later = [t for t in after if t is not None and t > WINDOWS[1]]
            with_delay = [r for r in falls if r["succeed_after_s"] is not None]
            out[(side, group)] = {
                "falls": len(falls),
                "within": [sum(1 for t in after if t is not None and t <= w) for w in WINDOWS],
                "later": (len(later), median(later)),
                "never": sum(1 for t in after if t is None),
                "in_flight": median([r["in_flight"] for r in falls]),
                # The break came before a successor could have the place: only a body with a
                # succession has the delay, which is each set's own.
                "before_successor": (
                    sum(1 for r in with_delay
                        if r["break_after_s"] is not None
                        and r["break_after_s"] < r["succeed_after_s"]),
                    len(with_delay),
                ),
            }
    return out


def num(x, unit=""):
    return "none" if x is None else f"{x:.2f}{unit}"


def share(k, n):
    return f"{k} of {n}" + (f" ({100 * k / n:.0f} %)" if n else "")


def print_reliefs(lines):
    groups = (
        ("the first 50 sets", [r for r in lines if r["draw"] % RELIEF_EVERY == 0]),
        ("the top-up", [r for r in lines if r["draw"] % RELIEF_EVERY != 0]),
    )
    for name, of in groups:
        c = reliefs(of)
        print(f"  {name}: {c['reliefs']} reliefs seen, in {c['runs']} runs")
        if not of:
            continue
        for key, words in TIMES:
            n, mid, longest = c["times"][key]
            print(f"    {words}: median {num(mid, ' s')}, longest {num(longest, ' s')} "
                  f"({n} of {c['reliefs']} reliefs)")
        n, mid, most = c["astray"]
        print(f"    men out of their places at the halt, of those in them at the order: "
              f"median {num(mid)}, most {num(most)} ({n} of {c['reliefs']} reliefs)")
        pj = c["pressed_jammed"]
        print(f"    pressed and jammed {pj[(True, True)]}, pressed and not jammed "
              f"{pj[(True, False)]}, jammed and not pressed {pj[(False, True)]}, neither "
              f"{pj[(False, False)]}")
        print(f"    unfinished: a century broke before the halt in {c['unfinished']['broke']}, "
              f"the run ended in {c['unfinished']['ended']}")
        print("    the runs beside their backed twins (better, worse, the same; no twin on "
              "disk):")
        for group, words in (("jammed", "with a jammed relief"),
                             ("pressed", "with a pressed relief and none jammed"),
                             ("rest", "the rest")):
            t = c["twins"][group]
            print(f"      {words}: {t['better']}, {t['worse']}, {t['same']}; {t['none']}")


def print_leaders(lines):
    c = leaders(lines)
    for (side, group), g in c.items():
        n = g["falls"]
        print(f"  {SIDES[side]}, {GROUPS[group]}: {n} falls in bodies not yet broken")
        if not n:
            continue
        later, mid = g["later"]
        print(f"    the body broke within {WINDOWS[0]:.0f} s after {share(g['within'][0], n)}, "
              f"within {WINDOWS[1]:.0f} s after {share(g['within'][1], n)}, later after "
              f"{later} (median {num(mid, ' s')}), never after {g['never']}")
        print(f"    the share of the body's standing men in flight at the fall: median "
              f"{num(g['in_flight'])}")
        k, of = g["before_successor"]
        if of:
            print(f"    the body broke before a successor could have the place, by the set's "
                  f"own delay: {share(k, of)}")


def self_test():
    def relief(draw, seed, pressed, jammed, twin, up=10.0, halted=20.0, back=5.0, astray=2,
               unfinished=None):
        return {"draw": draw, "seed": seed, "relief": 0, "ordered_s": 1200.0,
                "placed_at_order": [80, 80], "up_after_s": up, "halted_after_s": halted,
                "astray_at_halt": astray, "back_after_s": back, "pressed": pressed,
                "jammed": jammed, "unfinished": unfinished, "twin": twin}

    lines = [
        # Run (0, 1): two reliefs, one jammed. It is a run with a jammed relief.
        relief(0, 1, True, True, "better", up=10.0, halted=30.0, back=4.0, astray=6),
        relief(0, 1, False, False, "better", up=12.0, halted=20.0, back=2.0, astray=0),
        # Run (0, 2): pressed, not jammed, and never halted because a century broke.
        relief(0, 2, True, False, "worse", up=20.0, halted=None, back=None, astray=None,
               unfinished="broke"),
        # Run (4, 1): neither; the run ended before the fresh century was up.
        relief(4, 1, False, False, "same", up=None, halted=None, back=None, astray=None,
               unfinished="ended"),
        # Run (8, 3): neither, with no twin on disk.
        relief(8, 3, False, False, "none", up=14.0, halted=40.0, back=None, astray=1),
        # A set of the top-up: counted apart.
        relief(5, 1, False, True, "worse"),
    ]
    stride = reliefs([r for r in lines if r["draw"] % RELIEF_EVERY == 0])
    top = reliefs([r for r in lines if r["draw"] % RELIEF_EVERY != 0])
    none = {"better": 0, "worse": 0, "same": 0, "none": 0}
    want_stride = {
        "reliefs": 5, "runs": 4,
        "times": {"up_after_s": (4, 13.0, 20.0), "halted_after_s": (3, 30.0, 40.0),
                  "back_after_s": (2, 3.0, 4.0)},
        "astray": (3, 1, 6),
        "pressed_jammed": {(True, True): 1, (True, False): 1, (False, True): 0,
                           (False, False): 3},
        "unfinished": {"broke": 1, "ended": 1},
        "twins": {"jammed": dict(none, better=1), "pressed": dict(none, worse=1),
                  "rest": dict(none, same=1, none=1)},
    }
    checks = [("the reliefs of the first 50 sets", stride, want_stride),
              ("the top-up's reliefs", (top["reliefs"], top["runs"], top["twins"]["jammed"]),
               (1, 1, dict(none, worse=1))),
              ("no relief at all", reliefs([])["times"]["up_after_s"], (0, None, None))]

    def fall(side, group, after, flight=0.1, delay=None):
        return {"draw": 0, "seed": 1, "body": 0, "side": side, "group": group, "fall_s": 100.0,
                "break_after_s": after, "in_flight": flight, "succeed_after_s": delay}

    falls = [
        # Roman leaders: the break 3 s after (inside both windows, before a successor of 4 s),
        # at 5 s exactly (inside), at 5.05 s (outside the first, and not before a successor of
        # 5.05 s), at 30 s exactly, at 31 and 91 s, and never.
        fall(0, "leader", 3.0, 0.0, 4.0),
        fall(0, "leader", 5.0, 0.2, 2.0),
        fall(0, "leader", 5.05, 0.4, 5.05),
        fall(0, "leader", 30.0, 0.6, 15.0),
        fall(0, "leader", 31.0, 0.8, 15.0),
        fall(0, "leader", 91.0, 1.0, 15.0),
        fall(0, "leader", None, 0.3, 15.0),
        # Roman file leaders get the same window.
        fall(0, "file", 1.0, 0.5, 2.0),
        fall(0, "file", None, 0.7, 2.0),
        # The warband has no succession.
        fall(1, "leader", 2.0, 0.9),
    ]
    got = leaders(falls)
    checks += [
        ("Roman leaders", got[(0, "leader")],
         {"falls": 7, "within": [2, 4], "later": (2, 61.0), "never": 1, "in_flight": 0.4,
          "before_successor": (1, 7)}),
        ("Roman file leaders", got[(0, "file")],
         {"falls": 2, "within": [1, 1], "later": (0, None), "never": 1, "in_flight": 0.6,
          "before_successor": (1, 2)}),
        ("the warband's leaders", got[(1, "leader")],
         {"falls": 1, "within": [1, 1], "later": (0, None), "never": 0, "in_flight": 0.9,
          "before_successor": (0, 0)}),
        ("the warband's file leaders", got[(1, "file")]["falls"], 0),
        ("medians", (median([]), median([3]), median([4, 1]), median([9, 1, 5])),
         (None, 3, 2.5, 5)),
    ]
    bad = [(what, have, want) for what, have, want in checks if have != want]
    for what, have, want in bad:
        print(f"self-test: {what}: got {have}, not {want}")
    print(f"self-test: {len(checks) - len(bad)} of {len(checks)} counts as expected")
    return 1 if bad else 0


def main():
    args = sys.argv[1:]
    if args == ["--self-test"]:
        return self_test()
    format_only = "--format-only" in args
    if format_only:
        args.remove("--format-only")
    if len(args) < 2 or args[0] not in ("reliefs", "leaders"):
        print(__doc__)
        return 2
    what, problems = args[0], 0
    needs = (("relief", "pressed", "jammed", "unfinished", "twin", "astray_at_halt")
             + tuple(k for k, _ in TIMES) if what == "reliefs"
             else ("side", "group", "break_after_s", "in_flight", "succeed_after_s"))
    for path in args[1:]:
        counted, not_counted, unreadable = read(path)
        other = [r for r in counted if any(k not in r for k in needs)]
        problems += unreadable + len(other)
        print(f"{os.path.basename(path)}: {len(counted)} lines read, {not_counted} runs not "
              f"counted, {unreadable} lines that cannot be read, {len(other)} of another kind")
        if format_only or other:
            continue
        (print_reliefs if what == "reliefs" else print_leaders)(counted)
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())
