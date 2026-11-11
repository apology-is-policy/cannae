//! Replays for the Slice 0 report (docs/slice0/report-outline.md: section 5, "How the discs'
//! passage is shown in the relief", and section 6, "A century and its centurion: the count").
//! Nothing here is scored, and no record is changed.
//!
//!   cannae-replay drill --engine field|pairwise [--fight]
//!   cannae-replay reliefs --records DIR --engine E --rung R --freeze-seed F --out FILE
//!                         [--max 100] [--jobs 4]
//!   cannae-replay leaders --records DIR --engine E --rung R --freeze-seed F --out FILE
//!                         [--jobs 4]
//!
//! `drill` runs the relief alone, as the relief's own test does (`cannae-score`,
//! `tests/relief.rs`): the relief patch with its reliefs allowed from half a minute, men whom
//! nothing moves, the nominal set at rung 2, seed 3. For each relief it prints how long the
//! fresh century took to come up and the tired one to fall back, how far each went, when the
//! men were back in their places, how near a man of one came to a man of the other, and
//! whether it jammed. "Back in their places" is asked of the men who were in their places
//! when the relief was ordered, and of no others. With `--fight` the men are the troops the patch names, made unable to
//! flee as in that test's fight, so the relief comes in the first lull of a real fight; it
//! then says whether each relief was pressed too. That one is for checking the marks.
//!
//! `reliefs` runs the relieved runs of a cell's `relief` records again, lowest set first and
//! then lowest seed, until `--max` reliefs have been ordered in them. For each relief it
//! writes one line of JSON to FILE: the times of the drill and three marks, each taken
//! between the order and the tired century's halt, or the run's end if it never halted.
//! - Pressed: a bout began with a man of either century.
//! - Jammed: at some man step a man of the two centuries had eight or more others on their
//!   feet within a metre of him, one of them at least of the other century of the two. With
//!   himself that is nine on 3.14 m², the passage bench's own measure of density at its
//!   lowest jam (`cannae-harness`, `passage.rs`).
//! - Unfinished, by reason: one of the two centuries broke, or the run ended.
//!
//! It also writes how the run fared beside its `backed` twin.
//!
//! `leaders` runs a cell's `base` records of sets 0, 10, .. 190 again and writes one line for
//! each fall of a body's leader (the first in each body) and of a file leader (every one, by
//! his role at the tick he went down), in a front body not yet broken: when, how long until
//! his body broke, the share of the body's standing men in flight at that tick, and the
//! succession delay of the body's tie in that set.
//!
//! A run is counted only if it is the battle of its record: the scorer's own run of that set
//! and seed gives the record again, all but the wall time, and the battle stepped here has
//! the same events. Both commands print the number of runs and of those not counted, and no
//! outcome.
//!
//! They are for after the scoring. They ask for the records because they replay only a run
//! that has one.

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::Order;
use cannae_core::{BodyId, Event, EventKind, ManId, Posture, Role};
use cannae_harness::Scenario;
use cannae_harness::metrics::fortune;
use cannae_man::{Spread, Troop};
use cannae_score::draw::{Cell, Drawn, drawn, nominal};
use cannae_score::report::ROMANS;
use cannae_score::{Record, Run, Spec, record_file, run_logged};
use cannae_view::{Battle, BodyInfo, Config, build_from, engine_of, scenario_text};
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

/// A jam, as the passage bench measures density: this many others on their feet within this
/// distance of a man. Eight others and himself on a disc of one metre are 2.9 per m². One of
/// them must be of the other century of the two: a front in close fight is as dense, and
/// nobody passes through anybody there.
const JAM_OTHERS: usize = 8;
const JAM_WITHIN_MM: i64 = 1_000;

fn seconds(tick: u32) -> f64 {
    f64::from(tick) * f64::from(BASE_TICK.0) / 1000.0
}

fn scenario(variant: &str, text: &str) -> Result<Scenario, String> {
    Scenario::from_toml(text).map_err(|e| format!("{variant}: {e}"))
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

/// The standing men of one body, by the body each man began in.
fn standing<'a>(
    b: &'a Battle,
    of: &'a [Option<BodyId>],
    body: u32,
) -> impl Iterator<Item = usize> + 'a {
    let bodies = b.live.bodies();
    (0..bodies.len())
        .filter(move |&i| of[i] == Some(BodyId(body)) && bodies.posture[i] != Posture::Down)
}

