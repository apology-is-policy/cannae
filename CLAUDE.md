# CLAUDE.md -- Cannae

## Mission

A soldier-level simulator of land battles from 264 BC to AD 14. Behavior and realism come first;
graphics are minimal. Every soldier is an agent; outcomes emerge from bodies, psyches,
discipline, experience and imperfect command. The design of record is
[`docs/vision/`](docs/vision/README.md). Start there.

## Documents of record

| Path | Role |
|---|---|
| `docs/vision/` | The design. When the design changes, change these docs first, with Michal's approval. Never let code drift from them silently. |
| `docs/STATUS.md` | Live project state: current arc, chunks landed, next step, open decisions. Michal reads it on GitHub from his phone, so keep it current and readable. |
| `docs/reviews/` | Independent design reviews, verbatim, each with a disposition table |

**Deviation tracking.** If the work diverges from the vision, say so explicitly in the commit
message and in `docs/STATUS.md`. If the deviation is load-bearing, propose a vision update instead
of normalizing it.

## Working mode: guided, not autonomous

- **Michal approves arcs.** An arc is a coherent piece of work, for example "revise the vision
  after review 1" or "Slice 0". Inside an approved arc, run from chunk to chunk without asking,
  committing and pushing each chunk. At the arc's end, stop and report.
- **Do not start a new arc** without his go-ahead. No scope pivots.
- **Always ask first, even inside an arc:**
  - changes to the acceptance tests or to a core mechanism in `docs/vision/`;
  - deferring or dropping something the current chunk depends on (pulling a dependency forward is
    fine; note it);
  - destructive git operations (force-push, history rewrites, deleting branches);
  - anything outward-facing beyond pushing this repo. The repo is **public**: everything pushed
    is public;
  - spending money: rented machines, paid APIs.
- **Ask with `AskUserQuestion`** when a decision blocks the arc. Don't bury it in a summary.
- **The stop-nudge hook is silenced for this repo** (`.claude/.no-stop-nudge`, local only, listed
  in `.git/info/exclude`). Remove it when Michal grants autonomy.

## Git and commits

- One commit per chunk. A chunk is coherent and revertable.
- Detailed messages: what changed, why, and the rejected alternative if it isn't obvious. First
  line under about 70 characters.
- Plain ASCII in commit messages (`--`, `->`, straight quotes). Pass the body through a quoted
  HEREDOC.
- Attribution footer: use the lines from the harness's attribution reminder.
- Prefer new commits to `--amend`. Never force-push.
- Push to `origin/main` when a chunk lands inside an approved arc.

## Session continuity

**Memory** lives in `~/.claude/projects/-Users-northkillpd-projects-cannae/memory/`:

- `MEMORY.md`: the index.
- `project_next_session.md`: the pickup pointer. It holds:
  - the tip SHA;
  - the current arc and chunk;
  - what has landed and what is next;
  - traps to avoid;
  - the working set.

**Handoff protocol.** At every boundary (compaction, arc close, end of a session):

1. Update `docs/STATUS.md`.
2. Update `project_next_session.md`.
3. Commit and push.

**Checkpoint contract.** At every resting point:

1. Account for every background task: subagents, shells, watchers. Name any that are still
   running and say why, or say "nothing running".
2. Keep the handoff already written, so Michal can compact at any moment.
3. Say what is next: one `Next` action, and one `Ahead` line to the arc's close.

## Context economy and self-compaction

- **Thresholds** live in `.claude/ctx-thresholds` (CKPT 400k, WARN 500k, HARD 880k). They are
  read by `~/.claude/ctx-hook.sh`, which stays silent below CKPT.
- **At the CHECKPOINT WINDOW line:**
  1. Carry the work to a clean, committed boundary.
  2. Write the resume note, at most 8,000 characters:
     - what is in flight;
     - what must NOT be redone;
     - a `Working set` section listing the files to load first, one `- path` per line
       (`path:A-B` for a window).
  3. Save it to `~/.claude/thyla-selfcompact/-Users-northkillpd-projects-cannae.note`.
  4. Run `tools/selfcompact.sh "<reason>"`.

  The script queues `/compact` in this tmux pane. `~/.claude/resume-note.py` re-injects the note
  on the far side, and `tools/nudge-watch.sh` wakes the new session. The resumed session continues
  the approved arc. If the arc is finished, it reports and waits instead.
- **Only on the real signal.** Once queued, I cannot cancel a `/compact`; only Michal can.
  **Belay:** if HEAD has not moved across two self-compactions, stop and hand back.
- **Past 500k**, start no new chunk. **At 880k**, commit, hand off and stop.
- **Every call re-reads the whole context, so:**
  - Batch independent calls into one message, and status or memory updates into one script.
  - Never poll a background job; wait for its notification.
  - Cap output: count before dumping, and read windows rather than whole files.
  - After a compaction, load the working set first and do not re-survey.
  - When a chunk closes within about 100k of the line, compact before researching the next one.
- **Subagents cannot self-compact.** Give them narrow briefs with named inputs and a reading
  budget.

## Design reviews (Fable)

Michal wants independent reviews by the newest Fable model as the model grows and changes.

- **When:**
  - at the close of every slice;
  - before changing an acceptance test or a core mechanism;
  - after a major revision of `docs/vision/`;
  - whenever Michal asks.

  Propose a review at these points rather than running one unasked. Reviews cost time and tokens.
- **How:** a read-only `Plan` subagent, `model: "fable"`, `effort: "max"`. The brief points it
  at:
  - `docs/vision/` and `docs/STATUS.md`;
  - the changes since the last review;
  - the previous review's disposition table.

  Ask for an independent, critical read: prioritized findings, factual corrections as
  `file:line`, and its model ID on the first line.
- **Record:** save the report verbatim to `docs/reviews/YYYY-MM-DD-<topic>.md` with a disposition
  table (accepted, partial or rejected, and where it is addressed). Relay the findings to Michal
  with my own take. Don't adopt findings silently, and don't dismiss them silently. The reviewer
  advises; Michal decides.

## Invariants

- **Determinism.** Seed plus scenario reproduces a run bit for bit. Wall-clock time never affects
  simulation state: compute budgets are counted in work, never in milliseconds.
- **The model is independent of rendering.** The simulation crate has no renderer dependency.
- **Agents read only each other's observable state:** position, velocity, facing, posture. They
  never read internal psyche variables.
- **No LLM in the simulation loop.** LLM-authored plans never touch the sealed validation
  battles.
- **Cannae is the development scenario, not a holdout.** Sealed battles are hash-committed before
  the engine can run them.
- **Parsimony.** A mechanism enters the model only when a named historical pattern cannot be
  reproduced without it.

## Machine

MacBook Air M2 (Mac14,2), 8 GB RAM, fanless. Keep builds light. Measure sustained (throttled)
performance, not burst. Large batch runs belong on a rented many-core machine, and that needs
Michal's approval because it costs money.

## Style

- Rust: rustfmt and clippy clean.
- Comments explain a non-obvious why, never what.
- No backwards-compatibility shims without a need.
- Direct statements. During long work, give Michal a one-line note when starting a long step,
  changing direction, or finding something that changes the plan.
