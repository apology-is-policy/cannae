# 06 — Architecture and Performance

## 1. Stack decision

**Language: Rust.** Native speed, fearless data-parallelism (rayon), explicit control over memory
layout, a good ecosystem for both headless and GPU work, and no garbage collector pauses.

Alternatives considered:

- C++: the same speed with worse ergonomics and safety.
- Zig or Odin: excellent languages, but a thinner ecosystem.
- Swift with Metal: native, but less control and Apple-only.
- Julia: great for prototyping the numerics, but it has a GC and is weaker for a shipped app.

**Rendering: `winit` + `wgpu` (Metal backend) + `egui`, not SDL2.**

- The rendering problem is about 150,000 circles per frame. With `wgpu` that is **one instanced
  draw call**. Each soldier is a 16-byte instance (position, facing, color/state index). A
  fragment shader draws an anti-aliased circle from a signed distance field (SDF). Cost: about
  1 ms per frame on an M2.
- SDL2's 2D renderer was not built for this. You would have to batch everything through
  `SDL_RenderGeometry` yourself. It also adds a C dependency.
- SDL3's new GPU API is comparable, but its Rust bindings are still young.
- `wgpu` talks to Metal directly on Apple Silicon. It is pure Rust, and `egui` plugs in for
  inspector panels.
- `macroquad` or `bevy` would be faster to the first pixels. But bevy imposes its own ECS and
  scheduler on the simulation, and we deliberately keep the simulation independent of the
  renderer.

**The bottleneck is the simulation, not the drawing.** The machine is a fanless MacBook Air M2
(Mac14,2) with 8 GB of memory: 4 performance cores, 4 efficiency cores and an integrated GPU on
unified memory. Without a fan, sustained all-core load typically throttles by 15–25 %, so we plan
and measure on sustained throughput, never burst. The simulation runs on the CPU cores with rayon
over Structure-of-Arrays data. **The GPU only draws.** GPU compute on simulation state would make
results depend on the GPU's floating point and break bit-exact replays across machines,
including a rented one.

## 2. Workspace layout

```
cannae/
├─ crates/
│  ├─ cannae-sim/        # the model: pure library, no rendering, no I/O beyond traits
│  ├─ cannae-data/       # serde types: troop profiles, equipment, factions, commanders, scenarios
│  ├─ cannae-command/    # officers: scene graph, plan representation, plays, decisions
│  ├─ cannae-aggregate/  # body-level model of formed units, behind a world-view trait
│  ├─ cannae-metrics/    # outcome classifiers, sequence markers, batch metrics
│  ├─ cannae-cli/        # headless runner: run, batch (Monte Carlo), bench, replay-verify
│  └─ cannae-viewer/     # winit + wgpu + egui: live view, replay, inspector, overlays
├─ data/                 # RON/TOML definitions: equipment, troop types, armies, commanders
├─ scenarios/            # battles: terrain, deployments, plans, input distributions
│  └─ sealed/            # hash-committed dossiers of the sealed battles (07 §5)
├─ analysis/             # Python/polars notebooks over batch outputs (Parquet)
└─ docs/
```

The viewer talks to the simulation only through a **snapshot interface** (a read-only view of
state plus an event stream). It can run the simulation in-process or play back recorded runs.

- **cannae-aggregate** models a battle at the level of formed bodies (units), not soldiers. It
  is an officer's mental model, a fast test bed for the command AI, and a cheap rung for
  calibration. It sits behind a world-view trait, so the command AI runs unchanged against the
  full simulation or against the aggregate model.
- **cannae-metrics** holds the outcome classifiers that define A1 ([07](07-validation.md) §6).
  They are code, so they are tested like code.
- **Builds stay light.** 8 GB is tight for rust-analyzer plus a wgpu build. The simulation crates
  build and test without the viewer, which is built only when needed.

## 3. Determinism

- Fixed timestep. Seeded, **counter-based RNG**: each random draw is
  `hash(seed, agent_id, tick, stream)` (Philox- or SplitMix-style). Results therefore do not
  depend on how rayon schedules threads.
- **A double-buffered observables lane.** Position, velocity, facing and posture are the only
  state agents may read about each other. They are read at tick *t* and written for *t+1*.
  Private state (psyche, body, beliefs) updates in place and is read only by its owner. The type
  system then enforces both determinism and the fog-of-war rule.
- **Pairwise events need more than a double buffer.** A strike, a push, a missile hit or a
  grapple involves two agents. An intent pass records what each agent attempts. A resolve pass
  then gathers the events per target, or sorts them by a fixed key, and applies them in that
  order.
- **Parallel work is split into fixed-size blocks,** not by thread count, so the result is the
  same on 1, 4 or 8 threads. Per-thread accumulators are padded to 128 bytes, this CPU's cache
  line, to avoid false sharing.
- Floating point:
  - `f32` with no fast-math and no implicit FMA (Rust does not fuse unless asked).
  - Transcendental functions come from a pure-Rust `libm`, so results do not change across
    platforms.
  - Parallel reductions either use integers or run in a fixed order.
  - No GPU compute on simulation state: GPUs differ in floating point.
