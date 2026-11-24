# FigCraft Boards: implementer's map (read-only research, 2026-11-21)

Repo: `/home/satal/code/craft-diagrams/figcraft` (paths below are relative to it). Nothing was built or run; all
facts are from reading source. Line numbers are as read and drift.

## Recommendation

1. Put the Mermaid flowchart parser in craft-libs (rule 9; a sibling clone `../craft-libs-mermaid` exists, no mermaid
   crate in it yet), as a pure `text -> Graph{nodes, edges, direction, warnings}` crate with no figcraft types.
   FigCraft's engine only maps the graph onto `board.shape` / `board.connector` calls. Needs a craft-libs rev bump,
   which is its own PR (rule 10).
2. Layered layout: pure `layered_layout` in `crates/doc` next to `board_mindmap.rs` (no deps), applied by an
   undoable engine command. Reuse for Mermaid import.
3. Obstacle-aware elbows: change `board_route` to take `obstacles: &[Rect]`, and widen `settle_routes` dirtiness
   (see section 2). This is the riskiest item; do it after 1, 2, 4.
4. New shapes: only stadium, circle, double circle are missing. Append at the END of `BoardShape` (binary encodings).
5. Order: shapes -> parser lib -> import command -> layout -> router -> paste hook/menu -> file format/stencil.

## 1. Board types, rendering, hit-testing

`crates/doc/src/board.rs` (648 lines). Board objects are ordinary nodes with `Node.board: Option<BoardItem>`;
there is NO shape drawing or hit-test code in render/ui. A shape = Frame (vertical auto-layout, padding from
`shape_padding`) + child vector "Outline" (path from `BoardShape::path`, Scale constraints) + centred text.
Selection of the whole shape uses `Node::is_atom` (`crates/doc/src/node.rs` ~784).

```rust
pub enum BoardItem { Sticky{author}, Shape{shape: BoardShape}, Connector(Connector), Label{connector},
    Stamp{stamp,author,on}, Ink, Table, MindNode, CodeBlock{language} }
pub enum BoardShape {            // board.rs:169, serde camelCase, 23 variants
  Rectangle, RoundedRectangle, Ellipse, Diamond, Triangle, SpeechBubble, Chevron,
  /* added later: comment says they keep their place in binary encodings */
  TriangleDown, ParallelogramRight, ParallelogramLeft, Trapezoid, Pentagon, Hexagon, Octagon, Star, Plus,
  ArrowLeft, ArrowRight, Database, Document, PredefinedProcess, ManualInput, InternalStorage }
pub const ALL: [BoardShape; 23]            // :198 length must change
fn names(self) -> (&'static str, &'static str)   // :225 (id, English label); id()/from_id() case-insens./label()
pub fn path(self, w: f64, h: f64) -> BezPath      // :266 `poly` closure over unit-box points; helpers database/document/with_lines/star_points
pub fn text_insets(self) -> [f64; 4]              // :337 top,right,bottom,left fractions
pub enum Routing { Straight(default), Elbow, Curved }
pub enum Side { Auto(default), Top, Right, Bottom, Left }   // point(Rect), horizontal, normal()
pub enum End { Node{node, side, at: Option<[f64;2]>}, Point{x,y} }            // serde untagged
pub enum ConnectorPath { Runs{across: bool, at: Vec<f64>}, Bow{x,y} }
pub struct Connector { from: End, to: End, routing: Routing, path: Option<ConnectorPath>, branch: bool }
```

Already present: hexagon, parallelograms (R/L), trapezoid, Database (= cylinder), PredefinedProcess (= subroutine),
Document, Diamond, Ellipse, RoundedRectangle. MISSING: stadium/pill, circle, double circle (an Ellipse with equal w/h
is a circle; a true "circle" variant would need a size-lock, so cheapest is a Stadium variant plus doing circle via
Ellipse with w==h, and DoubleCircle as a new path with two subpaths).

### What adding a BoardShape variant touches (grep of every match)
- `crates/doc/src/board.rs`: enum, `ALL` (+length), `names`, `path`, `text_insets` (all exhaustive).
- `crates/format/src/rest.rs:773` (`BoardShape::from_id`, REST/JSON import).
- `crates/format/src/fig/map.rs:7,121-157,227-251`: `.fig`/`.jam` kind -> shape (`board_shape(kind)`), and
  `apply_native_board_shape` writes `{"type":"shape","shape":id}`; only needed if FigJam has a native kind.
  Tests: `format/src/fig/board_import_tests.rs`.
