<!-- Provenance: written by a research subagent of the Paullus session on 2026-11-10, read in
full by Paullus and kept verbatim. Feeds docs/slice0/priors-pairwise.md and the passage study
(S0.8). -->

# Crowd physics priors: sources checked

Checked 2026-11-10. About 15 web pages read. Tags:

- **[primary]**: I read the source itself, or a full reproduction or preprint of it.
- **[secondary]**: I read the claim only in another author's text.
- **[unconfirmed]**: I could not verify it.

PDFs were turned into text locally and searched. Quotes are verbatim from that text.

---

## 1. Fruin's body ellipse and shoulder breadth

### Fruin's ellipse

- **[unconfirmed] The exact "18 in by 24 in" wording.** No page I read states Fruin's ellipse in
  inches. I could not reach Fruin (1971) itself. The figure fits the two sources below, but I did
  not see it in print.
- **[secondary] 600 mm wide by 450 mm deep, area 0.212 m2** (about 23.6 by 17.7 in).
  - Source: Sorsa, J., Ruokokoski, M. and Siikonen, M.-L., "Human Body Size in Lift Traffic Design",
    *4th Symposium on Lift & Escalator Technologies* (2014), pp. 214-215. The proceedings year is
    from memory; I did not check it on the page.
  - What it says: "the area of the Fruin body ellipse with width 600 mm and depth 450 mm, which
    includes an additional 20 mm space in width and 120 mm in depth [Fruin 1971]."
  - How the ellipse was built: it was "derived for a large 95th percentile male with respect to
    maximum body breadth and depth". The paper also says: "the 95th percentile maximum body breadth
    and depth were 580 mm and 330 mm". The source for those two dimensions is Hertzberg, Emanuel and
    Alexander (1956), *The anthropometry of working positions*, WADC.
  - Average-man values from the same 1950s data: "body breadth 530 mm and depth 290 mm". With the
    same allowances, this gives an ellipse of 0.177 m2.
- **[secondary] Clothed-labourer measure behind Fruin's ellipse: 22.8 in by 13 in (57.9 by 33 cm).**
  - Source: G. K. Still, *Crowd Dynamics*, PhD thesis, University of Warwick (2000), Chapter 3,
    section 3.1.1. Read online at gkstill.com/CV/PhD/Chapter3.html.
  - This matches Sorsa's 580 by 330 mm. Still also says Fruin added about 1.5 in "to allow for heavy
    clothing", and calls the allowance overgenerous.
  - The two sources disagree on the allowance. Sorsa says Fruin added 20 mm in width and 120 mm in
    depth.
- **[primary] FHWA metric equivalent.**
  - Source: FHWA, *Capacity Analysis of Pedestrian and Bicycle Facilities*, publication FHWA-HRT-98-107 as the web page shows it (Feb 1998),
    section 2.1.
  - What it says: "a simplified body ellipse of 50 cm x 60 cm for standing areas ... with a total
    area of 0.3 m2, or roughly 108 percent of the ellipse suggested by Fruin."
  - My arithmetic: 0.30 / 1.08 = 0.278 m2. That equals the bounding rectangle of 18 by 24 in
    (0.457 m x 0.610 m = 0.279 m2). This supports 18 by 24 in indirectly.
  - Caution: FHWA's "area" is the product of the two axes, not the true area of the ellipse. The
    true area of an 18 by 24 in ellipse is pi/4 x 0.457 x 0.610 = 0.219 m2, which agrees with
    Sorsa's 0.212 m2.

### Shoulder breadth of adult men