- **A determinism hash per tick** goes into the replay. Continuous integration (CI) fails if a
  golden replay diverges. Golden replays only guard refactors that must not change behaviour.
  Intended behaviour changes are judged statistically, on paired seeds (common random numbers)
  over the micro-suite.
- Payoff: bit-exact replays, bisecting behavior regressions to a single commit, and reproducing
  any interesting Monte Carlo seed.

## 4. Tick structure (multi-rate)

| Rate | System |
|---|---|
| 20 Hz (base; we will test whether 10 Hz suffices) | Spatial index rebuild, motor/steering, crowd collision and pressure, melee resolution, missile flight, wounds and bleeding. Galloping cavalry gets substeps (14 m/s would tunnel through bodies at 20 Hz). |
| 2–5 Hz, staggered across agents (each agent's phase slot is assigned by hash) | Perception sampling, appraisal, psyche dynamics, action arbitration. The think rate sets how fast panic can travel, so reaction time is a model parameter. |
| 1–5 Hz | Building coarse fields (density, threat, sound, dust, rumours), unit-level formation frames and slot assignment |
| 0.1–1 Hz | Officer belief updates and decisions; messenger dispatch |
| 0.01–0.05 Hz, or on events | Commander re-planning |

**Activity level of detail (LOD).** Soldiers far from contact, such as a reserve line or men
waiting, run in a cheap mode: rigid motion in formation, psyche updated at 0.5–1 Hz. They promote
to the full model automatically when threat, noise or orders arrive. Early in a battle most men
are in the cheap mode. In the phase A1 cares about, the encirclement, nearly every man is under
pressure for hours, so LOD saves little exactly when it matters. LOD thresholds change behaviour,
so they are model parameters, and LOD on and off are checked for equivalence at small scale.

## 5. Data layout

- **Hand-rolled Structure of Arrays (SoA)**, grouped by access pattern. No generic ECS. Agent
  kinds are few (infantry, cavalry rider+horse, elephant+crew, chariot), and we want explicit
  control over memory and scheduling.
  - Hot every tick: position, velocity, facing, body radius, cell index.
  - Combat state: weapon state, target, attack timer, stance.
  - Psyche and body: about 100 bytes of f32 or 16-bit fixed-point integrator state (see
    [04](04-psyche.md) §5.1).
  - Static: identity, unit, social ties, traits.
  - Rare components (officer, signifer, musician, messenger) live in sparse side tables keyed by
    agent id.
- **Memory locality.** The fat state stays in unit and file order, which is already spatially
  coherent: the men of a file stand together. Each tick a compact proxy (id, position, cell) is
  sorted by cell for spatial queries.
- Budget at Cannae scale: about 136,000 soldiers and about 16,000 horses. The hot per-tick lane
  (about 32 bytes each) is about 5 MB and fits in cache. All state, at roughly 200–300 bytes per
  agent, is 30–45 MB. Memory is not the constraint on an 8 GB machine; bandwidth and cycles are.

## 6. Spatial queries and multi-resolution perception

This makes rich perception affordable for 100,000+ agents:

- **L1, a fine grid (about 2 m cells).** Hashed, or bounded to the occupied region: a dense 2 m
  grid over 5 × 5 km would be 6 million cells. Rebuilt every tick by a parallel counting sort of
  the compact proxy. Used for collision, melee targets and immediate neighbours; the same
  neighbour scan feeds local appraisal. A sarissa's reach is handled by a lookup that spans more
  cells.
- **L2, a medium grid (about 10 m).** Aggregates per cell, built only from observable state:
  friend and enemy counts, mean velocity and facing, how many men face away, run or have fallen,
  combat intensity. Nothing in a field reveals anyone's psyche.
- **L3, a coarse grid (about 40 m).** The same aggregates plus fields for sound emission and
  propagation (shadowed by terrain, as at Gergovia), dust density (advected by wind: Livy's
  Volturnus at Cannae), fog, and the rumour fields ([04](04-psyche.md) §3).
- **Line of sight** comes from a coarse heightmap with precomputed horizon data, plus occlusion
  from the dust and fog fields and from body density in the soldier's own formation.
- **What a soldier's perception costs.** A few nearest individuals from L1, plus a cone sample of
  L2 and L3 cells. Long-range awareness is **O(cells in view), not O(agents in view)**. Interior
  ranks see mostly backs, so long-range perception is shared per file or century: one sample
  serves the men who would see the same thing.

## 7. Physical models

- **Crowd physics.** Soft-body collisions (social force or position-based dynamics), body radius
  about 0.25–0.3 m including the shield. The overlap forces add up to a **pressure field**: support
  from behind, compression, inability to raise weapons, crush injuries in extreme cases (the
  encircled Romans at Cannae).
- **Melee as engagement opportunities**, not animation. Each tick an engaged soldier selects a
  target within reach in his forward arc. He attacks at a rate set by weapon tempo, his stamina
  and his stance. Each attack resolves:
  - hit chance (skill against skill, shield coverage against the attack angle, stance, fatigue,
    crowding);
  - body zone;
  - penetration against that zone's armour;
  - the wound outcome (minor, disabling, lethal; bleeding).

  A defensive stance lowers both the attack rate and the chance of being hit. That makes
  low-lethality standoffs possible, which matches the historical pattern. Second-rank spears,
  and five ranks of sarissae, attack through the gaps. The full design is in
  [10](10-combat-and-wounds.md).
- **Missiles are ballistic** with dispersion and time of flight. They hit shields (absorbed or
  pinning), body zones, or the ground. Volleys at an area and aimed shots at a man.
- **Terrain:**
  - a heightmap at about 1–2 m resolution (a 5 × 5 km field is about 6–25 M cells, which is fine);
  - ground type layers (firm, rough, marsh, sand, wood, river and fords);
  - obstacles as polylines (streams, ditches, palisades, camp walls);
  - effects: slope changes speed, fatigue, impetus and missile range.
- **Weather and time:** wind (dust and missiles), fog with time-varying density, rain (bowstrings,
  footing), heat, the sun's position (glare, heat), daylight. Weather is a later feature, built
  when a slice needs it.

## 8. Performance budget (fanless MacBook Air M2)

Design review 1 found the first estimates optimistic on three counts: the share of active agents,
the cost per agent, and sustained clock speed without a fan. The revised budget, until the Slice 0
spike measures it:

| Quantity | Estimate |
|---|---|
| Agents at Cannae | About 136,000 soldiers and about 16,000 horses |
| Real time at 20 Hz | About 3 M agent-ticks per second |
| CPU throughput | About 17 G cycles/s effective across the P and E cores at burst, and 15–25 % less sustained: about 4,500 cycles per agent-tick available in real time |
| Cost of an active agent | 1,200–2,500 cycles per tick until measured. Tuned particle codes spend hundreds on collision alone; a double buffer in which each agent writes only itself visits every pair twice; cone sampling of L2 and L3 with occlusion costs thousands per think. |
| Active fraction | Low during deployment and the approach; close to 100 % in the encirclement, which lasts hours |
| ⇒ Headless speed | About 3–7× real time over a whole battle at full Cannae scale, falling toward 2× in the encirclement. A battle of several hours takes roughly 40–90 minutes. |

Consequences:

- **The live viewer is safe at 1–4×.** Anything faster is playback of a recorded run.
- **Full-scale Cannae is 5–10 seeds a night** on the Air: enough for development, not for
  acceptance. Acceptance batches run on a rented many-core machine. That costs money, so it needs
  Michal's approval. The headless binary is deterministic across machines (§3), so results
  transfer.
- **Calibration happens small.** A few thousand agents, where a run takes seconds. Little Cannae at
  1:10 takes minutes per seed, so a hundred or more run overnight ([07](07-validation.md) §9).
- **Measure in week one.** The Slice 0 spike runs 150,000 agents with collision and a dummy think
  that has realistic memory traffic. It measures sustained cycles per agent-tick and checks that
  the hash is identical on 1, 4 and 8 threads ([08](08-roadmap.md)).
- Optimization levers in reserve:
  - shared long-range perception per file or century (§6);
  - NEON SIMD on SoA lanes;
  - tighter LOD, with its thresholds treated as model parameters (§4);
  - quiet-period tick throttling (deployment, lulls), likewise a model parameter.

## 9. Recording, replay and analysis

- **Event log.** Every death, wound, rout, rally, order sent, received or misread, rumor, leader
  event and formation event, with tick and position.
- **Keyframes.** Sparse, full-precision state every N ticks. Exact replay re-simulates from the
  nearest keyframe; bit-exact re-simulation cannot start from quantized state.
- **Playback stream.** Quantized state for the viewer: positions as u16 at about 5 cm resolution
  over the field, every frame. It is never used to re-simulate.
- **Player inputs** (pauses, orders, scripts, takeovers) are recorded in the replay, so a run with
  a human in it replays exactly.
- **Batch outputs.** Per-seed summary metrics (outcome class, casualties by type and phase,
  timeline markers) written to Parquet for analysis notebooks.
- **The chronicle (later).** A battle report generated from the event log, written the way
  Polybius would narrate it. A natural fit for an LLM pass over structured events.

## 10. Tooling that makes behavior debuggable

- **Inspector.** Click a soldier to see his traits, body, psyche and body variables (x, F, Φ, R, H, W′), what
  he currently perceives, the inputs to his last decision, and his rumors and beliefs.
- **Overlays.** Heatmaps of x, stamina, pressure, density, sound and dust. Officer belief maps
  against the truth.
- **Time controls.** Pause, step a tick, variable speed, deterministic scrubbing.
- **Behaviour regression.** CI runs the micro-suite on paired seeds and flags statistically
  significant shifts in its metrics.
- **Profiling.** `samply` and Instruments on macOS. `criterion` benchmarks per system in CI.