- `crates/engine/src/boards/board.rs:309` (`shape_of` error lists `ALL` ids), the `board.shape` params-doc string
  (hard-coded shape list), `boards/board/items.rs` (`make_shape`, `shape_padding`, `shape_cmd`),
  `boards/board/edit.rs:112-165` (`reshape` regenerates outline + padding), `boards/board/derived.rs:21`
  (`settle_text` re-pads on resize), `engine/src/board_tests.rs:262`.
- UI: `ui-egui/src/canvas/board_bar.rs:21,40,73-77,305-335` (shape grid, `SHAPE_COLUMNS` 6, draws `s.path(14,14)`,
  runs `board.setStyle {shape: id}`), `toolbar.rs:102`, `menu_contrast_tests.rs:176`,
  `board_ui_tests.rs:76-81,139,235` (index-based `pick_option`: depends on `ALL` order, so append), `i18n/tests.rs:134`
  (`dynamic_labels()` uses `BoardShape::ALL` labels: add to `locales/en.toml`).
- Docs: `docs/mcp.md` ~585 (shape list), `docs/boards-parity.md` row "Shapes with text" (says "23 kinds").
- Collab uses generic serde, no per-variant code seen (not inspected in detail).

## 2. board_route.rs and obstacles

`crates/doc/src/board_route.rs` (266 lines), pure geometry. Tests `doc/src/board_route_tests.rs` (49 lines).
```rust
pub enum RouteEnd { Point(Point), Box(Rect, Side), On(Rect, Side, Point) }   // :12
const ROOM: f64 = 24.0;  pub const END_GAP: f64 = 4.0;  pub const MAX_RUNS: usize = 64;
pub fn route_points(from: RouteEnd, to: RouteEnd, routing: Routing) -> Option<Vec<Point>>            // :40
pub fn route_along(from, to, routing, path: Option<&ConnectorPath>) -> Option<Vec<Point>>             // :54 honours hand-shaped Runs/Bow
pub fn reshaped(from, to, routing, path, portion: usize, p: Point) -> Option<ConnectorPath>           // :94
fn elbow(p0: Point, s0: Option<Side>, p1: Point, s1: Option<Side>, boxes: &[Rect]) -> Vec<Point>     // :219
```
Also `attached_sides`, `attach_points`, `path_handles`; private `along_runs`, `curve`, `tidy`, `attach`, `named_or_facing`.
Exported at `doc/src/lib.rs:34`.

Today: `route_points` builds `boxes` from ONLY the two ends. `elbow()` tries the direct `short_elbow`; if
`crosses(&direct, boxes)` (EDGE tolerance 0.5) it builds candidates (two L/Z shapes, plus lines at the midpoint and at
`all.x0-ROOM`, `all.x1+ROOM`, `all.y0-ROOM`, `all.y1+ROOM`, `all` = union of the two end boxes), drops crossing
ones, takes min of `length + ROOM*points`, else falls back to `direct`.

Call sites: `engine/src/boards/board/derived.rs` (`route` :280 -> `route_ends` :286 -> `route_along`; `place_route` :255),
re-exported `boards/board.rs:37`; used by `connectors.rs` (`make`, `reattach`) and `reshape.rs`; and
`format/src/fig/map.rs:15-53` (`route_along` for `.fig` connectors saved without paths).

Storage: the route is written as geometry into the connector's own Vector node (`place_route`: transform, size,
`VectorNetwork::from_path` for curved else `from_points`; skips the write if unchanged = idempotent). Normalize pass:
`engine/src/derived.rs` ~78 `board::settle_text`, then `settle_mind_maps`, then `settle_routes` and `settle_stamps`
(~168-172).

What an all-object router needs:
- Signature change: add `obstacles: &[Rect]` (world rects) to `route_points`, `route_along`, `reshaped` and `elbow`;
  update the `fig/map.rs` callers (pass `&[]` there) and `board_route_tests.rs`. Keep `&[]` == today's behaviour
  (existing tests must still pass).
