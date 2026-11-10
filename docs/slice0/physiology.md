<!-- Provenance: written by a research subagent of the Paullus session on 2026-11-10, read in
full by Paullus and kept verbatim. Feeds the stamina, power and gait lines of
docs/slice0/priors-shared.md. -->

# Physiology sources for the CP / W' stamina model

Checked 2026-11-10. Tags:

- **[primary]**: I read it in the source itself (full text, or the abstract in the official PubMed/PMC record when the number is in the abstract).
- **[secondary]**: I read it only in another paper, a review, Wikipedia, or a search-engine summary of the source.
- **[unconfirmed]**: I could not verify it. Do not use it without checking.
- **[derived]**: my own arithmetic from tagged inputs. No source states the result.

Paywalls blocked several primary texts: Skiba 2012 full text, Askew 2012 full text (PMC captcha, Cloudflare on royalsocietypublishing.org), and the Pandolf 1977 full text.

---

## 1. CP relative to VO2max; W' and W'/CP for non-elite men

### CP as a fraction of VO2max
- **CP ~ 70-80% VO2max in healthy young subjects; ~80-90% VO2max in well-trained individuals.** The same review puts LT/GET at 50-65% (healthy young) and 70-80% (well-trained). **[primary]**
  - Source: Poole DC, Burnley M, Vanhatalo A, Rossiter HB, Jones AM (2016). "Critical Power: An Important Fatigue Threshold in Exercise Physiology." *Med Sci Sports Exerc* 48(11):2320-2334. Read in the PMC author manuscript PMC5070974; the review attributes the ranges to its ref. 96.
- The same review defines the running convention: CP becomes **critical speed (CS)** when intensity is measured in speed, and W' becomes **D'** in metres. **[primary]**

### W' and CP: cycling (mechanical watts at the crank)
- **Black et al. (2015):** CP = 250 ± 47 W and W' = 20.6 ± 7.4 kJ from constant-power tests. With time trials, CP = 265 ± 44 W and W' = 18.1 ± 5.7 kJ. **[secondary]** (read in Jones & Vanhatalo 2017, below)
  - Original: Black MI, Jones AM, Bailey SJ et al. (2015). *Appl Physiol Nutr Metab* 40:662-670.
- **Chidnok et al. (2013):** W' (from the 3-min all-out test) = 16.5 ± 4.0 kJ. **[secondary]** (read in Jones & Vanhatalo 2017)
  - Original: *Med Sci Sports Exerc* 45:1377-1385.
- **Ferguson et al. (2010):** control W' ~22 kJ. **[secondary]** (read in Jones & Vanhatalo 2017)
- **Vanhatalo, Doust and Burnley (2007), "habitually active" subjects:** W' = 16.0 ± 3.8 kJ; WEP from the 3-min test = 15.0 ± 4.7 kJ. **[secondary]** (search summary of the abstract)
  - Original: *Med Sci Sports Exerc* 39(3):548-555.
  - A 2008 follow-up by the same group: end-test power 225 ± 52 W, rising to 248 ± 46 W after training. **[secondary]**
- Review read: Jones AM, Vanhatalo A (2017). "The 'Critical Power' Concept: Applications to Sports Performance with a Focus on Intermittent High-Intensity Exercise." *Sports Med* 47(Suppl 1):65-78. PMC5371646, open access. Read in full.
- **W'/CP (cycling) ~ 68-82 s** **[derived]**. From Black et al.: 18.1 kJ / 265 W = 68 s, and 20.6 kJ / 250 W = 82 s.

