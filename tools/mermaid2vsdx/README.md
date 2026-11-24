# mermaid2vsdx

Local tool (not part of any repo PR): converts a Mermaid flowchart to `.vsdx` plus an optional `.svg`
preview, chaining `craft-mermaid` (parse), `craft-graph-layout` (`layered`) and `craft-vsdx` (`write`)
as path dependencies. Layout units are points.

## Usage

    mermaid2vsdx input.mmd -o out.vsdx [--svg out.svg] [--direction TB|BT|LR|RL]
    cat in.mmd | mermaid2vsdx - -o out.vsdx

Warnings (unsupported shape substitutions, bad colours, parser warnings) go to stderr. Non-flowchart
diagrams and parse errors print `error: ...` and exit non-zero.

## Build (Docker, capped, serialised)

    /home/satal/code/craft-diagrams/tools/dcargo /home/satal/code/craft-diagrams/tools/mermaid2vsdx build --release

Binary: `target/release/mermaid2vsdx`.

## Render check with LibreOffice Draw

    cd samples
    ../target/release/mermaid2vsdx 1-simple.mmd -o 1-simple.vsdx --svg 1-simple.svg
    docker run --rm -v "$PWD":/d local-lo-draw soffice --headless --convert-to png --outdir /d /d/1-simple.vsdx

## Known limits

- Unsupported Mermaid shapes (cloud, notched, stacked, ...) are drawn as rectangles with a warning.
- `--o` / `--x` edge ends use the same arrowhead (only one arrow index is known).
- Edge to a subgraph attaches to the subgraph's first node. Per-subgraph `direction` is ignored.
- Connector text colour/font and subgraph title alignment are not in the vsdx model.
- LibreOffice rotates connector labels along vertical connectors.
