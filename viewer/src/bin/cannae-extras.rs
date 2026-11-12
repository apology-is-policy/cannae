//! The counts the scorer does not print (docs/slice0/report-outline.md, sections 1 to 4).
//!
//!     cannae-extras fails --records DIR [--freeze-seed F] [--seeds 10]
//!     cannae-extras rungs --records DIR [--freeze-seed F] [--seeds 10]
//!     cannae-extras two --first DIR --records DIR [--freeze-seed F] [--seeds 10] [--equal-only]
//!     cannae-extras topup --records DIR [--freeze-seed F] [--seeds 10]
//!
//! - `fails`: for each cell, on `base`, the failing sets by the part of the band that failed,
//!   each pooled value beside its band, the stragglers and the tempo.
//! - `rungs`: for each engine, one rung's memory statistic against another's, set by set.
//! - `two`: the field engine's first batch beside the new one, pair by pair.
//! - `topup`: one figure, the machine time the relief's top-up would take over the eight cells.
//!
//! It reads records and runs nothing. It prints outcomes of scored runs, so it is run after
//! the scoring. There are two exceptions, both run before it. `two --equal-only` prints how
//! many pairs of records are equal and nothing they hold. `topup` prints its one total and
//! names no cell, in a figure or in a refusal: Michal decides the top-up on the machine time
//! alone, and how many sets a cell would add is an outcome of `base`.
//!
//! It lives outside the scored code, like the viewer, and takes its verdicts from the scorer's
//! own functions. `fails` computes each verdict again from its parts and stops if one differs
//! from the scorer's.

use cannae_core::schedule::BASE_TICK;
use cannae_harness::metrics::{Bands, RunSummary, fortune, verdicts};
use cannae_score::draw::{Cell, drawn};
use cannae_score::report::{
    Cellbook, KNIFE_EDGE, RELIEF_EVERY, ROMANS, TOPUP_AT_S, TOPUP_MAX_SETS, TOPUP_MIN_SEEDS,
};
use cannae_score::{Engine, Record, record_file};
use std::collections::BTreeMap;
use std::path::Path;

/// The freeze seed of the scored batch.
const BATCH: u64 = 9_464_880_715_404_986_219;

const ENGINES: [Engine; 2] = [Engine::Pairwise, Engine::Field];

const PATTERNS: [&str; 8] = [
    "1 standoff",
    "2 pulses and lulls",
    "3 rear-first collapse",
    "4 lateral spread",
    "5 casualty asymmetry",
    "6 holds, then breaks (band only)",
    "7 casualty clock",
    "8 centurion gradient",
];

/// The only figures there are for tempo (preregistration, pattern 7): no band.
const TEMPO_FORWARD: [f64; 2] = [0.05, 0.25];
const TEMPO_STARTERS: [f64; 2] = [0.10, 0.25];

/// "The four minutes between the shock and the front" (preregistration, the memory variants),
/// in base ticks, against the ticks one level of *shaken* takes to wear off.
const FOUR_MINUTES: u32 = 4 * 60 * 1000 / BASE_TICK.0;

/// The pairs of rungs compared: each against rung 0, and each against the one below it.
const RUNG_PAIRS: [(u8, u8); 5] = [(1, 0), (2, 0), (3, 0), (2, 1), (3, 2)];

/// One seed's records on one variant.
#[derive(Default)]
struct Seed {
    stopped: Option<Record>,
    whole: Option<Record>,
}

impl Seed {
    /// The record the scorer scores: the stopped run's, or else the whole run's.
    fn scored(&self) -> Option<&Record> {
        self.stopped.as_ref().or(self.whole.as_ref())
    }
}

/// set -> seed.
type Sets = BTreeMap<u32, BTreeMap<u64, Seed>>;

/// One file of records, and the lines left out of it: unreadable, of another cell, variant or
/// freeze seed.
struct Read {
    sets: Sets,
    left_out: usize,
}

fn read(dir: &str, cell: Cell, variant: &str, freeze: u64) -> Result<Option<Read>, String> {
    let path = Path::new(dir).join(record_file(cell, variant));
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    let mut got = Read {
        sets: Sets::new(),
        left_out: 0,
    };
    for l in text.lines().filter(|l| !l.trim().is_empty()) {
        let Ok(r) = serde_json::from_str::<Record>(l) else {
            got.left_out += 1;
            continue;
        };
        if r.engine != cell.engine
            || r.rung != cell.rung
            || r.variant != variant
            || r.freeze_seed != Some(freeze)
        {
            got.left_out += 1;
            continue;
        }
        let seed = got
            .sets
            .entry(r.draw)
            .or_default()
            .entry(r.seed)
            .or_default();
        // A second record of the same run takes the first one's place, as in the scorer.
        if r.whole {
            seed.whole = Some(r);
        } else {
            seed.stopped = Some(r);
        }
    }
    Ok(Some(got))
}

/// The records each set is scored on, for the sets with every seed there: the scorer's rule.
fn complete(sets: &Sets, seeds: usize) -> BTreeMap<u32, Vec<&Record>> {
    sets.iter()
        .filter_map(|(d, s)| {
            let runs: Vec<&Record> = s.values().filter_map(Seed::scored).collect();
            (runs.len() == seeds).then_some((*d, runs))
        })
        .collect()
}

fn summaries(runs: &[&Record]) -> Vec<RunSummary> {
    runs.iter().map(|r| r.summary.clone()).collect()
}

/// The median as the scorer takes it (`cannae_harness::metrics`).
fn median(mut v: Vec<f64>) -> Option<f64> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let m = v.len() / 2;
    Some(if v.len() % 2 == 1 {
        v[m]
    } else {
        (v[m - 1] + v[m]) / 2.0
    })
}

/// The quartiles by nearest rank, as the scorer's report takes them.
fn quartiles(mut v: Vec<f64>) -> Option<[f64; 3]> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let at = |q: f64| v[((v.len() - 1) as f64 * q).round() as usize];
    Some([at(0.25), at(0.5), at(0.75)])
}

fn num(x: f64) -> String {
    if !x.is_finite() {
        "never".to_string()
    } else if x.abs() >= 100.0 {
        format!("{x:.0}")
    } else if x.abs() >= 1.0 {
        format!("{x:.2}")
    } else {
        format!("{x:.3}")
    }
}

fn spread(v: Vec<f64>, of: &str) -> String {
    let n = v.len();
    quartiles(v).map_or(format!("none ({n} {of})"), |[a, m, b]| {
        format!(
            "median {} (quartiles {} to {}, {n} {of})",
            num(m),
            num(a),
            num(b)
        )
    })
}

fn name(cell: Cell) -> String {
    format!("{:?} rung {}", cell.engine, cell.rung)
}

#[derive(Clone, Copy)]
enum Band {
    Min(f64),
    Within([f64; 2]),
}

impl Band {
    fn text(self) -> String {
        match self {
            Band::Min(m) => format!("at least {m}"),
            Band::Within([a, b]) => format!("{a} to {b}"),
        }
    }
}

/// A value a run gives to a pattern, pooled over a set's seeds by its median.
struct Row {
    /// Index of its pattern, 0 to 7.
    pattern: usize,
    name: &'static str,
    values: fn(&RunSummary) -> Vec<f64>,
    band: Band,
}

fn one(x: Option<f64>) -> Vec<f64> {
    x.into_iter().collect()
}