### W' and CP: running, team-sport men (closest to "fit, non-elite")
- **Vassallo et al. (2020), Table 1:** n = 9 healthy males (football 7, rugby 1, hockey 1), age 23 ± 4 y, body mass 77.8 ± 5.5 kg. **[primary]**
  - **W' = 31.4 ± 5.0 kJ**
  - **CP = 451.2 ± 24.8 W**
  - **VO2max = 51.1 ± 5.3 ml/kg/min**
  - **vVO2max = 14.6 ± 1.5 km/h (4.06 m/s)**
  - Source: Vassallo C, Gray A, Cummins C, Murphy A, Waldron M (2020). "Exercise tolerance during flat over-ground intermittent running: modelling the expenditure and reconstitution kinetics of work done above critical power." *Eur J Appl Physiol* 120(1):219-230. PMC6969867, read in full.
  - **Caveat:** these "watts" are *modelled mechanical* over-ground power. They come from GPS speed through a model of centre-of-mass, air and limb-swing work. They are not crank power and not metabolic power. W' and CP are therefore not comparable in absolute terms with cycling.
- **W'/CP (running) = 31.4 kJ / 451.2 W = 70 s** **[derived]**.
- **Conclusion across modes:** W'/CP is about 65-85 s in non-elite men **[derived]**. The ratio does not change when both W' and CP are converted to metabolic power with the same efficiency.

### D' for runners
- **[unconfirmed]** No source read gives typical D' values in metres for recreational men.

### Gross efficiency, to convert cycling CP and W' to metabolic power
- **Hopker J, Jobson S, Carter H, Passfield L (2010).** "Cycling Efficiency in Trained Male and Female Competitive Cyclists." *J Sports Sci Med* 9:332-337. Read on the JSSM abstract page. **[primary]**
  - Male GE = **19.9 ± 1.8% at 150 W** and **20.4 ± 1.5% at 180 W** (n = 13 males).
  - Female GE = 22.5% and 22.3%.
  - The males: VO2max 61.3 ± 5.4 ml/kg/min, 74.1 kg, maximal aerobic power 414 W.
- **Lab vs field:** trained males, GE 20.4% at 150 W in the lab, 18.4% in the field, and 23.2% at 60% of maximal minute power. **[secondary]** (search summary of a Canterbury Christ Church repository record; authors not seen)
- The often-quoted "18-23% typical range" is **[unconfirmed]**. I could not reach Ettema & Loras (2009).
- **Working value: GE ~ 20% for men** **[derived from the primary value above]**. Then metabolic power ~ 5 x mechanical power.

---

## 2. Metabolic cost of walking and running with load; VO2max; W per L O2

### Pandolf equation
- **Source:** Pandolf KB, Givoni B, Goldman RF (1977). "Predicting energy expenditure with loads while standing or walking very slowly." *J Appl Physiol* 43(4):577-581. PMID 908672. I confirmed the title and citation in PubMed **[primary]**, but no abstract or full text was available.
- **Equation** **[secondary]** (Wikipedia, "Pandolf equation", citing the 1977 paper; it matches several other secondary sources):
  - M = 1.5 W + 2.0 (W + L)(L/W)^2 + eta (W + L)(1.5 V^2 + 0.35 V G)
  - M = metabolic rate in **watts (gross)**
  - W = body mass in kg; L = load in kg
  - V = speed in m/s; G = grade in %
  - eta = terrain factor
- **Terrain factor eta** (Soule & Goldman 1972, via Wikipedia) **[secondary]**:

  | Surface | eta |
  |---|---|
  | Blacktop or treadmill | 1.0 |
  | Dirt road | 1.1 |
  | Light brush | 1.2 |
  | Heavy brush | 1.5 |
  | Swampy bog | 1.8 |
  | Loose sand | 2.1 |

- **Validity:** the equation under-predicts at heavy loads and high speeds. An Australian Army validation found **12-33% under-prediction** with contemporary military loads. **[secondary]**
- **Computed for a 70 kg man, level ground, eta = 1** **[derived]**:

  | Activity | Load | Metabolic power |
  |---|---|---|
  | Standing | none | 105 W |
  | Standing | 25 kg | 129 W |
  | Walking 1.3 m/s | none | 283 W |
  | Walking 1.3 m/s | 20 kg | **348 W** |
  | Walking 1.3 m/s | 25 kg | **370 W** |
  | Walking 1.3 m/s | 30 kg | **395 W** |

  Allowing for the reported under-prediction, the loaded walks could be about 390-525 W.

