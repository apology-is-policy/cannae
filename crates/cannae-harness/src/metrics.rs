//! The eight patterns of the Slice 0 patch, measured from observable state and the event log
//! only, so the same code scores both engines. This is analysis, not simulation state, so
//! floating point is allowed here and nowhere in the core.
//!
//! Definitions follow `docs/slice0/preregistration.md`; the pass bands live in
//! `scenarios/prereg.toml`.
#![allow(clippy::float_arithmetic)]
// Men are indexed across several parallel arrays; index loops read more plainly here.
#![allow(clippy::needless_range_loop)]

use crate::scenario::{Layout, Placed};
use cannae_core::schedule::BASE_TICK;
use cannae_core::{Bodies, BoutOutcome, Event, EventKind, Posture, Role, Side, Vec2, Wound};
use serde::Deserialize;

/// One man step's tallies.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sample {
    pub tick: u32,
    /// Per body: men by posture, in `Posture` order.
    pub postures: Vec<[u16; 5]>,
    pub in_place: Vec<u16>,
    /// Gap between the two sides' fronts in mm, if both have men facing the enemy.
    pub gap_mm: Option<i32>,
}

/// Records a [`Sample`] at every man step.
pub struct Recorder {
    axis: Vec2,
    axis_len: i64,
    n_bodies: usize,
    pub samples: Vec<Sample>,
}

/// The direction from side A's centroid to side B's at setup, in mm.
fn axis_of(bodies: &Bodies) -> Vec2 {
    let mut sum = [(0i64, 0i64, 0i64); 2];
    for i in 0..bodies.len() {
        let s = &mut sum[bodies.side[i] as usize];
        s.0 += bodies.pos[i].x as i64;
        s.1 += bodies.pos[i].y as i64;
        s.2 += 1;
    }
    let c = |s: (i64, i64, i64)| Vec2::new((s.0 / s.2.max(1)) as i32, (s.1 / s.2.max(1)) as i32);
    let d = c(sum[1]) - c(sum[0]);
    if d == Vec2::ZERO { Vec2::new(1, 0) } else { d }
}

fn faces_enemy(p: Posture) -> bool {
    matches!(p, Posture::Steady | Posture::Forward | Posture::Back)
}

/// The value at quantile `q` of a sorted slice.
fn quantile(sorted: &[i64], q: f64) -> i64 {
    let i = ((sorted.len() - 1) as f64 * q).round() as usize;
    sorted[i]
}

impl Recorder {
    pub fn new(initial: &Bodies, n_bodies: usize) -> Self {
        let axis = axis_of(initial);
        Recorder {
            axis,
            axis_len: axis.length().max(1),
            n_bodies,
            samples: Vec::new(),
        }
    }

    fn along(&self, p: Vec2) -> i64 {
        p.dot(self.axis) / self.axis_len
    }

    pub fn record(&mut self, tick: u32, bodies: &Bodies) {
        let mut postures = vec![[0u16; 5]; self.n_bodies];
        let mut in_place = vec![0u16; self.n_bodies];
        let mut fronts: [Vec<i64>; 2] = [Vec::new(), Vec::new()];
        for i in 0..bodies.len() {
            if let Some(b) = bodies.body[i] {
                postures[b.0 as usize][bodies.posture[i] as usize] += 1;
                in_place[b.0 as usize] += bodies.in_place[i] as u16;
            }
            if faces_enemy(bodies.posture[i]) {
                fronts[bodies.side[i] as usize].push(self.along(bodies.pos[i]));
            }
        }
        let gap_mm = if fronts[0].is_empty() || fronts[1].is_empty() {
            None
        } else {
            fronts[0].sort_unstable();
            fronts[1].sort_unstable();
            // A's front is its 90th percentile toward B; B's is its 10th toward A.
            Some((quantile(&fronts[1], 0.1) - quantile(&fronts[0], 0.9)) as i32)
        };
        self.samples.push(Sample {
            tick,
            postures,
            in_place,
            gap_mm,
        });
    }
}

