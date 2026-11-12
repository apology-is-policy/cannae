#!/usr/bin/env python3
"""Rechecks Slice 0's scored batch on a second machine (S0.9).

    tools/s09-recheck.py BATCH_DIR OUT_DIR [--jobs N] [--totals]

For every cell and variant, two parameter sets are run again here, all ten seeds each (and the
audit's whole runs, where a chosen set is one of the audit's), at the frozen code, and each
record is compared with the batch's, field for field, all but the wall time. That is 1 % of a
variant's runs (4 % for `relief` and `backed`, which run on every fourth set).

Which sets is fixed by the freeze seed, so that nobody picks them: for k = 0, 1, .. the set is

    candidates[ int(SHA-256("<freeze seed>/<engine>/r<rung>/<variant>/<k>")[:8], big-endian)
                mod len(candidates) ]

until two different sets are drawn; the candidates are 0..199, or every fourth for `relief` and
`backed`. A cell and variant whose chosen runs are not all in BATCH_DIR yet is skipped, so
this can be run as the batch goes. Runs already in OUT_DIR are not made again.

Prints, for each cell and variant, the sets and the counts of equal and unequal records, and
never what happened in a run. Exits 1 if any record differs. With `--totals` it prints the
last line only, and a cell's line only if one of its records differs: after `base` the batch
runs first the cells that can still pass, so which variants are in tells the gate's verdict.

    tools/s09-recheck.py BATCH_DIR OUT_DIR --topup [--jobs N]

The same for the relief's top-up, once `tools/s09-topup.sh` has run it: for each cell, and for
`relief` and `backed` apart, two of the cell's top-up sets (one, if it has only one) are run
again here and compared. The candidates are the sets the scorer names (`report::topup`), and
the key hashed is "<freeze seed>/<engine>/r<rung>/<variant>-topup/<k>". Which sets a cell has,
and how many, is an outcome of `base`: this mode prints one line of totals and no cell.

S09_FREEZE and S09_SEEDS are for trying the tool on unscored sets, never for the batch.
"""
import hashlib
import json
import os
import re
import subprocess
import sys

FREEZE = int(os.environ.get("S09_FREEZE", 9464880715404986219))
# The re-frozen code of Addendum 4: dd9d90a with ed6d363 and 449dcd6.
FROZEN = "40b8dcd"
VARIANTS = ["base", "veterans", "recruits", "flank", "shock", "volleys", "backed", "relief"]
SEEDS = int(os.environ.get("S09_SEEDS", 10))
AUDIT_EVERY = 20


def drawn(candidates, key):
    """Two of the candidates (all of them, if they are fewer), by the freeze seed's rule."""
    out, k = [], 0
    while len(out) < min(2, len(candidates)):
        h = hashlib.sha256(f"{key}/{k}".encode()).digest()
        s = candidates[int.from_bytes(h[:8], "big") % len(candidates)]
        if s not in out:
            out.append(s)
        k += 1
    return sorted(out)


def chosen(engine, rung, variant):
    stride = 4 if variant in ("backed", "relief") else 1
    return drawn(list(range(0, 200, stride)), f"{FREEZE}/{engine}/r{rung}/{variant}")


def topup_sets(batch, engine, rung):
    """The cell's top-up sets, from the scorer's own report."""
    report = subprocess.run(
        ["./target/release/score", "--substrate", engine, "--rung", str(rung),
         "--dir", batch, "--seeds", str(SEEDS)],
        capture_output=True, text=True, check=True,
    ).stdout
    named = re.search(r"the top-up, if it is run: sets \[(.*)\]", report)
    return [int(x) for x in named.group(1).split(",")] if named else []


def load(path):
    out = {}
    if os.path.exists(path):
        with open(path) as f:
            for line in f:
                try:
                    r = json.loads(line)
                except ValueError:
                    continue
                r.pop("wall_ms", None)
                # Records made by a later scorer may carry hashes the batch's do not.
                r.pop("end_hash", None)
                r.pop("log_hash", None)
                out[(r["draw"], r["seed"], r["whole"])] = r
    return out


def recheck(batch, out, engine, rung, variant, sets, jobs):
    """The counts of equal and unequal records of these sets, or None if the batch lacks one."""
    name = f"{engine}-r{rung}-{variant}.jsonl"
    want = [
        (s, seed, whole)
        for s in sets
        for seed in range(1, SEEDS + 1)
        for whole in ([False, True] if s % AUDIT_EVERY == 0 else [False])
    ]
    theirs = load(os.path.join(batch, name))
    if any(k not in theirs for k in want):
        return None
    ours = load(os.path.join(out, name))
    if any(k not in ours for k in want):
        subprocess.run(
            ["./target/release/cell", "--substrate", engine, "--rung", str(rung),
             "--variant", variant, "--sets", ",".join(map(str, sets)),
             "--seeds", str(SEEDS), "--freeze-seed", str(FREEZE), "--jobs", jobs,
             "--out", out],
            stdout=subprocess.DEVNULL,
            check=True,
        )
        ours = load(os.path.join(out, name))
    equal = sum(1 for k in want if ours.get(k) == theirs[k])
    return equal, len(want) - equal


def main():
    args = sys.argv[1:]
    jobs = "4"
    if "--jobs" in args:
        i = args.index("--jobs")
        jobs = args[i + 1]
        del args[i : i + 2]
    topup = "--topup" in args
    if topup:
        args.remove("--topup")
    totals = "--totals" in args
    if totals:
        args.remove("--totals")
    if len(args) != 2:
        sys.exit(__doc__)
    batch, out = args
    root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    os.chdir(root)
    frozen = subprocess.run(
        ["git", "diff", "--quiet", FROZEN, "--", "crates", "scenarios", "Cargo.toml", "Cargo.lock"]
    )
    if frozen.returncode != 0:
        sys.exit(f"this checkout's code differs from the frozen commit {FROZEN}")
    subprocess.run(["cargo", "build", "--release", "-p", "cannae-score", "-q"], check=True)
    os.makedirs(out, exist_ok=True)
    total = {"equal": 0, "unequal": 0, "waiting": 0}
    for engine in ("field", "pairwise"):
        for rung in range(4):
            candidates = topup_sets(batch, engine, rung) if topup else None
            for variant in ("backed", "relief") if topup else VARIANTS:
                if topup:
                    sets = drawn(candidates, f"{FREEZE}/{engine}/r{rung}/{variant}-topup")
                else:
                    sets = chosen(engine, rung, variant)
                counts = recheck(batch, out, engine, rung, variant, sets, jobs)
                if counts is None:
                    total["waiting"] += 1
                    continue
                equal, unequal = counts
                total["equal"] += equal
                total["unequal"] += unequal
                if not topup and (unequal or not totals):
                    print(f"{engine:8} r{rung} {variant:8}: sets {sets[0]:3}, {sets[1]:3}: "
                          f"{equal} records equal, {unequal} unequal", flush=True)
    if topup and total["waiting"]:
        # How many cells wait would tell how many have a set at all.
        sys.exit("the top-up's runs are not all in the batch's folder yet")
    if topup:
        print(f"the top-up: {total['equal']} records equal, {total['unequal']} unequal")
    else:
        print(f"all: {total['equal']} records equal, {total['unequal']} unequal; "
              f"{total['waiting']} of 64 cells and variants not yet in the batch")
    sys.exit(1 if total["unequal"] else 0)


if __name__ == "__main__":
    main()
