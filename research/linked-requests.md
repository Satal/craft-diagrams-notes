# Linked requests in storytold (read-only research)

Researched 2026-11-22 with `gh search issues/prs`, `gh issue view`, `gh pr view`, `gh api` GET only. Nothing was commented on, claimed, reacted to or opened.
Search covered ~40 terms across all non-archived storytold repos (issues) and ~26 terms (PRs). `gh search` caps results, so treat it as thorough, not exhaustive.

Legend: F = fixes fully, P = partly, E = enables / adjacent (PR should say "refs", not "fixes"). State is as of today. "Claim" = another account said they are working on it.

Recommendation: link PRs as follows.
- craft-libs graph-layout / mermaid / vsdx PRs: no craft-libs issue exists for any of them. Link `figcraft#1707` and `artcraft#2121` / `craft-launcher#801` from the consuming FigCraft PRs. craft-libs maintainers (aiongg) expect "code comparison, small scope, two consumers" and an entry on craft-libs#115; read that first.
- FigCraft routing PR: `Fixes figcraft#1707` (strongest, only real bug match, unclaimed, no open PR).
- Mermaid import PR: `Refs wordcraft#4131` / `wordcraft#3800` only if a WordCraft consumer is planned; they ask for Markdown import, not Boards.
- .vsdx / .vssx / shape-library PRs: `Refs artcraft#2121` and `craft-launcher#801`. Do not use "Fixes" (they ask for a whole app).

## Table, grouped by our item

### Item 1: craft-graph-layout (Sugiyama layout, nested groups, orthogonal routing, parallel separation, edge labels)

