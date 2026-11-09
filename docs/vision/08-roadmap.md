# 08 — Roadmap, Knowledge Base, Open Questions

## 1. Phases

| Phase | Goal | Exit criterion |
|---|---|---|
| **0. Vision** (this document set) | A shared picture of the scope and the core ideas | You sign off on the vision and answer the open questions below |
| **1. Knowledge base** | Every troop type, item, formation, commander and battle as a sourced entry. Every number carries a citation and an uncertainty range. | Complete coverage of [01](01-armies.md)–[03](03-formations-tactics.md) at citation level. Battle dossiers for the corpus. |
| **2. Matchup research** | "How each fares against each": troop type against troop type, under conditions (terrain, frontal or flank, fresh or tired, weather, experience), argued from sources | A matchup matrix with justifications, which becomes the expected bands for the micro-tests |
| **3. Engine skeleton** | Cargo workspace, deterministic tick, SoA, L1–L3 grids, crowd physics, formation slots and drill transitions, wgpu viewer, replay | 150,000 agents marching and deploying at Cannae scale in real time. Determinism hash stable. |
| **4. Body and combat** | Melee opportunity model, missiles, wounds, W′ stamina, encumbrance, crowd pressure | Micro-tests M1, M3, M4, M14, M15 in band |
| **5. Psyche** | Perception channels, appraisal, cusp dynamics, coupling, rumors, rout, pursuit, rally, surrender | M2, M6–M13, M16 in band. Stylized facts S1–S3 emerging. |
| **6. Command and control** | Officers, belief maps, orders and messengers, signals, plans and triggers, commander personalities | Cynoscephalae, Sabis and Gergovia behaviors appear. A Philippi-style misreading is possible. |
| **7. Calibration** | Fit to the calibration set. Score the hypothesis modes. | Calibration battles in band |
| **8. Acceptance** | Run the holdout set, Cannae included | A1, A2 and A3 pass, or we learn precisely why not |
| **9. Interaction** *(optional)* | Command one officer through his own belief map and channels | Decide after Phase 6 |

Phases 1–2 (research) and 3 (engine) can run in parallel. The engine skeleton does not depend on
the numbers.

## 2. The knowledge base

Planned format: one Markdown file per entity (troop type, item, formation, commander, battle)
with structured front matter that the data crates can ingest. Each claim cites its source; each
number carries a range and a confidence.

### Ancient sources (with caveats)

- **Polybius**: the best military source for the 3rd–2nd centuries BC; book VI on the army, 18.28–32 on legion against phalanx.
- **Livy**: rich detail. Rhetorical, with inflated numbers.
- **Caesar**: *Bellum Gallicum* and *Bellum Civile*. An eyewitness commander, self-serving. Plus the
  continuators: *Bellum Africum*, *Bellum Hispaniense*, *Bellum Alexandrinum*.
- **Plutarch**: the *Lives* of Fabius, Marcellus, Flamininus, Aemilius Paullus, Marius, Sulla,
  Lucullus, Crassus, Pompey, Caesar, Brutus, Antony.
- **Appian, Cassius Dio, Sallust** (*Jugurtha*), **Frontinus** (*Strategemata*), **Onasander**
  (*Strategikos*, 1st century AD), **Asclepiodotus**, **Aelian** and **Arrian** (*Tactica*), **Tacitus**
  (*Germania*, *Annals*).
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
  - Zeeman, catastrophe theory models of aggression;
  - Sethna et al., hysteresis and avalanches in the random-field Ising model;
  - Helbing, the social force and panic models;
  - Monod and Scherrer / Skiba, the critical-power and W′ models;
  - Grimm et al., pattern-oriented modelling;
  - Lanchester and Dupuy, as baselines to *beat*, not to adopt.

## 3. Open questions for you

1. **Your role.** A pure observer and sandbox at first (my recommendation), with commanding one
   officer through realistic C2 later? Or should playing as a commander be a goal from day one?
2. **The time window.** Include Pyrrhus (280–275 BC) for calibration, and Idistaviso (AD 16) as a
   stretch?
3. **Out of scope.** Agree that naval warfare, sieges and the campaign layer stay out, apart from
   initial conditions?
4. **Conflicting sources.** Agree on variants per source, with the acceptance bands required to
   hold across the plausible variants?
5. **Determinism.** Same-machine reproducibility is enough to start. Cross-platform comes almost
   free with pure-Rust `libm` and no FMA. Fine?
6. **Hypothesis modes.** Do you want the contested mechanics (line relief, othismos, frontage)
   switchable as a research feature? I recommend yes. It is cheap if we design for it now.
7. **Next deliverable.** The Phase 1 knowledge base first (research-heavy), or Phase 3 engine
   skeleton work in parallel so we have 150,000 circles on screen early?