/// What one run produced, pattern by pattern. `None` where a pattern does not apply (no break,
/// no bouts).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RunSummary {
    pub body_break: Vec<Option<u32>>,
    pub side_break: [Option<u32>; 2],
    pub loser: Option<Side>,
    pub contact: Option<u32>,
    /// End of the formed phase: the loser's break, or the end of the run.
    pub formed_end: u32,
    // 1. Standoff.
    pub formed_s: f64,
    pub median_gap_m: Option<f64>,
    // 2. Pulses and lulls.
    pub burstiness: Option<f64>,
    pub lull_share: Option<f64>,
    // 3. Rear-first collapse.
    pub first_turn_depth: Option<f64>,
    /// Men of the loser who turned before the collapse began: reported, not scored.
    pub stragglers: u32,
    // 4. Lateral spread.
    pub spread_rho: Option<f64>,
    pub spread_s: Option<f64>,
    // 5. Casualty asymmetry.
    pub loser_after_break: Option<f64>,
    // 6. Holds, then breaks.
    pub time_to_break_s: Option<f64>,
    // 7. The casualty clock.
    pub hits_per_file_hour: [Option<f64>; 2],
    pub bouts_formed: u32,
    pub lethality_per_bout: Option<f64>,
    /// Tempo, reported and not banded: the share of first-line front-rank man-time spent
    /// forward while formed, and the share of those men who started any bout while formed.
    pub front_forward_share: Option<f64>,
    pub front_starters_share: Option<f64>,
    // 8. The centurion gradient.
    pub leader_hits_formed: u32,
    pub hits_formed: u32,
    pub leader_hits_rout: u32,
    pub hits_rout: u32,
}

fn secs(ticks: u32) -> f64 {
    ticks as f64 * BASE_TICK.0 as f64 / 1000.0
}

/// Goh and Barabasi's burstiness of the gaps between successive events, (sigma - mu) /
/// (sigma + mu): 0 for a Poisson process at any rate, toward 1 for bursts, toward -1 for a
/// regular beat. On counts per bin instead, a random process at under about half an event per
/// bin would score above 0.2 (Varro, call 0001 turn 16).
pub fn burstiness(gaps: &[u32]) -> Option<f64> {
    if gaps.len() < 2 {
        return None;
    }
    let n = gaps.len() as f64;
    let mu = gaps.iter().map(|&c| c as f64).sum::<f64>() / n;
    let var = gaps.iter().map(|&c| (c as f64 - mu).powi(2)).sum::<f64>() / n;
    let sd = var.sqrt();
    if mu + sd == 0.0 {
        None
    } else {
        Some((sd - mu) / (sd + mu))
    }
}

fn ranks_of(v: &[f64]) -> Vec<f64> {
    let mut idx: Vec<usize> = (0..v.len()).collect();
    idx.sort_by(|&a, &b| v[a].total_cmp(&v[b]));
    let mut r = vec![0.0; v.len()];
    let mut i = 0;
    while i < idx.len() {
        let mut j = i;
        while j + 1 < idx.len() && v[idx[j + 1]] == v[idx[i]] {
            j += 1;
        }
        let avg = (i + j) as f64 / 2.0;
        for &k in &idx[i..=j] {
            r[k] = avg;
        }
        i = j + 1;
    }
    r
}

/// Spearman's rank correlation, with average ranks for ties.
pub fn spearman(x: &[f64], y: &[f64]) -> Option<f64> {
    if x.len() != y.len() || x.len() < 3 {
        return None;
    }
    let (rx, ry) = (ranks_of(x), ranks_of(y));
    let n = x.len() as f64;
    let (mx, my) = (rx.iter().sum::<f64>() / n, ry.iter().sum::<f64>() / n);
    let cov: f64 = rx.iter().zip(&ry).map(|(a, b)| (a - mx) * (b - my)).sum();
    let vx: f64 = rx.iter().map(|a| (a - mx).powi(2)).sum();
    let vy: f64 = ry.iter().map(|b| (b - my).powi(2)).sum();
    if vx == 0.0 || vy == 0.0 {
        None
    } else {
        Some(cov / (vx * vy).sqrt())
    }
}

fn median(v: &mut [f64]) -> Option<f64> {
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

pub struct Inputs<'a> {
    pub initial: &'a Bodies,
    pub placed: &'a [Placed],
    pub layout: &'a Layout,
    pub samples: &'a [Sample],
    pub events: &'a [Event],
    pub end_tick: u32,
    /// The share of a body's standing men who must be turned for it to count as broken.
    pub break_share: f64,
}

/// The first tick at which each man was disabled or killed.
fn first_hits(n: usize, events: &[Event]) -> Vec<Option<u32>> {
    let mut hit = vec![None; n];
    for e in events {
        if let EventKind::Wound { to } = e.kind
            && to >= Wound::Disabled
            && hit[e.man.0 as usize].is_none()
        {
            hit[e.man.0 as usize] = Some(e.tick);
        }
    }
    hit
}

