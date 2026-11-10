//! A live view of one battle of the patch, in a browser.
//!
//! `cargo run --release -- [--port 8377] [--bind 127.0.0.1]`, then open the address it prints.
//!
//! The battle is built as a scored run is (`cannae_score::run`): the same scenario files, man,
//! memory rung, frames, bouts, perception and substrate. Its parameters are the nominal set,
//! the centre of every prior, which is never scored. The drawn sets of the scored batch are not
//! offered: to watch one before it is scored would be to read its outcome.
//!
//! Wall time decides only how many ticks are stepped before the next frame is sent. It never
//! touches the state: the battle shown for a seed is the battle any run of that seed gives.

use cannae_core::schedule::BASE_TICK;
use cannae_core::{
    Bodies, Bouts, Event, EventKind, Frames, GridPerception, ManModel, Perceive, Posture, Resolver,
    Rng, Substrate, Wall,
};
use cannae_field::{Field, FieldPerception};
use cannae_gauges::Rung3;
use cannae_harness::scenario::frames_with;
use cannae_harness::{Runner, Scenario, place};
use cannae_man::{Marks, Men};
use cannae_pairwise::Pairwise;
use cannae_score::Engine;
use cannae_score::draw::{Cell, SubstrateParams, nominal};
use std::collections::VecDeque;
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender, TryRecvError, channel};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

const PAGE: &str = include_str!("page.html");

/// The scenario files of `scenarios/patch`. A variant named in a request is checked against
/// this list before it becomes part of a path.
const VARIANTS: [&str; 8] = [
    "base", "veterans", "recruits", "flank", "backed", "relief", "volleys", "shock",
];

/// The longest a frame may wait while the battle is stepped, and the least time between frames.
const STEP_BUDGET: Duration = Duration::from_millis(12);
const FRAME_EVERY: Duration = Duration::from_millis(33);

/// How many frames' worth of news are kept for a reader that joins late or falls behind.
const NEWS_KEPT: usize = 600;

#[derive(Clone, Debug)]
struct Config {
    engine: Engine,
    rung: u8,
    variant: String,
    seed: u64,
}

enum Cmd {
    Play(bool),
    /// Simulated seconds per wall second; infinite for as fast as the machine goes.
    Speed(f64),
    /// On to the end of the next man step.
    Step,
    Restart(Config),
}

/// A battle under way, whatever its engine and memory.
trait Live {
    /// One base tick. Returns whether it was a man step.
    fn step(&mut self) -> bool;
    fn tick(&self) -> u32;
    fn bodies(&self) -> &Bodies;
    fn events(&self) -> &[Event];
}

impl<P: Perceive, M: ManModel, S: Substrate, F: Frames, R: Resolver> Live
    for Runner<P, M, S, F, R>
{
    fn step(&mut self) -> bool {
        Runner::step(self)
    }

    fn tick(&self) -> u32 {
        self.tick
    }

    fn bodies(&self) -> &Bodies {
        &self.bodies
    }

    fn events(&self) -> &[Event] {
        self.log.all()
    }
}

struct Battle {
    live: Box<dyn Live>,
    end_tick: u32,
    /// How many events of the log earlier frames have already carried.
    sent: usize,
}

impl Battle {
    fn ended(&self) -> bool {
        self.live.tick() >= self.end_tick
    }
}