- **[primary] ANSUR II, U.S. Army men.**
  - What I did: computed the statistics myself from the public data file
    `ANSUR_II_MALE_Public.csv` (n = 4082, units mm), downloaded through Penn State OPEN Lab.
  - Report: Gordon, C. C. et al. (2015), *2012 Anthropometric Survey of U.S. Army Personnel:
    Methods and Summary Statistics*, NATICK/TR-15/007. I did not read the report's own tables. The
    values below are my computation (sample SD).

  | Dimension (men) | Mean | SD | P5 | P50 | P95 |
  |---|---|---|---|---|---|
  | Biacromial breadth (skeletal shoulder width) | 415.7 | 19.2 | 384 | 415 | 447 |
  | Bideltoid breadth (max shoulder width over the deltoids) | 510.4 | 32.5 | 459 | 509 | 567 |
  | Forearm-to-forearm breadth | 579.9 | 50.9 | 497 | 579 | 666 |
  | Chest breadth | 289.4 | 18.3 | 261 | 289 | 320 |
  | Chest depth | 253.8 | 26.2 | 211 | 253 | 298 |
  | Stature | 1756.2 | 68.6 | 1647 | 1755 | 1870 |

  - Caveat: these are modern soldiers, average height 175.6 cm. They are probably larger than
    ancient Mediterranean soldiers.
  - [unconfirmed] Measurement posture. A secondary table lists biacromial and bideltoid breadth as
    measured seated. I did not check this in the report.
- **[secondary] Pheasant and Haslegrave, *Bodyspace* (3rd edition, Taylor & Francis, 2006).**
  - Source: the table as reproduced in Sorsa et al. (2014), Table 1.
  - 95th-percentile shoulder (bideltoid) breadth for men:

    | Country | P95 shoulder breadth (mm) |
    |---|---|
    | USA | 515 |
    | UK | 510 |
    | France | 515 |
    | Netherlands | 520 |
    | Sweden | 510 |
    | Japan | 475 |
    | Poland | 475 |
    | India | 440 |
    | Sri Lanka | 400 |

  - 95th-percentile chest depth for men: USA 290 mm, UK 285 mm.
  - Pheasant's own ellipse adds 50 mm in both breadth and depth, giving 0.189 m2.
- **[unconfirmed] NASA-STD-3000.** Not read.

---

## 2. Crowd crush: densities, forces, asphyxia

### Densities

- **[primary] Fruin, J. J. (1993).**
  - Source: "The causes and prevention of crowd disasters", in R. A. Smith and J. F. Dickie (eds),
    *Engineering for Crowd Safety*, Elsevier, Amsterdam, pp. 99-108. The page range comes from
    Helbing and Mukerji's reference list. I read the full text as reproduced on gkstill.com, which
    has no page numbers.
  - "When crowd density equals the plan area of the human body, individual control is lost."
  - "At occupancies of about 7 persons per square meter the crowd becomes almost a fluid mass. Shock
    waves can be propagated through the mass sufficient to lift people off of their feet and propel
    them distances of 3 m (10 ft) or more."
- **[primary] Helbing, D. and Mukerji, P. (2012).**
  - Source: "Crowd disasters as systemic failures: analysis of the Love Parade disaster", *EPJ Data
    Science* 1:7. I read the arXiv version, 1206.5856.
  - "Once the crowd density exceeds between 4 or 5 persons per square meter, congestion can build up
    quickly, which implies high risks for people to stumble or fall."
  - Table 6, stage 0: "Densities are below 2-3 persons per square meter. Normal operation at low
    risk", with a safe flow of "82 persons per minute and meter" (1.37 persons per m per s).
  - The top stage, "Crowd turbulence occurs", is described as: "Injuries and fatalities are likely."
  - They stress that "it is often not the density alone that kills ... but the particular kind of
    dynamics", meaning force chains and "crowd turbulence".
- **[primary] Helbing, D., Johansson, A. and Al-Abideen, H. Z. (2007).**
  - Source: "The dynamics of crowd disasters: an empirical study", *Physical Review E* 75, 046109.
    I read arXiv physics/0701203v2. These are measured values from video of the 2006 Hajj crush at
    Jamarat.
  - Local densities ">10 persons per square meter" when measured over a radius of 1 m.
  - "The maximum local densities are twice as high as the average densities."
  - "When a local density of 6 persons per square meter is exceeded, the local flow decreases by a
    factor of 3 or more."
  - There is "a second flow maximum at 9 persons/m2".
  - Stop-and-go waves begin when the average flow drops below 0.8 persons per m per s.
  - Turbulence begins when the "pressure" (density times the variance of velocity) exceeds 0.02 s^-2.
