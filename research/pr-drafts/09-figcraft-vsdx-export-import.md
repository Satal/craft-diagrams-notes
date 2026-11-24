<!-- LOCAL DRAFT. Not posted. Work in progress: the source is uncommitted. Waits for 04 and 05; do not post before the .vsdx overlap question on the umbrella issue is answered. -->

## Title

boards: export and import VSDX drawings

## Description

A board page can be saved as a VSDX drawing, and a VSDX drawing can be opened as a new board or imported into the current one.

Refs storytold/artcraft#2121 and storytold/craft-launcher#801 (not fixed: they ask for a whole app). Part of #<umbrella>.

**What the user saw:** no way to exchange a board with other diagram tools.

**Cause:** new capability.

**What changed** (from the working tree; recheck against the final commit)

- `board.exportVsdx {path?}`, `file.export {format: "vsdx"}`, `figcraft-cli export --format vsdx`: shapes with outlines, text, fill, line, weight and dashes, sections as titled rectangles, stickies, glued connectors; ink, stamps, tables and images are left out and listed in `warnings`.
- `board.importVsdx {path? | data, name?, page?, x?, y?}`, `.vsdx` in File Open and Import: one page, one undo step; outlines that match a board shape become it, others become `BoardShape::Custom`; glued connectors attach; unreadable files add nothing.
- Uses `craft-vsdx` at the merged `rev`.
- Dialog and picker strings in `en.toml` use "VSDX drawing" (a format name, naming.md rule 3).
- Overlap to settle first: the `.vsdx` crate question and `.jam` import mapping (#1808), both raised on the umbrella issue.

**Provenance:** MS-VSDX only, through `craft-vsdx` (see storytold/craft-libs#<L3>). No file from a proprietary app was used. Real-world files, if used for manual testing, stay out of the repo (clean-room.md "Fixtures").

## Accessibility

Export adds a format option to an existing dialog (standard controls). <!-- fill: kittest check of the new format option, focus and announcement of the export result and its warnings count; no screen reader was run -->

## Tests and checks

`crates/engine/tests/it/board_vsdx.rs` (untracked at drafting time). <!-- fill: test names, what each checks, fails-without-change run, fmt, clippy, cargo test, cargo xtask ci incl. wasm, counts -->
Not run: <!-- fill: round trip with files from other applications; none exist in the repo -->

## Departures from SHOULD rules

<!-- fill -->

## Cut from

Repo `figcraft`, worktree `figcraft-vsdx`, branch `boards-vsdx`. At drafting the branch had the same six commits as `boards-diagram` (`57a1136d`) and about 26 modified files and four untracked (`vsdx.rs`, `library.rs`, `board_vsdx.rs`, `shape_libraries.rs`), +323 -36 on tracked files. Nothing is committed. Rebase onto `main` after pull requests 1 to 8 merge; the VSDX commits must not carry any of the diagram commits.
