# 08 — Roadmap, Knowledge Base, Reviews

## 1. Vertical slices

The first roadmap ran in layers: physics, then combat, then psyche, then command. Design review 1
showed that it met the real risks last. Nothing before the command phase could tell us whether
A1 or A2 was reachable, and the combat phase's exit tests already needed rout and fear from the
psyche phase. The roadmap is now a sequence of **vertical slices**. Each slice cuts through every
layer (engine, body, psyche, command, validation) at a small scale, and each closes with a design
review.

| Slice | Goal | Contents | Exit criterion |
|---|---|---|---|
| **0. Two spikes** (1–2 weeks) | Retire the two biggest unknowns before building anything else | **A behaviour toy:** two short lines in one dimension, with the psyche of [04](04-psyche.md) §5 and W′. **A performance spike:** 150,000 agents, collision, and a dummy think with realistic memory traffic. | Toy: a stable standoff, pulsing combat, rear-first collapse and the casualty asymmetry appear over a non-trivial parameter region, and the kernel comparison is decided. Spike: sustained cycles per agent-tick measured on the Air, and the hash is identical on 1, 4 and 8 threads. |
| **1. Maniple against warband, and the turned flank** (500–5,000 agents) | The real core, at small scale | The deterministic core (observables lane, intent pass, hashed grid); the unit frame and three drills ([03](03-formations-tactics.md) §1); W′ and psyche v1; a centurion state machine; one superior with a two-task plan and a messenger; a minimal viewer with the inspector; batch metrics | Micro-tests M2, M7, M15 and M16 in band. The global sensitivity analysis has run. |
| **2. Little Cannae at 1:10** | A1's structure, at a scale where a hundred seeds run overnight | Two cavalry types; rally and re-task; the scene graph; the plan representation with eight to ten plays ([09](09-intent-and-plans.md)); compression; the outcome classifiers; experimenter-mode takeover as the officers' test harness. Scenarios start after the skirmishers have withdrawn. | A1 at all three authoring tiers; the Dertosa and Trebia contrasts; the A2 variance decomposition; a scale study at 1:20, 1:10 and 1:5. **Before this slice can run full battles, the sealed battles are chosen and their dossiers hash-committed** ([07](07-validation.md) §5). |
| **3. Scale up** | Full-scale Cannae, then breadth | Full-scale performance work; then the mechanisms the corpus demands, one at a time | A1 at full scale. The sealed battles run once (A3). |
| **Later** | Breadth and interaction | Elephants, chariots, horse archers, the phalanx, fortifications, weather; the role-player mode in full | Pulled in as corpus battles need them |

Large batch runs (acceptance at full scale) need a rented many-core machine, which costs money
and needs Michal's approval ([06](06-architecture.md) §8).

## 2. Scope: what waits

Cannae needs about ten mechanisms. The vision describes several times that many. They stay in the
vision as later features, but a slice builds one only when a named historical pattern cannot be
reproduced without it (pillar 8 in the [README](README.md)). Deferred until a slice needs them:

- per-zone bleeding, helmet-specific hearing, shield drift;
- glycogen and hydration;
- language barriers, omens, looting;
- elephants, chariots, horse archers, sarissa mechanics;
- weather and field fortifications.

## 3. The knowledge base: demand-driven

Format: one Markdown file per entity (troop type, item, formation, commander, battle), with
structured front matter that the data crates can ingest. Each claim cites its source; each number
carries a range and a confidence.

**Written as slices need it,** not ahead of need. The vision's 17,600 words needed thirteen
factual corrections in review 1; a citation-level knowledge base written ahead of need would need
more. The first entries are the troop types on the path to Slice 2: Roman heavy infantry, Gallic
and Iberian infantry, Libyan heavy infantry, Roman and allied cavalry, Hasdrubal's Iberian and
Gallic heavy horse, and Numidian light horse. After that, whatever the sensitivity analysis
flags as mattering. Sealed-battle dossiers follow their own protocol ([07](07-validation.md) §5).

**Checking sources.** A quotation is checked against the primary text, in the original language
where translations differ, before it enters the docs. Review 1's correction about greaves came
from an English translation; the Greek says one greave.

### Ancient sources (with caveats)

- **Polybius**: the best military source for the 3rd–2nd centuries BC; book VI on the army, 18.28–32 on legion against phalanx.
- **Livy**: rich detail. Rhetorical, with inflated numbers.
- **Caesar**: *Bellum Gallicum* and *Bellum Civile*. An eyewitness commander, self-serving. Plus the
  continuators: *Bellum Africum*, *Bellum Hispaniense*, *Bellum Alexandrinum*.
