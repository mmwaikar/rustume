## 1. Confirm the root cause in the shell and profile renderers

- [x] 1.1 Inspect the current `src/app.rs` sidebar and profile helpers to confirm the exact root causes behind the sidebar icon visibility and profile-group layout bugs, and verify the expected shell behavior against the existing spec requirements.
- [x] 1.2 Confirm the concrete gpui-kit API used for `Bubble`, `GroupBox`, and summary popovers so the planned fix matches the dependency version already in the project.

## 2. Fix the resizable shell and sidebar appearance

- [x] 2.1 Keep the dock registration and pane split intact while removing the redundant `sidebar`/`content` labels from the visible shell and verify the sidebar still fills the full pane.
- [x] 2.2 Restore the sidebar icon and text readability using the existing icon/text primitives and confirm the active navigation remains visible after the fix.

## 3. Rebuild the Profile supporting layout

- [x] 3.1 Restructure the four supporting groups into the requested Education/Publications row and Network/Languages row with responsive stacking when the pane narrows.
- [x] 3.2 Remove duplicate internal labels from each GroupBox while preserving one clear group title per section.
- [x] 3.3 Rewrite the education renderer into a data-grid-style layout with aligned header and entry rows and verify the values remain source-derived rather than fabricated.
- [x] 3.4 Render network entries as single-line label + link rows and render language fluency with the existing Bubble control.

## 4. Correct publication formatting and summary behavior

- [x] 4.1 Keep the WordPress exclusion logic and ensure the Publications group omits blog entries while leaving ordinary publications intact.
- [x] 4.2 Make the publication title itself the link when a URL exists, remove the extra action link pattern, and keep the summary hidden behind a popover only when a description is present.
- [x] 4.3 Normalize publication date formatting to a human-readable month-year pattern and align the metadata into a single visually consistent line.

## 5. Validate the fix with focused tests and visual checks

- [x] 5.1 Run the relevant Rust tests for publication filtering, date formatting, and any helper used by the layout refactor, and verify the suite passes.
- [x] 5.2 Run formatting and a focused app build or smoke check, then inspect the sidebar and Profile at both wide and narrow widths to confirm the second-round bug list is resolved without regressions.