- **[primary] Still (2000), PhD thesis, Chapter 3.**
  - "From the Green Guide we have the limit of 4 people per square metre set as a safe density for a
    moving crowd" (section 3.1.8).
  - "Densities of 4.7 people per square metre are typical in and around Wembley Stadium."
  - Section 3.1.8 also states a criterion: "exposed to density above 4 people per square metre for
    more than 6 minutes". I saw it only as an extract, so I do not know its full context.
  - "density never exceed 6.25 people per square metre in some systems" (section 3.6).
- **[secondary] Still's "redlines".**
  - Static crowds: more than 5 persons per m2. Moving crowds: more than 3 persons per m2.
  - A classroom test: at "around 6-7 people per square metre" everyone is in contact and "any
    movement has a ripple (or domino) effect".
  - I saw both only in search snippets from a company blog (eiai.ie) and did not read the pages.

### Forces

- **[primary] The 4,500 N figure: Fruin (1993), inferred from damage, not measured.**
  - "Evidence of bent steel railings after several fatal crowd incidents show that forces of more
    than 4500 N (1,000 lbs.) occurred. Forces are due to pushing, and the domino effect of people
    leaning against each other."
  - No instrument measured this force. Fruin back-calculated it from bent railings.
- **[primary] Measured forces, as Fruin (1993) cites them.** I did not read the original reports.
  - Leaning and pushing on guardrails produces "force of 30% to 75% of participant weight".
  - US National Bureau of Standards guardrail study, three persons: 792 N leaning and 609 N pushing.
  - Australian Building Technology Centre study, three persons leaning and pushing together: 1370 N.
  - Same study, "5 persons were capable of developing a force of 3430 N (766 lbs.)" in a simulated
    "panic".
- **[primary] Calculated chest load, Fruin (1993).** People at the bottom of a pile about 3 m high
  "would experience chest pressures of 3600-4000 N". This is calculated, assuming half the weight
  above rests on the upper body.
- **[secondary] The "6 to 7 people" attribution.** A later review states that "forces of up to 4500 N
  can be generated by just 6 to 7 people pushing". I saw this only in a search snippet. The source
  appears to be the Cambridge *Disaster Medicine and Public Health Preparedness* review "Human
  stampedes: a systematic review of historical and peer-reviewed sources". It is not Fruin's own
  claim.
- **[primary] Damage at the Love Parade, Helbing and Mukerji (2012).** "Around 16:40, the forces in the
  crowd were so critical that a traffic sign was bent." This too is evidence from damage, not a
  measurement.

### Asphyxia: mechanism and timing

- **[primary] Fruin (1993).**
  - "Virtually all crowd deaths are due to compressive asphyxia."
  - "Victims of compression asphyxia can be revived only if resuscitation begins quickly."
  - He gives no timing.
- **[primary] Gill, J. R. and Landi, K. (2004).**
  - Source: "Traumatic asphyxial deaths due to an uncontrolled crowd", *American Journal of Forensic
    Medicine and Pathology* 25: 358-361. The case is 9 deaths in New York in 1991.
  - Victims "typically die ('standing up') in a vertical position".
  - "When force is applied front to back or vice versa, chest expansion is compromised. When force is
    applied side to side, chest expansion is not compromised."
  - "Victims do not collapse to the floor until after the crowd density and pressure have been
    relieved."
  - There were minimal blunt injuries and no fractures. The paper gives no timing.
  - Relevance: a disc model should probably treat front-to-back load as the lethal direction and
    side-to-side load as much less dangerous.
