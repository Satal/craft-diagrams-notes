//! mermaid2vsdx: Mermaid flowchart -> .vsdx (+ .svg preview).

mod shapes;
mod svg;

use std::collections::HashMap;
use std::io::Read;
use std::process::ExitCode;

use craft_graph_layout as gl;
use craft_mermaid as mm;
use craft_vsdx as vx;

const FONT_PT: f64 = 12.0;
const CHAR_W: f64 = 7.2;
const LINE_H: f64 = 14.4;
const MARGIN: f64 = 24.0;
const GROUP_PAD: f64 = 18.0;
const GROUP_TITLE_H: f64 = 26.0;

struct Args {
    input: String,
    out: Option<String>,
    svg: Option<String>,
    direction: Option<mm::Direction>,
}

fn parse_args() -> Result<Args, String> {
    let mut input = None;
    let mut out = None;
    let mut svg = None;
    let mut direction = None;
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "-o" | "--output" => out = Some(it.next().ok_or("-o needs a value")?),
            "--svg" => svg = Some(it.next().ok_or("--svg needs a value")?),
            "--direction" | "-d" => {
                let v = it.next().ok_or("--direction needs a value")?;
                direction = Some(match v.to_ascii_uppercase().as_str() {
                    "TB" | "TD" => mm::Direction::TopToBottom,
                    "BT" => mm::Direction::BottomToTop,
                    "LR" => mm::Direction::LeftToRight,
                    "RL" => mm::Direction::RightToLeft,
                    _ => return Err(format!("unknown direction '{v}' (use TB, BT, LR or RL)")),
                });
            }
            "-h" | "--help" => {
                println!("usage: mermaid2vsdx INPUT.mmd|- -o OUT.vsdx [--svg OUT.svg] [--direction TB|BT|LR|RL]");
                std::process::exit(0);
            }
            s if s.starts_with('-') && s != "-" => return Err(format!("unknown option '{s}'")),
            _ => {
                if input.replace(a).is_some() {
                    return Err("more than one input given".into());
                }
            }
        }
    }
    let input = input.ok_or("no input given (use a file or '-' for stdin)")?;
    Ok(Args { input, out, svg, direction })
}

pub fn text_size(s: &str) -> (f64, f64) {
    let lines: Vec<&str> = s.split('\n').collect();
    let w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) as f64 * CHAR_W;
    (w, lines.len() as f64 * LINE_H)
}

fn colour(s: &str) -> Option<Option<vx::Rgba>> {
    let t = s.trim().to_ascii_lowercase();
    if t == "none" || t == "transparent" {
        return Some(None);
    }
    if let Some(h) = t.strip_prefix('#') {
        let d: Option<Vec<u8>> = h.chars().map(|c| c.to_digit(16).map(|v| v as u8)).collect();
        let d = d?;
        return match d.len() {
            3 => Some(Some(vx::Rgba::rgb(d[0] * 17, d[1] * 17, d[2] * 17))),
            6 => Some(Some(vx::Rgba::rgb(d[0] * 16 + d[1], d[2] * 16 + d[3], d[4] * 16 + d[5]))),
            _ => None,
        };
    }
    if let Some(r) = t.strip_prefix("rgb(").and_then(|r| r.strip_suffix(')')) {
        let v: Vec<u8> = r.split(',').filter_map(|p| p.trim().parse::<f64>().ok()).map(|x| x.clamp(0.0, 255.0) as u8).collect();
        if v.len() == 3 {
            return Some(Some(vx::Rgba::rgb(v[0], v[1], v[2])));
        }
        return None;
    }
    let (r, g, b) = match t.as_str() {
        "black" => (0, 0, 0),
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 128, 0),
        "lime" => (0, 255, 0),
        "blue" => (0, 0, 255),
        "yellow" => (255, 255, 0),
        "orange" => (255, 165, 0),
        "purple" => (128, 0, 128),
        "pink" => (255, 192, 203),
        "gray" | "grey" => (128, 128, 128),
        "lightgray" | "lightgrey" => (211, 211, 211),
        "darkgray" | "darkgrey" => (169, 169, 169),
        "brown" => (165, 42, 42),
        "cyan" | "aqua" => (0, 255, 255),
        "magenta" | "fuchsia" => (255, 0, 255),
        "navy" => (0, 0, 128),
        "teal" => (0, 128, 128),
        "lightblue" => (173, 216, 230),
        "lightgreen" => (144, 238, 144),
        "gold" => (255, 215, 0),
        "salmon" => (250, 128, 114),
        "crimson" => (220, 20, 60),
        _ => return None,
    };
    Some(Some(vx::Rgba::rgb(r, g, b)))
}

