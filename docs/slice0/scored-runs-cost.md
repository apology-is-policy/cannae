# Slice 0: What the scored runs will cost (S0.9)

**Status:** Paullus, 2026-11-10. Measured on the Air while other work was running (load 4-12),
in bursts, not sustained, so every figure here is rough. Run:
`cargo run --release -p cannae-score --bin cell -- --substrate pairwise --rung 1 --seeds 16 --jobs 4`.
Code: `crates/cannae-score`.

## The batch

8 cells (two engines, rungs 0-3) × 200 parameter sets × 10 seeds × 7 variants: **112,000 runs,
56,000 on each engine.** Each run is the patch (322 men) for up to 7,800 simulated seconds.

## What one run costs

Every parameter at its placeholder or its prior's centre, until the sampler exists. Seconds of
wall time per run, with four runs at a time (more did not help while the machine was shared):

| | A whole run (7,800 s) | Stopped 15 minutes after the break |
|---|---|---|
| Pairwise | 10.0 | 1.0-2.1 (rungs 0-3) |
| Field | 2.4-2.6 | 0.24-0.30 |

Alone on one core, a whole run takes 32-34 s on the pairwise engine and 11-12 s on the field.
On the pairwise engine, the contact solver takes about 45 % of the time and the coarse half of
perception about 35 %.

## The whole batch

- **Every run to the end: about 190 hours, eight days of the Air flat out.** That does not fit.
- **With runs stopped 15 minutes after a side breaks: about 24 hours,** if the drawn sets break
  as the centres do. Every tenth of the runs that never break adds about 17 hours. A parameter
  set that passes leaves at most a quarter of its seeds unbroken (pattern 6). If every set
  were like that, the batch would take about three days. Sets that hold longer cost more.

`shock` is written and costs what `base` costs. `volleys` is being written by Varro, and
should cost about the same. `relief` adds a second line, so it will cost more per run.

## The stop rule, proposed

After the first side breaks, a run is scored for 15 more minutes and no longer, whether it
stops there or not (`ROUT_WINDOW_S` in the metrics). Fifteen minutes is the upper end of
pattern 4's own band, the time for a break to spread along the line.

- **What it changes:** patterns 4, 5 and 8 read the rout, and now read only its first 15
  minutes. Everything about the formed phase, and the time to break, is untouched.
- **Why defined this way:** a run that stops at the window's end then scores exactly as the
  same run carried on for two hours. A test checks this on both engines.
- **What it gives up:** losses in a long pursuit. In the patch no one pursues. At the centres
  the losers are kilometres away within half an hour, and no one is chasing them.

**Agreed with Varro** (call 0001, turn 34), **with an audit he added.** For one parameter set
in twenty, the seeds are also run to the end, and the report says how often anything happens
after the window: a bout, a rally on the standard, a second break. That is what the window
could hide. It costs about 9 hours more.

## What the costing runs saw

A run's cost depends on when it breaks, so the break times were read, and they are reported
here. At these settings a side broke in every run, 128 runs on both engines and all four
rungs. Most broke within the first minute: the median break came 32 s after the start, the
range was 15-871 s, and almost no one was down when it happened.

Varro asked one question about these runs, to know which rows of his table of hazards to
look at: which posture came first. It was answered from the same runs, which a seed
reproduces bit for bit (`--example first_turns`). On both engines, every man of the losing
side who turned had been giving ground first, never steady. Most runs began with men stepping
back. He is looking for a fault in the mechanism on scenes of his own, not on the patch.

These runs are not scored. Bouts and gait are still at their placeholders, not drawn. Under the
pre-registration no prior moves because of them. A defect found in them may be fixed, but only
on an argument from the mechanism, not from the pass marks.

## How the batch is run

- **Each cell, variant by variant:** `cell --substrate S --rung R --variant V --draws 200
  --seeds 10 --freeze-seed F --jobs 4 --out DIR`. Each run is appended to
  `DIR/<engine>-r<rung>-<variant>.jsonl` as one JSON line the moment it ends.
- **A stop costs only the runs in flight.** Started again with the same arguments, `cell` skips
  every run already in the file, so the batch can span nights and survive a crash or a
  reboot. A file written from another freeze seed is refused.
- **The audit runs with it:** sets 0, 20, 40 and so on are run a second time to the end, and
  each of those runs records what happened after its window.
- **Scoring:** `score --substrate S --rung R --dir DIR` reads a cell's files and prints the
  pattern verdicts, the dose-response, the knife-edge rule, the shapes, the memory variants
  and the audit ([preregistration](preregistration.md), "Scoring a parameter set").

## If it is still too slow

1. **Fewer seeds per set.** Five seeds instead of ten halves the cost, at the price of noisier
   medians per set.
2. **A rented machine.** Every run to the end is about 700 single-core hours. A 64-core machine
   would take about 11 hours, for tens of dollars. That needs Michal's approval.
3. **Speed work on the pairwise engine,** on the contact solver and perception. Results stay
   bit for bit the same.

**Recommended:** adopt the stop rule and run on the Air over two or three nights. Ask for a
rented machine only if many runs hold for the full two hours.
