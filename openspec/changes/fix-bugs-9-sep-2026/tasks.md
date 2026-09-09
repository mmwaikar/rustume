## 1. Shared Shell, Theme, and Assets

- [x] 1.1 Confirm the installed `gpui-kit` 0.6.0 APIs and bundled asset names for StatusBar, TextSelection, icons, theme fonts, and window maximization; record the chosen APIs in the implementation without adding an unnecessary dependency, verified by a successful dependency/source lookup.
- [x] 1.2 Replace the placeholder sidebar icon geometry with GPUI Kit asset-backed icons and preserve the existing conditional navigation and active-state behavior, verified by native compilation and a visible icon for every rendered destination.
- [x] 1.3 Add the bundled Inter font resource and configure normal and monospaced theme roles during application startup, verified by native and WASM-target compilation without font-loading errors.
- [ ] 1.4 Apply TextSelection behavior to shared shell/content text and specialized chart, profile, and skills labels while preserving link and row interactions, verified by manual drag-copy checks in Header, Sidebar, Footer, Profile, Experience, and Skills.
- [ ] 1.5 Replace the custom footer with StatusBar-style chrome and set initial desktop window options to maximized, verified by launch behavior and by scrolling long content while the footer remains outside the content scroll region.

## 2. Skills Data and View

- [x] 2.1 Update `skills_graph` to emit deterministic child nodes and parent-child edges for every keyword while preserving standalone parent skills and valid existing relationship semantics, verified with unit tests for multi-keyword, empty-keyword, duplicate/case variants, and isolated skills.
- [x] 2.2 Update the Skills view to render each child as its own node/row and replace the oversized level indicator with a content-sized Bubble-compatible control, verified by a multi-keyword fixture showing separate selectable children and a compact level label.

## 3. Experience View

- [x] 3.1 Replace horizontal width-scaled experience bars with a fixed-column, bottom-aligned vertical chart based on `company_months`, using rounded bars and proportional heights with a readable month value, verified with varied-duration fixture data and a zero/empty-data state.
- [ ] 3.2 Add stable company label/value layout with diagonal treatment for long names and selectable chart text, verified at wide and narrow window sizes without label overlap or clipped company names.

## 4. Verification and Regression Coverage

- [x] 4.1 Add or update focused tests for date/month formatting, skills graph parent-child structure, and experience data-to-bar scaling, verified with `cargo test`.
- [x] 4.2 Run formatting and compilation checks for the supported native and web targets, verified with `cargo fmt --check` and the repository's existing build/WASM commands.
- [ ] 4.3 Perform a launched-app smoke check covering maximized startup, sidebar icon visibility, selectable text, persistent StatusBar footer, independent scrolling, vertical experience chart, and skills hierarchy at desktop and narrow sizes; record any environment-specific limitation in the implementation report.