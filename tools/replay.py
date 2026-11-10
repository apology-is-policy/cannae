#!/usr/bin/env python3
"""Turns a directory of the harness's SVG frames into one self-contained replay page.

    tools/replay.py FRAMES_DIR OUT.html [--title TEXT] [--note TEXT]

The frames are those written by `--svg-dir` (crates/cannae-harness/src/svg.rs): one circle per
man, in the order of the men, so a man is the same index in every frame. The page moves each
man in a straight line between two frames; his posture is the earlier frame's.
"""
import json
import os
import re
import sys

MAN = re.compile(
    r'<circle cx="(-?\d+)" cy="(-?\d+)" r="\d+" fill="([^"]+)" stroke="([^"]+)" '
    r'stroke-width="(\d+)"/>'
    r'(?:<line x1="(-?\d+)" y1="(-?\d+)" x2="(-?\d+)" y2="(-?\d+)"[^>]*/>)?'
)
SIDES = {"#a8322d": 0, "#2d5ea8": 1}


def posture(fill, stroke):
    if fill == "#9a968c":
        return 4  # down
    if fill == "#e8c547":
        return 3  # turned
    if fill == "#f4f1ea":
        return 2  # back
    return 1 if stroke == "#111" else 0  # forward, steady


def main():
    args = sys.argv[1:]
    opts = {"--title": "Replay", "--note": ""}
    for k in list(opts):
        if k in args:
            i = args.index(k)
            opts[k] = args[i + 1]
            del args[i : i + 2]
    if len(args) != 2:
        sys.exit(__doc__)
    src, out = args
    names = sorted(n for n in os.listdir(src) if n.endswith(".svg"))
    if not names:
        sys.exit(f"no .svg frames in {src}")
    frames, times, side, lead = [], [], None, None
    for name in names:
        men = MAN.findall(open(os.path.join(src, name)).read())
        if side is None:
            side = [SIDES.get(m[2], SIDES.get(m[3], 0)) for m in men]
            lead = [1 if m[4] == "80" else 0 for m in men]
        if len(men) != len(side):
            sys.exit(f"{name}: {len(men)} men, the first frame has {len(side)}")
        xs, ys, ps, fx, fy = [], [], [], [], []
        for cx, cy, fill, stroke, _, x1, y1, x2, y2 in men:
            xs.append(int(cx))
            ys.append(int(cy))
            ps.append(posture(fill, stroke))
            fx.append(int(x2) - int(x1) if x1 else 0)
            fy.append(int(y2) - int(y1) if y1 else 0)
        frames.append([xs, ys, ps, fx, fy])
        # The harness names a frame by its second: <scenario>-<seconds>.svg.
        times.append(int(re.search(r"-(\d+)\.svg$", name).group(1)))
    data = {"t": times, "side": side, "lead": lead, "f": frames}
    page = (
        PAGE.replace("__TITLE__", opts["--title"])
        .replace("__NOTE__", opts["--note"])
        .replace("__DATA__", json.dumps(data, separators=(",", ":")))
    )
    with open(out, "w") as f:
        f.write(page)
    print(f"{out}: {len(frames)} frames, {len(side)} men, {os.path.getsize(out) // 1024} KB")


