<!-- LOCAL DRAFT. Not posted. -->

## Title

craft-vsdx: add VSDX drawing reader and writer

## Description

Apps can read VSDX drawings and VSSX stencils into a plain model and write a minimal VSDX drawing.

Part of storytold/figcraft#<umbrella>. Refs storytold/artcraft#2121 and storytold/craft-launcher#801 (the file format they ask for); neither is fixed by this crate.

**What the user saw:** no craft-libs crate reads or writes VSDX.

**Cause:** new capability.

**What changed** (new crate `craft-vsdx` on `craft-opc`, `craft-xml`, `thiserror`; dev `craft-testkit`; all already in the workspace, no new external dependency)

- `read(bytes)`: pages, flattened shapes and 1-D connectors, master inheritance (section 2.2.5.4), style sheets, cached values only, local-to-parent transforms through groups (section 2.2.3.2.1), geometry (move, line, cubic, elliptical arc, ellipse), text, fill, line, connection points and glue. Reports what it cannot draw as a `Warning` (NURBS, polyline and spline rows, pattern fills, themes, guides, missing masters, cut-off groups, foreign shapes).
- `read_stencil(bytes)`: stencil masters (name, shapes, PNG icon). Read-only.
- `write(&Drawing)`: the minimal package of section 3.1. Writing is minimal; no formula, theme or layer output.
- Limits (`Limits`) on part sizes, counts and group depth; a zip bomb is refused; master self-references are cut.
- Root `README.md` entry, `crates/vsdx/README.md`, `PROVENANCE.md`; `examples/roundtrip.rs` writes a sample flowchart.

**Provenance (clean room):** MS-VSDX version 20250819 (https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/), sections cited in the code (for example 2.2.3.2.1, 2.2.3.2.2, 2.2.5.4, 3.1); the package structure follows the Open Packaging Conventions via `craft-opc`. Choices where the specification is silent (O1 to O8, conflicts c1 to c4) are in the crate docs. No other implementation was read and no other description of the format used. Disclosure, as in `PROVENANCE.md`: the writer's output was once opened in an office suite as a black box to see that an arrowhead draws; no source or file internals were read. Fixtures are generated in test code; no file from a proprietary app was used.

**Names:** user-visible text says "VSDX drawing" and "VSSX stencil", format names under naming.md rule 3. The specification's formal title and interchange content types appear only where the format requires them.

## Tests and checks

`tests/it/` (`roundtrip.rs`, `inherit.rs`, `hostile.rs`, `main.rs`):

- round trip: shapes keep box, text and colours; connectors keep glue, ends and arrows; geometry stays inside the shape box; a second trip changes nothing; clashing ids are renumbered and glue follows; bad models are refused, not written;
- reading: an instance inherits what it does not set; a stencil lists its masters; style sheets supply unset cells; flips and rotation keep geometry in the box; glue comes from connects or endpoint formulas; what is not drawn is warned about;
- hostile: mutated packages and page XML never panic; truncated, missing-part, over-limit, huge-index, deep-group, zip-bomb and self-referencing master inputs.

Ran: <!-- fill: cargo fmt --check; cargo clippy -p craft-vsdx --all-targets -- -D warnings; cargo test -p craft-vsdx (counts) -->
Not run: <!-- fill: workspace-wide checks, other operating systems, wasm32 build; no check against files from other applications -->

## Departures from SHOULD rules

- `#![forbid(unsafe_code)]` is not set in the crate root (the workspace forbids unsafe). <!-- fill: add the attribute or drop this line -->
- No `formats.yml` entry; feature pull requests should not change workflows. Question for maintainers: is the `workspace.yml` gate enough, or is `formats.yml` coverage wanted as its own CI pull request?
- Two consumers (RFC-0009, open draft): FigCraft now; the "standard file format parser" exception applies; named responsible maintainer: <!-- fill -->. Overlap with a separately developed `.vsdx` engine is raised on the umbrella issue.

## Cut from

Repo `craft-libs`, worktree `craft-libs-vsdx`, branch `vsdx`: `c1741ada0`, `db7a7d6ed`, `d4f0cb1ba` on `main` `6a061a30b` (18 files, +2,987). The first subject reads `Add craft-vsdx VSDX drawing reader and writer`; `craft-vsdx: add VSDX drawing reader and writer` fits the repo's majority style better. <!-- fill: reword before posting -->
