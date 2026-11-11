#!/usr/bin/env python3
"""Cuts single panels out of the QC pass's pictures, for a page read on a phone.

  tools/qc-crop.py PICTURES LIST OUT [--width 560] [--scale 2] [--colours 64]

A picture from `tools/qc-render.py draw` has four panels of 30 m by 30 m. Four of them side
by side are too small on a phone, so the page (docs/slice0/qc/README.md) shows one panel, or
a part of one, at a time.

LIST has one cut to a line, `#` starts a comment:

    NAME ID PANEL [X0 Y0 X1 Y1]

    NAME    the cut is written to OUT/NAME.png
    ID      the picture, PICTURES/ID.svg
    PANEL   1 to 4, in reading order
    X0..Y1  a part of the panel, as fractions of its side from its top left corner; the
            whole panel if not given

Nothing is drawn again: the cut is the picture's own SVG with another view box and the
panel's label (its second, and the body's order) moved to the cut's corner, turned into a
PNG by a headless Chromium (--width: the cut's width in CSS pixels; --scale: device pixels to
one of them). If Pillow is there the PNG is reduced to a palette of --colours colours, which
makes it a third of the size; without Pillow it is left as it is.
"""
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

PANEL = 400


def cuts(path):
    for n, line in enumerate(Path(path).read_text().splitlines(), 1):
        words = line.split("#")[0].split()
        if not words:
            continue
        if len(words) not in (3, 7):
            sys.exit(f"{path}:{n}: want NAME ID PANEL [X0 Y0 X1 Y1]")
        part = [float(w) for w in words[3:]] or [0.0, 0.0, 1.0, 1.0]
        if not (0 <= part[0] < part[2] <= 1 and 0 <= part[1] < part[3] <= 1):
            sys.exit(f"{path}:{n}: the part must lie inside the panel")
        yield words[0], words[1], int(words[2]), part


def view(text, panel, part):
    """The cut's view box in the picture's own units."""
    shift = re.search(r'<g transform="translate\(([-\d.]+) ([-\d.]+)\)">', text)
    boxes = re.findall(rf'<rect x="([\d.]+)" y="([\d.]+)" width="{PANEL}" height="{PANEL}" '
                       r'fill="#fbfaf6"', text)
    if shift is None or not 1 <= panel <= len(boxes):
        return None
    x = float(shift.group(1)) + float(boxes[panel - 1][0])
    y = float(shift.group(2)) + float(boxes[panel - 1][1])
    return (x + part[0] * PANEL, y + part[1] * PANEL,
            (part[2] - part[0]) * PANEL, (part[3] - part[1]) * PANEL)


LABEL = re.compile(r'<text x="[\d.]+" y="[\d.]+" font-size="14" font-weight="600" fill="#111" '
                   r'stroke="#fbfaf6" stroke-width="3" paint-order="stroke">(\d+\.\d\d s[^<]*)'
                   r'</text>')


def relabel(text, panel, box, width):
    """The panels' own labels are taken out, since a cut may halve one, and the cut panel's
    label is written at the cut's top left corner, at the size it has on the page."""
    labels = LABEL.findall(text)
    if panel > len(labels):
        return text
    unit = box[2] / width
    mine = (f'<text x="{box[0] + 9 * unit:.2f}" y="{box[1] + 22 * unit:.2f}" '
            f'font-size="{16 * unit:.2f}" font-weight="600" fill="#111" stroke="#fbfaf6" '
            f'stroke-width="{4 * unit:.2f}" paint-order="stroke">{labels[panel - 1]}</text>')
    return LABEL.sub("", text).replace("</svg>", mine + "\n</svg>")


def main(argv):
    if len(argv) < 4:
        print(__doc__)
        return 2
    pictures, listed, out = Path(argv[1]), argv[2], Path(argv[3])
    rest = argv[4:]

    def option(name, default):
        return type(default)(rest[rest.index(name) + 1]) if name in rest else default

    width, scale, colours = option("--width", 560), option("--scale", 2.0), option("--colours", 64)
    browser = shutil.which("chromium") or shutil.which("chromium-browser")
    if browser is None:
        sys.exit("no chromium here")
    try:
        from PIL import Image
    except ImportError:
        Image = None
    out.mkdir(parents=True, exist_ok=True)
    for name, pid, panel, part in cuts(listed):
        text = (pictures / f"{pid}.svg").read_text()
        box = view(text, panel, part)
        if box is None:
            sys.exit(f"{pid}: no panel {panel}")
        height = round(width * box[3] / box[2])
        head = (f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="{box[0]:.2f} {box[1]:.2f} '
                f'{box[2]:.2f} {box[3]:.2f}" width="{width}" height="{height}" ')
        cut, n = re.subn(r'<svg xmlns="http://www.w3.org/2000/svg" viewBox="[^"]*" '
                         r'width="\d+" height="\d+" ', head, relabel(text, panel, box, width),
                         count=1)
        if n != 1:
            sys.exit(f"{pid}: not a picture of qc-render.py")
        png = (out / f"{name}.png").resolve()
        with tempfile.TemporaryDirectory() as tmp:
            svg = Path(tmp) / "cut.svg"
            svg.write_text(cut)
            subprocess.run([browser, "--headless", "--disable-gpu", "--no-sandbox",
                            "--hide-scrollbars", f"--user-data-dir={tmp}/profile",
                            f"--force-device-scale-factor={scale}",
                            f"--window-size={width},{height}", f"--screenshot={png}",
                            f"file://{svg}"], check=False, capture_output=True)
        if not png.exists():
            sys.exit(f"{name}: chromium wrote nothing")
        if Image is not None:
            Image.open(png).convert("RGB").quantize(
                colors=colours, dither=Image.Dither.NONE).save(png, optimize=True)
        print(f"{name}.png  {png.stat().st_size // 1024} KB  {pid} panel {panel}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
