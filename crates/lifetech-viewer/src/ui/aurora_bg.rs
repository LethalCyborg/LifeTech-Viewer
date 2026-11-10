//! Animated LifeTech background (login, loading screens without a last
//! view): a dark sky with drifting, linked points of light, slow orange and
//! cyan blooms, and an orange grid floor in perspective flowing toward the
//! viewer under a glowing horizon (additive light).
//! Drawn with egui meshes: a few thousand vertices, no textures.

use crate::theme::Palette;
use egui::epaint::{Mesh, Vertex, WHITE_UV};
use egui::{Color32, Pos2, Rect, Shape};

fn v(pos: Pos2, color: Color32) -> Vertex {
    Vertex { pos, uv: WHITE_UV, color }
}

/// Additive light: premultiplied color with zero alpha adds to what is below.
fn glow(c: Color32, k: f32) -> Color32 {
    let k = k.clamp(0.0, 1.0);
    Color32::from_rgba_premultiplied((c.r() as f32 * k) as u8, (c.g() as f32 * k) as u8, (c.b() as f32 * k) as u8, 0)
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let m = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t) as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

/// Deterministic pseudo-random in 0..1.
fn hash(i: u32) -> f32 {
    let mut x = i.wrapping_mul(0x9E37_79B9) ^ 0x85EB_CA6B;
    x ^= x >> 15;
    x = x.wrapping_mul(0x2C1B_3C6D);
    x ^= x >> 12;
    (x & 0xFFFF) as f32 / 65535.0
}

/// Vertical gradient quad.
fn gradient(mesh: &mut Mesh, r: Rect, top: Color32, bottom: Color32) {
    let i = mesh.vertices.len() as u32;
    mesh.vertices.push(v(r.left_top(), top));
    mesh.vertices.push(v(r.right_top(), top));
    mesh.vertices.push(v(r.right_bottom(), bottom));
    mesh.vertices.push(v(r.left_bottom(), bottom));
    mesh.add_triangle(i, i + 1, i + 2);
    mesh.add_triangle(i, i + 2, i + 3);
}

/// Paint the background over `rect`; `t` = time in seconds, `alpha` fades it.
pub fn paint(painter: &egui::Painter, p: &Palette, rect: Rect, t: f32, alpha: f32) {
    painter.extend(shapes(p, rect, t, alpha, false));
}

/// The background as a frosted pane shows it: the same sky at the same
/// moment, out of focus (no stars or network, the grid dimmed, the blooms
/// wider). The cards of the login screen
/// draw it under their tint: the blur of the in-world windows only knows the
/// 3D scene, and there is none behind them.
pub fn frosted(p: &Palette, rect: Rect, t: f32) -> Vec<Shape> {
    shapes(p, rect, t, 1.0, true)
}

/// Shared login glass: a softened sky under the skin's lighter panel tint.
pub const LOGIN_TINT: f32 = 0.56;

/// The sky shown through a login card, with its own lights and panel tint.
pub fn frosted_pane(p: &Palette, screen: Rect, t: f32, pane: Rect, lights: Vec<Shape>) -> Vec<Shape> {
    let mut out = frosted(p, screen, t);
    out.extend(lights);
    let alpha = (f32::from(p.panel_alpha) * LOGIN_TINT) as u8;
    out.push(Shape::rect_filled(
        pane,
        0.0,
        Color32::from_rgba_unmultiplied(p.panel.r(), p.panel.g(), p.panel.b(), alpha),
    ));
    out
}

/// Thin quad from `a` to `b`, `width` wide, colors graded along it.
fn line(mesh: &mut Mesh, a: Pos2, b: Pos2, width: f32, ca: Color32, cb: Color32) {
    let d = b - a;
    let len = d.length();
    if len < 0.01 {
        return;
    }
    let n = egui::vec2(-d.y, d.x) / len * (width * 0.5);
    let i = mesh.vertices.len() as u32;
    mesh.vertices.push(v(a - n, ca));
    mesh.vertices.push(v(a + n, ca));
    mesh.vertices.push(v(b + n, cb));
    mesh.vertices.push(v(b - n, cb));
    mesh.add_triangle(i, i + 1, i + 2);
    mesh.add_triangle(i, i + 2, i + 3);
}

/// Where the grid floor meets the sky, as a fraction of the height.
const HORIZON: f32 = 0.64;

