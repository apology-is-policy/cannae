> **Provenance.** Written by a research subagent briefed by Varro on 2026-11-10 (S0.4, the half on
> line-fighting ethnography, Collins and harm per exchange), read in full and kept verbatim below.
> The skeletal and experimental half is in [lethality-skeletal.md](lethality-skeletal.md).
>
> **What Varro takes from it** (the report advises; these are my decisions, for Paullus to sign):
>
> - **The lower end of pattern 7's band moves from 0.005 to 0.002.** The report finds 0.005 "about
>   right, leaning slightly high for an outer bound", with two independent and weak lines of
>   arithmetic that both reach a factor of four or more below it. The band is an outer envelope,
>   so it should sit where the evidence stops, not where it centres. A lower floor favours the
>   engine with more fighting and less harm in each bout, which is Paullus's and not mine; that is
>   why he asked me to set it.
> - **The prior on harm per strike (priors-shared.md, S8) is log-uniform from 0.003 to 0.1,** inside
>   the band and weighted toward its plausible part.
> - **Tempo is reported, not banded:** the share of a front-rank man's time spent forward while the
>   lines are formed, and the share of front-rank men who start any bout. The report's figures (5 to
>   25 % of the time; 10 to 25 % of the men) rest on Dani skirmishing and on an inference, which is
>   too little to fail an engine on.
> - **Nothing here measures harm per exchange between armoured men.** The number of passes in a
>   duel and the worth of a shield and armour are both assumed. Everything above inherits that.

Researcher model: claude-fable-5-2 (subagent), 2026-11-10.

# Lethality per bout and tempo: line-fighting ethnography, Collins, and harm per exchange

Reading budget: 35 fetches or searches, all used (list in section 5). This is the second half of a split
search; the skeletal, gladiator and armour half is in `lethality-skeletal.md` and is not repeated here.

