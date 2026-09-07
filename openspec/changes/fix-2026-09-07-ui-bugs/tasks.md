## 1. Confirm UI primitives and shell structure

- [x] 1.1 Inspect the pinned gpui-kit exports and examples for Dock, icon, scrolling, Bubble, and GroupBox controls, then record the concrete APIs used by the implementation and verify no new dependency is required
- [x] 1.2 Compose the existing Dock entity around the sidebar and main content pane, configure non-zero minimum pane sizes, and verify `cargo check` succeeds with the shell owning the split
- [x] 1.3 Add an independently scrollable, flexible right content pane while keeping header, footer, and sidebar persistent, and verify a long Profile view scrolls without moving the shell

## 2. Correct sidebar navigation

- [x] 2.1 Replace the current sidebar children with an ordered, data-driven list beginning with Profile and excluding Overview, while preserving conditional Projects and Blog Posts entries; verify rendered navigation order against populated and empty optional-data fixtures
- [x] 2.2 Replace text glyph prefixes with gpui-kit icon controls and shared accessible labels, and verify activating either an icon or label selects the expected section
- [x] 2.3 Verify Dock divider resizing at wide and narrow window sizes, confirming both sidebar and content remain visible and usable without zero-width collapse

## 3. Correct Profile presentation

- [x] 3.1 Replace the local Profile headline badge rendering with gpui-kit Bubble controls and verify every comma-separated headline claim is shown as a distinct control
- [x] 3.2 Consolidate name, headline, email, phone, and location into a responsive identity/contact layout, remove duplicated contact output, and verify summary text wraps without clipping at narrow widths
- [x] 3.3 Render Education, Publications, Network, and Languages in separate GroupBoxes, omit empty groups, and verify source fields and links remain present

## 4. Correct Blog Posts presentation

- [x] 4.1 Implement a pure publication-date formatter for supported ISO year-month/full-date values with `Mon Year` or `Month Year` output and an unchanged-value fallback; verify supported, partial, and malformed date cases with focused tests
- [x] 4.2 Update Blog Posts intro composition and sizing so the source sentence does not wrap unnecessarily when space permits, and verify the WordPress link remains usable
- [x] 4.3 Apply normalized dates to every WordPress publication card and verify the empty blog state remains clear without fabricated entries

## 5. Integration validation

- [x] 5.1 Run `cargo fmt --check`, focused unit tests, and native `cargo check`; verify no formatting, compilation, or date-formatting regressions
- [x] 5.2 Build the web artifact using the documented project commands and inspect wide/narrow views for Profile-first navigation, working icons, Dock resizing, right-pane scrolling, Profile groups, and Blog Posts dates
