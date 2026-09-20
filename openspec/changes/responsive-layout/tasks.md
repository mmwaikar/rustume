## 1. Make the left dock collapsible

- [x] 1.1 In `src/app.rs` `root_view`, call `dock.set_dock_collapsible(DockPlacement::Left, true, window, cx)` next to the existing `set_dock_size` and verify `cargo build --lib` compiles
- [x] 1.2 Confirm the built-in dock toggle button is visible for the left dock: `is_dock_collapsible` gates the button in gpui-kit `tab_panel.rs:221`, and it is now `true` for the left dock. `<src/app.rs>` builds and `target\debug\rustume.exe` launches without panic; sidebar is manually collapsible via the toggle at any width

## 2. Collapse sidebar on narrow viewports

- [x] 2.1 Add a `const NARROW_BREAKPOINT: f32 = 768.0` and a `narrow_viewport: bool` field initialized to false on `App` in `src/app.rs`; verify `cargo build --lib` compiles
- [x] 2.2 In `App::render`, read `window.bounds().size().width` and when it crosses the breakpoint flip `narrow_viewport`, calling `toggle_dock` on `self.dock` (closing when crossing below, opening when crossing above, guarded by `is_dock_open`); verify `cargo build --lib` compiles
- [ ] 2.3 Run `cargo run --bin rustume` and verify: at ≥768px the sidebar is open at 240px; resizing below 768px collapses it; resizing back above reopens it; on the narrow view the dock toggle reopens the sidebar

## 3. Verify web build

- [x] 3.1 Build the web target (`cargo build --lib --target wasm32-unknown-unknown --release` + `wasm-bindgen --target web --out-dir web/wasm`) and verify it compiles after the render changes
- [ ] 3.2 Serve `web/dist` with `RUSTUME_BASE_PATH=/` and verify in a browser at a narrow viewport width (780 px devtools) that the sidebar starts collapsed, content spans full width, and the toggle reopens the sidebar