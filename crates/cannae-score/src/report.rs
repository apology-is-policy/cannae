//! Scoring a cell from its records, as the preregistration says
//! ([preregistration](../../docs/slice0/preregistration.md), "The procedure" and "Scoring").
//!
//! - The eight bands are scored on `base`, per parameter set, over its seeds ([`verdicts`]).
//! - Pattern 6 passes only if, for the same set, the Roman line fares worse with recruits
//!   than with veterans, worse in `flank` than in `base`, and worse in `base` than in
//!   `relief`, each with a probability of superiority over seed pairs of at least 0.6
//!   ([`fortune`]).
//! - A cell passes if at least 10 % of its sets pass all eight, as observed.
//!
//! Everything else here is reported, not scored.
#![allow(clippy::float_arithmetic)]

use crate::Record;
use crate::draw::Cell;
use cannae_core::Side;
use cannae_harness::metrics::{
    Bands, RunSummary, fortune, side_time_to_break_s, superiority, superiority_of, verdicts,
    weibull_shape,
};
use std::collections::BTreeMap;
use std::fmt::{self, Write as _};
use std::path::Path;

/// The patch variants, in the order the report gives them.
pub const VARIANTS: [&str; 7] = [
    "base", "veterans", "recruits", "flank", "relief", "volleys", "shock",
];

/// The Roman line in every patch variant (tested against the scenario files).
pub const ROMANS: Side = Side::A;

/// The share of sets a cell must pass all eight with (agreed in call 0001, turn 44).
pub const KNIFE_EDGE: f64 = 0.10;

/// A dose-response statistic passes at this or above.
pub const DOSE_MIN: f64 = 0.6;

const PATTERNS: [&str; 8] = [
    "1 standoff",
    "2 pulses and lulls",
    "3 rear-first collapse",
    "4 lateral spread",
    "5 casualty asymmetry",
    "6 holds, then breaks",
    "7 casualty clock",
    "8 centurion gradient",
];

/// One seed's runs of one set on one variant: the stopped run, the whole one, or both.
#[derive(Default)]
struct Seed {
    stopped: Option<Record>,
    whole: Option<Record>,
}

impl Seed {
    /// The summary scored: the stopped run's, or the whole run's, which equals it.
    fn summary(&self) -> Option<&RunSummary> {
        self.stopped
            .as_ref()
            .or(self.whole.as_ref())
            .map(|r| &r.summary)
    }
}

/// variant -> set -> seed.
type Records = BTreeMap<String, BTreeMap<u32, BTreeMap<u64, Seed>>>;

/// What a cell's records hold.
pub struct Cellbook {
    cell: Cell,
    seeds: usize,
    records: Records,
    freeze_seeds: Vec<Option<u64>>,
    skipped_lines: usize,
}

impl Cellbook {
    /// Reads the cell's file for each variant in `dir`; a variant with no file is missing.
    pub fn read(dir: &Path, cell: Cell, seeds: usize) -> Result<Cellbook, String> {
        let mut book = Cellbook {
            cell,
            seeds,
            records: Records::new(),
            freeze_seeds: Vec::new(),
            skipped_lines: 0,
        };
        for v in VARIANTS {
            let path = dir.join(crate::record_file(cell, v));
            let text = match std::fs::read_to_string(&path) {
                Ok(t) => t,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
                Err(e) => return Err(format!("{}: {e}", path.display())),
            };
            for line in text.lines() {
                match serde_json::from_str::<Record>(line) {
                    Ok(r) => book.add(r),
                    Err(_) => book.skipped_lines += 1,
                }
            }
        }
        Ok(book)
    }

    /// Adds one record. A record of another cell is a mistake in the files and is dropped.
    pub fn add(&mut self, r: Record) {
        if r.engine != self.cell.engine || r.rung != self.cell.rung {
            self.skipped_lines += 1;
            return;
        }
        if !self.freeze_seeds.contains(&r.freeze_seed) {
            self.freeze_seeds.push(r.freeze_seed);
        }
        let seed = self
            .records
            .entry(r.variant.clone())
            .or_default()
            .entry(r.draw)
            .or_default()
            .entry(r.seed)
            .or_default();
        if r.whole {
            seed.whole = Some(r);
        } else {
            seed.stopped = Some(r);
        }
    }

