## Why

The 10-Sep-2026 review found that the experience chart is difficult to read because long company names compete with vertically stacked bars. The footer also presents inaccurate, non-linked attribution, so both pieces of presentation need a focused visualizer update before implementation continues.

## What Changes

- Keep the GPUI Kit chart component for experience data while using a readable layout for long company names, with horizontal bars or in-bar/side-by-side labels where vertical labels do not fit.
- Preserve proportional company-duration comparison and rounded bar styling in the selected chart layout.
- Replace the footer copy with a heart-marked attribution linking to Rust and gpui-kit.
- Preserve the existing `StatusBar` footer control and selectable text behavior.

## Capabilities

### New Capabilities

- `experience-view`: Presents company experience durations in a readable chart layout that adapts labels and retains proportional comparison.

### Modified Capabilities

- `visualizer-shell`: Requires the footer attribution to identify and link Rust and gpui-kit while remaining in the shell status bar.

## Impact

- Affects the experience rendering and shared footer in `src/app.rs`.
- May require focused rendering or helper tests for chart layout decisions and footer link targets.
- Does not change the resume JSON contract, duration calculations, dependencies, or navigation structure.
