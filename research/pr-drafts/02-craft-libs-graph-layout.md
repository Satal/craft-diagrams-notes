<!-- LOCAL DRAFT. Not posted. -->

## Title

graph-layout: add layered layout and orthogonal connector routing

## Description

Apps can lay out a flowchart in ranks, with nested groups and edge labels, and route connectors around obstacles, from one small crate.

Part of storytold/figcraft#<umbrella>. Consumed by FigCraft's connector routing (fixes figcraft#1707) and Mermaid import in later pull requests.

**What the user saw:** no craft-libs crate lays out graphs or routes connectors; FigCraft's elbows cross shapes (figcraft#1707).

**Cause:** new capability.

**What changed** (new crate `craft-graph-layout`, no dependencies, plain `f64` geometry, y down)

- `layered(&Graph, &LayeredOptions) -> Result<Layout, LayoutError>`: cycle removal, ranking honouring minimum lengths, long-edge splitting, crossing reduction, coordinates, nested groups kept contiguous with title bands, edge labels placed on their edge, orthogonal edges in four directions, back-edge loops, self loops and multi-edges.
- `route_orthogonal` and `route_all`: orthogonal visibility graph, A* with a bend penalty, ports on facing sides, spreading of connectors that share a side or channel.
- Limits: `MAX_*` caps on nodes and edges return an error. Output is deterministic.
- Root `README.md` entry; crate `PROVENANCE.md`.
- `examples/svg.rs` (219 lines) writes a layout as SVG for visual checks. <!-- fill: keep or drop; see departures -->

**Consumers (RFC-0009, open draft):** FigCraft today (routing and import, pull requests <!-- fill: numbers --> in figcraft). Candidates: WordCraft Mermaid and diagram import (wordcraft#4131, wordcraft#3800), DeckCraft and GridCraft "Diagram", any app taking on the diagramming requests (artcraft#2121, craft-launcher#801). It is not a file-format parser, so the "standard file format parser" exception does not apply. Responsible maintainer: <!-- fill: name agreed on the umbrella issue -->. If a second consumer is required first, the crate can stay in FigCraft and move later.

**Provenance (clean room):** implemented from papers only; sections are cited in the module docs and listed in `crates/graph-layout/PROVENANCE.md`: Sugiyama, Tagawa and Toda 1981; Eades, Lin and Smyth 1993 (cycle removal); Gansner, Koutsofios, North and Vo 1993, sections 2 and 3 (ranking, ordering); Brandes and Köpf 2001 (coordinates); Sander 1996 (groups); Wybrow, Marriott and Stuckey 2009 (routing). No graph-layout library's source was read. Tests use graphs written for this crate; no fixture from another tool.

## Tests and checks

Tests assert relations, in `tests/it/` (`layered.rs`, `route.rs`, `hostile.rs`):

- layout: edges point forward on a DAG, each direction puts targets after sources, no node overlap with variable sizes, groups contain members and exclude others, labels clear of nodes and edges, crossing reduction beats input order, deterministic, 500 nodes lay out, inputs over the caps are refused;
- routing: facing boxes get a straight line, a wall is walked round, random obstacle fields route cleanly, an enclosed target is unreachable, `route_all` separates parallel connectors;
- hostile: NaN and infinite numbers, bad indices, degenerate graphs, random junk never panic.

Ran: <!-- fill: cargo fmt --check; cargo clippy -p craft-graph-layout --all-targets -- -D warnings; cargo test -p craft-graph-layout (counts) -->
Not run: <!-- fill: workspace-wide checks, other operating systems, wasm32 build -->
Reviewed with the layout tests reverted to confirm they fail: <!-- fill -->

## Departures from SHOULD rules

- Size: about 4.9k lines in one pull request. If asked to split, cut routing (`route.rs`, `channel.rs`) from layered layout; `route_all` has its own API.
- Two consumers (RFC-0009, not yet binding): see above.
- `examples/svg.rs` is a dev tool, not scratch; drop it if reviewers prefer.

## Cut from

Repo `craft-libs`, worktree `craft-libs-layout`, branch `graph-layout-polish`: `eba44c9f2`, `b8032897f`, `3e11fca02`, `27449f5b1`, `e9bafab27`, `8323fe981` on `main` `6a061a30b` (20 files, +4,894). Open as one pull request with the final state; optionally squash to two or three commits with `graph-layout:` subjects. Branch `graph-layout` is the first commit only; do not open it separately.
