//! Replays for the Slice 0 report (docs/slice0/report-outline.md, section 5, "How the discs'
//! passage is shown in the relief"). Nothing here is scored, and no record is changed.
//!
//!   cannae-replay drill --engine field|pairwise
//!   cannae-replay reliefs --records DIR --engine E --rung R --freeze-seed F --out FILE
//!                         [--max 100] [--jobs 4]
//!
//! `drill` runs the relief alone, as the relief's own test does (`cannae-score`,
//! `tests/relief.rs`): the relief patch with its reliefs allowed from half a minute, men whom
//! nothing moves, the nominal set at rung 2, seed 3. For each relief it prints how long the
//! fresh century took to come up and the tired one to fall back, how far each went, and how
//! near a man of one came to a man of the other.
//!
//! `reliefs` runs the relieved runs of a cell's `relief` records again, lowest set first and
//! then lowest seed, until `--max` reliefs have been ordered in them. A run is counted only if
//! it is the battle of its record: the scorer's own run of that set and seed gives the record
//! again, all but the wall time, and the battle stepped here has the same events. For each
//! relief it writes one line of JSON to FILE: the times of the drill, whether it was pressed
//! (a bout began with a man of either century between the order and the tired century's
//! halt), whether it was unfinished at the run's end, and how the run fared beside its
//! `backed` twin. It prints the number of runs and of those not counted, and no outcome.
//!
//! It is for after the scoring. It asks for the records because it replays only a run that
//! has one.

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::Order;
use cannae_core::{BodyId, EventKind, ManId, Posture};
use cannae_harness::Scenario;
use cannae_harness::metrics::fortune;
use cannae_man::{Spread, Troop};
use cannae_score::draw::{Cell, drawn, nominal};
use cannae_score::report::ROMANS;
use cannae_score::{Engine, Record, Spec, record_file, run_logged};
use cannae_view::{Battle, Config, build_from, engine_of, scenario_text};
use std::fmt::Write as _;
use std::path::Path;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Men whom nothing moves: they follow their slots and do nothing else. The troops of the
/// relief's own test.
const STOLID: Troop = Troop {
    nerve: Spread { mean: 8_192, sd: 0 },
    ardour: Spread {
        mean: -8_192,
        sd: 0,
    },
    front_ardour: 0,
    leader_ardour: 0,
    ..Troop::HASTATI
};

fn seconds(tick: u32) -> f64 {
    f64::from(tick) * f64::from(BASE_TICK.0) / 1000.0
}

/// The reliefs of a scenario as pairs of body numbers: the tired body and the fresh one.
fn pairs(s: &Scenario) -> Result<Vec<(u32, u32)>, String> {
    let body = |name: &str| {
        s.bodies
            .iter()
            .position(|b| b.name == name)
            .map(|i| i as u32)
            .ok_or_else(|| format!("no body {name:?}"))
    };
    s.reliefs
        .iter()
        .map(|r| Ok((body(&r.of)?, body(&r.by)?)))
        .collect()
}

/// One relief as the frames ran it, in ticks.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Drill {
    /// The fresh body was ordered up.
    ordered: Option<u32>,
    /// The fresh body had the front, and the tired one was ordered back.
    up: Option<u32>,
    /// The tired body halted behind it.
    halted: Option<u32>,
}

impl Drill {
    /// Reads the two bodies' orders after a step. The second line of the patch has no order
    /// but its relief, and the first none but to stand, so an advance of the fresh body is the
    /// relief's order and a retirement of the tired one is its second half.
    fn watch(&mut self, b: &Battle, tired: u32, fresh: u32) {
        let (Some(t), Some(f)) = (b.live.body(BodyId(tired)), b.live.body(BodyId(fresh))) else {
            return;
        };
        if self.ordered.is_none() && f.order == Order::Advance {
            self.ordered = Some(f.order_tick);
        }
        if self.ordered.is_some() && self.up.is_none() && t.order == Order::Retire {
            self.up = Some(t.order_tick);
        }
        if self.up.is_some() && self.halted.is_none() && t.order == Order::Hold {
            self.halted = Some(t.order_tick);
        }
    }
}

