//! Frame dumps: the field as an SVG of circles. Seeing the patch is how nonsense gets caught
//! early.

use cannae_core::{Bodies, Posture, Role, Side};
use core::fmt::Write;

const RADIUS_MM: i32 = 280;
const MARGIN_MM: i32 = 3_000;

/// Renders every man as a circle: colour by side, fill by posture, a tick for his facing.
/// North is up, so world y is flipped.
pub fn render(bodies: &Bodies, title: &str) -> String {
    let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for p in &bodies.pos {
        x0 = x0.min(p.x);
        y0 = y0.min(p.y);
        x1 = x1.max(p.x);
        y1 = y1.max(p.y);
    }
    if bodies.is_empty() {
        (x0, y0, x1, y1) = (0, 0, 1, 1);
    }
    let (x0, y0, x1, y1) = (
        x0 - MARGIN_MM,
        y0 - MARGIN_MM,
        x1 + MARGIN_MM,
        y1 + MARGIN_MM,
    );
    let mut s = String::new();
    let _ = write!(
        s,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="1000">"#,
        x0,
        -y1,
        x1 - x0,
        y1 - y0
    );
    let _ = write!(
        s,
        r##"<rect x="{x0}" y="{}" width="{}" height="{}" fill="#f4f1ea"/><title>{title}</title>"##,
        -y1,
        x1 - x0,
        y1 - y0
    );
    for i in 0..bodies.len() {
        let p = bodies.pos[i];
        let colour = match bodies.side[i] {
            Side::A => "#a8322d",
            Side::B => "#2d5ea8",
        };
        let (fill, stroke, r) = match bodies.posture[i] {
            Posture::Steady => (colour, colour, RADIUS_MM),
            Posture::Forward => (colour, "#111", RADIUS_MM),
            Posture::Back => ("#f4f1ea", colour, RADIUS_MM),
            Posture::Turned => ("#e8c547", colour, RADIUS_MM),
            Posture::Down => ("#9a968c", "#9a968c", RADIUS_MM / 2),
        };
        let ring = match bodies.role[i] {
            Role::Leader | Role::Carrier => 80,
            _ => 40,
        };
        let _ = write!(
            s,
            r#"<circle cx="{}" cy="{}" r="{r}" fill="{fill}" stroke="{stroke}" stroke-width="{ring}"/>"#,
            p.x, -p.y
        );
        if bodies.posture[i] != Posture::Down {
            let f = bodies.facing[i].unit(RADIUS_MM * 2);
            let _ = write!(
                s,
                r##"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#111" stroke-width="40"/>"##,
                p.x,
                -p.y,
                p.x + f.x,
                -(p.y + f.y)
            );
        }
    }
    s.push_str("</svg>\n");
    s
}
