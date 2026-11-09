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
  `SDL_RenderGeometry` yourself, and you would get no GPU compute path. It also adds a C
  dependency.
- SDL3's new GPU API is comparable, but its Rust bindings are still young.
- `wgpu` talks to Metal directly on Apple Silicon. It gives us **compute shaders on unified
  memory** for later (no PCIe copies: the CPU and GPU share RAM). It is pure Rust. `egui` plugs in
  for inspector panels.
- `macroquad` or `bevy` would be faster to the first pixels. But bevy imposes its own ECS and
  scheduler on the simulation, and we deliberately keep the simulation independent of the
  renderer.

**The bottleneck is the simulation, not the drawing.** The M2 has 4 performance cores, 4
efficiency cores and a GPU with 8–10 cores on unified memory. The simulation runs on the CPU
cores with rayon over Structure-of-Arrays data. GPU compute is a later optimization for the
uniform, branch-free parts: crowd collision, field building, perception sampling.

## 2. Workspace layout

```
cannae/
├─ crates/
│  ├─ cannae-sim/       # the model: pure library, no rendering, no I/O beyond traits
│  ├─ cannae-data/      # serde types for troop profiles, equipment, factions, commanders, scenarios
│  ├─ cannae-cli/       # headless runner: run, batch (Monte Carlo), bench, replay-verify
│  └─ cannae-viewer/    # winit + wgpu + egui: live view, replay, inspector, overlays
├─ data/                # RON/TOML definitions: equipment, troop types, armies, commanders
├─ scenarios/           # battles: terrain, deployments, plans, weather, variants (per source)
├─ analysis/            # Python/polars notebooks over batch outputs (Parquet)
└─ docs/
```

The viewer talks to the simulation only through a **snapshot interface** (a read-only view of
state plus an event stream). It can run the simulation in-process or play back recorded runs.

## 3. Determinism

- Fixed timestep. Seeded, **counter-based RNG**: each random draw is
  `hash(seed, agent_id, tick, stream)` (Philox- or SplitMix-style). Results therefore do not
  depend on how rayon schedules threads.
- **Double-buffered state.** Read tick *t*, write tick *t+1*. No data races, no order dependence.
- Floating point:
  - `f32` with no fast-math and no implicit FMA (Rust does not fuse unless asked).
  - Transcendental functions come from a pure-Rust `libm`, so results do not change across
    platforms.
  - Parallel reductions either use integers or run in a fixed order.
- **A determinism hash per tick** goes into the replay. Continuous integration (CI) fails if a
  golden replay diverges.
- Payoff: bit-exact replays, bisecting behavior regressions to a single commit, and reproducing
  any interesting Monte Carlo seed.

## 4. Tick structure (multi-rate)

| Rate | System |
|---|---|
| 20 Hz (base; we will test whether 10 Hz suffices) | Spatial index rebuild, motor/steering, crowd collision and pressure, melee resolution, missile flight, wounds and bleeding. Galloping cavalry gets substeps (14 m/s would tunnel through bodies at 20 Hz). |
| 2–5 Hz, staggered across agents (each agent thinks in a phase slot) | Perception sampling, appraisal, psyche dynamics, action arbitration |
| 1–5 Hz | Building coarse fields (density, threat, sound, dust, morale), unit-level formation frames and slot assignment |
| 0.1–1 Hz | Officer belief updates and decisions; messenger dispatch |
| 0.01–0.05 Hz, or on events | Commander re-planning |

**Activity level of detail (LOD).** Soldiers far from contact, such as a reserve line or men
waiting, run in a cheap mode: rigid motion in formation, psyche updated at 0.5–1 Hz. They promote
to the full model automatically when threat, noise or orders arrive. Typically only 10–25 % of a
battle's soldiers are near contact at any moment.

## 5. Data layout

- **Hand-rolled Structure of Arrays (SoA)**, grouped by access pattern. No generic ECS. Agent
  kinds are few (infantry, cavalry rider+horse, elephant+crew, chariot), and we want explicit
  control over memory and scheduling.
  - Hot every tick: position, velocity, facing, body radius, cell index.
  - Combat state: weapon state, target, attack timer, stance.
  - Psyche and body: about 32–48 bytes, quantized u8/u16.
  - Static: identity, unit, social ties, traits.
  - Rare components (officer, signifer, musician, messenger) live in sparse side tables keyed by
    agent id.