pub fn summarize(inp: &Inputs) -> RunSummary {
    let n = inp.initial.len();
    let nb = inp.placed.len();
    let side_of_body: Vec<Side> = inp
        .placed
        .iter()
        .map(|p| inp.initial.side[p.first as usize])
        .collect();
    let broken = |c: &[u16; 5], extra: (u32, u32)| {
        let standing =
            c.iter().map(|&x| x as u32).sum::<u32>() - c[Posture::Down as usize] as u32 + extra.0;
        let turned = c[Posture::Turned as usize] as u32 + extra.1;
        standing > 0 && turned as f64 >= inp.break_share * standing as f64
    };
    let mut s = RunSummary {
        body_break: vec![None; nb],
        formed_end: inp.end_tick,
        ..Default::default()
    };
    for smp in inp.samples {
        let mut side_tally = [[0u16; 5]; 2];
        for (b, c) in smp.postures.iter().enumerate() {
            if s.body_break[b].is_none() && broken(c, (0, 0)) {
                s.body_break[b] = Some(smp.tick);
            }
            for p in 0..5 {
                side_tally[side_of_body[b] as usize][p] += c[p];
            }
        }
        for side in 0..2 {
            if s.side_break[side].is_none() && broken(&side_tally[side], (0, 0)) {
                s.side_break[side] = Some(smp.tick);
            }
        }
    }
    s.loser = match s.side_break {
        [Some(a), Some(b)] => Some(if a <= b { Side::A } else { Side::B }),
        [Some(_), None] => Some(Side::A),
        [None, Some(_)] => Some(Side::B),
        [None, None] => None,
    };
    let break_tick = s.loser.and_then(|l| s.side_break[l as usize]);
    if let Some(t) = break_tick {
        s.formed_end = t;
    }

    let first_bout = inp
        .events
        .iter()
        .find(|e| matches!(e.kind, EventKind::Bout { .. }))
        .map(|e| e.tick);
    let first_close = inp
        .samples
        .iter()
        .find(|smp| smp.gap_mm.is_some_and(|g| g <= 3_000))
        .map(|smp| smp.tick);
    s.contact = match (first_bout, first_close) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    };
    let start = s.contact.unwrap_or(inp.end_tick);
    let formed = start..s.formed_end.max(start);
    s.formed_s = secs(formed.end - formed.start);

    // 1. The gap while formed.
    let mut gaps: Vec<f64> = inp
        .samples
        .iter()
        .filter(|smp| formed.contains(&smp.tick))
        .filter_map(|smp| smp.gap_mm)
        .map(|g| g as f64 / 1000.0)
        .collect();
    s.median_gap_m = median(&mut gaps);

    // 2. Bouts per ten-second bin while formed.
    let bin = 10_000 / BASE_TICK.0;
    let nbins = ((formed.end - formed.start) / bin) as usize;
    if nbins >= 2 {
        let mut counts = vec![0u32; nbins];
        for e in inp.events {
            if matches!(e.kind, EventKind::Bout { .. }) && formed.contains(&e.tick) {
                let k = ((e.tick - formed.start) / bin) as usize;
                if k < nbins {
                    counts[k] += 1;
                }
            }
        }
        let max = *counts.iter().max().unwrap_or(&0);
        if max > 0 {
            let lulls = counts
                .iter()
                .filter(|&&c| c as f64 <= 0.2 * max as f64)
                .count();
            s.lull_share = Some(lulls as f64 / nbins as f64);
        }
    }

    // Burstiness of the gaps between bouts, per 9 m sector of front (by the defender's place
    // at setup), median over the sectors with at least three gaps. Flurries in different
    // sectors pooled together would add up toward random.
    let axis = axis_of(inp.initial);
    let len = axis.length().max(1);
    let lateral = |p: Vec2| axis.cross(p) / len;
    let lo = (0..n)
        .map(|i| lateral(inp.initial.pos[i]))
        .min()
        .unwrap_or(0);
    let mut sectors: Vec<Vec<u32>> = Vec::new();
    for e in inp.events {
        if let EventKind::Bout { .. } = e.kind
            && formed.contains(&e.tick)
        {
            let k = ((lateral(inp.initial.pos[e.man.0 as usize]) - lo) / 9_000) as usize;
            if sectors.len() <= k {
                sectors.resize(k + 1, Vec::new());
            }
            sectors[k].push(e.tick);
        }
    }
    let mut per_sector: Vec<f64> = sectors
        .iter()
        .filter_map(|ticks| {
            let gaps: Vec<u32> = ticks.windows(2).map(|w| w[1] - w[0]).collect();
            (gaps.len() >= 3).then(|| burstiness(&gaps)).flatten()
        })
        .collect();
    s.burstiness = median(&mut per_sector);

    // Turns, by man, in tick order.
    let mut turns: Vec<Vec<u32>> = vec![Vec::new(); n];
    for e in inp.events {
        if let EventKind::Posture {
            to: Posture::Turned,
            ..
        } = e.kind
        {
            turns[e.man.0 as usize].push(e.tick);
        }
    }
    let first_turn_from = |i: usize, from: u32| turns[i].iter().copied().find(|&t| t >= from);
    // The collapse of a tally: from the last sample before `until` at which under a quarter of
    // the break share had turned, reaching back through any earlier turns of the same men that
    // come in an unbroken chain, each within `CHAIN` of the next, so that the first men of a
    // cascade belong to it. A man who ran minutes before the cascade is a straggler, not its
    // first sign.
    const CHAIN: u32 = 30_000 / BASE_TICK.0;
    let quarter = inp.break_share / 4.0;
    let calm = |c: &[u16; 5]| {
        let standing = c.iter().map(|&x| x as u32).sum::<u32>() - c[Posture::Down as usize] as u32;
        standing == 0 || (c[Posture::Turned as usize] as f64) < quarter * standing as f64
    };
    let collapse_from = |until: u32, tally: &dyn Fn(&Sample) -> [u16; 5], men: &[usize]| {
        let from = inp
            .samples
            .iter()
            .filter(|smp| smp.tick < until && calm(&tally(smp)))
            .map(|smp| smp.tick)
            .max()
            .unwrap_or(0);
        let mut ts: Vec<u32> = men
            .iter()
            .flat_map(|&i| turns[i].iter().copied())
            .filter(|&t| t < until)
            .collect();
        ts.sort_unstable();
        let mut start = from;
        let mut next = ts.iter().copied().find(|&t| t >= from).unwrap_or(from);
        for &t in ts.iter().rev().filter(|&&t| t < from) {
            if next - t > CHAIN {
                break;
            }
            start = t;
            next = t;
        }
        start
    };

    if let (Some(loser), Some(bt)) = (s.loser, break_tick) {
        // 3. Depth of the first tenth of each broken body of the loser to turn.
        let mut depths = Vec::new();
        for (b, p) in inp.placed.iter().enumerate() {
            let Some(body_bt) = s.body_break[b] else {
                continue;
            };
            if side_of_body[b] != loser {
                continue;
            }
            let body_men: Vec<usize> = (p.first..p.first + p.count).map(|m| m as usize).collect();
            let from = collapse_from(body_bt, &|smp: &Sample| smp.postures[b], &body_men);
            let mut in_collapse: Vec<(u32, u32)> = (p.first..p.first + p.count)
                .filter_map(|m| first_turn_from(m as usize, from).map(|t| (t, m)))
                .filter(|&(t, _)| t <= body_bt)
                .collect();
            in_collapse.sort_unstable();
            let k = (p.count as usize).div_ceil(10);
            for &(_, m) in in_collapse.iter().take(k) {
                let r = inp.layout.rank[m as usize] as f64;
                let d = if p.ranks > 1 {
                    (r / (p.ranks - 1) as f64).min(1.0)
                } else {
                    0.0
                };
                depths.push(d);
            }
        }
        if !depths.is_empty() {
            s.first_turn_depth = Some(depths.iter().sum::<f64>() / depths.len() as f64);
        }

        // 4. Lateral spread across the loser's line, in 2 m bins of setup position, over the
        // turns from the start of the side's collapse.
        let side_tally = |smp: &Sample| {
            let mut t = [0u16; 5];
            for (b, c) in smp.postures.iter().enumerate() {
                if side_of_body[b] == loser {
                    for k in 0..5 {
                        t[k] += c[k];
                    }
                }
            }
            t
        };
        let men: Vec<usize> = (0..n).filter(|&i| inp.initial.side[i] == loser).collect();
        let from = collapse_from(bt, &side_tally, &men);
        s.stragglers = men
            .iter()
            .filter(|&&i| turns[i].first().is_some_and(|&t| t < from))
            .count() as u32;
        let lo = men
            .iter()
            .map(|&i| lateral(inp.initial.pos[i]))
            .min()
            .unwrap_or(0);
        let mut bins: Vec<(bool, Option<u32>)> = Vec::new();
        for &i in &men {
            let k = ((lateral(inp.initial.pos[i]) - lo) / 2_000) as usize;
            if bins.len() <= k {
                bins.resize(k + 1, (false, None));
            }
            bins[k].0 = true;
            if let Some(t) = first_turn_from(i, from) {
                bins[k].1 = Some(bins[k].1.map_or(t, |u: u32| u.min(t)));
            }
        }
        let turned: Vec<(usize, u32)> = bins
            .iter()
            .enumerate()
            .filter_map(|(k, b)| b.1.map(|t| (k, t)))
            .collect();
        if let Some(&(origin, t0)) = turned.iter().min_by_key(|(k, t)| (*t, *k)) {
            let (x, y): (Vec<f64>, Vec<f64>) = turned
                .iter()
                .map(|&(k, t)| ((k as f64 - origin as f64).abs(), (t - t0) as f64))
                .unzip();
            s.spread_rho = spearman(&x, &y);
            let occupied = bins.iter().filter(|b| b.0).count();
            let mut times: Vec<u32> = turned.iter().map(|&(_, t)| t).collect();
            times.sort_unstable();
            let need = (occupied * 8).div_ceil(10);
            if need > 0 && times.len() >= need {
                s.spread_s = Some(secs(times[need - 1] - t0));
            }
        }

        // 5. The loser's losses after the break.
        let hits = first_hits(n, inp.events);
        let (mut before, mut after) = (0u32, 0u32);
        for i in 0..n {
            if inp.initial.side[i] == loser
                && let Some(t) = hits[i]
            {
                if t >= bt { after += 1 } else { before += 1 }
            }
        }
        if before + after > 0 {
            s.loser_after_break = Some(after as f64 / (before + after) as f64);
        }

        // 6. Time from contact to the break.
        s.time_to_break_s = Some(secs(bt.saturating_sub(start)));
    }

    // 7. Hits per front file per hour while formed, each side against its own files.
    let hits = first_hits(n, inp.events);
    let hours = s.formed_s / 3600.0;
    for side in [Side::A, Side::B] {
        let files: u32 = front_bodies(inp, side)
            .iter()
            .map(|&b| inp.placed[b].files)
            .sum();
        let h = (0..n)
            .filter(|&i| inp.initial.side[i] == side)
            .filter(|&i| hits[i].is_some_and(|t| formed.contains(&t)))
            .count();
        if files > 0 && hours > 0.0 {
            s.hits_per_file_hour[side as usize] = Some(h as f64 / files as f64 / hours);
        }
    }
    let (mut bouts, mut lethal) = (0u32, 0u32);
    for e in inp.events {
        if let EventKind::Bout {
            defender,
            attacker_took,
            ..
        } = e.kind
            && formed.contains(&e.tick)
        {
            bouts += 1;
            lethal += (defender >= BoutOutcome::Disabled || attacker_took >= BoutOutcome::Disabled)
                as u32;
        }
    }
    s.bouts_formed = bouts;
    if bouts > 0 {
        s.lethality_per_bout = Some(lethal as f64 / bouts as f64);
    }

    // Tempo: the first line's front rank, forward and starting bouts while formed.
    let front: Vec<usize> = [Side::A, Side::B]
        .iter()
        .flat_map(|&side| front_bodies(inp, side))
        .flat_map(|b| {
            let p = &inp.placed[b];
            (p.first..p.first + p.count).map(|m| m as usize)
        })
        .filter(|&i| inp.layout.rank[i] == 0)
        .collect();
    if !front.is_empty() && formed.end > formed.start {
        let mut is_front = vec![false; n];
        for &i in &front {
            is_front[i] = true;
        }
        let mut since: Vec<Option<u32>> = vec![None; n];
        let mut forward_ticks = 0u64;
        let mut started = vec![false; n];
        let clip = |t: u32| t.clamp(formed.start, formed.end);
        for e in inp.events {
            let i = e.man.0 as usize;
            match e.kind {
                EventKind::Posture { from, to } if is_front[i] => {
                    if to == Posture::Forward && from != Posture::Forward {
                        since[i] = Some(e.tick);
                    } else if from == Posture::Forward
                        && to != Posture::Forward
                        && let Some(t0) = since[i].take()
                    {
                        forward_ticks += (clip(e.tick) - clip(t0)) as u64;
                    }
                }
                EventKind::Bout { attacker, .. }
                    if formed.contains(&e.tick) && is_front[attacker.0 as usize] =>
                {
                    started[attacker.0 as usize] = true;
                }
                _ => {}
            }
        }
        for t0 in since.into_iter().flatten() {
            forward_ticks += (clip(formed.end) - clip(t0)) as u64;
        }
        let man_ticks = front.len() as u64 * (formed.end - formed.start) as u64;
        s.front_forward_share = Some(forward_ticks as f64 / man_ticks as f64);
        s.front_starters_share =
            Some(front.iter().filter(|&&i| started[i]).count() as f64 / front.len() as f64);
    }

    // 8. Leaders' share of losses, formed against after the break.
    for i in 0..n {
        if let Some(t) = hits[i] {
            let leader = inp.initial.role[i] == Role::Leader;
            if formed.contains(&t) {
                s.hits_formed += 1;
                s.leader_hits_formed += leader as u32;
            } else if break_tick.is_some_and(|b| t >= b) {
                s.hits_rout += 1;
                s.leader_hits_rout += leader as u32;
            }
        }
    }
    s
}

