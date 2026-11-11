//! The visual QC pass (docs/slice0/qc-pass.md) over battles of the nominal set.
//!
//!   cannae-qc scan --out DIR [--jobs 4] [--engine field|pairwise] [--variant V] [--rung R]
//!                  [--seed S] [--dry]
//!   cannae-qc clip --engine E --rung R --variant V --seed S --from SEC --to SEC
//!                  --centre X,Y [--radius 15000] [--men A,B,C]
//!
//! `scan` runs the battles and writes one line of JSON for each flag to DIR/flags.jsonl and one
//! for each battle to DIR/battles.jsonl. It prints no count. `--engine`, `--variant`, `--rung`
//! and `--seed` each narrow the 320 battles to those that match. With `--dry` it runs the
//! battles without the detectors and the dormancy checks, to time them.
//!
//! `clip` runs one battle again and prints, as JSON, the men near a place over a stretch of
//! time. A battle is the same battle every time it is run, so what surrounds a flag is made
//! when the flag is looked at, and is not kept for the thousands that are not.

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::Standard;
use cannae_core::{BodyId, EventKind, ManId, Vec2};
use cannae_harness::Dormancy;
use cannae_score::Engine;
use cannae_view::detect::{Detector, Detectors, Flag, Sample, Scene};
use cannae_view::{Config, build, engine_name, engine_of, esc};
use std::fmt::Write as _;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

/// The variants in the order the batch runs them.
const ORDER: [&str; 8] = [
    "base", "veterans", "recruits", "flank", "shock", "volleys", "backed", "relief",
];
const SEEDS: std::ops::RangeInclusive<u64> = 1..=5;
/// Seconds of a man step.
const STEP_S: f64 = 0.25;

/// How long a battle is watched: `relief` and `backed` longer, since their second line acts
/// late.
fn watched_s(variant: &str) -> u32 {
    if matches!(variant, "relief" | "backed") {
        1_800
    } else {
        600
    }
}

fn ticks(seconds: u32) -> u32 {
    seconds * (1000 / BASE_TICK.0)
}

fn said(e: Box<dyn std::any::Any + Send>) -> String {
    e.downcast_ref::<String>()
        .cloned()
        .or_else(|| e.downcast_ref::<&str>().map(|s| s.to_string()))
        .unwrap_or_else(|| "a panic with no message".to_string())
}

struct Watched {
    flags: Vec<Flag>,
    hashes: Vec<u64>,
    men: usize,
}

/// One battle from its start for `seconds`, with the detectors on it unless `dry`.
fn watch(c: &Config, seconds: u32, dry: bool) -> Result<Watched, String> {
    let built = build(c)?;
    let mut battle = built.battle;
    let end = ticks(seconds).min(battle.end_tick);
    let b = battle.live.bodies();
    let n = b.len();
    let mut det = Detectors::new(Scene::new(&b.side, &b.body, &built.bodies, &b.pos));
    let mut slots = vec![None; n];
    let mut states = Vec::with_capacity(built.bodies.len());
    let mut seen = 0;
    while battle.live.tick() < end {
        if !battle.live.step() || dry {
            continue;
        }
        let live = &battle.live;
        for (i, s) in slots.iter_mut().enumerate() {
            *s = live.slot(ManId(i as u32));
        }
        states.clear();
        for k in 0..built.bodies.len() {
            states.push(live.body(BodyId(k as u32)).ok_or("a body with no frame")?);
        }
        let (b, events) = (live.bodies(), live.events());
        det.step(&Sample {
            pos: &b.pos,
            facing: &b.facing,
            posture: &b.posture,
            slot: &slots,
            state: &states,
            events: &events[seen..],
            fought: live.fought(),
        });
        seen = events.len();
    }
    Ok(Watched {
        flags: det.finish(),
        hashes: battle.live.hashes().to_vec(),
        men: n,
    })
}

/// The same battle with dormant men treated as `mode` says, for `seconds`. A failed assertion
/// comes back as its message.
fn again(c: &Config, mode: Dormancy, seconds: u32) -> Result<Vec<u64>, String> {
    let mut battle = build(c)?.battle;
    battle.live.set_dormancy(mode);
    let end = ticks(seconds).min(battle.end_tick);
    catch_unwind(AssertUnwindSafe(move || {
        while battle.live.tick() < end {
            battle.live.step();
        }
        battle.live.hashes().to_vec()
    }))
    .map_err(said)
}

