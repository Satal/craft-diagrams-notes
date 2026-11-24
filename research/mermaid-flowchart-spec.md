# Mermaid flowchart syntax specification (docs-derived, clean-room)

## 0. Provenance

Built only from user-facing documentation. No parser source, grammar or third-party implementation was opened. Pages were read via WebFetch (a summarising fetch tool, so exact wording of the docs was not seen; anything marked UNVERIFIED should be tested against real Mermaid output or treated as a design decision).

| Source | Read |
|---|---|
| https://mermaid.js.org/syntax/flowchart.html | 2026-11-21 |
| https://mermaid.js.org/intro/syntax-reference.html | 2026-11-21 |
| https://mermaid.js.org/config/directives.html | 2026-11-21 |
| https://mermaid.js.org/intro/ | 2026-11-21 |
| Per-diagram pages for header keywords: syntax/stateDiagram, pie, mindmap, entityRelationshipDiagram, timeline, sankey, block, c4, requirementDiagram, architecture, kanban, packet (.html) | 2026-11-21 |
| https://github.com/mermaid-js/mermaid/blob/develop/LICENSE (licence text only, not code) | 2026-11-21 |

Version described: docs site shows **Mermaid 12.3.0**. Version markers: edge ids/animation and `@{}` shapes 11.3.0; per-edge curve 11.10.0; collapsible subgraphs 11.17.0; step animation 12.3.0; v12.0.0 changed default theme/look/layout (ELK) - irrelevant to parsing.

Licence: repository is **MIT** (Copyright 2014-2022 Knut Sveidqvist). The LICENSE has nothing documentation-specific; the grant covers "software and associated documentation files". The docs pages themselves state no separate licence on the pages read. Implication: this spec is paraphrase with original examples; no docs text is copied.

## 1. Document header

- First significant line declares the type: `flowchart <DIR>` or `graph <DIR>` (synonyms). Direction optional in practice (UNVERIFIED; default TB). `flowchart-elk` still parses in v12 (treat as `flowchart`).
- Directions: `TB` (top to bottom), `TD` (same as TB), `BT`, `RL`, `LR`.
- Front matter: YAML block fenced by `---` lines before the diagram type line. The opening `---` must be the only content on the first line. Keys like `title:` and `config:` (theme, look, layout, htmlLabels, markdownAutoWrap, flowchart.curve...). Importer: skip the block entirely; optionally read `title`. Misspelt keys are silently ignored upstream; malformed YAML breaks the diagram.
- Comments: `%%` starts a comment that runs to end of line; the docs say comments must be on their own line, and flow syntax inside them is ignored. Avoid `{}` in comments (looks like a directive and breaks rendering).
- Directives: `%%{ init: {...} }%%` (also `initialize`), may be multi-line, may sit above or below the diagram. Deprecated since 10.5 in favour of front-matter `config`. Importer: recognise and skip (balanced to the closing `}%%`); ignore content.
- Statement separators: newline; optional `;` (optional semicolons since 0.2.16). Several statements may share a line separated by `;` (UNVERIFIED but implied). Spaces around links optional (`A-->B`).

## 2. Nodes

- A bare id is a node whose label is its id: `A`. Ids are alphanumeric-ish words; punctuation/spaces in ids are not documented, so restrict ids to `[A-Za-z0-9_]` plus tolerate `-`/unicode as a lenient extension (UNVERIFIED).
- Label via shape brackets: `A[Text]`. If a node is mentioned several times, the **last** text/shape defined wins; later mentions may omit text.
- Quoting: `A["text with (parens) or end"]`. Use quotes when label contains special characters or the word `end`, or for nested shape characters.
- Unicode is fine in quoted labels (`A["Heart ❤"]`).
- Line breaks: `<br/>` (also `<br>`) in labels (not on docs page read; common usage, UNVERIFIED); markdown strings preserve real newlines.
- Markdown strings: label wrapped as `"` + backtick ... backtick + `"`, e.g. ``A["`**bold** and *italic*`"]``. Works on node, edge and subgraph labels. Bold `**x**`, italic `*x*`, auto word-wrap (can be disabled by config). Importer: strip backticks, optionally keep emphasis as plain text.
- Entity codes: `#name;` or `#decimal;` e.g. `#quot;` = `"`, `#9829;` = heart, `#35;` = `#`. Decode in labels.
- Icons in text: `fa:fa-name` style tokens in labels; ignore or leave literal.

### 2.1 Classic shapes

