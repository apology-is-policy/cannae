> **Provenance.** Written by a research subagent briefed by Paullus on 2026-11-10 (S0.4, the skeletal and experimental half), read in full and kept verbatim. Varro covers the ethnography of line fighting and Collins. How this feeds the pre-registration is in [preregistration.md](preregistration.md), pattern 7.

# Lethality per bout: skeletal and experimental bounds, plus two citation checks

Researcher: claude-opus-5-5 subagent, 2026-11-10. Reading budget: about 30 fetches. "Primary" means I read
the publication or its indexed abstract. "Secondary" means I read only a summary, which is named.

## Task B: citation checks

### B1. Skiba et al. 2012: W' recovery time constant

**Citation (confirmed from the PubMed record via NCBI E-utilities):** Skiba PF, Chidnok W, Vanhatalo A,
Jones AM. "Modeling the expenditure and reconstitution of work capacity above critical power." *Medicine
and Science in Sports and Exercise* 2012 Aug; 44(8): 1526-1532. doi:10.1249/MSS.0b013e3182517a80. PMID 22382171.
Title, authors and journal match the code's citation. The volume (44) is right; the issue is 8 and the pages are 1526-1532.

**Formula: confirmed, from a secondary source.** tau_W' = 546 * exp(-0.01 * D_CP) + 316, in seconds,
with D_CP = CP - (recovery power), in watts. I could not open the full text (the PMC and Exeter
repository copies were blocked). The formula, attributed to Skiba et al. 2012 with the exponent written as
(CP - P_i), is reproduced in the documentation of the R package trackeR (topic `Wprime`, v1.5.2). The abstract
confirms that the time constants "were negatively correlated with the difference between sub-CP recovery
power and CP" and that the relationship "was best fit by an exponential (r = 0.77)". In the 2012 model,
recovery power is the mean of all power values below CP over the session, not the instantaneous value.
That matters if the code applies tau moment by moment. The later differential form is a different paper:
Skiba, Fulford, Clarke, Vanhatalo and Jones, *Eur J Appl Physiol* 115(4): 703-713 (2015).

### B2. Polybius 18.29-30: frontage

Greek text: Büttner-Wobst (Teubner, Leipzig 1893-), from the Perseus `canonical-greekLit` TEI
(tlg0543.tlg001.perseus-grc2), extracted directly. The translations are mine.

- **18.29.2:** ἐπεὶ γὰρ ὁ μὲν ἀνὴρ ἵσταται σὺν τοῖς ὅπλοις ἐν τρισὶ ποσὶ κατὰ τὰς ἐναγωνίους πυκνώσεις
  -- "since the man stands with his arms in three feet in the close orders used for battle".
- **18.30.6:** ἵστανται μὲν οὖν ἐν τρισὶ ποσὶ μετὰ τῶν ὅπλων καὶ Ῥωμαῖοι -- "The Romans too stand in three feet with their arms."
- **18.30.7-8:** τῆς μάχης δʼ αὐτοῖς κατʼ ἄνδρα τὴν κίνησιν λαμβανούσης διὰ τὸ τῷ μὲν θυρεῷ σκέπειν τὸ σῶμα,
  συμμετατιθεμένους αἰεὶ πρὸς τὸν τῆς πληγῆς καιρόν, τῇ μαχαίρᾳ δʼ ἐκ καταφορᾶς καὶ διαιρέσεως ποιεῖσθαι τὴν
  μάχην, προφανὲς ὅτι χάλασμα καὶ διάστασιν ἀλλήλων ἔχειν δεήσει τοὺς ἄνδρας ἐλάχιστον τρεῖς πόδας κατʼ
  ἐπιστάτην καὶ κατὰ παραστάτην, εἰ μέλλουσιν εὐχρηστεῖν πρὸς τὸ δέον -- "but since for them the fighting
  involves each man's own movement, because they cover the body with the shield, constantly shifting it to
  meet each blow, and fight with the sword by cut and thrust, it is plain that the men must have slack and
  separation from one another of at least three feet, both towards the man behind and towards the man beside,
  if they are to be effective."
- **18.30.9:** ἐκ δὲ τούτου συμβήσεται τὸν ἕνα Ῥωμαῖον ἵστασθαι κατὰ δύο πρωτοστάτας τῶν φαλαγγιτῶν, ὥστε πρὸς
  δέκα σαρίσας αὐτῷ γίνεσθαι τὴν ἀπάντησιν -- "so it follows that one Roman stands opposite two front-rank
  phalangites, and has to meet ten sarissas".

**What it claims.** Each Roman takes the same three feet as a phalangite, plus at least three feet of
clearance, so the Roman frontage is six feet, twice the phalangite's. 18.30.9 settles this arithmetic.
The scenario files are right, with three refinements:
1. The interval is a **minimum** (ἐλάχιστον).
2. The interval applies **in depth too** (κατʼ ἐπιστάτην, "to the man behind"), not only sideways.
3. The phalanx's three feet is its **compacted battle order** (ἐναγωνίους πυκνώσεις), not its normal order.