- **Memory locality.** Agents are periodically re-sorted by Morton order of their grid cell, so
  that neighbours in space are neighbours in memory.
- Budget at Cannae scale: about 136,000 soldiers and about 16,000 horses, with about 100–150 bytes
  of hot state each, comes to about 15–25 MB. That is roughly the size of the M2's performance
  cluster L2 plus the system-level cache.

## 6. Spatial queries and multi-resolution perception

This makes rich perception affordable for 100,000+ agents:

- **L1, a fine grid (about 2 m cells).** Rebuilt every tick by a parallel counting sort. Used for
  collision, melee targets and immediate neighbours. A sarissa's reach is handled by a lookup
  that spans more cells.
- **L2, a medium grid (about 10 m).** Aggregates per cell: friend and enemy counts, mean
  velocity and facing, mean behavioral state x, combat intensity.
- **L3, a coarse grid (about 40 m).** The same aggregates plus fields for sound emission and
  propagation, dust density (advected by wind: the Volturnus at Cannae) and fog.
- **Line of sight** comes from a coarse heightmap with precomputed horizon data, plus occlusion
  from the dust and fog fields and from body density in the soldier's own formation.
- **What a soldier's perception costs.** A few nearest individuals from L1, plus a cone sample of
  L2 and L3 cells. Long-range awareness is **O(cells in view), not O(agents in view)**.

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
  and five ranks of sarissae, attack through the gaps.
- **Missiles are ballistic** with dispersion and time of flight. They hit shields (absorbed or
  pinning), body zones, or the ground. Volleys at an area and aimed shots at a man.
- **Terrain:**
  - a heightmap at about 1–2 m resolution (a 5 × 5 km field is about 6–25 M cells, which is fine);
  - ground type layers (firm, rough, marsh, sand, wood, river and fords);
  - obstacles as polylines (streams, ditches, palisades, camp walls);
  - effects: slope changes speed, fatigue, impetus and missile range.
- **Weather and time:** wind (dust and missiles), fog with time-varying density, rain (bowstrings,
  footing), heat, the sun's position (glare, heat), daylight.

## 8. Performance budget (M2)

| Quantity | Estimate |
|---|---|
| Agents at Cannae | About 136,000 soldiers and about 16,000 horses |
| Real time at 20 Hz | About 2.7 M agent-ticks per second |
| CPU throughput | About 17 G cycles/s effective across the P and E cores, which is about 6,000 cycles per agent-tick available in real time |
| Planned cost | Active agents about 600–1,000 cycles; LOD agents about 100–150. Weighted average about 250–350. |
| ⇒ Headless speed | About 15–25× real time at full Cannae scale. A battle of several hours takes about 10–15 minutes. |

Consequences:

- **The live viewer runs comfortably in real time with time compression.**
- **Full-scale Monte Carlo is an overnight batch.** About 50–100 Cannae seeds per night on the
  M2. It could be far more on a rented many-core machine; the headless binary runs anywhere.
- **Daily development uses fast proxies.** Micro-scenarios such as a maniple against a warband
  or a turma against horse archers, plus scaled-down battles.
- Optimization levers in reserve:
  - quiet-period tick throttling (deployment and lulls at 5 Hz);
  - NEON SIMD on SoA lanes;
  - GPU compute for collision and field building;
  - tighter LOD.

## 9. Recording, replay and analysis

- **Event log.** Every death, wound, rout, rally, order sent, received or misread, rumor, leader
  event and formation event, with tick and position.
- **Snapshots.** Quantized state every N ticks. Positions as u16 at about 5 cm resolution over the
  field. Replays rebuild exact state by re-simulating from the nearest snapshot.
- **Batch outputs.** Per-seed summary metrics (outcome class, casualties by type and phase,
  timeline markers) written to Parquet for analysis notebooks.
- **The chronicle (later).** A battle report generated from the event log, written the way
  Polybius would narrate it. A natural fit for an LLM pass over structured events.

## 10. Tooling that makes behavior debuggable

- **Inspector.** Click a soldier to see his traits, body, psyche variables (x, F, Φ, R, H), what
  he currently perceives, the inputs to his last decision, and his rumors and beliefs.
- **Overlays.** Heatmaps of x, stamina, pressure, density, sound and dust. Officer belief maps
  against the truth.
- **Time controls.** Pause, step a tick, variable speed, deterministic scrubbing.
- **Profiling.** `samply` and Instruments on macOS. `criterion` benchmarks per system in CI.
