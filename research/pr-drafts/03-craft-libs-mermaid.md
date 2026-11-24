<!-- LOCAL DRAFT. Not posted. -->

## Title

craft-mermaid: add a bounded Mermaid flowchart parser

## Description

Apps can read Mermaid flowchart text into plain data, with a warning for every statement they cannot read.

Part of storytold/figcraft#<umbrella>. Consumers: FigCraft board import (later pull request); wordcraft#4131 and wordcraft#3800 ask for Mermaid in Markdown import and rendering and are the candidate second consumer. Not a fix for either.

**What the user saw:** no craft-libs crate reads Mermaid.

**Cause:** new capability.

**What changed** (new crate `craft-mermaid`, no dependencies)

- `parse` and `parse_with_limits` return `Diagram::Flowchart` (direction, nodes with shapes, edges with stroke, heads, length and label, nested subgraphs, styles, classes, notes, warnings) or `Diagram::Unsupported { kind }` for any other diagram header.
- Unknown statements become warnings with line and message; nothing is dropped silently. Byte, line and amplification limits turn oversized input into a `ParseError`.
- Choices where the documentation is silent are all lenient and listed in the crate docs, not claimed as Mermaid's behaviour.
- Root `README.md` entry and `crates/mermaid/PROVENANCE.md`.

**Provenance (clean room):** public Mermaid documentation only (flowchart, syntax reference, directives and per-diagram pages for header keywords), read 2026-11-21 at the version the site showed (12.3.0), URLs listed in the crate docs and `PROVENANCE.md`. No source, grammar, test or lexer of mermaid-js or any other parser was read; no output of Mermaid was observed; no documentation text or example is copied. Pages were read through a summarising fetch tool, so exact wording was not always seen; `PROVENANCE.md` says so. Every test input is original, written from the documented syntax; no fixture files.

## Tests and checks

`tests/it/main.rs`: tests `t01` to `t40` follow the sections of the flowchart documentation page (orientation, shapes, links, subgraphs, styles, comments); later tests cover choices the documentation leaves open, limits and a hostile-input sweep. <!-- fill: consider renaming t01..t40 to descriptive names before posting; gap 1 in the audit asked for self-explanatory names -->

Ran: <!-- fill: cargo fmt --check; cargo clippy -p craft-mermaid --all-targets -- -D warnings; cargo test -p craft-mermaid (counts) -->
Not run: <!-- fill: workspace-wide checks, other operating systems, wasm32 build -->

## Departures from SHOULD rules

- No crate README (the root README entry and `PROVENANCE.md` carry it). <!-- fill: add crates/mermaid/README.md if wanted; audit gap 4 recommended one -->
- Tests are named `tNN` in documentation order; see the fill marker above.
- Two consumers (RFC-0009, open draft): FigCraft now, WordCraft candidate. A text-format parser, so the "standard file format parser" exception may apply; maintainer approval and a named responsible maintainer: <!-- fill -->.

## Cut from

Repo `craft-libs`, worktree `craft-libs-mermaid`, branch `mermaid`: `328a1eb39`, `3803e2547`, `c1e2d736f` on `main` `6a061a30b` (13 files, +3,006).
