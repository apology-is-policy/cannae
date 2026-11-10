#!/bin/bash
# Slice 0's scored batch (S0.9), as pre-registered (docs/slice0/preregistration.md).
#
#   [SCORE=path/to/score] tools/s09-batch.sh OUT [field|pairwise|all]
#
# Runs every cell (two engines, rungs 0-3) on every variant: 200 parameter sets from the
# frozen seed, 10 seeds each, the stop rule on, the audit's whole runs for sets 0, 20, 40 and
# so on; `relief` and `backed` on every fourth set. Each run is appended to OUT the moment it
# ends, so the script can be stopped and started again and picks up where it was. The two
# engines' files never overlap, so two machines may each take one (results are the same bit
# for bit on the Mac and the Pi: checked on 32 runs before the batch).
#
# Order: `base` first in every cell, since the eight patterns are scored on it; then the
# variants of pattern 6's two legs; then the reported ones. The field before the pairwise
# engine within each variant, being cheaper. With SCORE set to a `score` that prints the base
# gate (preregistration, Addendum 1), the cells that `base` rules out run all their variants
# after those of the cells that can still pass; each gate line is logged. Must be run from a checkout whose
# `git diff dd9d90a -- crates scenarios Cargo.toml Cargo.lock` is empty (checked below).
set -u
OUT=${1:?usage: s09-batch.sh OUT [field|pairwise|all]}
WHICH=${2:-all}
FREEZE=9464880715404986219
cd "$(dirname "$0")/.."
if ! git diff --quiet dd9d90a -- crates scenarios Cargo.toml Cargo.lock; then
    echo "this checkout's code differs from the frozen commit dd9d90a" >&2
    exit 1
fi
cargo build --release -p cannae-score -q || exit 1
mkdir -p "$OUT"
engines="field pairwise"
[ "$WHICH" != all ] && engines=$WHICH
ALL=""
for e in $engines; do for r in 0 1 2 3; do ALL="$ALL $e:$r"; done; done
run() { # variant, extra flags; over the cells in $CELLS, each engine:rung
    for c in $CELLS; do
        e=${c%:*} r=${c#*:}
        echo "$(date -u +%FT%TZ) start $e r$r $1" >> "$OUT/progress.txt"
        ./target/release/cell --substrate "$e" --rung "$r" --variant "$1" --draws 200 \
            --seeds 10 --freeze-seed "$FREEZE" --jobs 4 --out "$OUT" ${2:-} \
            > "$OUT/log-$e-r$r-$1.txt" 2>&1 \
            || echo "$(date -u +%FT%TZ) FAILED $e r$r $1" >> "$OUT/progress.txt"
        echo "$(date -u +%FT%TZ) done $e r$r $1" >> "$OUT/progress.txt"
    done
}
variants() {
    for v in veterans recruits flank; do run $v; done
    for v in shock volleys; do run $v; done
    for v in backed relief; do run $v "--every 4"; done
}
CELLS=$ALL
run base
KEEP=$ALL LAST=""
if [ -n "${SCORE:-}" ]; then
    KEEP=""
    for c in $ALL; do
        g=$("$SCORE" --substrate "${c%:*}" --rung "${c#*:}" --dir "$OUT" --seeds 10 | grep "Base gate")
        echo "$(date -u +%FT%TZ) gate $c:${g:- no gate line}" >> "$OUT/progress.txt"
        case "$g" in
        *"cannot pass"*) LAST="$LAST $c" ;;
        *) KEEP="$KEEP $c" ;;
        esac
    done
fi
CELLS=$KEEP
variants
if [ -n "$LAST" ]; then
    CELLS=$LAST
    variants
fi
echo "$(date -u +%FT%TZ) batch finished" >> "$OUT/progress.txt"
