## Context

The experience view is rendered in `src/app.rs` from the duration map produced by the resume domain helpers. The current chart already scales bars from a shared maximum, but its company names are placed in the same vertical column flow as the bars, which is fragile for long names. The shell already uses a GPUI Kit `StatusBar`; its right side currently contains plain footer text rather than linked attribution segments.

## Goals / Non-Goals

**Goals:**

- Keep chart rendering owned by the existing experience view and preserve duration calculations.
- Select a label arrangement based on available width and label length so company names remain readable.
- Keep bars rounded, comparable, and anchored to a common baseline.
- Compose the footer from selectable text, a heart mark, and two GPUI Kit links with the exact requested destinations.
- Add focused checks for chart sizing/label decisions and footer link configuration where the existing test surface permits.

**Non-Goals:**

- No changes to resume JSON parsing, company-duration calculations, navigation, or other sections.
- No new chart library or dependency.
- No change to the StatusBar's persistent shell placement.

## Decisions

1. **Retain the existing chart component and data source.** The chart remains driven by the existing company-duration map and common maximum. This preserves the semantics already used by the view while allowing its presentation geometry to change.

2. **Prefer readable label placement over forced vertical labels.** Use stable chart columns and a layout decision based on label length and available width. When labels cannot fit below vertical bars, render a horizontal row with labels adjacent to their bars or place labels inside/alongside the bar row. A rotated label is acceptable only when it fits without overlap; it is not a requirement.

3. **Build footer attribution from separate link elements.** Keep the `StatusBar` as the shell footer, but replace the single copy string with a composed element containing the heart marker, selectable text, and one link for each technology. Separate links are required so each destination remains independently activatable and testable.

4. **Use the exact official destinations from the bug report.** Rust links to `https://rust-lang.org/` and gpui-kit links to `https://gpui-kit.com`; no redirect or local proxy is introduced.

## Risks / Trade-offs

- [Risk] A narrow window may still have insufficient space for labels and bars together. -> Keep the chart horizontally scrollable or use the compact side-by-side arrangement so content remains accessible without overlap.
- [Risk] GPUI Kit link composition may differ from the current StatusBar child API. -> Reuse the existing `Link` and selectable-text helpers and isolate footer composition in the shell renderer.
- [Risk] Visual layout behavior is difficult to assert with unit tests alone. -> Test deterministic layout thresholds and URL values, then validate the rendered shell at representative wide and narrow sizes during implementation.

## Migration Plan

1. Update the experience renderer and footer composition in the existing app shell.
2. Add or adjust focused tests for proportional bar sizing, label-mode selection, and footer URLs.
3. Run formatting, Rust tests, and the available native/web build checks.
4. Manually verify long company names, mixed-duration bars, empty experience, and both footer links at wide and narrow window sizes.
5. Roll back by reverting the implementation and test changes; resume data remains backward compatible.
