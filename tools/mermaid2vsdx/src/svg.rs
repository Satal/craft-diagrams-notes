//! SVG preview of the drawing model (same geometry and text the vsdx carries).

use craft_vsdx::{Page, Rgba, Segment, Shape};
use std::fmt::Write;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn col(c: Option<Rgba>) -> String {
    match c {
        None => "none".into(),
        Some(c) => format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b),
    }
}

fn path_d(shape: &Shape, p: &craft_vsdx::GeomPath) -> String {
    let (ox, oy) = (shape.x, shape.y);
    let mut d = String::new();
    for s in &p.segments {
        match *s {
            Segment::MoveTo(q) => write!(d, "M{:.2} {:.2} ", q.0 + ox, q.1 + oy),
            Segment::LineTo(q) => write!(d, "L{:.2} {:.2} ", q.0 + ox, q.1 + oy),
            Segment::CubicTo { c1, c2, to } => write!(d, "C{:.2} {:.2} {:.2} {:.2} {:.2} {:.2} ", c1.0 + ox, c1.1 + oy, c2.0 + ox, c2.1 + oy, to.0 + ox, to.1 + oy),
            Segment::ArcTo { to, .. } => write!(d, "L{:.2} {:.2} ", to.0 + ox, to.1 + oy),
            Segment::Ellipse { centre, a, b } => {
                let rx = (a.0 - centre.0).hypot(a.1 - centre.1);
                let ry = (b.0 - centre.0).hypot(b.1 - centre.1);
                let (cx, cy) = (centre.0 + ox, centre.1 + oy);
                write!(d, "M{:.2} {:.2} A{:.2} {:.2} 0 1 0 {:.2} {:.2} A{:.2} {:.2} 0 1 0 {:.2} {:.2} ", cx - rx, cy, rx, ry, cx + rx, cy, rx, ry, cx - rx, cy)
            }
        }
        .ok();
    }
    if p.closed {
        d.push('Z');
    }
    d
}

fn text(out: &mut String, cx: f64, cy: f64, s: &str, colour: &str, bg: bool) {
    let lines: Vec<&str> = s.split('\n').collect();
    let lh = 14.4;
    let y0 = cy - (lines.len() as f64 - 1.0) * lh / 2.0;
    if bg {
        let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as f64 * 7.2 + 6.0;
        let h = lines.len() as f64 * lh + 2.0;
        writeln!(out, "<rect x=\"{:.2}\" y=\"{:.2}\" width=\"{:.2}\" height=\"{:.2}\" fill=\"#fff\" fill-opacity=\"0.85\"/>", cx - w / 2.0, cy - h / 2.0, w, h).ok();
    }
    write!(out, "<text x=\"{cx:.2}\" text-anchor=\"middle\" fill=\"{colour}\">").ok();
    for (i, l) in lines.iter().enumerate() {
        write!(out, "<tspan x=\"{cx:.2}\" y=\"{:.2}\" dy=\"0.35em\">{}</tspan>", y0 + i as f64 * lh, esc(l)).ok();
    }
    out.push_str("</text>\n");
}

pub fn render(page: &Page, font: f64) -> String {
    let mut o = String::new();
    writeln!(o, "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.0}\" height=\"{h:.0}\" viewBox=\"0 0 {w:.2} {h:.2}\" font-family=\"Calibri, Arial, sans-serif\" font-size=\"{font}\">", w = page.width, h = page.height).ok();
    o.push_str("<defs><marker id=\"a\" viewBox=\"0 0 10 10\" refX=\"9\" refY=\"5\" markerWidth=\"8\" markerHeight=\"8\" orient=\"auto-start-reverse\" markerUnits=\"userSpaceOnUse\"><path d=\"M0 0L10 5L0 10Z\" fill=\"context-stroke\"/></marker></defs>\n");
    writeln!(o, "<rect width=\"100%\" height=\"100%\" fill=\"#fff\"/>").ok();
    for s in &page.shapes {
        for p in &s.geometry {
            let fill = if p.no_fill { "none".into() } else { col(s.fill) };
            let stroke = if p.no_line { "none".into() } else { col(s.line) };
            let dash = if s.dashed { " stroke-dasharray=\"6 4\"" } else { "" };
            writeln!(o, "<path d=\"{}\" fill=\"{}\" stroke=\"{}\" stroke-width=\"{}\"{}/>", path_d(s, p), fill, stroke, s.line_weight, dash).ok();
        }
        if !s.text.is_empty() {
            let c = col(Some(s.text_color.unwrap_or(Rgba::BLACK)));
            text(&mut o, s.x + s.width / 2.0, s.y + s.height / 2.0, &s.text, &c, false);
        }
    }
    for c in &page.connectors {
        let pts: Vec<String> = c.points.iter().map(|p| format!("{:.2},{:.2}", p.0, p.1)).collect();
        let dash = if c.dashed { " stroke-dasharray=\"6 4\"" } else { "" };
        let ms = if c.begin_arrow != 0 { " marker-start=\"url(#a)\"" } else { "" };
        let me = if c.end_arrow != 0 { " marker-end=\"url(#a)\"" } else { "" };
        writeln!(o, "<polyline points=\"{}\" fill=\"none\" stroke=\"{}\" stroke-width=\"{}\"{}{}{}/>", pts.join(" "), col(c.line), c.line_weight, dash, ms, me).ok();
        if !c.label.is_empty() && c.points.len() >= 2 {
            // midpoint along the polyline
            let total: f64 = c.points.windows(2).map(|w| (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1)).sum();
            let mut acc = 0.0;
            let mut mid = c.points[0];
            for w in c.points.windows(2) {
                let l = (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1);
                if acc + l >= total / 2.0 && l > 0.0 {
                    let t = (total / 2.0 - acc) / l;
                    mid = (w[0].0 + (w[1].0 - w[0].0) * t, w[0].1 + (w[1].1 - w[0].1) * t);
                    break;
                }
                acc += l;
            }
            text(&mut o, mid.0, mid.1, &c.label, "#333333", true);
        }
    }
    o.push_str("</svg>\n");
    o
}
