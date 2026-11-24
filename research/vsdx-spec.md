# VSDX / VSSX implementer's specification (clean-room)

Status: research notes, written 2026-11-21. Sources are public only. No implementation source was read.

## 0. Method, sources, confidence

Rules: `craft-contrib/standards/clean-room.md` (learn from public specs only; cite section; no remembered internals).

Short names used in citations:

- **MSV** = `https://learn.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-vsdx/<GUID>` ([MS-VSDX], page titled in brackets; edition = learn.microsoft.com pages updated 2024-04 / 2024-08, no version number shown on pages).
- **VDev** = `https://learn.microsoft.com/en-us/office/client-developer/visio/<slug>` (Visio developer docs, ShapeSheet reference, Visio XML reference).
- **OPC** = ECMA-376 Part 2 / ISO/IEC 29500-2:2012, cited via MSV pages 15a5e9d3 (section 10.1.2 content types), 86c4746a (section 9.1 parts), section 9.3 relationships. The OPC text itself was not re-fetched.

PDF = the full [MS-VSDX] v20250819 PDF (Release August 19, 2025), downloaded from the public link on the MS-VSDX landing page into `research/src-pdf/ms-vsdx-250819.pdf`; cited as `PDF §n` (section numbers as in its table of contents). Visio developer pages that rendered empty to plain fetch were read through a real browser (Playwright) and are cited by VDev slug.

Confidence tags: [S] stated by a fetched spec page; [D] stated by Visio developer docs; [N] NOT confirmed from a public page fetched in this session (implement defensively, verify against real files from our own generator or round-trip tests). Anything tagged [N] must not be treated as normative.

Update 2026-11-21: a second pass with a real browser and the MS-VSDX PDF resolved most [N] items; resolved items now carry their citation. Items still open are listed in section 13. The PDF drops some glyph-only table cells (for example the arrowhead picture column), so those remain open.

## 1. Package (OPC)

### 1.1 Container

- A .vsdx is an OPC package: a ZIP of parts plus `[Content_Types].xml` and `.rels` relationship parts. [D] Introduction to the Visio file format (.vsdx): VDev `introduction-to-the-visio-file-formatvsdx`. The "folders" are part-name prefixes, not real directories.
- Extensions: .vsdx drawing, .vsdm macro drawing, .vssx stencil, .vssm macro stencil, .vstx template, .vstm macro template. Macros only in the "m" variants. [D] same page.
- Content-type and part rules: OPC section 10.1.2 and 9.1 as cited by MSV `15a5e9d3-1653-4a27-b326-ef57c87010c9`, `86c4746a-7cad-41e0-a8eb-ee8fa420f4c7`. [S]

### 1.2 Parts that matter (MSV 2.3.1, GUID 33056f18-c258-4a29-8026-03c58cbc8bca) [S]

Enumerated parts: App, Comments, Connections, ContentType, Core, Custom, Document, Extensions, Image, Master, Masters, Page, Pages, Recordsets, Rels, Theme. All other parts are "unused and MUST be ignored". So readers MUST skip unknown parts; writers need only emit the minimum below.

Relationship sources [S]:

| Source part | Targets |
|---|---|
| package (`/_rels/.rels`) | Document (exactly one, explicit), App, Core, Custom |
| Document | Masters, Pages, Theme, Connections, Recordsets, Comments, Extensions |
| Masters | Master (each) |
| Pages | Page (each) |
| Page, Master | Image |

windows.xml: the intro page says the `Windows` element is the root of the window part [D]; MSV 2.3.1 does not list it, so it is ignorable. Whether it is needed to open a file in Visio is not stated by any public page (still [N]); we do not rely on it.

### 1.3 Part table

| Part | Content type | Relationship type (from source) | Root element | Cite |
|---|---|---|---|---|
| Document | `application/vnd.ms-visio.drawing.main+xml` (macro: `application/vnd.ms-visio.drawing.macroEnabled.main+xml`) | `http://schemas.microsoft.com/visio/2010/relationships/document` (from package) | `VisioDocument` | MSV 7ec3d7b0-0de2-4711-a7b6-92daa2020d71 (2.3.3.3) [S] |
| Pages | `application/vnd.ms-visio.pages+xml` | `http://schemas.microsoft.com/visio/2010/relationships/pages` (from Document); at most one | `Pages` | MSV 947b485d-676a-480b-96e6-c0e4d1bf58f3 (2.3.3.9) [S] |
| Page | `application/vnd.ms-visio.page+xml` | `http://schemas.microsoft.com/visio/2010/relationships/page` (from Pages) | `PageContents` | MSV 1f15c8f0-6565-465c-aefd-2be6af545e8a [S] |
| Masters | `application/vnd.ms-visio.masters+xml` | `http://schemas.microsoft.com/visio/2010/relationships/masters` (from Document) | `Masters` | MSV ac2cee21-ca0e-459b-85e3-35908a476f70 [S] |
| Master | `application/vnd.ms-visio.master+xml` | `http://schemas.microsoft.com/visio/2010/relationships/master` (from Masters) | `MasterContents` (type PageContents_Type) | MSV 10b28d2f-d32d-4b8c-96f5-dad13d32dd11 [S] |
| Theme | `application/vnd.openxmlformats-officedocument.theme+xml` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/theme` (from Document) | `theme` in DrawingML main ns `http://schemas.openxmlformats.org/drawingml/2006/main` | MSV 24711011-cb57-4f6d-8de8-5b95ac64f40a [S] |
| Image | `image/bmp`, `image/x-emf`, `image/gif`, `image/jpeg`, `image/png`, `image/tiff` | `http://schemas.openxmlformats.org/officeDocument/2006/relationships/image` (from Page or Master) | n/a (binary) | MSV 949e69ec-fcd6-4ca0-be82-c30421c1f9a0 [S] |
| Core/App/Custom props | OPC / extended-properties types | package-level | see 1.6 | MSV 2.3.1 lists them as parts [S]; type strings not given by MS-VSDX ("Specified outside this document", PDF §2.3.1); still [N] |

Part names: conventional names (`/visio/document.xml`, `/visio/pages/pages.xml`, `/visio/pages/page1.xml`, `/visio/pages/_rels/pages.xml.rels`, `/visio/masters/masters.xml`, `/visio/masters/master1.xml`, `/visio/theme/theme1.xml`, `/docProps/core.xml`, `/docProps/app.xml`, `/visio/windows.xml`) are shown as examples in the intro page's prefix discussion [D]; MSV does not mandate names, relationships and content types decide. Readers MUST resolve by relationship, not by path. [S inferred from 2.3.1 relationship-based enumeration]

### 1.4 Namespaces

- Visio core: `http://schemas.microsoft.com/office/visio/2011/1/core` (root `xmlns` for Document, Pages, Page, Masters, Master). [S] MSV 7ec3d7b0, 9136adc9 ("Target namespace").
- Relationships: `http://schemas.openxmlformats.org/officeDocument/2006/relationships` (prefix `r`, for `r:id` on `Rel`). [S] MSV 34f54be6 (Rel_Type).
- Package relationships namespace `http://schemas.openxmlformats.org/package/2006/relationships` and content-types namespace `http://schemas.openxmlformats.org/package/2006/content-types`: OPC [N] (not re-fetched).
- Visio developer XML reference also shows `http://schemas.microsoft.com/office/visio/2012/main` for the later schema file `VisioSchema15.xsd` [D] VDev `cell-element-connection-rowvisio-xml`. MS-VSDX documents the 2011/1/core namespace; this spec uses it. Readers should accept both namespace URIs by local name. [N for 2012/main being accepted by Visio]

### 1.5 Package-level and Document-level structure

`VisioDocument_Type` children, in sequence, all optional [S] MSV 327dc311-1aa4-460e-ada6-9ac8c75e4c79: `DocumentSettings` (ignored), `Colors` (colour table), `FaceNames` (font table), `StyleSheets`, `DocumentSheet`, `EventList` (ignored), `HeaderFooter` (ignored), `PublishSettings`.

`Pages` contains `Page*`. `Page_Type` [S] MSV 27e583c3-0ef3-4d7c-8fac-e37bd18d9dc2: attribute `ID` (required, one-based, unique), `Name`, `NameU`, `Background`, `BackPage`; children `PageSheet` and `Rel` (required; attribute `r:id` names the relationship from the Pages part to the page part). View attributes (ViewScale, ViewCenterX/Y...) ignored.