/// The banded values in the scorer's order (`verdicts`). Within a pattern, the scorer needs
/// every one of them to score it, but for the harm per bout, which fails pattern 7 if no seed
/// gives it.
fn rows(b: &Bands) -> Vec<Row> {
    let mut rows = vec![
        Row {
            pattern: 0,
            name: "formed phase, s",
            values: |r| vec![r.formed_s],
            band: Band::Min(b.standoff_min_formed_s),
        },
        Row {
            pattern: 0,
            name: "median gap while formed, m",
            values: |r| one(r.median_gap_m),
            band: Band::Min(b.standoff_min_median_gap_m),
        },
        Row {
            pattern: 1,
            name: "burstiness",
            values: |r| one(r.burstiness),
            band: Band::Min(b.pulses_min_burstiness),
        },
        Row {
            pattern: 1,
            name: "share of quiet bins",
            values: |r| one(r.lull_share),
            band: Band::Within(b.pulses_lull_share),
        },
        Row {
            pattern: 2,
            name: "depth of the first to turn",
            values: |r| one(r.first_turn_depth),
            band: Band::Min(b.rear_first_min_depth),
        },
        Row {
            pattern: 3,
            name: "spread, Spearman's rho",
            values: |r| one(r.spread_rho),
            band: Band::Min(b.spread_min_rho),
        },
        Row {
            pattern: 3,
            name: "spread, s",
            values: |r| one(r.spread_s),
            band: Band::Within(b.spread_s),
        },
        Row {
            pattern: 4,
            name: "loser's share of hits after the break",
            values: |r| one(r.loser_after_break),
            band: Band::Min(b.asymmetry_min_after_break),
        },
        Row {
            pattern: 6,
            name: "hits per front file per hour",
            values: |r| r.hits_per_file_hour.iter().flatten().copied().collect(),
            band: Band::Within(b.clock_hits_per_file_hour),
        },
    ];
    if let Some(band) = b.clock_lethality_per_bout {
        rows.push(Row {
            pattern: 6,
            name: "harm per bout",
            values: |r| one(r.lethality_per_bout),
            band: Band::Within(band),
        });
    }
    rows
}

fn pooled(row: &Row, runs: &[RunSummary]) -> Option<f64> {
    median(runs.iter().flat_map(|r| (row.values)(r)).collect())
}

/// The shares of a set's seeds that break early and that never break (pattern 6).
fn holds(runs: &[RunSummary], b: &Bands) -> (f64, f64) {
    let n = runs.len() as f64;
    let early = runs
        .iter()
        .filter(|r| r.time_to_break_s.is_some_and(|t| t < b.holds_early_s))
        .count() as f64;
    let never = runs
        .iter()
        .filter(|r| r.time_to_break_s.is_none_or(|t| t > b.holds_never_s))
        .count() as f64;
    (early / n, never / n)
}

/// The leaders' share of the hits in the formed fight and in the rout (pattern 8), if both
/// phases have hits.
fn gradient(runs: &[RunSummary]) -> Option<(f64, f64)> {
    let (lf, f, lr, r) = runs.iter().fold((0, 0, 0, 0), |a, x| {
        (
            a.0 + x.leader_hits_formed,
            a.1 + x.hits_formed,
            a.2 + x.leader_hits_rout,
            a.3 + x.hits_rout,
        )
    });
    (f > 0 && r > 0).then(|| (lf as f64 / f as f64, lr as f64 / r as f64))
}

/// A pattern's parts for one set, each with whether the set fails on it; or, if the pattern
/// cannot be scored for the set, what it lacks.
type Parts = Result<Vec<(String, bool)>, Vec<String>>;

fn judge(runs: &[RunSummary], b: &Bands, rows: &[Row]) -> [Parts; 8] {
    std::array::from_fn(|p| match p {
        5 if runs.is_empty() => Err(vec!["no run".to_string()]),
        5 => {
            let (early, never) = holds(runs, b);
            Ok(vec![
                (
                    "too many seeds break early".to_string(),
                    early > b.holds_max_early,
                ),
                (
                    "too many seeds never break".to_string(),
                    never > b.holds_max_never,
                ),
            ])
        }
        7 => match gradient(runs) {
            Some((formed, rout)) => Ok(vec![(
                "leaders' share of hits in the formed fight not above their share in the rout"
                    .to_string(),
                formed <= rout,
            )]),
            None => {
                let mut lacks = Vec::new();
                if runs.iter().all(|r| r.hits_formed == 0) {
                    lacks.push("no hit in the formed fight".to_string());
                }
                if runs.iter().all(|r| r.hits_rout == 0) {
                    lacks.push("no hit in the rout".to_string());
                }
                Err(lacks)
            }
        },
        _ => {
            let mut list = Vec::new();
            let mut lacks = Vec::new();
            for (k, row) in rows.iter().filter(|r| r.pattern == p).enumerate() {
                let x = pooled(row, runs);
                // Pattern 7 is scored whenever the clock has a value; a set with no harm per
                // bout then fails on it.
                let optional = p == 6 && k > 0;
                if x.is_none() && !optional {
                    lacks.push(format!("no seed gives the {}", row.name));
                }
                match row.band {
                    Band::Min(m) => list.push((
                        format!("{} under its minimum", row.name),
                        x.is_some_and(|x| x < m),
                    )),
                    Band::Within([lo, hi]) => {
                        list.push((
                            format!("{} under its band", row.name),
                            x.is_some_and(|x| x < lo),
                        ));
                        list.push((
                            format!("{} over its band", row.name),
                            x.is_some_and(|x| x > hi),
                        ));
                    }
                }
                if optional {
                    list.push((format!("no seed gives the {}", row.name), x.is_none()));
                }
            }
            if lacks.is_empty() {
                Ok(list)
            } else {
                Err(lacks)
            }
        }
    })
}

fn passes(parts: &Parts) -> Option<bool> {
    parts
        .as_ref()
        .ok()
        .map(|list| list.iter().all(|(_, failed)| !failed))
}

/// The smallest count of n fair coins that comes up, or more, no more than once in forty
/// times: the upper end of the two-sided 95 % range. `None` if n is too small to have one.
fn coin_line(n: usize) -> Option<usize> {
    // P(X = n) = 2^-n, then down from there.
    let mut pmf = 0.5f64.powi(n as i32);
    let mut tail = 0.0;
    let mut line = None;
    for k in (0..=n).rev() {
        tail += pmf;
        if tail > 0.025 {
            break;
        }
        line = Some(k);
        pmf *= k as f64 / (n - k + 1) as f64;
    }
    line
}

struct Asked {
    dir: String,
    first: Option<String>,
    freeze: u64,
    seeds: usize,
    equal_only: bool,
}

