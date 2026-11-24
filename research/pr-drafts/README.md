# PR drafts: diagramming in FigCraft Boards

Local drafts. Nothing here has been posted, and nothing may be until the umbrella issue is approved. Drafted 2026-11-22 from `research/standards-audit.md` and `research/linked-requests.md`. Results are placeholders (`<!-- fill -->`); fill them from runs at posting time.

## Order

| File | Repo | Pull request | Cut from | Waits for |
|---|---|---|---|---|
| 00 | figcraft | Umbrella issue | n/a | go-ahead |
| 01 | figcraft | Six shapes | `boards-diagram` `9572e878` | none |
| 02 | craft-libs | `craft-graph-layout` | `graph-layout-polish` (6 commits) | none |
| 03 | craft-libs | `craft-mermaid` | `mermaid` (3 commits) | none |
| 04 | craft-libs | `craft-vsdx` | `vsdx` (3 commits) | none |
| 05 | figcraft | One rev bump | none yet | 02, 03, 04 merged |
| 06 | figcraft | Routing (`Fixes #1707`) | `f923272f` + docs rows | 05 |
| 07 | figcraft | Mermaid import and tidy | `8a767e45` + docs rows | 01, 05, 06 |
| 08 | figcraft | Dialog, paste, menu | `137b7f97` | 07 |
| 09 | figcraft | VSDX export and import | `boards-vsdx` (uncommitted) | 04, 05, overlap answer |
| 10 | figcraft | Shape libraries and VSSX | `boards-vsdx` (uncommitted) | 04, 05 |

01 to 04 are independent and can be open together. Open 01 first (no craft-libs dependency), then 02 to 04.

## Pre-posting checklist

- [ ] Umbrella issue approved and posted; replace `<umbrella>` in every draft. Decide whether craft-libs also gets a short issue; the audit recommends one per repo, these drafts use the figcraft one with a cross-repo `Part of`.
- [ ] Re-run the claims.md "before you start" duplicate and claim searches the day before posting (mermaid, vsdx, graph layout, shapes, connectors, routing) and compare with open PRs. At drafting there were none. Check figcraft#2231 (mind maps) for overlap in `board.rs` before 01 and 06, and #1705 for shape duplicates.
- [ ] Do not claim anything before the umbrella or the first PR's `Part of` is posted.
- [ ] Keep `1b26b33b` (engine clippy lints) out of every feature PR. Include it only as its own PR linking a failing `main` CI run.
- [ ] One rev bump PR (05) only; no feature PR moves the craft-libs `rev`. Replace the two local path lines in `Cargo.toml` and the "Path deps while in review" comment with git entries at the merged rev.
- [ ] RFC-0009 two-consumer argument for graph-layout (open draft, craft-contrib PR #10, not binding): FigCraft now; WordCraft (wordcraft#4131, #3800) and DeckCraft/GridCraft "Diagram" candidate; ask on the umbrella which maintainer is responsible. Fallback: keep the crate in FigCraft until a second app needs it (decision d1 in the audit).
- [ ] Commit all uncommitted work first. At drafting: `boards-diagram` has 8 modified files (`diagram.rs`, `connectors.rs`, `board_diagram.rs`, `clipboard.rs`, `mermaid_ui_tests.rs`, `items.rs`, `mindmap.rs`, `tools.rs`); `graph-layout-polish` has changes in `layered.rs` and `tests/it/layered.rs`; `boards-vsdx` is wholly uncommitted. File and line counts in the drafts come from commits only; recheck.
- [ ] Rebuild figcraft against the polished `graph-layout`: its path dependency pointed at the `craft-libs` checkout, which is on `graph-layout` (`eba44c9f2`, first commit only).
- [ ] Split the docs commit `57a1136d` by capability (routing, import and tidy, UI); one `ROADMAP.md` line per PR; regenerate `docs/parity-checklist.md` with `cargo xtask parity` after each rebase; do not hand-merge.
- [ ] Fill every placeholder from runs: fmt, clippy `-D warnings`, tests with counts, `cargo xtask ci` including wasm, the fails-without-the-change run. Do not write "CI passed" before it has.
- [ ] craft-libs PRs: add or confirm crate README for mermaid (audit gap 4), rename or explain `t01` to `t40` tests, decide on `examples/svg.rs`, optional `forbid(unsafe_code)` in vsdx, do not touch workflows.
- [ ] UI PR (08, 09, 10): Accessibility section, kittest tree check and announcement, screenshot in the description only, paste behaviour called out; decide whether paste-to-import ships in 08 (audit a3).
- [ ] Names: no other company's product names in titles, labels or READMEs; format names ("VSDX drawing", "Mermaid") only as in naming.md rule 3. Record the naming check for new shape and library names in `docs/feature-names.md`.
- [ ] Clean-room statements: PROVENANCE files present in each crate; no file from a proprietary app in any repo; research notes and fetched pages not committed.
- [ ] Do not mention how the code was written anywhere in titles, bodies or commits; authorship is the committer's.
- [ ] Draft PRs only while unfinished; link the PR each one waits for.

## Gaps in these drafts

- No GitHub access was used, so issue and PR numbers, maintainer names and current state are as of the two research files (2026-11-22).
- 09 and 10 are written from uncommitted work and are skeletal.
- The mind-map owner's handle and the `.vsdx` project's name are left out of the issue text; add them at posting if wanted (handles are in `linked-requests.md`).
- No issue template exists in craft-libs or figcraft `.github`; the umbrella uses the Problem / Proposal / Affects / Open questions shape of the craft-contrib template, adapted.
