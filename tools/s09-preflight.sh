#!/bin/bash
# The pre-flight of Slice 0's scored batch: every step the batch will run is started once, with
# the batch's own arguments, before the batch is.
#
#   tools/s09-preflight.sh SCRATCH [SECONDS]
#
# The batch has 64 steps: two engines by four rungs by eight variants. Each is run here for
# SECONDS (default 120) into a folder of its own under SCRATCH, and then stopped. A step that
# ends before its time is reported with its exit status, which is what the batch itself reads;
# so is one that made no run, and one whose log says a battle panicked. The count of runs each
# made times the batch again.
#
# Why it exists: the first batch's pairwise half died at its first run of a whole cell, which
# nothing had ever started (docs/STATUS.md, 2026-11-11). The batch must run nothing for the
# first time.
#
# Run it on the machine that will run the batch, from the checkout the batch will run from,
# with nothing else running there. SCRATCH must not be the batch's folder: these are runs of
# the scored sets, they are thrown away, and nothing of them is printed but their number. A
# step's log holds each run's outcome, so of a failed step's log only the lines that name a
# failure are shown.
set -u
SCRATCH=${1:?usage: s09-preflight.sh SCRATCH [SECONDS]}
SECS=${2:-120}
FREEZE=9464880715404986219
cd "$(dirname "$0")/.."
cargo build --release -p cannae-score -q || exit 1
mkdir -p "$SCRATCH" || exit 1
if [ -n "$(ls -A "$SCRATCH")" ]; then
    echo "$SCRATCH is not empty" >&2
    exit 1
fi
bad=0
total=0
printf '%-9s %-4s %-9s %6s  %s\n' engine rung variant runs result
for v in base veterans recruits flank shock volleys backed relief; do
    every=""
    case $v in backed | relief) every="--every 4" ;; esac
    for e in field pairwise; do
        for r in 0 1 2 3; do
            dir="$SCRATCH/$e-r$r-$v"
            mkdir -p "$dir"
            # shellcheck disable=SC2086
            ./target/release/cell --substrate "$e" --rung "$r" --variant "$v" --draws 200 \
                --seeds 10 --freeze-seed "$FREEZE" --jobs 4 --out "$dir" $every \
                > "$dir/log.txt" 2>&1 &
            pid=$!
            t=0
            while [ "$t" -lt "$SECS" ] && kill -0 "$pid" 2>/dev/null; do
                sleep 1
                t=$((t + 1))
            done
            if kill -0 "$pid" 2>/dev/null; then
                kill "$pid"
                wait "$pid" 2>/dev/null
                result="ran $SECS s"
            else
                wait "$pid"
                result="ENDED after $t s, exit $?"
            fi
            runs=$(cat "$dir"/*.jsonl 2>/dev/null | wc -l | tr -d ' ')
            total=$((total + runs))
            case $result in
            ran*) [ "$runs" -gt 0 ] || result="$result, NO RUN MADE" ;;
            esac
            # A battle that panics takes its thread with it and the others run on: the step
            # then fails only when its last battle ends, long after this stops it.
            if grep -q 'panicked' "$dir/log.txt"; then
                result="$result, A BATTLE PANICKED"
            fi
            printf '%-9s %-4s %-9s %6s  %s\n' "$e" "r$r" "$v" "$runs" "$result"
            case $result in
            *ENDED* | *"NO RUN"* | *PANICKED*)
                bad=$((bad + 1))
                grep -iE 'panic|overflow|abort|error|fatal|killed|signal|no such file|usage' \
                    "$dir/log.txt" | tail -5 | sed 's/^/    /'
                ;;
            esac
        done
    done
done
echo "$total runs in 64 steps of $SECS s; $bad steps failed"
[ "$bad" -eq 0 ]