fn asked(mut args: impl Iterator<Item = String>) -> Result<Asked, String> {
    let (mut dir, mut first, mut freeze, mut seeds, mut equal_only) =
        (None, None, BATCH, 10usize, false);
    while let Some(a) = args.next() {
        let mut value = || args.next().ok_or_else(|| format!("{a} needs a value"));
        match a.as_str() {
            "--records" => dir = Some(value()?),
            "--first" => first = Some(value()?),
            "--freeze-seed" => {
                freeze = value()?
                    .parse()
                    .map_err(|e| format!("--freeze-seed: {e}"))?;
            }
            "--seeds" => seeds = value()?.parse().map_err(|e| format!("--seeds: {e}"))?,
            "--equal-only" => equal_only = true,
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    Ok(Asked {
        dir: dir.ok_or("needs --records")?,
        first,
        freeze,
        seeds,
        equal_only,
    })
}

fn bands() -> Result<Bands, String> {
    Bands::from_toml(include_str!("../../../scenarios/prereg.toml")).map_err(|e| e.to_string())
}

/// The scorer's own verdicts for a cell, by set. Stops if the folder holds records of another
/// freeze seed, which the scorer would have scored with the rest.
fn scorer(a: &Asked, cell: Cell, b: &Bands) -> Result<cannae_score::report::Report, String> {
    let rep = Cellbook::read(Path::new(&a.dir), cell, a.seeds)?.score(b);
    if rep.freeze_seeds.iter().any(|f| *f != Some(a.freeze)) {
        return Err(format!(
            "{}: records of another freeze seed than {} in {}",
            name(cell),
            a.freeze,
            a.dir
        ));
    }
    Ok(rep)
}

fn fails(a: &Asked) -> Result<(), String> {
    let b = bands()?;
    let rows = rows(&b);
    for engine in ENGINES {
        for rung in 0..4u8 {
            let cell = Cell { engine, rung };
            let Some(got) = read(&a.dir, cell, "base", a.freeze)? else {
                println!("{}: no file of base\n", name(cell));
                continue;
            };
            let sets = complete(&got.sets, a.seeds);
            let theirs = scorer(a, cell, &b)?;
            if theirs.sets.len() != sets.len() {
                return Err(format!(
                    "{}: {} sets here and {} in the scorer",
                    name(cell),
                    sets.len(),
                    theirs.sets.len()
                ));
            }
            let mut judged = Vec::new();
            for (s, (d, runs)) in theirs.sets.iter().zip(&sets) {
                let runs = summaries(runs);
                let parts = judge(&runs, &b, &rows);
                let mine: Vec<Option<bool>> = parts.iter().map(passes).collect();
                if s.draw != *d || mine != s.verdicts || mine != verdicts(&runs, &b) {
                    return Err(format!(
                        "{}, set {d}: the parts give another verdict than the scorer",
                        name(cell)
                    ));
                }
                judged.push((runs, parts));
            }
            println!(
                "{}, base: {} sets of {} seeds; {} lines left out; every verdict as the scorer's",
                name(cell),
                sets.len(),
                a.seeds,
                got.left_out
            );
            for (p, pattern) in PATTERNS.iter().enumerate() {
                let count = |want: Option<bool>| {
                    judged
                        .iter()
                        .filter(|(_, parts)| passes(&parts[p]) == want)
                        .count()
                };
                println!(
                    "  {pattern}: {} pass, {} fail, {} cannot be scored",
                    count(Some(true)),
                    count(Some(false)),
                    count(None)
                );
                let mut tally: Vec<(String, usize)> = Vec::new();
                for list in judged
                    .iter()
                    .filter_map(|(_, parts)| parts[p].as_ref().ok())
                {
                    if tally.is_empty() {
                        tally = list.iter().map(|(n, _)| (n.clone(), 0)).collect();
                    }
                    for (t, (_, failed)) in tally.iter_mut().zip(list) {
                        t.1 += usize::from(*failed);
                    }
                }
                for (part, n) in &tally {
                    println!("      fail on: {part}: {n}");
                }
                let mut lacks: BTreeMap<&str, usize> = BTreeMap::new();
                for l in judged
                    .iter()
                    .filter_map(|(_, parts)| parts[p].as_ref().err())
                    .flatten()
                {
                    *lacks.entry(l.as_str()).or_default() += 1;
                }
                for (what, n) in &lacks {
                    println!("      cannot be scored: {what}: {n}");
                }
                for row in rows.iter().filter(|r| r.pattern == p) {
                    let v = judged.iter().filter_map(|(r, _)| pooled(row, r)).collect();
                    println!(
                        "      {}: {}; band {}",
                        row.name,
                        spread(v, "sets with a value"),
                        row.band.text()
                    );
                }
                if p == 5 {
                    let (early, never): (Vec<f64>, Vec<f64>) =
                        judged.iter().map(|(r, _)| holds(r, &b)).unzip();
                    println!(
                        "      share of seeds breaking within {} s: {}; at most {}",
                        b.holds_early_s,
                        spread(early, "sets"),
                        b.holds_max_early
                    );
                    println!(
                        "      share of seeds unbroken after {} s: {}; at most {}",
                        b.holds_never_s,
                        spread(never, "sets"),
                        b.holds_max_never
                    );
                }
                if p == 7 {
                    let (formed, rout): (Vec<f64>, Vec<f64>) =
                        judged.iter().filter_map(|(r, _)| gradient(r)).unzip();
                    println!(
                        "      leaders' share of hits, formed fight: {}",
                        spread(formed, "sets with a value")
                    );
                    println!(
                        "      leaders' share of hits, rout: {}",
                        spread(rout, "sets with a value")
                    );
                }
            }
            let broke: Vec<f64> = judged
                .iter()
                .flat_map(|(r, _)| r.iter())
                .filter(|r| r.loser.is_some())
                .map(|r| f64::from(r.stragglers))
                .collect();
            let with = broke.iter().filter(|&&s| s >= 1.0).count();
            println!(
                "  Stragglers (the loser's men who turned before the collapse), over the {} runs in \
                 which a side broke: one or more in {with} ({:.1} %); {}; the most {}",
                broke.len(),
                100.0 * with as f64 / broke.len().max(1) as f64,
                spread(broke.clone(), "runs"),
                broke.iter().copied().fold(0.0, f64::max)
            );
            type Share = fn(&RunSummary) -> Option<f64>;
            let tempo: [(&str, Share, [f64; 2]); 2] = [
                (
                    "front-rank man-time spent forward",
                    |r| r.front_forward_share,
                    TEMPO_FORWARD,
                ),
                (
                    "front-rank men who start any bout",
                    |r| r.front_starters_share,
                    TEMPO_STARTERS,
                ),
            ];
            for (what, of, [lo, hi]) in tempo {
                let v: Vec<f64> = judged
                    .iter()
                    .filter_map(|(r, _)| median(r.iter().filter_map(of).collect()))
                    .collect();
                let inside = v.iter().filter(|&&x| x >= lo && x <= hi).count();
                println!(
                    "  Tempo, share of {what}: {}; sets within {lo} to {hi} (no band): {inside}",
                    spread(v, "sets with a value")
                );
            }
            println!();
        }
    }
    Ok(())
}

/// The two-sided sign line over the sets not tied, in words: whether the count is past it,
/// and which way.
fn sign_line(larger: usize, smaller: usize, more: &str, less: &str) -> String {
    let n = larger + smaller;
    match coin_line(n) {
        None => format!("cannot be told with {n} sets not tied"),
        Some(line) => {
            let said = if larger >= line {
                format!("PAST IT: {more}")
            } else if smaller >= line {
                format!("PAST IT THE OTHER WAY: {less}")
            } else {
                "not past it".to_string()
            };
            format!("the two-sided line is {line} of {n}: {said}")
        }
    }
}

/// One comparison of two rungs over the sets both have. For each set: the higher rung's
/// statistic less the lower one's, and whether the set is one of those counted apart among the
/// ties, which `apart` names.
fn against(what: &str, sets: &[(f64, bool)], apart: &str) {
    let larger = sets.iter().filter(|s| s.0 > 0.0).count();
    let smaller = sets.iter().filter(|s| s.0 < 0.0).count();
    let same = sets.len() - larger - smaller;
    let counted = sets.iter().filter(|s| s.1).count();
    println!(
        "    {what}: larger in {larger}, smaller in {smaller}, the same in {same} of {} sets \
         ({counted} of them {apart}); median difference {}; {}",
        sets.len(),
        median(sets.iter().map(|s| s.0).collect())
            .map_or("none".to_string(), |m| format!("{m:+.3}")),
        sign_line(
            larger,
            smaller,
            "the higher rung shows more",
            "the higher rung shows less"
        )
    );
}

/// set -> seed -> how the Roman line fared: the value the statistics compare.
type Fortunes = BTreeMap<u32, BTreeMap<u64, (u8, f64)>>;

fn fortunes(a: &Asked, cell: Cell, variant: &str) -> Result<Fortunes, String> {
    let Some(got) = read(&a.dir, cell, variant, a.freeze)? else {
        return Ok(Fortunes::new());
    };
    Ok(got
        .sets
        .iter()
        .map(|(d, seeds)| {
            let fared = seeds
                .iter()
                .filter_map(|(s, x)| Some((*s, fortune(&x.scored()?.summary, ROMANS))))
                .collect();
            (*d, fared)
        })
        .collect())
}

/// For one cell, each set of `relief` with whether a relief was ordered in any of its runs.
fn relieved(a: &Asked, cell: Cell) -> Result<BTreeMap<u32, bool>, String> {
    let Some(got) = read(&a.dir, cell, "relief", a.freeze)? else {
        return Ok(BTreeMap::new());
    };
    Ok(got
        .sets
        .iter()
        .map(|(d, seeds)| {
            let ordered = seeds
                .values()
                .filter_map(Seed::scored)
                .any(|r| r.relieved_s.iter().any(Option::is_some));
            (*d, ordered)
        })
        .collect())
}

/// What is counted apart among the ties of the memory statistics.
const ONE_RUN: &str = "the same run for run at both rungs";

/// The statistics compared between rungs, each with the two variants it is taken over.
const COMPARED: [(&str, [&str; 2]); 3] = [
    ("shock", ["base", "shock"]),
    ("volleys", ["base", "volleys"]),
    ("the drill", ["backed", "relief"]),
];

fn rungs(a: &Asked) -> Result<(), String> {
    let b = bands()?;
    for engine in ENGINES {
        // rung -> set -> shock, volleys, the drill.
        let mut stats: Vec<BTreeMap<u32, [Option<f64>; 3]>> = Vec::new();
        // rung -> variant -> every run's fortune.
        let mut fared: Vec<BTreeMap<&str, Fortunes>> = Vec::new();
        for rung in 0..4u8 {
            let cell = Cell { engine, rung };
            let rep = scorer(a, cell, &b)?;
            stats.push(
                rep.sets
                    .iter()
                    .map(|s| (s.draw, [s.shock, s.volleys, s.drill]))
                    .collect(),
            );
            let mut by_variant = BTreeMap::new();
            for v in ["base", "shock", "volleys", "backed", "relief"] {
                by_variant.insert(v, fortunes(a, cell, v)?);
            }
            fared.push(by_variant);
        }
        if stats.iter().all(BTreeMap::is_empty) {
            println!("{engine:?}: no set scored at any rung\n");
            continue;
        }
        println!(
            "{engine:?}: sets scored at rungs 0 to 3: {}, {}, {}, {}",
            stats[0].len(),
            stats[1].len(),
            stats[2].len(),
            stats[3].len()
        );
        // The sets both rungs have a statistic for: its difference, and whether every run of
        // the two variants fared the same at both rungs.
        let diffs = |k: usize, hi: u8, lo: u8| -> Vec<(u32, f64, bool)> {
            let (hi, lo) = (hi as usize, lo as usize);
            stats[hi]
                .iter()
                .filter_map(|(d, h)| {
                    let l = stats[lo].get(d)?;
                    let one_run = COMPARED[k].1.iter().all(|v| {
                        let at = |rung: usize| fared[rung].get(v).and_then(|f| f.get(d));
                        at(hi).is_some() && at(hi) == at(lo)
                    });
                    Some((*d, h[k]? - l[k]?, one_run))
                })
                .collect()
        };
        let plain =
            |v: &[(u32, f64, bool)]| -> Vec<(f64, bool)> { v.iter().map(|x| (x.1, x.2)).collect() };
        for (k, (variant, _)) in COMPARED.iter().enumerate().take(2) {
            println!("  {variant}, P(the Roman line fares worse than in base):");
            for (hi, lo) in RUNG_PAIRS {
                let v = diffs(k, hi, lo);
                against(&format!("rung {hi} against rung {lo}"), &plain(&v), ONE_RUN);
                if (hi, lo) == (2, 1) {
                    let mut slow = Vec::new();
                    let mut fast = Vec::new();
                    for (d, x, one_run) in &v {
                        let wear = drawn(a.freeze, Cell { engine, rung: 2 }, *d)?
                            .man
                            .shaken_wear;
                        if wear > FOUR_MINUTES {
                            slow.push((*x, *one_run));
                        } else {
                            fast.push((*x, *one_run));
                        }
                    }
                    against(
                        "  the sets where one level of shaken takes more than four minutes to \
                         wear off",
                        &slow,
                        ONE_RUN,
                    );
                    against("  the other sets", &fast, ONE_RUN);
                }
            }
        }
        println!(
            "  the drill alone, P(the Roman line fares worse backed than relieved), on the sets \
             of the stride:"
        );
        // Where no relief was ordered, `relief` is `backed` run for run and the statistic is
        // 0.5 at both rungs: those ties are the ones that mean something here (Varro).
        let ordered = [
            relieved(a, Cell { engine, rung: 2 })?,
            relieved(a, Cell { engine, rung: 3 })?,
        ];
        let drill: Vec<(f64, bool)> = diffs(2, 3, 2)
            .iter()
            .map(|(d, x, _)| (*x, ordered.iter().all(|o| o.get(d) == Some(&false))))
            .collect();
        against(
            "rung 3 against rung 2",
            &drill,
            "with no relief ordered in any run at either rung",
        );
        println!();
    }
    Ok(())
}

/// A value a run gives to a pattern, one for a run (the list of cell-check.md, with the clock
/// once for each side).
type RunValue = (&'static str, fn(&RunSummary) -> Option<f64>);

const RUN_VALUES: [RunValue; 12] = [
    ("formed phase, s", |r| Some(r.formed_s)),
    ("median gap while formed, m", |r| r.median_gap_m),
    ("burstiness", |r| r.burstiness),
    ("share of quiet bins", |r| r.lull_share),
    ("depth of the first to turn", |r| r.first_turn_depth),
    ("spread, Spearman's rho", |r| r.spread_rho),
    ("spread, s", |r| r.spread_s),
    ("loser's share of hits after the break", |r| {
        r.loser_after_break
    }),
    // A side that never broke comes last.
    ("time to break, s (never: last)", |r| {
        Some(r.time_to_break_s.unwrap_or(f64::INFINITY))
    }),
    ("hits per front file per hour, side A (the Romans)", |r| {
        r.hits_per_file_hour[0]
    }),
    ("hits per front file per hour, side B (the warband)", |r| {
        r.hits_per_file_hour[1]
    }),
    ("harm per bout", |r| r.lethality_per_bout),
];

/// How many of the pairs have the larger value first, the smaller, the same, and no value on
/// one side or on both.
fn sides<T>(pairs: &[(T, T)], of: impl Fn(&T) -> Option<f64>) -> [usize; 4] {
    let mut c = [0; 4];
    for (o, n) in pairs {
        c[match (of(o), of(n)) {
            (Some(x), Some(y)) if x > y => 0,
            (Some(x), Some(y)) if x < y => 1,
            (Some(_), Some(_)) => 2,
            _ => 3,
        }] += 1;
    }
    c
}

/// Equal field for field, all but the wall time and the count of held frames.
fn equal_but(a: &Record, b: &Record) -> bool {
    let strip = |r: &Record| Record {
        wall_ms: 0,
        held: 0,
        ..r.clone()
    };
    strip(a) == strip(b)
}

/// The 95 % Wilson interval of k successes in n, as the scorer takes it (`report.rs`, where it
/// is private).
fn wilson(k: usize, n: usize) -> (f64, f64) {
    if n == 0 {
        return (0.0, 1.0);
    }
    let (p, n, z) = (k as f64 / n as f64, n as f64, 1.96f64);
    let mid = (p + z * z / (2.0 * n)) / (1.0 + z * z / n);
    let half = z * (p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt() / (1.0 + z * z / n);
    ((mid - half).max(0.0), (mid + half).min(1.0))
}

/// One batch's count of sets that pass all eight on `base` with pattern 6 by its band, with
/// its share, its interval and the knife-edge share beside it.
fn by_the_band(which: &str, k: usize, n: usize) -> String {
    let (lo, hi) = wilson(k, n);
    let share = k as f64 / n.max(1) as f64;
    format!(
        "all eight, pattern 6 by its band, {which}: {k} of {n} sets, {:.1} % (95 % interval \
         {:.1}-{:.1} %); at least {:.0} %: {}",
        100.0 * share,
        100.0 * lo,
        100.0 * hi,
        100.0 * KNIFE_EDGE,
        if n > 0 && share >= KNIFE_EDGE {
            "yes"
        } else {
            "no"
        }
    )
}

fn two(a: &Asked) -> Result<(), String> {
    let first = a.first.as_deref().ok_or("needs --first")?;
    let b = bands()?;
    let rows = rows(&b);
    for rung in 0..4u8 {
        let cell = Cell {
            engine: Engine::Field,
            rung,
        };
        let (Some(old), Some(new)) = (
            read(first, cell, "base", a.freeze)?,
            read(&a.dir, cell, "base", a.freeze)?,
        ) else {
            println!("{}: one of the two has no file of base\n", name(cell));
            continue;
        };
        // The first batch was written before the count existed, so none of its records has
        // one. A count there means that --first is not the first batch, and a batch beside
        // itself would give every pair as equal.
        let counted = |s: &Seed| {
            [&s.stopped, &s.whole]
                .into_iter()
                .flatten()
                .any(|r| r.held != 0)
        };
        if old.sets.values().flat_map(BTreeMap::values).any(counted) {
            return Err(format!(
                "{}: a record in {first} has a count of held frames, and the first batch has none",
                name(cell)
            ));
        }
        // 1. The pairs: a stopped run with a stopped run, a whole run with a whole one.
        let (mut pairs, mut only_first, mut only_new) = (0, 0, 0);
        let (mut zero, mut zero_equal, mut held, mut held_differ) = (0, 0, 0, 0);
        let empty = BTreeMap::new();
        for (d, seeds) in &old.sets {
            for (seed, o) in seeds {
                let n = new.sets.get(d).and_then(|m| m.get(seed));
                for (o, n) in [
                    (&o.stopped, n.and_then(|n| n.stopped.as_ref())),
                    (&o.whole, n.and_then(|n| n.whole.as_ref())),
                ] {
                    match (o, n) {
                        (Some(o), Some(n)) => {
                            pairs += 1;
                            if n.held == 0 {
                                zero += 1;
                                zero_equal += usize::from(equal_but(o, n));
                            } else {
                                held += 1;
                                held_differ += usize::from(o.summary != n.summary);
                            }
                        }
                        (Some(_), None) => only_first += 1,
                        _ => {}
                    }
                }
            }
        }
        for (d, seeds) in &new.sets {
            for (seed, n) in seeds {
                let o = old.sets.get(d).unwrap_or(&empty).get(seed);
                only_new +=
                    usize::from(n.stopped.is_some() && o.is_none_or(|o| o.stopped.is_none()))
                        + usize::from(n.whole.is_some() && o.is_none_or(|o| o.whole.is_none()));
            }
        }
        println!(
            "{}, base: {pairs} pairs; without a partner, {only_first} records of the first \
             batch and {only_new} of the new; lines left out, {} and {}",
            name(cell),
            old.left_out,
            new.left_out
        );
        println!(
            "  count of held frames zero: {zero} pairs, {zero_equal} equal but for the wall \
             time and the count, {} UNEQUAL",
            zero - zero_equal
        );
        println!("  count of held frames one or more: {held} pairs");
        if a.equal_only {
            println!();
            continue;
        }
        println!("    of those, {held_differ} differ in their summary");
        // 2. Where the defect could act, over the runs the scorer scores.
        let new_sets = complete(&new.sets, a.seeds);
        let old_sets = complete(&old.sets, a.seeds);
        fn acted(runs: &[&Record]) -> usize {
            runs.iter().filter(|r| r.held >= 1).count()
        }
        println!(
            "  the new batch's scored runs with a count of one or more: {} of {}, in {} of {} \
             sets",
            new_sets
                .values()
                .map(|r| acted(r.as_slice()))
                .sum::<usize>(),
            new_sets.values().map(Vec::len).sum::<usize>(),
            new_sets
                .values()
                .filter(|r| acted(r.as_slice()) > 0)
                .count(),
            new_sets.len()
        );
        let counts: Vec<f64> = new_sets
            .values()
            .flatten()
            .map(|r| f64::from(r.held))
            .collect();
        println!(
            "    the count over those scored runs: {}; the most {}",
            spread(counts.clone(), "runs"),
            counts.iter().copied().fold(0.0, f64::max)
        );
        // 3. The verdicts of the two batches, set by set, pattern 6 by its band.
        let mut both_have = 0;
        let mut table = [[0usize; 4]; 9];
        for (d, n) in &new_sets {
            let Some(o) = old_sets.get(d) else { continue };
            both_have += 1;
            let vo = verdicts(&summaries(o), &b);
            let vn = verdicts(&summaries(n), &b);
            let cell_of = |o: bool, n: bool| match (o, n) {
                (true, true) => 0,
                (true, false) => 1,
                (false, true) => 2,
                (false, false) => 3,
            };
            for p in 0..8 {
                table[p][cell_of(vo[p] == Some(true), vn[p] == Some(true))] += 1;
            }
            let all = |v: &[Option<bool>; 8]| v.iter().all(|x| *x == Some(true));
            table[8][cell_of(all(&vo), all(&vn))] += 1;
        }
        println!(
            "  verdicts over the {both_have} sets both batches have whole (pass in both, in \
             the first alone, in the new alone, in neither):"
        );
        for (p, row) in table.iter().enumerate() {
            let label = PATTERNS
                .get(p)
                .copied()
                .unwrap_or("all eight, pattern 6 by its band");
            println!(
                "    {label:<34} {:>5} {:>5} {:>5} {:>5}   {}",
                row[0],
                row[1],
                row[2],
                row[3],
                sign_line(
                    row[1],
                    row[2],
                    "more sets pass in the first batch alone",
                    "more sets pass in the new batch alone"
                )
            );
        }
        // Each batch's own count, taken the same way: all eight with pattern 6 by its band,
        // which is all the first batch can give. `score`'s line "All eight" needs the two
        // dose-response statistics as well, so for a batch with `base` alone it reads 0.
        for (which, k) in [
            ("the first batch", table[8][0] + table[8][1]),
            ("the new batch", table[8][0] + table[8][2]),
        ] {
            println!("    {}", by_the_band(which, k, both_have));
        }
        // 4. The runs the defect acted in: beside their own pairs, and beside the rest.
        let mut paired: Vec<(&RunSummary, &RunSummary)> = Vec::new();
        let (mut with, mut rest): (Vec<&RunSummary>, Vec<&RunSummary>) = (Vec::new(), Vec::new());
        for (d, seeds) in &new.sets {
            for (seed, n) in seeds {
                let Some(n) = n.scored() else { continue };
                if n.held == 0 {
                    rest.push(&n.summary);
                    continue;
                }
                with.push(&n.summary);
                let o = old.sets.get(d).unwrap_or(&empty).get(seed);
                if let Some(o) = o.and_then(Seed::scored) {
                    paired.push((&o.summary, &n.summary));
                }
            }
        }
        println!(
            "  the {} pairs of scored runs with a count of one or more, the first batch's \
             value against the new one's (larger, smaller, the same, one or both without):",
            paired.len()
        );
        for (what, of) in RUN_VALUES {
            let c = sides(&paired, |r| of(r));
            println!(
                "    {what:<52} {:>5} {:>5} {:>5} {:>5}",
                c[0], c[1], c[2], c[3]
            );
        }
        // Pattern 8 has no value for one run: the leaders' shares pooled over those pairs.
        let shares = |runs: Vec<RunSummary>| {
            gradient(&runs).map_or("none".to_string(), |(formed, rout)| {
                format!("{} and {}", num(formed), num(rout))
            })
        };
        println!(
            "    leaders' share of hits over those pairs, in the formed fight and in the rout: \
             the first batch {}; the new batch {}",
            shares(paired.iter().map(|p| p.0.clone()).collect()),
            shares(paired.iter().map(|p| p.1.clone()).collect())
        );
        // The ten runs of a set share their parameters, so the sets are counted too: each
        // value pooled over a set's seeds as the scorer pools it, in each batch.
        let touched: Vec<(Vec<RunSummary>, Vec<RunSummary>)> = new_sets
            .iter()
            .filter(|(_, n)| acted(n.as_slice()) > 0)
            .filter_map(|(d, n)| Some((summaries(old_sets.get(d)?), summaries(n))))
            .collect();
        println!(
            "  the {} sets with such a run that both batches have whole, each value pooled \
             over the set's seeds, the first batch's against the new one's (larger, smaller, \
             the same, one or both without):",
            touched.len()
        );
        let said = |what: &str, c: [usize; 4]| {
            println!(
                "    {what:<52} {:>5} {:>5} {:>5} {:>5}   {}",
                c[0],
                c[1],
                c[2],
                c[3],
                sign_line(
                    c[0],
                    c[1],
                    "larger in the first batch",
                    "larger in the new batch"
                )
            );
        };
        for row in &rows {
            said(row.name, sides(&touched, |r| pooled(row, r)));
        }
        said(
            "time to break, s (never: last)",
            sides(&touched, |r| {
                median(
                    r.iter()
                        .map(|r| r.time_to_break_s.unwrap_or(f64::INFINITY))
                        .collect(),
                )
            }),
        );
        println!(
            "  the new batch's {} scored runs with a count of one or more against its {} \
             others (median of each; chance that one of the first has the larger value):",
            with.len(),
            rest.len()
        );
        for (what, of) in RUN_VALUES {
            let x: Vec<f64> = with.iter().filter_map(|r| of(r)).collect();
            let y: Vec<f64> = rest.iter().filter_map(|r| of(r)).collect();
            // With fewer than ten runs in a group the medians stand alone, as in the cell check.
            let chance = if x.len() < 10 || y.len() < 10 {
                "too few".to_string()
            } else {
                let mut wins = 0.0;
                for p in &x {
                    for q in &y {
                        wins += if p > q {
                            1.0
                        } else if p == q {
                            0.5
                        } else {
                            0.0
                        };
                    }
                }
                format!("{:.3}", wins / (x.len() * y.len()) as f64)
            };
            let med = |v: Vec<f64>| median(v).map_or("none".to_string(), num);
            println!("    {what:<52} {:>8} {:>8} {chance:>7}", med(x), med(y));
        }
        println!();
    }
    Ok(())
}

/// The batch runs four battles at a time on the Pi (`tools/s09-batch.sh`), so a step takes a
/// quarter of its runs' own times added up.
const PI_JOBS: f64 = 4.0;

/// Said whichever cell it is, so that the refusal does not tell which cells have run what.
const NOT_IN: &str = "`relief` and `backed` are not yet in for every set of the stride in every \
                      cell; the top-up's time is read when the batch has finished";

/// Whether a run of `base` was still being fought at the top-up's twenty minutes, contact made
/// and neither side broken. The scorer's own rule is private to `Cellbook::topup`; `topup`
/// below stops unless this one gives the scorer's sets.
fn fought(r: &RunSummary) -> bool {
    let at = TOPUP_AT_S * 1000 / BASE_TICK.0;
    r.contact.is_some_and(|c| c < at) && r.side_break.iter().all(|b| b.is_none_or(|t| t >= at))
}

/// The sets of `base` still fought at twenty minutes in enough seeds for the top-up, on the
/// stride of `relief` and off it.
fn long_fought(base: &Sets, seeds: usize) -> Vec<u32> {
    complete(base, seeds)
        .into_iter()
        .filter(|(_, runs)| runs.iter().filter(|r| fought(&r.summary)).count() >= TOPUP_MIN_SEEDS)
        .map(|(d, _)| d)
        .collect()
}

/// The wall time of a set's stopped runs, if every seed has one. The top-up's sets are off the
/// audit's stride, so its runs are all stopped ones.
fn stopped_ms(sets: &Sets, d: u32, seeds: usize) -> Option<u64> {
    let runs: Vec<u64> = sets
        .get(&d)?
        .values()
        .filter_map(|s| s.stopped.as_ref())
        .map(|r| r.wall_ms)
        .collect();
    (runs.len() == seeds).then(|| runs.iter().sum())
}

/// What the top-up's runs of one variant would take in one cell, in milliseconds of the runs'
/// own clocks: the time `base` took on the top-up's sets, times what the variant cost beside
/// `base` on the sets of the stride that were as long fought. Where the stride has no such
/// set, all of its sets give the ratio.
fn topup_ms(base: &Sets, variant: &Sets, tops: &[u32], seeds: usize) -> Result<f64, String> {
    let long = long_fought(base, seeds);
    // (as long fought as the top-up's, the time on `base`, the time on the variant)
    let mut stride = Vec::new();
    for d in base.keys().filter(|d| *d % RELIEF_EVERY == 0) {
        let Some(b) = stopped_ms(base, *d, seeds) else {
            continue;
        };
        let v = stopped_ms(variant, *d, seeds).ok_or(NOT_IN)?;
        stride.push((long.contains(d), b, v));
    }
    if tops.is_empty() {
        return Ok(0.0);
    }
    let like = stride.iter().any(|s| s.0);
    let (b, v) = stride
        .iter()
        .filter(|s| s.0 || !like)
        .fold((0, 0), |(b, v), s| (b + s.1, v + s.2));
    if b == 0 {
        return Err("no time of `base` on the stride to set the variant's beside".into());
    }
    let mut top = 0;
    for d in tops {
        top += stopped_ms(base, *d, seeds)
            .ok_or("a set of the top-up lacks a stopped run of `base` on some seed")?;
    }
    Ok(top as f64 * v as f64 / b as f64)
}

/// The one figure of the top-up's time, from the milliseconds of the runs' own clocks.
fn topup_line(ms: f64) -> String {
    let hours = ms / PI_JOBS / 3_600_000.0;
    let time = if ms == 0.0 {
        "no time at all, there being no set for it in any cell".to_string()
    } else if hours < 0.5 {
        "under half an hour of the Pi's time over the eight cells".to_string()
    } else {
        // Rounded by hand: `{:.0}` takes a half to the even number, and half an hour to none.
        let h = hours.round();
        format!(
            "about {h} hour{} of the Pi's time over the eight cells",
            if h == 1.0 { "" } else { "s" }
        )
    };
    format!("The relief's top-up, if it is run: {time}.")
}

fn topup(a: &Asked) -> Result<(), String> {
    let b = bands()?;
    let mut ms = 0.0;
    for engine in ENGINES {
        for rung in 0..4 {
            let cell = Cell { engine, rung };
            let tops = scorer(a, cell, &b)?.topup;
            let sets = |variant: &str| -> Result<Sets, String> {
                Ok(read(&a.dir, cell, variant, a.freeze)?.ok_or(NOT_IN)?.sets)
            };
            let base = sets("base")?;
            let mine: Vec<u32> = long_fought(&base, a.seeds)
                .into_iter()
                .filter(|d| d % RELIEF_EVERY != 0)
                .take(TOPUP_MAX_SETS)
                .collect();
            if mine != tops {
                return Err("the top-up's sets as found here are not the scorer's".into());
            }
            for variant in ["relief", "backed"] {
                ms += topup_ms(&base, &sets(variant)?, &tops, a.seeds)?;
            }
        }
    }
    println!("{}", topup_line(ms));
    println!(
        "    (the time `base` took on the top-up's sets, times what `relief` and `backed` \
         cost beside `base`\n    on the stride's sets as long fought; four runs at a time)"
    );
    Ok(())
}

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_default();
    let done = asked(args).and_then(|a| match command.as_str() {
        "fails" => fails(&a),
        "rungs" => rungs(&a),
        "two" => two(&a),
        "topup" => topup(&a),
        _ => Err(
            "usage: cannae-extras fails|rungs|two|topup --records DIR [--first DIR] \
             [--freeze-seed F] [--seeds 10] [--equal-only]"
                .to_string(),
        ),
    });
    if let Err(e) = done {
        eprintln!("cannae-extras: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_coin_line() {
        // Exact two-sided 95 % lines of a fair coin.
        assert_eq!(coin_line(200), Some(115));
        assert_eq!(coin_line(50), Some(33));
        assert_eq!(coin_line(6), Some(6));
        assert_eq!(coin_line(5), None);
        assert_eq!(coin_line(0), None);
    }

    #[test]
    fn the_sign_line_in_words() {
        let said = |larger, smaller| sign_line(larger, smaller, "MORE", "LESS");
        assert!(said(115, 85).ends_with("115 of 200: PAST IT: MORE"));
        assert!(said(85, 115).ends_with("PAST IT THE OTHER WAY: LESS"));
        assert!(said(114, 86).ends_with("not past it"));
        assert_eq!(said(3, 2), "cannot be told with 5 sets not tied");
    }

    #[test]
    fn a_batchs_count_by_the_band() {
        // The interval against figures made apart, in Python, from the same formula.
        for (k, n, lo, hi) in [
            (0, 200, 0.0, 0.018846),
            (7, 200, 0.017055, 0.070471),
            (20, 200, 0.065670, 0.149407),
            (200, 200, 0.981154, 1.0),
        ] {
            let got = wilson(k, n);
            assert!(
                (got.0 - lo).abs() < 1e-6 && (got.1 - hi).abs() < 1e-6,
                "{k} of {n}: {got:?}"
            );
        }
        assert_eq!(wilson(0, 0), (0.0, 1.0));
        // The knife-edge share is met on it, not above it.
        assert!(
            by_the_band("X", 20, 200)
                .ends_with("20 of 200 sets, 10.0 % (95 % interval 6.6-14.9 %); at least 10 %: yes")
        );
        assert!(by_the_band("X", 19, 200).ends_with("at least 10 %: no"));
        assert!(by_the_band("X", 0, 0).ends_with("at least 10 %: no"));
    }

    #[test]
    fn the_sides_of_pairs() {
        let pairs = [
            (Some(2.0), Some(1.0)),
            (Some(1.0), Some(2.0)),
            (Some(1.0), Some(1.0)),
            (None, Some(1.0)),
            (None, None),
            (Some(f64::INFINITY), Some(f64::INFINITY)),
        ];
        assert_eq!(sides(&pairs, |x| *x), [1, 1, 2, 2]);
    }

    #[test]
    fn four_minutes_in_ticks() {
        assert_eq!(FOUR_MINUTES, 4_800);
    }

    #[test]
    fn the_parts_give_the_scorers_verdict() {
        let b = bands().unwrap();
        let rows = rows(&b);
        let run = |formed: f64, gap: Option<f64>, t: Option<f64>| RunSummary {
            formed_s: formed,
            median_gap_m: gap,
            time_to_break_s: t,
            hits_per_file_hour: [Some(1.0), None],
            ..RunSummary::default()
        };
        let sets = [
            vec![
                run(1_000.0, Some(5.0), Some(900.0)),
                run(2_000.0, Some(6.0), None),
            ],
            vec![run(1.0, Some(0.1), Some(2.0)), run(2.0, None, Some(3.0))],
            vec![run(1.0, None, None)],
            vec![],
        ];
        for runs in sets {
            let mine: Vec<Option<bool>> = judge(&runs, &b, &rows).iter().map(passes).collect();
            assert_eq!(mine, verdicts(&runs, &b));
        }
    }

    /// Made-up sets of none to four runs, each value below, on and above every end of its band
    /// or missing: the parts give the scorer's verdict for every one, and every pattern passes,
    /// fails and cannot be scored on the way.
    #[test]
    fn the_parts_give_the_scorers_verdict_at_every_edge() {
        fn some(roll: &mut impl FnMut(usize) -> usize, from: &[f64]) -> Option<f64> {
            (roll(4) > 0).then(|| from[roll(from.len())])
        }
        let b = bands().unwrap();
        let rows = rows(&b);
        // A generator of its own (xorshift), so that the test needs no crate.
        let mut state = 0x9E37_79B9_7F4A_7C15u64;
        let mut roll = move |n: usize| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state % n as u64) as usize
        };
        // Every end of a band is positive, so half of it is below and half as much again above.
        let least = |m: f64| vec![m * 0.5, m, m * 1.5];
        let within = |[lo, hi]: [f64; 2]| vec![lo * 0.5, lo, (lo + hi) / 2.0, hi, hi * 1.5];
        let harm = b.clock_lethality_per_bout.map_or(vec![0.01], within);
        let mut seen = [[false; 3]; 8];
        for _ in 0..20_000 {
            let runs: Vec<RunSummary> = (0..roll(5))
                .map(|_| {
                    let hits_formed = roll(3) as u32;
                    let hits_rout = roll(3) as u32;
                    RunSummary {
                        formed_s: least(b.standoff_min_formed_s)[roll(3)],
                        median_gap_m: some(&mut roll, &least(b.standoff_min_median_gap_m)),
                        burstiness: some(&mut roll, &least(b.pulses_min_burstiness)),
                        lull_share: some(&mut roll, &within(b.pulses_lull_share)),
                        first_turn_depth: some(&mut roll, &least(b.rear_first_min_depth)),
                        spread_rho: some(&mut roll, &least(b.spread_min_rho)),
                        spread_s: some(&mut roll, &within(b.spread_s)),
                        loser_after_break: some(&mut roll, &least(b.asymmetry_min_after_break)),
                        time_to_break_s: some(
                            &mut roll,
                            &within([b.holds_early_s, b.holds_never_s]),
                        ),
                        hits_per_file_hour: [
                            some(&mut roll, &within(b.clock_hits_per_file_hour)),
                            some(&mut roll, &within(b.clock_hits_per_file_hour)),
                        ],
                        lethality_per_bout: some(&mut roll, &harm),
                        hits_formed,
                        leader_hits_formed: roll(hits_formed as usize + 1) as u32,
                        hits_rout,
                        leader_hits_rout: roll(hits_rout as usize + 1) as u32,
                        ..RunSummary::default()
                    }
                })
                .collect();
            let mine: Vec<Option<bool>> = judge(&runs, &b, &rows).iter().map(passes).collect();
            let theirs = verdicts(&runs, &b);
            assert_eq!(mine, theirs, "{runs:?}");
            for (p, v) in theirs.iter().enumerate() {
                let k = match v {
                    Some(true) => 0,
                    Some(false) => 1,
                    None => 2,
                };
                seen[p][k] = true;
            }
        }
        assert!(seen.iter().all(|s| s.iter().all(|x| *x)), "{seen:?}");
    }

    const CELL: Cell = Cell {
        engine: Engine::Field,
        rung: 1,
    };

    /// A stopped run of two seeds' sets: its wall time, and whether it was still being fought
    /// at the top-up's twenty minutes.
    fn rec(variant: &str, draw: u32, seed: u64, wall_ms: u64, long: bool) -> Record {
        let at = TOPUP_AT_S * 1000 / BASE_TICK.0;
        Record {
            engine: CELL.engine,
            rung: CELL.rung,
            variant: variant.into(),
            freeze_seed: Some(1),
            draw,
            seed,
            whole: false,
            sim_s: 0,
            wall_ms,
            summary: RunSummary {
                contact: Some(100),
                side_break: [(!long).then_some(at - 1), None],
                ..RunSummary::default()
            },
            audit: None,
            relieved_s: vec![],
            held: 0,
        }
    }

    /// Both seeds of each set: (set, the wall time of each run, still fought at twenty minutes).
    fn sets_of(variant: &str, sets: &[(u32, u64, bool)]) -> Sets {
        let mut got = Sets::new();
        for &(d, ms, long) in sets {
            for seed in 1..=2 {
                got.entry(d).or_default().entry(seed).or_default().stopped =
                    Some(rec(variant, d, seed, ms, long));
            }
        }
        got
    }

    #[test]
    fn the_top_ups_sets_are_the_scorers() {
        let at = TOPUP_AT_S * 1000 / BASE_TICK.0;
        let mut book = Cellbook::read(Path::new("no such folder"), CELL, 2).unwrap();
        let mut base = Sets::new();
        // (contact, the first break) for each of two seeds, as in the scorer's own test, and
        // more sets off the stride than the top-up takes.
        let fought_ = (Some(100), None);
        let late = (Some(100), Some(at));
        let mut sets = vec![
            (0, [fought_, fought_]),
            (1, [fought_, late]),
            (2, [fought_, (Some(100), Some(at - 1))]),
            (3, [fought_, (None, None)]),
            (5, [late, late]),
            (6, [(Some(at), None), fought_]),
            (7, [(Some(at - 1), None), fought_]),
        ];
        sets.extend((9..60).map(|d| (d, [fought_, late])));
        for (d, runs) in sets {
            for ((contact, broke), seed) in runs.into_iter().zip(1..) {
                let mut r = rec("base", d, seed, 0, true);
                r.summary.contact = contact;
                r.summary.side_break = [None, broke];
                book.add(r.clone());
                base.entry(d).or_default().entry(seed).or_default().stopped = Some(r);
            }
        }
        // A set with one seed missing is no set of the top-up's.
        base.entry(61).or_default().entry(1).or_default().stopped =
            Some(rec("base", 61, 1, 0, true));
        book.add(rec("base", 61, 1, 0, true));
        let mine: Vec<u32> = long_fought(&base, 2)
            .into_iter()
            .filter(|d| d % RELIEF_EVERY != 0)
            .take(TOPUP_MAX_SETS)
            .collect();
        assert_eq!(mine, book.topup());
        assert_eq!(mine[..3], [1, 5, 7]);
        assert_eq!(mine.len(), TOPUP_MAX_SETS);
    }

    #[test]
    fn the_top_ups_time() {
        // Set 0 was long fought and cost three times as much relieved; set 4 was short and
        // cost the same. The top-up's sets are long fought, so set 0 gives the ratio.
        let base = sets_of(
            "base",
            &[
                (0, 100, true),
                (4, 10, false),
                (1, 50, true),
                (2, 7, false),
                (3, 80, true),
            ],
        );
        let relief = sets_of("relief", &[(0, 300, true), (4, 10, true)]);
        assert_eq!(topup_ms(&base, &relief, &[1, 3], 2), Ok(3.0 * 260.0));
        // No set of the top-up: no time, whatever the ratio.
        assert_eq!(topup_ms(&base, &relief, &[], 2), Ok(0.0));
        // No long-fought set on the stride: all of the stride's sets give the ratio.
        let base = sets_of("base", &[(0, 100, false), (4, 10, false), (1, 50, true)]);
        assert_eq!(topup_ms(&base, &relief, &[1], 2), Ok(100.0 * 620.0 / 220.0));
        // A set of the stride not yet run on the variant: refused, and no cell is named.
        let short = sets_of("relief", &[(0, 300, true)]);
        assert_eq!(topup_ms(&base, &short, &[1], 2), Err(NOT_IN.to_string()));
        assert_eq!(topup_ms(&base, &short, &[], 2), Err(NOT_IN.to_string()));
        // A whole run is not a stopped one.
        let mut whole = sets_of("relief", &[(0, 300, true), (4, 10, true)]);
        let seed = whole.get_mut(&4).unwrap().get_mut(&2).unwrap();
        seed.whole = seed.stopped.take();
        assert_eq!(topup_ms(&base, &whole, &[1], 2), Err(NOT_IN.to_string()));
    }

    #[test]
    fn the_top_ups_one_figure() {
        let hours = |h: f64| topup_line(h * PI_JOBS * 3_600_000.0);
        assert!(hours(0.0).contains("no time at all"));
        assert!(hours(0.2).contains("under half an hour"));
        assert!(hours(0.5).contains("about 1 hour of"));
        assert!(hours(1.2).contains("about 1 hour of"));
        assert!(hours(2.5).contains("about 3 hours of"));
        assert!(hours(13.6).contains("about 14 hours of"));
    }
}