/// `Ok` if `other` is the start of `normal`, hash for hash.
fn same_start(normal: &[u64], other: &[u64]) -> Result<(), String> {
    match normal.iter().zip(other).position(|(a, b)| a != b) {
        Some(k) => Err(format!(
            "the state differs from the normal run's after man step {k}, at {} s",
            k as f64 * STEP_S
        )),
        None if other.len() > normal.len() => Err("it ran longer than the normal run".into()),
        None => Ok(()),
    }
}

struct Task {
    engine: Engine,
    rung: u8,
    variant: &'static str,
    seed: u64,
}

impl Task {
    fn config(&self) -> Config {
        Config {
            engine: self.engine,
            rung: self.rung,
            variant: self.variant.to_string(),
            seed: self.seed,
        }
    }

    fn json(&self) -> String {
        format!(
            "\"engine\":\"{}\",\"rung\":{},\"variant\":\"{}\",\"seed\":{}",
            engine_name(self.engine),
            self.rung,
            self.variant,
            self.seed
        )
    }
}

/// The lines for the two files: the battle's flags, and the battle itself.
fn one(t: &Task, dry: bool) -> (String, String) {
    let c = t.config();
    let started = Instant::now();
    let seconds = watched_s(t.variant);
    let watched = catch_unwind(AssertUnwindSafe(|| watch(&c, seconds, dry)))
        .unwrap_or_else(|e| Err(format!("the battle panicked: {}", said(e))));
    let w = match watched {
        Ok(w) => w,
        Err(e) => {
            return (
                String::new(),
                format!("{{{},\"error\":\"{}\"}}\n", t.json(), esc(&e)),
            );
        }
    };
    let mut flags = String::new();
    for f in &w.flags {
        let from = f.first.saturating_sub(f.detector.lead_in());
        let _ = writeln!(
            flags,
            "{{{},\"detector\":\"{}\",\"n\":{},\"kind\":\"{}\",\"from_s\":{},\"first_s\":{},\
             \"last_s\":{},\"men\":{:?},\"body\":{},\"at\":[{},{}],\"worst\":{},\"note\":\"{}\"}}",
            t.json(),
            f.detector.name(),
            f.detector as u8,
            f.kind,
            from as f64 * STEP_S,
            f.first as f64 * STEP_S,
            f.last as f64 * STEP_S,
            f.men,
            f.body.map_or(-1, |b| b as i64),
            f.at.x,
            f.at.y,
            f.worst,
            esc(&f.note)
        );
    }
    let mut checks = String::new();
    if !dry {
        // 21: the first 300 s with the runner's check on. 22: the first 120 s deciding for
        // every man. Either must also give the normal run's state at every man step.
        for (name, mode, seconds) in [
            ("dormancy-check", Dormancy::Check, 300),
            ("dormancy-equal", Dormancy::EvaluateAll, 120),
        ] {
            let result = again(&c, mode, seconds).and_then(|h| same_start(&w.hashes, &h));
            let _ = write!(
                checks,
                ",\"{name}\":\"{}\"",
                result.map_or_else(|e| esc(&e), |()| "ok".to_string())
            );
        }
    }
    let battle = format!(
        "{{{},\"seconds\":{seconds},\"men\":{},\"man_steps\":{},\"wall_ms\":{},\"flags\":{}{checks}}}\n",
        t.json(),
        w.men,
        w.hashes.len(),
        started.elapsed().as_millis(),
        w.flags.len()
    );
    (flags, battle)
}