/// A side's first-line bodies: those whose front rank stood within 5 m of the side's front at
/// setup. A second line or a reserve does not count against the clock.
fn front_bodies(inp: &Inputs, side: Side) -> Vec<usize> {
    let axis = axis_of(inp.initial);
    let len = axis.length().max(1);
    let along = |i: usize| inp.initial.pos[i].dot(axis) / len;
    let sign = if side == Side::A { 1 } else { -1 };
    let fronts: Vec<(usize, i64)> = inp
        .placed
        .iter()
        .enumerate()
        .filter(|(_, p)| inp.initial.side[p.first as usize] == side)
        .map(|(b, p)| {
            let f = (p.first..p.first + p.count)
                .filter(|&m| inp.layout.rank[m as usize] == 0)
                .map(|m| sign * along(m as usize))
                .max()
                .unwrap_or(i64::MIN);
            (b, f)
        })
        .collect();
    let best = fronts.iter().map(|f| f.1).max().unwrap_or(0);
    fronts
        .into_iter()
        .filter(|f| best - f.1 <= 5_000)
        .map(|f| f.0)
        .collect()
}

/// Probability of superiority: P(a < b) + P(a = b) / 2, with `None` (never broke) above every
/// time. For dose-response: does variant `a` break sooner than `b`?
pub fn superiority(a: &[Option<f64>], b: &[Option<f64>]) -> Option<f64> {
    if a.is_empty() || b.is_empty() {
        return None;
    }
    let key = |v: &Option<f64>| v.unwrap_or(f64::INFINITY);
    let mut wins = 0.0;
    for x in a {
        for y in b {
            let (x, y) = (key(x), key(y));
            wins += if x < y {
                1.0
            } else if x == y {
                0.5
            } else {
                0.0
            };
        }
    }
    Some(wins / (a.len() * b.len()) as f64)
}

