## Why

The 8-Sep-2026 profile review found several presentation defects in the GPUI shell and supporting profile panels: dock names are visible as redundant labels, the sidebar background does not fill its pane, and the four supporting sections have duplicated headings and an inefficient layout. These issues make the profile harder to scan and obscure the distinction between navigation chrome and resume content.

## What Changes

- Hide dock panel identity labels from the user-facing shell while retaining the dock panel registrations required for resizing.
- Make the sidebar background fill its complete dock pane and keep the existing adjustable dock behavior.
- Remove redundant inner headings from Education, Publications, Network, and Languages GroupBoxes.
- Arrange supporting groups as Education/Publications on the first row and Network/Languages on the second row when space allows, with responsive stacking on narrow panes.
- Render each network name and linked value on one line, and render language fluency with the existing `Bubble` control.
- Replace the Education free-form entries with a readable data-grid-style table.
- Filter WordPress blog publications out of the Publications group.
- Make each remaining publication title the source link and expose its description through a popover instead of a permanently rendered summary and separate action link.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `visualizer-shell`: Remove redundant dock labels and ensure the full resizable sidebar pane is visually filled.
- `compendium-profile-layout`: Define the two-row supporting-group arrangement, single-heading GroupBoxes, education grid, inline network rows, and language fluency bubbles.
- `profile-view`: Define WordPress filtering for publications, linked publication titles, and popover descriptions while preserving source data behavior.

## Impact

- **Code:** Primarily `src/app.rs`, including dock panel presentation, sidebar rendering, profile group layout, education/network/language/publication views, and small helper functions.
- **Data:** No JSON Resume schema or asset changes; WordPress entries continue to be identified by the existing `wordpress_publications` helper.
- **Dependencies:** Prefer existing gpui-kit `GroupBox`, `Bubble`, `Link`, and popover APIs. Add no dependency unless the installed gpui-kit version lacks the required popover primitive.
- **Validation:** Extend focused Rust unit/render checks where practical and manually inspect the native or web UI at wide and narrow pane sizes for grid order, wrapping, dock fill, and popover behavior.