### Running at about 4 m/s
- **Net cost of level running ~ 1 kcal/kg/km, roughly independent of speed** (Margaria et al. 1963, *J Appl Physiol* 18:367-370). **[secondary]** (search summaries; one treadmill study gave 0.97 kcal/kg/km)
  - That is ~4.2 J/kg/m **[derived]**.
  - At 4 m/s for 70 kg: **~1170 W net, ~1250-1270 W gross** **[derived]**.
  - In oxygen: ~3.7 L/min, ~53 ml/kg/min **[derived]**.
- **Cross-check:** in Vassallo 2020, fit team-sport men reach VO2max at 4.06 m/s **[primary]**. So 4 m/s is roughly vVO2max for a fit man, well above CP.

### VO2max of fit young men
- Semi-professional team-sport men: **51.1 ± 5.3 ml/kg/min** (Vassallo 2020) **[primary]**.
- Male judo athletes: **50-55 ml/kg/min** (Franchini E et al. 2011, "Physiological profiles of elite judo athletes," *Sports Med* 41(2):147-166, abstract) **[primary]**.
- Trained male cyclists: **61.3 ± 5.4** (Hopker 2010) **[primary]**.
- Amateur boxers: VO2peak ~62 ml/kg/min **[derived]**, from Arseneau 2011: 43.4 ml/kg/min equals 69.7% of VO2peak.
- Untrained young men: **[unconfirmed]** (the usual figure is ~40-45, but I read no source).

### Watts per L O2 per minute
- **Weir (1949) equation:** EE (kcal) = 3.941 VO2 + 1.106 VCO2, in litres. Equivalently, kcal per L O2 = 3.941 + 1.106 RER. **[secondary]** (PLOS ONE 2019 validation paper and Wikipedia, via search)
  - Original: Weir JB de V (1949), *J Physiol* 109:1-9.
- Energy per litre and per L/min **[derived]**:

  | RER | kJ per L O2 | W per L O2/min |
  |---|---|---|
  | 0.70 | 19.7 | **329** |
  | 0.85 | 20.4 | **340** |
  | 1.00 | 21.1 | **352** |

  Use **~340-350 W per L O2/min** for hard exercise.
- **Resting rate** of 3.5 ml/kg/min (1 MET), about 80 W for 70 kg: **[unconfirmed]**. This is the standard convention, but I read no source this session.

---

## 3. Armour, and the intensity of fighting

### Askew, Formenti and Minetti (2012)
- **Citation:** Askew GN, Formenti F, Minetti AE (2012). "Limitations imposed by wearing armour on Medieval soldiers' locomotor performance." *Proc R Soc B* 279(1729):640-644. doi 10.1098/rspb.2011.0816. Published online 20 Jul 2011.
- From the abstract, read in the PMC record PMC3248716 **[primary]**:
  - **Net cost of locomotion (Cmet) in armour was 2.1-2.3x that of unloaded walking, and 1.9x for running, at the same speed.**
  - Armour cost was "much higher than equivalent trunk loading."
  - The authors attribute the extra cost mostly to limb swing and impaired breathing.
  - "A typical suit of field armour weighed between 30 and 50 kg."
- From the full text, read only in reproductions and summaries **[secondary]** (brewminate.com reproduction plus a search summary):
  - n = 4 experienced male fight-interpreters (Royal Armouries), mean age 36, body mass 79 kg.
  - Armour 35 ± 5 kg, 44 ± 3% of body mass. The body-mass increase was 1.4-fold, smaller than the cost increase.
  - The paper cites earlier work in which back-loading raised cost **~70% (walking) and ~50% (running)**. This is the backpack comparison.
  - Lower-limb swing power rose ~63%.
  - Ventilation and breathing frequency rose, but tidal volume did not; the authors read this as restricted inspiration.
  - At 32 ml O2/kg/min, a 38-year-old can sustain 2.7 m/s unloaded but only 1.7 m/s walking in armour.
