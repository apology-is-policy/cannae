#!/usr/bin/env python3
"""Counts and pictures of the visual QC pass's flags (docs/slice0/qc-pass.md).

  tools/qc-render.py counts QCDIR
      The counts by detector, engine and variant, and by kind, as Markdown.
  tools/qc-render.py pick QCDIR [--engine field|pairwise]
      The flags the rule says are looked at: for each detector, engine and kind, the first
      three in the order of variant, rung, seed and time; the first in the variant that has
      the most of that kind; and of detectors 25 and 26 the first of each kind in `backed` and
      in `relief`. One line each.
  tools/qc-render.py draw QCDIR [--engine E] [--only ID] [--also FILE] [--png] [--scale S]
                          [--jobs N]
      A picture for each picked flag, in QCDIR/pictures: the men within 15 m at four moments,
      as SVG. The battle is run again for it (`cannae-qc clip`), which gives the same battle;
      --jobs runs that many at a time. With --png each is also made a PNG, by Quick Look on a
      Mac and by a headless Chromium elsewhere (--scale: its pixels to a unit of the drawing, 2
      if not given). Writes QCDIR/pictures/page-ENGINE.md, one entry for each picture, with the
      link that opens the live viewer there.
      --also FILE draws, in place of the picks, the flags a file names: one to a line as
      `ENGINE RUNG VARIANT SEED DETECTOR-NUMBER FROM_S [BODY]`, for a flag someone chose to
      look at beyond the rule. Their pictures are named `also-...` and their page is
      page-also-ENGINE.md.

QCDIR is the folder `cannae-qc scan --out` wrote. The battles are run again by the program at
viewer/target/release/cannae-qc, or the one the environment's CANNAE_QC names: it must be the
build that made the flags. Nothing here judges a flag.
"""
import json
import math
import os
import re
import shutil
import subprocess
import sys
import tempfile
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
# The QC program built from the code under the pass; CANNAE_QC names another build of it.
QC = Path(os.environ.get("CANNAE_QC", ROOT / "viewer" / "target" / "release" / "cannae-qc"))
ORDER = ["base", "veterans", "recruits", "flank", "shock", "volleys", "backed", "relief"]
ENGINES = ["field", "pairwise"]
PICKED = 3
# Varro, 2026-11-11: of detectors 25 and 26 the first flag of each kind in each of these
# variants is looked at too, since the relief is where his fix meets other code.
ALSO = {25: ["backed", "relief"], 26: ["backed", "relief"]}
# Every detector of the list (docs/slice0/qc-pass.md), so that one with no flag has its row.
NAMES = {
    1: "slot-jump", 2: "halted-body-walks", 3: "pinned-runner", 4: "overlap-enemies",
    5: "overlap-own", 6: "crowd", 7: "jump", 8: "down-moves", 9: "facing-in-place",
    10: "flight-toward", 11: "bout-reach", 12: "bout-state", 13: "bout-through", 14: "flicker",
    15: "standard-front", 16: "out-of-place", 17: "forward-idle", 18: "gap", 19: "leaderless",
    20: "far", 23: "slot-twice", 24: "down-rises", 25: "slot-back", 26: "stalled",
    27: "over-packed", 28: "sudden-flight",
}
RADIUS = 15_000
PANEL = 400
POSTURES = ["steady", "forward", "giving ground", "in flight", "down"]
SIDE_FILL = ["#2b6cb0", "#c2410c"]
SIDE_NAME = ["Romans", "Gauls"]


def load(qcdir, name):
    path = Path(qcdir) / name
    return [json.loads(line) for line in path.read_text().splitlines() if line.strip()]


def in_order(f):
    return (ORDER.index(f["variant"]), f["rung"], f["seed"], f["from_s"], f["men"])


def picked(flags, engine=None):
    """The flags to look at, each with an id that names its picture."""
    groups = defaultdict(list)
    for f in sorted(flags, key=in_order):
        if engine is None or f["engine"] == engine:
            groups[(f["n"], f["engine"], f["kind"])].append(f)
    out = []
    for (n, eng, kind), fs in sorted(groups.items()):
        chosen = fs[:PICKED]
        # "More are looked at ... when a count is far larger in one variant": the first flag in
        # the variant with the most of this kind, the earlier variant if two have as many.
        by_variant = Counter(f["variant"] for f in fs)
        most = max(ORDER, key=lambda v: by_variant[v])
        for v in ALSO.get(n, []) + [most]:
            first = next((f for f in fs if f["variant"] == v), None)
            if first is not None and first not in chosen:
                chosen.append(first)
        for k, f in enumerate(chosen, 1):
            f = dict(f)
            f["id"] = f"{n:02d}-{f['detector']}-{eng}" + (f"-{kind}" if kind else "") + f"-{k}"
            f["of"] = len(fs)
            out.append(f)
    return out