| Syntax | Shape |
|---|---|
| `id[t]` | rectangle |
| `id(t)` | rounded rectangle |
| `id([t])` | stadium |
| `id[[t]]` | subroutine |
| `id[(t)]` | cylinder / database |
| `id((t))` | circle |
| `id(((t)))` | double circle |
| `id>t]` | asymmetric (flag); only this orientation exists |
| `id{t}` | rhombus / decision |
| `id{{t}}` | hexagon |
| `id[/t/]` | parallelogram (lean right) |
| `id[\t\]` | parallelogram alt (lean left) |
| `id[/t\]` | trapezoid (wide at bottom) |
| `id[\t/]` | trapezoid alt (wide at top) |

Lexing hint: the opener determines the closer; longest opener wins (`(((`, `((`, `([`, `(`; `[[`, `[(`, `[/`, `[\`, `[`; `{{`, `{`; `>`).

### 2.2 Extended shape syntax (11.3.0+)

Form: `id@{ shape: NAME, label: "text" }`. Body is a small YAML-ish/flow-map list of `key: value` pairs separated by commas; values may be quoted. Keys: `shape`, `label`, `icon`, `img`, `form`, `pos`, `h`, `w`, `constraint`. `A@{ shape: rect }` equals `A["A"]`. May appear on a node inside an edge statement (`A@{...} --> B`) (UNVERIFIED).

Icon form: `A@{ icon: "fa:user", form: "square"|"circle"|"rounded", label: "...", pos: "t"|"b", h: 60 }`. Image form: `A@{ img: "url", label: "...", pos: "t"|"b", w: 60, h: 60, constraint: "on"|"off" }`. Importer: treat as a labelled placeholder box.

Full shape table (short name; meaning; aliases). Aliases are interchangeable with the short name.

| Short | Meaning | Aliases |
|---|---|---|
| rect | rectangle / process | proc, process, rectangle |
| rounded | rounded rectangle / event | event |
| stadium | stadium / terminal | pill, terminal |
| fr-rect | framed rectangle / subprocess | framed-rectangle, subproc, subprocess, subroutine |
| cyl | cylinder / database | cylinder, database, db |
| circle | circle | circ |
| sm-circ | small circle / start | small-circle, start |
| dbl-circ | double circle | double-circle |
| fr-circ | framed circle / stop | framed-circle, stop |
| f-circ | filled circle / junction | filled-circle, junction |
| cross-circ | crossed circle / summary | crossed-circle, summary |
| diam | diamond / decision | decision, diamond, question |
| hex | hexagon / prepare | hexagon, prepare |
| odd | odd (asymmetric flag) | - |
| lean-r | lean right / data in-out | in-out, lean-right |
| lean-l | lean left | lean-left, out-in |
| trap-b | trapezoid base bottom / priority | priority, trapezoid, trapezoid-bottom |
| trap-t | trapezoid base top / manual operation | inv-trapezoid, manual, trapezoid-top |
| sl-rect | sloped rectangle / manual input | manual-input, sloped-rectangle |
| notch-rect | notched rectangle / card | card, notched-rectangle |
| notch-pent | notched pentagon / loop limit | loop-limit, notched-pentagon |
| div-rect | divided rectangle | div-proc, divided-process, divided-rectangle |
| lin-rect | lined rectangle / shaded process | lin-proc, lined-process, lined-rectangle, shaded-process |
| st-rect | stacked rectangle / multi-process | processes, procs, stacked-rectangle |
| tag-rect | tagged rectangle | tag-proc, tagged-process, tagged-rectangle |
| bow-rect | bow-tie rectangle / stored data | bow-tie-rectangle, stored-data |
| win-pane | window pane / internal storage | internal-storage, window-pane |
| delay | half-rounded rectangle / delay | half-rounded-rectangle |
| h-cyl | horizontal cylinder / DAS | das, horizontal-cylinder |
| lin-cyl | lined cylinder / disk | disk, lined-cylinder |
| curv-trap | curved trapezoid / display | curved-trapezoid, display |
| doc | document | document |
| lin-doc | lined document | lined-document |
| docs | stacked document | documents, st-doc, stacked-document |
| tag-doc | tagged document | tagged-document |
| tri | triangle / extract | extract, triangle |
| flip-tri | flipped triangle / manual file | flipped-triangle, manual-file |
| hourglass | hourglass / collate | collate |
| bolt | lightning bolt / com link | com-link, lightning-bolt |
| brace | curly brace / comment | brace-l, comment |
| brace-r | brace right | - |
| braces | braces both sides | - |
| flag | flag / paper tape | paper-tape |
| fork | filled bar / fork-join | join |
| bang | bang | - |
| cloud | cloud | - |
| datastore | data store | data-store |
| folder | folder | directory |
| browser | browser window | - |
| console | console | - |
| bucket | object storage bucket | - |
| person | person icon | - |
| text | plain text block | - |

Unknown shape names: upstream behaviour unspecified; importer should warn and fall back to rect. Importer mapping suggestion: map to the nearest of {rect, rounded, stadium, circle, diamond, hexagon, cylinder, parallelogram, trapezoid, subroutine, other-as-rect} and keep the original name in metadata.

## 3. Links (edges)

| Syntax | Meaning |
|---|---|
| `A --> B` | arrow |
| `A --- B` | open line |
| `A -.-> B` / `A -.- B` | dotted arrow / dotted line |
| `A ==> B` / `A === B` | thick arrow / thick line |
| `A ~~~ B` | invisible (layout only; draw nothing) |
| `A --o B` | circle end |
| `A --x B` | cross end |
| `A <--> B` | arrows both ends |
| `A o--o B` | circle both ends |
| `A x--x B` | cross both ends |

Also `<-.->`, `<==>` exist in practice (UNVERIFIED in the docs read).

Length: extra characters in the body lengthen the rank span. Normal `---`,`----`,`-----`; arrow `-->`,`--->`,`---->`; thick `===`,`====`; thick arrow `==>`,`===>`,`====>`; dotted `-.-`,`-..-`,`-...-`; dotted arrow `-.->`,`-..->`,`-...->`. Importer: treat as same kind, optional min-rank hint = extra chars.

Edge text, two syntaxes:
- Pipe: `A -->|label| B`, `A ---|label| B`.
- Inline: `A -- label --> B`, `A -- label --- B`, `A -. label .-> B`, `A == label ==> B`. With inline text and a longer span, add dashes at the right: `A -- label ----> B`.
- Labels may be quoted, markdown strings, with entity codes.

Chaining: `A --> B --> C`; each link has its own label/kind: `A -- t1 --> B -- t2 --> C`.
Multi-node: `A & B --> C & D` = cartesian product (4 edges, order A-C, A-D, B-C, B-D per the docs' expanded form), `a --> b & c --> d` chains through the group.
Node shape/class shorthand can appear on any of these operands: `A:::cls & B[Label] --> C`.

Edge ids (11.3+): `A e1@--> B` (id directly attached to the arrow token, whitespace before). Later statements: `e1@{ animate: true }`, `e1@{ animation: fast|slow }`, `e1@{ curve: linear }` (curves: basis, bumpX, bumpY, cardinal, catmullRom, linear, monotoneX, monotoneY, natural, step, stepAfter, stepBefore), `e1@{ step: 1 }` (12.3). Edge ids can be targets of `class` and `linkStyle`-like class animation. Importer: parse and ignore props; keep id.

Edge ordering matters: `linkStyle N` indexes edges by zero-based order of definition in the source, after `&` expansion and chain splitting.

## 4. Subgraphs

```
subgraph id1 [Title]     %% id plus bracket title
subgraph "Title"         %% quoted title only (id derived from title)
subgraph Title           %% bare words: title = id
  direction LR           %% optional, per-subgraph
  ...statements...
end
```
- `end` closes. Nesting allowed. A node belongs to the subgraph(s) in which it is first/also mentioned (membership by appearance; UNVERIFIED for nodes mentioned in two subgraphs).
- Edges may target/originate at a subgraph id (`one --> two`, `A --> sg1`).
- `direction` inside subgraph is ignored if any inner node links to outside nodes; it then inherits parent direction.
- 11.17: `sgid@{ view: collapsed|expanded }`: collapsed shows a single node; crossing edges redirected, internal edges dropped. Importer: may ignore (render expanded) and report.

## 5. Styling

- `style id fill:#f9f,stroke:#333,stroke-width:4px,color:#fff,stroke-dasharray:5 5` - comma-separated CSS pairs. Maps: `fill` -> fill, `stroke` -> stroke colour, `stroke-width`, `color` -> text colour, `stroke-dasharray` -> dash. Other CSS keys: ignore.
- `classDef name prop:val,...;` ; multiple names: `classDef a,b ...`. `classDef default ...` applies to nodes with no class. Trailing `;` allowed. Commas inside values (e.g. dasharray in animations) escaped `\,`.
- `class n1,n2 name;` assigns. Shorthand `A:::name`, also on multi-operand lines. Edge ids can take class too.
- `linkStyle 3 stroke:#ff3,stroke-width:4px,color:red;`, `linkStyle 1,2,7 color:blue;`, `linkStyle default ...`. Index = edge definition order, 0-based.
- External CSS is unreliable; classDef is the supported route.
- Importer minimum: fill, stroke, stroke-width, color on nodes and links; resolve precedence default class < named class < `style` (UNVERIFIED precedence; `style` presumably wins).

## 6. Interaction

`click id callback`, `click id call callback()`, `click id "url" ["tooltip"] [_self|_blank|_parent|_top]`, `click id href "url" ...`. Disabled under strict security. Safe to ignore entirely (consume the line); optionally keep the URL as a node hyperlink.

## 7. Other diagram types (detect and report unsupported)

First non-comment, non-frontmatter, non-directive token:
`sequenceDiagram`, `classDiagram`, `stateDiagram` / `stateDiagram-v2`, `erDiagram`, `journey`, `gantt`, `pie`, `quadrantChart`, `requirementDiagram`, `gitGraph`, `C4Context` / `C4Container` / `C4Component` / `C4Dynamic` / `C4Deployment`, `mindmap`, `timeline`, `zenuml`, `sankey` (docs also use `sankey-beta` historically), `xychart-beta` / `xychart`, `block` (`block-beta` historic), `packet` (`packet-beta`), `kanban`, `architecture-beta`, `radar-beta`, `usecase-beta`, `treemap`, plus newer listed types without verified keywords: swimlanes, event modeling, venn, ishikawa, wardley, cynefin, treeView, agentflow. Keywords verified from per-type pages: state, pie, mindmap, erDiagram, timeline, sankey, block, C4*, requirementDiagram, architecture-beta, kanban, packet. Others from the intro page examples or memory (UNVERIFIED): zenuml, radar-beta, treemap and the newer set. Report "unsupported diagram type X" on any unknown first keyword.

## 8. Ambiguities and edge cases (with doc guidance)

1. Lowercase `end` as a word in a node label/id breaks flowcharts and sequence diagrams. Quote the label, or capitalise (`End`, `END`).
2. After a link, a node id starting with `o` or `x` glued to the link (`A---oB`, `A---xB`) is read as circle/cross edge end. Put a space (`A--- oB`) or capitalise.
3. Nested shape characters in labels (e.g. parentheses inside `[...]`) break parsing unless the label is quoted.
4. `%%` comment containing `{}` may be read as a directive.
5. Asymmetric shape only exists as `>text]`.
6. Requested rank span is only a hint; renderer may lengthen.
7. Repeated node definitions: last label wins.
8. Typos in keywords break the diagram; unknown front-matter keys are ignored.
9. Alias duplicates in the shape table (`cloud`, `tag-doc`) are harmless.
10. Reserved-word list is not given beyond `end`; also `graph`/`flowchart`, `subgraph`, `direction`, `style`, `class`, `classDef`, `click`, `linkStyle`, `default` are statement keywords (UNVERIFIED as ids), so avoid them as ids and handle gracefully.
11. Version drift: ELK/default look changes in v12 do not affect syntax.

## 9. Proposed test corpus

Notation: `N(id,shape,"label")`, `E(src,dst,kind,"label")`, kinds: arrow, open, dotted-arrow, dotted, thick-arrow, thick, invisible, circle, cross, bi-arrow, bi-circle, bi-cross. `SG(id,"title",[members])`. `dir=`.

```
T01  flowchart LR\nA --> B
     dir=LR N(A,rect,"A") N(B,rect,"B") E(A,B,arrow)
T02  graph TD\nA-->B
     dir=TD(=TB) same nodes; E arrow
T03  flowchart TB;A;B;A-->B;
     semicolons: N(A) N(B) E(A,B,arrow)
T04  ---\ntitle: Demo\nconfig:\n  theme: dark\n---\nflowchart BT\nX-->Y
     frontmatter skipped; dir=BT; E(X,Y,arrow)
T05  flowchart RL\n%% a comment A --> Z\nA --> B
     comment ignored; only E(A,B,arrow); dir=RL
T06  %%{init: {"theme":"forest"}}%%\nflowchart LR\nA-->B
     directive skipped; E(A,B)
T07  flowchart LR\nA[Rect] B(Round) C([Stad]) D[[Sub]] E[(DB)] F((Circ))
     shapes rect, rounded, stadium, subroutine, cylinder, circle
T08  flowchart LR\nA>Flag] B{Dec} C{{Hex}} D(((Dbl)))
     shapes asymmetric, diamond, hexagon, double-circle
T09  flowchart LR\nA[/P1/] B[\P2\] C[/T1\] D[\T2/]
     lean-r, lean-l, trapezoid(bottom wide), trapezoid-alt(top wide)
T10  flowchart LR\nA["Quoted (with) {chars}"]
     N(A,rect,"Quoted (with) {chars}")
T11  flowchart LR\nA["End of line"] --> B["end"]
     labels "End of line", "end"; quoted end legal
T12  flowchart LR\nA["a<br/>b"]
     label "a" newline "b"
T13  flowchart LR\nA["quote #quot;x#quot; and #9829;"]
     label `quote "x" and (heart)`
T14  flowchart LR\nA["`**Bold** line\nsecond`"]
     markdown label; text "Bold line / second" (emphasis optional)
T15  flowchart LR\nA@{ shape: diamond, label: "Choose" }
     N(A,diamond,"Choose")
T16  flowchart LR\nA@{ shape: cyl }\nB@{ shape: db, label: "Store" }\nC@{ shape: question }
     cylinder"A", cylinder"Store", diamond"C"
T17  flowchart LR\nA@{ shape: processes }\nB@{ shape: stop }\nC@{ shape: nosuchshape }
     st-rect, fr-circ, unknown->warn+rect
T18  flowchart LR\nA@{ icon: "fa:user", form: "circle", label: "Who", pos: "t" }
     icon node placeholder labelled "Who"
T19  flowchart LR\nA --- B -.- C === D ~~~ E
     E: open, dotted, thick, invisible
T20  flowchart LR\nA --> B -.-> C ==> D --o E --x F
     arrow, dotted-arrow, thick-arrow, circle, cross
T21  flowchart LR\nA <--> B o--o C x--x D
     bi-arrow, bi-circle, bi-cross
T22  flowchart LR\nA ----> B ====> C -...-> D
     arrow(len4), thick-arrow(len4), dotted-arrow(len3); rank hints optional
T23  flowchart LR\nA -->|yes| B\nB -- no --> C\nC -. maybe .-> D\nD == sure ==> E
     labels yes,no,maybe,sure with kinds arrow,arrow,dotted-arrow,thick-arrow
T24  flowchart LR\nA -- long label ---- B
     E(A,B,open,"long label") span 3
T25  flowchart LR\nA --> B --> C
     E(A,B) E(B,C)
T26  flowchart LR\nA & B --> C & D
     E(A,C) E(A,D) E(B,C) E(B,D)
T27  flowchart LR\na --> b & c --> d
     E(a,b) E(a,c) E(b,d) E(c,d)
T28  flowchart LR\nA e1@--> B\ne1@{ animate: true }
     E(A,B,arrow,id=e1); props ignored
T29  flowchart TB\nsubgraph one [First]\n  A --> B\nend\nC --> one
     SG(one,"First",[A,B]) E(A,B) E(C,one)
T30  flowchart TB\nsubgraph "Outer"\n  subgraph inner\n    direction LR\n    X --> Y\n  end\n  Z\nend
     SG(Outer,[Z]) containing SG(inner,[X,Y],dir=LR)
T31  flowchart LR\nA:::hot --> B\nclassDef hot fill:#f96,stroke:#333,color:#fff
     A class hot -> fill #f96 stroke #333 text #fff
T32  flowchart LR\nA --> B\nclass A,B big\nclassDef big stroke-width:4px
     both nodes class big (order of classDef vs class irrelevant)
T33  flowchart LR\nclassDef default fill:#eee\nA --> B\nstyle B fill:#0f0,stroke:#000
     A fill #eee (default); B fill #0f0 stroke #000
T34  flowchart LR\nA --> B\nB --> C\nlinkStyle 1 stroke:#f00,stroke-width:3px\nlinkStyle default color:blue
     edge index 1 (B,C) red 3px; all edges text blue
T35  flowchart LR\nA --> B\nclick A "https://example.com" "tip" _blank\nclick B call cb()
     graph unchanged; click lines consumed
T36  flowchart LR\ndev --- ops\nA --- oB\nA---Xb
     edges: dev-ops open; A-oB open (node "oB"); A-Xb open (node "Xb")
T37  flowchart LR\nA---oB
     circle edge A--oB (node "B") - documented gotcha
T38  sequenceDiagram\nA->>B: hi
     report unsupported: sequenceDiagram
T39  flowchart LR\nA[One]\nA[Two] --> B
     N(A,rect,"Two") last label wins
T40  flowchart LR\nA["x"] --> B\n\n\n   C --> D   \n
     blank lines and indentation tolerated
```