/// One relief as it went, in ticks.
#[derive(Clone, Debug, Default, PartialEq)]
struct Drill {
    /// The fresh body was ordered up.
    ordered: Option<u32>,
    /// The fresh body had the front, and the tired one was ordered back.
    up: Option<u32>,
    /// The tired body halted behind it.
    halted: Option<u32>,
    /// The men in their places when the relief was ordered: the tired body's and the fresh
    /// one's. They are the measure of what follows. The frames never take a man out of his
    /// body, so a man who fled long ago is still of it, and he is out of his place as long as
    /// he is adrift: for a man still in flight, the rest of the run. A count over every man
    /// on his feet would call a thinned century never back.
    placed: (Vec<usize>, Vec<usize>),
    /// How many of those, of both bodies and still on their feet, were out of their places at
    /// the halt.
    astray: Option<usize>,
    /// After the halt, nine in ten of those of the tired body, still on their feet, were in
    /// their places again.
    back: Option<u32>,
    jammed: bool,
    /// The least distance between a standing man of one body and one of the other, mm.
    nearest: Option<f64>,
}

impl Drill {
    /// The men who were in their places at the order, are on their feet, and are out of
    /// their places now.
    fn astray_now<'a>(&'a self, b: &'a Battle) -> impl Iterator<Item = usize> + 'a {
        let bodies = b.live.bodies();
        self.placed
            .0
            .iter()
            .chain(&self.placed.1)
            .copied()
            .filter(move |&i| bodies.posture[i] != Posture::Down && !bodies.in_place[i])
    }

    /// Reads the two bodies after a step. The second line of the patch has no order but its
    /// relief, and the first none but to stand, so an advance of the fresh body is the
    /// relief's order and a retirement of the tired one is its second half. The frames' own
    /// tests read the drill by these orders too.
    fn watch(&mut self, b: &Battle, of: &[Option<BodyId>], tired: u32, fresh: u32, man_step: bool) {
        let (Some(t), Some(f)) = (b.live.body(BodyId(tired)), b.live.body(BodyId(fresh))) else {
            return;
        };
        let bodies = b.live.bodies();
        if self.ordered.is_none() && f.order == Order::Advance {
            self.ordered = Some(f.order_tick);
            let placed = |body: u32| {
                standing(b, of, body)
                    .filter(|&i| bodies.in_place[i])
                    .collect::<Vec<usize>>()
            };
            self.placed = (placed(tired), placed(fresh));
        }
        if self.ordered.is_some() && self.up.is_none() && t.order == Order::Retire {
            self.up = Some(t.order_tick);
        }
        if self.up.is_some() && self.halted.is_none() && t.order == Order::Hold {
            self.halted = Some(t.order_tick);
            self.astray = Some(self.astray_now(b).count());
        }
        if self.halted.is_some() && self.back.is_none() {
            let (mut men, mut placed) = (0, 0);
            for &i in &self.placed.0 {
                if bodies.posture[i] != Posture::Down {
                    men += 1;
                    placed += usize::from(bodies.in_place[i]);
                }
            }
            if men > 0 && placed * 10 >= men * 9 {
                self.back = Some(b.live.tick());
            }
        }
        if !man_step || self.ordered.is_none() || self.halted.is_some() {
            return;
        }
        let pair: Vec<usize> = standing(b, of, tired)
            .chain(standing(b, of, fresh))
            .collect();
        if !self.jammed {
            self.jammed = pair.iter().any(|&i| {
                let other = if of[i] == Some(BodyId(tired)) {
                    fresh
                } else {
                    tired
                };
                let near = || {
                    (0..bodies.len()).filter(move |&j| {
                        j != i
                            && bodies.posture[j] != Posture::Down
                            && bodies.pos[j].dist_sq(bodies.pos[i]) <= JAM_WITHIN_MM * JAM_WITHIN_MM
                    })
                };
                near().count() >= JAM_OTHERS && near().any(|j| of[j] == Some(BodyId(other)))
            });
        }
        for i in standing(b, of, tired) {
            for j in standing(b, of, fresh) {
                let d = (bodies.pos[i].dist_sq(bodies.pos[j]) as f64).sqrt();
                if self.nearest.is_none_or(|n| d < n) {
                    self.nearest = Some(d);
                }
            }
        }
    }
}

