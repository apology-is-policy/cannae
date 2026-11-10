# 02 — Equipment as Physics

Equipment is not a +2 attack bonus. Every item has physical properties, and those properties feed
movement, fatigue, combat geometry, protection, perception and morale. Values below are starting
ranges for calibration, not final numbers. Each one will carry its source in the knowledge base.

## 1. What an item affects

| Property | Feeds |
|---|---|
| Mass and where it sits (carried, worn, in hand) | Speed, stamina drain, recovery rate, swimming and fording |
| Length and reach | Who can strike whom; how many ranks fight (phalanx: 5) |
| Attack modes (thrust, cut, throw, shoot) and their tempo | Attack rate, recovery time, energy per attack |
| Required space | Frontage per man; whether he can fight when crowded (Cannae compression) |
| Penetration and blunt trauma against each armour class | Wound outcome by body zone |
| Protection by zone (head, torso, arms, legs; front and back) | Hit-zone resolution. The back is exposed when fleeing. |
| Shield coverage, weight and grip | Block probability, missile cover, a forward-arc bias, fatigue |
| Helmet vision and hearing restriction | Perception radius and arc. Hearing horn signals. |
| Visual signature (crests, colors, nakedness, standards) | Intimidation, identifying friend from foe, officers spotting individual bravery |
| Durability and state | Bent pila, pinned shields, broken spears, an empty quiver |

Loadouts are state machines. A hastatus begins with 2 pila, a sword and a shield. After the
volley he has a sword and a shield. If an enemy pilum pins his shield, he may drop it. A
phalangite whose sarissa breaks or is dropped in a melee falls back to a dagger and a small
shield. When missile troops run dry they become fragile infantry or withdraw. All of this is
tracked per man.

## 2. Hand weapons

| Weapon | Length / mass | Modes | Notes for the simulation |
|---|---|---|---|
| ***Gladius hispaniensis*** | Blade about 60–68 cm; about 0.9–1.2 kg | Thrust (preferred), cut | Works in close order. Short recovery. Thrust wounds to the torso are lethal. |
| **Mainz gladius** (Augustan) | Blade about 50–55 cm, long point | Thrust | Even better in a crush |
| **Celtic long sword** | 2nd century BC about 70–80 cm; 1st century BC up to about 90 cm | Cut | Needs space: lateral clearance for a full swing. Dangerous against unarmoured men. Hampered in a crush or behind a pinned shield. Polybius says they bent; treat that as calibration-suspect. |
| ***Falcata*** | About 55–60 cm, forward-curved | Powerful cut, thrust | Iberian. Strong cut against shield rims and limbs. |
| ***Hasta*** (triarii, auxilia) | About 2–2.5 m | Thrust | Gives a reach advantage in the first exchange. Second-rank support possible. |
| **Hoplite / Hellenistic spear** | About 2–2.5 m | Thrust | — |
| ***Sarissa*** | About 5.5–6.5 m; about 5–6.5 kg | Two-handed thrust | 5 ranks engage. Useless at close range, so the bearer must keep distance. Hard to turn; almost no lateral agility. Turning to face a flank threat means re-forming the block. |
| ***Framea*** | Short shaft, small head | Throw or thrust | Germanic all-purpose weapon |
| ***Rhomphaia* / *falx*** | Long curved blade, two-handed | Cut, hook | Defeats shields and helmets. Needs space, and the user carries no shield. |
| **Xyston / cavalry lance** | About 3.5–4 m | Couched or overarm thrust | Shock value tied to charge speed. Can break. |
| ***Kontos*** (cataphract) | About 3.5–4.5 m, two-handed | Thrust | Penetrates formed ranks of unsteady infantry |
| **Dagger** (*pugio*, *kopis*, ...) | 20–35 cm | Thrust | Last resort, also used to finish off the wounded |

## 3. Missiles