**Labels.** "Primary" means I read the publication itself (here usually the full text as OCR on the
Internet Archive, or the publisher's PDF). "Secondary" means I read only a summary, which is named.
"Search excerpt" is the weakest kind of secondary: I saw only the sentences a search tool quoted from the
page. "From memory" means not seen at all in this search; such items are listed only as leads and carry
no weight in the proposal. OCR page numbers are from running heads and may be off by one.

## 1. Source tables

### A. Ethnography of massed fighting at a standoff

| # | Source | What was observed | Numbers | Implication for P_d and tempo | Confidence |
|---|---|---|---|---|---|
| A1 | Heider KG. *The Dugum Dani*. Viking Fund Publications in Anthropology 49, 1970, pp. 105-112, 128, 230-231, 281-282, Appendix II (pp. 310-313). Full text read (primary). | Grand Valley Dani, formal battles and raids, April to early September 1961, observed by the author | **5.5 months: 9 battles (2 did not develop) and 9 raids. 2 men died of wounds received in battles; 6 men and boys were killed outright in raids** (p. 107). "In the course of eight battles I observed ... no one was carried away dead, although two men died later" (p. 111). No immediate death from an arrow in the whole fieldwork (p. 282). Front rank "not more than twenty or thirty meters deep"; "**seldom more than a dozen men on the actual front**. Fighters move up, shoot arrows or throw small spears, and then drop back" (p. 110). Active fighters mostly aged 18-30. Men strung out a kilometre or more behind the front. Battleground strip about 50 m wide. Fighting starts between 9 and 11 and runs "until rain or dusk"; noon rests; one late engagement "fought for fifteen minutes", another had "half an hour of fighting" between long pauses (App. II). Spear wounds "rare" because battles "rarely involve hand-to-hand" fighting (p. 281). Ritual war deaths 0.48% of population per year (4 of about 2,000 in five months); 100 of 350 remembered male deaths (28.5%) from war, which he calls inflated (pp. 230-231). | A day-long battle of hundreds produced on average a quarter of a death. At any moment about a dozen men were in the danger zone, and each chose when to go in and when to leave. Killing was done in raids, not battles. Heider's own three statements of the delay before the two deaths differ slightly (10 days to 9 months; "six weeks"; "some months"). | High on the counts (eyewitness, small sample), not transferable as a rate |
| A2 | Gardner R, Heider KG. *Gardens of War*. Random House 1969, pp. 136-141 (two-column OCR; exact page uncertain). Full text read (primary). | The same battles, described from film and field notes | "**Sustained fighting along an active and open front seldom lasts longer than ten or fifteen minutes and rarely involves more than two hundred men on each side -- and usually less than half this number.**" "**An average day's fighting will consist of ten to twenty clashes**"; "a steady series of brief clashes and relatively long interruptions". Opening probe by "perhaps thirty or forty" men; first arrows at fifty yards; in the thick of it "fifty or more men are shooting arrows, often no further away than fifty feet". Deployment gap about 500 yards; taunting across 200 yards. "Several hundred on both sides", "the handful" wounded, death "rarely". "Every foray of any size or intensity will provide some wounded on both sides." Men are hit in the back and buttocks when they turn to run back. Fresh groups are sent in to relieve those at the front. "Half the deaths in all Dani fighting can be attributed to infection." Warriors go six or seven hours without food. Mead's introduction: "few of them at any one moment heavily engaged". | **Best tempo source found.** By my arithmetic: 10-20 clashes capped at 10-15 min each is 100-300 min at the very most, in a day of roughly 7-9 hours. "Brief clashes and relatively long interruptions" puts the active share under half; I read it as a tenth to a third. Within a clash, a dozen at the very front (A1) out of 100-200 engaged is 6-12%, more when fewer were engaged. The dangerous moment is disengaging, not engaging. | High on the description; my shares are inference |
| A3 | Vayda AP. *War in Ecological Perspective*. Plenum 1976, ch. 2, pp. 15-25 and notes. Full text read (primary; his data are informants' accounts, not observation). Rappaport 1967 not seen. | Maring (Tsembaga and neighbours), New Guinea, wars before the mid-1950s | **"Nothing fights":** daily at a fixed fight ground, within arrow range, from behind wooden shields "as tall as the men and about 2 1/2 feet wide"; men "darted out from behind the shields to shoot their arrows and then darted back"; went on "for days and even for weeks, deaths or serious injuries in them were rare" (p. 15). **"True fights":** axes and jabbing spears added; "formations several ranks deep"; front-rank men "fought duels with one another from behind their huge shields", covered by archers in the rear ranks; "front positions were exchanged for rear ones from time to time" and men withdrew "to catch their breath"; "**most fatalities ... occurred when an enemy arrow or throwing spear brought down a man in the front ranks so that he could be finished off with axes in a quick charge**"; this "could proceed for weeks and even for months without heavy casualties"; not fought daily; truces up to three weeks (pp. 17-18). A rout "lasted but a few hours at the most". Heaviest recorded losses: Manamban (about 600 people) had **20 deaths at the fight ground** before a rout that killed 8 men and 3 women; Tyenda (about 300) lost 14 men, 6 women and 3 children in a rout (p. 25). One war fought only by archery across a river: over a month, nobody killed (note to ch. 2). | **The closest analogue found to shielded front-rank duelling.** With a body-length shield and the freedom not to press, close-weapon duelling between formed ranks was sustainable for weeks. Death followed disablement, as in the skeletal half. No count of fighting days, participants per day, or wounds. | Medium (reconstructed from informants; no durations) |
| A4 | Keeley LH. *War Before Civilization*. Oxford 1996, ch. 3-4 and 6, Table 4.1 (pp. 194-195), Table 6.1. Full text read (primary for his compilation; secondary for the ethnographies; several table cells illegible in the OCR). | Compiled casualty figures for "formal battles" | Mae Enga "Great Fight", 1930s: 2,000 engaged, 0.5% killed. **Mae Enga, one clan, 1971: 70 engaged, 1.4% killed, 38.6% wounded** (Meggitt 1977: 101), "which the clan regarded as a normal casualty rate"; over half the wounds were on the limbs and "not considered very serious"; **one killed per ten to thirty wounded** in Mae Enga formal battles. Mohave (closed with lances and clubs): 12% killed, 18% wounded of about 50. Manamba Maring 1956: 11.1% killed of an assumed 180 warriors (30% of population; battle deaths only). For arranged battles (Mtetwa, Cahto-Yuki, Mae Enga) "the proportion of warriors killed in action is below 2 percent" (pp. 63-65). Battles end when a side "has suffered a few serious casualties" (p. 60, OCR damaged). Mae Enga put "shield-bearing spearmen forward, with unshielded archers firing between and over them" (p. 46-47). Hand-to-hand fighting between groups "seldom took place in band and village societies"; discipline and file-closers are what bring men into "the killing zone" (pp. 46-47). | A battle-day can wound a large share of men (40%) while killing about 1%. Wounded-to-killed of 10-30 to 1 for missile fights, but about 2 to 1 (Keeley's text; 1.5 to 1 by his table) for the one group that closed with shock weapons (Mohave). | Medium (secondary for the underlying data) |
| A5 | Keeley 1996, ch. 6 (about pp. 91-94) and Table 6.1. Primary for the argument. | The cumulative-loss point | "A relatively low loss rate per war, battle, or raid can cumulate very rapidly": 5% lost per battle, four battles a year, is 64% in five years. Annual war deaths as % of population: Dugum Dani 0.48, Mae Enga 0.32, Murngin 0.33, Tiwi 0.16, Mtetwa 0.59; Germany 1900-1990 0.16, France 1800-1899 0.07. | Low loss per battle is not low loss per year. It also does not contradict a low P_d: the yearly toll comes from many battle-days plus raids and routs. | High for the argument; table sources not checked |
| A6 | Collins 2008, ch. 1, p. 28 (see B1). | His summary of tribal battle, from ethnographic film | "Battles are short, mostly skirmishes among a few hundred men or less, intermittently for a few hours, usually ending when a single victim is killed or seriously wounded ... they dart back and forth across a skirmish line, a few men at a time, running away if they are in enemy terrain for more than a few seconds." | Agrees with A1-A2. It is a generalisation, not a count. | Medium |
| A7 | Roscoe P. "Dead Birds: the 'theater' of war among the Dugum Dani." *American Anthropologist* 113 (2011). **Not seen** (the PDF link served another page); search summary only. | Reinterpretation | Argues that the restraint of Dani battle came from the terrain, not from cultural rules, and that the Dani meant to kill. | If right, low lethality at a standoff needs no "pulled punches". | Low (not read) |

Not seen at all: Meggitt 1977 (only through Keeley); Matthiessen 1962; Hart and Pilling 1960 (Tiwi;
only Keeley's rate from Pilling 1968); Warner 1937 (Murngin; only Keeley's remark that the peace-making
duel was "relatively harmless" but could turn serious); Chagnon; Rappaport 1967; Turney-High; Gat. For
pre-Shakan Nguni battles I have only Keeley's table rows from Otterbein 1967 (Mtetwa 1810 and 1813, 1,800
engaged, percentage killed illegible in my copy but under 2% by his text; Butelezi 1810, 600 engaged,
8.3% killed).

**What transfers.** (1) The intermittent shape: brief clashes, long pauses, ended by dusk, rain or a few
serious casualties. (2) A small share of men at the sharp end at any moment, with rotation. (3) Danger
concentrated at disengaging and on the man who goes down. (4) A large shield makes static close fighting
sustainable for a long time. (5) The heavy killing belongs to the rout, a separate and brief phase.
**What does not transfer.** (1) The per-exchange rate: these are arrows and thrown spears at 15-50 m,
without armour. (2) The freedom to choose distance: a drilled line held in place is precisely what these
societies lacked (Keeley's own point). (3) Scale, kinship between enemies, and agreed rules.

### B. Collins, *Violence: A Micro-sociological Theory* (Princeton 2008)

I read chapter 1 (the publisher's sample, pp. 1-35) and Collins's 2011 restatement. **Chapters 2, 3 and
10, which hold his numbers, were not seen.** Chapter numbers are from the preview in chapter 1 (pp.
32-34); chapter titles are from the publisher's table of contents as quoted in a search result (ch. 2
"Confrontational Tension and Incompetent Violence", ch. 3 "Forward Panic", ch. 10 "The Violent Few").

| # | Source | Claim | What it rests on | Measured or inferred | Confidence |
|---|---|---|---|---|---|
| B1 | Collins 2008, ch. 1, pp. 8-9, 32. Primary. | **Confrontational tension and fear** is the normal state of men in an antagonistic confrontation; violence happens only by a pathway around it. Ch. 2 ("Confrontational Tension and Incompetent Violence"): "most violence is bluster and standoff, with little actually happening, or incompetent performance". | Named in ch. 1: du Picq (questionnaires from combat officers), S.L.A. Marshall (post-battle interviews), Keegan, Grossman, and 1960s ethnographic film of tribal fighting. Griffith and Holmes are not named in ch. 1. | Inference from heterogeneous material. He says so: "My sources are very heterogeneous" (p. 32). | High that he claims it |
| B2 | Same, pp. 14-16. Primary. | **Fights are short.** "Most serious fights on the individual or small-group level are extremely short"; the O.K. Corral gunfight "took less than thirty seconds"; duels "typically very brief, within a few seconds of actual violence"; Mensur bouts "could be extended for as long as fifteen minutes" (Twain 1880) only because they were controlled; of boxing, "it takes continuous social pressure to keep a fight going". | Anecdote and comparison. | Inference. No distribution of fight lengths is given in ch. 1. | High that he claims it |
| B3 | Same, pp. 9, 32-33. Primary. | **Forward panic** (ch. 3): "when a tense confrontation is suddenly resolved in favor of one side, which takes overwhelming superiority"; the source of atrocity and overkill; a variant of attacking the weak. | Case studies. | Inference. | High that he claims it |
| B4 | Same, pp. 6, 33-34; Collins, "Reply to Kalyvas, Wieviorka, and Magaudda", *Sociologica* 2/2011, doi:10.2383/35867, pp. 2, 4. Both primary. | **The violent few** (ch. 10-11): riot photographs "dramatically show the division between the active few on the violent front and the supporting mass"; violence "is carried out by only a small proportion of persons present"; those competent at it are "a relatively small group (**I have estimated on the order of 15% or less in most conflict-engaged sub-populations**)". | His photograph collections; Marshall. | **His estimate**, stated as such. Not a measurement. | High that he claims it; low as a number |
| B5 | Spiller RJ. "S.L.A. Marshall and the ratio of fire." *RUSI Journal*, 1988. Search excerpt of a hosted copy (secondary). Chambers 2003 (*Parameters*) not found. | Criticism of Marshall's ratio of fire (the figure is given as 15% or "fewer than 25%" in the excerpts). | Marshall's notes contain no record of asking soldiers whether they fired; his claim of about 400 companies interviewed is challenged; colleagues' testimony. | The ratio of fire should be treated as **unmeasured**. | Medium |
| B6 | Armstrong P. Review, *Canadian Journal of Sociology* 33(4): 982-985 (2008). Primary. | Criticism: the theory is ahistorical; tension and fear are treated as "transcendental"; micro and macro cannot be cleanly separated; biologists will object. | -- | -- | High |
| B7 | Wieviorka, Kalyvas and Magaudda in *Sociologica* 2/2011. Seen only through Collins's reply (secondary). | Wieviorka: tension and fear vary greatly between individuals; meaning and macro-history matter more. Collins answers that it "is by far the most common pattern" wherever there is detailed evidence. | -- | -- | Medium |
| B8 | Two papers on CCTV footage of robberies (PubMed 31044635; PMC8276331). Search excerpts only; authors not captured. | Empirical test: no support for Collins's link between offender dominance and violence; "the absence, not the presence" of dominance went with violence; they ask for more weight on instrumental motive and premeditation. | Coded video. | **Measured**, but robbery, not group combat. | Low (excerpts) |

Not found: the 2009 *British Journal of Sociology* symposium (Felson, Cooney and others are from
memory); Mazur's hormonal critique (from memory). From memory and not verified: chapter 2 builds on
Marshall and adds Collins's own counts from combat photographs; chapter 3 reads the lopsided losses of
ancient battles as forward panic after one side breaks; chapter 10 extends the "few" to fighter aces,
snipers and police. I give no numbers for these.

**What is measured in Collins.** Nothing that bears directly on bouts. The one number he offers (15% or
less) is his estimate, and its main quantitative ancestor (Marshall) is discredited as a measurement.
What survives is a consistent qualitative pattern across unlike settings, which the ethnography in A
supports independently: little of a confrontation is fighting, a few do most of it, and killing follows
collapse.

### C. Harm per exchange between armed men who are both trying

| # | Source | What was measured | Numbers | Implication | Confidence |
|---|---|---|---|---|---|
| C1 | Gelli J. "Il duello in Italia nell'ultimo ventennio 1879-1899. Note statistiche." *Nuova Antologia*, 1 Jan 1901. **Not seen.** Figures from a history-forum answer (search excerpt; the page itself returned an error) and an Italian article (search excerpt). | Italian duels, mostly sabre | **3,918 duels, 20 deaths, 5,090 wounds, 1,475 of them judged serious.** A second excerpt gives 2,759 duels in 1879-1889 and 1,155 in the next decade (sum 3,914). An 1897 American newspaper (search excerpt) gives 2,759 duels and **50** deaths in ten years, about 2,400 with sabres and 179 with pistols. The death counts conflict. | Per duel: 1.30 wounds, at most 0.38 serious wounds, 0.005 deaths (0.018 on the newspaper's count). 29% of wounds serious. **No count of passes.** | Low-medium (twice removed; internal conflict) |
| C2 | French duels, late nineteenth century. Search excerpts only (a forum answer; a school handout citing Tarde). Nye 1993 and Ferreus's *Annuaire du duel* not seen. | Fatality per duel | "Under 2 percent"; "2 percent killed and 10 percent seriously wounded" for political and press duels; Tarde: one death per 30 duels. | Same order as Italy. No count of reprises. | Low |
| C3 | Tarrago R et al. "Temporal demands of elite fencing." *PLoS ONE* 2023, doi:10.1371/journal.pone.0285033. Search excerpt of the PMC record (secondary). Two earlier epee studies, excerpts only. | Timing of elite epee, 2014 World Championships | **Mean action ("Allez" to "Halte") 17.9 +/- 3.1 s; rest 15.5 +/- 5.5 s; work to rest 1:0.9** (foil 1:2.6, sabre 1:9.2). Earlier study, 29 bouts: work 164, 160 and 119 s in the three periods. Another: work 44.3%, rest 55.7%, action 17.7 s. | **An exchange between two men who must attack lasts about 18 s, and even with a referee restarting them, fighting fills under half the time.** Touches per action not found. | Medium |
| C4 | Bledsoe GH, Li G, Levy F. "Injury risk in professional boxing." *Southern Medical Journal* (year and pages not checked). Abstract via search excerpt (secondary). | Nevada professional bouts, 2001-2003 | **17.1 injuries per 100 boxer-matches; 3.4 per 100 boxer-rounds** (men 3.6, women 1.2). Knockdowns per round: not found. | Sanity anchor: about 0.34% per man per 18 s of continuous unarmoured fighting, for any recorded injury. | Medium |
| C5 | Rosso et al., *BMJ Open Sport and Exercise Medicine* 2023 (karate). Search excerpt. | European and Swiss competitions | 228 injuries in 2,404 fights; 47.4 per 1,000 athlete-exposures; 22.5 per 1,000 minutes of exposure. | About 0.7% per man per 18 s, any injury, mostly minor. Same order as C4. | Low-medium |
| C6 | Mensur | -- | Nothing quantitative found. Popular pages say 20-30 bouts per student (anecdote). | -- | Nothing found |
| C7 | Buhurt, HMB, IMCF, SCA, HEMA | -- | **Nothing found** in three web searches and one PubMed query (seven hits, none relevant). One unopened lead: a 2016 questionnaire study of longsword injuries (Wauters and Van Tiggelen), existence not confirmed. | The one source type that would measure armoured men per bout is missing. | Nothing found |
| C8 | Stick fighting (Nguni, Surma, others); riot police shield lines | -- | Nothing quantitative found (one web search; the PubMed query covered both). | -- | Nothing found |

## 2. The inferential gap, stated once

No source here observes drilled, armoured swordsmen held in a line. The ethnographic groups could choose
their distance and chose missiles. Duellists had no shield, no armour, no neighbours, and a doctor who
stopped the fight at first blood. Sportsmen have blunt weapons and a referee who forces the tempo.
Collins generalises from photographs and from firearms-era combat. Above all, **nobody counted
exchanges**: every per-exchange figure below is a per-event count divided by a number of exchanges that I
have had to assume.

## 3. Proposal

### (a) The lower end of P_d

**Line 1, the duel arithmetic.** P_d = (wounds per committed pass, unarmoured) x (share disabling) / F.

- Wounds per duel: 5,090 / 3,918 = 1.30 (C1).
- Passes per duel, N: **no data.** Assume 2 to 15. Wounds per pass = 0.09 to 0.65.
- Share disabling: 1,475 / 5,090 = 0.29 (Gelli's "serious", as reported).
- Unarmoured, unshielded: 0.025 to 0.19 per pass.
- F, the factor for a large shield, helmet and body armour: **no data.** Assume 5 to 20. The only supports
  are that a body-length shield let the Maring duel for weeks (A3), and the other half's finding that
  mail defeats most cuts and thrusts.
- Result: 0.025 / 20 = **0.0013** at the low extreme; 0.19 / 5 = 0.038 at the high; with N = 6 and
  F = 10, 1.30 / 6 x 0.29 / 10 = **0.006**.

**Line 2, the ethnographic floor.** The heaviest Maring case: 20 deaths among an assumed 180 warriors
(11.1%) over the fight-ground phase (A3, A4). Assume 20 to 60 fighting days ("weeks and even months",
not daily): 0.19% to 0.56% killed per warrior per fighting day. Assume 10 to 50 committed exchanges per
man per day (the Dani had 10-20 clashes a day): 4 x 10^-5 to 6 x 10^-4 deaths per man per exchange.
Assume 3 to 10 disabling wounds per death (the Mae Enga had 10-30 wounds per death, more than half of
them minor). For the pair: 2 x (3 to 10) x that = **0.0002 to 0.011**, geometric centre about 0.002.
Three assumed quantities; it shows only what shielded men who are free not to press can sustain.

**Line 3, the casualty clock from the other half, read with the tempo evidence below.** If a facing pair
spends at most 25% of its time in bouts (60 bouts an hour), the clock gives P_d of at least 0.011. This
points the other way. I give it least weight as a floor, because it assumes that the standing fight
itself must supply one disabling hit per front position every 1.5-3 hours, while both halves of this
search find that most killing follows the break.

**Answer.** 0.005 is about right, leaning slightly high for an outer bound. It sits inside both
independent ranges (0.0013-0.038 and 0.0002-0.011) and close to the centre of the duel line, so nothing
here argues for raising it. Both lines extend below it by a factor of four or more. I propose an **outer
lower bound of 0.002**, with 0.005 to 0.01 as the lower edge of the plausible band. The evidence cannot
distinguish 0.002 from 0.005.

### (b) Tempo

- **Length of a flurry:** 18 s (epee, measured, C3); "a few seconds" (Collins, inference). A bout of a
  few seconds to twenty is supported.
- **Share of time fighting:** Dani fronts were active for under half of the day ("brief clashes and
  relatively long interruptions"), and a man was at the very front for a small part of that, so a few
  percent of his day (A1-A2, my arithmetic). Refereed epee, where both must fight, reaches 44-53% (C3). I propose **5-25% of time at
  the front in committed bouts, with 50% as a ceiling reached only under compulsion.**
- **Rate:** the epee ceiling is one action per 33 s, 1.8 a minute, held for about three minutes at a
  time. At 15 s per bout, 5-25% is **0.2 to 1 bout per front man per minute**, or 12-60 per pair per
  hour. That selects the upper two rows of the other half's table (P_d of about 0.01-0.07) and makes its
  third row (120 an hour) the sport ceiling.
- **Shape:** clashes of minutes separated by longer lulls, about 10-20 a day among the Dani.

### (c) Who steps forward

At any moment about 6-12% of the Dani engaged were on the actual front (a dozen of 100-200; A1-A2, two
sources combined by me). Collins estimates 15% or less (B4). Marshall's 15-25% is not a measurement (B5). Among the Dani the rest rotated in by choice, and the
forward men were mostly aged 18-30. For a drilled front rank, where position is fixed, I propose that
**10-25% of front-rank men initiate most committed bouts and the rest mainly answer.** That step, from
free skirmishers to a fixed rank, is my inference; no source measures it.

## 4. Weakest link

The number of committed passes per duel (N) and the shield-and-armour factor (F). Neither has any source.
The duel line is one twice-removed statistic divided by two assumptions, and its low end moves in
proportion to each. The ethnographic line is weaker still (three assumptions) and is offered only because
it is independent and lands in the same place. A count of reprises in recorded sharp duels, or injury
rates per bout from armoured full-contact fighting, would replace an assumption with a measurement; I
found neither.

## 5. What was fetched, and what was not found

**Fetched and read (35 requests in all):**
- Internet Archive full texts (OCR): Heider 1970; Gardner and Heider 1969; Vayda 1976; Keeley 1996
  (nine requests including catalogue lookups).
- Princeton University Press sample chapter of Collins 2008 (ch. 1) (one request).
- *Sociologica* 2/2011, Collins's reply (doi:10.2383/35867) (one).
- *Canadian Journal of Sociology* 33(4), Armstrong's review (one).
- PubMed E-utilities: one search and one abstract fetch (seven hits, none relevant).
- Eighteen web searches. Results used as search excerpts are marked as such in the tables.
- Three failed page requests: Roscoe 2011 (twice) and the forum page with the Gelli figures.

**Looked for and not found or not opened:**
- Roscoe 2011 (link served a different page, twice).
- Meggitt 1977, Matthiessen 1962, Hart and Pilling 1960, Warner 1937, Rappaport 1967 (catalogued as
  lending-only or not searched for after the budget ran down); Chagnon on club fights and chest-pounding
  duels; Turney-High; Gat.
- Collins 2008 chapters 2, 3 and 10; the 2009 *British Journal of Sociology* symposium; Chambers 2003 on
  Marshall; Griffith, Keegan, Holmes and du Picq themselves.
- Gelli 1901, Hughes 2007, Nye 1993, Ferreus's *Annuaire du duel*, Tarde: none opened. The forum page
  carrying the Gelli figures returned an error; I have them only as search excerpts.
- The number of passes or reprises in real duels: nothing.
- Mensur injury statistics: nothing.
- Buhurt, HMB, IMCF, SCA or HEMA injury rates: nothing (four attempts).
- Stick-fighting injury rates: nothing.
- Riot police injuries per hour of confrontation: nothing (covered only by the PubMed query).
- Boxing knockdowns per round and punch counts: nothing; only the injury rate per round.
- Touches per action in epee: nothing.
