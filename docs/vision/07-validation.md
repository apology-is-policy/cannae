# 07 — Validation

## 1. Philosophy

- **Pattern-oriented modelling** (Grimm et al.): validate against many patterns at several
  scales at once. That means individual patterns (fatigue, the length of a duel), unit patterns
  (how long a maniple holds against a warband) and battle patterns (outcome, casualty ratio,
  duration, the *sequence* of events). A model that hits one number by tuning is worthless. A
  model that reproduces many independent patterns with one parameter set is credible.
- **Calibration and holdout sets.** We tune parameters on calibration battles and micro-tests
  only. **Cannae sits in the holdout set**, so the acceptance test stays honest. If we ever tune
  on Cannae, we will say so and move another battle into the holdout set.
- **Distributions, not single runs.** Every check is a Monte Carlo band over N seeds (N = 100–500
  for battles; thousands for micro-tests).
- **Source variants.** Scenarios come in variants per source: Polybius' numbers, Livy's, and a
  Delbrück-style sceptical reduction. Uncertain terrain gets alternatives too (Cannae's site and
  the course of the Aufidus are debated). A robust result holds across variants. When it does
  not, that is a finding worth reporting.
- **Hypothesis modes.** Contested mechanics are switchable: line relief R1–R4, othismos on or off,
  frontage per man, maniple intervals. Running the corpus under each mode scores how well it fits.
  The simulator becomes evidence in historical debates.

## 2. Stylized facts (must hold across the corpus)

| ID | Fact | Measurable target (initial) |
|---|---|---|
| S1 | **Casualty asymmetry.** The losing side suffers most of the casualties, mostly after it breaks. | Winners ≤ about 5–15 %; losers about 20–60 %+ in decisive battles. Most of the loser's dead fall after the first major rout. |
| S2 | **Duration and pulsing.** Large battles last hours. Melee comes in pulses of minutes, separated by lulls. | Clash and lull structure visible in the contact-intensity time series |
| S3 | Flank and rear attacks decide out of all proportion to the kills they cause | The rout probability from a flank charge is much higher than from a frontal charge of equal force |
| S4 | Fresh reserves committed at the right moment are decisive | Varying the timing of reserve commitment shifts the outcome distribution |
| S5 | Light troops rarely decide alone, except against troops that cannot respond, or in broken terrain | Carrhae passes; generic skirmish-only fights stay indecisive |
| S6 | Cavalry wins wings and does the pursuing. It rarely breaks steady, formed heavy infantry head-on. | Horses refuse; a frontal cavalry charge into steady infantry fails |
| S7 | The phalanx is invincible frontally on flat ground and collapses when disordered or flanked | Micro-test M3 |
| S8 | Elephants cut both ways | Micro-test M6, Zama, Magnesia, Panormus |
| S9 | Veterans beat recruits, even when outnumbered | Pharsalus (22,000 against 45,000) and the micro-tests |
| S10 | Encircled armies are compressed, lose the ability to fight, and are slaughtered or surrender | Cannae, Teutoburg |
| S11 | The death or flight of a commander is frequently decisive | Pydna, Metaurus, Philippi |
| S12 | Disciplined units can retire in order or break out even in defeat | The Trebia breakout |

## 3. Micro-scenario suite (fast, run daily)