- **[secondary] Timing: G. Keith Still, quoted by NPR, as cited in Al Jazeera (30 Oct 2022).**
  - "It takes 30 seconds before you lose consciousness, and around about six minutes, you're into
    compressive or restrictive asphyxia."
  - This is an expert's estimate in a press interview, not a measurement.
- **[secondary] Timing: a critical-care anaesthesiologist quoted by AP (Astroworld coverage, Dec 2021).**
  - Victims would pass out "within a minute or so". After "three to four minutes" the injury cannot
    be reversed.
  - Seen only in search snippets.
- **[unconfirmed] A measured or peer-reviewed time to loss of consciousness.** I did not find one.

---

## 3. Bottleneck flow

### Normal walking: Seyfried et al. (2009)

- **[primary] Source and setup.**
  - Seyfried, A., Rupprecht, T., Passon, O., Steffen, B., Klingsch, W. and Boltes, M., "New insights
    into pedestrian flow through bottlenecks", *Transportation Science* 43(3): 395-406. I read arXiv
    physics/0702004v2 (dated 18 Oct 2007).
  - Bottleneck 2.8 m long, width b from 0.8 to 1.2 m in steps of 0.1 m.
  - Runs with N = 20, 40 and 60 people, starting at a density of 3.3 per m2.
  - Participants were told "not to push and to walk with normal velocity".
- **[primary] Specific flow Js, from Table 2, in persons per m per s.**

  | b (m) | N = 60 | N = 40 | N = 20 |
  |---|---|---|---|
  | 0.8 | 1.61 | 1.77 | 1.86 |
  | 0.9 | 1.86 | 1.91 | 2.06 |
  | 1.0 | 1.90 | 2.08 | 2.19 |
  | 1.1 | 1.93 | 1.93 | 1.78 |
  | 1.2 | 1.97 | 1.81 | 2.31 |

  - Figure 9 fits all the data with **J = 1.9 b** (persons per s, b in m).
- **[primary] Growth with width is linear and continuous, not stepwise.**
  - "The data show a linear growth of the flow with the width."
  - "All of these results are compatible with a linear and continuous increase of the flow with the
    width of the bottleneck."
  - Lanes form for b >= 0.9 m through the "zipper effect". Their spacing "increases continuously with
    b", so the flow does not step.
  - The only possible edge is near b = 0.7 m in Kretz's data, where the zipper effect begins.
- **[primary] Planning guidelines.** The guidelines in the paper (SFPE handbook, Weidmann,
  Predtechenskii and Milinskii) predict a capacity of 1.2 to 1.6 persons per m per s.
- **[primary] Starting density matters a great deal.**
  - In Nagai's data at b = 1.2 m, J rose "from J = 1.04 s^-1 to 3.31 s^-1 when the initial density is
    increased from 0.4 m^-2 to 5 m^-2".
  - Density in the jam in front of the bottleneck: 4 to 6 per m2.
- **[secondary] Kretz et al. (2006), as listed in Seyfried's Table 1.**
  - J of about 1.43, 1.58, 1.85 and 2.15 persons per s at b = 0.8, 0.9, 1.0 and 1.2 m. Kretz's
    bottleneck was 0.4 m long.
  - Caution: I inferred which column is which from the extracted text.
- **[secondary] Hoogendoorn and Daamen (2005).**
  - Their claim: capacity grows stepwise, one lane at a time, with lane spacing independent of width.
  - The claim rests on only two widths, 1 m and 2 m.
  - Seyfried's Table 1 lists the "HG" values as 0.78 persons per s (b = 1.0 and 1.2 m) and 1.56
    persons per s (b = 1.5 and 1.6 m).
  - I did not read the original.
- **[primary] Range from fundamental diagrams, as reviewed in Garcimartin et al. (2016).**
  - Maximum Js of 1.2 to 1.9 persons per m per s, at critical densities of 1.2 to 2.1 per m2.
  - Js = 1.8 at 4 per m2, in a study reaching 10 per m2.
  - Js = 1.5 at 6.6 per m2.
  - Osaka commuters: Js = 2.3 at 3.6 per m2.