    /// The summaries of one set on one variant, by seed, if every seed is there.
    fn runs(&self, variant: &str, draw: u32) -> Option<Vec<&RunSummary>> {
        let seeds = self.records.get(variant)?.get(&draw)?;
        let runs: Vec<&RunSummary> = seeds.values().filter_map(Seed::summary).collect();
        (runs.len() == self.seeds).then_some(runs)
    }

    /// The cell scored.
    pub fn score(&self, bands: &Bands) -> Report {
        let mut rep = Report {
            cell: self.cell,
            seeds: self.seeds,
            freeze_seeds: self.freeze_seeds.clone(),
            skipped_lines: self.skipped_lines,
            sets: Vec::new(),
            incomplete_sets: 0,
            audits: Vec::new(),
        };
        let base_sets: Vec<u32> = self
            .records
            .get("base")
            .map(|m| m.keys().copied().collect())
            .unwrap_or_default();
        let lines = |a: &[&RunSummary]| -> Vec<(u8, f64)> {
            a.iter().map(|r| fortune(r, ROMANS)).collect()
        };
        let roman = |a: &[&RunSummary]| -> Vec<Option<f64>> {
            a.iter().map(|r| side_time_to_break_s(r, ROMANS)).collect()
        };
        let compare = |worse: &str, better: &str, d: u32| {
            let (w, b) = (self.runs(worse, d)?, self.runs(better, d)?);
            Some((
                superiority_of(&lines(&w), &lines(&b))?,
                superiority(&roman(&w), &roman(&b))?,
            ))
        };
        for d in base_sets {
            let Some(base) = self.runs("base", d) else {
                rep.incomplete_sets += 1;
                continue;
            };
            let owned: Vec<RunSummary> = base.iter().map(|r| (*r).clone()).collect();
            let v = verdicts(&owned, bands);
            let dose = [
                compare("recruits", "veterans", d),
                compare("flank", "base", d),
                compare("base", "relief", d),
            ];
            let dose_ok = dose.iter().all(|x| x.is_some_and(|(f, _)| f >= DOSE_MIN));
            let mut full = v;
            full[5] = Some(v[5] == Some(true) && dose_ok);
            let roman_broke = base
                .iter()
                .filter(|r| r.side_break[ROMANS as usize].is_some())
                .count();
            let times: Vec<Option<f64>> = base.iter().map(|r| r.time_to_break_s).collect();
            rep.sets.push(SetScore {
                draw: d,
                verdicts: v,
                dose,
                passes_all: full.iter().all(|p| *p == Some(true)),
                passed: full.iter().filter(|p| **p == Some(true)).count(),
                roman_broke,
                shape: weibull_shape(&times, bands.holds_never_s),
                shock: compare("shock", "base", d).map(|(f, _)| f),
                volleys: compare("volleys", "base", d).map(|(f, _)| f),
            });
        }
        for (variant, sets) in &self.records {
            let mut audit = AuditTally::default();
            for seeds in sets.values() {
                for s in seeds.values() {
                    if let (Some(a), Some(b)) = (&s.stopped, &s.whole) {
                        audit.paired += 1;
                        audit.equal += usize::from(a.summary == b.summary);
                    }
                    let Some(w) = &s.whole else { continue };
                    audit.whole += 1;
                    if let Some(a) = w.audit {
                        audit.closed += 1;
                        audit.bouts += usize::from(a.bouts > 0);
                        audit.rallies += usize::from(a.rallies > 0);
                        audit.second_breaks += usize::from(a.second_break);
                    }
                }
                if variant == "shock" || variant == "volleys" {
                    for s in seeds.values() {
                        if let Some(r) = s.summary() {
                            audit.losses_before_contact.push(r.losses_before_contact);
                        }
                    }
                }
            }
            rep.audits.push((variant.clone(), audit));
        }
        rep
    }
}