fn apply_colour(what: &str, name: &str, val: &Option<String>, warns: &mut Vec<String>) -> Option<Option<vx::Rgba>> {
    let v = val.as_ref()?;
    match colour(v) {
        Some(c) => Some(c),
        None => {
            warns.push(format!("{what}: unsupported {name} colour '{v}', ignored"));
            None
        }
    }
}

fn depth(sgs: &[mm::Subgraph], mut i: usize) -> usize {
    let mut d = 0;
    while let Some(p) = sgs[i].parent {
        d += 1;
        i = p;
        if d > 64 {
            break;
        }
    }
    d
}

/// First node (depth first) inside subgraph `i`.
fn first_node(fc: &mm::Flowchart, i: usize) -> Option<usize> {
    if let Some(&n) = fc.subgraphs[i].members.first() {
        return Some(n);
    }
    (0..fc.subgraphs.len()).filter(|&c| fc.subgraphs[c].parent == Some(i)).find_map(|c| first_node(fc, c))
}

/// Moves end point `p` (on the box side, polyline neighbour `q`) along the segment axis onto the
/// real outline of ellipse and straight-edged polygon shapes, so ends touch the shape.
fn snap(s: &vx::Shape, p: (f64, f64), q: (f64, f64)) -> (f64, f64) {
    let Some(g) = s.geometry.first() else { return p };
    let (lx, ly) = (p.0 - s.x, p.1 - s.y);
    // direction pointing into the shape (opposite to the neighbour)
    let (dx, dy) = (p.0 - q.0, p.1 - q.1);
    let l = dx.hypot(dy);
    if l < 1e-9 {
        return p;
    }
    let (ux, uy) = (dx / l, dy / l);
    let mut best: Option<f64> = None;
    let mut consider = |t: f64| {
        if t >= -1e-6 && best.is_none_or(|b| t < b) {
            best = Some(t.max(0.0));
        }
    };
    if let Some(vx::Segment::Ellipse { centre, a, b }) = g.segments.first() {
        let rx = (a.0 - centre.0).abs().max(1e-6);
        let ry = (b.1 - centre.1).abs().max(1e-6);
        let (ox, oy) = ((lx - centre.0) / rx, (ly - centre.1) / ry);
        let (vx_, vy) = (ux / rx, uy / ry);
        let (qa, qb, qc) = (vx_ * vx_ + vy * vy, 2.0 * (ox * vx_ + oy * vy), ox * ox + oy * oy - 1.0);
        let disc = qb * qb - 4.0 * qa * qc;
        if disc >= 0.0 {
            consider((-qb - disc.sqrt()) / (2.0 * qa));
            consider((-qb + disc.sqrt()) / (2.0 * qa));
        }
    } else {
        let mut pts = Vec::new();
        for sg in &g.segments {
            match sg {
                vx::Segment::MoveTo(p) | vx::Segment::LineTo(p) => pts.push(*p),
                _ => return p,
            }
        }
        for w in pts.windows(2) {
            let (a, b) = (w[0], w[1]);
            let (ex, ey) = (b.0 - a.0, b.1 - a.1);
            let den = ux * ey - uy * ex;
            if den.abs() < 1e-9 {
                continue;
            }
            let t = ((a.0 - lx) * ey - (a.1 - ly) * ex) / den;
            let u = ((a.0 - lx) * uy - (a.1 - ly) * ux) / den;
            if (-1e-9..=1.0 + 1e-9).contains(&u) {
                consider(t);
            }
        }
    }
    match best {
        Some(t) if t < 400.0 => (p.0 + ux * t, p.1 + uy * t),
        _ => p,
    }
}