| Issue | Title | State | Relates | Fixes | Claim / existing PR |
|---|---|---|---|---|---|
| [figcraft#1707](https://github.com/storytold/figcraft/issues/1707) | Boards: elbow connectors route straight through shapes that are not their endpoints (also: parallel elbows overlap on shared segments) | OPEN | 1, 4 | F (obstacle avoidance and parallel separation) | No claim, no assignee. #1717 (merged) added hand-drawn path handles only; bflatastic: "connectors still don't route around other shapes by themselves." Reporter retested on 0.15.0, still broken (2026-11-03) |
| [deckcraft#1718](https://github.com/storytold/deckcraft/issues/1718) | Connector between top anchors of two aligned rectangles does not auto-route (straight line, no handle) | OPEN | 1 | P only if DeckCraft/WordCraft consume the router; it is an anchor-to-anchor elbow bug, not obstacles | No claim. Related merged #1851 (adjust defaults, "doesn't change routing"); open wordcraft#5565 (Hamza0101) |
| [deckcraft#1023](https://github.com/storytold/deckcraft/issues/1023) | Bend handles (draggable bend on elbow, automatic one-bend routing at right angles) | OPEN | 1 | E | No claim. #1728 closed as dup of this |
| [deckcraft#1609](https://github.com/storytold/deckcraft/issues/1609) | Connectors ("don't seem to route properly yet") | OPEN | 1 | E (vague, no repro) | No claim. Open PR deckcraft#1014 (zberry, default sites face the other shape) |
| [vectorcraft#8255](https://github.com/storytold/vectorcraft/issues/8255) | Smart connectors and dynamic anchor points for technical diagramming | OPEN | 1, 5 | E | No claim, no PR |
| [wordcraft#3631](https://github.com/storytold/wordcraft/issues/3631) | connectors::reroute is O(connectors x laid-out objects) | OPEN | 1 | E (perf of WordCraft's own rerouter) | OPUZZLES reproduced; fix PR wordcraft#4622 MERGED |
| [figcraft#2226](https://github.com/storytold/figcraft/issues/2226) | Boards: mind maps (FigJam parity) | OPEN | 1, 4 | E (separate tree layout, `figcraft_doc::mind_layout`) | TrevorBrown-dev owns it; #2228 merged, #2231 OPEN |
| [figcraft#1916](https://github.com/storytold/figcraft/issues/1916) | Boards: connectors start inside the shape border | OPEN (fixed by merged #1971) | 1 | none; router must keep `board_route::END_GAP` = 4 pt | TrevorBrown-dev, #1971 |
| [cadcraft#1205](https://github.com/storytold/cadcraft/issues/1205) | Data-driven tables, reports, connectors (wire data) | OPEN | none | not related (electrical wiring), noted to rule out | asimfali, PRs #1206/#1405/#1406 open |

No craft-libs issue mentions graph layout, Sugiyama, layered layout or edge-label placement.

### Item 2: craft-mermaid (flowchart parser)

| Issue | Title | State | Relates | Fixes | Claim / PR |
|---|---|---|---|---|---|
| [wordcraft#4131](https://github.com/storytold/wordcraft/issues/4131) | Markdown import: support Mermaid code blocks as diagrams | OPEN | 2, 4 | E now (P if a WordCraft consumer renders the parsed graph) | No claim. Triage bot split into 3 problems; no PR |
| [wordcraft#3800](https://github.com/storytold/wordcraft/issues/3800) | Add LaTeX and Mermaid rendering for Markdown | OPEN | 2 | E (Mermaid half) | No claim. Siblings wordcraft#4130 (LaTeX), #6341, #5664; PR #4500 is LaTeX only |
| [designcraft#1181](https://github.com/storytold/designcraft/issues/1181) | Frames rendered from source code (LilyPond first; Mermaid in body) | OPEN | 2 (weak) | E | No PR. Low relevance |
| [artcraft#2183](https://github.com/storytold/artcraft/issues/2183) | Proposal: file-format coverage (lists Mermaid, PlantUML, draw.io, .vsdx) | OPEN | 2, 3 | E (umbrella) | matbrgz; no claim |

No FigCraft issue asks for Mermaid import, so `board.importMermaid` has no direct request.

### Item 3: craft-vsdx (.vsdx read/write, .vssx stencils)

| Issue | Title | State | Relates | Fixes | Claim / PR |
|---|---|---|---|---|---|
| [artcraft#2121](https://github.com/storytold/artcraft/issues/2121) | App Request: Visio, a diagramming Craft app (stencils, smart connectors, .vsdx) | OPEN | 3, 4, 5, 6 | P (format, stencils, glued orthogonal routing; not a whole app) | **Competing project, see conflicts.** matbrgz's suggested v0 (open .vsdx/.vssx, glued orthogonal connectors, save .vsdx) matches our scope |
| [artcraft#2141](https://github.com/storytold/artcraft/issues/2141) | App Request: BoardCraft whiteboard and diagrams | OPEN | 3, 4, 5 | E | charles-hood asks to keep it separate from #2121. Related: craft-contrib#66 (RFC-0066 BoardCraft, XmYx, PR OPEN); artcraft-services#99 closed dup |
| [craft-launcher#801](https://github.com/storytold/craft-launcher/issues/801) | Visio-like diagramming support | OPEN, priority: low | 3, 4, 5 | P/E | tsw2k (2026-10-27) maps the gap: shape libraries, connector waypoints, .vsdx/.drawio "not tracked anywhere"; suggests deciding "a board extension in FigCraft, or a separate app" |
| [artcraft#2334](https://github.com/storytold/artcraft/issues/2334) | Diagram / Flowchart Node Canvas Module | OPEN | 3, 4 | E | barrile; no comments. Dup of closed effectcraft#1486 |
| [artcraft#2847](https://github.com/storytold/artcraft/issues/2847) | Adding App Microsoft Visio like App | OPEN | 3, 5 | E | rblumhagen, one-line request |
| [wordcraft#2028](https://github.com/storytold/wordcraft/issues/2028) | Software for drawing complex diagrams like MS Visio (UML, brainstorms) | OPEN, needs info, priority: low | 3, 5 | E | vinukn allows "a feature in figcraft"; no claim |
| [wordcraft#4866](https://github.com/storytold/wordcraft/issues/4866) | Create VisioCraft | CLOSED 2026-10-29 | 3 | none | jawbu: "Suggest this on the ArtCraft Discord" |
| [effectcraft#1486](https://github.com/storytold/effectcraft/issues/1486), [photocraft#10564](https://github.com/storytold/photocraft/issues/10564) | Visio-like canvas / "Add Microsoft visio" | CLOSED | 3 | none (dups, wrong repo) | |
| [artcraft#2174](https://github.com/storytold/artcraft/issues/2174), [#2182](https://github.com/storytold/artcraft/issues/2182), [#2184](https://github.com/storytold/artcraft/issues/2184), [#2527](https://github.com/storytold/artcraft/issues/2527), [#2336](https://github.com/storytold/artcraft/issues/2336) | Roadmap, Craft Libraries (shared stencils), compatibility map, dedup tracker, incubator RFC | OPEN | 3, 5 | E (index issues) | matbrgz meta-trackers; do not link as "fixes" |
| [craft-libs#127](https://github.com/storytold/craft-libs/issues/127) | craft-metafile: EMF/EMF+/WMF reader | OPEN | 3 (adjacent: Visio OLE previews are EMF) | E | aiongg |

### Item 4: FigCraft Boards (new shapes, elbow routing, importMermaid, tidyDiagram)

| Issue / PR | Title | State | Relates | Fixes | Claim / PR |
|---|---|---|---|---|---|
| [figcraft#1707](https://github.com/storytold/figcraft/issues/1707) | (see item 1) | OPEN | 4 | F | none |
| [figcraft#1705](https://github.com/storytold/figcraft/pull/1705) | PR: boards have 16 more shapes (23 total) | MERGED 2026-10-30 (bflatastic) | 4 | **Overlap**, not a fix | Already has trapezoid, parallelogram, hexagon, octagon, star, plus, arrows, database, document, predefined process, manual input, internal storage. Check our stadium, double circle, trapezoid down, delay, card, flag against `BoardShape::ALL` |
| [figcraft#1919](https://github.com/storytold/figcraft/issues/1919) | Boards: drag from a selected shape's handle to draw a connector / connected shape | OPEN (implemented by merged #1947, boards-parity says "done") | 4 | none | Stale-open; context only |
| [figcraft#1597](https://github.com/storytold/figcraft/issues/1597), [#1596](https://github.com/storytold/figcraft/issues/1596), [#1708](https://github.com/storytold/figcraft/issues/1708), [#1710](https://github.com/storytold/figcraft/issues/1710), [#1387](https://github.com/storytold/figcraft/issues/1387) | connected-shape quick create, connector toolbar, side attach, path handles, connector preview | CLOSED (merged #1631/#1632/#1714/#1717/#1526) | 4 | none | Existing behaviour our work must preserve |
| [figcraft#2226](https://github.com/storytold/figcraft/issues/2226) / PR [#2231](https://github.com/storytold/figcraft/pull/2231) | Mind maps | OPEN / OPEN | 4 | none | TrevorBrown-dev; `tidyDiagram` must not fight the mind-map normalize pass |
| [figcraft#2319](https://github.com/storytold/figcraft/pull/2319) | M8: Tidy up keeps order and sizes, evens gaps (design-file Tidy up) | MERGED | 4 (name clash with `board.tidyDiagram`) | none | Different feature |
| [figcraft#1207](https://github.com/storytold/figcraft/issues/1207), [#470](https://github.com/storytold/figcraft/issues/470) | Whiteboard / FigJam workspace requests | CLOSED | 4, 5 | none | #470 comment (guilhermefeitosa66) asks for the diagramming angle: smart elbow connectors, flowchart shapes. Closed by Boards (#1229/#1249) |
| [figcraft#2278](https://github.com/storytold/figcraft/issues/2278) | Support Slides, Buzz and FigJam workspaces | OPEN | 4 (weak) | none | no claim |
| [wordcraft#1534](https://github.com/storytold/wordcraft/issues/1534) | More preset shapes and image fills (also folds in closed #6133 flowchart/callout categories) | OPEN | 4 (WordCraft side) | none for FigCraft | **dkhlapov claim 2026-10-16**, PR wordcraft#2393 OPEN |
| [wordcraft#5987](https://github.com/storytold/wordcraft/issues/5987), [#3041](https://github.com/storytold/wordcraft/issues/3041), [gridcraft#2589](https://github.com/storytold/gridcraft/issues/2589), [vectorcraft#4162](https://github.com/storytold/vectorcraft/issues/4162) | Shapes menu / gallery / more shapes in other apps | OPEN | 4 (weak) | none | no FigCraft-board equivalent |

### Item 5: FigCraft Boards .vsdx export/import and shape libraries panel (.vssx)

| Issue | Title | State | Relates | Fixes | Claim / PR |
|---|---|---|---|---|---|
| [artcraft#2121](https://github.com/storytold/artcraft/issues/2121) | Visio app request | OPEN | 5 | P | see conflicts |
| [craft-launcher#801](https://github.com/storytold/craft-launcher/issues/801) | Visio-like diagramming support | OPEN | 5 | P | tsw2k lists shape libraries and .vsdx/.drawio as the missing parts |
| [wordcraft#2028](https://github.com/storytold/wordcraft/issues/2028) | Draw complex diagrams like Visio | OPEN | 5 | E | no claim |
| [artcraft#2141](https://github.com/storytold/artcraft/issues/2141) | BoardCraft | OPEN | 5 | E | charles-hood: stencil ecosystem (Cisco/Dell/AWS .vssx packs) |
| [wordcraft#2868](https://github.com/storytold/wordcraft/issues/2868) / [deckcraft#958](https://github.com/storytold/deckcraft/issues/958) | Library side pane (Insert > Library) | OPEN | 5 (name clash "Library") | none | psperera, PRs wordcraft#6784, deckcraft#2147, gridcraft#4302 OPEN, all on craft-libs#1584 (`craft-library`, PR OPEN) |
| [wordcraft#2426](https://github.com/storytold/wordcraft/pull/2426) | PR: edit a draw.io diagram in place | OPEN (renlixing87) | 5 (adjacent, draw.io not vsdx) | none | linked to wordcraft#2028 |
| [gridcraft#4212](https://github.com/storytold/gridcraft/issues/4212) | Excel Shapes and Lines definition file (mentions Visio) | OPEN | 5 (weak) | none | dwdgit1 |
| [figcraft#1808](https://github.com/storytold/figcraft/issues/1808) | .jam import: stickies/shapes/connectors become native board objects | OPEN | 5 (adjacent: import into board model) | none | **TrevorBrown-dev claim 2026-11-10: "I'm taking this one."** Connectors still import as vectors |

boards-parity.md (figcraft) lists "shape libraries", "advanced shape libraries (engineering and other icon shapes)" and "Connector waypoints placed by hand" as `missing`. No issue tracks them.

### Item 6: mermaid2vsdx CLI

| Issue | Title | State | Relates | Fixes | Claim / PR |
|---|---|---|---|---|---|
| [artcraft#2121](https://github.com/storytold/artcraft/issues/2121) | Visio app request | OPEN | 6 | E | charles-hood's MisterDiagram CLI + MCP already writes .vsdx |
| [wordcraft#4131](https://github.com/storytold/wordcraft/issues/4131) / [#3800](https://github.com/storytold/wordcraft/issues/3800) | Mermaid rendering | OPEN | 6 | E | no claim |

No issue asks for a Mermaid-to-Visio converter.

### Counts (distinct items linked; strong = fixes or direct partial fix)

| Our item | Linked | Strong |
|---|---|---|
| 1 graph-layout | 9 | 1 (figcraft#1707) |
| 2 mermaid | 4 | 0 (wordcraft#4131 closest) |
| 3 vsdx | 11 | 1 partial (artcraft#2121) |
| 4 Boards diagram | 11 | 1 (figcraft#1707) |
| 5 Boards vsdx/libraries | 8 | 2 partial (artcraft#2121, craft-launcher#801) |
| 6 mermaid2vsdx | 3 | 0 |

## Claimed by someone else / conflicts

1. **MisterDiagram (charles-hood), conflicts with items 3, 5, 6.** In artcraft#2121 charles-hood built a clean-room Rust .vsdx / ShapeSheet engine (read, render, author, glued connector routing, .vssx stencils, MCP server, PPTX export). 0.1.0-beta.1 released 2026-11-04 at github.com/MisterDiagram/MisterDiagram (MIT/Apache). He withdrew a transfer offer on 2026-10-17 ("I never heard back from the maintainers on the offer"). matbrgz keeps a fork under `artcraft-candidates/DiagramCraft` (artcraft#2336). Risk: duplicate .vsdx work. Decision needed: reuse, interoperate or proceed independently.
2. **figcraft#1808 (TrevorBrown-dev, claimed 2026-11-10)**: .jam connectors/shapes to native board objects. Overlaps if .vsdx import creates native board shapes/connectors; share the mapping.
3. **figcraft#2226 mind maps (TrevorBrown-dev), PR #2231 open.** `tidyDiagram` and the router must not alter mind-map branches (connectors marked `branch`, laid out in the normalize pass).
4. **figcraft#1705 (merged)** already added 16 shapes (23 total). Our six new shapes need a duplicate check.
5. **wordcraft#1534 (dkhlapov claim 2026-10-16), PR wordcraft#2393 open**: only a conflict if shapes move into a shared craft-libs crate.
6. **craft-libs#1584 (psperera), PR open**: shared Library pane; naming/UX clash with a "shape libraries panel".
7. **wordcraft#2426 (renlixing87), PR open**: draw.io in-place editing; overlaps only if we add draw.io.
8. No assignee or claim on figcraft#1707, deckcraft#1718, deckcraft#1023, vectorcraft#8255, artcraft#2334/#2847/#2141, craft-launcher#801, wordcraft#2028/#3799/#3800/#4131 as of today.
9. I could not establish who counts as a maintainer (a triage bot ticket-craft and a duplicate-closing account FredApps are active), so quotes below are stated direction, not formal policy.

## Maintainer direction (quotes)

No one identifiable as a maintainer has commented on figcraft#1707, artcraft#2121 or craft-launcher#801. What exists:

- figcraft docs/boards-parity.md (updated 2026-11-12): "Shape auto-fit to its text, the advanced shape libraries (engineering and other icon shapes) | missing".
- Same doc: "Connector waypoints placed by hand | missing | routes are computed from the ends; no bend points of your own".
- Same doc, Connectors row: "an elbow whose short way would cross what it joins goes around their boxes, 24 units clear". Routing avoids only the two joined shapes, which is the #1707 gap.
- bflatastic on figcraft#1707 (2026-10-30): "connectors still don't route around other shapes by themselves."
- tsw2k on craft-launcher#801 (2026-10-27, community): "Boards are scoped to whiteboard parity, so none of that is tracked anywhere right now."
- jawbu on wordcraft#4866 (closed 2026-10-29): "Suggest this on the ArtCraft Discord".
- aiongg closing craft-libs#94 (2026-10-19): "...the 'code comparison, small scope, two consumers' rule on #115". New craft-libs crates should expect that bar; craft-libs#115 is the intake issue.
- FredApps closed wordcraft#6133 and deckcraft#1728 as duplicates of #1534 and #1023: link PRs to the canonical issue.
- Labels: craft-launcher#801 and wordcraft#2028 are `priority: low`; wordcraft#2028 also `needs info`; figcraft#1707 is `priority: medium`.
- Explicit "won't do": none found.

## Caveats
- Did not read every comment on the large meta-trackers (artcraft#2174/#2183/#2184/#2527); treated as index issues.
- `gh search` is capped and rate limited; issues in cadcraft, effectcraft, photocraft were only title-checked.