fn shapes(p: &Palette, rect: Rect, t: f32, alpha: f32, soft: bool) -> Vec<Shape> {
    let mut out = Vec::new();
    let a = alpha.clamp(0.0, 1.0);
    let (w, h) = (rect.width(), rect.height());
    let fade = |c: Color32| c.gamma_multiply(a);
    let horizon = rect.top() + h * HORIZON;
    let cx = rect.center().x;

    // ---- sky and floor
    let mut base = Mesh::default();
    let deep = mix(p.navy, Color32::BLACK, 0.5);
    let warm = mix(p.navy, p.indigo, 0.20);
    gradient(
        &mut base,
        Rect::from_min_max(rect.min, Pos2::new(rect.right(), horizon)),
        fade(deep),
        fade(warm),
    );
    gradient(
        &mut base,
        Rect::from_min_max(Pos2::new(rect.left(), horizon), rect.max),
        fade(mix(p.navy, p.indigo, 0.08)),
        fade(deep),
    );
    out.push(Shape::mesh(base));

    // ---- slow light blooms: orange low on the left, cyan high on the right
    let drift = |speed: f32, phase: f32| (t * speed + phase).sin();
    let span = w.max(h);
    out.push(radial_glow_shape(
        Pos2::new(
            rect.left() + w * (0.22 + 0.05 * drift(0.07, 0.0)),
            rect.top() + h * (0.55 + 0.04 * drift(0.05, 1.0)),
        ),
        span * if soft { 0.62 } else { 0.55 },
        p.violet,
        0.08 * a,
    ));
    out.push(radial_glow_shape(
        Pos2::new(
            rect.left() + w * (0.80 + 0.04 * drift(0.06, 2.0)),
            rect.top() + h * (0.18 + 0.04 * drift(0.04, 3.0)),
        ),
        span * if soft { 0.55 } else { 0.48 },
        p.teal,
        0.09 * a,
    ));
    // dawn on the horizon
    out.push(radial_glow_shape(
        Pos2::new(cx, horizon),
        w * 0.55,
        p.violet,
        (0.12 + 0.02 * drift(0.3, 0.0)) * a,
    ));

    // ---- faint stars, twinkling
    if !soft {
        for i in 0..90u32 {
            let x = rect.left() + hash(i * 3) * w;
            let y = rect.top() + hash(i * 3 + 1).powf(1.3) * h * (HORIZON - 0.04);
            let s = 0.4 + hash(i * 3 + 2) * 0.9;
            let tw = 0.5 + 0.5 * (t * (0.5 + hash(i + 977) * 1.5) + hash(i + 31) * std::f32::consts::TAU).sin();
            out.push(Shape::circle_filled(Pos2::new(x, y), s, glow(Color32::WHITE, 0.30 * tw * a)));
        }
    }

    // ---- network: drifting points of light, linked when near
    if !soft {
        const NODES: u32 = 44;
        let nodes: Vec<(Pos2, Color32, f32)> = (0..NODES)
            .map(|i| {
                let sp = 0.05 + hash(i * 7 + 3) * 0.08;
                let ph = hash(i * 7 + 4) * std::f32::consts::TAU;
                let x = rect.left() + (hash(i * 7) * 1.1 - 0.05) * w + (t * sp + ph).sin() * w * 0.03;
                let y = rect.top() + (0.04 + hash(i * 7 + 1) * (HORIZON - 0.10)) * h + (t * sp * 0.8 + ph * 1.3).cos() * h * 0.025;
                let c = match i % 5 {
                    0 | 3 => p.violet,
                    1 => p.teal,
                    2 => p.violet_light,
                    _ => p.ink,
                };
                let pulse = 0.65 + 0.35 * (t * (0.8 + hash(i * 7 + 5)) + ph).sin();
                (Pos2::new(x, y), c, pulse)
            })
            .collect();
        let reach = (w * w + h * h).sqrt() * 0.12;
        let mut links = Mesh::default();
        for (i, (pa, ca, ka)) in nodes.iter().enumerate() {
            for (pb, cb, kb) in nodes.iter().skip(i + 1) {
                let d = pa.distance(*pb);
                if d < reach {
                    let k = (1.0 - d / reach).powi(2) * 0.30 * a;
                    line(&mut links, *pa, *pb, 1.0, glow(*ca, k * ka), glow(*cb, k * kb));
                }
            }
        }
        out.push(Shape::mesh(links));
        for (i, (pos, c, k)) in nodes.iter().enumerate() {
            let r = 1.4 + hash(i as u32 * 7 + 6) * 1.4;
            out.push(radial_glow_shape(*pos, r * 5.0, *c, 0.22 * k * a));
            out.push(Shape::circle_filled(*pos, r, glow(*c, 0.9 * k * a)));
        }
    }

    // ---- grid floor in perspective, flowing toward the viewer
    let mut grid = Mesh::default();
    let floor = rect.bottom() - horizon;
    let strength = if soft { 0.45 } else { 1.0 };
    let spacing = w * 0.075;
    let lines = (w / spacing * 1.6) as i32 + 2;
    for j in -lines..=lines {
        let xb = cx + j as f32 * spacing * 1.3;
        let edge = 1.0 - (j.abs() as f32 / lines as f32).powf(1.5) * 0.7;
        line(
            &mut grid,
            Pos2::new(cx + (xb - cx) * 0.02, horizon),
            Pos2::new(xb, rect.bottom()),
            1.2,
            glow(p.violet, 0.0),
            glow(p.violet, 0.36 * edge * strength * a),
        );
    }
    const ROWS: u32 = 16;
    let flow = (t * 0.15).rem_euclid(1.0);
    for k in 0..ROWS {
        let s = (k as f32 + flow) / ROWS as f32;
        let y = horizon + floor * s.powf(2.2);
        let bright = 0.36 * s.powf(0.7) * strength * a;
        let thick = 0.8 + s * 1.0;
        let l = Pos2::new(rect.left(), y);
        let m = Pos2::new(cx, y);
        let r = Pos2::new(rect.right(), y);
        line(&mut grid, l, m, thick, glow(p.violet, bright * 0.5), glow(p.violet_light, bright));
        line(&mut grid, m, r, thick, glow(p.violet_light, bright), glow(p.violet, bright * 0.5));
    }
    out.push(Shape::mesh(grid));

    // ---- horizon line: brightest in the middle
    let mut edge = Mesh::default();
    let hl = Pos2::new(rect.left(), horizon);
    let hm = Pos2::new(cx, horizon);
    let hr = Pos2::new(rect.right(), horizon);
    let hot = 0.75 * a;
    line(&mut edge, hl, hm, 1.6, glow(p.violet, 0.1 * a), glow(p.violet_pale, hot));
    line(&mut edge, hm, hr, 1.6, glow(p.violet_pale, hot), glow(p.violet, 0.1 * a));
    // haze just above it
    let haze = Rect::from_min_max(Pos2::new(rect.left(), horizon - h * 0.08), Pos2::new(rect.right(), horizon));
    gradient(&mut edge, haze, glow(p.violet, 0.0), glow(p.violet, 0.10 * a));
    out.push(Shape::mesh(edge));
    out
}

