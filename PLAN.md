# Diagramming for FigCraft Boards — plan

Approved scope (user, 2026-11-21): build locally, no PRs/issues/comments until the user has used it.
Order: Mermaid import + auto-layout + obstacle-avoiding connectors + shapes → file formats (.vsdx) → stencils (.vssx).

Workspace: `/home/satal/code/craft-diagrams/` — `craft-libs/`, `figcraft/`, `craft-contrib/` (rules), `research/`, `tools/`.
Builds: **only** through `tools/dcargo <repo-dir> <cargo args>` (Docker `craft-rust:1.95`, capped 5 GB / 3 CPUs, serialised by flock — a build may wait for another agent's). Never install toolchains on the host.

## Rules every agent follows
- Read `<repo>/AGENTS.md` fully before writing code. It overrides defaults. Key: clean room (rules 23–26), tests assert relations with one `tests/it/main.rs` per crate (27–31), no panics in non-test code, no `allow` to silence lints, English-key i18n, accessibility for UI (21), naming (22: never "Visio"; say "VSDX drawing", "diagram", "flowchart").
- Clean room: algorithms from papers only (cite in crate docs); Mermaid from `research/mermaid-flowchart-spec.md` (docs-derived) only — never read mermaid-js or any other parser source; VSDX from `research/vsdx-spec.md` (MS-VSDX) only — never read MisterDiagram, libvisio or any other implementation.
- No new external dependencies unless already in the repo's lockfile with an allowlisted licence.
- Work on a local git branch; commit locally with short imperative subjects in the repo's style; **do not push, do not touch GitHub**. Never mention AI/Claude in commits.
- Before finishing: `dcargo <repo> fmt`, `clippy -p <crate> --all-targets -- -D warnings`, `test -p <crate>`; report exactly what ran.

## Work packages

### WP1 craft-libs `craft-graph-layout` (crates/graph-layout) — no dependencies
Pure geometry in f64, y-down. API sketch (implementer may refine, keep it small):
- `Graph { nodes: Vec<NodeSpec{ size: (w,h), group: Option<usize> }>, edges: Vec<EdgeSpec{ from, to, min_len: u32 }>, groups: Vec<GroupSpec{ parent: Option<usize>, padding }> }`
- `layered(&Graph, &LayeredOptions{ direction: Down|Up|Right|Left, node_sep, rank_sep, ... }) -> Result<Layout, LayoutError>`; `Layout { nodes: Vec<Rect>, groups: Vec<Rect>, edges: Vec<Vec<Point>> }` (orthogonal polylines).
- Sugiyama pipeline: cycle removal (greedy, Eades–Lin–Smyth 1993), layering (longest path then network-simplex, Gansner et al. 1993, honouring min_len), dummy nodes for long edges, crossing reduction (layer sweep, barycentre/median, Gansner 1993 / Sugiyama 1981), coordinate assignment (Brandes & Köpf 2001), group (subgraph) containment as contiguous blocks.
- `route_orthogonal(obstacles: &[Rect], from: Port, to: Port, opts) -> Option<Vec<Point>>`: orthogonal visibility graph + A* minimising length + bend penalty (Wybrow, Marriott & Stuckey 2009), plus `route_all` with nudging/separation of shared segments. `Port { rect, side: Option<Side>, point }`.
- Hard limits: node/edge counts capped (return error, never hang); hostile-input sweep (NaN, zero sizes, self loops, duplicate edges, huge graphs) in tests.

### WP2 craft-libs `craft-mermaid` (crates/mermaid) — no dependencies
- `parse(&str) -> Result<Diagram, ParseError>`; `Diagram::Flowchart(Flowchart{ direction, nodes, edges, subgraphs, class_defs, styles, warnings })`; other diagram headers → `Diagram::Unsupported{ kind }`.
- Node shapes as an enum covering the spec's bracket forms and `@{ shape: }` names (aliases folded). Edge: kind (arrow/open/dotted/thick/invisible), heads both ends (arrow/circle/cross/none), length, label. Labels: text with `<br>` → newlines, entity codes decoded, markdown-string flag.
- Unknown statements become `warnings` (line, message) — never silently dropped; byte/line limits.
- Tests: the spec's corpus as relation tests + hostile-input sweep.

### WP3 FigCraft Boards integration (branch `boards-diagram`), after WP1+WP2
- Local-only path deps on the two new crates (the eventual PR flow: craft-libs PR → rev bump PR → FigCraft PR).
- New BoardShape variants needed for Mermaid shapes not present (map rest to nearest + warning).
- `board.importMermaid {text, at?}`: parse → layout → executes board.shape/board.connector (like templates) in one undo step; selects result; reports warnings/unsupported in the result and in the GUI.
- `board.tidy {ids?, direction}`: layered layout of selected shapes + their connectors.
- Obstacle-avoiding elbow routing in the normalize pass (all board objects as obstacles), falling back to today's route.
- Paste of Mermaid text onto a board → import (if a paste hook exists); menu + Actions palette entry; CLI/MCP via command ids.
- Tests (relations: no node overlap, edges ranked forward, routes don't cross obstacles), docs rows (boards-parity.md, gaps.md, ROADMAP.md).

### WP4 craft-libs `craft-vsdx` + FigCraft board export/import (after WP3)
- From `research/vsdx-spec.md`, built on craft-libs `opc`/`zip`/`xml`. Write first (shapes with geometry + text, glued connectors), then read (cached V values, master inheritance).

### WP5 Stencils
- `.vssx` masters → shape library entries (name, geometry, icon); board panel to browse/insert; our own built-in libraries (flowchart, network, UML basics) drawn in code.

### WP6 Try-it build
- Linux build / WASM build of the branch the user can run; short "how to try" note.

## Not doing without the user
PRs, issue claims, comments, pushes, RFCs; licence or dependency additions outside the allowlist.
