#!/usr/bin/env python3
"""The page of every flag looked at in the visual QC pass (docs/slice0/qc-pass.md: "Michal
gets a page with a picture and a caption for each flag looked at").

  tools/qc-page.py RESULTS HEADING ENGINE PICTURES OUT

    RESULTS   the list with the bins, docs/slice0/qc-results.md
    HEADING   the heading of the table of flags in it, without the `## `
    ENGINE    field or pairwise
    PICTURES  the folder `tools/qc-render.py draw` wrote: ID.svg and page-ENGINE.md
    OUT       the page is OUT/ENGINE.md and its pictures OUT/ENGINE/ID.svg

The table's rows are `| FLAG | where | bin | reason |`, where FLAG is the detector's number
in backticks, the kind if the detector has several, and the flag's place among the picked
ones: "`01` by-the-lead 2". A row may stand for several flags with one bin and reason:
"`02` 1, 2, 3; 4 = 1" (the fourth is the same drawing as the first), "`18` left 1, 2; 3
(note)" (the third was judged from its note alone), "`19` closer-standing 2 (clip)" (judged
by a replay of the battle). Each flag is joined to the picture of that name and to what the
tool measured there. Nothing is judged here: the bin and the reason are copied.

A picture whose panels are the same drawing as an earlier one's (the same moment in a
variant that shares its start with another) is not copied again; its entry names the other.
The program stops if a row has no picture or a picture no row.
"""
import re
import shutil
import sys
from pathlib import Path


def rows(results, heading):
    """(number, kind or None, place, bin, reason, how judged, the place it is said to equal)
    for each flag of the table."""
    text = Path(results).read_text()
    start = text.find(f"## {heading}\n")
    if start < 0:
        sys.exit(f"no heading {heading!r} in {results}")
    body = text[start:].split("\n## ", 1)[0]
    out = []
    for line in body.splitlines():
        m = re.match(r"\| `(\d\d)` (?:([a-z-]+) )?([\d,;= ()a-z]+?) \| (.*?) \| (.*?) \| (.*) \|$",
                     line)
        if not m:
            continue
        number, kind, flags, _, bin_, reason = m.groups()
        for part in flags.split(";"):
            g = re.match(r"\s*([\d, ]+?)(?: = (\d+)| \((note|clip)\))?\s*$", part)
            if not g:
                sys.exit(f"cannot read the flags of the row: {line}")
            for place in g.group(1).split(","):
                out.append((number, kind, int(place), bin_.strip("*"), reason, g.group(3),
                            int(g.group(2)) if g.group(2) else None))
    return out


def entries(page):
    """What the tool wrote for each picture: its `Where`, `Measured` and `In the viewer`."""
    out = {}
    for block in Path(page).read_text().split("### ")[1:]:
        pid = block.split("\n", 1)[0].strip()
        fields = dict(re.findall(r"^- \*\*(.+?):\*\* (.*)$", block, re.M))
        out[pid] = fields
    return out


def drawing(svg):
    """The panels alone: the title and the measurement above them name the variant."""
    return "\n".join(line for line in Path(svg).read_text().splitlines()
                     if 'font-size="15"' not in line and 'font-size="13"' not in line)


def main(argv):
    if len(argv) != 6:
        print(__doc__)
        return 2
    results, heading, engine, pictures, out = argv[1], argv[2], argv[3], Path(argv[4]), Path(argv[5])
    made = entries(pictures / f"page-{engine}.md")
    table = rows(results, heading)
    used, seen, parts = set(), {}, []
    folder = out / engine
    if folder.exists():
        shutil.rmtree(folder)
    folder.mkdir(parents=True)
    counts = {}
    said, found = {}, {}
    for number, kind, place, bin_, reason, how, equals in table:
        mine = [i for i in made if i.startswith(f"{number}-") and f"-{engine}-" in i]
        if kind:
            ids = [i for i in mine if i.endswith(f"-{engine}-{kind}-{place}")]
        elif len({i.rsplit("-", 1)[0] for i in mine}) == 1:
            # A detector with one kind: the row leaves the kind out.
            ids = [i for i in mine if i.endswith(f"-{place}")]
        else:
            ids = []
        if len(ids) != 1:
            sys.exit(f"`{number}` {kind or ''} {place}: {len(ids)} pictures fit")
        pid = ids[0]
        used.add(pid)
        counts[bin_.split()[0]] = counts.get(bin_.split()[0], 0) + 1
        drawn = drawing(pictures / f"{pid}.svg")
        twin = seen.get(drawn)
        if equals is not None:
            said[pid] = pid.rsplit("-", 1)[0] + f"-{equals}"
        if twin is not None:
            found[pid] = twin
        if twin is None:
            seen[drawn] = pid
            shutil.copyfile(pictures / f"{pid}.svg", folder / f"{pid}.svg")
            shown = f"![{pid}]({engine}/{pid}.svg)"
        else:
            shown = f"*The same drawing as [`{twin}`](#{twin}).*"
        f = made[pid]
        parts.append("\n".join([
            f"### {pid}",
            "",
            shown,
            "",
            f"- **Where:** {f['Where']}",
            f"- **Measured:** {f['Measured']}",
            f"- **Bin:** {bin_}. {reason}" + ("" if reason.endswith(".") else ".")
            + {None: "", "note": " Judged from the flag's note; the picture was not looked at.",
               "clip": " Judged by a replay of the battle."}[how],
            f"- **In the viewer:** {f['In the viewer']}",
            "",
        ]))
    left = sorted(i for i in made if i not in used)
    if left:
        sys.exit(f"pictures without a row: {', '.join(left)}")
    words = {"fine": "{n} fine", "gap": "{n} gaps", "defect": "{n} of a defect"}
    tally = ", ".join(words.get(name, "{n} " + name).format(n=n)
                      for name, n in sorted(counts.items(), key=lambda c: c[0] != "defect"))
    listed = Path(results).name
    head = "\n".join([
        f"# Slice 0, QC pass: every {engine} flag looked at",
        "",
        f"The {len(table)} flags the rule picks on the {engine} engine, each with its picture, "
        "what the detector",
        "measured, the bin it went into and why, and the link that opens the live viewer a few",
        "seconds before it (on the Mac only, while the viewer runs there).",
        "",
        f"- **Bins:** {tally}. The letters are explained in",
        f"  [{listed}](../{listed}), and the page that shows one picture for each kind of",
        "  thing found is [README.md](README.md).",
        "- **A picture** has four panels of 30 m by 30 m: the moments around the flag. A",
        f"  drawing shown once already is not repeated: {len(seen)} pictures for {len(table)} "
        "flags.",
        "- **Made by** [`tools/qc-page.py`](../../../tools/qc-page.py) from the table in",
        f"  {listed} and the pictures of [`tools/qc-render.py`](../../../tools/qc-render.py).",
        "  Nothing here is written by hand.",
        "",
    ])
    (out / f"{engine}.md").write_text(head + "\n" + "\n".join(parts))
    print(f"{len(table)} flags, {len(seen)} pictures, {tally}")
    # What the table says is the same drawing against what the files say. For the reader of
    # this output only: the page shows what the files say.
    for pid in sorted(set(said) | set(found)) if said else []:
        if said.get(pid) != found.get(pid):
            print(f"  {pid}: the table says {said.get(pid, 'its own drawing')}, "
                  f"the files say {found.get(pid, 'its own drawing')}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