fn nearest_side(s: &vx::Shape, p: (f64, f64)) -> u32 {
    // connection points: 1 top, 2 right, 3 bottom, 4 left (box-local); p is page space
    let d = |c: (f64, f64)| ((s.x + c.0) - p.0).hypot((s.y + c.1) - p.1);
    let mut best = (1, f64::MAX);
    for (i, c) in s.connections.iter().enumerate() {
        let v = d(*c);
        if v < best.1 {
            best = (i as u32 + 1, v);
        }
    }
    best.0
}

fn run() -> Result<(), String> {
    let args = parse_args()?;
    let text = if args.input == "-" {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).map_err(|e| format!("reading stdin: {e}"))?;
        s
    } else {
        std::fs::read_to_string(&args.input).map_err(|e| format!("reading {}: {e}", args.input))?
    };
    let out = match (&args.out, args.input.as_str()) {
        (Some(o), _) => o.clone(),
        (None, "-") => return Err("-o is required when reading stdin".into()),
        (None, p) => std::path::Path::new(p).with_extension("vsdx").to_string_lossy().into_owned(),
    };

    let fc = match mm::parse(&text).map_err(|e| format!("parse error: {e}"))? {
        mm::Diagram::Flowchart(f) => f,
        mm::Diagram::Unsupported { kind } => {
            return Err(format!("unsupported diagram type '{kind}': only flowchart/graph diagrams can be converted"));
        }
    };
    let mut warns: Vec<String> = fc.warnings.iter().map(|w| format!("line {}: {}", w.line, w.message)).collect();

    // --- node sizing and shape geometry
    let mut fc = fc;
    for n in &mut fc.nodes {
        n.label = n.label.replace("\\n", "\n");
    }
    let mut shape_list: Vec<shapes::Built> = Vec::new();
    for n in &fc.nodes {
        let (b, w) = shapes::build(n);
        if let Some(w) = w {
            warns.push(w);
        }
        shape_list.push(b);
    }

    // --- layout
    let direction = args.direction.unwrap_or(fc.direction);
    let node_group: Vec<Option<usize>> =
        (0..fc.nodes.len()).map(|i| fc.subgraphs.iter().position(|s| s.members.contains(&i))).collect();
    let graph = gl::Graph {
        nodes: shape_list.iter().enumerate().map(|(i, b)| gl::NodeSpec { size: gl::Size::new(b.w, b.h), group: node_group[i] }).collect(),
        edges: {
            let mut v = Vec::new();
            for (i, e) in fc.edges.iter().enumerate() {
                let ep = |x: mm::Endpoint, warns: &mut Vec<String>| match x {
                    mm::Endpoint::Node(n) => Some(n),
                    mm::Endpoint::Subgraph(s) => {
                        let n = first_node(&fc, s);
                        match n {
                            Some(n) => warns.push(format!("edge {i}: endpoint is subgraph '{}', attached to its first node '{}'", fc.subgraphs[s].id, fc.nodes[n].id)),
                            None => warns.push(format!("edge {i}: subgraph '{}' is empty, edge dropped", fc.subgraphs[s].id)),
                        }
                        n
                    }
                };
                let (a, b) = (ep(e.from, &mut warns), ep(e.to, &mut warns));
                let (Some(a), Some(b)) = (a, b) else {
                    // keep indices aligned: self-edge placeholder flagged below
                    v.push(gl::EdgeSpec::new(usize::MAX, usize::MAX));
                    continue;
                };
                let label = e.label.as_ref().filter(|l| !l.is_empty()).map(|l| {
                    let (w, h) = text_size(l);
                    gl::Size::new(w + 10.0, h + 4.0)
                });
                v.push(gl::EdgeSpec { from: a, to: b, min_len: u32::from(e.length), label });
            }
            v
        },
        groups: fc
            .subgraphs
            .iter()
            .map(|s| gl::GroupSpec { parent: s.parent, padding: GROUP_PAD, title_height: if s.title.is_empty() { 0.0 } else { GROUP_TITLE_H } })
            .collect(),
    };
    let dropped: Vec<bool> = graph.edges.iter().map(|e| e.from == usize::MAX).collect();
    let mut lg = graph.clone();
    let mut map = Vec::new(); // layout edge idx -> fc edge idx
    lg.edges = graph.edges.iter().enumerate().filter(|(i, _)| !dropped[*i]).map(|(i, e)| {
        map.push(i);
        *e
    }).collect();
    let opts = gl::LayeredOptions {
        direction: match direction {
            mm::Direction::TopToBottom => gl::Direction::Down,
            mm::Direction::BottomToTop => gl::Direction::Up,
            mm::Direction::LeftToRight => gl::Direction::Right,
            mm::Direction::RightToLeft => gl::Direction::Left,
        },
        ..gl::LayeredOptions::default()
    };
    let layout = gl::layered(&lg, &opts).map_err(|e| format!("layout failed: {e}"))?;

    // --- build the drawing (layout units are points)
    let mut page = vx::Page { name: "Flowchart".into(), width: layout.size.width + 2.0 * MARGIN, height: layout.size.height + 2.0 * MARGIN, ..Default::default() };
    let mut next_id = 1u32;

    // groups, outermost first so inner ones draw over them
    let mut order: Vec<usize> = (0..fc.subgraphs.len()).collect();
    order.sort_by_key(|&i| (depth(&fc.subgraphs, i), i));
    for &gi in &order {
        let sg = &fc.subgraphs[gi];
        let r = layout.groups[gi];
        let mut st = sg.style.clone();
        for c in &sg.classes {
            for d in fc.class_defs.iter().filter(|d| &d.name == c) {
                let mut base = d.style.clone();
                base.merge(&st);
                st = base;
            }
        }
        let what = format!("subgraph '{}'", sg.id);
        let fill = apply_colour(&what, "fill", &st.fill, &mut warns).unwrap_or(Some(vx::Rgba::rgb(0xFF, 0xFF, 0xDE)));
        let line = apply_colour(&what, "stroke", &st.stroke, &mut warns).unwrap_or(Some(vx::Rgba::rgb(0xAA, 0xAA, 0x33)));
        let tc = apply_colour(&what, "color", &st.color, &mut warns).flatten();
        let (x, y) = (r.x + MARGIN, r.y + MARGIN);
        page.shapes.push(vx::Shape {
            id: next_id,
            name: format!("Group {}", sg.id),
            x,
            y,
            width: r.width,
            height: r.height,
            geometry: vec![vx::GeomPath::rect(r.width, r.height)],
            fill,
            line,
            line_weight: st.stroke_width.unwrap_or(1.0),
            dashed: st.stroke_dasharray.is_some(),
            ..Default::default()
        });
        next_id += 1;
        if !sg.title.is_empty() {
            page.shapes.push(vx::Shape {
                id: next_id,
                name: format!("Title {}", sg.id),
                x,
                y,
                width: r.width,
                height: GROUP_TITLE_H,
                text: sg.title.clone(),
                text_color: tc,
                ..Default::default()
            });
            next_id += 1;
        }
    }

    // nodes
    let mut node_ids = Vec::new();
    let mut node_page_idx = Vec::new();
    for (i, n) in fc.nodes.iter().enumerate() {
        let b = &shape_list[i];
        let r = layout.nodes[i];
        // keep the laid-out box but scale the geometry if the layout changed the size (it does not)
        let st = fc.node_style(i);
        let what = format!("node '{}'", n.id);
        let fill = apply_colour(&what, "fill", &st.fill, &mut warns).unwrap_or(b.default_fill);
        let line = apply_colour(&what, "stroke", &st.stroke, &mut warns).unwrap_or(b.default_line);
        let tc = apply_colour(&what, "color", &st.color, &mut warns).flatten();
        let id = next_id;
        next_id += 1;
        node_ids.push(id);
        node_page_idx.push(page.shapes.len());
        page.shapes.push(vx::Shape {
            id,
            name: n.id.clone(),
            x: r.x + MARGIN,
            y: r.y + MARGIN,
            width: r.width,
            height: r.height,
            geometry: b.geometry.clone(),
            text: b.text.clone(),
            fill,
            line,
            line_weight: st.stroke_width.unwrap_or(1.25),
            dashed: st.stroke_dasharray.is_some(),
            text_color: tc,
            connections: vec![(r.width / 2.0, 0.0), (r.width, r.height / 2.0), (r.width / 2.0, r.height), (0.0, r.height / 2.0)],
            ..Default::default()
        });
    }

    // edges
    let mut warned_heads: HashMap<&'static str, ()> = HashMap::new();
    for (li, &ei) in map.iter().enumerate() {
        let e = &fc.edges[ei];
        if e.stroke == mm::Stroke::Invisible {
            continue;
        }
        let pts = &layout.edges[li];
        if pts.len() < 2 {
            warns.push(format!("edge {ei}: layout gave no route, dropped"));
            continue;
        }
        let mut points: Vec<(f64, f64)> = pts.iter().map(|p| (p.x + MARGIN, p.y + MARGIN)).collect();
        let (from_n, to_n) = (lg.edges[li].from, lg.edges[li].to);
        let fs = &page.shapes[node_page_idx[from_n]];
        let ts = &page.shapes[node_page_idx[to_n]];
        let n = points.len();
        let (p0, pe) = (snap(fs, points[0], points[1]), snap(ts, points[n - 1], points[n - 2]));
        points[0] = p0;
        points[n - 1] = pe;
        let from = (node_ids[from_n], vx::ConnectionPoint::Point(nearest_side(fs, points[0])));
        let to = (node_ids[to_n], vx::ConnectionPoint::Point(nearest_side(ts, points[points.len() - 1])));
        let mut head = |h: mm::Head| match h {
            mm::Head::None => 0,
            mm::Head::Arrow => vx::ARROW,
            mm::Head::Circle => {
                if warned_heads.insert("circle", ()).is_none() {
                    warns.push("circle edge ends (--o) drawn as arrowheads".into());
                }
                vx::ARROW
            }
            mm::Head::Cross => {
                if warned_heads.insert("cross", ()).is_none() {
                    warns.push("cross edge ends (--x) drawn as arrowheads".into());
                }
                vx::ARROW
            }
        };
        let (begin_arrow, end_arrow) = (head(e.start_head), head(e.end_head));
        let st = fc.edge_style(ei);
        let what = format!("edge {ei}");
        let line = apply_colour(&what, "stroke", &st.stroke, &mut warns).unwrap_or(Some(vx::Rgba::rgb(0x33, 0x33, 0x33)));
        let weight = st.stroke_width.unwrap_or(if e.stroke == mm::Stroke::Thick { 3.0 } else { 1.25 });
        let id = next_id;
        next_id += 1;
        page.connectors.push(vx::Connector {
            id,
            from: Some(from),
            to: Some(to),
            points,
            begin_arrow,
            end_arrow,
            line,
            line_weight: weight,
            dashed: e.stroke == mm::Stroke::Dotted || st.stroke_dasharray.is_some(),
            label: e.label.clone().unwrap_or_default(),
        });
    }

    let drawing = vx::Drawing { pages: vec![page] };
    let bytes = vx::write(&drawing).map_err(|e| format!("writing vsdx: {e}"))?;
    std::fs::write(&out, bytes).map_err(|e| format!("writing {out}: {e}"))?;
    if let Some(p) = &args.svg {
        std::fs::write(p, svg::render(&drawing.pages[0], FONT_PT)).map_err(|e| format!("writing {p}: {e}"))?;
    }
    for w in warns {
        eprintln!("warning: {w}");
    }
    for w in &fc.notes {
        eprintln!("note: line {}: {}", w.line, w.message);
    }
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