- **Exact ratio of armour cost to equivalent back-loaded cost:** **[unconfirmed]**. I did not see the number.
- **Relevance:** this is full-body plate. A legionary's mail, helmet and shield plus a pack sits between "trunk load" (use Pandolf) and "limb-distributed armour" (use Askew).

### Measured intensity of fighting

**Boxing sparring** **[primary]**, abstract in PubMed. Arseneau E, Mekary S, Leger LA (2011). "VO2 requirements of boxing exercises." *J Strength Cond Res* 25(2):348-359.
- Subjects: 9 male amateur boxers, 71.4 kg.
- VO2 for sparring was estimated from post-exercise measurements.
- Sparring: **43.4 ± 5.9 ml/kg/min = 69.7 ± 8.0% VO2peak**, at **85.5% HRmax**.
- Pad work: 41.1 ml/kg/min, 66.1% VO2peak.
- Gym sparring: **91.7% HRmax, blood lactate 9.4 ± 2.2 mmol/L**. Lab sparring: 6.1 mmol/L.

**Armoured medieval combat (buhurt-style "pro-fight"), case study, n = 1** **[primary]**. Mohammadalizadeh M et al. (2024). "Kinematic and Physiological Analysis of Medieval Combat Sport Using Motion Analysis, Blood Lactate Measurement, and Heart Rate Monitoring: A Case Study." *Sensors* 24(11):3443. PMC11174425, read in full.
- The fighter: 31 y, 89.5 kg, armour >20 kg.
- Protocol: 3 x 90 s rounds of maximal-effort strike drills, 45 s rest between rounds.
- Blood lactate at the end of rounds 1, 2 and 3: **4.7, 6.9, 8.2 mmol/L**. Lactate threshold on the ergometer: 2.9 mmol/L at 150 W.
- Heart rate peaks: **180, 188, 193 bpm**. Age-predicted maximum: 189.
- Strike velocities declined across rounds.
- This is a drill against a partner, not free sparring.

**Judo match** **[secondary]** (search summary of a Spanish-language study, 8 male Galician selection fighters): **92.65 ± 3.26% HRmax, lactate 13.75 ± 3.09 mmol/L**. Carballeira and Iglesias.

**Not found** **[unconfirmed]**: direct VO2 in HEMA or longsword sparring; MMA lactate; kendo; fencing.

### Interpretation
- Measured *aerobic* uptake in sparring is only ~70% VO2peak, which is about 0.9-1.0 CP if CP is 75% VO2max **[derived]**.
- Lactate of 6-14 mmol/L and heart rate near maximum show a large anaerobic share. The total metabolic demand is therefore above CP: a severe-domain, intermittent effort.
- **A figure of "1.5-2 CP" for fighting is [unconfirmed]**. No study I found measures total (aerobic plus anaerobic) metabolic power during weapon combat. The defensible statement is: "fighting bouts sit above CP and drain W' over roughly 1-5 minutes." Calibrate against the case study: three 90 s rounds took the fighter to near-maximal heart rate and 8 mmol/L lactate.

---

## 4. Recovery: the Skiba et al. (2012) time constant

- **Citation:** Skiba PF, Chidnok W, Vanhatalo A, Jones AM (2012). "Modeling the expenditure and reconstitution of work capacity above critical power." *Med Sci Sports Exerc* 44(8):1526-1532. PMID 22382171.
- From the abstract **[primary]**:
  - 7 subjects, cycle ergometer.
  - Protocol: 60 s severe work, then 30 s recovery at varied power.
  - The recharge time constant was negatively correlated with (CP - recovery power).
  - "best fit by an exponential (r = 0.77)."
  - The abstract does **not** print the formula.
