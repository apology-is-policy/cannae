#!/bin/bash
# The pairwise engine's second speed line (S0.7): the spike with the perception's test and
# Views on a pool of threads, read by the same rule as the first. Its conditions are in
# docs/slice0/spike-pairwise.md, "The second line", written before this script.
#
#   tools/s07-readings-pairwise-2.sh OUT
#
# 1. Builds two spikes from $SPIKE (the branch spike-perceive): as built for the first
#    reading (91185c4), and with the pool (200ef7b: the program of d24b053 and a comment).
# 2. The same thing. Ten runs of ten simulated seconds, the line and the pocket: as built at
#    one and eight threads, with the pool at one, four and eight. Every run of a scene must
#    give the count of Views, the check and the final hash that spike-pairwise.md records from
#    the Pi. If one does not, nothing is read.
# 3. The reading. The pocket and the line with the pool, eight threads, the field's
#    perception, 150,000 men, ten minutes each. A run starts only when the 1-minute load is
#    under 2, and the load is logged before and after it. A run that ends with the load at
#    its threads + 2 or more is void and is taken once more; both are reported.
#
# OUT/summary.txt gives, for each run, the mean rate over the second half of its wall time
# (the figure the rule names), the mean of the whole run and the slowest report; then
# 2 / (1/line + 1/pocket), the figure read against 3x. Each run's own output keeps the
# perception's three costs at every report, so the share left on one thread can be read.
#
# One reading: nothing is tuned and read again.
set -u
OUT=${1:?usage: s07-readings-pairwise-2.sh OUT}
SPIKE=${SPIKE:-$HOME/projects/cannae-spike}
BUILT=91185c4
POOLED=200ef7b
mkdir -p "$OUT" || exit 1
OUT=$(cd "$OUT" && pwd)
if [ -n "$(ls -A "$OUT")" ]; then
    echo "$OUT is not empty" >&2
    exit 1
fi
cd "$SPIKE" || exit 1
if [ -n "$(git status --porcelain --untracked-files=no)" ]; then
    echo "$SPIKE has uncommitted changes" >&2
    exit 1
fi
if [ "$(git rev-parse --short=7 spike-perceive)" != "$POOLED" ]; then
    echo "spike-perceive is not at $POOLED" >&2
    exit 1
fi
SUM="$OUT/summary.txt"
for c in $BUILT $POOLED; do
    git checkout -q --detach "$c" || exit 1
    cargo build --release -p cannae-pairwise --example spike -q || exit 1
    cp target/release/examples/spike "$OUT/spike-$c" || exit 1
done
git checkout -q spike-perceive || exit 1
echo "$(date -u +%FT%TZ) second line: as built $BUILT, pooled $POOLED," \
    "$(sysctl -n machdep.cpu.brand_string)" >> "$SUM"

# The same thing, before anything is read.
want_line="views 453000 (check 640000), final hash 3ab06d787dd54e81"
want_pocket="views 809613 (check 7715255), final hash 69cd46eef151b1f5"
same=1
for scene in line pocket; do
    want=$want_line
    [ "$scene" = pocket ] && want=$want_pocket
    for run in "$BUILT 1" "$BUILT 8" "$POOLED 1" "$POOLED 4" "$POOLED 8"; do
        # shellcheck disable=SC2086
        set -- $run
        got=$("$OUT/spike-$1" "$scene" --threads "$2" --perceive field --seconds 10 2>&1 \
            | grep '^done' | sed 's/.*views/views/')
        mark=same
        if [ "$got" != "$want" ]; then
            mark=DIFFERS
            same=0
        fi
        echo "  ten seconds, $scene, $1, $2 threads: $got  $mark" >> "$SUM"
    done
done
if [ "$same" -ne 1 ]; then
    echo "the pooled spike does not give the same thing: nothing is read" >> "$SUM"
    exit 1
fi

BIN="$OUT/spike-$POOLED"
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
# Runs one reading. Returns 2 if the rule voids it.
one() { # name, scene, threads, wall seconds
    local name=$1 scene=$2 threads=$3 wall=$4 before after
    if ! quiet; then
        echo "$name: not run, the load stayed at 2 or more for 30 minutes" >> "$SUM"
        return 1
    fi
    before=$(load)
    echo "$(date -u +%FT%TZ) $name: $scene, $threads threads, perception field on the pool," \
        "$wall s of wall time; load before $before" >> "$SUM"
    "$BIN" "$scene" --threads "$threads" --perceive field --seconds 100000 \
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
    awk -v after="$after" -v limit="$((threads + 2))" 'BEGIN { exit (after + 0 >= limit ? 2 : 0) }'
}
# The second-half figure of a run, if it was read and is not void.
half() {
    awk -v name="$1:" '
        $2 == name { on = 1; r = ""; next }
        on && /^  second half/ { r = $3; sub("x", "", r) }
        on && /VOID/ { r = "" }
        on && /^[0-9]/ { on = 0 }
        END { print r }' "$SUM"
}
figure() { # scene: the run, or the run taken once more if the first was void
    local r
    r=$(half "$1-pool-8")
    [ -z "$r" ] && r=$(half "$1-pool-8-again")
    echo "$r"
}
for scene in pocket line; do
    one "$scene-pool-8" "$scene" 8 600
    [ $? -eq 2 ] && one "$scene-pool-8-again" "$scene" 8 600
done
line=$(figure line)
pocket=$(figure pocket)
if [ -n "$line" ] && [ -n "$pocket" ]; then
    awk -v l="$line" -v p="$pocket" 'BEGIN {
        printf "whole battle, the line and the pocket at equal shares of simulated time: %.2fx", 2 / (1 / l + 1 / p)
        printf " (line %.2fx, pocket %.2fx); the man is not in it\n", l, p }' >> "$SUM"
else
    echo "no whole-battle figure: the line or the pocket was not read, or is void twice" >> "$SUM"
fi
echo "$(date -u +%FT%TZ) finished" >> "$SUM"