fn scan(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let (mut out, mut jobs, mut engines, mut dry) =
        (None, 4usize, vec![Engine::Field, Engine::Pairwise], false);
    let (mut only_variant, mut only_rung, mut only_seed) = (None, None, None);
    while let Some(a) = args.next() {
        match (a.as_str(), args.next()) {
            ("--dry", rest) => {
                dry = true;
                if rest.is_some() {
                    return Err("--dry goes last".into());
                }
            }
            ("--out", Some(v)) => out = Some(v),
            ("--jobs", Some(v)) => jobs = v.parse().map_err(|_| "--jobs wants a number")?,
            ("--engine", Some(v)) => engines = vec![engine_of(&v).ok_or("unknown engine")?],
            ("--variant", Some(v)) => only_variant = Some(v),
            ("--rung", Some(v)) => {
                only_rung = Some(v.parse::<u8>().map_err(|_| "--rung wants 0 to 3")?)
            }
            ("--seed", Some(v)) => {
                only_seed = Some(v.parse::<u64>().map_err(|_| "--seed wants a number")?)
            }
            _ => return Err(format!("unknown argument {a:?}")),
        }
    }
    let out = out.ok_or("scan needs --out DIR")?;
    std::fs::create_dir_all(&out).map_err(|e| format!("{out}: {e}"))?;
    let mut tasks = Vec::new();
    for &engine in &engines {
        for variant in ORDER {
            for rung in 0..=3 {
                for seed in SEEDS {
                    tasks.push(Task {
                        engine,
                        rung,
                        variant,
                        seed,
                    });
                }
            }
        }
    }
    tasks.retain(|t| {
        only_variant.as_deref().is_none_or(|v| v == t.variant)
            && only_rung.is_none_or(|r| r == t.rung)
            && only_seed.is_none_or(|s| s == t.seed)
    });
    if tasks.is_empty() {
        return Err("no battle matches".into());
    }
    // Plain threads: a battle on the pairwise engine hands its physics to a pool of its own,
    // and a pool's worker that waits on another pool takes up more of its own work meanwhile.
    let next = AtomicUsize::new(0);
    let done = AtomicUsize::new(0);
    let made: Mutex<Vec<Option<(String, String)>>> =
        Mutex::new(tasks.iter().map(|_| None).collect());
    let started = Instant::now();
    std::thread::scope(|scope| {
        for _ in 0..jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let k = next.fetch_add(1, Ordering::Relaxed);
                    let Some(task) = tasks.get(k) else { break };
                    let lines = one(task, dry);
                    made.lock().expect("no thread panicked holding it")[k] = Some(lines);
                    let n = done.fetch_add(1, Ordering::Relaxed) + 1;
                    eprintln!(
                        "{n} of {} battles, {} s",
                        tasks.len(),
                        started.elapsed().as_secs()
                    );
                }
            });
        }
    });
    let (mut flags, mut battles) = (String::new(), String::new());
    for lines in made.into_inner().expect("no thread panicked holding it") {
        let (f, b) = lines.expect("every battle was run");
        flags.push_str(&f);
        battles.push_str(&b);
    }
    let write = |name: &str, text: &str| {
        let path = format!("{out}/{name}");
        std::fs::write(&path, text).map_err(|e| format!("{path}: {e}"))
    };
    if !dry {
        write("flags.jsonl", &flags)?;
    }
    write("battles.jsonl", &battles)?;
    let names: Vec<&str> = Detector::ALL.iter().map(|d| d.name()).collect();
    println!(
        "{} battles in {} s, written to {out}. Detectors: {}",
        tasks.len(),
        started.elapsed().as_secs(),
        names.join(", ")
    );
    Ok(())
}