/// Builds the battle as `cannae_score` builds a scored run, and describes it for the page.
fn build(c: &Config) -> Result<(Battle, String), String> {
    if !VARIANTS.contains(&c.variant.as_str()) {
        return Err(format!("unknown variant {:?}", c.variant));
    }
    let path = format!(
        "{}/../scenarios/patch/{}.toml",
        env!("CARGO_MANIFEST_DIR"),
        c.variant
    );
    let text = std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))?;
    let s = Scenario::from_toml(&text).map_err(|e| format!("{path}: {e}"))?;
    let d = nominal(Cell {
        engine: c.engine,
        rung: c.rung,
    })?;
    let troops = s
        .bodies
        .iter()
        .map(|b| {
            d.troop(&b.troop)
                .ok_or_else(|| format!("unknown troop {:?}", b.troop))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let (bodies, placed) = place(&s);
    let frames = frames_with(&s, &placed, &bodies, |t| d.tie(t))?;
    let n = bodies.len();
    let rng = Rng::new(c.seed);
    let walls = s.walls();
    let volleys = s.volleys()?;
    let meta = meta_json(c, &s, &bodies, &walls);
    macro_rules! battle {
        ($perception:expr, $memory:expr, $substrate:expr) => {{
            let mut men = Men::with_memory($memory, d.man, &bodies, &troops, &rng);
            men.power = d.power;
            let fighting = (Bouts::new(d.bouts, men.kits()), volleys);
            let runner = Runner::new(
                bodies,
                $perception,
                men,
                $substrate,
                frames,
                fighting,
                rng,
                d.speeds,
            );
            Box::new(runner) as Box<dyn Live>
        }};
    }
    let live = match (d.substrate, d.rung()) {
        (SubstrateParams::Pairwise(p), rung) => {
            let mut substrate = Pairwise::new(p);
            substrate.walls = walls;
            match rung {
                Some(r) => battle!(
                    GridPerception::default(),
                    Marks::new(r, &d.man, n),
                    substrate
                ),
                None => battle!(
                    GridPerception::default(),
                    Rung3::new(&d.man, d.gauges, n),
                    substrate
                ),
            }
        }
        (SubstrateParams::Field(p), rung) => {
            let substrate = Field::with_walls(p, &walls);
            match rung {
                Some(r) => battle!(
                    FieldPerception::default(),
                    Marks::new(r, &d.man, n),
                    substrate
                ),
                None => battle!(
                    FieldPerception::default(),
                    Rung3::new(&d.man, d.gauges, n),
                    substrate
                ),
            }
        }
    };
    let battle = Battle {
        live,
        end_tick: s.duration_s * 1000 / BASE_TICK.0,
        sent: 0,
    };
    Ok((battle, meta))
}

fn engine_name(e: Engine) -> &'static str {
    match e {
        Engine::Pairwise => "pairwise",
        Engine::Field => "field",
    }
}

/// A string made safe inside a JSON string.
fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if (c as u32) < 0x20 => out.push(' '),
            c => out.push(c),
        }
    }
    out
}

fn ints(out: &mut String, values: impl Iterator<Item = i64>) {
    out.push('[');
    for (i, v) in values.enumerate() {
        if i > 0 {
            out.push(',');
        }
        let _ = write!(out, "{v}");
    }
    out.push(']');
}

/// What does not change during a battle: who each man is, and the ground.
fn meta_json(c: &Config, s: &Scenario, b: &Bodies, walls: &[Wall]) -> String {
    let mut m = String::new();
    let _ = write!(
        m,
        "{{\"scenario\":\"{}\",\"engine\":\"{}\",\"rung\":{},\"variant\":\"{}\",\"seed\":\"{}\",\
         \"tickMs\":{},\"durationS\":{},\"n\":{}",
        esc(&s.name),
        engine_name(c.engine),
        c.rung,
        esc(&c.variant),
        c.seed,
        BASE_TICK.0,
        s.duration_s,
        b.len()
    );
    m.push_str(",\"side\":");
    ints(&mut m, b.side.iter().map(|&x| x as i64));
    m.push_str(",\"role\":");
    ints(&mut m, b.role.iter().map(|&x| x as i64));
    m.push_str(",\"body\":");
    ints(
        &mut m,
        b.body.iter().map(|x| x.map_or(-1, |id| id.0 as i64)),
    );
    m.push_str(",\"bodies\":[");
    for (i, body) in s.bodies.iter().enumerate() {
        if i > 0 {
            m.push(',');
        }
        let _ = write!(m, "\"{}\"", esc(&body.name));
    }
    m.push_str("],\"walls\":[");
    for (i, w) in walls.iter().enumerate() {
        if i > 0 {
            m.push(',');
        }
        let _ = write!(m, "[{},{},{},{}]", w.a.x, w.a.y, w.b.x, w.b.y);
    }
    m.push_str("]}");
    m
}

