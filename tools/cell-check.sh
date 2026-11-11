#!/bin/bash
# The field's cell check: the nominal set on `base` at the scored cell (1.024 m) and at half
# of it (0.512 m). Its conditions are in docs/slice0/cell-check.md, written before this
# script was run.
#
#   tools/cell-check.sh OUT
#
# 1. Builds `cell` and `score` twice in $SRC (the worktree of the branch cell-check): the
#    re-frozen code (40b8dcd), and the same with one line changed (9241361: the centre's
#    cell_shift, 10 to 9). The two commits must differ in that one line and in nothing else.
# 2. Runs the field's own tests on each build. Their outcome is reported beside the check;
#    a failure stops nothing.
# 3. Runs the nominal set on `base`, rungs 0 to 3, stopped by the rule: seeds 1 to 60 at
#    1.024 m and seeds 1 to 30 at 0.512 m. Seeds 31 to 60 at 1.024 m are the control, in
#    which nothing differs but the seeds. The 1.024 m records are then copied apart into
#    seeds 1 to 30 (OUT/1024) and 31 to 60 (OUT/1024b), and `score` is run on each group of
#    thirty, for the scorer's own eight verdicts.
# 4. tools/cell-check.py writes OUT/summary.txt; OUT/sha256.txt holds the records' hashes.
#
# Both builds read scenarios/ from $SRC as they run. The two commits hold the same scenarios.
#
# One run: nothing is tuned and run again. A step that fails is written to OUT/failed.txt and
# the script goes on, so that what did run is kept.
set -u
OUT=${1:?usage: cell-check.sh OUT}
SRC=${SRC:-$HOME/projects/cannae-cellcheck}
JOBS=${JOBS:-4}
SEEDS=30
SCORED=40b8dcd
HALF=9241361
HERE=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$OUT" || exit 1
OUT=$(cd "$OUT" && pwd)
if [ -n "$(ls -A "$OUT")" ]; then
    echo "$OUT is not empty" >&2
    exit 1
fi
cd "$SRC" || exit 1
if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    echo "$SRC has uncommitted changes" >&2
    exit 1
fi
if [ "$(git rev-parse --short=7 cell-check)" != "$HALF" ]; then
    echo "cell-check is not at $HALF" >&2
    exit 1
fi
if [ "$(git diff --numstat "$SCORED" "$HALF" | tr '\t' ' ')" != "1 1 crates/cannae-field/src/field.rs" ]; then
    echo "$HALF is not $SCORED with one line changed" >&2
    exit 1
fi
failed() { echo "$(date -u +%FT%TZ) $*" >> "$OUT/failed.txt"; }
echo "$(date -u +%FT%TZ) started: scored $SCORED, half cell $HALF, $SEEDS seeds, $JOBS jobs," \
    "$(sysctl -n machdep.cpu.brand_string 2>/dev/null || uname -m)" > "$OUT/log.txt"

for pair in "1024 $SCORED" "512 $HALF"; do
    # shellcheck disable=SC2086
    set -- $pair
    git checkout -q --detach "$2" || exit 1
    cargo build --release -p cannae-score --bin cell --bin score -q || exit 1
    cp target/release/cell "$OUT/cell-$1" || exit 1
    cp target/release/score "$OUT/score-$1" || exit 1
    cargo test -p cannae-field > "$OUT/tests-$1.txt" 2>&1 \
        || failed "the field's tests on the build of $2 (see tests-$1.txt)"
done
git checkout -q cell-check || exit 1

for rung in 0 1 2 3; do
    for pair in "1024 1024all $((2 * SEEDS))" "512 512 $SEEDS"; do
        # shellcheck disable=SC2086
        set -- $pair
        "$OUT/cell-$1" --substrate field --rung "$rung" --variant base --nominal yes \
            --seeds "$3" --jobs "$JOBS" --out "$OUT/$2" > "$OUT/$2-r$rung.log" 2>&1 \
            || failed "the runs at cell $1, rung $rung (see $2-r$rung.log)"
        echo "$(date -u +%FT%TZ) cell $1, rung $rung: done" >> "$OUT/log.txt"
    done
done
# The 1.024 m records by their seeds: the first thirty, and the control's thirty.
mkdir -p "$OUT/1024" "$OUT/1024b" || exit 1
for rung in 0 1 2 3; do
    f="field-r$rung-base.jsonl"
    python3 - "$OUT/1024all/$f" "$OUT/1024/$f" "$OUT/1024b/$f" "$SEEDS" <<'PY' \
        || failed "the 1.024 m records of rung $rung could not be copied apart"
import json, sys
src, first, second, n = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])
with open(src) as f, open(first, "w") as a, open(second, "w") as b:
    for line in f:
        (a if json.loads(line)["seed"] <= n else b).write(line)
PY
    for name in 1024 512 1024b; do
        bin=1024
        [ "$name" = 512 ] && bin=512
        "$OUT/score-$bin" --substrate field --rung "$rung" --dir "$OUT/$name" \
            --seeds "$SEEDS" > "$OUT/score-$name-r$rung.txt" 2>&1 \
            || failed "score on group $name, rung $rung"
    done
done
(cd "$OUT" && shasum -a 256 1024all/*.jsonl 512/*.jsonl > sha256.txt) \
    || failed "the records' hashes"

python3 "$HERE/cell-check.py" "$OUT/1024" "$OUT/512" "$OUT/1024b" --seeds "$SEEDS" \
    --score-dir "$OUT" \
    > "$OUT/summary.txt" 2>&1 || failed "cell-check.py reports a problem (see summary.txt)"
echo "$(date -u +%FT%TZ) finished" >> "$OUT/log.txt"
