//! What the live viewer and the QC pass share: one battle of the patch, built as a scored run
//! is (`cannae_score::run`): the same scenario files, man, memory rung, frames, bouts,
//! perception and substrate. Its parameters are the nominal set, the centre of every prior,
//! which is never scored. The drawn sets of the scored batch are not offered: to watch one
//! before it is scored would be to read its outcome.
//!
//! [`build_from`] takes any parameter set. It is for the replays the report owes
//! (`cannae-replay`), which run a scored set again after the scoring, and only a run that has
//! a record. The viewer and the QC pass do not call it.

pub mod detect;

use cannae_core::schedule::BASE_TICK;
use cannae_core::view::{BodyState, SlotView};
use cannae_core::{
    Bodies, BodyId, Bouts, Event, FrameQuery, Frames, GridPerception, ManId, ManModel, Perceive,
    Posture, Resolver, Rng, Side, Substrate, Tie, Wall,
};
use cannae_field::{Field, FieldPerception};
use cannae_gauges::Rung3;
use cannae_harness::scenario::frames_with;
use cannae_harness::{Dormancy, Runner, Scenario, place};
use cannae_man::{Marks, Men, Troop};
use cannae_pairwise::Pairwise;
use cannae_score::Engine;
use cannae_score::draw::{Cell, Drawn, SubstrateParams, nominal};
use std::fmt::Write as _;

/// The scenario files of `scenarios/patch`. A variant named in a request is checked against
/// this list before it becomes part of a path.
pub const VARIANTS: [&str; 8] = [
    "base", "veterans", "recruits", "flank", "backed", "relief", "volleys", "shock",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub engine: Engine,
    pub rung: u8,
    pub variant: String,
    pub seed: u64,
}

/// The bout phase, with a note of the postures it read. A bout is judged by what its men were
/// doing when it was resolved, and by the time a step returns its outcomes have been applied.
/// It passes everything through and draws nothing, so the battle is the one a bare resolver
/// gives (tested against `cannae_score::run_logged`).
pub struct Spy<R> {
    inner: R,
    read: Vec<Posture>,
}

impl<R: Resolver> Resolver for Spy<R> {
    fn resolve<F: FrameQuery>(
        &mut self,
        bodies: &Bodies,
        frames: &F,
        tick: u32,
        rng: &Rng,
        out: &mut Vec<Event>,
    ) {
        self.read.clear();
        self.read.extend_from_slice(&bodies.posture);
        self.inner.resolve(bodies, frames, tick, rng, out);
    }
}

/// A battle under way, whatever its engine and memory.
pub trait Live {
    /// One base tick. Returns whether it was a man step.
    fn step(&mut self) -> bool;
    fn tick(&self) -> u32;
    fn bodies(&self) -> &Bodies;
    fn events(&self) -> &[Event];
    fn slot(&self, man: ManId) -> Option<SlotView>;
    fn body(&self, body: BodyId) -> Option<BodyState>;
    /// The postures the bout phase read at the last man step.
    fn fought(&self) -> &[Posture];
    /// The state hash after every man step so far.
    fn hashes(&self) -> &[u64];
    /// How dormant men are treated. To be set before the first step.
    fn set_dormancy(&mut self, d: Dormancy);
}

impl<P: Perceive, M: ManModel, S: Substrate, F: Frames, R: Resolver> Live
    for Runner<P, M, S, F, Spy<R>>
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

    fn slot(&self, man: ManId) -> Option<SlotView> {
        self.frames.slot(man)
    }

    fn body(&self, body: BodyId) -> Option<BodyState> {
        self.frames.body(body)
    }

    fn fought(&self) -> &[Posture] {
        &self.resolver.read
    }

    fn hashes(&self) -> &[u64] {
        &self.hashes
    }

    fn set_dormancy(&mut self, d: Dormancy) {
        self.dormancy = d;
    }
}

pub struct Battle {
    pub live: Box<dyn Live>,
    pub end_tick: u32,
    /// How many events of the log earlier frames have already carried.
    pub sent: usize,
}