/// One parameter set's scores.
pub struct SetScore {
    pub draw: u32,
    /// On `base`, pattern 6 by its band alone.
    pub verdicts: [Option<bool>; 8],
    /// Recruits against veterans, `flank` against `base`, `base` against `relief`: how the
    /// Roman line fared (the rule), and its break time alone (reported beside it). `None`
    /// where a variant's runs are missing.
    pub dose: [Option<(f64, f64)>; 3],
    /// All eight, with pattern 6 needing the three dose-response statistics.
    pub passes_all: bool,
    /// How many of the eight passed, pattern 6 as scored: the second key for ranking rungs if
    /// none passes.
    pub passed: usize,
    /// Seeds of `base` in which the Roman line broke.
    pub roman_broke: usize,
    pub shape: Option<f64>,
    /// P(the Roman line fares worse in the variant than in `base`), reported.
    pub shock: Option<f64>,
    pub volleys: Option<f64>,
}

/// What the whole runs of one variant showed after the window, and what the memory variants
/// cost the fronts before contact.
#[derive(Default)]
pub struct AuditTally {
    pub whole: usize,
    /// Whole runs whose window closed before the run ended.
    pub closed: usize,
    pub bouts: usize,
    pub rallies: usize,
    pub second_breaks: usize,
    /// Seeds run both ways, and how many of them summarised the same.
    pub paired: usize,
    pub equal: usize,
    pub losses_before_contact: Vec<[u32; 2]>,
}

pub struct Report {
    pub cell: Cell,
    pub seeds: usize,
    pub freeze_seeds: Vec<Option<u64>>,
    pub skipped_lines: usize,
    pub sets: Vec<SetScore>,
    /// Sets with some `base` runs but not every seed.
    pub incomplete_sets: usize,
    pub audits: Vec<(String, AuditTally)>,
}

impl Report {
    /// Sets passing all eight, and the share.
    pub fn passing(&self) -> (usize, f64) {
        let k = self.sets.iter().filter(|s| s.passes_all).count();
        (k, k as f64 / self.sets.len().max(1) as f64)
    }

    /// The knife-edge rule on the observed share. `None` with no set scored.
    pub fn cell_passes(&self) -> Option<bool> {
        (!self.sets.is_empty()).then(|| self.passing().1 >= KNIFE_EDGE)
    }
}

/// The 95 % Wilson interval of k successes in n.
fn wilson(k: usize, n: usize) -> (f64, f64) {
    if n == 0 {
        return (0.0, 1.0);
    }
    let (p, n, z) = (k as f64 / n as f64, n as f64, 1.96f64);
    let mid = (p + z * z / (2.0 * n)) / (1.0 + z * z / n);
    let half = z * (p * (1.0 - p) / n + z * z / (4.0 * n * n)).sqrt() / (1.0 + z * z / n);
    ((mid - half).max(0.0), (mid + half).min(1.0))
}

/// The quartiles of `v`, by nearest rank.
fn quartiles(mut v: Vec<f64>) -> Option<[f64; 3]> {
    if v.is_empty() {
        return None;
    }
    v.sort_by(f64::total_cmp);
    let at = |q: f64| v[((v.len() - 1) as f64 * q).round() as usize];
    Some([at(0.25), at(0.5), at(0.75)])
}

fn spread(v: Vec<f64>) -> String {
    let n = v.len();
    quartiles(v).map_or("none".into(), |[a, m, b]| {
        format!("median {m:.2} (quartiles {a:.2}-{b:.2}, {n} sets)")
    })
}

