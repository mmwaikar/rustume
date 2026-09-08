## 1. Confirm UI primitives and shell behavior

- [x] 1.1 Inspect the pinned gpui-kit source/examples for dock panel title visibility, full-height pane styling, and the supported popover/disclosure API; record the concrete APIs selected and verify they compile in the current dependency graph.
- [x] 1.2 Update the dock/sidebar render boundary to retain resizable panel registration while suppressing standalone `sidebar`/`content` labels and painting the sidebar background across the full pane; verify the existing navigation, pane resizing, and content scrolling behavior remains available.

## 2. Rebuild Profile supporting layout

- [x] 2.1 Refactor Profile supporting-group composition into explicit Education/Publications and Network/Languages rows with responsive minimum widths and no empty groups; verify the four populated groups appear in the requested order at wide width and stack without clipping at narrow width.
- [x] 2.2 Remove inner duplicate headings from the Education, Publications, Network, and Languages renderers while retaining the GroupBox titles; verify each group has exactly one visible heading.
- [x] 2.3 Replace free-form education paragraphs with aligned data-grid-style headers and rows for institution, study details, date range, and area; verify multiple entries preserve all populated source values and missing fields do not fabricate text.
- [x] 2.4 Render each network label and linked value in one flex row and render language fluency with the existing Bubble control; verify links remain usable and rows wrap/read correctly in a narrow pane.

## 3. Filter and present publications

- [x] 3.1 Add a pure publication-selection path that excludes WordPress blog publications using the existing classification semantics and omits the Publications GroupBox when no ordinary publications remain; verify mixed, ordinary-only, and blog-only inputs.
- [x] 3.2 Render ordinary publication titles as links when URLs exist, remove the separate `Open publication` action, and attach the pinned gpui-kit popover/disclosure only for non-empty summaries; verify URL-less and summary-less entries have no broken link or empty affordance.

## 4. Validate the profile bug fix

- [x] 4.1 Add focused native tests for publication filtering and any pure formatting/selection helpers introduced by the refactor; verify they pass with `cargo test`.
- [x] 4.2 Run `cargo fmt --check` and `cargo test`, then perform a visual smoke check of the profile at wide and narrow dock sizes; verify no duplicate shell/group labels, full sidebar fill, requested row order, education grid, inline network values, language bubbles, filtered publications, and working description popovers.
