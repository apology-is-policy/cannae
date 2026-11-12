#!/bin/bash
# The relief's top-up (docs/slice0/preregistration.md, "The relief"): `relief` and `backed` on
# the sets the scorer names for each cell (`report::topup`), at most 25 a cell. Run only if
# Michal gives the machine time, and only when the batch has finished.
#
#   tools/s09-topup.sh OUT
#
# OUT is the batch's folder. The new runs are appended to the cell's own files of `relief` and
# `backed`, where the scorer reports them apart from the first 50 sets'. Runs already there
# are not made again, so the script can be stopped and started again.
#
# The sets go from `score` to `cell --sets` and are neither printed nor logged: how many sets
# a cell adds is an outcome of `base`, and so is how long its step takes. progress.txt gets a
# line when a step starts and ends, as in the batch. Nobody reads a log of `cell` or the times
# of single steps before the scoring.
#
# The last line is "top-up finished" only if no step failed on this pass. A pass skips the
# runs already made, so that line means every run of the top-up is in. After a failed step the
# script can be started again: only a failed step of the batch itself stops it.
#
# S09_FREEZE and S09_SEEDS are for trying the script on unscored sets, never for the batch.
set -u
OUT=${1:?usage: s09-topup.sh OUT}
FREEZE=${S09_FREEZE:-9464880715404986219}
SEEDS=${S09_SEEDS:-10}
cd "$(dirname "$0")/.."
if ! git diff --quiet 40b8dcd -- crates scenarios Cargo.toml Cargo.lock; then
    echo "this checkout's code differs from the re-frozen commit 40b8dcd" >&2
    exit 1
fi
if ! grep -q "batch finished" "$OUT/progress.txt" 2>/dev/null; then
    echo "the batch in $OUT has not finished" >&2
    exit 1
fi
if grep -v gate "$OUT/progress.txt" | grep -v top-up | grep -q FAILED; then
    echo "a step of the batch in $OUT failed" >&2
    exit 1
fi
cargo build --release -p cannae-score -q || exit 1
failed=0
for e in field pairwise; do
    for r in 0 1 2 3; do
        report=$(./target/release/score --substrate "$e" --rung "$r" --dir "$OUT" --seeds "$SEEDS") || {
            echo "$(date -u +%FT%TZ) FAILED top-up $e r$r: the scorer" >> "$OUT/progress.txt"
            failed=$((failed + 1))
            continue
        }
        sets=$(echo "$report" | sed -n 's/^ *the top-up, if it is run: sets \[\(.*\)\]$/\1/p' | tr -d ' ')
        for v in backed relief; do
            echo "$(date -u +%FT%TZ) start top-up $e r$r $v" >> "$OUT/progress.txt"
            if [ -n "$sets" ]; then
                ./target/release/cell --substrate "$e" --rung "$r" --variant "$v" --sets "$sets" \
                    --seeds "$SEEDS" --freeze-seed "$FREEZE" --jobs 4 --out "$OUT" \
                    > "$OUT/log-$e-r$r-$v-topup.txt" 2>&1 || {
                    echo "$(date -u +%FT%TZ) FAILED top-up $e r$r $v" >> "$OUT/progress.txt"
                    failed=$((failed + 1))
                }
            fi
            echo "$(date -u +%FT%TZ) done top-up $e r$r $v" >> "$OUT/progress.txt"
        done
    done
done
if [ "$failed" -gt 0 ]; then
    echo "$(date -u +%FT%TZ) top-up stopped short: $failed steps failed" >> "$OUT/progress.txt"
    exit 1
fi
echo "$(date -u +%FT%TZ) top-up finished" >> "$OUT/progress.txt"