/// The shape k of a Weibull fitted to times to break by maximum likelihood, with seeds still
/// unbroken at `censor_s` censored there. Reported for each cell, not banded: k = 1 is a line
/// whose chance of breaking is the same every minute, k > 1 one that grows brittle. `None`
/// with fewer than two breaks, or when every break came at the same moment.
///
/// The profile likelihood gives 1/k + mean(ln t, breaks) = sum(t^k ln t) / sum(t^k) over all
/// seeds; the left side minus the right falls as k grows, so bisection finds the root.
pub fn weibull_shape(times_s: &[Option<f64>], censor_s: f64) -> Option<f64> {
    let broke: Vec<f64> = times_s
        .iter()
        .flatten()
        .copied()
        .filter(|&t| t <= censor_s)
        .map(|t| t.max(1e-3))
        .collect();
    if broke.len() < 2 || broke.iter().all(|&t| t == broke[0]) {
        return None;
    }
    let all: Vec<f64> = times_s
        .iter()
        .map(|t| {
            t.filter(|&t| t <= censor_s)
                .map_or(censor_s, |t| t.max(1e-3))
        })
        .collect();
    // Scaled so that t^k cannot overflow for large k.
    let top = all.iter().copied().fold(0.0, f64::max);
    let mean_ln = broke.iter().map(|t| (t / top).ln()).sum::<f64>() / broke.len() as f64;
    let g = |k: f64| {
        let (mut num, mut den) = (0.0, 0.0);
        for &t in &all {
            let x = t / top;
            let w = x.powf(k);
            num += w * x.ln();
            den += w;
        }
        1.0 / k + mean_ln - num / den
    };
    let (mut lo, mut hi) = (0.01, 50.0);
    if g(hi) > 0.0 {
        return Some(hi);
    }
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        if g(mid) > 0.0 { lo = mid } else { hi = mid }
    }
    Some(0.5 * (lo + hi))
}