impl Battle {
    pub fn ended(&self) -> bool {
        self.live.tick() >= self.end_tick
    }
}

/// One body as the scenario sets it up. Its men are numbered from `first`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyInfo {
    pub name: String,
    pub side: Side,
    pub first: u32,
    pub count: u32,
    pub file_mm: i32,
    pub rank_mm: i32,
    /// The man behind the rear rank at the start, if the body has one.
    pub closer: Option<ManId>,
    pub tie: Tie,
}

pub struct Built {
    pub battle: Battle,
    /// What does not change during the battle, as JSON for the page.
    pub meta: String,
    pub bodies: Vec<BodyInfo>,
}

/// The text of a variant's scenario file.
pub fn scenario_text(variant: &str) -> Result<String, String> {
    if !VARIANTS.contains(&variant) {
        return Err(format!("unknown variant {variant:?}"));
    }
    let path = format!(
        "{}/../scenarios/patch/{variant}.toml",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read_to_string(&path).map_err(|e| format!("{path}: {e}"))
}

/// Builds the battle as `cannae_score` builds a scored run, at the nominal set.
pub fn build(c: &Config) -> Result<Built, String> {
    let text = scenario_text(&c.variant)?;
    let s = Scenario::from_toml(&text).map_err(|e| format!("{}: {e}", c.variant))?;
    let d = nominal(Cell {
        engine: c.engine,
        rung: c.rung,
    })?;
    build_from(c, &s, &d, None)
}

/// The same for any scenario and parameter set. `c` names the battle and gives its seed.
/// `troops`, if given, fill the bodies in place of the troops the scenario names: one for
/// each body, in the file's order.
pub fn build_from(
    c: &Config,
    s: &Scenario,
    d: &Drawn,
    troops: Option<&[Troop]>,
) -> Result<Built, String> {
    let troops = match troops {
        Some(t) if t.len() == s.bodies.len() => t.to_vec(),
        Some(t) => {
            return Err(format!(
                "{} troops for {} bodies",
                t.len(),
                s.bodies.len()
            ));
        }
        None => s
            .bodies
            .iter()
            .map(|b| {
                d.troop(&b.troop)
                    .ok_or_else(|| format!("unknown troop {:?}", b.troop))
            })
            .collect::<Result<Vec<_>, _>>()?,
    };
    let (bodies, placed) = place(s);
    let frames = frames_with(s, &placed, &bodies, |t| d.tie(t))?;
    let n = bodies.len();
    let rng = Rng::new(c.seed);
    let walls = s.walls();
    let volleys = s.volleys()?;
    let meta = meta_json(c, s, &bodies, &walls);
    let infos = s
        .bodies
        .iter()
        .zip(&placed)
        .map(|(b, p)| BodyInfo {
            name: b.name.clone(),
            side: bodies.side[p.first as usize],
            first: p.first,
            count: p.count,
            file_mm: b.file_mm,
            rank_mm: b.rank_mm,
            closer: b.closer.then(|| ManId(p.first + p.files * p.ranks)),
            tie: d.tie(b.tie),
        })
        .collect();
    macro_rules! battle {
        ($perception:expr, $memory:expr, $substrate:expr) => {{
            let mut men = Men::with_memory($memory, d.man, &bodies, &troops, &rng);
            men.power = d.power;
            let fighting = Spy {
                inner: (Bouts::new(d.bouts, men.kits()), volleys),
                read: Vec::new(),
            };
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
    Ok(Built {
        battle,
        meta,
        bodies: infos,
    })
}

pub fn engine_name(e: Engine) -> &'static str {
    match e {
        Engine::Pairwise => "pairwise",
        Engine::Field => "field",
    }
}

pub fn engine_of(name: &str) -> Option<Engine> {
    match name {
        "pairwise" => Some(Engine::Pairwise),
        "field" => Some(Engine::Field),
        _ => None,
    }
}

/// A string made safe inside a JSON string.
pub fn esc(s: &str) -> String {
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

pub fn ints(out: &mut String, values: impl Iterator<Item = i64>) {
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