/// Whether a bout began with a man of either body between two ticks.
fn pressed(
    events: &[Event],
    of: &[Option<BodyId>],
    (tired, fresh): (u32, u32),
    (from, until): (u32, u32),
) -> bool {
    let in_pair = |m: ManId| matches!(of.get(m.0 as usize), Some(Some(BodyId(x))) if *x == tired || *x == fresh);
    events.iter().any(|e| {
        (from..=until).contains(&e.tick)
            && matches!(e.kind, EventKind::Bout { attacker, .. }
                if in_pair(attacker) || in_pair(e.man))
    })
}

/// Where a body's standing men are along the line of advance: the rearmost and the foremost.
fn span(b: &Battle, of: &[Option<BodyId>], body: u32) -> Option<(i32, i32)> {
    let bodies = b.live.bodies();
    let xs: Vec<i32> = standing(b, of, body).map(|i| bodies.pos[i].x).collect();
    Some((*xs.iter().min()?, *xs.iter().max()?))
}

fn drill(mut args: impl Iterator<Item = String>) -> Result<(), String> {
    let (mut engine, mut fight) = (None, false);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--engine" => engine = args.next().as_deref().and_then(engine_of),
            "--fight" => fight = true,
            other => return Err(format!("unknown argument {other:?}")),
        }
    }
    let engine = engine.ok_or("drill needs --engine field|pairwise")?;
    let text = scenario_text("relief")?;
    let soon = text.replace("not_before_s = 1200", "not_before_s = 30");
    if soon == text {
        return Err("the relief patch no longer says not_before_s = 1200".to_string());
    }
    let s = scenario("relief", &soon)?;
    let d = nominal(Cell { engine, rung: 2 })?;
    let c = Config {
        engine,
        rung: 2,
        variant: "relief".to_string(),
        seed: 3,
    };
    let troops = s
        .bodies
        .iter()
        .map(|body| {
            if !fight {
                return Ok(STOLID);
            }
            // The troops the patch names, with a nerve no alarm reaches: nobody flees.
            let named = d
                .troop(&body.troop)
                .ok_or_else(|| format!("unknown troop {:?}", body.troop))?;
            Ok(Troop {
                nerve: Spread { mean: 8_192, sd: 0 },
                ..named
            })
        })
        .collect::<Result<Vec<Troop>, String>>()?;
    let mut b = build_from(&c, &s, &d, Some(&troops))?.battle;
    let of = b.live.bodies().body.clone();
    let pairs = pairs(&s)?;
    let mut drills = vec![Drill::default(); pairs.len()];
    // The fresh body's front and the tired body's rear, at the order, when the fresh body was
    // up, and at the halt.
    let mut at = vec![[None::<(i32, i32)>; 3]; pairs.len()];
    // Who was out of his place at the halt: his role and the body he began in.
    let mut astray = vec![String::new(); pairs.len()];
    let end = if fight { 600 } else { 240 } * 1000 / BASE_TICK.0;
    while b.live.tick() < end {
        let man_step = b.live.step();
        for (k, &(tired, fresh)) in pairs.iter().enumerate() {
            let before = (drills[k].ordered, drills[k].up, drills[k].halted);
            drills[k].watch(&b, &of, tired, fresh, man_step);
            let now = &drills[k];
            let both = || Some((span(&b, &of, fresh)?.1, span(&b, &of, tired)?.0));
            if before.0 != now.ordered {
                at[k][0] = both();
            }
            if before.1 != now.up {
                at[k][1] = both();
            }
            if before.2 != now.halted {
                at[k][2] = both();
                let bodies = b.live.bodies();
                astray[k] = now
                    .astray_now(&b)
                    .map(|i| format!("{:?} of body {}", bodies.role[i], of[i].map_or(0, |x| x.0)))
                    .collect::<Vec<_>>()
                    .join(", ");
            }
        }
        if drills.iter().all(|d| d.back.is_some()) {
            break;
        }
    }
    if !fight && !b.live.events().is_empty() {
        return Err(format!(
            "{} events in a drill of men whom nothing moves",
            b.live.events().len()
        ));
    }
    println!(
        "the relief {}, {engine:?}, nominal set, rung 2, seed 3; a man walks {} mm/s",
        if fight {
            "in a fight of men who cannot flee"
        } else {
            "alone"
        },
        d.speeds.walk
    );
    for (k, dr) in drills.iter().enumerate() {
        let (Some(o), Some(u), Some(h)) = (dr.ordered, dr.up, dr.halted) else {
            println!("  relief {k}: not finished: {dr:?}");
            continue;
        };
        let ([Some(a), Some(m), Some(z)], Some(near)) = (at[k], dr.nearest) else {
            println!("  relief {k}: a body has no man standing");
            continue;
        };
        let (came, went) = (m.0 - a.0, m.1 - z.1);
        let walk = f64::from(d.speeds.walk);
        println!(
            "  relief {k}: ordered at {:.2} s; the fresh century up after {:.2} s, its front \
             {came} mm forward (a free walk: {:.2} s); the tired one halted {:.2} s later, its \
             rear {went} mm back (a free walk: {:.2} s)",
            seconds(o),
            seconds(u - o),
            f64::from(came) / walk,
            seconds(h - u),
            f64::from(went) / walk,
        );
        println!(
            "    of the {} and {} men in their places at the order, {} out of them at the halt \
             ({}); nine in ten of the tired century's in their places again {} after it; nearest \
             two men of the two {near:.0} mm; jammed: {}{}",
            dr.placed.0.len(),
            dr.placed.1.len(),
            dr.astray.unwrap_or(0),
            astray[k],
            dr.back
                .map_or("never".to_string(), |t| format!("{:.2} s", seconds(t - h))),
            dr.jammed,
            if fight {
                format!(
                    "; pressed: {}",
                    pressed(b.live.events(), &of, pairs[k], (o, h))
                )
            } else {
                String::new()
            }
        );
    }
    Ok(())
}