- **Formula** **[secondary]**: tau_W' = 546 e^(-0.01 D_CP) + 316.
  - Read in Vassallo et al. 2020 (Eq. 5), which states "tau_W' is the time constant (s)".
  - Read in Jones & Vanhatalo 2017 (Eq. 3; Jones and Vanhatalo co-authored Skiba 2012).
  - Both define D_CP = CP - recovery power P_R.
- **Units:**
  - tau is in **seconds** **[secondary]**.
  - D_CP is in **watts of external mechanical power** (cycle-ergometer power). Vassallo states P and CP in W **[secondary]**.
  - Consistency check **[derived]**: Jones & Vanhatalo say tau ~377 s at 20 W recovery. The formula gives 377 s when D_CP = 220 W, which implies CP ~240 W. That is consistent with D_CP in watts for subjects with CP ~240 W.
- **Values reported for Skiba 2012** **[secondary]** (Jones & Vanhatalo 2017):

  | Recovery | tau |
  |---|---|
  | 20 W | ~377 s |
  | Moderate domain | ~452 s |
  | Heavy domain | ~580 s |
  | Above CP | ~7056 s (no net recharge) |

- **Other implementation details** **[secondary]**:
  - The 2012 model uses a constant recovery power: the mean of all sub-CP powers (trackeR R package documentation).
  - Skiba 2015 replaced it with a differential form: W'bal = W'0 - W'exp e^(-D_CP t / W'0).
  - Bartram et al. (2018) refit it for elite cyclists: tau = 2287.2 D_CP^-0.688.
- **Caution for the sim:** the constants 546, 0.01 and 316 carry units: s, 1/W, s. They are fitted to *mechanical* watts on cycle ergometers with CP ~240 W. If the sim expresses power in metabolic watts or as a fraction of CP, rescale D_CP accordingly. For example, D_CP in mechanical W ~ (D_CP / CP) x 240 for a CP-240 W subject.
- **Independent recovery data:** Ferguson C, Rossiter HB, Whipp BJ et al. (2010). *J Appl Physiol* 108:866-874. **[secondary]** (Jones & Vanhatalo 2017)
  - W' after exhaustion recovered from **~22 kJ** to **8, 14, 19 kJ** after 2, 6, 15 min. That is **36%, 64%, 86%** **[derived]**.
  - W' recovery half-time **~234 s**. Blood lactate clearance half-time ~1366 s. VO2 (PCr proxy) half-time ~74 s.

---

## 5. Pushing in a press

**Martin E, Beckham G (2020).** "Force production during the sustained phase of Rugby scrums: a systematic literature review." *BMC Sports Sci Med Rehabil* 12:33. PMC7249338, read in full. All points below are **[primary]** for the review.
- 26 studies reviewed.
- **Individual sustained force against a machine: group means 1000-2000 N** (male players beyond high-school age, mostly forwards).
- **Full pack: 4000-8000 N.**
- **A pack transmits on average 65% (range 52-74%) of the sum of its individual forces** (Quarrie & Wilson 2000).
- A **~10% force advantage** seems necessary for one pack to drive another back.
- Pushes in tests lasted **5-10 s**.
- **Fatigue:** with 5 x 5 s maximal isometric individual pushes and 20 s rest, force fell **23%** from rep 1 to rep 5 (Morel et al.).
- **Posture:** force is optimal with shoulders at ~40% of stature and knee and hip angles of ~120 deg.
- Force scales strongly with body mass.

**Other figures** **[secondary]** (search summaries, originals not checked):
- South African study: whole-scrum initial shove 8.6 kN, **1062 N per player**.
- Sydney front-row study: impact up to 16.5 kN, sustained ~8.0 kN. It is unclear whether this is per individual.

**Metabolic cost of hard pushing:** **[unconfirmed]**. I found no study measuring VO2 or metabolic power during sled pushing or scrummaging.
- Waller et al. (2016), push-sled sprints: lactate only, no VO2 **[secondary]**.
- A backward sled-drag session: post-exercise lactate 12.4 ± 2.6 mmol/L. That is pulling, not pushing **[secondary]**.

