#!/usr/bin/env python3
"""Reads one output file of spike_field by the rule in docs/slice0/spike-field.md,
"How a sustained reading is read". Written before any sustained reading existed.

  read_spike.py FILE [--from WALL_S]     (default 300)

Prints the figure of record (mean rate from the first report at or after WALL_S of wall
time to the end of the run, end values from the `done:` line), the whole-run mean, the
slowest interval between two reports, the state of the scene in the last reports, and
the perception's cost per man step where the run printed it.
"""
import re
import sys

MAN_STEP_S = 0.25

REPORT = re.compile(
    r"^\s+wall\s+(\d+) s\s+sim\s+([\d.]+) s\s+rate\s+([\d.]+)x\s+free (\d+)\s+hot (\d+)\s+"
    r"passes (\d+)\s+tiles (\d+)\s+densest ([\d.]+) \(peak ([\d.]+)\) per m2\s+"
    r"crushed (\d+)\s+views (\d+)"
)
PERC = re.compile(r"^\s+perception, ms: rebuild (\d+), coarse (\d+), views (\d+)")
DONE = re.compile(
    r"^done: (\d+) simulated s in (\d+) s, ([\d.]+)x real time overall, peak ([\d.]+) per m2, "
    r"crushed (\d+), views (\d+) \(check (\d+)\), final hash ([0-9a-f]{16})"
)


def main():
    args = sys.argv[1:]
    since = 300.0
    if "--from" in args:
        i = args.index("--from")
        since = float(args[i + 1])
        del args[i : i + 2]
    path = args[0]
    lines = open(path).read().splitlines()
    head = lines[0] if lines else ""
    reports, done = [], None
    for ln in lines:
        m = REPORT.match(ln)
        if m:
            g = m.groups()
            reports.append(
                dict(
                    wall=float(g[0]), sim=float(g[1]), rate=float(g[2]), free=int(g[3]),
                    hot=int(g[4]), passes=int(g[5]), tiles=int(g[6]), densest=float(g[7]),
                    peak=float(g[8]), crushed=int(g[9]), views=int(g[10]), perc=None,
                )
            )
            continue
        m = PERC.match(ln)
        if m and reports:
            reports[-1]["perc"] = tuple(int(x) for x in m.groups())
            continue
        m = DONE.match(ln)
        if m:
            g = m.groups()
            done = dict(sim=float(g[0]), wall=float(g[1]), rate=float(g[2]), peak=float(g[3]),
                        crushed=int(g[4]), views=int(g[5]), hash=g[7])
    print(head)
    if done is None:
        print("NO `done:` LINE: the run did not finish; nothing is read from it.")
        return 2
    print(f"reports: {len(reports)}; done: {done['sim']:.0f} sim s in {done['wall']:.0f} s; "
          f"hash {done['hash']}")
    print(f"whole run: {done['rate']:.2f}x over {done['sim']:.0f} simulated s")

    start = next((r for r in reports if r["wall"] >= since), None)
    if start is None or done["wall"] - start["wall"] <= 0:
        print(f"NO FIGURE OF RECORD: no report at or after {since:.0f} s of wall time.")
    else:
        dsim = done["sim"] - start["sim"]
        dwall = done["wall"] - start["wall"]
        # The printed wall times are whole seconds and the done line's sim is a whole second.
        lo = (dsim - 0.5) / (dwall + 1.0)
        hi = (dsim + 0.5) / (dwall - 1.0) if dwall > 1.0 else float("inf")
        print(f"FIGURE OF RECORD: {dsim / dwall:.2f}x  (sim {start['sim']:.1f} -> {done['sim']:.0f} s "
              f"= {dsim:.0f} s, over wall {start['wall']:.0f} -> {done['wall']:.0f} s = {dwall:.0f} s; "
              f"between {lo:.2f}x and {hi:.2f}x as printed values round)")

    # Intervals between reports; the first runs from the start of the run.
    prev_wall, prev_sim = 0.0, 0.0
    slowest = None
    for r in reports:
        iv = dict(w0=prev_wall, w1=r["wall"], s0=prev_sim, s1=r["sim"], rate=r["rate"])
        if slowest is None or iv["rate"] < slowest["rate"]:
            slowest = iv
        prev_wall, prev_sim = r["wall"], r["sim"]
    if slowest:
        print(f"slowest interval: {slowest['rate']:.2f}x  (wall {slowest['w0']:.0f} -> {slowest['w1']:.0f} s, "
              f"sim {slowest['s0']:.1f} -> {slowest['s1']:.1f} s)")
    if reports:
        print("rates by report: " + " ".join(f"{r['rate']:.2f}" for r in reports))
        print("scene, last three reports (hot, densest per m2, free, views so far):")
        for r in reports[-3:]:
            print(f"  wall {r['wall']:.0f} s: hot {r['hot']}, densest {r['densest']:.1f} "
                  f"(peak {r['peak']:.1f}), free {r['free']}, passes {r['passes']}, "
                  f"crushed {r['crushed']}, views {r['views']}")

    # Perception: the ms printed with a report cover the interval that ends at it.
    prev_wall, prev_sim, prev_views = 0.0, 0.0, 0
    tot = [0, 0, 0]
    steps = 0.0
    wall_cov = 0.0
    views_cov = 0
    any_perc = False
    for r in reports:
        if r["perc"] is not None and (start is not None and r["wall"] > start["wall"]):
            any_perc = True
            for k in range(3):
                tot[k] += r["perc"][k]
            steps += (r["sim"] - prev_sim) / MAN_STEP_S
            wall_cov += r["wall"] - prev_wall
            views_cov += r["views"] - prev_views
        prev_wall, prev_sim, prev_views = r["wall"], r["sim"], r["views"]
    if any_perc and steps > 0:
        allp = sum(tot)
        print(f"perception after the report at {start['wall']:.0f} s, over {steps:.0f} man steps "
              f"({wall_cov:.0f} s of wall):")
        print(f"  per man step, ms: rebuild {tot[0] / steps:.1f}, dormancy test {tot[1] / steps:.1f}, "
              f"views {tot[2] / steps:.1f}, total {allp / steps:.1f}")
        print(f"  share of wall time: {100.0 * allp / 1000.0 / wall_cov:.0f} %; "
              f"views per man step {views_cov / steps:.0f}; "
              f"ns per view {1e6 * tot[2] / views_cov:.0f}" if views_cov else "")
        # What the substrate alone would run at in this same run, with the perception's time taken out.
        rest = wall_cov - allp / 1000.0
        if rest > 0:
            print(f"  the same intervals without the perception's time: {steps * MAN_STEP_S / rest:.2f}x; "
                  f"the perception alone would allow {steps * MAN_STEP_S / (allp / 1000.0):.2f}x")
    return 0


if __name__ == "__main__":
    sys.exit(main())