/// What the two commands over a cell's records are given.
struct Asked {
    dir: String,
    cell: Cell,
    freeze: u64,
    out: String,
    max: usize,
    jobs: usize,
}

fn asked(mut args: impl Iterator<Item = String>) -> Result<Asked, String> {
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
        return Err("needs --records, --engine, --rung, --freeze-seed and --out".to_string());
    };
    if Path::new(&out).exists() {
        return Err(format!("{out} is there already"));
    }
    Ok(Asked {
        dir,
        cell: Cell { engine, rung },
        freeze,
        out,
        max,
        jobs,
    })
}

/// The scored records of one variant: stopped runs of the freeze seed. None if the file is
/// not there.
fn records(a: &Asked, variant: &str) -> Result<Vec<Record>, String> {
    let path = Path::new(&a.dir).join(record_file(a.cell, variant));
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    let mut all = Vec::new();
    for (n, l) in text.lines().enumerate() {
        if l.trim().is_empty() {
            continue;
        }
        let r: Record = serde_json::from_str(l)
            .map_err(|e| format!("{}, line {}: {e}", path.display(), n + 1))?;
        if !r.whole && r.freeze_seed == Some(a.freeze) {
            all.push(r);
        }
    }
    all.sort_by_key(|r| (r.draw, r.seed));
    Ok(all)
}

/// The scorer's own run of a record's set and seed, if it gives the record again.
fn again(
    s: &Scenario,
    variant: &str,
    a: &Asked,
    r: &Record,
) -> Result<(Drawn, Run, Vec<Event>), String> {
    let d = drawn(a.freeze, a.cell, r.draw)?;
    let spec = Spec {
        seed: r.seed,
        duration_s: s.duration_s,
        stop_at_rout_end: true,
    };
    let (run, events) = run_logged(s, &d, spec)?;
    let mut made = Record::new(a.cell, variant, Some(a.freeze), false, &run);
    made.wall_ms = r.wall_ms;
    if made != *r {
        return Err("the scorer's run does not give the record again".to_string());
    }
    Ok((d, run, events))
}

/// The same battle, to be stepped here.
fn stepped(
    s: &Scenario,
    variant: &str,
    a: &Asked,
    d: &Drawn,
    seed: u64,
) -> Result<(Battle, Vec<BodyInfo>), String> {
    let c = Config {
        engine: a.cell.engine,
        rung: a.cell.rung,
        variant: variant.to_string(),
        seed,
    };
    let built = build_from(&c, s, d, None)?;
    Ok((built.battle, built.bodies))
}