| ID | Scenario | Expected band (to refine from sources and experimental archaeology) |
|---|---|---|
| M1 | Duel matrix: legionary, Gaul, Iberian, phalangite, Celtiberian | Sanity-checks the weapon and armour numbers. No duel is a foregone conclusion. |
| M2 | Century or maniple against an equal Gallic warband on open ground | Romans win most runs by endurance and line relief. The Gauls win a meaningful share when their first rush breaks the line. |
| M3 | Phalanx syntagma(s) against maniples: flat ground, then rough ground | Flat: the phalanx wins frontally. Rough ground or a flank opportunity: the maniples win. |
| M4 | Cavalry against infantry: steady and frontal, disordered, flanked | Steady infantry holds. Disordered or flanked infantry routs. |
| M5 | Horse archers against an infantry square: with and without arrow resupply, with and without slingers, high ground | Resupply means slow destruction. Slingers or high ground turn it around. |
| M6 | Elephants against green and habituated infantry, with and without lanes and trumpets | Green troops panic. Lanes and skirmishers neutralise the elephants. Some charges rebound into their own side. |
| M7 | Flank charge on a recruit cohort and on a veteran cohort | Veterans often turn to face. Recruits often rout. |
| M8 | Triplex against simplex acies (equal numbers) facing a persistent enemy | The triplex outlasts, given fatigue and morale. If not, the model is wrong. |
| M9 | Pursuit: disciplined against undisciplined pursuers | Kill rates in pursuit. How often pursuers rally. How exposed they are to a counter-charge. |
| M10 | The Pharsalus halt | Veterans self-pace the charge. Recruits do not. |
| M11 | The Sabis: surprise attack on men building camp | Veterans self-organize into a line. Recruits need officers. |
| M12 | Gergovia: an eager assault, then a recall signal in the din | Some units do not hear or do not obey |
| M13 | Feigned flight: trained against untrained cavalry | Trained cavalry turns back. Untrained cavalry routs for real. Pursuers get disordered. |
| M14 | Compression: an encircled block | The share of men able to fight collapses. Casualties accelerate. |
| M15 | Lulls emerge | Contact intensity pulses without any scripted lull |
| M16 | Crumble or snap: the same unit with narrow and wide courage spreads | Snap versus progressive crumbling, as the psyche model predicts |

## 4. Battle corpus

C marks the calibration set; H marks the holdout set. The split is provisional.

| Battle | Year | Set | What it tests |
|---|---|---|---|
| Bagradas (Tunis) | 255 BC | C | Elephants and cavalry against a legion on flat ground (Xanthippus) |
| Panormus | 250 BC | H | Elephants panicked by missiles from a ditch, stampeding into their own side |
| Trebia | 218 BC | C | Cold, hunger and wet. Mago's ambush. Cavalry superiority. 10,000 Romans breaking out through the center. |
| Lake Trasimene | 217 BC | H | Ambush of a column in fog. No deployment. Death of the commander. |
| **Cannae** | 216 BC | **H** | **Acceptance test A1** |
| Dertosa (Ibera) | 215 BC | H | **Counter-test.** A Cannae-style deployment fails because the Iberian center breaks immediately. The same plan with unreliable troops must fail. |
| Metaurus | 207 BC | H | Nero's flank march behind his own line on his own initiative. Hasdrubal's death. |
| Ilipa | 206 BC | C | Reverse Cannae. Deception in deployment. An enemy fighting unfed. |
| Great Plains | 203 BC | H | The wings collapse while the Celtiberians fight to the death in the center |
| Zama | 202 BC | C | Elephants against lanes. Three lines on each side. Cavalry returning on the rear. |
| Cynoscephalae | 197 BC | C | Fog, a half-formed phalanx, a tribune's attack on the rear |
| Magnesia | 190 BC | H | Chariots stampeded. Elephants panicking in the phalanx intervals. Antiochus over-pursues. |
| Pydna | 168 BC | C | Phalanx disorder over rough ground, maniples exploiting the gaps |
| Arausio | 105 BC | H | Divided command, uncoordinated attacks. A catastrophe. |
| Aquae Sextiae | 102 BC | C | Uphill defence, Marcellus' ambush in the rear |
| Vercellae | 101 BC | H | Heat, dust, sun. Habituated Roman troops. |
| Chaeronea / Orchomenus | 86–85 BC | C | Field works against chariots and phalanx. Sulla rallying his men in person. |
| Tigranocerta | 69 BC | H | A small army breaks a huge host by hitting its cataphracts in flank and rear from a hill |
| Carrhae | 53 BC | C | Horse archers with resupply, cataphracts, helplessness, Publius' sortie |
| Bibracte | 58 BC | C | Pila pinning shields, an uphill fight, the Boii and Tulingi flank attack, the third line facing about |
| Vosges (Ariovistus) | 58 BC | H | Young Crassus commits the third line on his own initiative |
| Sabis | 57 BC | C | Surprise during camp building. Self-organizing veterans. Caesar's personal intervention. |
| Gergovia | 52 BC | H | A recall unheard. Over-eager assault. Roman defeat. |
| Bagradas (Curio) | 49 BC | H | Feigned retreat and cavalry envelopment by Juba's general Saburra |
| Dyrrachium | 48 BC | H | Panic at the fortification line. Pompey fails to pursue. |
| **Pharsalus** | 48 BC | C | Experience against numbers. The hidden fourth line against cavalry. The self-paced charge. |
| Zela | 47 BC | H | An enemy attacking uphill into men building camp. Scythed chariots. |
| Ruspina | 46 BC | C | Encirclement by light troops and horse. Orbis. Alternating facings to break out. |
| Thapsus | 46 BC | H | Elephants panicked by missiles. Soldiers forcing the attack signal. Massacre. |
| Munda | 45 BC | H | A redeployment misread as flight starts a collapse. A desperate fight. |
| Forum Gallorum | 43 BC | H | Veterans against veterans fighting in silence, recruits held back |
| Philippi I and II | 42 BC | H | Cassius' fatal misreading of the battle. Two wings with opposite results. |
| Teutoburg | AD 9 | C | A column ambushed in forest and rain over several days (the field part only) |

