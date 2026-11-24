//! Node sizing and Mermaid shape -> vsdx geometry.

use crate::text_size;
use craft_mermaid::{Node, Shape as S};
use craft_vsdx::{GeomPath, Rgba, Segment};

pub struct Built {
    pub w: f64,
    pub h: f64,
    pub geometry: Vec<GeomPath>,
    pub text: String,
    pub default_fill: Option<Rgba>,
    pub default_line: Option<Rgba>,
}

const K: f64 = 0.5523;

fn line_path(pts: &[(f64, f64)]) -> GeomPath {
    let mut segments = vec![Segment::MoveTo(pts[0])];
    segments.extend(pts[1..].iter().map(|&p| Segment::LineTo(p)));
    GeomPath { segments, closed: false, no_fill: true, ..GeomPath::default() }
}

fn rounded(w: f64, h: f64, r: f64) -> GeomPath {
    let r = r.min(w / 2.0).min(h / 2.0);
    let k = r * (1.0 - K);
    let segments = vec![
        Segment::MoveTo((r, 0.0)),
        Segment::LineTo((w - r, 0.0)),
        Segment::CubicTo { c1: (w - k, 0.0), c2: (w, k), to: (w, r) },
        Segment::LineTo((w, h - r)),
        Segment::CubicTo { c1: (w, h - k), c2: (w - k, h), to: (w - r, h) },
        Segment::LineTo((r, h)),
        Segment::CubicTo { c1: (k, h), c2: (0.0, h - k), to: (0.0, h - r) },
        Segment::LineTo((0.0, r)),
        Segment::CubicTo { c1: (0.0, k), c2: (k, 0.0), to: (r, 0.0) },
    ];
    GeomPath { segments, closed: true, ..GeomPath::default() }
}

fn cylinder(w: f64, h: f64, e: f64) -> Vec<GeomPath> {
    let (hw, ke, kw) = (w / 2.0, e * K, w / 2.0 * K);
    let outline = GeomPath {
        segments: vec![
            Segment::MoveTo((0.0, e)),
            Segment::CubicTo { c1: (0.0, e - ke), c2: (hw - kw, 0.0), to: (hw, 0.0) },
            Segment::CubicTo { c1: (hw + kw, 0.0), c2: (w, e - ke), to: (w, e) },
            Segment::LineTo((w, h - e)),
            Segment::CubicTo { c1: (w, h - e + ke), c2: (hw + kw, h), to: (hw, h) },
            Segment::CubicTo { c1: (hw - kw, h), c2: (0.0, h - e + ke), to: (0.0, h - e) },
        ],
        closed: true,
        ..GeomPath::default()
    };
    let front = GeomPath {
        segments: vec![
            Segment::MoveTo((0.0, e)),
            Segment::CubicTo { c1: (0.0, e + ke), c2: (hw - kw, 2.0 * e), to: (hw, 2.0 * e) },
            Segment::CubicTo { c1: (hw + kw, 2.0 * e), c2: (w, e + ke), to: (w, e) },
        ],
        closed: false,
        no_fill: true,
        ..GeomPath::default()
    };
    vec![outline, front]
}

fn document(w: f64, h: f64, d: f64) -> GeomPath {
    GeomPath {
        segments: vec![
            Segment::MoveTo((0.0, 0.0)),
            Segment::LineTo((w, 0.0)),
            Segment::LineTo((w, h - d)),
            Segment::CubicTo { c1: (0.8 * w, h - 2.0 * d), c2: (0.7 * w, h - 2.0 * d), to: (w / 2.0, h - d) },
            Segment::CubicTo { c1: (0.3 * w, h), c2: (0.2 * w, h), to: (0.0, h - d) },
        ],
        closed: true,
        ..GeomPath::default()
    }
}

