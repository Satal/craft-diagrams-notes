<!-- LOCAL DRAFT. Not posted. -->

## Title

boards: add stadium, double circle, trapezoid down, delay, card and flag shapes

## Description

Boards offer six more flowchart shapes: stadium, double circle, trapezoid down, delay, card and flag.

Part of #<umbrella>. Left for the series: routing, Mermaid import, VSDX.

**What the user saw:** the shape picker stopped at the 23 shapes from #1705, so common flowchart terminals and delays had to be approximated with other shapes.

**Cause:** new capability.

**What changed**

- `BoardShape` gains six variants with their outline path, id, label and text insets (`crates/doc/src/board.rs`).
- `en.toml`: six shape names. No other locale touched.
- `docs/mcp.md`: shape list. <!-- fill: parity row / shape count in docs/boards-parity.md, split out of the docs commit -->
- No dependency, no craft-libs change, so this pull request does not wait for anything.

**Names:** descriptive geometry names, not product names. Checked against `BoardShape::ALL` on <!-- fill: date --> so none repeats an existing shape or label.

**Linked requests:** none directly. Overlap check against #1705 (merged, 16 shapes): <!-- fill: result at posting time -->.

**Provenance:** outlines drawn in code from basic geometry (arcs, lines); no asset, icon or file from another product.

## Tests and checks

- Existing `crates/doc/src/board_shape_tests.rs` loops over `BoardShape::ALL`: unique id and label, outline inside its box and filling it, text insets leave room. The six new shapes are covered by those loops. <!-- fill: confirm the loops fail if a new shape's outline leaves its box; no test added by the commit -->
- Ran: <!-- fill: cargo fmt --check, cargo clippy -D warnings, cargo test (exact commands and counts) -->
- Ran: `cargo xtask ci` steps: <!-- fill: layers, assets, wasm, parity, docs; list any not run -->
- Not run: <!-- fill -->

## Accessibility

Shapes appear in the existing shape picker as buttons with translated names; no new control, keyboard path or announcement. <!-- fill: confirm the picker exposes each shape by name (board_picker_tests) -->. No screen reader was run.

## Departures from SHOULD rules

None known.

## Cut from

Repo `figcraft`, branch `boards-diagram`, commit `9572e878` only (4 files, +69 -3), on `main` `5c2a3784`. Cherry-pick onto a fresh branch from current `main`. Do not include `1b26b33b` (clippy lints).
