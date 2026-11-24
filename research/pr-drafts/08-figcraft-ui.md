<!-- LOCAL DRAFT. Not posted. Waits for 07. -->

## Title

boards: import Mermaid from a dialog, by paste and tidy from the menu

## Description

On a board, File has "Import Mermaid flowchart..." and Arrange has "Tidy diagram"; pasting flowchart text imports it.

Part of #<umbrella>. Waits for #<07>.

**What the user saw:** the engine commands from #<07> had no way in for people who do not use the CLI or MCP.

**Cause:** new capability.

**What changed**

- A dialog (`menu_input.rs`) with a text field, a hint, Ctrl+Enter to import, and a refused flowchart keeps the dialog and the text.
- Menu entries and Actions palette entries from `catalog.rs`; on a design file the entry says it is for boards.
- Paste: text that starts with `flowchart` or `graph`, pasted on a board, imports a diagram; elsewhere it stays text. **This changes paste behaviour on boards.** It sits in this pull request so it can be dropped without touching the engine commands. <!-- fill: decision d3 / a3, ship or drop -->
- `en.toml`: three strings (English only). Result toast lists warnings.
- Unchanged: everything else in paste, the clipboard menu, other locales.

**Names:** "Mermaid" is a language name (naming.md rule 3); the labels describe what the command does.

**Screenshot:** <!-- fill: screenshot of the dialog, attached in this description only; not committed -->

## Accessibility

- Pattern: modal dialog with one multi-line text field and two buttons (import, cancel).
- Keyboard: Ctrl+Enter submits, Esc closes and returns focus, Tab leaves the text field (no trap). The dialog is labelled by its hint and takes focus with `request_focus`.
- Tests (`crates/ui-egui/src/mermaid_ui_tests.rs`): `the_import_dialog_takes_a_flowchart_by_keyboard`, `a_refused_flowchart_keeps_the_dialog_and_its_text`, `on_a_design_file_the_menu_entry_says_it_is_for_boards`, `pasting_a_flowchart_on_a_board_imports_it_and_elsewhere_it_stays_text`, `tab_leaves_the_flowchart_field`.
- Not yet covered: <!-- fill: kittest check of the accessibility tree (roles, names, focus after each step); whether the import result toast is announced through a live region (accessibility.md rule 4). Add before posting, or say they cannot be written -->
- No screen reader was run. The dialog uses standard controls, so no `needs screen reader test` label is requested. <!-- fill: confirm -->
- Colour is not the only signal; no new colours. Nothing in the flow needs dragging.

## Tests and checks

- Fails with the change reverted: <!-- fill -->
- Ran: <!-- fill: fmt, clippy -D warnings, cargo test incl. i18n key test both ways, cargo xtask ci; counts -->
- Not run: <!-- fill -->

## Departures from SHOULD rules

- Paste behaviour change bundled here; see above.
- Accessibility tree test and announcement: see the fill marker above.

## Cut from

Repo `figcraft`, branch `boards-diagram`, commit `137b7f97` (8 files, +194 -3). The working tree has uncommitted edits to `clipboard.rs` and `mermaid_ui_tests.rs` (+13 test lines) that belong here; commit them first. Add the UI rows of `57a1136d` if any remain.