| Missile | Range: effective / maximum | Rate | Ammunition | Notes |
|---|---|---|---|---|
| **Heavy pilum** | About 10–20 m / about 30 m | 1 per throw, usually thrown just before contact | 1–2 | A 60–90 cm iron shank behind a small pyramidal head. Punches through a shield and can **pin** it or wound the man behind. The shank bends so it cannot be thrown back *(debated)*. The volley also disorders the enemy right before contact, which is a morale and cohesion shock. |
| **Light pilum** | About 15–25 m / about 35 m | — | 1 | Thrown first |
| ***Hasta velitaris*** | About 20–30 m / about 40 m | Several per minute | About 5–7 | The head bends on impact |
| **Soliferrum / falarica** | About 15–25 m | — | 1–2 | An all-iron Iberian javelin, high penetration. The falarica could be incendiary. |
| **Generic javelin** | About 20–30 m / about 50 m | Several per minute | 3–7 | Cavalry javelins too (Numidians, Tarantines) |
| **Sling** (Balearic, Rhodian, Achaean) | About 100–200 m against formations / 300–400 m | About 4–8 aimed per minute | 20–50+ lead *glandes* or stones | Lead shot of about 30–60 g. Blunt trauma through armour, concussions. Hard to see in flight, and so demoralising. Outranges Parthian bows (Ventidius). |
| **Composite bow** (Cretan, Parthian, Syrian) | About 50–150 m / 250–300 m | About 6–12 per minute | About 30 per quiver; Parthians resupplied by camel | Damp weakens the sinew. The arrival angle matters: plunging fire over shields versus direct fire. Shields stop most arrows. Men grow weary and helpless under fire they cannot answer. |
| **Scorpio / field artillery** (late Republic) | Several hundred m | Slow | Limited | Caesar used them on field works (Axona). Optional. |

Missiles are physical objects. Each one gets a launch, a flight time with dispersion, and an
impact against a shield, a body zone or the ground. Fire can be aimed at a man or dropped into an
area. With volley timing, plunging arcs and the angle between shields and incoming fire, a
testudo works mechanically. No special rule is needed.

## 4. Body armour and helmets

| Armour | Mass | Coverage | Against cut / thrust / missile / blunt |
|---|---|---|---|
| ***Lorica hamata*** (mail) | About 9–15 kg | Torso, shoulders, upper thighs | Excellent / good / good / poor without padding underneath |
| **Pectorale** (chest plate) | About 1–2 kg | A span square (about 23 cm) over the heart (Polybius 6.23) | A small zone only |
| **Linothorax** | About 4–6 kg | Torso | Good / fair / fair / fair |
| **Bronze cuirass** (muscle or bell) | About 5–8 kg | Torso | Very good, but hot |
| **Scale** (*squamata*) | About 10–15 kg | Torso | Good; weak to upward thrusts |
| ***Lorica segmentata*** (earliest fragments: Dangstetten, about 9 BC) | About 7–9 kg | Torso, shoulders | Very good against cut and thrust |
| **Cataphract full armour** (rider and horse) | Rider about 20+ kg; horse barding 30–40+ kg | Nearly everything | Very good. Heat and fatigue are severe. |
| **Greaves** | 1–2 kg each | Shins | The leading leg, the one exposed under the shield |
| **Helmets:** Montefortino, Coolus, Attic, Boeotian, Thracian, Agen-Port, Imperial Gallic | 1.5–3 kg | Head, with cheek guards | Each type has its own **vision/hearing modifiers**. Open-faced Roman helmets with cheek pieces: decent vision, reduced hearing. Boeotian (cavalry): open, good vision. Many Germans and some Gauls fought bareheaded. |

## 5. Shields

