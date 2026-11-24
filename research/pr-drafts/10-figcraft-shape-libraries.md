<!-- LOCAL DRAFT. Not posted. Work in progress: the source is uncommitted. Waits for 04 and 05, and for the maintainer answer on whether libraries stay FigCraft-only. -->

## Title

boards: add a shape libraries panel and VSSX stencil loading

## Description

A toolbar popover lists built-in shape libraries and VSSX stencils loaded from files; a click adds the shape at the centre of the view.

Refs storytold/artcraft#2121 and storytold/craft-launcher#801 (the shape-library part only). Part of #<umbrella>. `docs/boards-parity.md` lists "shape libraries" as `missing`; no issue tracks them.

**What the user saw:** only the built-in board shapes were available; no network, UML or arrow sets, no stencils.

**Cause:** new capability.

**What changed** (from the working tree; recheck against the final commit)

- Built-in libraries drawn in code: flowchart (the board shapes), network, UML and arrows.
- `board.shapeLibraries`, `board.loadShapeLibrary {path | data, name?}`, `board.removeShapeLibrary {library}`, `board.insertLibraryShape {library, name, x?, y?}`; loaded stencil paths are remembered in preferences.
- Uses `craft-vsdx` stencil reading at the merged `rev`.
- Naming: "Shape libraries" is a descriptive name. It is distinct from the shared Library side pane in craft-libs#1584 (psperera); <!-- fill: naming.md rule 4 check, date and sources, recorded in docs/feature-names.md; and say whether "Library" clashes -->.

**Provenance:** built-in libraries are original work drawn in code with generic device and UML names; record them in `ATTRIBUTION.md` (clean-room.md "Assets" 6). No stencil file from a proprietary product is committed; stencils used in tests are generated.

## Accessibility

- Pattern: toolbar popover with a list of buttons. Each shape button is named "{shape}, {library}" and is reachable by keyboard.
- Tests: <!-- fill: `board_picker_tests.rs` changes; kittest check of focus into and out of the popover, Esc, tree roles and names; announcement of "Loaded shape library ..." and "Shape library removed" -->
- Alternatives to drag: inserting is by click or keyboard, not drag.
- No screen reader was run.

## Tests and checks

<!-- fill: names, counts, fmt, clippy, cargo test, cargo xtask ci incl. wasm, i18n key test (about 40 new keys) -->

## Departures from SHOULD rules

- Size: about 40 new strings and a new widget; consider splitting built-in libraries from VSSX loading. <!-- fill -->

## Cut from

Repo `figcraft`, worktree `figcraft-vsdx`, branch `boards-vsdx`: `library.rs`, `shape_libraries.rs`, the `en.toml` and `board_picker_tests.rs` changes. Uncommitted at drafting.