/// Where a body's standing men are along the line of advance: the rearmost and the foremost.
fn span(b: &Battle, of: &[Option<BodyId>], body: u32) -> Option<(i32, i32)> {
    let bodies = b.live.bodies();
    let xs = (0..bodies.len())
        .filter(|&i| of[i] == Some(BodyId(body)) && bodies.posture[i] != Posture::Down)
        .map(|i| bodies.pos[i].x);
    Some((xs.clone().min()?, xs.max()?))
}

/// The least distance between a standing man of one body and a standing man of the other, mm.
fn nearest(b: &Battle, of: &[Option<BodyId>], one: u32, other: u32) -> Option<f64> {
    let bodies = b.live.bodies();
    let men = |body: u32| {
        (0..bodies.len())
            .filter(move |&i| of[i] == Some(BodyId(body)) && bodies.posture[i] != Posture::Down)
    };
    let mut least: Option<i64> = None;
    for i in men(one) {
        for j in men(other) {
            let d = bodies.pos[i].dist_sq(bodies.pos[j]);
            if least.is_none_or(|l| d < l) {
                least = Some(d);
            }
        }
    }
    least.map(|d| (d as f64).sqrt())
}

fn drill(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let mut engine = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--engine" => engine = args.next().as_deref().and_then(engine_of),
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    let engine = engine.ok_or("drill needs --engine field|pairwise")?;
    let text = scenario_text("relief")?;
    let soon = text.replace("not_before_s = 1200", "not_before_s = 30");
    if soon == text {
        return Err("the relief patch no longer says not_before_s = 1200".to_string());
    }
    let s = Scenario::from_toml(&soon)?;
    let d = nominal(Cell { engine, rung: 2 })?;
    let c = Config {
        engine,
        rung: 2,
        variant: "relief".to_string(),
        seed: 3,
    };
    let troops = vec![STOLID; s.bodies.len()];
    let mut b = build_from(&c, &s, &d, Some(&troops))?.battle;
    let of = b.live.bodies().body.clone();
    let pairs = pairs(&s)?;
    let mut drills = vec![Drill::default(); pairs.len()];
    // Where the fresh body's front and the tired body's rear stood at the order, at the
    // half-way mark and at the halt, and the nearest two men of the two bodies came.
    let mut at = vec![[None::<(i32, i32)>; 3]; pairs.len()];
    let mut least = vec![None::<f64>; pairs.len()];
    let end = 180 * 1000 / BASE_TICK.0;
    while b.live.tick() < end {
        let man_step = b.live.step();
        for (k, &(tired, fresh)) in pairs.iter().enumerate() {
            let before = drills[k];
            drills[k].watch(&b, tired, fresh);
            let now = drills[k];
            let both = || Some((span(&b, &of, fresh)?.1, span(&b, &of, tired)?.0));
            if before.ordered != now.ordered {
                at[k][0] = both();
            }
            if before.up != now.up {
                at[k][1] = both();
            }
            if before.halted != now.halted {
                at[k][2] = both();
            }
            if man_step && now.ordered.is_some() && now.halted.is_none() {
                let d = nearest(&b, &of, tired, fresh);
                if let Some(d) = d
                    && least[k].is_none_or(|l| d < l)
                {
                    least[k] = Some(d);
                }
            }
        }
        if drills.iter().all(|d| d.halted.is_some()) {
            break;
        }
    }
    if !b.live.events().is_empty() {
        return Err(format!(
            "{} events in a drill of men whom nothing moves",
            b.live.events().len()
        ));
    }
    println!(
        "the relief alone, {engine:?}, nominal set, rung 2, seed 3; a man walks {} mm/s",
        d.speeds.walk
    );
    for (k, dr) in drills.iter().enumerate() {
        let (Some(o), Some(u), Some(h)) = (dr.ordered, dr.up, dr.halted) else {
            println!("  relief {k}: not finished in 180 s: {dr:?}");
            continue;
        };
        let ([Some(a), Some(m), Some(z)], Some(near)) = (at[k], least[k]) else {
            println!("  relief {k}: a body has no man standing");
            continue;
        };
        let (came, went) = (m.0 - a.0, m.1 - z.1);
        let walk = f64::from(d.speeds.walk);
        println!(
            "  relief {k}: ordered at {:.2} s; the fresh century up after {:.2} s, its front \
             {came} mm forward (a free walk: {:.2} s); the tired one halted {:.2} s later, its \
             rear {went} mm back (a free walk: {:.2} s); nearest two men of the two {:.0} mm",
            seconds(o),
            seconds(u - o),
            f64::from(came) / walk,
            seconds(h - u),
            f64::from(went) / walk,
            near
        );
    }
    Ok(())
}

