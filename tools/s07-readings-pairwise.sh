#!/bin/bash
# The pairwise engine's sustained speed readings (S0.7), taken by the rule in
# docs/slice0/spike-field.md, "How a sustained reading is read".
#
#   tools/s07-readings-pairwise.sh OUT
#
# Five runs of the speed spike at commit 91185c4 (branch spike-perceive, in $SPIKE), 150,000
# bodies each. A run starts only when the 1-minute load is under 2, and the load is logged
# before and after it.
#
#   pocket, line and rout: eight threads, the field's perception, ten minutes each;
#   pocket: four threads, the field's perception, ten minutes;
#   pocket: eight threads, the core's reference perception, five minutes.
#
# For each run OUT/summary.txt gives the mean rate over the second half of its wall time (for
# a ten-minute run, from the first report at or after five minutes to the last: the figure the
# rule names), the mean of the whole run, and the slowest report with the simulated time it
# was at. A run that ends with the load at its threads + 2 or more is marked void.
#
# The figure of record for the 3x line (docs/slice0/spike-pairwise.md, Michal 2026-11-10) is
# 2 / (1/line + 1/pocket) from the two eight-thread runs with the field's perception; the
# script prints it last, with the pocket's own figure beside it.
#
# Nothing is tuned and read again as the same line: a second reading is a second line.
set -u
OUT=${1:?usage: s07-readings-pairwise.sh OUT}
SPIKE=${SPIKE:-$HOME/projects/cannae-spike}
WANT=91185c4
mkdir -p "$OUT" || exit 1
OUT=$(cd "$OUT" && pwd)
cd "$SPIKE" || exit 1
if [ "$(git rev-parse --short=7 HEAD)" != "$WANT" ]; then
    echo "$SPIKE is not at $WANT" >&2
    exit 1
fi
if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    echo "$SPIKE has uncommitted changes" >&2
    exit 1
fi
cargo build --release -p cannae-pairwise --example spike -q || exit 1
BIN=./target/release/examples/spike
SUM="$OUT/summary.txt"
load() { sysctl -n vm.loadavg | awk '{print $2}'; }
# Waits for a 1-minute load under 2. Gives up after 30 minutes.
quiet() {
    local t=0
    while awk -v l="$(load)" 'BEGIN { exit !(l >= 2) }'; do
        [ "$t" -ge 1800 ] && return 1
        sleep 10
        t=$((t + 10))
    done
}
one() { # name, scene, threads, perception, wall seconds
    local name=$1 scene=$2 threads=$3 perceive=$4 wall=$5 before after
    if ! quiet; then
        echo "$name: not run, the load stayed at 2 or more for 30 minutes" >> "$SUM"
        return
    fi
    before=$(load)
    echo "$(date -u +%FT%TZ) $name: $scene, $threads threads, perception $perceive, $wall s" \
        "of wall time; load before $before" >> "$SUM"
    "$BIN" "$scene" --threads "$threads" --perceive "$perceive" --seconds 100000 \
        --wall-s "$wall" --report-every-s 30 > "$OUT/$name.txt" 2>&1
    after=$(load)
    awk -v half="$((wall / 2))" -v after="$after" -v limit="$((threads + 2))" '
        /^  wall / {
            n++; W[n] = $2; S[n] = $5; r = $8; sub("x", "", r)
            if (!k && W[n] >= half) k = n
            if (slow == "" || r + 0 < slow) { slow = r + 0; at = S[n] }
            last = $0
        }
        END {
            if (n < 2 || !k || k == n) { print "  too few reports to read"; exit }
            printf "  second half %.2fx (simulated %.1f to %.1f s, wall %d to %d s)\n",
                (S[n] - S[k]) / (W[n] - W[k]), S[k], S[n], W[k], W[n]
            printf "  whole run %.2fx; slowest report %.2fx, at simulated %.1f s\n",
                S[n] / W[n], slow, at
            printf "  last report:%s\n", last
            printf "  load after %s%s\n", after,
                (after + 0 >= limit ? "  VOID: at its threads + 2 or more" : "")
        }' "$OUT/$name.txt" >> "$SUM"
    grep '^done' "$OUT/$name.txt" | sed 's/^/  /' >> "$SUM"
}
echo "$(date -u +%FT%TZ) readings at $WANT, $(sysctl -n machdep.cpu.brand_string)" >> "$SUM"
one pocket-field-8 pocket 8 field 600
one line-field-8 line 8 field 600
one rout-field-8 rout 8 field 600
one pocket-field-4 pocket 4 field 600
one pocket-grid-8 pocket 8 grid 300
# The second-half figure of a run, if it was read and is not void.
half() {
    awk -v name="$1:" '
        $2 == name { on = 1; next }
        on && /^  second half/ { r = $3; sub("x", "", r) }
        on && /VOID/ { r = "" }
        on && /^[0-9]/ { on = 0 }
        END { print r }' "$SUM"
}
line=$(half line-field-8)
pocket=$(half pocket-field-8)
if [ -n "$line" ] && [ -n "$pocket" ]; then
    awk -v l="$line" -v p="$pocket" 'BEGIN {
        printf "whole battle, the line and the pocket at equal shares of simulated time: %.2fx", 2 / (1 / l + 1 / p)
        printf " (line %.2fx, pocket %.2fx); the man is not in it\n", l, p }' >> "$SUM"
else
    echo "no whole-battle figure: the line or the pocket was not read, or is void" >> "$SUM"
fi
echo "$(date -u +%FT%TZ) finished" >> "$SUM"