/// What one replayed run gave: its lines, or why it is not counted.
type Said = Result<Vec<String>, String>;

/// Runs `one` over the chosen records on `jobs` threads, writes its lines to the file in the
/// records' order, and prints the counts.
fn over(
    a: &Asked,
    what: &str,
    on_record: usize,
    chosen: &[&Record],
    one: impl Fn(&Record) -> Said + Sync,
) -> Result<(), String> {
    let next = AtomicUsize::new(0);
    let done: Mutex<Vec<(usize, Said)>> = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..a.jobs.max(1) {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(r) = chosen.get(i) else { break };
                    let said = one(r);
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
    std::fs::write(&a.out, text).map_err(|e| format!("{}: {e}", a.out))?;
    println!(
        "{:?} rung {}: {on_record} {what} on record, {} replayed, {not_counted} not counted; {}",
        a.cell.engine,
        a.cell.rung,
        done.len(),
        a.out
    );
    Ok(())
}

/// What one replayed run says of its reliefs: a line of JSON for each.
fn relief_lines(
    s: &Scenario,
    pairs: &[(u32, u32)],
    a: &Asked,
    r: &Record,
    twin: Option<&Record>,
) -> Result<Vec<String>, String> {
    let (d, run, events) = again(s, "relief", a, r)?;
    let (mut b, _) = stepped(s, "relief", a, &d, r.seed)?;
    let of = b.live.bodies().body.clone();
    let mut drills = vec![Drill::default(); pairs.len()];
    let end = run.sim_s * 1000 / BASE_TICK.0;
    while b.live.tick() < end {
        let man_step = b.live.step();
        for (k, &(tired, fresh)) in pairs.iter().enumerate() {
            drills[k].watch(&b, &of, tired, fresh, man_step);
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
    let broke = |body: u32| {
        r.summary
            .body_break
            .get(body as usize)
            .is_some_and(Option::is_some)
    };
    let after = |t: Option<u32>, from: Option<u32>| match (t, from) {
        (Some(t), Some(f)) => format!("{:.2}", seconds(t - f)),
        _ => "null".to_string(),
    };
    let mut lines = Vec::new();
    for (k, (dr, &(tired, fresh))) in drills.iter().zip(pairs).enumerate() {
        let Some(o) = dr.ordered else { continue };
        let until = dr.halted.unwrap_or(end);
        let pressed = pressed(&events, &of, (tired, fresh), (o, until));
        let unfinished = match dr.halted {
            Some(_) => "null",
            None if broke(tired) || broke(fresh) => "\"broke\"",
            None => "\"ended\"",
        };
        let mut l = String::new();
        let _ = write!(
            l,
            "{{\"draw\":{},\"seed\":{},\"relief\":{k},\"ordered_s\":{:.2},\"placed_at_order\":\
             [{},{}],\"up_after_s\":{},\
             \"halted_after_s\":{},\"astray_at_halt\":{},\"back_after_s\":{},\
             \"pressed\":{pressed},\"jammed\":{},\"unfinished\":{unfinished},\
             \"twin\":\"{fared}\"}}",
            r.draw,
            r.seed,
            seconds(o),
            dr.placed.0.len(),
            dr.placed.1.len(),
            after(dr.up, dr.ordered),
            after(dr.halted, dr.up),
            dr.astray.map_or("null".to_string(), |n| n.to_string()),
            after(dr.back, dr.halted),
            dr.jammed,
        );
        lines.push(l);
    }
    Ok(lines)
}

fn reliefs(args: impl Iterator<Item = String>) -> Result<(), String> {
    let a = asked(args)?;
    let relieved: Vec<Record> = records(&a, "relief")?
        .into_iter()
        .filter(|r| r.relieved_s.iter().any(Option::is_some))
        .collect();
    let backed = records(&a, "backed")?;
    // The runs to replay: in order, until `max` reliefs have been ordered in them.
    let mut ordered = 0;
    let chosen: Vec<&Record> = relieved
        .iter()
        .take_while(|r| {
            let more = ordered < a.max;
            ordered += r.relieved_s.iter().filter(|t| t.is_some()).count();
            more
        })
        .collect();
    let s = scenario("relief", &scenario_text("relief")?)?;
    let pairs = pairs(&s)?;
    over(&a, "relieved runs", relieved.len(), &chosen, |r| {
        let twin = backed.iter().find(|t| t.draw == r.draw && t.seed == r.seed);
        relief_lines(&s, &pairs, &a, r, twin)
    })
}

/// What one replayed run of `base` says of its leaders' and file leaders' falls.
fn leader_lines(s: &Scenario, a: &Asked, r: &Record) -> Result<Vec<String>, String> {
    let (d, run, events) = again(s, "base", a, r)?;
    let (mut b, infos) = stepped(s, "base", a, &d, r.seed)?;
    let of = b.live.bodies().body.clone();
    let end = run.sim_s * 1000 / BASE_TICK.0;
    let breaks = &r.summary.body_break;
    let mut led_down = vec![false; s.bodies.len()];
    let mut seen = 0;
    let mut lines = Vec::new();
    while b.live.tick() < end {
        // A man's role is read as it stood before the step in which he went down.
        let roles = b.live.bodies().role.clone();
        b.live.step();
        let bodies = b.live.bodies();
        let new = &b.live.events()[seen..];
        seen += new.len();
        for e in new {
            let EventKind::Posture {
                to: Posture::Down, ..
            } = e.kind
            else {
                continue;
            };
            let m = e.man.0 as usize;
            let Some(BodyId(body)) = of[m] else { continue };
            let k = body as usize;
            if !s.bodies[k].front || breaks[k].is_some_and(|t| t <= e.tick) {
                continue;
            }
            let group = match roles[m] {
                Role::Leader if !led_down[k] => {
                    led_down[k] = true;
                    "leader"
                }
                Role::FileLeader => "file",
                _ => continue,
            };
            let (mut men, mut turned) = (0u32, 0u32);
            for i in standing(&b, &of, body) {
                men += 1;
                turned += u32::from(bodies.posture[i] == Posture::Turned);
            }
            let mut l = String::new();
            let _ = write!(
                l,
                "{{\"draw\":{},\"seed\":{},\"body\":{body},\"side\":{},\"group\":\"{group}\",\
                 \"fall_s\":{:.2},\"break_after_s\":{},\"in_flight\":{:.4},\
                 \"succeed_after_s\":{}}}",
                r.draw,
                r.seed,
                bodies.side[m] as u8,
                seconds(e.tick),
                breaks[k].map_or("null".to_string(), |t| format!(
                    "{:.2}",
                    seconds(t - e.tick)
                )),
                if men > 0 {
                    f64::from(turned) / f64::from(men)
                } else {
                    0.0
                },
                infos[k]
                    .tie
                    .succeed_after
                    .map_or("null".to_string(), |t| format!("{:.2}", seconds(t))),
            );
            lines.push(l);
        }
    }
    if b.live.events() != events.as_slice() {
        return Err("the battle stepped here is not the scorer's".to_string());
    }
    Ok(lines)
}

fn leaders(args: impl Iterator<Item = String>) -> Result<(), String> {
    let a = asked(args)?;
    let base = records(&a, "base")?;
    let chosen: Vec<&Record> = base
        .iter()
        .filter(|r| r.draw < 200 && r.draw % 10 == 0)
        .collect();
    let s = scenario("base", &scenario_text("base")?)?;
    over(&a, "runs of base", base.len(), &chosen, |r| {
        leader_lines(&s, &a, r)
    })
}

fn main() {
    let mut args = std::env::args().skip(1);
    let done = match args.next().as_deref() {
        Some("drill") => drill(args),
        Some("reliefs") => reliefs(args),
        Some("leaders") => leaders(args),
        _ => Err("usage: cannae-replay drill --engine E [--fight]\n       \
                  cannae-replay reliefs --records DIR --engine E --rung R --freeze-seed F \
                  --out FILE [--max 100] [--jobs 4]\n       \
                  cannae-replay leaders --records DIR --engine E --rung R --freeze-seed F \
                  --out FILE [--jobs 4]"
            .to_string()),
    };
    if let Err(e) = done {
        eprintln!("{e}");
        std::process::exit(2);
    }
}