def also(flags, path):
    """The flags a file names, beyond the rule: each with an id that says where it is."""
    out = []
    for line in Path(path).read_text().splitlines():
        w = line.split("#")[0].split()
        if not w:
            continue
        eng, rung, variant, seed, n, start = w[0], int(w[1]), w[2], int(w[3]), int(w[4]), float(w[5])
        body = int(w[6]) if len(w) > 6 else None
        found = [f for f in flags
                 if (f["engine"], f["rung"], f["variant"], f["seed"], f["n"], f["from_s"])
                 == (eng, rung, variant, seed, n, start) and body in (None, f["body"])]
        if not found:
            sys.exit(f"no such flag: {line}")
        f = dict(found[0])
        f["id"] = (f"also-{n:02d}-{f['detector']}-{eng}-{variant}-r{rung}-s{seed}-t{start:g}"
                   + (f"-b{body}" if body is not None else ""))
        f["of"] = sum(1 for g in flags if (g["n"], g["engine"], g["kind"]) == (n, eng, f["kind"]))
        out.append(f)
    return out


def counts(qcdir):
    flags = load(qcdir, "flags.jsonl")
    battles = load(qcdir, "battles.jsonl")
    lines = []
    names = dict(NAMES)
    for f in flags:
        names[f["n"]] = f["detector"]
    for eng in ENGINES:
        mine = [b for b in battles if b["engine"] == eng]
        if not mine:
            continue
        lines.append(f"**The {eng} engine, {len(mine)} battles.** Flags by variant; in brackets, "
                     "the battles with at least one.\n")
        lines.append("| # | Detector | " + " | ".join(ORDER) + " | All |")
        lines.append("|---|---|" + "---|" * (len(ORDER) + 1))
        for n in sorted(names):
            row, total, hit = [], 0, set()
            for v in ORDER:
                fs = [f for f in flags if f["n"] == n and f["engine"] == eng and f["variant"] == v]
                row.append(str(len(fs)) if fs else "")
                total += len(fs)
                hit |= {(f["rung"], f["variant"], f["seed"]) for f in fs}
            lines.append(f"| {n} | `{names[n]}` | " + " | ".join(row) + f" | {total} ({len(hit)}) |")
        lines.append("")
        kinds = Counter((f["n"], f["detector"], f["kind"]) for f in flags
                        if f["engine"] == eng and f["kind"])
        if kinds:
            lines.append("By kind: " + "; ".join(
                f"`{d}` {k}: {c}" for (n, d, k), c in sorted(kinds.items())) + ".\n")
        for check in ("dormancy-check", "dormancy-equal"):
            failed = [b for b in mine if b.get(check, "ok") != "ok"]
            lines.append(f"`{check}`: {len(mine) - len(failed)} of {len(mine)} battles pass."
                         + "".join(f"\n- {b['variant']}, rung {b['rung']}, seed {b['seed']}: "
                                   f"{b[check]}" for b in failed))
        errors = [b for b in mine if "error" in b]
        for b in errors:
            lines.append(f"- **A battle did not run:** {b['variant']}, rung {b['rung']}, "
                         f"seed {b['seed']}: {b['error']}")
        lines.append("")
    return "\n".join(lines)


def clip(f):
    start = max(0.0, f["from_s"] - 3.0)
    end = min(f["last_s"], f["from_s"] + 30.0) + 3.0
    cmd = [str(QC), "clip", "--engine", f["engine"], "--rung", str(f["rung"]),
           "--variant", f["variant"], "--seed", str(f["seed"]),
           "--from", str(start), "--to", str(end),
           "--centre", f"{f['at'][0]},{f['at'][1]}", "--radius", str(RADIUS + 3_000)]
    if f["men"]:
        cmd += ["--men", ",".join(map(str, f["men"]))]
    return json.loads(subprocess.run(cmd, check=True, capture_output=True, text=True).stdout)


