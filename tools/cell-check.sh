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
# 3. Runs the nominal set on `base`, rungs 0 to 3, seeds 1 to $SEEDS, on each build, stopped
#    by the rule; then `score` on each rung's records, for its eight verdicts.
# 4. tools/cell-check.py writes OUT/summary.txt.
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

for name in 1024 512; do
    for rung in 0 1 2 3; do
        "$OUT/cell-$name" --substrate field --rung "$rung" --variant base --nominal yes \
            --seeds "$SEEDS" --jobs "$JOBS" --out "$OUT/$name" > "$OUT/$name-r$rung.log" 2>&1 \
            || failed "the runs at cell $name, rung $rung (see $name-r$rung.log)"
        "$OUT/score-$name" --substrate field --rung "$rung" --dir "$OUT/$name" \
            --seeds "$SEEDS" > "$OUT/score-$name-r$rung.txt" 2>&1 \
            || failed "score at cell $name, rung $rung"
        echo "$(date -u +%FT%TZ) cell $name, rung $rung: done" >> "$OUT/log.txt"
    done
done

python3 "$HERE/cell-check.py" "$OUT/1024" "$OUT/512" --seeds "$SEEDS" --score-dir "$OUT" \
    > "$OUT/summary.txt" 2>&1 || failed "cell-check.py reports a problem (see summary.txt)"
echo "$(date -u +%FT%TZ) finished" >> "$OUT/log.txt"
