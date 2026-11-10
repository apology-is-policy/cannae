# 01 — Armies and Troop Types (264 BC – AD 14)

A troop type in the simulator is not a stat block. It is a **profile** that seeds individual
soldiers:

- **Equipment**: a loadout; see [02-equipment.md](02-equipment.md).
- **Drill**: the formations and maneuvers the type can execute, and how cleanly.
- **Discipline**: how strongly orders override personal drives.
- **Experience**: a distribution over men, not one value per unit.
- **Temperament**: the shape of its fighting. Examples: a fierce first rush with low endurance,
  or slow to start and very hard to break.
- **Cohesion source**: what binds the men together. Examples: drill and centurions, kin and
  chieftain, pay, a king's presence.
- **Social structure**: tent-group, century, clan, retinue.

Numbers below are nominal establishments. Real field strengths were usually lower. Contested
points are marked *(debated)*. Many of them become parameters or scenario variants in the simulator.

---

## 1. Rome

### 1.1 The mid-Republican manipular legion (c. 264 – c. 107 BC)

Main source: Polybius VI.19–42, describing the army of about 220–160 BC.

**Establishment.** 4,200 infantry, raised to about 5,000 in emergencies (the Cannae legions),
plus 300 cavalry.

- 1,200 velites, 1,200 hastati, 1,200 principes.
- 600 triarii. This stays at 600 even when the legion is enlarged.
- 30 maniples, 10 in each heavy line. A maniple has two centuries. Hastati and principes maniples
  are about 120 men; triarii maniples are about 60. The velites are distributed among the
  maniples.
- Each maniple has two centurions. When both are present, the first elected commands the right
  half of the maniple and the second the left; if one is absent, the other commands the whole
  (Polybius 6.24). Each centurion picks an *optio*, who stands at the rear. Each maniple has two
  *signiferi*, chosen as "the bravest and most vigorous".
- Cavalry: 10 *turmae* of 30 men. Each turma has 3 decurions and 3 optiones.
- Six *tribuni militum* per legion, rotating in pairs.

**Consular army.** Two legions plus two allied *alae*. Allied infantry roughly matches the
citizen infantry; allied cavalry is about three times the citizen cavalry. Twelve Roman
*praefecti sociorum* command the allies. The *extraordinarii* are a picked fifth of the allied
infantry and a third of the allied cavalry, kept at the consul's disposal.

**At Cannae.** Eight enlarged legions plus as many allies: about 80,000 infantry and just over
6,000 cavalry (Polybius 3.113). About 10,000 of them were left guarding the camps.