Stretch scenarios: Pyrrhus at Heraclea, Asculum and Beneventum (280–275 BC); Raphia (217 BC);
Idistaviso (AD 16).

## 5. Acceptance test A1: Cannae, in measurable terms

**Outcome classes**, computed automatically from the event log:

- (a) Carthaginian decisive victory with encirclement (historical);
- (b) Carthaginian victory without a closed encirclement;
- (c) Roman breakthrough in the center (a Roman victory or a draw);
- (d) other.

**Initial targets**, to be agreed:

- P(a) ≥ 0.6, and P(a) ≤ 0.95. Hannibal's plan must remain a gamble.
- P(c) ≥ 0.03, with the cause identifiable: a Gallic center rupturing early.
- In class (a):
  - Roman killed plus captured within the source envelope (about 50,000–80,000).
  - Carthaginian losses about 4,000–10,000.
  - The Gauls carry the largest share of Carthaginian losses.

**Sequence markers in class (a):**

- The Roman cavalry on the right breaks before the infantry fight is decided.
- The allied cavalry collapses after Hasdrubal arrives in its rear.
- The Libyans turn inward after the Roman mass has pushed deep into the crescent.
- The rear is closed by cavalry.
- The share of encircled Romans able to fight collapses (compression).

## 6. Acceptance test A2: indeterminacy

1. **A balanced-scenario set.** Historical near-even fights (Munda, Philippi I, Forum Gallorum)
   plus synthetic matchups of similar quality.
2. **The naive predictor.** Numbers × troop-quality weights, Lanchester-style. On the balanced
   set its accuracy should be clearly below certainty. On lopsided scenarios it should be right,
   because indeterminacy must not mean noise.
3. **Outcome entropy.** Balanced scenarios show non-degenerate distributions.
4. **Mechanism attribution.** For every seed, an analysis of the event log names the decisive
   event: the first system-wide rout cascade, a commander's death, the timing of a reserve, a
   misread signal. Outcomes must be explainable from mechanisms in most seeds.
5. **Blind trials.** You look at deployments, predict outcomes with probabilities, and we track
   your Brier score. If you can predict perfectly, A2 fails.

## 7. Counterfactual sanity checks

| Counterfactual | Expected direction |
|---|---|
| Cannae with Libyans in the center instead of Gauls | The center holds better, but there is less force for the envelopment |
| Cannae with Caesar's veterans instead of the 216 BC legions | Higher odds of a breakout or a turn-to-face |
| Zama without lanes | The elephants do much more damage |
| Carrhae with 4,000 slingers on the Roman side | The Parthian advantage drops sharply |
| Pydna on flat ground | The phalanx is much stronger |
| Pharsalus without the fourth line | Pompey's cavalry envelopment succeeds far more often |