| Shield | Size | Mass | Notes |
|---|---|---|---|
| **Scutum** (oval, then curved rectangle) | About 120–128 × 63–75 cm, curved | About 6–10 kg (the Fayum example is about 10 kg) | Excellent coverage. The iron boss (*umbo*) strikes offensively. Very tiring over hours. Planted, it makes a wall. Overlapped, it makes a testudo. |
| ***Thureos*** (Celtic or Hellenistic oval) | About 110 × 60 cm, flatter | About 4–7 kg | Good coverage. Celtic examples are lighter. |
| ***Aspis* / hoplon** | About 90 cm round | About 6–8 kg | Arm-strapped; needs close order |
| ***Pelte*** (phalangite) | About 60–70 cm | About 2–4 kg | Slung from the neck so both hands hold the sarissa. Weak protection once the man is disarmed or flanked. |
| ***Caetra*** (Iberian) | About 30–50 cm | About 1–2 kg | Buckler for parrying, agile |
| ***Parma*** (velites, cavalry) | About 90 cm round | About 3–4 kg | — |
| **Germanic / Gallic flat shields** | Large, flat | About 4–6 kg | Painted. Easily pinned by pila. |

Shield-specific mechanics:

- The left-side offset: the shield's coverage is skewed. Hoplite lines drifted right as each man
  sought his neighbour's shield (Thucydides). Formations may shift subtly in the simulation.
- Pinned shields, split shields.
- Coverage against plunging versus direct fire.
- Locking shields with neighbours, which requires drill and close order.

## 6. Mounts and animals

- **Horses** were small, about 13–14.2 hands. Breeds varied: hardy Numidian ponies, large
  Nisaean horses for cataphracts.
  - Speeds: walk about 1.7 m/s, trot about 4 m/s, canter about 6–7 m/s, gallop about 12–15 m/s.
  - A gallop is sustainable only for short bursts. Horses tire, need water, panic, and refuse to
    charge into a dense, steady wall of men. That last point decides most cavalry-against-infantry
    outcomes.
  - Horses fear elephants and camels by smell and sight unless habituated. That is a separate
    psyche model for the mount.
- **No stirrups.** The Celtic **four-horned saddle**, adopted by Rome by the 1st century BC, gave a
  firm seat for shock combat. The rider's seat stability modifies whether he is unhorsed and how
  much force his lance delivers.
- **Elephants.**
  - Types: African forest elephants (about 2.5 m at the shoulder, Carthage and Ptolemies) and
    Indian elephants (about 3 m, Seleucids, often with towers).
  - They have their own psyche. Wounds, fire, trumpets, screams and missiles at the mahout drive
    them to panic or a rampage, which hits friend and foe alike.
  - Countermeasures that must work mechanically: skirmishers, lanes, trumpets, missiles, burning
    objects, anti-elephant wagons.
- **Camels**: the Parthian arrow train, Arab camel archers at Magnesia. Horses unused to camels
  are frightened by them.
- **Chariots**: scythed chariots (Seleucid, Pontic) and British war chariots. Both are
  vehicle-animal composites with a crew.

## 7. Encumbrance and the march

- **Battle load versus march load.** A legionary on the march carries about 25–35 kg (pack,
  tools, rations, stakes). When attacked on the march he must drop it and uncover his shield. At
  the Sabis there was no time to take off shield covers or put on helmets (*BG* 2.21).
- Scenarios begin in a given **state of readiness**: marching column, deploying, deployed,
  building camp. Readiness determines how long it takes to fight effectively.
- Water and food states carry over from the scenario's initial conditions. At the Trebia the
  Romans fought cold, wet and unfed; the Carthaginians had eaten and oiled themselves by their
  fires.

## 8. Visual and identity signals

- **Standards** (*signa*, *aquila*, *vexilla*, Celtic boar standards, dragon standards) are
  physical objects carried by agents. They are reference points for alignment, rallying beacons,
  objects of honour, and targets. Losing one is a morale shock. Saving one is a story.
- **Crests, shield devices and tunic colors.** These enable friend-or-foe identification at a
  distance. Identification fails in civil wars, where Roman faces Roman with the same kit.
  Misidentification events (Munda, Philippi) need the means of identification to be modelled.
- **Intimidation signature.** The naked Gaesatae at Telamon, the tall Roman crests, the Germans'
  size as Roman recruits perceived it. Each is an input to the enemy's threat appraisal, not a
  combat stat.
