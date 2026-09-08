## Why

The second 8-Sep-2026 review surfaced a smaller but still visible set of profile-shell defects introduced by the AI implementation: sidebar icons are not rendering clearly, the full sidebar pane is not painted with the expected background, and the Profile supporting content still has visual inconsistencies in spacing, grouping, and line formatting. These issues reduce scannability and make the resume look unfinished even though the underlying data is present.

## What Changes

- Restore the sidebar icon treatment so the selected destinations are readable and visually distinct.
- Keep the existing dock split behavior while making the whole sidebar pane use the configured green background.
- Normalize the Profile section by removing duplicated internal labels, arranging the four supporting groups in the requested row structure, and keeping responsive layout behavior.
- Replace the ad hoc education rendering with a cleaner data-grid-style layout.
- Keep network entries on a single visible line and render language fluency using the existing Bubble control.
- Clean up publication rendering so titles, publisher/date, and summary popovers align correctly and the date format follows the expected month-year convention.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `visualizer-shell`: Fix sidebar icon clarity and full-pane background treatment without changing the already-working dock resizing behavior.
- `compendium-profile-layout`: Rework supporting-group layout, education presentation, network row formatting, and language bubble rendering within the Profile section.
- `profile-view`: Correct publication title/date alignment and summary presentation while preserving the source data and WordPress filtering rules.

## Impact

- **Code:** Primarily `src/app.rs`, including sidebar rendering, panel styling, profile helper methods, and publication/education formatting.
- **Data:** No JSON Resume schema changes; the fix remains limited to presentation and layout decisions.
- **Dependencies:** Reuse the existing gpui-kit `Bubble`, `GroupBox`, `Link`, and popover primitives already in the app rather than introducing a new UI dependency.
- **Validation:** Verify the visual behavior at wide and narrow pane sizes and keep all fixes scoped to the second review items without broadening feature scope.