/// The observable state now; and, apart from it, the news since the last call: bouts, changes
/// of posture with their causes, wounds and standards. `None` if nothing happened.
fn frame_json(
    battle: &mut Battle,
    rate: f64,
    speed: f64,
    playing: bool,
) -> (String, Option<String>) {
    let b = battle.live.bodies();
    let mut f = String::with_capacity(32 * b.len() + 256);
    let _ = write!(
        f,
        "{{\"tick\":{},\"rate\":{:.2},\"speed\":{},\"playing\":{},\"ended\":{}",
        battle.live.tick(),
        if rate.is_finite() { rate } else { 0.0 },
        // Zero stands for flat out.
        if speed.is_finite() { speed } else { 0.0 },
        playing,
        battle.ended()
    );
    f.push_str(",\"x\":");
    ints(&mut f, b.pos.iter().map(|p| p.x as i64));
    f.push_str(",\"y\":");
    ints(&mut f, b.pos.iter().map(|p| p.y as i64));
    f.push_str(",\"a\":");
    ints(&mut f, b.facing.iter().map(|t| t.0 as i64));
    f.push_str(",\"p\":");
    ints(&mut f, b.posture.iter().map(|&x| x as i64));
    f.push_str(",\"w\":");
    ints(&mut f, b.wound.iter().map(|&x| x as i64));
    f.push_str(",\"ip\":");
    ints(&mut f, b.in_place.iter().map(|&x| x as i64));
    f.push('}');
    let events = battle.live.events();
    let (mut bouts, mut moves, mut wounds, mut standards) =
        (String::new(), String::new(), String::new(), String::new());
    let item = |list: &mut String| {
        if !list.is_empty() {
            list.push(',');
        }
    };
    for e in &events[battle.sent.min(events.len())..] {
        match e.kind {
            // Attacker, defender, what the defender took, what the attacker took.
            EventKind::Bout {
                attacker,
                defender,
                attacker_took,
            } => {
                item(&mut bouts);
                let _ = write!(
                    bouts,
                    "[{},{},{},{}]",
                    attacker.0, e.man.0, defender as u8, attacker_took as u8
                );
            }
            // A man stepping out to strike and back again is the pulse of the fight, and far
            // too frequent to tell one by one: only giving way and coming back are news.
            EventKind::Posture { from, to }
                if matches!(to, Posture::Back | Posture::Turned | Posture::Down)
                    || matches!(from, Posture::Back | Posture::Turned) =>
            {
                item(&mut moves);
                let c = e.causes;
                let _ = write!(
                    moves,
                    "[{},{},{},{},\"{:?}\",\"{:?}\",\"{:?}\"]",
                    e.tick, e.man.0, from as u8, to as u8, c[0], c[1], c[2]
                );
            }
            EventKind::Wound { to } => {
                item(&mut wounds);
                let _ = write!(wounds, "[{},{},{}]", e.tick, e.man.0, to as u8);
            }
            EventKind::StandardDown { body } => {
                item(&mut standards);
                let _ = write!(standards, "[{},{},0]", e.tick, body.0);
            }
            EventKind::StandardUp { body } => {
                item(&mut standards);
                let _ = write!(standards, "[{},{},1]", e.tick, body.0);
            }
            _ => {}
        }
    }
    battle.sent = events.len();
    let quiet = bouts.is_empty() && moves.is_empty() && wounds.is_empty() && standards.is_empty();
    let news = (!quiet).then(|| {
        format!(
            "{{\"bouts\":[{bouts}],\"moves\":[{moves}],\"wounds\":[{wounds}],\
             \"standards\":[{standards}]}}"
        )
    });
    (f, news)
}

/// What the page is told: the latest description and frame, each with a count of how many
/// there have been, so that a reader knows what it has not yet sent on. A frame may be skipped
/// by a slow reader; the news may not, so it is kept, each item under its frame's number.
#[derive(Default)]
struct Shared {
    meta: String,
    metas: u64,
    frame: String,
    frames: u64,
    news: VecDeque<(u64, String)>,
}

#[derive(Default)]
struct Hub {
    shared: Mutex<Shared>,
    changed: Condvar,
}

impl Hub {
    fn publish(&self, meta: Option<String>, (frame, news): (String, Option<String>)) {
        let mut s = self.shared.lock().expect("hub");
        if let Some(m) = meta {
            s.meta = m;
            s.metas += 1;
            // A new battle: the last one's news is no longer news.
            s.news.clear();
        }
        s.frame = frame;
        s.frames += 1;
        if let Some(n) = news {
            let at = s.frames;
            s.news.push_back((at, n));
            if s.news.len() > NEWS_KEPT {
                s.news.pop_front();
            }
        }
        self.changed.notify_all();
    }
}

fn start(hub: &Hub, config: &Config, speed: f64, playing: bool) -> Option<Battle> {
    match build(config) {
        Ok((mut battle, meta)) => {
            let update = frame_json(&mut battle, 0.0, speed, playing);
            hub.publish(Some(meta), update);
            Some(battle)
        }
        Err(e) => {
            eprintln!("{e}");
            hub.publish(
                Some(format!("{{\"error\":\"{}\"}}", esc(&e))),
                ("{}".to_string(), None),
            );
            None
        }
    }
}

