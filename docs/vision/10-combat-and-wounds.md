# 10 — The Contact Zone, Combat and Wounds

What happened where the lines touched? Which weapon is best at which distance, and which fares
badly against which? Should combat use the dodge and critical-hit rolls familiar from
role-playing games? Should wounds be remembered, and should they affect both body and mind? This
document answers those questions. The short answers: the contact zone was a pulsing string of
duels, not a grinding melee; there are no dodge or crit rolls, because geometry decides; and
wounds are tracked by body zone and a blood pool, feeding both the body and the psyche.

Equipment properties are in [02](02-equipment.md); the psyche in [04](04-psyche.md).

## 1. What the contact zone looked like

- **Not a continuous melee.** Combat came in surges and lulls (Sabin; Goldsworthy). Heavy
  infantry threw a great deal (Zhmodikov). The front rank was a string of duels, and most men
  were not fighting at any moment: they supported, pushed, waited and feared
  ([03](03-formations-tactics.md) §4).
- **Room to fight.** A Roman needed at least three feet clear of the men beside, in front and
  behind, so against a phalanx "one Roman must stand opposite two men in the first rank", facing
  ten pikes (Polybius 18.30).
- **Closing in.** Against the Insubres in 223 BC, the Romans came to close quarters and so robbed
  the Gauls "of the power of raising their hands and cutting", then thrust at breast and face
  (Polybius 2.33). Dionysius describes Romans ducking under raised Gallic swords, crouching low,
  and striking at groin, sides and breast (14.10).
- **The shield** was the main defence and a weapon: its boss punched. Pila pinned shields
  together, so many Helvetii threw theirs away and fought unprotected (*BG* 1.25). At the Vosges,
  Roman soldiers leapt onto the German shield wall, tore shields away and struck down from above
  (*BG* 1.52).
- **What it left behind.** At Zama the ground between the lines was choked with slippery corpses
  and discarded arms; Scipio carried his wounded to the rear and re-formed before the next
  clash (Polybius 15.14). After Cannae, men were found alive the next morning with their thighs
  and hamstrings cut, baring their throats to be finished (Livy 22.51).

## 2. No dodge-and-crit table

Flat dodge and critical-hit percentages hide exactly what decided ancient fights: which side the
shield was on, whether there was room to swing, and whether a man faced one enemy or two. Each
engaged pair is resolved through a chain instead:

1. **Geometry.** Distance; the attack's angle against the defender's facing (shield side, weapon
   side, behind); lateral clearance; footing; pressure from behind.
2. **Opportunity.** Weapon tempo and recovery time; stamina (W′); aggression (x and Φ in
   [04](04-psyche.md)); stance; split attention. Two on one is deadly, which is why a ruptured
   line rolls up from its edges.
3. **Defence.** The shield's geometric coverage of the attack line; an active block, depending on
   skill, fatigue and attention; evasion only where there is room to move.
4. **Armour.** The zone struck; the kind of blow (cut, thrust, blunt); its energy against that
   zone's protection.
5. **Outcome.** Deflected, minor, serious, disabling or lethal.

**A "critical hit" emerges from the situation:** a thrust into a gap (face, neck, armpit,
groin), a dropped shield, a man off balance or down, a back turned. Nothing is rolled as a crit.

## 3. Reach, space and the distance contest

Illustrative values, to be sourced in the knowledge base:

| Weapon | Effective reach | Space it needs | Notes |
|---|---|---|---|
| *Gladius hispaniensis* | 0.5–1.0 m | Little | Thrust first; works in close order |
| Celtic long sword | 0.9–1.5 m | Room at the side and overhead | A cutting sword; crowding neutralises it |
| *Falcata* | 0.6–1.2 m | Moderate | Cut and thrust |
| Spear (*hasta*, about 2.5 m) | 1.2–2.2 m, also from the second rank | Little at the side | Weak inside about 0.8 m |
| *Sarissa* | 2.5–6 m; five ranks of points | Cannot turn | Helpless inside about 2 m |
| *Rhomphaia*, *falx* | 1–1.8 m | A great deal; no shield | Two-handed |

- **The distance contest.** Each man steers toward the distance his weapon prefers, and crowd
  physics settles who gets it. Gladius men press in; long-sword men want room; spearmen hold the
  gap; a phalanx wins outside its points and loses inside them.
- **Wear and states.** Shields split and get pinned. Spears and sarissae snap. Gallic swords bend
  after the first cut (Polybius 2.33), a claim to calibrate, not adopt. *Off balance* (from a boss
  punch, a slip, a pinned shield) and *down* are states that open a man to the next blow.
- **Matchups emerge** from these properties: gladius against long sword, in the open and in a
  press; mail against cuts; the phalanx inside and outside its points. They are checked against
  the matchup research, not written in.

## 4. Wounds and the body

- **A wound list per man:** zone, type (cut, thrust, blunt, missile), severity.
- **One blood pool,** about 5 litres, read through the trauma-medicine classes of blood loss:
  under 15 %, barely noticed; 15–30 %, weaker and anxious; 30–40 %, confused, falls out of the
  fight; over 40 %, collapse and death. A cut thigh or neck artery empties it in minutes. A belly
  wound disables at once.
- **Loss of function:** the weapon arm; the shield arm, so the shield sags, which is the real
  killer; a leg, so he can neither charge nor flee; a blow to the helmet (concussion); an eye.
- **Disabled men stay in the simulation.** They lie, crawl, are carried off (which thins the
  line), are trampled, killed or captured. Bodies change the ground, as at Zama.
- **Deaths after the battle.** The share of severe wounds that kill within days feeds A1's
  casualty mapping ([07](07-validation.md) §6.4).

## 5. Wounds and the mind

- **Pain makes fear jump** (F). Many wounded men leave the line with a helper, thinning it twice.
- **Some temperaments turn pain into fury** (Φ). At Dyrrachium the centurion Scaeva's shield was
  found with 120 holes in it; four centurions of one cohort lost their eyes (Caesar, *BC* 3.53).
- **Gore seen by others frightens them.** Macedonians used to wounds from spears and arrows were
  terrified by bodies the Spanish sword had cut apart: arms severed at the shoulder, heads cut
  off, entrails exposed (Livy 31.34). Each wound carries a horror value, damped by habituation
  ([04](04-psyche.md) §4).
- **Breakdown without a wound.** At Marathon, Epizelus went blind in the fight without being
  struck (Herodotus 6.117). The psyche can freeze, collapse or flee without a scratch.
- **Across battles,** wounds and trauma carry over through a scenario's initial conditions.

## 6. Cost

About 16 bytes of wound state and 8 bytes of combat state per man. Fewer than one man in ten is
in contact at any moment, even in the encirclement, where the pocket's perimeter is short
compared with its mass. So the contact zone can be the most detailed part of the model.

## 7. Calibration

- Duel lengths from historical European martial arts and reenactment, as rough bounds.
- Low losses while lines fight, until one breaks (S1 in [07](07-validation.md)).
- Wound distributions: the mass graves at Visby (1361) show many leg wounds below the shield
  line, a medieval analogue; Celsus, *De Medicina* book 7, on the surgery of wounds and missiles;
  Salazar, *The Treatment of War Wounds in Graeco-Roman Antiquity* (2000).

## 8. Staging

Following review 1: Slice 1 starts with body zones, wound severity and one blood pool.
Per-wound bleeding, helmet-specific hearing, shield drift and the other refinements enter only
when a named pattern needs them ([08](08-roadmap.md) §2).