PAGE = r"""<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>__TITLE__</title>
<style>
:root {
  --bg: #fbfaf7; --ink: #1c1a17; --muted: #6b665c; --line: #d9d4c8; --field: #f4f1ea;
  --a: #a8322d; --b: #2d5ea8; --turned: #e8c547; --down: #9a968c; --edge: #111111;
  --accent: #1c1a17; --accent-ink: #fbfaf7;
}
@media (prefers-color-scheme: dark) {
  :root {
    --bg: #171614; --ink: #ece8df; --muted: #a39d90; --line: #3a372f; --field: #24221e;
    --a: #e0635c; --b: #6f9fe6; --turned: #e8c547; --down: #6e6a61; --edge: #f2efe8;
    --accent: #ece8df; --accent-ink: #171614;
  }
}
* { box-sizing: border-box; }
body { margin: 0; background: var(--bg); color: var(--ink);
  font: 16px/1.45 system-ui, -apple-system, "Segoe UI", sans-serif; }
main { max-width: 980px; margin: 0 auto; padding: 20px 16px 32px; }
h1 { font-size: 1.25rem; margin: 0 0 4px; }
p.note { margin: 0 0 14px; color: var(--muted); font-size: 0.92rem; }
canvas { display: block; width: 100%; height: min(62vh, 620px); background: var(--field);
  border: 1px solid var(--line); border-radius: 8px; touch-action: none; }
.bar { display: flex; flex-wrap: wrap; gap: 10px; align-items: center; margin: 12px 0 6px; }
button, select { font: inherit; color: var(--ink); background: transparent;
  border: 1px solid var(--line); border-radius: 6px; padding: 7px 12px; min-height: 40px; }
button#play { background: var(--accent); color: var(--accent-ink); border-color: var(--accent);
  min-width: 84px; }
button[aria-pressed="true"] { border-color: var(--ink); }
input[type=range] { flex: 1 1 220px; min-width: 160px; accent-color: var(--ink); }
#clock { font-variant-numeric: tabular-nums; min-width: 5.5em; }
#count { color: var(--muted); font-size: 0.92rem; font-variant-numeric: tabular-nums; }
ul.legend { list-style: none; display: flex; flex-wrap: wrap; gap: 6px 18px; padding: 0;
  margin: 10px 0 0; font-size: 0.92rem; color: var(--muted); }
ul.legend i { display: inline-block; width: 12px; height: 12px; border-radius: 50%;
  margin-right: 6px; vertical-align: -1px; border: 2px solid transparent; }
</style>
</head>
<body>
<main>
<h1>__TITLE__</h1>
<p class="note">__NOTE__</p>
<canvas id="c" aria-label="The men of both sides, drawn from above"></canvas>
<div class="bar">
  <button id="play">Play</button>
  <input id="seek" type="range" min="0" max="1000" value="0" aria-label="Time">
  <span id="clock">0 s</span>
  <select id="speed" aria-label="Speed">
    <option value="1">1x</option><option value="2">2x</option>
    <option value="5" selected>5x</option><option value="10">10x</option>
  </select>
  <button id="near" aria-pressed="true">The line</button>
  <button id="far" aria-pressed="false">Whole field</button>
</div>
<div id="count"></div>
<ul class="legend">
  <li><i style="background:var(--a)"></i>Romans</li>
  <li><i style="background:var(--b)"></i>Warband</li>
  <li><i style="background:var(--b);border-color:var(--edge)"></i>Going forward</li>
  <li><i style="background:var(--field);border-color:var(--a)"></i>Giving ground</li>
  <li><i style="background:var(--turned);border-color:var(--a)"></i>Turned to flee</li>
  <li><i style="background:var(--down);width:8px;height:8px"></i>Down</li>
</ul>
</main>
<script>
const D = __DATA__;
const N = D.side.length, T = D.t, F = D.f;
const cv = document.getElementById('c'), g = cv.getContext('2d');
const css = k => getComputedStyle(document.documentElement).getPropertyValue(k).trim();
let col = {};
function colours() {
  for (const k of ['a', 'b', 'turned', 'down', 'edge', 'field']) col[k] = css('--' + k);
}
function bounds(frames) {
  let b = [Infinity, Infinity, -Infinity, -Infinity];
  for (const f of frames) for (let i = 0; i < N; i++) {
    b[0] = Math.min(b[0], f[0][i]); b[1] = Math.min(b[1], f[1][i]);
    b[2] = Math.max(b[2], f[0][i]); b[3] = Math.max(b[3], f[1][i]);
  }
  const m = 0.08 * Math.max(b[2] - b[0], b[3] - b[1]) + 1500;
  return [b[0] - m, b[1] - m, b[2] + m, b[3] + m];
}
const NEAR = bounds([F[0]]), FAR = bounds(F);
let view = NEAR.slice(), now = T[0], playing = false, last = 0;
function size() {
  const r = cv.getBoundingClientRect(), d = window.devicePixelRatio || 1;
  cv.width = Math.round(r.width * d); cv.height = Math.round(r.height * d);
}
function draw() {
  let k = 0;
  while (k + 1 < T.length && T[k + 1] <= now) k++;
  const a = F[k], b = F[Math.min(k + 1, F.length - 1)];
  const u = k + 1 < T.length ? (now - T[k]) / (T[k + 1] - T[k]) : 0;
  const w = cv.width, h = cv.height;
  const s = Math.min(w / (view[2] - view[0]), h / (view[3] - view[1]));
  const ox = (w - s * (view[2] - view[0])) / 2 - s * view[0];
  const oy = (h - s * (view[3] - view[1])) / 2 - s * view[1];
  g.fillStyle = col.field; g.fillRect(0, 0, w, h);
  const r = Math.max(280 * s, 1.6), lw = Math.max(60 * s, 1);
  let up = [0, 0], fled = [0, 0], down = [0, 0];
  // Men down first, so that the standing are drawn over them.
  for (const pass of [0, 1]) for (let i = 0; i < N; i++) {
    const p = a[2][i];
    if ((p === 4) !== (pass === 0)) continue;
    const x = ox + s * (a[0][i] + u * (b[0][i] - a[0][i]));
    const y = oy + s * (a[1][i] + u * (b[1][i] - a[1][i]));
    const side = D.side[i], c = side ? col.b : col.a;
    if (p === 4) {
      down[side]++;
      g.fillStyle = col.down; g.beginPath(); g.arc(x, y, r / 2, 0, 7); g.fill();
      continue;
    }
    if (p === 3) fled[side]++; else up[side]++;
    g.beginPath(); g.arc(x, y, r, 0, 7);
    g.fillStyle = p === 2 ? col.field : p === 3 ? col.turned : c;
    g.fill();
    g.lineWidth = D.lead[i] ? 2 * lw : lw;
    g.strokeStyle = p === 1 ? col.edge : c;
    g.stroke();
    g.beginPath(); g.moveTo(x, y); g.lineTo(x + s * a[3][i], y + s * a[4][i]);
    g.lineWidth = lw; g.strokeStyle = col.edge; g.stroke();
  }
  document.getElementById('clock').textContent = now.toFixed(0) + ' s';
  document.getElementById('seek').value = 1000 * (now - T[0]) / (T[T.length - 1] - T[0] || 1);
  document.getElementById('count').textContent =
    `Romans: ${up[0]} standing, ${fled[0]} fleeing, ${down[0]} down.  ` +
    `Warband: ${up[1]} standing, ${fled[1]} fleeing, ${down[1]} down.`;
}
function tick(ms) {
  if (playing) {
    now += (ms - last) / 1000 * +document.getElementById('speed').value;
    if (now >= T[T.length - 1]) { now = T[T.length - 1]; toggle(false); }
    draw();
    requestAnimationFrame(tick);
  }
  last = ms;
}
function toggle(on) {
  playing = on;
  document.getElementById('play').textContent = on ? 'Pause' : 'Play';
  if (on) {
    if (now >= T[T.length - 1]) now = T[0];
    last = performance.now(); requestAnimationFrame(tick);
  }
}
document.getElementById('play').onclick = () => toggle(!playing);
document.getElementById('seek').oninput = e => {
  now = T[0] + e.target.value / 1000 * (T[T.length - 1] - T[0]); draw();
};
function look(v, on, off) {
  view = v.slice();
  document.getElementById(on).setAttribute('aria-pressed', 'true');
  document.getElementById(off).setAttribute('aria-pressed', 'false');
  draw();
}
document.getElementById('near').onclick = () => look(NEAR, 'near', 'far');
document.getElementById('far').onclick = () => look(FAR, 'far', 'near');
// Drag to move the view, wheel or pinch to zoom about the pointer.
const held = new Map();
let pinch = 0;
function zoom(f, px, py) {
  // The view is drawn at one scale and centred, so undo the same fit here.
  const r = cv.getBoundingClientRect(), vw = view[2] - view[0], vh = view[3] - view[1];
  const k = Math.min(r.width / vw, r.height / vh);
  const wx = view[0] + (px - r.left - (r.width - k * vw) / 2) / k;
  const wy = view[1] + (py - r.top - (r.height - k * vh) / 2) / k;
  view = [wx + (view[0] - wx) * f, wy + (view[1] - wy) * f,
          wx + (view[2] - wx) * f, wy + (view[3] - wy) * f];
  draw();
}
cv.onpointerdown = e => { cv.setPointerCapture(e.pointerId); held.set(e.pointerId, e); };
cv.onpointerup = cv.onpointercancel = e => { held.delete(e.pointerId); pinch = 0; };
cv.onpointermove = e => {
  const was = held.get(e.pointerId);
  if (!was) return;
  held.set(e.pointerId, e);
  const r = cv.getBoundingClientRect();
  if (held.size === 1) {
    const k = Math.max((view[2] - view[0]) / r.width, (view[3] - view[1]) / r.height);
    const dx = (e.clientX - was.clientX) * k, dy = (e.clientY - was.clientY) * k;
    view = [view[0] - dx, view[1] - dy, view[2] - dx, view[3] - dy];
    draw();
  } else if (held.size === 2) {
    const [p, q] = [...held.values()];
    const d = Math.hypot(p.clientX - q.clientX, p.clientY - q.clientY);
    if (pinch) zoom(pinch / d, (p.clientX + q.clientX) / 2, (p.clientY + q.clientY) / 2);
    pinch = d;
  }
};
cv.addEventListener('wheel', e => {
  e.preventDefault(); zoom(e.deltaY > 0 ? 1.15 : 1 / 1.15, e.clientX, e.clientY);
}, { passive: false });
matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => { colours(); draw(); });
addEventListener('resize', () => { size(); draw(); });
colours(); size(); draw();
</script>
</body>
</html>
"""

if __name__ == "__main__":
    main()