def moments(f, frames):
    """Four moments to show: just before, the start, and on to the end (or 30 s)."""
    times = [fr["t"] for fr in frames]
    start = f["from_s"]
    end = min(f["last_s"], start + 30.0)
    if end - start < 1.0:
        want = [start - 0.25, start, start + 0.25, start + 2.0]
    else:
        want = [start - 1.0, start, (start + end) / 2, end]
    out = []
    for w in want:
        t = min(times, key=lambda x: abs(x - w))
        if t not in out:
            out.append(t)
    return out


def esc(s):
    return str(s).replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def wrap(text, width):
    words, lines, line = text.split(), [], ""
    for w in words:
        if line and len(line) + 1 + len(w) > width:
            lines.append(line)
            line = w
        else:
            line = (line + " " + w).strip()
    return lines + ([line] if line else [])


def man_svg(x, y, facing, posture, side, r):
    """One man: a disc by his side, drawn by his posture, with a tick for his facing."""
    c = SIDE_FILL[side]
    if posture == 4:
        d = r * 0.8
        return (f'<path d="M{x - d:.1f} {y - d:.1f}L{x + d:.1f} {y + d:.1f}M{x - d:.1f} '
                f'{y + d:.1f}L{x + d:.1f} {y - d:.1f}" stroke="{c}" stroke-opacity="0.55" '
                f'stroke-width="1.6"/>')
    a = facing / 65536 * 2 * math.pi
    tx, ty = x + math.cos(a) * (r + 4), y - math.sin(a) * (r + 4)
    tick = f'<path d="M{x:.1f} {y:.1f}L{tx:.1f} {ty:.1f}" stroke="{c}" stroke-width="1.4"/>'
    if posture == 0:
        body = f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r}" fill="{c}"/>'
    elif posture == 1:
        body = (f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r}" fill="{c}" stroke="#111" '
                f'stroke-width="1.6"/>')
    elif posture == 2:
        body = (f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r}" fill="{c}" fill-opacity="0.25" '
                f'stroke="{c}" stroke-width="1.4"/>')
    else:
        body = (f'<circle cx="{x:.1f}" cy="{y:.1f}" r="{r}" fill="#fff" stroke="#b91c1c" '
                f'stroke-width="1.8" stroke-dasharray="2.5 1.5"/>')
    return tick + body


def panel(f, data, frame, ox, oy):
    meta = data["meta"]
    cx, cy = f["at"]
    scale = PANEL / (2 * RADIUS)
    r = max(280 * scale, 3.4)
    subjects = set(f["men"])
    body = f["body"]

    def at(x, y):
        return ox + (x - cx + RADIUS) * scale, oy + (cy - y + RADIUS) * scale

    def inside(px, py):
        return ox + 2 <= px <= ox + PANEL - 2 and oy + 2 <= py <= oy + PANEL - 2

    out = [f'<rect x="{ox}" y="{oy}" width="{PANEL}" height="{PANEL}" fill="#fbfaf6" '
           f'stroke="#888"/>']
    where = {}
    for i, x, y, a, p, ip, slot in frame["men"]:
        where[i] = at(x, y)
    marks = []
    for i, x, y, a, p, ip, slot in frame["men"]:
        px, py = where[i]
        shown = slot is not None and p != 4 and (i in subjects or meta["body"][i] == body)
        if shown:
            sx, sy = at(*slot)
            if inside(sx, sy):
                marks.append(f'<path d="M{sx - 3:.1f} {sy:.1f}H{sx + 3:.1f}M{sx:.1f} '
                             f'{sy - 3:.1f}V{sy + 3:.1f}" stroke="#444" stroke-width="1"/>')
                if not ip and inside(px, py):
                    marks.append(f'<path d="M{px:.1f} {py:.1f}L{sx:.1f} {sy:.1f}" stroke="#444" '
                                 f'stroke-width="0.7" stroke-dasharray="2 2"/>')
    out += marks
    for a, d, took, gave in frame["bouts"]:
        if a in where and d in where and inside(*where[a]) and inside(*where[d]):
            (x1, y1), (x2, y2) = where[a], where[d]
            out.append(f'<path d="M{x1:.1f} {y1:.1f}L{x2:.1f} {y2:.1f}" stroke="#dc2626" '
                       f'stroke-width="1.2" stroke-opacity="0.8"/>')
    carriers = {b[1] for b in frame["bodies"]}
    for i, x, y, a, p, ip, slot in frame["men"]:
        px, py = where[i]
        if not inside(px, py):
            continue
        out.append(man_svg(px, py, a, p, meta["side"][i], r))
        if i in carriers and p != 4:
            out.append(f'<rect x="{px - 2:.1f}" y="{py - 2:.1f}" width="4" height="4" '
                       f'fill="#fff" stroke="#111" stroke-width="0.8"/>')
    for i in f["men"]:
        if i in where and inside(*where[i]):
            px, py = where[i]
            out.append(f'<circle cx="{px:.1f}" cy="{py:.1f}" r="{r + 4:.1f}" fill="none" '
                       f'stroke="#111" stroke-width="1.6"/>')
            out.append(f'<text x="{px + r + 6:.1f}" y="{py - r - 3:.1f}" font-size="12" '
                       f'font-weight="600" fill="#111" stroke="#fbfaf6" stroke-width="3" '
                       f'paint-order="stroke">{i}</text>')
    label = f"{frame['t']:.2f} s"
    if body is not None and 0 <= body < len(frame["bodies"]):
        label += f"   order: {frame['bodies'][body][0].lower()}"
    out.append(f'<text x="{ox + 8}" y="{oy + 18}" font-size="14" font-weight="600" '
               f'fill="#111" stroke="#fbfaf6" stroke-width="3" paint-order="stroke">'
               f'{esc(label)}</text>')
    return "\n".join(out)


