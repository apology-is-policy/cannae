#!/usr/bin/env python3
"""The warband beside an open flank: the count (docs/slice0/report-outline.md, section 6).

    tools/s09-open-flank.py RECORDS_DIR [--freeze-seed F] [--format-only]
    tools/s09-open-flank.py --self-test

For each cell, on `base`: the share of runs in which one Roman century broke and, a minute
later, the other century and the warband were both unbroken. It counts the opening, and not
what the warband's men did at it: no rule lets a body turn on a flank.

It reads the scored record alone (`summary.body_break`, in base ticks), so it replays nothing.
It prints outcomes of scored runs, so it is run after the scoring. With `--format-only` it
prints how many records it could read and nothing they hold: that is for trying it on real
files beforehand.
"""

import json
import os
import sys

FREEZE = 9464880715404986219
# A base tick is 50 ms (cannae_core::schedule::BASE_TICK).
TICKS_PER_S = 20
# "A minute later": our choice, fixed in the outline before any record was read.
LATER = 60 * TICKS_PER_S
# The bodies of the base patch, in the file's order.
CENTURIES = (0, 1)
WARBAND = 2


def opening(body_break, sim_s):
    """Whether a run has the opening: "yes", "no", or "cut" if the run ended inside the minute
    with nothing yet to tell them apart."""
    end = sim_s * TICKS_PER_S
    cut = False
    for c in CENTURIES:
        at = body_break[c]
        if at is None:
            continue
        other = body_break[CENTURIES[1 - c]]
        warband = body_break[WARBAND]
        then = at + LATER
        if any(t is not None and t <= then for t in (other, warband)):
            continue
        # The record holds samples below the end tick only, so a minute that ends at the end
        # was not seen out either.
        if then >= end:
            cut = True
            continue
        return "yes"
    return "cut" if cut else "no"


def cell(path, freeze, engine, rung):
    """The scored stopped runs of one file, as (opening, problem) counts. A record of another
    cell than the file's name says is a problem."""
    counts = {"yes": 0, "no": 0, "cut": 0}
    problems = 0
    seen = set()
    with open(path) as f:
        for line in f:
            if not line.strip():
                continue
            try:
                r = json.loads(line)
                if r["whole"] or r["freeze_seed"] != freeze or r["variant"] != "base":
                    continue
                key = (r["draw"], r["seed"])
                breaks = r["summary"]["body_break"]
                mine = r["engine"].lower() == engine and r["rung"] == rung
                if key in seen or len(breaks) != 3 or not mine:
                    problems += 1
                    continue
                seen.add(key)
                counts[opening(breaks, r["sim_s"])] += 1
            except (ValueError, KeyError, TypeError):
                problems += 1
    return counts, problems


def self_test():
    m = LATER
    cases = [
        # One century broke; the other and the warband never did.
        (([100, None, None], 3_000), "yes"),
        # The other century broke within the minute.
        (([100, 100 + m, None], 3_000), "no"),
        # The other century broke after the minute: it stood beside the gap for that minute.
        (([100, 101 + m, None], 3_000), "yes"),
        # The warband had broken by then: it stands beside nothing.
        (([100, None, 50], 3_000), "no"),
        (([100, None, 100 + m], 3_000), "no"),
        (([100, None, 101 + m], 3_000), "yes"),
        # Nobody broke, or only the warband.
        (([None, None, None], 3_000), "no"),
        (([None, None, 400], 3_000), "no"),
        # The second century is the one that broke first.
        (([None, 200, None], 3_000), "yes"),
        # Both broke together.
        (([300, 300, None], 3_000), "no"),
        # One century broke and the warband ten ticks after it; the other century broke long
        # after both. Neither break left an opening with a warband beside it.
        (([100, 5_000, 110], 3_000), "no"),
        # The warband broke first, then one century: nothing stands beside the gap.
        (([900, None, 100], 3_000), "no"),
        # The run ended inside the minute, both still standing.
        (([100, None, None], (100 + m - 1) // TICKS_PER_S), "cut"),
        # The minute ends at the run's end tick: that state was not sampled.
        (([100, None, None], (100 + m) // TICKS_PER_S), "cut"),
        (([100, None, None], (100 + m) // TICKS_PER_S + 1), "yes"),
    ]
    bad = [(c, want, opening(*c)) for c, want in cases if opening(*c) != want]
    for c, want, got in bad:
        print(f"self-test: {c} gave {got}, not {want}")
    print(f"self-test: {len(cases) - len(bad)} of {len(cases)} cases as expected")
    return 1 if bad else 0


def main():
    args = sys.argv[1:]
    if args == ["--self-test"]:
        return self_test()
    freeze, format_only, directory = FREEZE, False, None
    while args:
        a = args.pop(0)
        if a == "--freeze-seed":
            freeze = int(args.pop(0))
        elif a == "--format-only":
            format_only = True
        elif directory is None:
            directory = a
        else:
            print(__doc__)
            return 2
    if directory is None:
        print(__doc__)
        return 2
    for engine in ("pairwise", "field"):
        for rung in range(4):
            path = os.path.join(directory, f"{engine}-r{rung}-base.jsonl")
            if not os.path.exists(path):
                print(f"{engine} rung {rung}: no file")
                continue
            counts, problems = cell(path, freeze, engine, rung)
            n = sum(counts.values())
            if format_only:
                print(f"{engine} rung {rung}: {n} runs read, {problems} lines with a problem")
                continue
            share = f"{100 * counts['yes'] / n:.1f} %" if n else "no runs"
            print(
                f"{engine} rung {rung}: one century broke and, a minute later, the other and "
                f"the warband both stood in {counts['yes']} of {n} runs ({share}); "
                f"{counts['cut']} runs ended inside the minute; {problems} lines with a problem"
            )
    return 0


if __name__ == "__main__":
    sys.exit(main())