The Greek says only "in three feet". Paton's Loeb (LacusCurtius) adds "in breadth" at both 18.29.2 and
18.30.6, and renders 18.30.8 as "from the man next him in the same rank". I saw only an excerpt of the
rest of that sentence, so I can't say whether his rendering keeps the depth clause. A Greek foot is about
0.30 m (not re-checked), so six feet is about 1.8 m.

## Task A: skeletal and experimental constraints on lethality per bout

**The inferential gap, stated once.** None of these sources measures what one bout does. Battle graves
hold only the dead, mostly men killed in rout, massacre or execution after they were helpless. Bone
records only some wounds (soft-tissue kills leave no trace), and taphonomy removes more. These
assemblages constrain *where* blows land, *how survivable* wounds were, and *how killing happens*. They
do not give a per-bout rate. Only gladiator bouts come close.

| # | Source | What was measured | Numbers | Implication for P(disable) per bout | Confidence |
|---|---|---|---|---|---|
| 1 | Kanz F, Grossschmidt K. "Head injuries of Roman gladiators." *Forensic Sci Int* 160(2-3): 207-216 (2006). Abstract read (primary). | Cranial trauma, Ephesus gladiator cemetery, 2nd-3rd c. AD | MNI 68, males mostly 20-30. **11 (16%) had 16 well-healed cranial traumata**, 5 of them multiple. **10 (15%) had 10 perimortem cranial traumata.** No individual had multiple perimortem injuries ("strict nature of combat rules"). | Head wounds were survived about as often as they killed, in men fighting single formal bouts. Fatal head trauma may be partly a deliberate death-blow, which the authors suggest. A regulated bout ends with **one** decisive wound; the many-wound pattern of battle graves comes from killing helpless men. Supports keeping "killed" a minority of disabling outcomes. | High on the numbers, medium on interpretation |
| 2 | Pompeian fight records (1st c. AD), as read by A. Manas (UC Berkeley), quoted in *Live Science*, "Did Roman gladiators really fight to the death?" (secondary). G. Ville, *La gladiature en Occident* (Rome 1981), not seen. | Share of fights ending in the loser's death | About **1 death in 5 fights** in the 1st c. AD (so about 0.1 per fighter per fight); about 1 in 2 in the 3rd c. Popular sources repeat "1 in 9" per fighter (not checked). | **Upper anchor.** A gladiator fight went to a decision between two men with no neighbours, no rear ranks, often a bare torso and a small shield. If a fight held about 5-20 committed exchanges (**unknown; no source found**), then P(decision \| bout) is about 0.05-0.2 and P(death \| bout) about 0.01-0.04. Formed shield infantry should sit below this. | Medium for the per-fight figure, low for the conversion |
| 3 | Novak S. "Battle-related trauma." In Fiorato V, Boylston A, Knüsel C (eds), *Blood Red Roses*, Oxbow 2000, pp. 90-102. Read via *Medievalists.net* (2024) and *Strange Remains* (2015) (secondary). | Towton mass grave, AD 1461 | **27 of 28 crania** had perimortem weapon injuries. **113 cranial wounds** in total, about 4 per skull (Strange Remains only). **9 crania had healed earlier wounds** (about 32%). 13 individuals had postcranial wounds, nearly half only one, one had nine. No torso wounds. Forearm wounds were the commonest postcranial injury. The count of individuals varies by source (37-51). | Overkill on helpless men: the deaths happened in the rout, not in formed exchange. A third were veterans carrying survived head wounds, so a fighter survived more than one battle. Forearm "defensive" wounds suggest that "hampered" was common. This gives no per-bout rate. | Medium (secondary, but consistent across sources) |
| 4 | Ingelmark BE. "The skeletons." In Thordeman B, *Armour from the Battle of Wisby 1361*, Stockholm 1939. Read via *Medievalists.net* (secondary). | Visby, AD 1361: three mass graves | **1185 individuals.** **456 "cases" of cutting-weapon wounds** (wounds or men, ambiguous) and 126 arrow or bolt wounds. Head wounds in graves 1-3: **42.3%, 52.3%, 5.4%**. Mostly lower-body wounds (no percentage found). | Under about 0.5 bone-registered wounds per dead man: many men died with no bony lesion. Bone counts therefore undercount wounds, and "wounds per victim" cannot serve as "hits to kill". Head and legs were where blows landed past shield and mail. | Medium-low (secondary; 1185 matches several sources) |
| 5 | Redfern RC. "A re-appraisal of the evidence for violence in the Late Iron Age human remains from Maiden Castle hillfort." *Proc Prehist Soc* 77: 111-138 (2011). Read via Smith M, Russell M, Cheetham P, "Fraught with high tragedy...", *Oxford J Archaeol* (2025), doi:10.1111/ojoa.12324 (open-access PDF, read). | "War cemetery", 1st c. AD | **26 of 62 adults (42%)** had assault trauma: males 18/36, females 8/25. **78 injuries**, 7 of them healed. Only 9 of 26 had a single wound; one had 11. New radiocarbon dates spread the deaths over several episodes, not one Roman assault. | Assault and overkill again, not line fighting. Its relevance to bouts is the same as Towton's: killing is concentrated on disabled or defenceless men. | High (numbers), but not a battle |
| 6 | Holst MK et al. "Direct evidence of a large Northern European Roman period martial event and postbattle corpse manipulation." *PNAS* 115(23): 5920-5925 (2018). Full text read (primary). | Alken Enge, Jutland, early 1st c. AD | 2,095 bones, **MNI 82** (estimated more than 380). Perimortem trauma on **139 bones** (sharp 119, penetrating 14, blunt 6). Antemortem (healed) trauma on **19 bones**. The authors read the "relative absence of healed sharp force trauma" as little previous battle experience. | Disarticulated, so no per-man counts. Perimortem lesions outnumber healed ones about 7 to 1, on the femora, tibiae, humeri and crania (the limbs and head outside shield cover). Some posterior femoral cuts suggest fleeing or immobilised men. | High (numbers), low for per-bout use |
| 7 | Tollense (Brinker U et al., ch. 3 in Dolfini A et al. (eds), *Prehistoric Warfare and Violence*, Springer 2018; Flohr S et al., *Int J Paleopathol* 9: 76-81, 2015); Kalkriese (Großkopf 2007, secondary snippet); Krefeld-Gellep (not searched) | -- | Tollense: over 110-140 individuals, some with healed lesions; no healed-share figure found. A once-"healed" arrow wound was reclassified as perimortem by micro-CT. Kalkriese: eight bone pits, MNI not verified. | Nothing quantitative. Healed shares are sensitive to method. | Low |
| 8 | Williams A. *The Knight and the Blast Furnace*. Brill 2003, ch. 9 (read via myArmoury forum and blog summaries, secondary). Godehardt E et al. 2007 (arrowheads against a plywood-and-leather scutum, abstract only). D. Sim's pilum drop tests (Osprey blog, 2017). | Energy needed to defeat armour; energy that weapons deliver | Arrow through **riveted mail plus padded jack: about 120 J**. Sword or axe blow: **60-130 J**. Thrust: **up to about 63 J underarm, 115 J overarm** (Horsfall). Scutum tests: qualitative only. Pilum and arrows do penetrate plywood. | A thrust or cut landing on mail plus padding usually fails to penetrate. Cuts on mail become blunt trauma. A blow to the shield is spent on the shield. So disabling hits need an unarmoured or lightly covered target (face, neck, forearm, thigh, shin). For a scutum-and-mail legionary that is a small share of the body. I infer that much less than half of landed blows defeat the protection; this is not a measured fraction. | Low-medium (secondhand; medieval mail, not Roman) |