- `route()` in engine must gather obstacles: `doc.world_bounds(n)` of every non-connector, non-label, non-ink board
  node on the page (skip sections/frames that merely CONTAIN the ends, skip the two end nodes' own boxes as today).
- Dirtiness: `settle_routes(doc, before, touched)` (~lines 38-70) reroutes only connectors that are touched or whose end
  is touched/gone (`hit` closure). With obstacles, ANY moved/resized/added/removed obstacle can change any elbow.
  Safe rule: if any non-connector board node on the page is in `touched`, reroute every elbow connector on that page
  (perf bound: cap obstacle count, e.g. 500, and fall back to ends-only above it, like `MIND_MAX_NODES`).
- Search: the existing candidate set is too small for multi-obstacle cases. Use a small orthogonal visibility graph:
  grid lines at every obstacle edge +/- ROOM and at the ends, A* with a bend penalty; `MAX_RUNS` 64 bounds output.
  Must stay deterministic (all peers recompute the same geometry: local edit, undo, live session).
- Hand-shaped `ConnectorPath::Runs` stay authoritative (no obstacle avoidance on them).

## 3. Command system

- `crates/engine/src/cmd.rs`: `CommandSpec{id,label,params,history,run}`; `spec!(id, label, params_doc, history_bool, run_fn)`;
  registered by per-module `specs()` lists in `registry()` (board ones ~lines 122-126). `find_command`,
  `all_commands`, `param_names`, `needs_params`, `refuse_unknown_params` (unknown names refused; names parsed from the
  params doc, `{a, b?}`, `?` = optional), `doc_items`.
- `engine/src/boards/board.rs` `specs()` holds board.* commands + params docs; `require(s,cmd)` refuses on a non-board
  file (`NEEDS_BOARD`); `shape_of`, `routing_of`; consts `STICKY` 200, `SHAPE` 160.
- Undo: `engine/src/session/exec.rs` `execute` = one undo step per command, rollback on error, `guard::run` catches
  panics, then `normalize_edit`. `session/undo.rs`: `Session::grouped`, `batch`, `request_ui(kind,data)`. So a command
  that internally calls other commands' `run` fns is ONE undo step with no extra work.
- Precedent for building a diagram: `engine/src/boards/board_templates.rs`: `BoardTemplate{id,name,description,size,make: fn(&mut Builder)->Result<()>}`;
  `template_cmd` makes a `Builder{s,page,at,made}` whose `run(cmd,p)` does `find_command(cmd)` then `(spec.run)(self.s,&p)`;
  helpers `shape`, `connector`, `sticky`, `section`; result `json!({"name":..,"ids":made})`, selection set. A flow-chart
  template exists (`flow_chart`): copy its Builder usage for Mermaid import.
- Shape/connector primitives: `boards/board/items.rs` `make_shape(doc,parent,at,size,shape,text,color)`, `shape_cmd`
  (size 1..=100000); `connectors.rs` `connector_cmd`, `make(doc,page,Connector,caps)` (errors "the ends can't be placed",
  "the ends are in the same place", "connect two different layers"; takes `label?`).
- Layout precedent: `crates/doc/src/board_mindmap.rs` pure `mind_layout(&[MindBox{id,rect,parent,left}]) -> Vec<(NodeId, Point)>`
  (`MIND_GAP_X` 88, `MIND_GAP_Y` 48, `MIND_MAX_NODES` 2000, `MIND_MAX_DEPTH` 64; returns empty for non-finite/oversized
  input). Engine side `boards/board/mindmap.rs`: `settle_mind_maps` in normalize + `shift(doc,id,Point)` (world delta
  -> local transform). For tidy: pure `layered_layout(&[Box{id,size}], &[(from,to)], Direction) -> Vec<(NodeId,Point)>`
  in doc (longest-path layering, cycle break by DFS, barycentre ordering, `MAX_*` caps), engine command
  `board.tidy {direction?: "TB"|"LR"|"BT"|"RL", ids?}` gathering `world_bounds` and `End::Node` connector edges, then
  `shift`. NOT in normalize (user-triggered, undoable `spec!`); normalize then re-routes connectors.
  Note an existing "Tidy up" exists for auto-layout rows (ROADMAP M8): choose a different id (`board.tidyDiagram`) to avoid confusion.

## 4. Surfacing

- Menu catalog `crates/engine/src/catalog.rs`: `MENU` items `m!(menu,label,id)`, `OWN` list (~305; `file.newBoard`,
  `file.newFromTemplate` are there), `parity()`, `SEARCH_ALIASES`. No `board.*` command is in the menu catalog today.
- `cargo xtask parity` (`xtask/src/main.rs` ~221-266) measures `catalog::MENU` only and writes
  `docs/parity-checklist.md` (date = last commit date). A command outside `MENU` does not change parity numbers.
- Actions palette `ui-egui/src/menus/palette.rs:285`: lists every engine command except `needs_params` ones (a required,
  non-`?` param hides it). A required `text` param therefore hides `board.importMermaid` from the palette: either make
  `text?` optional (empty -> request_ui dialog) or add a menu item. A menu item is handled in `ui-egui/src/menus.rs`
  ~188 (`match` on id, e.g. `"collab.join" => ...`), dialogs live in `ui-egui/src/dialogs/`.
  Template picker precedent: `ui-egui/src/widgets/board_picker.rs:135` runs `board.template`: reuse for a stencil picker.
- CLI: `apps/figcraft-cli/src/files.rs:425 run_script` runs a script inside `s.grouped("run <script>", ..)` (one undo
  step, all-or-nothing; `s.ui_requests.clear()` after each step); `commands` listing `main.rs` ~106. The control channel
  drops `request_ui`. CLI tests: `apps/figcraft-cli/tests/it/` (cli.rs, generated_docs.rs ...). Check `generated_docs.rs`:
  it likely fails when a command is missing from docs/mcp.md.
- MCP: `docs/mcp.md` "### Boards" (~534-640) one bullet per board command; `batch`/`command.batch` with `$step`/`$ref` ~406.
  Add bullets for each new command; update the `board.shape` list.
- A string param: `{text, direction?, at?}` in the params doc; read with `params::` helpers; validate length (cap, e.g.
  64 KB) and node/edge counts (cap, e.g. 500 nodes) and refuse beyond.
- Rule 15 (never present partial/unsupported as success): result JSON carries `"warnings": [..]` (one string per
  skipped construct: subgraph, classDef, click, unknown shape syntax, line N) and `"skipped": n`; the GUI paste/menu path
  uses `app.run_user(...)` so the import report dialog shows them (`ui-egui/src/dialogs/import_report.rs`; the same shape as
  `edit.pasteDesignLayers`'s `warnings`, handled in `ui-egui/src/menus/clipboard.rs`: toast "left out" count). Refuse
  (Err) when NOTHING parses (not a flowchart: sequenceDiagram, gantt ...), naming the first line; partial -> Ok + warnings.

## 5. Paste hook

`ui-egui/src/menus/clipboard.rs`: `paste(app, event_text, at)` -> `paste_text(app, &text, (cx,cy), center)`:
```rust
fn paste_text(app: &mut FigApp, text: &str, (cx, cy): (f64, f64), center: bool)
   // comment link -> comment.open; dev-mode refuse; board && spreadsheet_cells(text) -> board.table;
   // looks_like_svg -> file.import; else text.create
```
Insert a branch after the `spreadsheet_cells` branch: `board && figcraft_engine::mermaid::looks_like_flowchart(text)`
(first non-blank, non-`%%` line starts `flowchart `/`graph ` + direction) -> `app.run_user("board.importMermaid", {text, x, y})`,
and toast the warnings like the table branch. Precedent for the engine-side detector: `engine/src/board_table.rs`
`spreadsheet_cells(text) -> Option<(Vec<Vec<String>>, bool)>` + tests `engine/src/board_table_paste_tests.rs`
(includes `junk_never_panics`, :67). Synthetic control-channel pastes never read the system clipboard
(`app.control_synthetic`), so tests drive `paste_text` or the command directly. Decide: auto-convert on paste vs ask
(a pasted code sample that happens to start `graph TD` would surprise people; consider toast with Undo).

## 6. File formats

- Native `.figcraft` = zip of `document.json` (+images, versions) in `crates/format/src/lib.rs`
  (types come from `craft-figcraft-format`: `EXTENSION`, `FORMAT`, `VERSION`). Importers: `format::{rest,fig,penpot,sketch}`.
- `.jam` precedent: there is NO `.jam` module; it is the `.fig` reader (header `fig-jam.` -> `FileKind::Board`), plus
  extension lists: `format/src/thumbnail.rs:89` (`"fig" | "jam" => FileKind::Fig`), `ui-egui/src/open.rs:8,15-20`
  (`&["figcraft","fig","jam","penpot","sketch","json","svg",...]`, `is_document`), `ui-egui/src/app/files.rs:87`
  (open filter), `ui-egui/src/home.rs:186` (toast text "Drop a .figcraft, .fig, .jam, ..." keyed in every
  `locales/*.toml`: changing the English string orphans all translations; add a NEW key instead), tests
  `ui-egui/src/home_tests.rs:901-915`, `engine/src/jam_open_tests.rs`, `format/src/fig/board_import_tests.rs:427`.
  Engine `file.import`/`file.open` routing by extension/sniff (not read in detail: grep `"jam"` found no engine hit,
  so sniffing is by content; check `engine/src/cmd/file*.rs`).
- A new board file format (feature 5): reuse the `.figcraft` zip container and a text/JSON member, or a craft-libs
  format crate (the suite pattern: `craft-*-format`, see `craft-libs/crates/*-format`, `stencilcraft-format`
  exists there already: look at it before inventing a stencil/shape-library format). Save-as dialog filter lists are in
  `ui-egui/src/app/files.rs` (rfd; UI layer only).

## 7. i18n

English text is the key: `tr("Fill")`, `trf("… {n} …", &[("n",..)])`, `trn`, `command(id, english)`. Catalogs
`crates/ui-egui/locales/*.toml`, baked by `build.rs`; command labels translate by id under `[commands]`.
Feature PRs add English ONLY (rule 16; translations come from storytold/craft-i18n). New shape labels: add to
`en.toml` (test `ui-egui/src/i18n/tests.rs` `dynamic_labels()`). Docs: `docs/translating.md`. Engine-side warnings
(English strings in JSON) are not translated.

## 8. Tests

Layout: flat `crates/engine/src/*_tests.rs` (mounted from lib.rs): `board_tests.rs`, `board_connector_tests.rs`,
`board_reshape_tests.rs`, `board_mindmap_tests.rs`, `board_connected_tests.rs`, `board_tools_tests.rs`,
`board_tables_tests.rs`, `board_table_paste_tests.rs`, `board_code_tests.rs`, `board_grid_tests.rs`, `jam_open_tests.rs`.
Doc: `doc/src/board_route_tests.rs`, `board_shape_tests.rs`, `board_mindmap_tests.rs`. UI: `ui-egui/src/board_ui_tests.rs`,
`board_connect_tests.rs`, `board_line_tests.rs`, `board_picker_tests.rs`. Integration binaries (rule 30, one per crate):
`crates/engine/tests/it/main.rs`, `crates/format/tests/it`, `crates/mcp/tests/it`, `apps/figcraft-cli/tests/it`.
Hostile input: no single global sweep file found; the convention is a per-feature junk test
(`board_table_paste_tests.rs:67 junk_never_panics`, `board_connector_tests.rs:164 reattach_survives_junk`); rule 29 says
crash fixes join the hostile-input sweep: a Mermaid parser needs its own proptest/junk corpus (NaN-free, deep nesting,
10^5 edges, unterminated quotes, NUL, huge labels).

Relation-asserting excerpts (style to copy; rule 27 asserts relations, not magic numbers):
```rust
// doc/src/board_route_tests.rs: attached end starts END_GAP outside the side
assert_eq!(pts.first().copied(), Some(Point::new(100.0 + END_GAP, 30.0)), "{routing:?}: leaves the right side");
// same file: elbows from one side share their first run along its normal
assert_eq!(p[0], q[0], "{routing:?}: the same start");   assert_eq!(p[1].y, p[0].y, "...out along the side's normal first");
// engine/src/board_connector_tests.rs:141 elbows_go_around_the_objects_they_join (helper `through(pts, rect)` at :133)
```
Tests to add: elbow avoids a third object (`!through(pts, third)`), obstacle moved -> connector re-routed (via
`settle_routes`), parse -> shape/connector counts and edge endpoints equal node ids, import is one undo step
(`undo` restores empty page), layering: every edge goes to a strictly later layer in TB, `board.tidy` idempotent.

## 9. Docs rows to update

- `docs/boards-parity.md`: features table rows: "Shapes with text" (shape count), "Connectors", "Curved connectors,
  elbows around objects ..." (:40, says elbows go around the objects they JOIN: update when all-object), "Templates"
  (:58), "Mind maps" (:60), "`.jam` import and export" (:66); new rows for Mermaid import, tidy diagram, stencil
  picker; changelog table at the bottom (newest first) + header `Last updated`/`Change`.
- `docs/gaps.md`: row 24 (:40) "Boards: the whiteboard tail" estimate 20-44 h (reduce with the change, like past
  entries "30-60 h -> 28-56 h"); changelog entries (:618+ style `Gap 24: ...`); header line 3.
- `ROADMAP.md`: header line 5 "Change:" string; M12 (boards) and M9 (`.jam`) entries; changelog table rows near :471.
- `docs/parity-checklist.md` is GENERATED by `cargo xtask parity`: do not hand-edit; only moves if `catalog::MENU` changes.
- `docs/mcp.md` Boards section. `xtask/src/docs_check.rs` (`check(root)`) enforces docs consistency; run `cargo xtask docs`.
- `docs/translating.md` untouched.

## 10. craft-libs dependency model and layers

- Root `Cargo.toml` `[workspace.dependencies]`: every craft-libs crate is a git dep
  `git = "https://github.com/storytold/craft-libs", rev = "a49329e7..."` (comment "Keep every craft-libs crate on one
  rev"). Exceptions today: `craft-image-codecs` rev `ba25ccd2...` (different!) and `craft-gif`. Rule 10: single rev,
  a rev move is its own PR: adding a mermaid crate means bumping ALL entries (and fixing the image-codecs drift) in a
  dedicated PR first.
- Internal crates are `path` deps. `xtask layers` (`xtask/src/layers.rs`) reads `cargo metadata --no-deps`, looks only at
  crates named `figcraft-*` in TABLE (geom/color 0; doc 1; layout/text/pathops/collab/plugin 2; render/format 3;
  svg/pdf/vectordrawable 4; engine 5; ui-egui/mcp/collab-net 6). Rules: no `egui|eframe|rfd|winit` below L6; no
  `tungstenite|tokio|tokio-tungstenite|mio|socket2` below L6; no dep on same-or-higher-layer `figcraft-*` crate.
  EXTERNAL deps (including craft-libs crates) are NOT restricted by the layer check, and dev-deps are skipped. So
  `crates/doc` (L1, today depends only on geom + color) and `crates/engine` (already depends on `craft-docpath`,
  `craft-font-store`, `craft-font-catalog`) MAY take a craft-libs mermaid crate. Prefer engine (L5) or format (L3, which
  already takes craft-penpot/sketch/fig) over doc, to keep doc dependency-free. Licence must be on the standards list (rule 26).

## 11. Build footprint

- egui/eframe/winit/rfd: only `crates/ui-egui` (egui, egui_extras, craft-ui, craft-update with egui feature) and the
  desktop `apps/figcraft` (eframe/rfd). `crates/{doc,engine,format,render,mcp}` and `apps/figcraft-cli` list none (grep of
  their Cargo.toml), and `xtask layers` forbids them below L6.
- `cargo test -p figcraft-engine` therefore compiles no GUI crates. Engine pulls render, format, svg, pdf, text
  (harfrust/skrifa/icu_segmenter, pure Rust), craft-font-store with `bundled-index`. I did not run it; no system-library
  build.rs was checked beyond noting `.cargo/config.toml` sets `CRAFT_FONTS_DIR = ../craft-fonts` (optional: missing
  directory only warns, bundled fonts used; text tests may rely on fonts: check `crates/text/build.rs` before asserting
  headless CI is fine). Git deps need network on first build (craft-libs).
- Test the pure pieces in `figcraft-doc` (deps: geom, color only) for the fastest loop.

## Decisions needed from you

- d1: Mermaid parser location: new craft-libs crate (rule 9, needs a rev-bump PR first) vs a module in
  figcraft-engine now and promote later. Recommendation: craft-libs.
- d2: Obstacle routing scope: reroute all elbows on the page when any board object changes (simple, deterministic) vs
  spatial index with incremental dirtiness. Recommendation: simple first, with a 500-object cap.
- d3: Shapes: add only `Stadium` and `DoubleCircle` (+ `Circle` as size-locked Ellipse via import), appended after
  `InternalStorage`. Agree?
- d4: Paste auto-convert of Mermaid text on a board vs menu item only (plus palette). Recommendation: both, with a toast
  carrying warnings; paste detection limited to `flowchart`/`graph` headers.
- d5: New board export/import format: own crate in craft-libs (check `stencilcraft-format` first) vs a member inside
  `.figcraft`. Not needed until feature 5.

## Actions for you

- a1: Approve d1 (blocks the parser PR and the craft-libs rev bump).
- a2: Confirm the craft-libs rev to bump to and whether `craft-image-codecs`'s differing rev is intentional.
- a3: Confirm the layout command's id (`board.tidyDiagram`) and default direction (TB).
