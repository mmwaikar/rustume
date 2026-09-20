## Context

The shell in `src/app.rs` builds a header, a `DockArea` entity (`self.dock`), and a `StatusBar` footer. The left dock is registered in `root_view` with a fixed `px(240.0)` size. The `DockArea` (gpui-base 0.6.0) exposes `set_dock_collapsible`, `is_dock_open`, and `toggle_dock`; `Window::bounds()` (gpui-pre 0.3.5 window.rs:2681) provides the current viewport size, and gpui re-renders on window resize.

Today the dock is not marked collapsible (`set_dock_collapsible` is never called), so `toggle_dock` would refuse to close it. That is why there is currently no collapse affordance.

## Goals / Non-Goals

**Goals:**
- Collapse the left dock below 768px so content gets the full width by default.
- Let visitors reopen it on narrow screens via the dock's existing toggle button.
- Keep the current 240px open behavior at and above 768px.
- React to live resizes across the breakpoint without a reload.

**Non-Goals:**
- Mobile-specific navigation redesign (hamburger, off-canvas menu) — out of scope.
- Making individual view layouts responsive (they already wrap with `flex_wrap`/`min_w`).
- Adapting the geography graph's fixed pixel coordinates to narrow widths.
- Repositioning the dock toggle; we reuse gpui-kit's built-in affordance.

## Decisions

**D1: Read window width in `App::render` instead of a resize listener.**
gpui re-renders on window resize and passes `&mut Window` to `render`, so `window.bounds().size().width` is the current width with no extra wiring. A dedicated resize handler would only duplicate what the render loop already does.

**D2: Drive dock open/closed through `toggle_dock`, only on breakpoint crossings.**
Store the last-known narrow/wide state (a `bool` on `App`) so render only calls `toggle_dock` when the state actually flips, not every frame. Wide→narrow closes the dock; narrow→wide opens it. Because `toggle_dock` toggles, we guard with `is_dock_open()` before toggling so a visitor who manually reopened the dock on narrow stays in their state only until the next crossing — acceptable per the spec scenarios.

Alternative considered: always force-close/reopen based purely on width each frame. Rejected — it would fight the visitor's manual toggle and cause churn.

**D3: Mark the left dock collapsible once in `root_view`.**
`toggle_dock` refuses to close a non-collapsible dock. Adding `dock.set_dock_collapsible(DockPlacement::Left, true, window, cx)` in `root_view` (next to `set_dock_size`) makes collapsing legal while keeping the initial state unchanged. The built-in toggle button (`toggle_button_visible` default is `true`) then renders the affordance.

**D4: Single source of truth for the breakpoint.**
A `const NARROW_BREAKPOINT: f32 = 768.0` in `src/app.rs`. Both App state tracking and `root_view` read it indirectly (only render compares), so there is one constant to tune.

## Risks / Trade-offs

- [Visitor manually opens the dock on narrow, then resizes edges between 767/768px] → Toggling on crossing is idempotent and cheap; the shell settles into the width's default within one frame after the crossing stops.
- [Repeated resize events during dragging] → Guarded by the stored-state comparison in D2; each crossing triggers one toggle, so no thrash.
- [`.dock_size` returns `Option`] → Only relied on for reading; we never read it for the collapse decision, only the window width, so no unwrap hazard.
- [Web vs desktop width units] → `window.bounds()` is in logical pixels on both platforms; 768px matches the common CSS breakpoint used for the same page's desktop/file nav.