/// Soft radial light (additive): `intensity` at the center fading to zero
/// at `radius`, smooth (no visible rings).
pub fn radial_glow(painter: &egui::Painter, center: Pos2, radius: f32, color: Color32, intensity: f32) {
    painter.add(radial_glow_shape(center, radius, color, intensity));
}

/// `radial_glow` as a shape, for a caller that orders its own shapes.
pub fn radial_glow_shape(center: Pos2, radius: f32, color: Color32, intensity: f32) -> Shape {
    let mut m = Mesh::default();
    const SEG: usize = 48;
    const RINGS: [(f32, f32); 4] = [(0.0, 1.0), (0.35, 0.62), (0.7, 0.22), (1.0, 0.0)];
    for (r, k) in RINGS {
        for s in 0..SEG {
            let a = s as f32 / SEG as f32 * std::f32::consts::TAU;
            let pos = center + egui::vec2(a.cos(), a.sin()) * radius * r;
            m.vertices.push(v(pos, glow(color, intensity * k)));
        }
    }
    for ring in 0..RINGS.len() as u32 - 1 {
        for s in 0..SEG as u32 {
            let (a0, a1) = (ring * SEG as u32 + s, ring * SEG as u32 + (s + 1) % SEG as u32);
            let (b0, b1) = (a0 + SEG as u32, a1 + SEG as u32);
            m.add_triangle(a0, b0, b1);
            m.add_triangle(a0, b1, a1);
        }
    }
    Shape::mesh(m)
}