fn clip(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let (mut engine, mut rung, mut variant, mut seed) = (None, None, None, None);
    let (mut from, mut to, mut centre, mut radius) = (None, None, None, 15_000i64);
    let mut men: Vec<u32> = Vec::new();
    let pair = |v: &str| -> Option<Vec2> {
        let (x, y) = v.split_once(',')?;
        Some(Vec2::new(x.parse().ok()?, y.parse().ok()?))
    };
    while let Some(a) = args.next() {
        let v = args.next().ok_or_else(|| format!("{a} needs a value"))?;
        let bad = || format!("{a}: bad value {v:?}");
        match a.as_str() {
            "--engine" => engine = Some(engine_of(&v).ok_or_else(bad)?),
            "--rung" => rung = Some(v.parse::<u8>().map_err(|_| bad())?),
            "--variant" => variant = Some(v),
            "--seed" => seed = Some(v.parse::<u64>().map_err(|_| bad())?),
            "--from" => from = Some(v.parse::<f64>().map_err(|_| bad())?),
            "--to" => to = Some(v.parse::<f64>().map_err(|_| bad())?),
            "--centre" => centre = Some(pair(&v).ok_or_else(bad)?),
            "--radius" => radius = v.parse().map_err(|_| bad())?,
            "--men" => {
                men = v
                    .split(',')
                    .map(|m| m.parse().map_err(|_| bad()))
                    .collect::<Result<_, _>>()?
            }
            _ => return Err(format!("unknown argument {a:?}")),
        }
    }
    let c = Config {
        engine: engine.ok_or("clip needs --engine")?,
        rung: rung.ok_or("clip needs --rung")?,
        variant: variant.ok_or("clip needs --variant")?,
        seed: seed.ok_or("clip needs --seed")?,
    };
    let (from, to) = (
        from.ok_or("clip needs --from")?,
        to.ok_or("clip needs --to")?,
    );
    let centre = centre.ok_or("clip needs --centre")?;
    let built = build(&c)?;
    let mut battle = built.battle;
    let end = ((to * 1000.0) as u32 / BASE_TICK.0 + 1).min(battle.end_tick);
    let mut o = String::new();
    let _ = write!(
        o,
        "{{\"engine\":\"{}\",\"rung\":{},\"variant\":\"{}\",\"seed\":{},\"centre\":[{},{}],\
         \"radius\":{radius},\"men\":{men:?},\"meta\":{},\"frames\":[",
        engine_name(c.engine),
        c.rung,
        esc(&c.variant),
        c.seed,
        centre.x,
        centre.y,
        built.meta
    );
    let (mut seen, mut first) = (0, true);
    while battle.live.tick() < end {
        let tick = battle.live.tick();
        if !battle.live.step() {
            continue;
        }
        let live = &battle.live;
        let (b, events) = (live.bodies(), live.events());
        let t = tick as f64 * BASE_TICK.0 as f64 / 1000.0;
        let new = &events[seen..];
        seen = events.len();
        if t < from {
            continue;
        }
        if !first {
            o.push(',');
        }
        first = false;
        let _ = write!(o, "{{\"t\":{t},\"men\":[");
        let near =
            |i: usize| b.pos[i].dist_sq(centre) <= radius * radius || men.contains(&(i as u32));
        for (k, i) in (0..b.len()).filter(|&i| near(i)).enumerate() {
            let slot = live.slot(ManId(i as u32));
            let _ = write!(
                o,
                "{}[{i},{},{},{},{},{},{}]",
                if k > 0 { "," } else { "" },
                b.pos[i].x,
                b.pos[i].y,
                b.facing[i].0,
                b.posture[i] as u8,
                slot.is_some_and(|s| s.in_place) as u8,
                slot.map_or("null".to_string(), |s| format!(
                    "[{},{}]",
                    s.target.x, s.target.y
                )),
            );
        }
        o.push_str("],\"bouts\":[");
        let bouts = new.iter().filter_map(|e| match e.kind {
            EventKind::Bout {
                attacker,
                defender,
                attacker_took,
            } => Some((attacker.0, e.man.0, defender as u8, attacker_took as u8)),
            _ => None,
        });
        for (k, (a, d, took, gave)) in bouts.enumerate() {
            let _ = write!(o, "{}[{a},{d},{took},{gave}]", if k > 0 { "," } else { "" });
        }
        o.push_str("],\"bodies\":[");
        for k in 0..built.bodies.len() {
            let st = live.body(BodyId(k as u32)).ok_or("a body with no frame")?;
            let carrier = match st.standard {
                Standard::Carried(m) => m.0 as i64,
                Standard::Down(_) => -1,
            };
            let _ = write!(
                o,
                "{}[\"{:?}\",{carrier},{}]",
                if k > 0 { "," } else { "" },
                st.order,
                st.leader.map_or(-1, |l| l.0 as i64)
            );
        }
        o.push_str("]}");
    }
    o.push_str("]}");
    println!("{o}");
    Ok(())
}

fn main() {
    let mut args = std::env::args().skip(1);
    let done = match args.next().as_deref() {
        Some("scan") => scan(args),
        Some("clip") => clip(args),
        _ => Err(
            "usage: cannae-qc scan --out DIR [--jobs 4] [--engine E] [--dry]\n       \
                  cannae-qc clip --engine E --rung R --variant V --seed S --from SEC --to SEC \
                  --centre X,Y [--radius MM] [--men A,B]"
                .to_string(),
        ),
    };
    if let Err(e) = done {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
