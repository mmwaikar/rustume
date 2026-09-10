## Why

The 10-Sep-2026 Round 3 review found that the experience chart still renders all company bars with the same color and places an unhelpful country legend below the chart. It also found that links throughout the app, including blog and project links, do not consistently use the expected default blue styling.

## What Changes

- Preserve newest-first company ordering while ensuring each bar uses the color assigned to its country.
- Show a truthful country legend associated with the chart, positioned within the chart presentation rather than as an unrelated bottom block.
- Ensure the legend identifies unknown locations distinctly and matches the colors used by the bars.
- Apply the default blue link styling through the shared link helper so footer, blog, project, profile, and network links are consistent.

## Capabilities

### New Capabilities

- `experience-view`: Defines country-colored company experience bars with an accurate colocated legend.

### Modified Capabilities

- `visualizer-shell`: Requires consistent default blue styling for links throughout the visualizer.

## Impact

- Affects experience chart data/rendering and legend layout in `src/app.rs`.
- Affects the shared link helper and all link-bearing views in `src/app.rs`.
- May add focused tests for country-to-color mapping, legend placement data, and shared link styling.
- Does not change the resume JSON schema, URLs, navigation, or external dependencies.