/// Owns the battle: steps it at the asked speed and publishes frames.
fn simulate(hub: Arc<Hub>, commands: Receiver<Cmd>, mut config: Config) {
    let tick_ms = BASE_TICK.0 as f64;
    let (mut playing, mut speed) = (false, 1.0f64);
    let mut battle = start(&hub, &config, speed, playing);
    // Simulated milliseconds the battle is behind the clock.
    let mut owed = 0.0f64;
    let mut last = Instant::now();
    let mut last_frame = Instant::now();
    let (mut window, mut window_ticks, mut rate) = (Instant::now(), 0u32, 0.0f64);
    loop {
        let cmd = if playing {
            match commands.try_recv() {
                Ok(c) => Some(c),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => return,
            }
        } else {
            match commands.recv_timeout(Duration::from_millis(250)) {
                Ok(c) => Some(c),
                Err(RecvTimeoutError::Timeout) => None,
                Err(RecvTimeoutError::Disconnected) => return,
            }
        };
        if let Some(cmd) = cmd {
            match cmd {
                Cmd::Play(on) => playing = on,
                Cmd::Speed(s) => speed = s,
                Cmd::Step => {
                    playing = false;
                    if let Some(b) = battle.as_mut() {
                        while !b.ended() && !b.live.step() {}
                    }
                }
                Cmd::Restart(c) => {
                    config = c;
                    battle = start(&hub, &config, speed, playing);
                }
            }
            playing &= battle.as_ref().is_some_and(|b| !b.ended());
            (owed, last, window, window_ticks) = (0.0, Instant::now(), Instant::now(), 0);
            if !playing {
                rate = 0.0;
            }
            if let Some(b) = battle.as_mut() {
                hub.publish(None, frame_json(b, rate, speed, playing));
            }
        }
        let Some(b) = battle.as_mut() else {
            continue;
        };
        if !playing {
            last = Instant::now();
            continue;
        }
        let now = Instant::now();
        let flat_out = !speed.is_finite();
        if !flat_out {
            owed += now.duration_since(last).as_secs_f64() * 1000.0 * speed;
        }
        last = now;
        let deadline = now + STEP_BUDGET;
        let mut stepped = 0u32;
        while (flat_out || owed >= tick_ms) && !b.ended() && Instant::now() < deadline {
            b.live.step();
            owed -= tick_ms;
            stepped += 1;
        }
        // A machine that cannot keep up runs as fast as it can, and builds no debt beyond a
        // tenth of a second of wall time.
        owed = if flat_out {
            0.0
        } else {
            owed.min(100.0 * speed)
        };
        window_ticks += stepped;
        let span = window.elapsed().as_secs_f64();
        if span >= 1.0 {
            rate = window_ticks as f64 * tick_ms / 1000.0 / span;
            (window, window_ticks) = (Instant::now(), 0);
        }
        if b.ended() {
            playing = false;
        }
        if !playing || (stepped > 0 && last_frame.elapsed() >= FRAME_EVERY) {
            hub.publish(None, frame_json(b, rate, speed, playing));
            last_frame = Instant::now();
        }
        if stepped == 0 {
            std::thread::sleep(Duration::from_millis(2));
        }
    }
}

fn command(query: &str) -> Result<Cmd, String> {
    let get = |k: &str| {
        query
            .split('&')
            .filter_map(|kv| kv.split_once('='))
            .find(|(key, _)| *key == k)
            .map(|(_, v)| v)
    };
    match get("do") {
        Some("play") => Ok(Cmd::Play(true)),
        Some("pause") => Ok(Cmd::Play(false)),
        Some("step") => Ok(Cmd::Step),
        Some("speed") => match get("x") {
            Some("max") => Ok(Cmd::Speed(f64::INFINITY)),
            Some(x) => match x.parse::<f64>() {
                Ok(v) if v > 0.0 && v <= 100.0 => Ok(Cmd::Speed(v)),
                _ => Err(format!("bad speed {x:?}")),
            },
            None => Err("speed needs x".to_string()),
        },
        Some("restart") => {
            let engine = match get("engine") {
                Some("field") => Engine::Field,
                Some("pairwise") => Engine::Pairwise,
                other => return Err(format!("unknown engine {other:?}")),
            };
            let rung = get("rung")
                .and_then(|v| v.parse::<u8>().ok())
                .filter(|r| *r <= 3)
                .ok_or("the rung is 0 to 3")?;
            let variant = get("variant")
                .filter(|v| VARIANTS.contains(v))
                .ok_or("unknown variant")?
                .to_string();
            let seed = get("seed")
                .and_then(|v| v.parse::<u64>().ok())
                .ok_or("the seed is a whole number")?;
            Ok(Cmd::Restart(Config {
                engine,
                rung,
                variant,
                seed,
            }))
        }
        other => Err(format!("unknown command {other:?}")),
    }
}

