## Why

The site is deployed at `https://www.codionics.com/rustume/` and opens in narrow browser windows and mobile viewports, but the shell always renders a fixed 240px sidebar dock plus a right content pane. On narrow screens the sidebar consumes most of the width, leaving the content cramped and barely readable.

## What Changes

- Detect the viewport width at render time from the window bounds.
- Below a responsive breakpoint (768px), the primary sidebar dock collapses so the content pane gets the full window width by default.
- The sidebar stays available on narrow screens via the dock's existing toggle affordance, which a visitor can use to reopen it.
- At or above the breakpoint, the previous behavior is preserved: the sidebar is open at 240px and panes remain resizable.

## Capabilities

### New Capabilities

- none

### Modified Capabilities

- `visualizer-shell`: the shell must collapse the primary sidebar on narrow viewports (below 768px) while preserving the full sidebar at wider widths and keeping navigation reachable through the dock toggle.

## Impact

- `src/app.rs`: `App::render` must read the window width and apply the responsive dock state; `root_view` must mark the left dock collapsible so it honors the collapse at narrow widths.
- The `DockArea` entity (`gpui_base`'s `set_dock_collapsible`, `is_dock_open`, `toggle_dock`) drives the open/closed state; no new dependencies.
- No spec changes for individual views (blog, projects, profile, skills); they already wrap content. The geography graph's fixed pixel coordinates are explicitly out of scope.