<!-- LOCAL DRAFT. Not posted. Post in storytold/figcraft. Replace the fill markers at posting time. -->

# Title

FigCraft Boards: diagrams from text, obstacle-avoiding connectors and VSDX files

## Body

**Problem:** Boards can draw flowcharts by hand, but elbow connectors do not avoid shapes that are not their endpoints (#1707), there is no way to build a diagram from text or tidy one, and diagram files from other tools cannot be opened or saved (`docs/boards-parity.md` lists shape libraries and hand-placed connector waypoints as `missing`).

**Proposal:** Add the pieces below as small pull requests, in this order. Each links this issue with `Part of #<this issue>`; only the routing pull request uses `Fixes`.

| # | Repo | Pull request | Waits for |
|---|---|---|---|
| 1 | figcraft | Six more flowchart shapes | nothing |
| 2 | craft-libs | `craft-graph-layout`: layered layout and orthogonal connector routing | nothing |
| 3 | craft-libs | `craft-mermaid`: Mermaid flowchart parser | nothing |
| 4 | craft-libs | `craft-vsdx`: VSDX drawing reader and writer, VSSX stencil masters | nothing |
| 5 | figcraft | One `rev` bump to the merged craft-libs commit | 2, 3, 4 |
| 6 | figcraft | Elbow connectors route around other objects | 5 |
| 7 | figcraft | Import Mermaid flowcharts and tidy diagrams (engine, MCP, CLI) | 1, 5, 6 |
| 8 | figcraft | Mermaid import dialog, paste and menu entries | 7 |
| later | figcraft | VSDX export and import on boards | 4, 5 |
| later | figcraft | Shape libraries panel and VSSX loading | 4, 5 |

Pull requests 2 to 4 are independent and can be reviewed in any order. The two "later" rows are not finished; they are listed so the direction is visible, will get their own pull requests and are not part of the first batch.

**Requests this relates to**

- Fixes #1707 (elbows through shapes, parallel elbows overlapping) with pull request 6. Nothing else here uses "Fixes".
- Refs storytold/artcraft#2121 and storytold/craft-launcher#801: the file format, stencils and glued orthogonal connectors they ask for are covered in part by the later VSDX and shape-library pull requests. Neither request is fixed by this work, which is a board feature and not a separate app.
- Second consumer for the Mermaid parser: storytold/wordcraft#4131 and storytold/wordcraft#3800 ask for Mermaid in Markdown import and rendering. The parser returns plain data (nodes, edges, subgraphs, warnings), so a WordCraft consumer can use it with the layout crate; no WordCraft work is proposed here.
- Context only, not linked as fixes: storytold/artcraft#2183 (file-format coverage), storytold/artcraft#2141 (whiteboard and diagrams), storytold/craft-libs#115 (craft-libs intake).

**craft-libs scope.** `craft-mermaid` and `craft-vsdx` are a text-format parser and a file-format reader/writer. `craft-graph-layout` has one consumer today (FigCraft) and is not a format parser. Candidate second consumers are WordCraft (Mermaid and diagram import), DeckCraft and GridCraft (the names list uses "Diagram" for their SmartArt replacement), and any app that takes on the diagramming requests above. RFC-0009 (craft-contrib PR #10) is an open draft and not binding; it asks for two consumers or an approved exception and a named responsible maintainer. Question for maintainers: how do you read it for the layout crate, and who would be its responsible maintainer? If the answer is "not yet", the crate can live inside FigCraft until a second app needs it and move later.

**Coordination**

- Mind maps (#2226, pull request #2231): mind-map branches have their own tree layout in the normalise pass. Diagram tidy and the new router will leave connectors marked as mind-map branches alone, and I will check for overlap in `board.rs` before pull request 1 and before 6. Owner of #2231: if your pull request lands first, I will rebase on it; if you see a conflict with the above, say so here.
- `.vsdx` overlap: artcraft#2121 reports a separately developed Rust `.vsdx` engine (released 2026-11-04 as a beta, MIT or Apache-2.0). `craft-vsdx` was written from the public MS-VSDX specification only, and no other implementation was read. I have not proposed depending on that project. Question for maintainers: prefer one shared crate, or is a second reader and writer acceptable? I will not start the VSDX FigCraft pull requests until this is answered.
- `.jam` import (#1808, claimed): if VSDX import also creates native board shapes and connectors, the mapping from file shapes to board shapes should be shared. I will ask on #1808 before writing that part.
- Shape names: #1705 added 16 shapes. Pull request 1 adds six that are not in `BoardShape::ALL`; its description records the check.

**Clean room.** Layout from published papers, Mermaid from its public documentation, VSDX from MS-VSDX v20250819. Each pull request lists its sources; none reads another implementation's source.

**Not in scope:** a separate diagramming app, other diagram kinds than flowcharts, draw.io files, the binary `.vsd` format.

**Open questions:** (1) RFC-0009 reading for `craft-graph-layout`; (2) `.vsdx` crate overlap; (3) whether shape libraries should stay FigCraft-only or share a stencil model in craft-libs later (RFC-0030 is in flight); (4) whether paste-to-import ships in the first UI pull request.

<!-- fill: claim and duplicate searches repeated on <date>; results -->