- **Plutarch**: the *Lives* of Fabius, Marcellus, Flamininus, Aemilius Paullus, Marius, Sulla,
  Lucullus, Crassus, Pompey, Caesar, Brutus, Antony.
- **Appian, Cassius Dio, Sallust** (*Jugurtha*), **Frontinus** (*Strategemata*), **Onasander**
  (*Strategikos*, 1st century AD), **Asclepiodotus**, **Aelian** and **Arrian** (*Tactica*), **Tacitus**
  (*Germania*, *Annals*), **Aulus Gellius** (formation terms, 10.9).
- **Vegetius**: late (4th–5th century AD). Use only with care.

### Modern scholarship

- **Battle mechanics:** Goldsworthy, *The Roman Army at War 100 BC–AD 200*; Sabin, "The Face of
  Roman Battle" (JRS 2000); Daly, *Cannae: The Experience of Battle in the Second Punic War*;
  Koon, *Infantry Combat in Livy's Battle Narratives*; Zhmodikov on heavy-infantry missile
  combat; Lendon, *Soldiers and Ghosts*; M. J. Taylor on manipular tactics; Wheeler on the legion
  as phalanx.
- **The army and its equipment:** Keppie, *The Making of the Roman Army*; Bishop & Coulston, *Roman Military Equipment*;
  Quesada Sanz on Iberian arms; Connolly, *Greece and Rome at War*; Sekunda; Head, *Armies of the
  Macedonian and Punic Wars*; Roth, *The Logistics of the Roman Army at War*.
- **Campaigns and topography:** Lazenby, Hoyos, Seibert, Goldsworthy on the Punic Wars; Kromayer & Veith,
  *Antike Schlachtfelder* (battlefield topography); Delbrück (critique of the numbers); Samuels,
  "The Reality of Cannae".
- **The psychology of combat:** Ardant du Picq, *Battle Studies*; Keegan, *The Face of Battle*;
  Holmes, *Acts of War*; Marshall and Grossman, used critically.
- **Modelling:**
  - FitzHugh and Nagumo, excitable systems; Zeeman, catastrophe models of aggression (the first
    draft's inspiration);
  - Sethna et al., hysteresis and avalanches in the random-field Ising model;
  - Helbing, the social force and panic models;
  - Monod and Scherrer / Skiba, the critical-power and W′ models;
  - Grimm et al., pattern-oriented modelling;
  - Saltelli et al., global sensitivity analysis; Craig et al. and Vernon et al., history
    matching;
  - Klein, recognition-primed decision making (for officers, [09](09-intent-and-plans.md));
  - Lanchester and Dupuy, as baselines to *beat*, not to adopt.

## 4. Design reviews

Independent reviews by the newest Fable model, as the model grows and changes. The cadence (from
[CLAUDE.md](../../CLAUDE.md)):

- at the close of every slice;
- before changing an acceptance test or a core mechanism;
- after a major revision of these documents;
- whenever Michal asks.

Each review is saved verbatim in [`docs/reviews/`](../reviews/) with a disposition table: accepted,
partially accepted or rejected, and where it is addressed. Review 1 (2026-11-09) produced the
revision of these documents in arc V.

## 5. Questions, resolved and open

| Question | Resolution |
|---|---|
| **The player's role** | Sets up battles, historical or invented, watches, and can pause and take over commanders. Two modes: experimenter and role-player. See the [README](README.md) and [09](09-intent-and-plans.md). *Michal, 2026-11-09.* |
| **Contested mechanics** | Frontage, intervals and othismos are parameters. One relief mechanism (R2) is built, with a seam for the others ([03](03-formations-tactics.md) §3). *Michal, after review 1.* |
| **Knowledge base or engine first** | Neither: the Slice 0 spikes first, and the knowledge base grows as slices need it. *Michal, after review 1.* |
| **Conflicting sources** | Inputs are sampled continuously across the sources; genuinely discrete alternatives stay discrete ([07](07-validation.md) §1). *Review 1, accepted.* |
| **Determinism** | Bit-exact across machines: pure-Rust `libm`, no FMA, no GPU compute on simulation state ([06](06-architecture.md) §3). *Review 1, accepted.* |
| **The time window** | Open, not blocking. Default: Pyrrhus (280–275 BC) and Idistaviso (AD 16) as stretch scenarios only. |
| **Out of scope** | Open, not blocking. Default: naval warfare, sieges and the campaign layer stay out, except as initial conditions. |
| **Which battles to seal** | Open, decided with Michal before Slice 2 ([07](07-validation.md) §5). |