`Masters_Type` [S] MSV 59c63b9c: `Master*`, `MasterShortcut*`. `Master_Type` [S] MSV 555d23f9-a1aa-401b-b176-80a9d0ed4109: children `PageSheet`, `Rel` (required), `Icon` (ignored); attributes `ID` (required, >= 1, unique), `BaseID` (GUID), `UniqueID` (GUID), `Name`, `NameU`, `PatternFlags` (table values 1, 2, 65, 1026, 4100, 20484), and ignored `Prompt`, `Hidden`, `IconSize`, `IconUpdate`, `MatchByName`, `AlignName`, `MasterType`.

### 1.6 Doc properties

app.xml, core.xml, custom.xml hold document properties [D] intro page. Parts exist in the MSV enumeration [S]. Package relationship type URIs for core and extended properties are standard OPC / ECMA-376 Part 2 and Part 1 strings [N, not fetched here]. They are optional for opening. Recommendation: emit core.xml (title) and app.xml only if a round-trip check shows Visio wants them; otherwise omit.

### 1.7 Minimum viable file

Required: `[Content_Types].xml`, `/_rels/.rels` with a `document` relationship, Document part, a Pages part with one Page entry and a Pages->Page relationship, and the Page part. Optional for validity per the schema: everything else (all `VisioDocument` children optional [S]; `PageContents` children optional [S]). Masters, Theme, StyleSheets, DocProps are optional on the schema evidence. PDF §3.1.1 shows a Document part with FaceNames, StyleSheets (style 0) and DocumentSheet only, no Theme. Whether desktop Visio opens a file without StyleSheets/Theme is still [N]: schema says optional but real-world tolerance is unverified; the cautious writer emits a StyleSheets part with style 0 (see section 9) and omits Masters unless used.

Skeleton (shape of file, values illustrative):

```
[Content_Types].xml        Default xml=application/xml, rels=application/vnd.openxmlformats-package.relationships+xml  [N: these two defaults are OPC convention]
                           Override /visio/document.xml -> application/vnd.ms-visio.drawing.main+xml
                           Override /visio/pages/pages.xml -> application/vnd.ms-visio.pages+xml
                           Override /visio/pages/page1.xml -> application/vnd.ms-visio.page+xml
/_rels/.rels               Relationship Type=.../visio/2010/relationships/document Target=visio/document.xml
/visio/_rels/document.xml.rels   -> pages.xml (type .../visio/2010/relationships/pages)
/visio/pages/_rels/pages.xml.rels -> page1.xml (type .../visio/2010/relationships/page)
/visio/document.xml        <VisioDocument xmlns=".../office/visio/2011/1/core" xmlns:r="...relationships"/>
/visio/pages/pages.xml     <Pages><Page ID="0"? ...>
```

Note on Page@ID: MSV says one-based and unique [S] (MSV 27e583c3). Start at 1.

## 2. XML data model: Cell / Section / Row

All property data is "cells". [S] MSV 6f23bcc4-af93-4023-a380-3e78a228e166 (Cell_Type); [D] intro page example `<Cell N="PinX" U="IN" V="2"/>`.

### 2.1 Cell attributes

- `N` (required): cell name; must be a defined cell name (MSV 2.4.4, c31ebb48-e792-4308-8bc0-ebcba281ce20). [S]
- `V`: cached value, string. "1.#INF" = very large float; `"themed"` = inherits from the theme (MSV 9c650d6d). [S]
- `U`: units. Values include `IN`, `MM`, `CM`, `M`, `FT`, `PT`, `DEG`, `RAD`, `BOOL`, `COLOR`, `STR`, `PER`, `PNT` and others. VDev default is `DL` (drawing units) when absent. [S] MSV 6f23bcc4; [D] VDev `cell-element-connection-rowvisio-xml`.
- `F`: formula. A formula string; the literal `No Formula` (locally deleted or blocked); or `Inh` (inherited, from master or style). [S] MSV 6f23bcc4; [D] same VDev page.
- `E`: error string, one of `#DIM!`, `#DIV/0!`, `#VALUE!`, `#REF!`, `#NUM!`, `#N/A`. When present, V holds the last valid value. [S] MSV 6f23bcc4; [D] VDev.

Evaluation rule [S] MSV 6f23bcc4: when F is present, V is used until formula evaluation is triggered and succeeds; afterwards the most recent non-error result applies. Consequence: a reader that does not evaluate formulas SHOULD use V.

### 2.2 Sections and rows

`Section_Type` [S] MSV 735b599d-1359-4767-8593-1c508a885779: `N` required (may repeat only for `Geometry`); `Del` (deleted: inherited values not used); `IX` zero-based strictly increasing, implicit if absent; children `Cell*`, `Trigger*`, `Row*`.

`Row_Type` [S] MSV 7ae7c325-097c-4225-adf5-e7b8b6695639:
- `N` for named sections (User, Property, Actions, Control, Hyperlink, ActionTag);
- `IX` for indexed sections (Character, Field, FillGradient, Geometry, Layer, LineGradient, Paragraph, Reviewer, Scratch, Tabs), implicit by counting; N and IX are mutually exclusive;
- `T` (row type) only in Geometry sections;
- `Del` as for sections.

Section names seen in MSV Sections list (ae0d39d7-aa9a-4a5d-9c51-ea0feb34e00f) include those above; the Connection section's XML name is `Connection` [D] VDev `cell-element-connection-rowvisio-xml` (rows of a "Connection Section").

### 2.3 ShapeSheet element (Shape, PageSheet, StyleSheet, DocumentSheet)

`ShapeSheet_Type` [S] MSV 5d6be8d6-1cab-4722-ba32-d73febc4e51d. Child order: `Cell*`, `Trigger*`, `Section*`, then `Text`, `Data1..3` (ignored), `ForeignData`, `Shapes` (sub-shapes). `Text` is ignored in StyleSheet, DocumentSheet, PageSheet.

Shape attributes [S] same page:
- `ID` required, unsigned int, unique; the page states ">= 4" for shapes (see 2.4).
- `Type`: `Group`, `Guide`, `Foreign` or `Shape`.
- `Name`, `NameU`.
- `Master`: ID of a Master in Masters; `MasterShape`: ID of a shape inside that master (ignored if `Master` is present).
- `Del`: master-instance shape deleted locally.
- `LineStyle`, `FillStyle`, `TextStyle`: StyleSheet IDs.

### 2.4 ID constraints and conflicts

- MSV states shape ID MUST be >= 4 [S] (MSV 5d6be8d6). Writers use IDs from 4 upward.
- MSV `PageContents_Type` lists `Connects` but the `Connects` / `Connect_Type` pages say "unused and MUST be ignored" (MSV aa20e379-d503-4a3a-81d7-355bf01547ec, via search result). The Visio developer docs describe real semantics (see section 6). This is a genuine conflict between the two public sources. Decision: write `Connects` (glue) and ALSO write explicit endpoint cells with formulas (section 6) so that a reader that ignores Connects still sees correct geometry. Readers that honour MSV MUST NOT fail if Connects is absent.

## 3. Units and coordinates

### 3.1 Units

