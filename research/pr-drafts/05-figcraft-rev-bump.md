<!-- LOCAL DRAFT. Not posted. Cannot be cut yet: no commit exists. -->

## Title

Dependencies: align craft-libs at merged <short rev>

## Description

No user-visible change: every craft-libs crate moves to one merged commit, which brings in `craft-graph-layout`, `craft-mermaid` and `craft-vsdx` for the diagram pull requests.

Part of #<umbrella>. Waits for the three craft-libs pull requests: <!-- fill: craft-libs PR links -->. The feature pull requests (routing, Mermaid import, UI, VSDX) wait for this one.

**What the user saw:** n/a.

**Cause:** pull-requests.md "Shape" 5: a feature pull request does not move the craft-libs `rev`.

**What changed**

- `Cargo.toml`: every craft-libs git `rev` (currently `a49329e71e32ad93794ab595c41f33209a7d2220`) becomes `<full rev>`; `Cargo.lock` regenerated. Nothing else.
- craft-libs range pulled in: <!-- fill: git log a49329e71..<rev> summary, or link -->
- Compatibility fixes only if needed to build, listed here: <!-- fill -->
- The new crates are not added as dependencies here; the feature pull requests do that with `git` and the same `rev` (not the local path lines on `boards-diagram`).

Precedent: figcraft#2514.

## Tests and checks

Ran: <!-- fill: cargo build, cargo test, cargo clippy -D warnings, cargo xtask ci including wasm; counts -->
Not run: <!-- fill -->

## Departures from SHOULD rules

None known.

## Cut from

Nothing yet. Create from current `main` after the craft-libs pull requests merge. Do not use any commit on `boards-diagram`; those carry local path dependencies.