/// The records of one file, or none if the file is not there.
fn records(path: &Path) -> Result<Vec<Record>, String> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .enumerate()
        .map(|(n, l)| {
            serde_json::from_str(l).map_err(|e| format!("{}, line {}: {e}", path.display(), n + 1))
        })
        .collect()
}

/// What one replayed run says of its reliefs: a line of JSON for each, or why it is not
/// counted.
fn replay(
    s: &Scenario,
    pairs: &[(u32, u32)],
    cell: Cell,
    freeze: u64,
    r: &Record,
    twin: Option<&Record>,
) -> Result<Vec<String>, String> {
    let d = drawn(freeze, cell, r.draw)?;
    let spec = Spec {
        seed: r.seed,
        duration_s: s.duration_s,
        stop_at_rout_end: true,
    };
    let (run, events) = run_logged(s, &d, spec)?;
    let mut again = Record::new(cell, "relief", Some(freeze), false, &run);
    again.wall_ms = r.wall_ms;
    if again != *r {
        return Err("the scorer's run does not give the record again".to_string());
    }
    let c = Config {
        engine: cell.engine,
        rung: cell.rung,
        variant: "relief".to_string(),
        seed: r.seed,
    };
    let mut b = build_from(&c, s, &d, None)?.battle;
    let of = b.live.bodies().body.clone();
    let mut drills = vec![Drill::default(); pairs.len()];
    let end = run.sim_s * 1000 / BASE_TICK.0;
    while b.live.tick() < end {
        b.live.step();
        for (k, &(tired, fresh)) in pairs.iter().enumerate() {
            drills[k].watch(&b, tired, fresh);
        }
    }
    if b.live.events() != events.as_slice() {
        return Err("the battle stepped here is not the scorer's".to_string());
    }
    let ordered: Vec<Option<u32>> = drills
        .iter()
        .map(|d| d.ordered.map(|t| t * BASE_TICK.0 / 1000))
        .collect();
    if ordered != r.relieved_s {
        return Err(format!(
            "the reliefs were ordered at {ordered:?} here and at {:?} in the record",
            r.relieved_s
        ));
    }
    let fared = match twin {
        None => "none",
        Some(t) => {
            let (relieved, backed) = (fortune(&r.summary, ROMANS), fortune(&t.summary, ROMANS));
            match relieved.partial_cmp(&backed) {
                Some(std::cmp::Ordering::Greater) => "better",
                Some(std::cmp::Ordering::Less) => "worse",
                _ => "same",
            }
        }
    };
    let in_pair = |m: ManId, tired: u32, fresh: u32| {
        matches!(of.get(m.0 as usize), Some(Some(BodyId(x))) if *x == tired || *x == fresh)
    };
    let mut lines = Vec::new();
    for (k, (dr, &(tired, fresh))) in drills.iter().zip(pairs).enumerate() {
        let Some(o) = dr.ordered else { continue };
        let until = dr.halted.unwrap_or(end);
        let pressed = events.iter().any(|e| {
            (o..=until).contains(&e.tick)
                && matches!(e.kind, EventKind::Bout { attacker, .. }
                    if in_pair(attacker, tired, fresh) || in_pair(e.man, tired, fresh))
        });
        let time = |t: Option<u32>, from: Option<u32>| match (t, from) {
            (Some(t), Some(f)) => format!("{:.2}", seconds(t - f)),
            _ => "null".to_string(),
        };
        let mut l = String::new();
        let _ = write!(
            l,
            "{{\"draw\":{},\"seed\":{},\"relief\":{k},\"ordered_s\":{:.2},\"up_after_s\":{},\
             \"halted_after_s\":{},\"pressed\":{pressed},\"unfinished\":{},\"twin\":\"{fared}\"}}",
            r.draw,
            r.seed,
            seconds(o),
            time(dr.up, dr.ordered),
            time(dr.halted, dr.up),
            dr.halted.is_none()
        );
        lines.push(l);
    }
    Ok(lines)
}