def legend(x, y):
    out, r = [], 5
    out.append(f'<text x="{x}" y="{y + 4}" font-size="12" fill="#333">Romans</text>')
    out.append(man_svg(x + 58, y, 0, 0, 0, r))
    out.append(f'<text x="{x + 82}" y="{y + 4}" font-size="12" fill="#333">Gauls</text>')
    out.append(man_svg(x + 126, y, 32768, 0, 1, r))
    px = x + 160
    for p, name in enumerate(POSTURES):
        out.append(man_svg(px, y, 0, p, 0, r))
        out.append(f'<text x="{px + 13}" y="{y + 4}" font-size="12" fill="#333">{name}</text>')
        px += 30 + 7 * len(name)
    out.append(f'<text x="{px}" y="{y + 4}" font-size="12" fill="#333">'
               '+ slot   □ standard   ○ the men flagged   red line: a bout</text>')
    return "\n".join(out)


def picture(f, data):
    frames = {fr["t"]: fr for fr in data["frames"]}
    shown = moments(f, data["frames"])
    gap, top = 10, 0
    title = (f"{f['n']}. {f['detector']}" + (f" ({f['kind']})" if f["kind"] else "")
             + f": the {f['engine']} engine, rung {f['rung']}, \"{f['variant']}\", "
             f"seed {f['seed']}, {f['from_s']:g} s to {f['last_s']:g} s")
    notes = wrap(f["note"] + "." + (f" Men: {', '.join(map(str, f['men']))}." if f["men"] else "")
                 + (f" Body: {data['meta']['bodies'][f['body']]}."
                    if f["body"] is not None and f["body"] >= 0 else ""), 118)
    head = 26 + 17 * len(notes) + 8
    cols = 2
    rows = (len(shown) + 1) // 2
    width = cols * PANEL + (cols + 1) * gap
    height = head + rows * (PANEL + gap) + 34
    # Square, with the drawing centred: Quick Look crops a picture that is taller than wide.
    side = max(width, height)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {side} {side}" '
           f'width="{side}" height="{side}" font-family="-apple-system, Helvetica, Arial, '
           f'sans-serif">',
           f'<rect width="{side}" height="{side}" fill="#fff"/>',
           f'<g transform="translate({(side - width) / 2:.1f} {(side - height) / 2:.1f})">',
           f'<text x="{gap}" y="{top + 19}" font-size="15" font-weight="700" fill="#111">'
           f'{esc(title)}</text>']
    for k, line in enumerate(notes):
        out.append(f'<text x="{gap}" y="{top + 38 + 17 * k}" font-size="13" fill="#333">'
                   f'{esc(line)}</text>')
    for k, t in enumerate(shown):
        ox = gap + (k % cols) * (PANEL + gap)
        oy = head + (k // cols) * (PANEL + gap)
        out.append(panel(f, data, frames[t], ox, oy))
    bar = 5_000 * PANEL / (2 * RADIUS)
    ly = head + rows * (PANEL + gap) + 12
    out.append(f'<path d="M{width - gap - bar:.1f} {ly}H{width - gap}" stroke="#111" '
               f'stroke-width="2"/>')
    out.append(f'<text x="{width - gap - bar - 30:.1f}" y="{ly + 4}" font-size="12" '
               f'fill="#333">5 m</text>')
    out.append(legend(gap, ly))
    out.append("</g></svg>")
    return "\n".join(out)


def entry(f):
    t = max(0, int(f["from_s"] - 3))
    link = (f"http://127.0.0.1:8377/?engine={f['engine']}&rung={f['rung']}"
            f"&variant={f['variant']}&seed={f['seed']}&t={t}"
            + (f"&man={f['men'][0]}" if f["men"] else ""))
    length = f["last_s"] - f["from_s"]
    return "\n".join([
        f"### {f['id']}",
        "",
        f"![{f['id']}]({f['id']}.svg)",
        "",
        f"- **Where:** the {f['engine']} engine, rung {f['rung']}, `{f['variant']}`, seed "
        f"{f['seed']}, at {f['from_s']:g} s" + (f", for {length:g} s" if length else "") + ".",
        f"- **Measured:** {f['note']}." + (f" Kind: {f['kind']}." if f["kind"] else "")
        + f" One of {f['of']} of this kind on this engine.",
        f"- **In the viewer:** {link}",
        "- **Bin:** not judged yet.",
        "",
    ])


def to_png(svg, scale):
    """A PNG beside the SVG, named as Quick Look names it: ID.svg.png."""
    if shutil.which("qlmanage"):
        subprocess.run(["qlmanage", "-t", "-s", "1640", "-o", str(svg.parent), str(svg)],
                       check=False, capture_output=True)
        return
    browser = shutil.which("chromium") or shutil.which("chromium-browser")
    if browser is None:
        return
    side = re.search(r'width="(\d+)"', svg.read_text()).group(1)
    # A profile of its own, so that several can run at once.
    with tempfile.TemporaryDirectory() as profile:
        subprocess.run([browser, "--headless", "--disable-gpu", "--no-sandbox",
                        "--hide-scrollbars", f"--user-data-dir={profile}",
                        f"--force-device-scale-factor={scale}", f"--window-size={side},{side}",
                        f"--screenshot={svg.resolve()}.png", f"file://{svg.resolve()}"],
                       check=False, capture_output=True)


def main(argv):
    if len(argv) < 3 or argv[1] not in ("counts", "pick", "draw"):
        print(__doc__)
        return 2
    what, qcdir, rest = argv[1], argv[2], argv[3:]
    engine = rest[rest.index("--engine") + 1] if "--engine" in rest else None
    only = rest[rest.index("--only") + 1] if "--only" in rest else None
    if what == "counts":
        print(counts(qcdir))
        return 0
    extra = rest[rest.index("--also") + 1] if "--also" in rest else None
    if extra:
        flags = also(load(qcdir, "flags.jsonl"), extra)
    else:
        flags = picked(load(qcdir, "flags.jsonl"), engine)
    if what == "pick":
        for f in flags:
            print(f"{f['id']}: {f['variant']} r{f['rung']} seed {f['seed']} at {f['from_s']:g} s"
                  f" to {f['last_s']:g} s, men {f['men']}, one of {f['of']}")
        return 0
    pictures = Path(qcdir) / "pictures"
    pictures.mkdir(exist_ok=True)
    pages = defaultdict(list)
    jobs = int(rest[rest.index("--jobs") + 1]) if "--jobs" in rest else 1
    scale = rest[rest.index("--scale") + 1] if "--scale" in rest else "2"
    flags = [f for f in flags if not only or f["id"] == only]

    def draw(f):
        svg = pictures / f"{f['id']}.svg"
        svg.write_text(picture(f, clip(f)))
        if "--png" in rest:
            to_png(svg, scale)
        return svg

    # The work is in the programs run, so threads are enough; the pages keep the picking order.
    with ThreadPoolExecutor(max_workers=jobs) as pool:
        for f, svg in zip(flags, pool.map(draw, flags)):
            pages[f["engine"]].append(entry(f))
            print(svg, flush=True)
    if not only:
        for eng, entries in pages.items():
            name = f"page-also-{eng}.md" if extra else f"page-{eng}.md"
            (pictures / name).write_text("\n".join(entries))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
