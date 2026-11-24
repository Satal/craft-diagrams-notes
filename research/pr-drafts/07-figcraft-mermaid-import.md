<!-- LOCAL DRAFT. Not posted. Waits for 01, 05, 06. -->

## Title

boards: import Mermaid flowcharts and tidy diagrams

## Description

Boards can build a flowchart from Mermaid text and tidy a diagram into ranks, from the command line and MCP.

Part of #<umbrella>. Refs wordcraft#4131 and wordcraft#3800 as the other request for the parser (they ask for Markdown, not Boards; not fixed here). Left for the series: dialog, paste and menu (#<08>), VSDX.

**What the user saw:** diagrams had to be built shape by shape; there was no way to lay out or tidy them.

**Cause:** new capability.

**What changed**

- `board.importMermaid {text, x?, y?}` returns `{ids, shapes, connectors, sections, warnings}`: shapes sized to their labels, a section per subgraph, elbow connectors, laid out in ranks in the chart's direction without overlaps, selected, in one undo step. Another kind of diagram or text that does not parse is refused and adds nothing.
- `board.tidyDiagram {ids?, direction?}` returns `{ids, connectors}`: lays selected shapes and stickies out along their connectors, keeping the top-left. Refused with fewer than two. Not the design-file "Tidy up" of #2319.
- Mermaid shapes with no board outline map to the nearest shape with a warning. Warnings list what is drawn differently or left out (Markdown labels, cross arrowheads).
- Adds `craft-mermaid` as a dependency (git, merged `rev`). Uses the six shapes from #<01> and the router from #<06>.
- Docs: `docs/mcp.md`, `docs/boards-parity.md`, `docs/gaps.md` rows and one `ROADMAP.md` line for this capability. <!-- fill: split from commit 57a1136d; regenerate parity-checklist.md with cargo xtask parity -->
- Unchanged: existing commands, mind-map normalise pass (tidy leaves mind-map branches alone <!-- fill: verify, no test yet -->), undo model.

**Names:** "Mermaid" is a language name under naming.md rule 3 and appears only as the import language, never as our feature's name.

**Provenance:** layout and routing from `craft-graph-layout` (papers), parsing from `craft-mermaid` (public documentation only); see their pull requests. No other tool's output was observed.

## Tests and checks

`crates/engine/tests/it/board_diagram.rs`: a flowchart becomes shapes and connectors that do not overlap; left-to-right runs across and top-down runs down; subgraph members sit inside their section; a longer label makes a wider shape; one undo removes the whole import; another diagram kind or unreadable text adds nothing; a shape without its own outline comes with a warning; hostile flowcharts are refused or imported without a panic; tidy lays overlapping shapes out along their connectors; tidy needs two shapes.

- Fails with the change reverted: <!-- fill -->
- Ran: <!-- fill: fmt, clippy -D warnings, cargo test, cargo xtask ci incl. wasm; counts -->
- Checked that the CLI and MCP result carries `warnings`: <!-- fill -->
- Not run: <!-- fill -->

## Departures from SHOULD rules

- Size: about 850 lines, mostly `diagram.rs` and its tests. <!-- fill: justify or split import from tidy -->
- No `#[allow]` added unless a neighbouring test module requires the local convention: <!-- fill: try removing the two allow lines on test modules in tests/it/main.rs -->.

## Cut from

Repo `figcraft`, branch `boards-diagram`, commit `8a767e45` (4 files, +846) plus the matching docs rows of `57a1136d`. The working tree has uncommitted edits to `diagram.rs`, `board_diagram.rs`, `items.rs`, `mindmap.rs`, `tools.rs` and `connectors.rs` (225 insertions, 33 deletions at drafting time); commit and review them before cutting, and re-check the file counts above.