fn reliefs(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let (mut dir, mut engine, mut rung, mut freeze, mut out) = (None, None, None, None, None);
    let (mut max, mut jobs) = (100usize, 4usize);
    while let Some(a) = args.next() {
        let mut value = || args.next().ok_or_else(|| format!("{a} needs a value"));
        match a.as_str() {
            "--records" => dir = Some(value()?),
            "--engine" => engine = engine_of(&value()?),
            "--rung" => rung = value()?.parse::<u8>().ok(),
            "--freeze-seed" => freeze = value()?.parse::<u64>().ok(),
            "--out" => out = Some(value()?),
            "--max" => max = value()?.parse().map_err(|e| format!("--max: {e}"))?,
            "--jobs" => jobs = value()?.parse().map_err(|e| format!("--jobs: {e}"))?,
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    let (Some(dir), Some(engine), Some(rung), Some(freeze), Some(out)) =
        (dir, engine, rung, freeze, out)
    else {
        return Err("reliefs needs --records, --engine, --rung, --freeze-seed and --out".into());
    };
    if Path::new(&out).exists() {
        return Err(format!("{out} is there already"));
    }
    let cell = Cell { engine, rung };
    let dir = Path::new(&dir);
    let scored = |r: &Record| !r.whole && r.freeze_seed == Some(freeze);
    let mut relieved: Vec<Record> = records(&dir.join(record_file(cell, "relief")))?
        .into_iter()
        .filter(|r| scored(r) && r.relieved_s.iter().any(Option::is_some))
        .collect();
    relieved.sort_by_key(|r| (r.draw, r.seed));
    let backed: Vec<Record> = records(&dir.join(record_file(cell, "backed")))?
        .into_iter()
        .filter(scored)
        .collect();
    // The runs to replay: in order, until `max` reliefs have been ordered in them.
    let mut ordered = 0;
    let chosen: Vec<&Record> = relieved
        .iter()
        .take_while(|r| {
            let more = ordered < max;
            ordered += r.relieved_s.iter().filter(|t| t.is_some()).count();
            more
        })
        .collect();
    let s = Scenario::from_toml(&scenario_text("relief")?)?;
    let pairs = pairs(&s)?;
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<(usize, Result<Vec<String>, String>)>> = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(r) = chosen.get(i) else { break };
                    let twin = backed
                        .iter()
                        .find(|t| t.draw == r.draw && t.seed == r.seed);
                    let said = replay(&s, &pairs, cell, freeze, r, twin);
                    done.lock().unwrap().push((i, said));
                }
            });
        }
    });
    let mut done = done.into_inner().unwrap();
    done.sort_by_key(|(i, _)| *i);
    let (mut text, mut not_counted) = (String::new(), 0);
    for (i, said) in &done {
        match said {
            Ok(lines) => {
                for l in lines {
                    text.push_str(l);
                    text.push('\n');
                }
            }
            Err(why) => {
                not_counted += 1;
                let _ = writeln!(
                    text,
                    "{{\"draw\":{},\"seed\":{},\"not_counted\":\"{}\"}}",
                    chosen[*i].draw,
                    chosen[*i].seed,
                    why.replace('"', "'")
                );
            }
        }
    }
    std::fs::write(&out, text).map_err(|e| format!("{out}: {e}"))?;
    println!(
        "{engine:?} rung {rung}: {} relieved runs on record, {} replayed, {not_counted} not \
         counted; {out}",
        relieved.len(),
        done.len()
    );
    Ok(())
}

fn main() {
    let mut args = std::env::args().skip(1);
    let done = match args.next().as_deref() {
        Some("drill") => drill(args),
        Some("reliefs") => reliefs(args),
        _ => Err("usage: cannae-replay drill --engine E\n       \
                  cannae-replay reliefs --records DIR --engine E --rung R --freeze-seed F \
                  --out FILE [--max 100] [--jobs 4]"
            .to_string()),
    };
    if let Err(e) = done {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