| Type | Who | Kit (summary) | Role and style | Psyche / discipline notes |
|---|---|---|---|---|
| **Velites** | Youngest and poorest citizens | Bundle of light javelins (*hasta velitaris*: shaft about 90 cm and finger-thick; head about 23 cm, thin enough to bend on impact so it cannot be thrown back), *parma* about 90 cm across, sword, plain helmet, sometimes a wolf-skin cover | Skirmish ahead of the line, screen it, then retire through the intervals. Fight elephants and light troops. | Polybius says the wolf skin let officers **see** who was brave. Being observed is a real motivator, so it is a psyche input. Brittle in contact. |
| **Hastati** | Young men, roughly in their 20s | *Scutum* about 120 × 75 cm, *gladius hispaniensis*, 2 pila (one heavy, one light), helmet, one greave (Polybius' word is singular). Most wear a bronze chest plate a span square, about 23 cm, the "heart-guard" (Polybius 6.23). Men above the 10,000-drachma property class wear mail. Crest of 3 feathers about 45 cm tall, "to look twice their height". | First heavy line: pilum volley, then sword fight. | Eager and less steady. Kept in order by centurions in front and optiones behind. |
| **Principes** | Men in their prime, late 20s to 30s | Same as hastati, with more mail | Second line: relieves or reinforces the hastati | More experienced. Their arrival is both a fresh body of men and a morale event. |
| **Triarii** | Veterans, 30s and older | Same, but a thrusting spear (*hasta*) instead of pila, and more mail | Last line. Kneel behind the principes with spears grounded, then form a final wall. "*Res ad triarios rediit*": it has come to the triarii. | Steadiest men, half-size maniples. Calm under pressure. Strong rally anchor for men falling back. |
| **Equites** | Wealthiest citizens | Greek-style lance, shield and cuirass by the 2nd century BC (Polybius 6.25) | Wing cavalry, 300 per legion | Brave but tactically mediocre against Hannibal's horse. Often dismount to fight on foot, as at Cannae. |
| **Socii (alae)** | Italian and Latin allies, in town or ethnic cohorts (Marsi, Paeligni, Samnites, ...) | Converging on legionary equipment through the 2nd century BC | Flank infantry beside the legions, and the bulk of the cavalry | Cohesion through home-town cohorts. Reliability varies with the political moment (defections after Cannae). |
| **Extraordinarii** | Picked allies | As socii | The consul's elite: vanguard on the march, rearguard in retreat | High morale and good drill |

**System mechanics this era needs:**

- The three-line rotation (see [03](03-formations-tactics.md)).
- Velites screening and retiring through the line.
- Maniples as independent tactical bodies.
- Tribunes and centurions as distinct command layers.
- Mixed citizen and allied contingents.

### 1.2 Transition to cohorts (c. 210 – 100 BC)

- Polybius explains the cohort (3 maniples) while describing Scipio at **Ilipa (206 BC)**. So the
  cohort already worked as a maneuver unit in Spain long before Marius.
- The "Marian reforms" (around 107–100 BC) are a convenient label for a gradual process
  *(debated)*: recruiting landless volunteers, state-issued equipment, the eagle (Marius, 104 BC),
  marching with packs ("Marius' mules"), and the disappearance of velites and citizen cavalry.

### 1.3 The late-Republican cohortal legion (c. 100 – 31 BC)

**Establishment.**

- 10 cohorts. Each cohort has 3 maniples, which are 6 centuries of 80 men. About 4,800 nominal.
- Field strength was often far lower. At Pharsalus, Caesar's 80 cohorts totalled 22,000 men,
  about 275 per cohort. Pompey had 110 cohorts and 45,000 men (Caesar, *BC* 3.88–89).
- Uniform kit across the lines: one or two pila, scutum, gladius, mail, Montefortino or Coolus
  helmet.
- The **aquila** (eagle standard). Legion numbers and names now carry identity across decades.

**Officers.**

- **Legatus**: under Caesar, a senatorial deputy commanding one or several legions (Labienus,
  Quintus Cicero, Sabinus and Cotta, the younger Crassus, Antony).
- **Quaestor.**
- **Tribuni militum**: often young political appointees of uneven competence. During the panic
  at Vesontio (*BG* 1.39) they were the first to want to go home.
- **Centurions**: 60 per legion in a strict hierarchy. The **primus pilus** is the senior.
  Centurions lead from the front and die at extraordinary rates: 46 at Gergovia, 30 at
  Pharsalus.
- **Optio**, **signifer**, **aquilifer**, **cornicen** (horn signaller), **tesserarius**
  (watchword).
- **Evocati**: veterans recalled to service.

**Light troops and cavalry** now come from foreigners:

- Cretan and Ituraean archers.
- Balearic slingers.
- Numidian, Gallic, German and Spanish horse.
- Caesar's **antesignani**: picked men "before the standards", used as light infantry and mixed
  with the cavalry at Pharsalus.

**Caesar's legions and the experience gradient.**

- Legio X (*Equestris*, after Caesar mounted it on Gallic horses to meet Ariovistus), VII, VIII,
  IX, XI–XIV and others.
- Veteran and recruit legions behave differently, and Caesar's narrative says so explicitly:
  - **Sabis**: the veterans formed up on their own without orders (*BG* 2.20).
  - **Pharsalus**: the veterans halted mid-charge on their own initiative to catch their breath
    (*BC* 3.92–93).
  - **Forum Gallorum (43 BC)**: veterans fought in silence while the recruits were held back.

These episodes are behavioral specifications. They are not flavor text.

| Type | Kit | Role | Psyche / discipline notes |
|---|---|---|---|
| **Veteran legionary** (Legio X under Caesar) | Mail, scutum, 1–2 pila, gladius | Line infantry, assault, reserve | Self-organizing. Paces his own effort. Hard to panic, but feels betrayal or neglect (the mutiny of 47 BC). Strong rivalry between legions. Intense attachment to the commander. |
| **Recruit legionary** | Same | Same | Same kit, different man. Panics at unfamiliar threats, needs officers nearby, misjudges distances and timing. |
| **Evocati** | Same, often better | Elite reserve, centurion-like presence | Raise the steadiness of the men around them |
| **Antesignani** | Lighter legionary kit | Skirmish, support the cavalry, seize ground | Picked and aggressive |

### 1.4 The Augustan army (31 BC – AD 14)

- A standing professional army: 28 legions after Actium, 25 after the loss of XVII, XVIII and XIX
  at Teutoburg in AD 9.
- Service was 16 years plus 4 as a veteran (13 BC), then 20 plus 5 (AD 5).
- **Auxilia** become regular units:
  - *cohortes peditatae* (infantry cohorts, about 480 men);
  - *alae* (cavalry wings, about 500);
  - *cohortes equitatae* (mixed foot and horse).
  - They are recruited from Gauls, Germans (Batavians), Thracians, Spaniards and Syrian archers,
    and commanded by equestrian *praefecti*.
- New offices: the *praefectus castrorum* (camp prefect), the Praetorian cohorts.
- Kit: mail is still dominant. The earliest *lorica segmentata* fragments come from the
  Dangstetten camp (about 9 BC); more were found at Kalkriese (the Teutoburg battlefield, AD 9).
  The long-pointed Mainz-type gladius. The scutum moves toward the
  curved rectangle.
- **Auxiliaries fight differently from legionaries.** Their drill and discipline come from their
  own traditions, overlaid with Roman training. Their unit cohesion is often ethnic. They usually
  carry lighter armour, a flat oval shield, and a spear (*hasta*) or javelins, and fight in
  looser order. Typical roles are skirmishing, rough ground, screening and pursuit.

### 1.5 Rome's allies and mercenaries across the period

| Contingent | When | Notes |
|---|---|---|
| Massinissa's Numidians | 206 BC on | At Zama: about 4,000 horse and 6,000 foot. Decisive with Laelius' cavalry. |
| Pergamenes (Eumenes II) | 190 BC | At Magnesia his slingers and archers stampeded the scythed chariots |
| Aetolian League cavalry | 200–189 BC | Good light and medium horse (Cynoscephalae) |
| Achaean League | 2nd century BC | Hellenistic-style infantry and cavalry |
| Cretan archers | Throughout | Mercenaries on every side |
| Balearic slingers | Throughout, Roman service after 123 BC | See Carthage |
| Gallic, German and Spanish horse | Caesar | German horse decided cavalry fights before and at Alesia (*BG* 7.67, 7.80). Caesar had sent across the Rhine for horsemen and the light infantry who fought among them, and gives no number (7.65); the 400 often quoted were his German horse at Noviodunum (7.13). Germans mixed horsemen with runners on foot. |
| Galatian legions of Deiotarus | 1st century BC | Roman-style legions raised by a client king; later Legio XXII Deiotariana |

---

## 2. Carthage

Carthage fought mainly with **mercenaries and subject levies**. Citizens fought in emergencies.
Hannibal's army in Italy was a coalition with no common language. Polybius (11.19) marvels that it
never mutinied in 16 years.

**System consequence.** Command runs through national officers and interpreters. Contingents
respond first to their own chiefs. Order transmission across language groups is slower and less
faithful.

**Hannibal's army.**

- Entering Italy: 12,000 African and 8,000 Iberian infantry, and at most 6,000 horse (Polybius
  3.56).
- At Cannae: just over 40,000 infantry, including the Gauls, and about 10,000 horse.

| Type | Kit | Role and style | Psyche / discipline notes |
|---|---|---|---|
| **Libyan / African heavy infantry** | Originally spear and shield, possibly drilled as a phalanx after Xanthippus *(debated)*. Re-armed with captured Roman equipment after the Trebia and Trasimene (Polybius 3.114). | The core of Hannibal's infantry. At Cannae half stood at each end of the line, held back as a reserve (Polybius 3.113), and turned inward on the Roman flanks. Their formation is not recorded; the deep columns of many reconstructions are a scenario variant. | Long-service veterans of Spain, the Alps and three victories. Very high discipline and trust in Hannibal. Able to execute timed, complex maneuvers. |
| **Libyphoenicians** | Similar | Heavy infantry in Spain and Africa | Mixed Punic-Libyan settlers |
| **Carthaginian citizens** | Hoplite or phalanx style | Emergency levy (Zama second line, Bagradas) | Low experience. At Zama they clashed with their own retreating mercenaries. |
| **Iberian *scutarii*** | Large oval shield, *falcata* or straight sword, *soliferrum* or *falarica* javelins. White linen tunics with purple borders (Polybius 3.114). | Medium-heavy line infantry. Alternated with Gallic companies in the Cannae centre. | Good individual swordsmen. Organised by tribe. Can fight in open order or in line. |
| **Iberian *caetrati*** | Small round *caetra* buckler, falcata, javelins | Light and medium infantry, rough ground, ambush | Agile. Skirmish, then close in. |
| **Celtiberians** | Excellent swords (the probable model for the gladius hispaniensis), javelins, varied shields | Shock and line infantry, hired by both sides | Fierce and loyal to pay. In 211 BC their desertion doomed the elder Scipios. |
| **Cisalpine Gauls** (Insubres, Boii) | Long cutting sword, large oval shield, spears. At Cannae they fought naked (Polybius 3.114); Livy has them bare above the navel (*super umbilicum*, 22.46). | Shock infantry with a terrifying first charge. Cannae centre. | Huge initial fury and low endurance (Polybius 2.33, a stereotype to calibrate rather than adopt). Weak drill. Cohesion through chieftain and kin. At Cannae they took about 4,000 of the 5,700 Carthaginian dead. |
| **Ligurians** | Light and medium infantry kit | Zama first line, rough ground | Tough hill fighters |
| **Balearic slingers** | Three slings of different lengths for different ranges, lead and stone shot | Long-range missile troops | Elite missile troops. Trebia and Cannae (Paullus was hit by a sling stone early, per Livy). |
| ***Lonchophoroi* and Mauri** | Javelins, light shields | Light infantry screen | Skirmishers |
| **Numidian light horse** | Small, hardy horses ridden without a bridle (neck rope and stick), javelins, small round shield, no armour | Harass, feigned flight, envelop, pursue | Masters of loose swarming and pursuit. Do not charge formed infantry head-on. Pinned the allied cavalry at Cannae. |
| **Iberian heavy horse** | Spear, sword or falcata, shield. Can dismount. | Shock cavalry | Disciplined. At Cannae they rallied after breaking the Roman horse and rode on to the next task (a crucial behavioural test: rally and re-task after success). |
| **Gallic heavy horse** | Spear, sword, shield, some mail | Shock cavalry | Noble warriors. Aggressive, prone to over-pursue. |
| **War elephants** | North African forest elephants, smaller than Indian. Towers *(debated)*. Mahout. | Shock against infantry and horses unused to them, and screening | Double-edged. At Zama, trumpets and lanes in the Roman line turned them back. At the Metaurus mahouts carried chisel and mallet to kill rampaging animals. 37 crossed the Alps. 80 fought at Zama, mostly fresh and poorly trained. |

**Carthaginian commanders.**

- Hamilcar Barca (Sicily, Mount Eryx).
- Xanthippus, the Spartan mercenary who won at Bagradas in 255 BC with elephants and cavalry on
  open ground.
- Hannibal, his brothers Hasdrubal (the Metaurus) and Mago.
- Hasdrubal, the cavalry commander at Cannae.
- Hanno son of Bomilcar.
- Maharbal ("you know how to win a victory, not how to use it").
- Carthalo.
- Hasdrubal Gisco.

---

## 3. The Hellenistic kingdoms (Macedon, Seleucids, Ptolemies, Pontus, Armenia)

### 3.1 The phalanx and its supports

| Type | Kit | Role and style | Psyche / discipline notes |
|---|---|---|---|
| **Phalangites** (*pezhetairoi*) | *Sarissa* about 6.3 m long (14 cubits, Polybius), two-handed. Small *pelte* shield slung from the neck. Helmet. Front ranks wear a linen or bronze cuirass. | Normally 16 deep. The sarissae of the first 5 ranks project past the front (Polybius 18.29–30). Ranks 6–16 hold theirs angled upward to break missiles. Frontally unbeatable on flat ground. Weak on the flanks and rear, on rough ground, and once its order breaks. | High drill and uniform motion. Cohesion depends on formation integrity. Once ranks open (Pydna, Cynoscephalae), the men are nearly helpless with their small shields and daggers. |
| **Syntagma** (16 × 16 = 256) | — | The basic block. Builds into chiliarchies and *strategiai*. | Natural sub-unit for command and simulation |
| **Hypaspists / *peltastai*** (Antigonid guard), **Argyraspides** (Seleucid Silver Shields, about 10,000) | Phalanx or more flexible kit | Elite guard infantry | Highest phalanx discipline |
| **Chalkaspides / Leukaspides** (Bronze / White Shields) | Phalanx | Macedonian phalanx divisions at Pydna | — |
| ***Thureophoroi* / *thorakitai*** | Oval *thureos* shield, spear, javelins, sword (*thorakitai* add mail) | Versatile medium infantry for rough ground, flank guards, sieges | Flexible |
| **Thracians** | *Rhomphaia* (two-handed curved blade) or javelins and pelte | Peltasts and shock infantry | Fierce; loyalty follows pay |
| **Cretan archers** | Composite bow, small shield, short sword | Missile support | Professional mercenaries |
| **Tarantine cavalry** | Javelins, shield | Light javelin horse | Skirmish and screen |
| **Companions / *hetairoi*** | *Xyston* lance (about 3.5–4 m), cuirass | Shock cavalry, often led by the king in person | Elite. A king leading the charge in person is a command risk (Antiochus III at Magnesia). |
| **Cataphracts** (Seleucid from about 200 BC; Armenian; Parthian) | Horse and rider fully armoured in scale or lamellar, *kontos* lance | Heavy shock cavalry. Devastating head-on, slow, tire fast in heat. | Steady, but exhaustion and heat are major factors |
| **Scythed chariots** | Blades on the axles and yoke | Shock and terror | Easily neutralised. At Magnesia, missiles panicked the horses and the chariots broke their own left. At Chaeronea in 86 BC the Romans jeered and called for more "as at the races". |
| **Indian elephants** (Seleucid) | Larger than African, with towers | Shock, screening | At Raphia (217 BC) the smaller African elephants fled from the Indian ones. At Magnesia, elephants posted in the phalanx's intervals panicked and wrecked it. |
| **Dahae / Arab camel archers / Galatians** | Various | Exotic contingents (Magnesia) | Mixed reliability |
| **Galatians** (Anatolian Celts) | Celtic kit | Mercenary shock infantry and cavalry | Like the Gauls (see §4). Defeated by Manlius Vulso in 189 BC. |
| **Ptolemaic *machimoi*** | Phalanx (trained as phalangites for Raphia) | Native Egyptian line infantry | Raw but drilled |

### 3.2 Pontic and Armenian armies (1st century BC)

- **Mithridates VI** fielded phalanxes that included freed slaves (Chaeronea and Orchomenus,
  86–85 BC). He also had scythed chariots, large cavalry forces and, later, troops trained "in the
  Roman manner". His generals were **Archelaus** and **Taxiles**. At Zela (47 BC) his son
  **Pharnaces** attacked uphill into Caesar.
- **Tigranes the Great** fielded a huge army with cataphracts. At **Tigranocerta (69 BC)**
  Lucullus' small force seized a hill, hit the cataphracts in flank and rear, and broke the whole
  host. Tigranes' quip: "if they are ambassadors, too many; if soldiers, too few."

---

## 4. Gauls, Belgae and Britons (Transalpine, Caesar's era)

| Type | Kit | Role and style | Psyche / discipline notes |
|---|---|---|---|
| **Tribal warriors** | Long iron sword (La Tène III, longer and better than in Hannibal's day), large flat oval shield, spears. Mail is a Celtic invention but rare outside the nobility. | A dense mass, opened by a terrifying charge. Caesar says the Helvetii formed a *phalanx* with overlapping shields (*BG* 1.24–25). Roman pila pinned the shields together, and many men threw their shields away and fought unprotected. | Cohesion through clan and chieftain. Wagon laagers with families watching from behind are a strong motivator and also a trap. Loud: carnyx and war cries. |
| **Retinues** (*ambacti*, *soldurii*) | Best kit | The chieftain's oath-bound bodyguard | Will not survive their lord |
| **Belgae** (e.g. the Nervii) | As above, with little cavalry | At the **Sabis (57 BC)**: an ambush out of the woods, into Caesar's army while it was building camp. Nearly won. | Among the fiercest of the Gauls. The Nervii fought to near-annihilation. |
| **Gallic cavalry** (Aedui, Remi, Treveri) | Spear, sword, shield, mail for the rich. The four-horned saddle gave a firm seat without stirrups. | Shock and skirmish. Caesar's main cavalry source. | Aristocratic, brave. Discipline variable. |
| **Archers** | Bows | Vercingetorix raised them deliberately in 52 BC | Low status |
| **British charioteers** (*essedarii*) | Chariot with warrior and driver. The warrior dismounts to fight, and the chariot waits as a taxi. | Mobility, terror, harassment. Cassivellaunus kept about 4,000 charioteers (*essedarii*) after dismissing most of his army (*BG* 5.19); how many chariots that means is unclear. | Caesar admired their skill |

**Commanders:** Vercingetorix (scorched earth, Gergovia, Alesia), the Helvetian leaders, Boduognatus of the Nervii, Ambiorix (who destroyed Sabinus and Cotta in 54 BC), Commius, Cassivellaunus.

---

## 5. Germans

| Type | Kit | Role and style | Psyche / discipline notes |
|---|---|---|---|
| **Suebi under Ariovistus** (58 BC) | *Framea* (a short-bladed spear for throwing or thrusting), shield, few swords, little armour | A tribal phalanx organised by clan. **Horsemen paired with runners on foot** who held onto the horses' manes (*BG* 1.48). | Women on the wagons urged them on (*BG* 1.51). The matrons, casting lots and divining, declared that the Germans could not win if they fought before the new moon, so Ariovistus avoided a general engagement (*BG* 1.50). A model of how ritual and omens constrain timing. |
| **Cimbri and Teutones** (113–101 BC) | Similar, with some bronze-age-style helmets reported (Plutarch) | Huge migrating hosts. At Vercellae the Cimbri front ranks were reportedly chained together (Plutarch, *Marius* 27, *debated*). | Terrifying to Roman recruits. Marius made his men watch the Teutones from camp for days to get used to them (Plutarch, *Marius* 16), which is an explicit habituation effect. |
| **Cherusci and allies under Arminius** (AD 9) | Light kit | Ambush and attrition from the forest edge against a strung-out marching column in rain | Arminius was a Roman-trained auxiliary officer. He knew the Roman column's weaknesses. |
| **Germanic wedge** (*cuneus*, Tacitus *Germania* 6) | — | A battle line formed of wedges (*acies per cuneos componitur*). Depth and shape are not recorded. "Boar's head" is not Tacitus' term: *caput porcinum*, "pig's head", is late Roman soldiers' slang for a wedge (Vegetius 3.19; Ammianus 17.13.9). | Clan cohesion: "kinsmen fight beside kinsmen". The *barritus* war chant: they judged how the battle would go from the sound of it (Tacitus *Germania* 3). That is a ready-made audio-to-morale feedback loop. |

---

## 6. Iberia (independent), Numidia, Lusitania

- **Lusitanians under Viriathus (147–139 BC):** guerrilla war, feigned flight into mountain
  ambushes, very mobile light infantry and horse.
- **Celtiberians:** see §2. Numantia (siege, out of scope). Fierce in pitched battle.
- **Sertorius (80–72 BC):** trained Iberians in Roman methods. A hybrid force.
- **Cantabrians and Asturians (29–19 BC):** mountain war. The "Cantabrian circle" of javelin
  horsemen wheeling past the enemy (Arrian, later).
- **Numidian kingdoms:**
  - **Jugurtha (112–105 BC):** raids, ambushes, battle at the Muthul.
  - **Juba I (49–46 BC):** destroyed Curio at the Bagradas in 49 BC through a feigned retreat and
    cavalry envelopment. Brought elephants to Thapsus.
  - Numidian infantry also existed, some of it Roman-trained.

---

## 7. Parthia

**Carrhae (53 BC).** Surena fielded about 1,000 cataphracts and about 9,000 horse archers, plus a
camel train carrying fresh arrows. The arrow resupply removed the usual limit on horse archers:
running out of ammunition. Against him, Crassus' square was helpless. His son Publius led a sortie
and was destroyed, and his head was then displayed on a spear, a huge morale shock.

| Type | Kit | Role and style | Psyche / discipline notes |
|---|---|---|---|
| **Horse archers** | Composite bow, about 30+ arrows per quiver, plus camel resupply | Harassment, the "Parthian shot" in feigned flight | Light; avoid contact |
| **Cataphracts** | Full horse and rider armour, kontos | Shock, delivered once the enemy is disordered or has closed up into a dense mass | Nobles |

Later Roman answers:

- **Ventidius Bassus (39–38 BC)** used slingers and fought from high ground. Sling range and
  shock outmatched the horse archers.
- **Antony's retreat (36 BC)**: the testudo against arrows (Plutarch, Cassius Dio 49.30).

---

## 8. Others in scope

- **Illyrians, Dalmatians, Pannonians:** the Great Illyrian Revolt, AD 6–9. Light infantry and
  rough-ground fighting against Tiberius.
- **Dacians** under Burebista (1st century BC): the *falx* (Trajan-era detail is out of scope).
- **The Achaean League and Sparta:** Mantinea, 207 BC, Philopoemen.
- **The Maccabees and Seleucids:** Beth Zachariah, 162 BC, with elephants. Optional.
- **Spartacus' slave army (73–71 BC):** gladiators plus escaped slaves, with captured Roman
  equipment, increasingly drilled. Defeated at the Silarius in 71 BC. Spartacus killed his horse
  before the battle, a commitment signal.
- **Roman against Roman (49–31 BC):** the civil wars are the cleanest **experience-versus-
  discipline** experiments in the corpus. Both sides have identical kit and doctrine; experience,
  leadership and morale differ. See Pharsalus, Munda, Forum Gallorum and Philippi in
  [07](07-validation.md).
- **Pyrrhus of Epirus (280–275 BC):** just before the window. Heraclea, Asculum, Beneventum. The
  first encounters between the legion and the phalanx with elephants. Excellent calibration
  material. At Asculum the Romans fielded anti-elephant wagons.