**Note** **[derived, not from a source]**:
- A moving push does mechanical work F x v. For example, 1000 N at 0.5 m/s = 500 W mechanical, about 2.5 kW metabolic at 20% efficiency: far above CP.
- A static press does no external work. Its cost is isometric, and I found no data on it.
- The 23% decline over 5 x 5 s and the 5-10 s test durations suggest near-maximal pushing is a W'-limited, seconds-to-tens-of-seconds effort.

---

## Worked conversion to multiples of CP

All of this is **[derived]** and illustrative. Assumptions:
- 70 kg man, VO2max 50 ml/kg/min = 3.5 L/min, about 1200 W metabolic at ~345 W per L/min.
- CP at 75% VO2max, about 900 W metabolic (gross).
- W'/CP about 70 s.

| Activity | Metabolic W | x CP | Basis |
|---|---|---|---|
| Rest (1 MET, unconfirmed) | ~80 | ~0.09 | convention |
| Stand, unloaded | 105 | 0.12 | Pandolf |
| Stand, 25 kg | 129 | 0.14 | Pandolf |
| Walk 1.3 m/s, 25 kg | 370 (to ~490 if under-predicted) | 0.41 (0.55) | Pandolf, plus 12-33% |
| Walk 1.3 m/s in full plate | net ~2.1-2.3x unloaded | ~0.6-0.7 | Askew (plate, not legionary kit) |
| Run 4 m/s, unloaded | ~1250 | ~1.4 | Margaria; ~vVO2max (Vassallo) |
| Sparring (aerobic part only) | ~70% VO2peak | ~0.9-1.0 aerobic, plus a large anaerobic share | Arseneau; lactate 6-9 mmol/L |
| Fighting, total | ? | >1 (1.5-2 unconfirmed) | no direct measurement |
| Pushing in a press | ? | unconfirmed | forces only (1-2 kN per man) |

---

## Sources read

- [Poole et al. 2016, PMC5070974](https://pmc.ncbi.nlm.nih.gov/articles/PMC5070974/), full text via NCBI efetch
- [Jones & Vanhatalo 2017, PMC5371646](https://pmc.ncbi.nlm.nih.gov/articles/PMC5371646/), full text
- [Vassallo et al. 2020, PMC6969867](https://pmc.ncbi.nlm.nih.gov/articles/PMC6969867/), full text
- [Skiba et al. 2012, PubMed 22382171](https://pubmed.ncbi.nlm.nih.gov/22382171/), abstract
- [Askew et al. 2012, PMC3248716](https://pmc.ncbi.nlm.nih.gov/articles/PMC3248716/), abstract only
- [brewminate.com reproduction of Askew et al. 2012](https://brewminate.com/limitations-imposed-by-wearing-armour-on-medieval-soldiers-locomotor-performance), secondary
- [Pandolf equation, Wikipedia](https://en.wikipedia.org/wiki/Pandolf_equation), secondary
- [Pandolf et al. 1977, PubMed 908672](https://pubmed.ncbi.nlm.nih.gov/908672/), citation only
- [Arseneau et al. 2011, PubMed 21217532](https://pubmed.ncbi.nlm.nih.gov/21217532/), abstract
- [Franchini et al. 2011, PubMed 21244106](https://pubmed.ncbi.nlm.nih.gov/21244106/), abstract
- [Mohammadalizadeh et al. 2024, PMC11174425](https://pmc.ncbi.nlm.nih.gov/articles/PMC11174425/), full text
- [Martin & Beckham 2020, PMC7249338](https://pmc.ncbi.nlm.nih.gov/articles/PMC7249338/), full text
- [Hopker et al. 2010, JSSM](https://www.jssm.org/jssm-09-332.xml-abst), abstract page
- Search-engine summaries only, all secondary: Weir equation; Margaria running cost; the Vanhatalo 2007 abstract; the Carballeira judo study; the Canterbury gross-efficiency study; sled studies.