/// The pre-registered pass bands. Each per-run statistic is pooled over a parameter set's seeds
/// by its median; a parameter set passes a pattern when the pooled value lies in the band.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Bands {
    pub break_share: f64,
    pub standoff_min_formed_s: f64,
    pub standoff_min_median_gap_m: f64,
    pub pulses_min_burstiness: f64,
    pub pulses_lull_share: [f64; 2],
    pub rear_first_min_depth: f64,
    pub spread_min_rho: f64,
    pub spread_s: [f64; 2],
    pub asymmetry_min_after_break: f64,
    /// At most `holds_max_early` of seeds may break within `holds_early_s` of contact, and at
    /// most `holds_max_never` may still be unbroken `holds_never_s` after contact. A scenario
    /// must run long enough past contact to see that far.
    pub holds_early_s: f64,
    pub holds_max_early: f64,
    pub holds_never_s: f64,
    pub holds_max_never: f64,
    pub clock_hits_per_file_hour: [f64; 2],
    /// Fixed after the lethality search (S0.4).
    pub clock_lethality_per_bout: Option<[f64; 2]>,
}

impl Bands {
    pub fn from_toml(text: &str) -> Result<Bands, toml::de::Error> {
        toml::from_str(text)
    }
}

/// Per-pattern verdicts for one parameter set over its seeds. `None` means the pattern could
/// not be scored (for example, no seed broke), which the pre-registration counts as a failure.
pub fn verdicts(runs: &[RunSummary], b: &Bands) -> [Option<bool>; 8] {
    let med = |f: &dyn Fn(&RunSummary) -> Option<f64>| {
        let mut v: Vec<f64> = runs.iter().filter_map(f).collect();
        median(&mut v)
    };
    let inside = |x: f64, r: [f64; 2]| x >= r[0] && x <= r[1];
    let formed = med(&|r| Some(r.formed_s));
    let gap = med(&|r| r.median_gap_m);
    let p1 = formed
        .zip(gap)
        .map(|(f, g)| f >= b.standoff_min_formed_s && g >= b.standoff_min_median_gap_m);
    let p2 = med(&|r| r.burstiness)
        .zip(med(&|r| r.lull_share))
        .map(|(bu, l)| bu >= b.pulses_min_burstiness && inside(l, b.pulses_lull_share));
    let p3 = med(&|r| r.first_turn_depth).map(|d| d >= b.rear_first_min_depth);
    let p4 = med(&|r| r.spread_rho)
        .zip(med(&|r| r.spread_s))
        .map(|(rho, t)| rho >= b.spread_min_rho && inside(t, b.spread_s));
    let p5 = med(&|r| r.loser_after_break).map(|a| a >= b.asymmetry_min_after_break);
    let p6 = (!runs.is_empty()).then(|| {
        let n = runs.len() as f64;
        let early = runs
            .iter()
            .filter(|r| r.time_to_break_s.is_some_and(|t| t < b.holds_early_s))
            .count() as f64;
        let never = runs
            .iter()
            .filter(|r| r.time_to_break_s.is_none_or(|t| t > b.holds_never_s))
            .count() as f64;
        early / n <= b.holds_max_early && never / n <= b.holds_max_never
    });
    let clock = runs
        .iter()
        .flat_map(|r| r.hits_per_file_hour.iter().flatten().copied())
        .collect::<Vec<f64>>();
    let p7 = median(&mut clock.clone()).map(|c| {
        let lethal_ok = match b.clock_lethality_per_bout {
            Some(r) => med(&|r| r.lethality_per_bout).is_some_and(|l| inside(l, r)),
            None => true,
        };
        inside(c, b.clock_hits_per_file_hour) && lethal_ok
    });
    let (lf, f, lr, r) = runs.iter().fold((0, 0, 0, 0), |a, x| {
        (
            a.0 + x.leader_hits_formed,
            a.1 + x.hits_formed,
            a.2 + x.leader_hits_rout,
            a.3 + x.hits_rout,
        )
    });
    let p8 = (f > 0 && r > 0).then(|| lf as f64 / f as f64 > lr as f64 / r as f64);
    [p1, p2, p3, p4, p5, p6, p7, p8]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scenario::{Scenario, place};
    use cannae_core::{Cause, ManId};

    #[test]
    fn the_bands_file_parses() {
        let b = Bands::from_toml(include_str!("../../../scenarios/prereg.toml")).unwrap();
        assert_eq!(b.break_share, 0.4);
        assert_eq!(b.clock_lethality_per_bout, Some([0.002, 0.15]));
        let empty = verdicts(&[], &b);
        assert!(empty.iter().all(Option::is_none));
    }

    #[test]
    fn statistics() {
        // On gaps: a regular beat is -1, a burst after a long lull is high, and random gaps
        // (exponential quantiles) are near 0 whatever their rate.
        assert_eq!(burstiness(&[5, 5, 5, 5]), Some(-1.0));
        assert!(burstiness(&[1, 1, 1, 100, 1, 1, 1]).unwrap() > 0.3);
        for scale in [20.0, 2_000.0] {
            let gaps: Vec<u32> = (0..400)
                .map(|i| (-(1.0 - (i as f64 + 0.5) / 400.0).ln() * scale).round() as u32)
                .collect();
            let b = burstiness(&gaps).unwrap();
            assert!(b.abs() < 0.06, "random gaps at scale {scale}: {b}");
        }
        let rho = spearman(&[1.0, 2.0, 3.0, 4.0], &[10.0, 20.0, 25.0, 40.0]).unwrap();
        assert!((rho - 1.0).abs() < 1e-12);
        assert_eq!(superiority(&[Some(1.0)], &[None]), Some(1.0));
        assert_eq!(superiority(&[Some(2.0)], &[Some(2.0)]), Some(0.5));
        // Weibull quantiles at shape 1 and 2 give their shapes back, with and without
        // censoring the latest third.
        for k in [1.0, 2.0] {
            let times: Vec<Option<f64>> = (0..300)
                .map(|i| {
                    let u = (i as f64 + 0.5) / 300.0;
                    Some(1_800.0 * (-(1.0 - u).ln()).powf(1.0 / k))
                })
                .collect();
            let full = weibull_shape(&times, f64::INFINITY).unwrap();
            assert!((full - k).abs() < 0.05 * k, "k {k}: {full}");
            let mut sorted: Vec<f64> = times.iter().flatten().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let cut = sorted[200];
            let censored = weibull_shape(&times, cut).unwrap();
            assert!((censored - k).abs() < 0.1 * k, "k {k} censored: {censored}");
        }
        assert_eq!(weibull_shape(&[Some(5.0), None], 10.0), None);
    }

    const LINE: &str = r#"
        name = "line"
        seed = 1
        duration_s = 600
        [[body]]
        name = "a"
        side = "A"
        troop = "t"
        files = 10
        ranks = 4
        file_mm = 1000
        rank_mm = 1000
        centre_mm = [0, 0]
        facing_deg = 0
        [[body]]
        name = "b"
        side = "B"
        troop = "t"
        files = 10
        ranks = 4
        file_mm = 1000
        rank_mm = 1000
        centre_mm = [8000, 0]
        facing_deg = 180
    "#;

    fn posture_event(tick: u32, man: u32, to: Posture) -> Event {
        Event {
            tick,
            man: ManId(man),
            pos: Vec2::ZERO,
            kind: EventKind::Posture {
                from: Posture::Steady,
                to,
            },
            causes: [Cause::None; 3],
        }
    }

    /// A hand-made collapse: side A's rear rank turns first, from file 0 outward, and losses
    /// fall mostly afterwards.
    #[test]
    fn a_rear_first_collapse_scores_as_one() {
        let s = Scenario::from_toml(LINE).unwrap();
        let (mut bodies, placed) = place(&s);
        let initial = bodies.clone();
        let layout = Layout::of(&placed);
        let mut rec = Recorder::new(&initial, placed.len());
        let mut events = Vec::new();
        // Contact at tick 100; a bout every so often.
        for t in (100..1000).step_by(40) {
            events.push(Event {
                tick: t,
                man: ManId(40),
                pos: Vec2::ZERO,
                kind: EventKind::Bout {
                    attacker: ManId(0),
                    defender: BoutOutcome::Nothing,
                    attacker_took: BoutOutcome::Nothing,
                },
                causes: [Cause::Bout; 3],
            });
        }
        // Rear rank (rank 3: men 30-39) turns file by file from tick 1000, then rank 2.
        let mut t = 1000;
        for m in (30..40).chain(20..30) {
            events.push(posture_event(t, m, Posture::Turned));
            t += 10;
        }
        // A straggler from the front rank runs at tick 300 and stands again at 400, long before
        // the collapse: he must not count as its first sign.
        events.push(posture_event(300, 5, Posture::Turned));
        events.push(posture_event(400, 5, Posture::Steady));
        // One loss before the break, three after.
        let wound = |tick, man| Event {
            tick,
            man: ManId(man),
            pos: Vec2::ZERO,
            kind: EventKind::Wound { to: Wound::Dead },
            causes: [Cause::Bout; 3],
        };
        events.extend([
            wound(500, 1),
            wound(1300, 30),
            wound(1310, 31),
            wound(1320, 32),
        ]);
        events.sort_by_key(|e| (e.tick, e.man));
        // Replay into samples every man step.
        let mut k = 0;
        for tick in (0..2000).step_by(5) {
            while k < events.len() && events[k].tick <= tick {
                crate::runner::apply(&mut bodies, &events[k]);
                k += 1;
            }
            rec.record(tick, &bodies);
        }
        let sum = summarize(&Inputs {
            initial: &initial,
            placed: &placed,
            layout: &layout,
            samples: &rec.samples,
            events: &events,
            end_tick: 2000,
            break_share: 0.4,
        });
        assert_eq!(sum.loser, Some(Side::A));
        assert_eq!(sum.contact, Some(100));
        // 40% of 40 men is 16 turned: the 16th turn is at tick 1150.
        assert_eq!(sum.side_break[0], Some(1150));
        assert_eq!(sum.first_turn_depth, Some(1.0));
        assert!(sum.spread_rho.unwrap() > 0.9);
        assert_eq!(sum.stragglers, 1);
        assert_eq!(sum.loser_after_break, Some(0.75));
        assert_eq!(sum.time_to_break_s, Some(secs(1050)));
        assert_eq!(sum.bouts_formed, 23);
        assert_eq!(sum.lethality_per_bout, Some(0.0));
        // One hit on A's 10 files in 1050 ticks (52.5 s) of formed fighting.
        let c = sum.hits_per_file_hour[0].unwrap();
        assert!((c - 1.0 / 10.0 / (52.5 / 3600.0)).abs() < 1e-9);
        // The gap at setup: centres 8 m apart less two half-depths of 1.5 m.
        assert_eq!(rec.samples[0].gap_mm, Some(5_000));
    }
}