### Pushing: Garcimartin et al. (2016)

- **[primary] Source and setup.**
  - Garcimartin, A., Parisi, D. R., Pastor, J. M., Martin-Gomez, C. and Zuriguel, I., "Flow of
    pedestrians through narrow doors with different competitiveness", *J. Stat. Mech.* (2016) 043402.
  - Doors 75 cm and 69 cm wide. Starting density about 4 per m2.
  - Three competitiveness levels, by instruction:
    - LC: avoid contact;
    - MC: soft contact allowed;
    - HC: "moderate pushing and elbowing ... allowed".
- **[primary] Results.**
  - Js is about 4.0 persons per m per s (J = 2.8 per s) "independently of the scenario". That is about
    twice the normal-walking values above.
  - At HC there is an initial burst of "3.2 persons per second, so 4.5 persons per second per meter".
    The flow then "decreases abruptly" as density builds at the door.
  - Faster-is-slower is "obvious" for the 75 cm door. For the 69 cm door it appears "only ... in a
    certain lapse of the evacuation". For the last few people "'faster is faster'".
  - The time gaps between passages have heavy, power-law tails (clogs).

### Pushing: Pastor et al. (2015)

- **[primary] Source and setup.**
  - Pastor, J. M., Garcimartin, A., Gago, P. A., Peralta, J. P., Martin-Gomez, C., Ferrer, L. M.,
    Maza, D., Parisi, D. R., Pugnaloni, L. A. and Zuriguel, I., "Experimental proof of
    faster-is-slower in systems of frictional particles flowing through constrictions", *Physical
    Review E* 92, 062817. I read arXiv 1507.05110v1.
  - 95 ± 3 people through a 69 cm door.
  - Approach speeds were 0.410, 0.497 and 0.717 m/s at low, medium and high competitiveness.
  - Pressure sensors at the doorpost (128 of them) counted readings above 3 kg/cm2, and the count
    rose with competitiveness.
- **[primary] Results.**
  - The mean time gap rises with competitiveness, so flow falls. The paper claims this is
    faster-is-slower in humans.
  - The effect is mild: "<tau> is only mildly altered".
  - The tail exponent of the time-gap distribution is the clearer sign: alpha = 6.8, 5.5 and 4.2 for
    low, medium and high. Lower alpha means more frequent long clogs.
  - In humans they show only the rising branch. They did not trace the minimum of evacuation time
    against desired speed.
- **[unconfirmed] Not read.** The Julich motivation-level entrance experiments (e.g. Adrian et al.
  2020) and Zuriguel et al. (2014) on clogging and arching.

---

## 4. Turning on the spot

- **[primary] Mason et al. (2022).**
  - Source: Mason, R., Byerley, J., Baker, A., Powell, D., Pearson, L. T., Barry, G., Godfrey, A.,
    Mancini, M., Stuart, S. and Morris, R., "Suitability of a low-cost wearable sensor to assess
    turning in healthy adults", *Sensors* 22(23): 9322. Table 2.
  - Subjects: 30 healthy adults aged 23.9 ± 4.9. Yaw was measured at the lower back
    (pelvis/trunk), not the head.

  | Task | Peak yaw rate (deg/s) | Mean yaw rate (deg/s) |
  |---|---|---|
  | 360 deg turns in place, alternating, for 2 min | 190.6 ± 27.4 (AX6); 189.8 ± 30.9 (Opal) | 123.6 ± 21.4; 125.8 ± 24.3 |
  | 180 deg turns while walking (2-minute walk, 5 m course) | 178.8 ± 25.3; 181.6 ± 30.0 | 80.7 ± 10.7; 83.7 ± 12.0 |
  | Turning course | about 123 | about 56 |

- **[unconfirmed] Pace instruction.** I did not confirm whether the turns were at the subject's own
  pace or as fast as possible. Treat the figures as typical, not maximal.
- **[unconfirmed] Maximal or athletic turning rates.** No source found.