- Visio's native length unit in the file is inches; `U="IN"` seen in the intro example `<Cell N="PinX" U="IN" V="2"/>` [D]. A cell's V is in the unit named by U, default drawing units [D].
- When `U` is absent a vLength cell value is inches: lengthInternalUnitNumber is inches [S] PDF §2.5.8.3, and the malformed-cell default table gives "0.00 inches" for vLength [S] PDF §2.2.5.3.3.1 (Cell Default Values: a cell with no V is "malformed"; default for vLength is 0.00 inches, vAngle 0.00 radians). Writer rule: still write `U="IN"` on length cells for clarity (the spec's own examples omit U on shapes, PDF §3.1.3). Length cells are vLength tokens (MSV 418310fd, b68fb65a, f8906ddd: "vLength custom token grouping") [S]. A reader should honour U if present and otherwise treat length values as inches [S] PDF §2.5.8.3.
- Angles: `U="DEG"` or `U="RAD"` possible [S]. The Angle cell's internal unit is radians [S] PDF §2.5.8.1 (angleInternalUnitNumber) and §2.4.4.13; a bare V is therefore radians. Writing `U="RAD"` is harmless. See 3.3.

### 3.2 Coordinate system

- Cartesian, x horizontal, y vertical [S] MSV 99f006f5-c67e-4e29-8a19-134ae611fd8f. Y increases upward and the page origin is the lower-left corner: [S] PDF §2.2.2.2 ("The origin of a drawing page is the lower-left corner of the drawing page ... increasing the y-coordinate specifies the position upward").
- Local-to-parent transform, in this exact order [S] MSV 99f006f5:
  1. subtract LocPinX; 2. subtract LocPinY; 3. if FlipX = 1 mirror about the y-axis; 4. if FlipY = 1 mirror about the x-axis; 5. rotate counter-clockwise about the origin by Angle; 6. add PinX; 7. add PinY.
  
  Equivalent: a point (x, y) in shape space maps to parent `P = R(angle) * F * ((x, y) - (LocPinX, LocPinY)) + (PinX, PinY)`, with `F = diag(FlipX ? -1 : 1, FlipY ? -1 : 1)`.
  
  Important: the spec subtracts the loc-pin before flipping, so flips act about the pin, not about the shape centre. [S inferred]
- Shape-local space: origin at the shape's bottom-left; x in [0, Width], y in [0, Height] for an unrotated, unflipped shape. [S] by PDF §3.1.3 sample (RelMoveTo 0,0 is "the left edge ... bottom edge of the shape" in the notes table; Width 2 / LocPinX 1 = "one inch right from the left edge of the shape") and PDF §2.2.3.2.1.1 (relative coordinates are scalars times Width/Height).
- A shape's PinX/PinY is in the coordinate system of the shape's parent (page, or enclosing group's local space) [S] MSV b68fb65a: "defined in relation to the coordinate system of the shape's parent".
- LocPinX/LocPinY "defined in relation to the coordinate system of the shape" [S] MSV 418310fd. (The page text for both PinX and LocPinX calls them "x-coordinate of the center of rotation".) The pin is the point of the shape that sits at PinX/PinY on the parent.

### 3.3 Converting from a top-left px/pt system

Given board coordinates (bx, by) with origin top-left, y down, in px at `ppi` (e.g. 96) or pt (72 per inch), and the page size `PW x PH` inches:

```
inch_x = bx / ppi
inch_y = PH - (by / ppi)
```

Shape rect (x, y, w, h) in top-left px, no rotation, LocPin at the centre:

```
Width  = w / ppi;  Height = h / ppi
PinX   = (x + w/2) / ppi
PinY   = PH - (y + h/2) / ppi
LocPinX = Width/2;  LocPinY = Height/2
```

Rotation: board clockwise degrees become Visio counter-clockwise: `Angle = -theta` (radians if the cell is in radians, see below) because the transform rotates counter-clockwise in a y-up system [S derived from MSV 99f006f5]. Flip: a board's y-down mirror is already accounted for by the y inversion above; flips map to FlipX/FlipY directly.

Angle: counterclockwise-increasing, radians internally [S] PDF §2.4.4.13 ("Increasing numbers indicate counterclockwise rotation") and §2.5.8.1; relative to the parent's coordinate system. The VDev Angle page (angle-cell-shape-transform-section) says the default 1-D formula is `=ATAN2(EndY-BeginY,EndX-BeginX)` [D]. Form: `<Cell N="Angle" V="0.5235987755983"/>` (U optional). Transform order local->parent: subtract LocPinX/LocPinY, mirror about y-axis if FlipX=1, mirror about x-axis if FlipY=1, rotate counterclockwise by Angle, add PinX/PinY [S] PDF §2.2.3.2.1.

## 4. Page contents

### 4.1 PageContents

`PageContents_Type` [S] MSV 2a3ecbee / 5f4ff60e: `Shapes` (0..1) and `Connects` (0..1). `Shapes_Type` [S] MSV 4f0a2823-44fc-487d-a588-f234df1147bd: sequence of `Shape`. Root element name `PageContents`; a master's root is `MasterContents` (same type) [S] MSV 10b28d2f.

### 4.2 PageSheet (page size)

`PageSheet_Type` [S] MSV f81673b1-da84-4754-b19e-a0475d889120: `Cell`, `Trigger`, `Section` children; attributes `LineStyle`, `FillStyle`, `TextStyle`. Page cells `PageWidth` and `PageHeight` (both vLength, MUST be > 0 [S] PDF §2.4.4.251) are relevant only inside a PageSheet and ignored in DocumentSheet, Shape and StyleSheet [D] VDev Visio XML reference "PageWidth/PageHeight element" (via search result; pages from the Visio 2007 to 2010 XML reference, learn.microsoft.com/en-us/previous-versions/office/developer/office-2007/aa722186(v=office.12)). Values are drawing units (inches when U="IN"). [D] `<PageSheet><Cell N="PageWidth" V="11"/><Cell N="PageHeight" V="8.5"/>`: example form [S] PDF §3.1.2 (`<Cell N="PageWidth" V="8.5"/>`, PageHeight 11, PageScale 1, DrawingScale 1, no U).

PageScale / DrawingScale: ratio of PageScale to DrawingScale is the drawing scale; both default to 1 inch : 1 inch for unscaled pages [D] previous-versions pages aa722194, aa722015 (search result). Defaults are one inch for both [S] PDF §2.4.4.246, §2.4.4.94; the spec's own sample writes `PageScale` V=1 and `DrawingScale` V=1 with no U (PDF §3.1.2). Writer: emit both as 1.

### 4.3 Essential 2-D shape cells

All are `vLength`-type tokens with a ShapeSheet_Type parent [S]: PinX (MSV b68fb65a), PinY (df919a5d), Width (f8906ddd), Height (3a6641a8), LocPinX (418310fd), LocPinY (fd5943ad). `Angle` (2f78fef4), `FlipX` (6279be42), `FlipY` (2c77fe58): pages exist; Angle page 404 in this session, FlipX/FlipY not fetched. [N for value semantics]

Section grouping: these cells are in the "Shape Transform" section of the ShapeSheet; description: "general positioning information about a shape: width, height, angle, centre of rotation (pin), whether flipped, and how the shape should behave when resized within a group" [D] VDev `shape-transform-section`. In the XML, singleton cells are direct children of the Shape [S] MSV 5d6be8d6 (Cell children of ShapeSheet).

Required for rendering a free-standing 2-D shape: PinX, PinY, Width, Height; LocPinX, LocPinY (default to Width/2, Height/2 by formula `Width*0.5`, `Height*0.5` in masters/styles [N; the intro example shows `F="Width*0"` style formulas on Geometry cells [D]]). Always write all six explicitly.

### 4.4 Geometry sections

Rules [S] MSV c6f4364f-5fb7-49f3-993e-49d4d709aa02 (2.4.1.8) and 1b69f6b8-31d4-4b28-bc37-70ee4956d8cb (2.2.3.2.2):
- `Geometry` sections have a ShapeSheet parent, IX = 0,1,... (repeatable section name).
- Each Geometry section is one path. The first row MUST be `MoveTo`, `RelMoveTo`, `Ellipse` or `InfiniteLine`.
- `Ellipse` and `InfiniteLine` are the only segment of their path.
- `MoveTo`/`RelMoveTo` start a path or start a new sub-path after a break; every other row is a segment from the previous vertex.
- Section-level cells: `NoFill`, `NoLine`, `NoShow`, `NoSnap`, `NoQuickDrag`? Only `NoFill`, `NoLine`, `NoShow` are confirmed: `NoFill` PtgBool (MSV 6e12d295-b27d-4501-8838-af38f0f8f127): 1 = fill properties not applied to this path, 0 = applied; parent MUST be a Geometry section [S]. `NoLine` and `NoShow` (MSV d93f47f5..., 31b6d084...) analogous per the geometry page [S]. A path draws only if its layer is visible and NoShow != 1 [S]. Line and fill formatting come from the shape's line and fill cells [S].

Example form [D] intro page: `<Section N="Geometry" IX="0"><Cell N="NoFill" V="0"/>...<Row IX="1" T="MoveTo"><Cell N="X" V="0" F="Width*0"/><Cell N="Y" V="0" F="Height*0"/></Row>`.

Row `IX` values are 1-based in the intro example (row IX="1" T="LineTo") [D]. MSV says IX implicit by counting from 0 for indexed sections [S] MSV 7ae7c325. Both appear; a reader MUST take IX as given when present and count when absent; a writer should write IX explicitly, starting at 1 for the first row, matching the intro example. Resolved by the spec's own samples: Geometry rows are written with IX starting at 1 (`RelMoveTo IX="1"` ... PDF §3.1.3; the section's NoFill/NoLine/NoShow cells sit directly under `Section`), while Character, Paragraph and Tabs rows in a StyleSheet start at `IX="0"` (PDF §3.1.1). Row_Type's own text calls IX "one-based" (PDF §2.3.4.2.81) but allows implicit counting; the samples are the safer model. Writer: follow the samples; reader: take IX as given, else count.

Row types (cells are named by letter) [S], MSV GUIDs on each:

| T | Cells | Meaning | Cite |
|---|---|---|---|
| `MoveTo` | X, Y | begin path or sub-path at (X,Y) in shape local coords | c8d4bbc7-1acd-4f05-8f8e-f6cd21a85651 |
| `LineTo` | X, Y | straight segment to (X,Y) | 8a289a3e-4dc1-4e90-b005-f6f0242768fa |
| `ArcTo` | X, Y, A | circular arc to (X,Y); A = distance from arc midpoint to chord midpoint ("bow"; sign selects side) | ba9a42ab-a668-41c9-b416-5fa04698f719 |
| `EllipticalArcTo` | X, Y, A, B, C, D | end (X,Y); (A,B) a point on the arc; C = angle of the major axis relative to the parent x-axis; D = major-to-minor ratio | c02155d1-394e-4744-8723-d474f6ec82e2 |
| `Ellipse` | X, Y, A, B, C, D | centre (X,Y); (A,B) first point on the ellipse; (C,D) second point. MUST be the only row in its section | 6a237f55-85cd-4b10-a1f6-325df559fe9d |
| `RelMoveTo` | X, Y | as MoveTo, relative coordinates | b358786e-c22b-4eb1-9446-611d362210c8 |
| `RelLineTo` | X, Y | as LineTo, relative | ee800037-097e-410b-abc2-ee1a3d9a9876 |
| `RelCubBezTo` | X, Y, A, B, C, D | cubic Bezier: end (X,Y); control point 1 (A,B) (begin control); control point 2 (C,D) (end control); all relative | fe4ffa27-e91f-49a9-8b1a-358692bb9e19 |
| `RelEllipticalArcTo` | X, Y, A, B, C, D | relative EllipticalArcTo; (A,B) control point, C angle, D ratio | 254aba03-1384-449d-b563-e62b6b809229 |
| `NURBSTo` | X, Y, A, B, C, D, E | end (X,Y); A second-to-last knot; B last weight; C first knot; D first weight; E a formula that must contain a NURBS function | b6a3beeb-34e3-48e6-9cf4-b74cf646149a |
| others | RelQuadBezTo (5ef65107-af0d-4883-a080-f721fab64b8d), PolylineTo (fbee4637-8599-4947-9905-65fd8fbf6481), SplineStart / SplineKnot (2d9eb38c-23c4-4bf5-9bd6-b58b64fc5d2f), InfiniteLine (aa836815-e431-444a-a88e-64450e7c5348) | cells not fetched (PDF §2.4.4 has per-cell text; geometry row semantics in PDF §2.4.1.x) [N] | |

Relative coordinate system [S] MSV 6179f5b2-a7ea-40b9-b636-17a5d6c70bab: a relative coordinate = scalar times the shape's Width (x) or Height (y). It applies to RelCubBezTo, RelEllipticalArcTo, RelLineTo, RelMoveTo, RelQuadBezTo, and to the NURBSTo E cell and PolylineTo A cell. So a relative X of 0.5 is Width/2 regardless of resizing: this is how scalable shapes are made.

Absolute (non-Rel) rows use shape-local units (inches). On resize, formulas such as `Width*0.5` keep them scaled; V caches the evaluation [D intro example `F="Width*0"`].

Practical subset (see section 11): write `MoveTo` + `LineTo` (closing by returning to the start point) for rectangles, diamonds and polygons; `Ellipse` for circles/ellipses (single-row section); optionally `RelMoveTo`/`RelLineTo`/`RelCubBezTo` for scalable shapes. A closed polygon is closed by the last vertex equalling the first [N: the closure rule is not stated in fetched pages; fill needs a closed path; always repeat the first vertex].

### 4.5 Fill, line cells

All from the Line Format / Fill Format sections; a shape's values are inherited from its LineStyle/FillStyle/TextStyle stylesheet unless overridden (section 9).

| Cell | Meaning and values | Cite |
|---|---|---|
| `LinePattern` | 0 none, 1 solid, 2..23 assorted dash patterns (index into a pattern collection); USE() for custom | [D] VDev `linepattern-cell-line-format-section` |
| `LineColor` | 0..23 index into the colour collection; custom colours via `RGB(r,g,b)` or `HSL()`; custom colours have numeric values >= 24; transparency in `LineColorTrans` | [D] VDev `linecolor-cell-line-format-section` |
| `LineWeight` | line width; unit length (inches) | vLength, MUST be > 0 [S] PDF §2.4.4.186; VDev lineweight-cell-line-format-section [D]: value with unit, in inches when not drawing-scaled; unscaled by the drawing scale [S] PDF §2.2.2.3 |
| `BeginArrow` / `EndArrow` | 0 none, 1..45 arrowhead styles (indexed Line dialog entries); `USE()` custom | [D] VDev `beginarrow-cell-line-format-section`, `endarrow-cell-line-format-section` |
| `BeginArrowSize` / `EndArrowSize` | size of arrowhead | [D] named in the BeginArrow page Remarks; 0 very small, 1 small, 2 medium, 3 large, 4 extra large [S] PDF §2.4.4.21; VDev beginarrowsize-cell page extends the table to 5 jumbo, 6 colossal [D] |
| `Rounding`, `LineCap` | rounding radius, cap style (round or square) | [D] `line-format-section` description; Rounding is a length (radius) [D] VDev rounding-cell-line-format-section; LineCap 0 rounded, 1 square, 2 extended [D] VDev linecap-cell-line-format-section |
| `FillPattern` | 0 none (transparent), 1 solid foreground, 2..40 assorted patterns; USE() custom | [D] VDev `fillpattern-cell-fill-format-section` |
| `FillForegnd`, `FillBkgnd` | foreground / background colour; same colour model as LineColor (index or RGB()) | [D] section description `fill-format-section`; vColor: `#RRGGBB` literal, or index into the colour table (0..23 in VDev; out-of-range falls back to index 0) [S] PDF §2.5.6.2, [D] VDev fillforegnd-cell page; FillPattern 0 none, 1 solid, 2..40 patterns [D] VDev fillpattern-cell |
| `FillForegndTrans`, `FillBkgndTrans`, `ShdwPattern` ... | transparency in XML is normalised 0..1 (0 opaque, 1 fully transparent) [S] PDF §2.4.4.120 (VDev page says 0-100 percent in the UI; use 0..1 in files); ShdwPattern 0..40 like FillPattern [D] | [S/D] |

Colour literal in V: the grammar has `color-value = "#" 6HEXDIG` (e.g. `#FF8800`) [S] MSV e617d7e0 (formula ABNF). So a plain V of `#RRGGBB` is a valid colour value; the index form (0..23) refers to the document's colour table (`Colors` element, MSV 235d2b62-31ad-4aa9-9a2e-9789a6a6de30 [S existence]). Writer: always use `#RRGGBB` in V (no F), never table indices, except for 0 (black) / 1 (white)? [N: do not rely on indices].

Fill/line "no fill" and "no line" for a shape: `FillPattern=0` and `LinePattern=0` [D], and per path `NoFill=1`/`NoLine=1` [S].

### 4.6 Text

`Text` element [S] MSV 3031da58-e11e-460b-9df5-9cfb6bc0a836: mixed content; child markers `cp` (character run: required `IX` = IX of a Row in the Character section), `pp` (Paragraph IX), `tp` (Tabs IX), `fld` (field). Text before the first marker uses the default formatting. `Text` is a child of `Shape` after the Sections [S] MSV 5d6be8d6.

Example form: `<Text><cp IX="0"/>Hello</Text>`. XML characters must be escaped; newlines are literal; a paragraph break is a newline in the text [N; not confirmed].

Text-block cells (Text Block Format section) [D] VDev `text-block-format-section`: "alignment, margins and default tab stop"; cells include `LeftMargin`, `RightMargin`, `TopMargin`, `BottomMargin`, `VerticalAlign`, `TextBkgnd`, `DefaultTabStop`, `TextDirection` [N for names other than VerticalAlign]. `VerticalAlign`: 0 top, 1 middle, 2 bottom [D] VDev `verticalalign-cell-text-block-format-section`.

Text-transform section [D] VDev `text-transform-section` ("positioning information about a shape's text block"; cells TxtPinX, TxtPinY, TxtLocPinX, TxtLocPinY, TxtWidth, TxtHeight, TxtAngle [N for names]): if absent the text block is the shape's own box. Writer omits it. For a custom polygon, text sits in the shape's Width x Height box.

Character section [S existence: MSV a8b6f1ba-88f8-47eb-8e5e-fb5045026efa; row IX referenced by `cp`]. Cell names (`Font`, `Color`, `Size`, `Style`) [N: not fetched]. Paragraph section (`HorzAlign` etc.) [N; the `horzalign-cell-paragraph-section` VDev slug 404'd]. Do not hard-code these names from memory beyond what a round trip proves; the writer's minimum is to omit Character/Paragraph and rely on stylesheet defaults, setting only `VerticalAlign` and text colour through a Character row only after verification.

## 5. Page-level connectors

### 5.1 Representation (from the public docs)

A connector is an ordinary `Shape` that is 1-D: it carries endpoint cells `BeginX`, `BeginY`, `EndX`, `EndY` (cell names; "x-coordinate of the begin point of the 1-D shape, in relation to the origin of its parent") [D] VDev `beginx-cell-1-d-endpoints-section`. EndX/EndY analogous [D slugs follow the same pattern; not separately fetched, N for text]. These live in a 1-D endpoints "XForm1D" row [D] (CellsSRC row `visRowXForm1D`, cells `vis1DBeginX`). Its geometry is usually a single path `MoveTo (0,0)` then `LineTo (Width, 0)` or elbow segments, with Width = distance between endpoints and Angle giving direction [N; conventional; verify].

### 5.2 Glue: the Connects element

`Connects` is a child of `PageContents` [S] MSV (PageContents_Type). Each `Connect` has 6 attributes [D] VDev `connect_type-complextypevisio-xml` (schema file VisioSchema15-2012-06-05.xsd):

- `FromSheet` (required, unsignedInt): ID of the shape (the connector) the connection originates from.
- `FromCell` (optional string): name of the cell on the source shape, e.g. `BeginX`, `EndX`. [Role of FromCell: Visio SDK 2003 description, "at least one of FromCell or FromPart must be present" ]: [D] previous-versions office-2003 aa197578.
- `FromPart` (optional int): part number of the from-side; per 2003 SDK, one of FromCell/FromPart must be present [D].
- `ToSheet` (required): ID of the shape glued to.
- `ToCell` (optional string): cell on the target, for example `PinX` (glue to the whole shape) or a connection-point row cell name such as `Connections.X<n>` [D] VDev x-cell-connection-points-section ("Cell name: Connections.X i where i = 1, 2, 3..."); Visio object-model docs: ToCell is the `Connections.X<n>` cell for a connection point, `PinX` for glue to a 2-D shape's pin (dynamic glue), `Angle` for guides [D] https://learn.microsoft.com/en-us/office/vba/api/visio.connect.tocell. NOTE: MS-VSDX marks Connect FromCell/ToCell/FromPart/ToPart "unused and MUST be ignored" (PDF §2.3.4.2.12).
- `ToPart` (optional int).

Conflict flag: MSV's Connect_Type page (learn.microsoft.com/gl-es/openspecs/sharepoint_protocols/ms-vsdx/aa20e379-d503-4a3a-81d7-355bf01547ec, from search) says the type and its attributes are "unused and MUST be ignored" (the Visio Services profile). The Visio developer reference gives them real semantics. Implementation: (a) write `Connects` for desktop Visio; (b) on read, treat `Connects` as optional hints and fall back to geometric endpoint matching if absent.

### 5.3 Making a glued connector self-consistent

Glue is expressed twice [D/N]:
1. In the connector's endpoint cells, by formula: `BeginX` with `F="PAR(PNT(Sheet.5!Connections.X1,Sheet.5!Connections.Y1))"`-style references to the target's connection point. The formula grammar has `Par`, `Pnt`, `PntX`, `PntY`, `Ref`, `Sheet.<id>!` references (`sheetref "!" CellRef`, `ShapeSheetRef`) [S] MSV e617d7e0. The exact formula Visio emits for glue is not shown by any public page read (still [N]). Cached V values for BeginX..EndY must hold the real coordinates in any case.
2. In `Connects`, one `Connect` per glued end: `FromSheet=<connector id> FromCell="BeginX" ToSheet=<target id> ToCell="PinX"` (whole-shape glue) or `ToCell` naming the connection-point cell (point glue).

Reader minimum: use cached `V` of BeginX/BeginY/EndX/EndY for the geometry, and use `Connects` (if present) only to recover which shapes each end is attached to.

### 5.4 Connection section (connection points on shapes)

Named `Connection` section, rows `IX`-indexed. Row cells [D] VDev `cell-element-connection-rowvisio-xml`:

| N | Meaning |
|---|---|
| `X`, `Y` | connection point in the shape's local coordinates |
| `DirX`, `DirY` | x/y components of the alignment vector ("A" and "B" in extended rows) |
| `Type` | 0 inward, 1 outward, 2 inward and outward (C in extended rows) [D via search of Connection Points Row page] |
| `AutoGen` | 1 = generated automatically |
| `Prompt` | reserved |

Standard rectangle connection points are the four edge midpoints, written as rows with `X`=`Width*0.5`, `Y`=`Height` etc. (formula values not shown publicly [N]); put cached values in V. Row cells are X, Y (local-coordinate lengths), DirX, DirY (floating direction vector, 0,0 = directionless) and Type (0 inward, 1 outward, 2 both) [D] VDev connection-points-row-connection-points-section; in MS-VSDX X/Y/DirX/DirY/Type here are "used during formula evaluation only" (PDF §2.4.4.380, .82, .83, .364).

### 5.5 Routing and arrows

- `ShapeRouteStyle` (Shape Layout section): 0 page default; 1 right angle; 2 straight; 3 and 4 organisation chart (top-to-bottom, left-to-right); 5 flowchart top-to-bottom [D] VDev `shaperoutestyle-cell-shape-layout-section` (GitHub copy of the same docs and search digest). Page-wide equivalent `RouteStyle` in the Page Layout section [D].
- `ConLineRouteExt` (Shape Layout): 0 page default, 1 straight, 2 curved [D] VDev `conlinerouteext-cell-shape-layout-section`.
- Arrowheads: `BeginArrow`/`EndArrow` 0..45 [D]. Which number is a plain filled triangle: do NOT assume; community posts quote values (for example 13) that are not in the official page [D caveat from search]. Writer for "arrow at end" must be verified against rendering with our own round trip before shipping; reader treats 0 as none and non-zero as some arrowhead.
- Visio re-routes connectors itself when opened and a shape moves; the file's stored geometry is the last route. For an exporter: emit the polyline path ourselves in the connector's Geometry section; set `ShapeRouteStyle` = 1 for orthogonal intent [D value meaning].

## 6. Masters and stencils

### 6.1 Structure

- Masters part lists `Master` entries (ID, NameU, UniqueID, BaseID, PatternFlags...) each with a `Rel` to a Master part [S] MSV 59c63b9c, 555d23f9.
- A Master part's `MasterContents` has `Shapes`, like a page [S] MSV 10b28d2f. Its `PageSheet` (in the Masters-part entry) carries page-style cells.
- A master may relate to an Image part (e.g. a preview) [S] 2.3.1.
- Master `Icon` element is ignored per MSV [S] (the Master_Type child). So master icons in the data stream are not required to be read; any preview image is via the master part's Image relationship (Image part has Page or Master as source, PDF §2.3.1) or the `Icon` element's opaque content [N]. For library icons, render our own thumbnail from the geometry rather than relying on this.

### 6.2 Instance and inheritance

- A page `Shape` references a master via `Master="<Master ID>"` [S] MSV 5d6be8d6. `MasterShape` (ignored if Master is present) selects a shape within that master (used for sub-shapes of group masters).
- A master-instance shape stores only overrides. Cells inherited from the master appear with `F="Inh"` (formula inherited) [S] MSV 6f23bcc4 and carry the current V value; sections/rows locally removed carry `Del="1"` [S] MSV 735b599d / 7ae7c325.
- What a reader must resolve: if a Shape has `Master` and lacks a Section or Cell, take it from the master's top shape (single-shape masters: the one shape in `MasterContents/Shapes`); geometry rows are inherited by row IX; a local row with `Del="1"` removes the inherited row; a local row with `IX` replaces/overrides the inherited row's cells. [S for Del semantics; the merge procedure is the glossary "Inheritance" page MSV 5865d55a-f28e-4dc7-b02d-79e35e8cd7eb, not fetched: N for details].
- Therefore import MUST NOT assume every master-based shape has full geometry in the page XML. Fallback if the master is missing or unreadable: draw the bounding box from Width/Height at PinX/PinY.
- Style inheritance: `LineStyle`, `FillStyle`, `TextStyle` IDs select StyleSheets from which unset line, fill and text cells inherit [S] MSV 9136adc9. The inheritance chain is shape -> master shape -> style sheet; style sheets inherit from other style sheets through their own LineStyle/FillStyle/TextStyle attributes [S].

### 6.3 .vssx stencils

- Same OPC package, extension .vssx, Document and Masters parts; the Pages part may be absent or minimal [D] intro page says the file types share one format; the `.vssx` structural differences are not specified: MS-VSDX (PDF, searched for "vssx"/"stencil": no hits) lists only drawing content types, and the intro page only names the extension [D]. Stencil content-type strings are therefore still [N].
- Stencil content = the Masters part and the Master parts it targets (+ StyleSheets/Theme from the Document). Reader for the shape-library use case reads `Masters` -> each `Master` -> `NameU`/`Name` -> `Rel` -> master part -> `MasterContents/Shapes` -> top-level shape geometry. [S for all links]
- The master shape's Width/Height give its natural size; its Geometry sections give outline; connection points from `Connection` section rows.
- Names: `Name` is language-dependent, `NameU` language-independent [S] MSV 555d23f9. Use NameU as the stable key.
- `UniqueID` and `BaseID` GUIDs identify masters across documents [S]; use UniqueID as library key.

## 7. Formulas

### 7.1 What V can be trusted for

Per MSV 6f23bcc4 the cached V is the value until evaluation is triggered; so V is the right input for an importer that does not evaluate. Exceptions needing care:
- `E` present: V is the last valid value; treat as normal if V is parseable; flag otherwise.
- V of `themed`: value comes from the theme; resolve via theme or default to a neutral (section 8).
- `F="Inh"`: V still holds the inherited value if the writer saved it; if V is absent, resolve from master or style.
- V can be absent when F alone is given: per PDF §2.2.5.3.3.1 a Cell_Type without a V attribute is "malformed" and takes the unit-dependent default (0 inches, 0 radians, #000000 ...), so a cell with only F is not guaranteed to evaluate to a useful cached value. Writer MUST emit V for every cell (resolved: no V means default, not recompute).

### 7.2 Grammar (MSV e617d7e0, "Formula ABNF and Full Grammar Definition") [S]

- Atoms: numeric literals with units (token kinds `PtgNum`, `PtgNumMM`, `PtgNumI` (inches), `PtgNumPct`, `PtgAngDD`, `PtgAngRad`, `PtgBool`, `PtgString`, `PtgColorRGB` (`#RRGGBB`), `PtgErr`...), functions, references.
- References: `[sheetref "!"] CellRef`, with `sheetref` one of `CrossPageRef`, `DocSheetRef`, `MasterSheetRef`, `PageSheetRef`, `ShapeSheetRef`, `StyleSheetRef`. Parenthesised sub-expressions allowed.
- Operators are expressed as functions in the grammar (Add, Sub, Mul, Div, Pow, UMinus, UPlus, EEQ/EGE/EGT/ELE/ELT/ENE, And, Or, Not, Cat...); infix surface forms and precedence: PDF §2.5.2 table: 1 parentheses; 2 unary +/-; 3 `^`; 4 `*` `/`; 5 binary `+` `-`; 6 `&` (concat); 7 `<` `_LT_` `<=` `_LE_` `>` `_GT_` `>=` `_GE_`; 8 `=` `_EQ_` `<>` `_NE_` (and logical operators below); left to right [S].
- Function list (complete in grammar): Abs, ACos, Add, And, Ang360, AngleToLoc, AngleToPar, ASin, ATan, ATan2, BitAnd/BitNot/BitOr/BitXor, BkgPageName, Blend, Bound, Cat, Category, Ceiling, CellIsThemed, Char, Company, Cos, CosH, Creator, CY, Date, DateTime, DateValue, Day, DayOfYear, Deg, DependsOn, Description, Directory, Div, DocCreation..., EEQ..ENE, FEQ..FNE, FieldPicture, FileName, Find, Floor, Format, FormatEx, FormulaExists, Gravity, Guard, HasCategory, Hour, HSL, Hue, HueDiff, HyperlinkBase, ID, IF, IfError, Index, Int, IntersectX, IntersectY, Intup, Is1D, IsErr, IsErrNA, IsError, IsErrValue, IsThemed, Keywords, Language, Left, Len, Ln, Loc, LocalFormulaExists, LocToLoc, LocToPar, Log10, Lookup, Lower, Lum, LumDiff, Magnitude, Manager, MasterName, Max, Mid, Min, Minute, Modulus, Month, MsoShade, MsoTint, Mul, NA, Name, Not, Now, Nurbs, Or, PageCount, PageName, PageNumber, Par, Pct, Pi, Pnt, PntX, PntY, PolyLine, Pow, Rad, Rand, Ref, Replace, RGB, Right, Round, Sat, SatDiff, Second, SetAtRef, SetAtRefEval, SetAtRefExpr, Shade, ShapeText, Sign, Sin, SinH, Sqrt, StrSame, StrSameEx, Sub, Subject, Substitute, Sum, Tan, TanH, TextHeight, TextWidth, Theme, ThemeCBV, ThemeGuard, ThemeProp, ThemeRestore, ThemeVal, Time, TimeValue, Tint, Title, Tone, Trim, Trunc, UMinus, UniChar, UPlus, Upper, Use, Version, WeekDay, Year.

### 7.3 Minimal evaluation subset (recommendation)

For import without relying on V in odd cases, implement: numeric literals with unit suffix; `Width`, `Height`, `Sheet.N!Cell`, `Cell` same-sheet refs; `+ - * /`; parentheses; `RGB(r,g,b)`; `Min`, `Max`, `Abs`, `IF(cond,a,b)`, comparisons. Everything else: use V; if no V, drop the cell and log. For export, write V always and F only where scaling matters (geometry `Width*k`, `Height*k`); never emit functions outside this subset. (Recommendation, not a spec statement.)

## 8. Themes

- Theme part is DrawingML theme [S] MSV 24711011; content type and relationship above (section 1.3).
- Cells may carry `V="themed"` or reference theme functions (`Theme`, `ThemeVal`, `ThemeProp`, `ThemeGuard`, `CellIsThemed`, `IsThemed` in the grammar) [S] MSV 6f23bcc4, e617d7e0. "Theme inheritance" is defined in the glossary [S existence: MSV 9c650d6d].
- Minimum for us: do not emit a theme part and never write `themed` or theme formulas. Importer: if V is `themed` with no resolvable value, substitute black line (#000000), white fill (#FFFFFF)? [N; pick neutral and log].
- A theme part is optional on the schema evidence (Document relationships are all optional [S]); desktop tolerance unverified [N]; PDF §3.1.1 sample has no Theme.

## 9. StyleSheets

- `StyleSheets` (child of VisioDocument, optional [S] MSV 327dc311) holds `StyleSheet*` [S] MSV a525051f.
- `StyleSheet_Type` [S] MSV 9136adc9-bd94-425e-a32d-15fee80707ef: `ID` (required, unique), `Name`, `NameU`, `LineStyle`, `FillStyle`, `TextStyle` (the IDs of parent styles to inherit from), children `Cell`, `Trigger`, `Section`.
- A shape picks styles through `LineStyle`/`FillStyle`/`TextStyle` attributes (IDs) [S] MSV 5d6be8d6. The intro page example shows `LineStyle="3" FillStyle="3" TextStyle="3"` on a shape [D, attributes named in the Shape example].
- Style ID 0 is the "No Style" sheet referenced by `LineStyle="0" FillStyle="0" TextStyle="0"` on DocumentSheet, PageSheet and Shape in the spec's own sample [S] PDF §3.1.1-3.1.3. That sample defines it with EnableLineProps/EnableFillProps/EnableTextProps=1, LineWeight 0.01041666666666667, LineColor 0, LinePattern 1, FillForegnd 1, FillPattern 1, VerticalAlign 1, Character row IX=0 (Font "Calibri", Color 0, FontScale 1, Size 0.1666666666666667), Paragraph row IX=0 (IndFirst/IndLeft/IndRight 0, SpLine -1.2, HorzAlign 1, Bullet 0, BulletStr "", BulletFont 0, BulletFontSize -1 (the notes table says 0; the XML says -1), TextPosAfterBullet 0, Flags 0), Tabs row IX=0. Safe writer approach: omit the style attributes and write every needed cell on the shape directly (no inheritance); then no StyleSheets part is required by the schema [S optionality]. Risk: a missing stylesheet might cause desktop Visio to fall back to its defaults for unwritten cells [N]; ensure every rendering-critical cell is explicit.
- Reader: resolve `LineStyle` etc. through StyleSheets to fill any cell the shape (and its master) lacks; style chains follow the style's own LineStyle/FillStyle/TextStyle attributes [S].

## 10. Colours and fonts tables

- `Colors` child of VisioDocument: the colour table; ids 0..23 are indexes used by LineColor/FillForegnd [S existence MSV 235d2b62; indexing from [D] LineColor page "0 to 23 ... collection of line colors"]. Entry element `ColorEntry` with attributes `IX` (0-based, <= 253) and `RGB` (hex string) [S] PDF §2.3.4.2.7-8.
- `FaceNames`: font table (MSV f6b0e8dc-fa26-48bd-812a-937da904b562, existence [S]); Character `FaceName` has attributes NameU (required), UnicodeRanges, CharSets, Panos, Panose, Flags [S] PDF §2.3.4.2.45. The Character `Font` cell takes a font name string in the spec's sample (`<Cell N="Font" V="Calibri"/>`, PDF §3.1.1); VDev font-cell page [D] says a font number, 0 = default font (older numbering). Writer: omit and let the character default apply, or set no font.

## 11. Recommended minimal subsets

### (a) Export board to .vsdx

Package: `[Content_Types].xml`, `/_rels/.rels`, document.xml (+rels), pages.xml (+rels), page1.xml; optionally docProps/core.xml. No Masters, no Theme, no StyleSheets (see 9 risk). One Page per board, `Page ID` from 1.

Page: `PageSheet` with `PageWidth`, `PageHeight` (inches). Shape IDs from 4 upward (MSV 5d6be8d6 constraint).

Per shape (Type="Shape"): cells PinX, PinY, Width, Height, LocPinX, LocPinY (all `U="IN"`, V only), `Angle` if rotated (verify units), `FlipX/FlipY` if mirrored; line cells `LineColor` (`#RRGGBB`), `LineWeight` (inches), `LinePattern`; fill cells `FillForegnd` (`#RRGGBB`), `FillPattern` (1 solid, 0 none); text block cells `VerticalAlign` (1 middle). Geometry:

- Rectangle: `Geometry IX=0` rows MoveTo(0,0), LineTo(W,0), LineTo(W,H), LineTo(0,H), LineTo(0,0).
- Diamond: MoveTo(W/2,0), LineTo(W,H/2), LineTo(W/2,H), LineTo(0,H/2), LineTo(W/2,0).
- Ellipse: one `Ellipse` row: X = W/2, Y = H/2; A,B = (W, H/2) (point on the ellipse at the right end); C,D = (W/2, H) (point at the top). [S] MSV 6a237f55 semantics: first and second points on the ellipse.
- Custom polygon: normalise the vertices into [0,W] x [0,H] in shape-local inches (y flipped), MoveTo + LineTo ... close with the first vertex.
- Emit geometry V values only (no F) for simplicity; add `F="Width*k"` only if you want scalable shapes. Optional.
- Text: `<Text>` after sections; escape `& < >`.

Connectors (orthogonal, glued, arrowed):
- Shape Type="Shape", 1-D: cells BeginX, BeginY, EndX, EndY (V, inches); `ShapeRouteStyle` V=1 [D]; `EndArrow` non-zero per verified index; geometry section with our computed polyline in the connector's local coordinates [N conventions: Width/Height/Pin/Angle for a 1-D shape; verify].
- Glue: `Connects` entries (FromSheet = connector ID, FromCell BeginX/EndX, ToSheet = target ID, ToCell PinX) [D/N]; also formulas on BeginX... referencing target (optional, [N]).
- For targets, add a `Connection` section with four rows (edge midpoints, `DirX/DirY` outward, `Type` 0?) [D cell names] only if we want point-level glue.

Whatever is not verified [N] must be proved by a round trip against a real Visio-saved or LibreOffice-opened sample that WE generate (fixture rule: clean-room.md "Fixtures"); do not copy sample files from other projects.

### (b) Import .vsdx to a board using cached V

1. Open ZIP. Find the Document via the package-level `document` relationship; then `pages` and each `page` via relationships; read `PageContents`. Ignore unknown parts and attributes [S].
2. Page size from the Page's `PageSheet` cells `PageWidth`, `PageHeight` (V, inches).
3. For each `Shape` in `Shapes` (recursively for `Type="Group"`): read cells by `N`. Take V; honour U. Required: PinX, PinY, Width, Height, LocPinX, LocPinY (defaults: LocPinX = Width*0.5 [D] VDev locpinx-cell page, similarly LocPinY; TxtWidth default Width*1, TxtPinX default Width*0.5 [D]); Angle, FlipX, FlipY.
4. Convert: board y = (PH - parent_y) * ppi; apply the section 3.2 transform; groups compose the transform from child space to parent space.
5. Geometry: iterate Geometry sections (skip when `NoShow`=1; fill off when `NoFill`=1; line off when `NoLine`=1); rows MoveTo/LineTo/Ellipse/ArcTo/EllipticalArcTo/Rel* and NURBS/Polyline fallbacks (approximate NURBS by chord polyline through the end points) [S row semantics].
6. Style cells: LineColor/FillForegnd V (`#RRGGBB` literal accepted; indices 0..23 via the Colors table or a default palette; `themed` -> neutral), LinePattern (0 = none), FillPattern (0 = none), LineWeight, arrows.
7. Master-based shapes: resolve per section 6.2; if unresolved, use the bounding box.
8. Connectors: 1-D shapes (BeginX..EndY present): polyline between endpoints plus any Geometry path; use `Connects` to bind ends to shape IDs.
9. Text: `Text` element, strip `cp/pp/tp/fld` markers into plain text (fields: use the cached text if present).
10. Sub-shapes under `Shapes` of a Group.

### (c) Load a .vssx stencil's masters as a shape library

Walk Document -> Masters -> each `Master` (`ID`, `NameU`, `Name`, `UniqueID`) -> `Rel r:id` -> master part -> `MasterContents/Shapes`. Take the first (or the group-parent) shape's Width/Height, Geometry sections (all path rows per 4.4) and `Connection` section rows; resolve cell values from V; resolve `Inh` cells through StyleSheets. Icon: do not rely on the Icon element ([S] ignorable); render a thumbnail from geometry ourselves (our art, per clean-room.md). Cache key: `UniqueID`.

## 12. Spec-conflict register

| # | Conflict | Resolution used |
|---|---|---|
| c1 | MSV: Connects/Connect "unused, MUST be ignored" (Visio Services profile) vs Visio developer reference defining them | Write both; read optionally |
| c2 | MSV: shape ID MUST be >= 4 vs small IDs elsewhere | Use >= 4 |
| c3 | MSV Row IX implicit from 0 for Geometry vs intro example IX="1" | Always write IX explicitly; follow the example; accept either on read |
| c4 | Namespaces 2011/1/core (MSV) vs 2012/main (VDev XML reference) | Write 2011/1/core (every PDF §3 sample uses it); read both by local name [N] |

## 13. Status of the former gaps (second pass 2026-11-21)

Resolved (see the citations inline):

1. Angle: radians internally, counterclockwise positive, relative to parent (PDF §2.4.4.13, §2.5.8.1; VDev angle-cell-shape-transform-section).
2. FlipX/FlipY: value 0 or 1; 1 mirrors about the y-axis (FlipX) / x-axis (FlipY) in the local-to-parent transform (PDF §2.4.4.126-127, §2.2.3.2.1).
4. Character cells: Font (name string in samples), Color, Size (inches), Style bit flags (0x01 bold, 0x02 italic, 0x04 underline, 0x08 small caps), Case, Pos, FontScale, Transparency; Paragraph cells: HorzAlign 0 left / 1 centre / 2 right / 3 justify / 4 distribute, IndFirst/IndLeft/IndRight, SpLine (negative = multiple of font size, e.g. -1.2), SpBefore, SpAfter, Bullet; text-block margins LeftMargin/RightMargin/TopMargin/BottomMargin, VerticalAlign 0 top / 1 middle / 2 bottom, TextDirection 0/1 (PDF §2.4.4, §2.2.8; VDev cell pages). Cell-name forms in formulas: `Char.Size[i]` etc. [D].
5. LineWeight, FillForegnd, FillBkgnd, transparency (PDF §2.4.4.186, .119-.120, .171-.172; VDev pages).
6. Glue target cell names: `Connections.X<n>`; also `PinX` for dynamic glue (VDev x-cell-connection-points-section; Visio object-model Connect.ToCell). MS-VSDX itself says Connects are ignorable (PDF §2.3.4.2.12).
8. Style 0 and StyleSheets/Theme: the spec's own sample defines style 0 and omits a Theme (PDF §3.1.1). The "does desktop Visio open it" question is NOT answered by any public page; see open item O2.
9. Text: `Text` is mixed content with cp/pp/tp/fld children (PDF §2.3.4.2.96, §2.2.8). All spec sample parts carry `xml:space="preserve"` on the root (PDF §3.1). Newline handling is not specified (see O4).
11. PageScale/DrawingScale: default one inch each; samples write 1 and 1 (PDF §2.4.4.246, §2.4.4.94, §3.1.2).
12. Formula operators: PDF §2.5.2 precedence table (see section 8).
Also resolved: coordinate origin lower-left, y up (PDF §2.2.2.2); IX numbering per samples (see 4.1); default units when U absent (PDF §2.5.8); ShapeRouteStyle values (VDev shaperoutestyle-cell-shape-layout-section: 0 page default, 1 right angle, 2 straight, 3-8 org chart/flowchart/tree, 16 centre to centre, 17-22 simple; ConLineRouteExt 0 default, 1 straight, 2 curved). MS-VSDX says ShapeRouteStyle is "used during formula evaluation only" (PDF §2.4.4.311), so it has no rendering effect in a file: draw the connector geometry yourself.

Still open (tried; no public page states it):

- O1 (item 3, arrow index): the index range is 0 (none) to 45, plus 254 = "use master" via the USE function (PDF §2.4.4.20; VDev beginarrow-cell, use-function). The glyph column of the table is a picture and is lost in the PDF and in the web page text, so the index for "filled triangle" is still unknown. Tried: PDF text, VDev BeginArrow/EndArrow pages in a browser, MS-VSDX BeginArrow page. Next: a round trip with our own generator and a viewer that is allowed by clean-room rules, or a published screenshot of the Line dialog. Arrow SIZE is resolved (0..4 in MS-VSDX, 0..6 in VDev).
- O2 (item 8): whether desktop Visio opens a package without StyleSheets, Theme or docProps. The spec marks them optional; no public page reports Visio's behaviour. Tried: PDF §2.3, VDev intro page.
- O3 (item 7): `.vssx` specifics (content type strings for stencil main parts, required parts) and Icon contents. MS-VSDX lists only drawing/macro-drawing content types (PDF §2.3.3.3) and has no mention of "vssx" or "stencil"; the intro page only lists the extensions. Tried: PDF full-text search, VDev intro, learn search for stencil pages.
- O4 (item 9): newline and escaping rules in `Text` (how a paragraph break is represented, `&#10;` vs `<pp>`). `xml:space="preserve"` is used by the samples; the paragraph/newline mapping is not stated. Tried: PDF §2.2.8, Text_Type.
- O5 (item 10): OPC relationship type URIs for core/app/custom properties and default content types. MS-VSDX says these are "Specified outside this document" (PDF §2.3.1); needs ECMA-376 Part 2 (not fetched here; out of the requested source list).
- O6: the exact formulas Visio writes for glued BeginX/EndX (`PAR(PNT(Sheet.n!Connections.X1,...))` form) and connection-point row formulas. Not shown on any public page read.
- O7: 1-D shape local-coordinate conventions for a connector (Width, Height, Angle, LocPin) beyond the VDev default formulas (Angle = ATAN2(EndY-BeginY,EndX-BeginX), Width = distance between endpoints, VDev angle and width cell pages). Height of a 1-D shape not confirmed.
- O8: master `Icon` content and `Inh` resolution details are not stated beyond the schema.

## 14. Source index

- [MS-VSDX] 2.3.1 parts: MSV/33056f18-c258-4a29-8026-03c58cbc8bca
- Document: MSV/7ec3d7b0-0de2-4711-a7b6-92daa2020d71; VisioDocument_Type: MSV/327dc311-1aa4-460e-ada6-9ac8c75e4c79
- Pages: MSV/947b485d-676a-480b-96e6-c0e4d1bf58f3; Page: MSV/1f15c8f0-6565-465c-aefd-2be6af545e8a; Page_Type: MSV/27e583c3-0ef3-4d7c-8fac-e37bd18d9dc2
- Masters: MSV/ac2cee21-ca0e-459b-85e3-35908a476f70; Master: MSV/10b28d2f-d32d-4b8c-96f5-dad13d32dd11; Master_Type: MSV/555d23f9-a1aa-401b-b176-80a9d0ed4109
- Theme: MSV/24711011-cb57-4f6d-8de8-5b95ac64f40a; Image: MSV/949e69ec-fcd6-4ca0-be82-c30421c1f9a0
- ShapeSheet_Type: MSV/5d6be8d6-1cab-4722-ba32-d73febc4e51d; PageSheet_Type: MSV/f81673b1-da84-4754-b19e-a0475d889120; Shapes_Type: MSV/4f0a2823-44fc-487d-a588-f234df1147bd
- Cell_Type: MSV/6f23bcc4-af93-4023-a380-3e78a228e166; Section_Type: MSV/735b599d-1359-4767-8593-1c508a885779; Row_Type: MSV/7ae7c325-097c-4225-adf5-e7b8b6695639; Text_Type: MSV/3031da58-e11e-460b-9df5-9cfb6bc0a836
- Coordinate system: MSV/99f006f5-c67e-4e29-8a19-134ae611fd8f; relative: MSV/6179f5b2-a7ea-40b9-b636-17a5d6c70bab
- Geometry: MSV/c6f4364f-5fb7-49f3-993e-49d4d709aa02; path: MSV/1b69f6b8-31d4-4b28-bc37-70ee4956d8cb; NoFill: MSV/6e12d295-b27d-4501-8838-af38f0f8f127
- PinX MSV/b68fb65a-6d93-4afa-86c3-94dcc6e13c5e; Width MSV/f8906ddd-ccbe-43e1-9655-285cb1afff23; LocPinX MSV/418310fd-e01e-4d63-b948-b0ff3eceeca3
- StyleSheet_Type: MSV/9136adc9-bd94-425e-a32d-15fee80707ef; formula grammar: MSV/e617d7e0-b0d9-4019-890c-cb4de0e3c6bf
- Connect_Type (VDev): `connect_type-complextypevisio-xml`; Connection row: `cell-element-connection-rowvisio-xml`; BeginX: `beginx-cell-1-d-endpoints-section`
- Line/Fill/Text sections: `line-format-section`, `fill-format-section`, `text-block-format-section`, `text-transform-section`, `shape-transform-section`; cells `linepattern-cell-line-format-section`, `linecolor-cell-line-format-section`, `beginarrow-cell-line-format-section`, `fillpattern-cell-fill-format-section`, `verticalalign-cell-text-block-format-section`
- Routing: `shaperoutestyle-cell-shape-layout-section`, `conlinerouteext-cell-shape-layout-section`
- Intro: `introduction-to-the-visio-file-formatvsdx`