fn respond(out: &mut TcpStream, status: &str, kind: &str, body: &[u8]) -> std::io::Result<()> {
    write!(
        out,
        "HTTP/1.1 {status}\r\nContent-Type: {kind}\r\nContent-Length: {}\r\n\
         Cache-Control: no-store\r\nConnection: close\r\n\r\n",
        body.len()
    )?;
    out.write_all(body)
}

/// A stream of server-sent events: the description when it changes, then the news and each
/// frame as they come. A reader that falls behind gets the latest frame, not the ones it
/// missed, but all the news.
fn events(out: &mut TcpStream, hub: &Hub) -> std::io::Result<()> {
    out.set_nodelay(true)?;
    out.write_all(
        b"HTTP/1.1 200 OK\r\nContent-Type: text/event-stream\r\nCache-Control: no-store\r\n\
          Connection: keep-alive\r\n\r\n",
    )?;
    let (mut metas, mut frames) = (0u64, 0u64);
    loop {
        let (meta, news, frame) = {
            let mut s = hub.shared.lock().expect("hub");
            while s.metas == metas && s.frames == frames {
                let (guard, wait) = hub
                    .changed
                    .wait_timeout(s, Duration::from_secs(10))
                    .expect("hub");
                s = guard;
                if wait.timed_out() {
                    break;
                }
            }
            let meta = (s.metas != metas).then(|| s.meta.clone());
            // All the news since the last frame this reader was sent. A reader that has just
            // joined gets what is kept of the battle so far.
            let news: Vec<String> = s
                .news
                .iter()
                .filter(|(at, _)| *at > frames)
                .map(|(_, n)| n.clone())
                .collect();
            let frame = (s.frames != frames).then(|| s.frame.clone());
            (metas, frames) = (s.metas, s.frames);
            (meta, news, frame)
        };
        if meta.is_none() && frame.is_none() {
            // Nothing new in ten seconds: a comment keeps the connection from being dropped.
            out.write_all(b": waiting\n\n")?;
        }
        if let Some(m) = meta {
            write!(out, "event: meta\ndata: {m}\n\n")?;
        }
        for n in news {
            write!(out, "event: news\ndata: {n}\n\n")?;
        }
        if let Some(f) = frame {
            write!(out, "event: frame\ndata: {f}\n\n")?;
        }
        out.flush()?;
    }
}

fn serve(stream: TcpStream, hub: &Hub, commands: &Sender<Cmd>) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut request = String::new();
    reader.read_line(&mut request)?;
    let mut header = String::new();
    loop {
        header.clear();
        if reader.read_line(&mut header)? == 0 || header.trim_end().is_empty() {
            break;
        }
    }
    let mut out = stream;
    let target = request.split_whitespace().nth(1).unwrap_or("/");
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    match path {
        "/" => respond(
            &mut out,
            "200 OK",
            "text/html; charset=utf-8",
            PAGE.as_bytes(),
        ),
        "/events" => events(&mut out, hub),
        "/cmd" => match command(query) {
            Ok(cmd) => {
                let _ = commands.send(cmd);
                respond(&mut out, "204 No Content", "text/plain", b"")
            }
            Err(e) => respond(
                &mut out,
                "400 Bad Request",
                "text/plain; charset=utf-8",
                e.as_bytes(),
            ),
        },
        _ => respond(&mut out, "404 Not Found", "text/plain", b"not found"),
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let (mut port, mut bind) = (8377u16, String::from("127.0.0.1"));
    while let Some(a) = args.next() {
        match (a.as_str(), args.next()) {
            ("--port", Some(v)) if v.parse::<u16>().is_ok() => port = v.parse().unwrap_or(port),
            ("--bind", Some(v)) => bind = v,
            _ => {
                eprintln!("usage: cannae-view [--port 8377] [--bind 127.0.0.1]");
                std::process::exit(2);
            }
        }
    }
    let listener = match TcpListener::bind((bind.as_str(), port)) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{bind}:{port}: {e}");
            std::process::exit(1);
        }
    };
    let hub = Arc::new(Hub::default());
    let (commands, received) = channel();
    let first = Config {
        engine: Engine::Pairwise,
        rung: 3,
        variant: "base".to_string(),
        seed: 1,
    };
    {
        let hub = Arc::clone(&hub);
        std::thread::spawn(move || simulate(hub, received, first));
    }
    println!("Cannae live view: http://{bind}:{port}/");
    for stream in listener.incoming().flatten() {
        let (hub, commands) = (Arc::clone(&hub), commands.clone());
        std::thread::spawn(move || {
            // A reader that goes away ends its thread with an error, which is the normal end.
            let _ = serve(stream, &hub, &commands);
        });
    }
}