**Net result.** Hits land on the head and limbs (Towton, Visby, Alken Enge, Maiden Castle agree). Healed weapon trauma is common (Towton 9/28 crania, Ephesus 11/68), so many wounds only hamper. A formal bout ends on one decisive wound, while graves show repeated blows to helpless men: killing clusters after disablement or flight.

### Proposed range

Let P_d = P(at least one of the two men is disabled or killed in one committed bout).

**1. The casualty clock makes P_d a function of the bout rate.** Assume about one disabling hit per front
position per side every T = 1.5-3 h. A facing pair then suffers about 2 disabling hits per T. With b
committed bouts per pair per hour, P_d is about 2/(b*T):

| Bout rate b (per pair per hour) | Share of time in bouts (15 s bouts) | P_d |
|---|---|---|
| 20 | about 8% | 0.03-0.07 |
| 60 | about 25% | 0.011-0.022 |
| 120 | about 50% | 0.006-0.011 |

**2. The gladiator upper anchor.** Unprotected, fight-to-decision bouts give P(decision \| bout) of about
0.05-0.2. A large shield, mail and a helmet, plus neighbours and the option of not pressing, should cut
this several-fold for formed infantry.

**3. Skeletal consistency.** P_d must leave room for (a) survived wounds being common and (b) veterans
surviving several battles. That argues against P_d much above 0.1. Hits concentrate on the head and
limbs, so the killed share of P_d should be small.

**Proposal:**
- **P_d per bout (either man): 0.01-0.07**, central 0.02-0.04. Outer envelope 0.005-0.15.
- **Per man:** half of P_d.
- **Killed outright:** about 10-30% of P_d, so roughly 0.002-0.02 per bout. Most disabled men die later,
  in rout or by dispatch, not in the bout.
- **"Hampered":** should be at least as frequent as "disabled", given the defensive forearm wounds and the
  healed-trauma prevalence.

**Weakest link.** The number of committed bouts per unit time is unknown, for gladiator fights and for
formed lines alike. The range is therefore set mainly by the casualty clock plus an assumed bout rate. The
skeletal and experimental data only check it for consistency (where hits land, how survivable they are,
and that killing follows disablement); none of them measures it. The gladiator anchor carries a
second unverified step: Ville's death rates, here taken from a secondary report, and an assumed 5-20
exchanges per fight.

Fetched: PubMed via E-utilities (PMIDs 22382171, 16289900, 29784805), PMC6003345, Perseus TEI on GitHub,
LacusCurtius Paton, rdocumentation trackeR, Bournemouth eprint 41028, medievalists.net (2024), Live Science.