impl fmt::Display for Report {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = self.sets.len();
        writeln!(
            f,
            "{:?} rung {}: {n} parameter sets scored at {} seeds each; freeze seed {:?}",
            self.cell.engine, self.cell.rung, self.seeds, self.freeze_seeds
        )?;
        if self.incomplete_sets > 0 || self.skipped_lines > 0 {
            writeln!(
                f,
                "  not scored: {} sets missing seeds of base, {} unreadable or stray lines",
                self.incomplete_sets, self.skipped_lines
            )?;
        }
        writeln!(
            f,
            "\n  {:<34} {:>5} {:>5} {:>11}",
            "pattern, on base", "pass", "fail", "unscorable"
        )?;
        for (i, name) in PATTERNS.iter().enumerate() {
            let count =
                |want: Option<bool>| self.sets.iter().filter(|s| s.verdicts[i] == want).count();
            let label = if i == 5 {
                "6 holds, then breaks (band only)"
            } else {
                name
            };
            writeln!(
                f,
                "  {label:<34} {:>5} {:>5} {:>11}",
                count(Some(true)),
                count(Some(false)),
                count(None)
            )?;
        }
        let with_dose = self
            .sets
            .iter()
            .filter(|s| {
                s.verdicts[5] == Some(true)
                    && s.dose.iter().all(|x| x.is_some_and(|(f, _)| f >= DOSE_MIN))
            })
            .count();
        writeln!(
            f,
            "  {:<34} {with_dose:>5} {:>5}",
            "6 with dose-response (scored)",
            n - with_dose
        )?;
        let (k, share) = self.passing();
        let (lo, hi) = wilson(k, n);
        writeln!(
            f,
            "\n  All eight: {k} of {n} sets, {:.1} % (95 % interval {:.1}-{:.1} %). Knife-edge rule, \
             at least {:.0} % as observed: {}",
            100.0 * share,
            100.0 * lo,
            100.0 * hi,
            100.0 * KNIFE_EDGE,
            match self.cell_passes() {
                Some(true) => "PASS",
                Some(false) => "FAIL",
                None => "nothing scored",
            }
        )?;
        writeln!(
            f,
            "  Patterns passed per set, of eight: {}",
            spread(self.sets.iter().map(|s| s.passed as f64).collect())
        )?;
        let names = [
            "recruits worse than veterans",
            "flank worse than base",
            "base worse than relief",
        ];
        writeln!(
            f,
            "\n  Dose-response, how the Roman line fared (the rule, at least {DOSE_MIN} to pass):"
        )?;
        for (j, name) in names.iter().enumerate() {
            let v: Vec<f64> = self
                .sets
                .iter()
                .filter_map(|s| s.dose[j].map(|x| x.0))
                .collect();
            let ok = v.iter().filter(|&&x| x >= DOSE_MIN).count();
            writeln!(f, "    {name:<30} {}; {ok} at or above", spread(v))?;
        }
        writeln!(
            f,
            "  Beside it, the Roman line's break time alone (reported):"
        )?;
        for (j, name) in names.iter().enumerate() {
            let v: Vec<f64> = self
                .sets
                .iter()
                .filter_map(|s| s.dose[j].map(|x| x.1))
                .collect();
            let ok = v.iter().filter(|&&x| x >= DOSE_MIN).count();
            writeln!(f, "    {name:<30} {}; {ok} at or above", spread(v))?;
        }
        let few = self.sets.iter().filter(|s| s.roman_broke < 2).count();
        writeln!(
            f,
            "    sets where the Roman line broke in fewer than 2 seeds of base: {few}"
        )?;
        writeln!(
            f,
            "\n  Shape of the time to break on base (Weibull k, reported): {}",
            spread(self.sets.iter().filter_map(|s| s.shape).collect())
        )?;
        writeln!(
            f,
            "  Memory variants, P(the Roman line fares worse than in base), reported:"
        )?;
        writeln!(
            f,
            "    shock   {}",
            spread(self.sets.iter().filter_map(|s| s.shock).collect())
        )?;
        writeln!(
            f,
            "    volleys {}",
            spread(self.sets.iter().filter_map(|s| s.volleys).collect())
        )?;
        let mut audit = String::new();
        for (variant, a) in &self.audits {
            if a.whole == 0 && a.losses_before_contact.is_empty() {
                continue;
            }
            write!(
                audit,
                "    {variant:<9} {} whole runs, {} with the window closed before the end: after it, \
                 {} with a bout, {} with a rally, {} with a second break; stopped and whole \
                 summaries equal in {} of {}",
                a.whole, a.closed, a.bouts, a.rallies, a.second_breaks, a.equal, a.paired
            )?;
            if !a.losses_before_contact.is_empty() {
                let med = |side: usize| {
                    quartiles(
                        a.losses_before_contact
                            .iter()
                            .map(|l| l[side] as f64)
                            .collect(),
                    )
                    .map_or(0.0, |q| q[1])
                };
                write!(
                    audit,
                    "; front losses before contact, median per run: Romans {:.0}, warband {:.0}",
                    med(0),
                    med(1)
                )?;
            }
            audit.push('\n');
        }
        if !audit.is_empty() {
            writeln!(f, "\n  The audit (reported):\n{audit}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wilson_brackets_ten_percent_of_two_hundred() {
        let (lo, hi) = wilson(20, 200);
        assert!(
            (0.06..0.07).contains(&lo) && (0.14..0.16).contains(&hi),
            "{lo} {hi}"
        );
    }
}