pub fn build(n: &Node) -> (Built, Option<String>) {
    let label = n.label.clone();
    let (tw, th) = text_size(&label);
    let mut warn = None;
    let lavender = Some(Rgba::rgb(0xEC, 0xEC, 0xFF));
    let purple = Some(Rgba::rgb(0x93, 0x70, 0xDB));
    let (mut fill, mut line) = (lavender, purple);
    let mut text = label.clone();
    let min = |w: f64, h: f64, mw: f64, mh: f64| (w.max(mw), h.max(mh));
    let (w, h);
    let geometry;
    match n.shape {
        S::Rect => {
            (w, h) = min(tw + 28.0, th + 20.0, 64.0, 36.0);
            geometry = vec![GeomPath::rect(w, h)];
        }
        S::Text => {
            (w, h) = min(tw + 20.0, th + 12.0, 24.0, 24.0);
            geometry = vec![GeomPath::rect(w, h)];
            fill = None;
            line = None;
        }
        S::Rounded => {
            (w, h) = min(tw + 28.0, th + 20.0, 64.0, 36.0);
            geometry = vec![rounded(w, h, 10.0)];
        }
        S::Stadium => {
            (w, h) = min(tw + 40.0, th + 20.0, 72.0, 36.0);
            geometry = vec![rounded(w, h, h / 2.0)];
        }
        S::Subroutine => {
            (w, h) = min(tw + 44.0, th + 20.0, 80.0, 36.0);
            geometry = vec![GeomPath::rect(w, h), line_path(&[(9.0, 0.0), (9.0, h)]), line_path(&[(w - 9.0, 0.0), (w - 9.0, h)])];
        }
        S::Cylinder => {
            let e = 8.0;
            (w, h) = min(tw + 28.0, th + 20.0 + 3.0 * e, 64.0, 56.0);
            geometry = cylinder(w, h, e);
        }
        S::Circle => {
            let d = (tw.hypot(th) + 12.0).max(48.0);
            (w, h) = (d, d);
            geometry = vec![GeomPath::ellipse(w, h)];
        }
        S::SmallCircle | S::FilledCircle => {
            (w, h) = (18.0, 18.0);
            geometry = vec![GeomPath::ellipse(w, h)];
            fill = Some(Rgba::rgb(0x33, 0x33, 0x33));
            text.clear();
        }
        S::DoubleCircle => {
            let d = (tw.hypot(th) + 24.0).max(60.0);
            (w, h) = (d, d);
            let mut inner = GeomPath::ellipse(w - 12.0, h - 12.0);
            for s in &mut inner.segments {
                if let Segment::Ellipse { centre, a, b } = s {
                    for p in [centre, a, b] {
                        p.0 += 6.0;
                        p.1 += 6.0;
                    }
                }
            }
            inner.no_fill = true;
            geometry = vec![GeomPath::ellipse(w, h), inner];
        }
        S::Diamond => {
            (w, h) = min(tw * 1.6 + 36.0, th * 1.7 + 30.0, 88.0, 60.0);
            geometry = vec![GeomPath::diamond(w, h)];
        }
        S::Hexagon => {
            (w, h) = min(tw + 56.0, th + 20.0, 88.0, 40.0);
            let i = (h * 0.3).min(w / 4.0);
            geometry = vec![GeomPath::polygon(&[(i, 0.0), (w - i, 0.0), (w, h / 2.0), (w - i, h), (i, h), (0.0, h / 2.0)])];
        }
        S::Asymmetric => {
            (w, h) = min(tw + 40.0, th + 20.0, 72.0, 36.0);
            geometry = vec![GeomPath::polygon(&[(0.0, 0.0), (w, 0.0), (w, h), (0.0, h), (h * 0.3, h / 2.0)])];
        }
        S::LeanRight | S::LeanLeft | S::TrapezoidBottom | S::TrapezoidTop => {
            (w, h) = min(tw + 48.0, th + 20.0, 84.0, 36.0);
            let s = h * 0.3;
            let pts: [(f64, f64); 4] = match n.shape {
                S::LeanRight => [(s, 0.0), (w, 0.0), (w - s, h), (0.0, h)],
                S::LeanLeft => [(0.0, 0.0), (w - s, 0.0), (w, h), (s, h)],
                S::TrapezoidBottom => [(s, 0.0), (w - s, 0.0), (w, h), (0.0, h)],
                _ => [(0.0, 0.0), (w, 0.0), (w - s, h), (s, h)],
            };
            geometry = vec![GeomPath::polygon(&pts)];
        }
        S::Document => {
            let d = 8.0;
            (w, h) = min(tw + 28.0, th + 20.0 + d, 64.0, 44.0);
            geometry = vec![document(w, h, d)];
        }
        other => {
            (w, h) = min(tw + 28.0, th + 20.0, 64.0, 36.0);
            geometry = vec![GeomPath::rect(w, h)];
            warn = Some(format!("node '{}': shape {:?} not supported, drawn as a plain rectangle", n.id, other));
        }
    }
    (Built { w, h, geometry, text, default_fill: fill, default_line: line }, warn)
}
