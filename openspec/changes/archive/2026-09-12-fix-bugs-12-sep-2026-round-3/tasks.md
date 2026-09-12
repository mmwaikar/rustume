## 1. Tree State And Node Presentation

- [x] 1.1 Identify available folder-open, folder-closed, and leaf/skill icon assets and wire them into the existing icon helper; verify the assets resolve during native compilation and the WASM build.
- [x] 1.2 Initialize all root `TreeItem`s collapsed while preserving expand/collapse interaction; verify the initial tree state reports no expanded skill branches and expansion reveals only that parent’s children.
- [x] 1.3 Render state-aware parent/leaf icons and apply one-level indentation from each tree entry’s depth without changing node labels or selection semantics; verify a parent/child pair has distinct icons and the child row is indented.

## 2. Skills Layout

- [x] 2.1 Replace the fixed Skills tree height with a flex-growing or otherwise available-height layout inside the existing scrollable content pane; verify the tree occupies the remaining section area without an avoidable short scrollbar region.

## 3. Regression Coverage

- [x] 3.1 Extend Skills tree fixtures/tests to assert collapsed initial state, parent and child node identity, icon-selection inputs, and depth-preserving nested items; verify with focused Skills tests.
- [x] 3.2 Preserve case-insensitive sub-skill deduplication, isolated skills, shared-keyword relationships, and valid graph endpoints; verify with `cargo test skills_graph` and the focused tree tests.

## 4. Integrated Validation

- [x] 4.1 Run the complete Rust test suite with `cargo test` and confirm no unrelated resume-view regressions.
- [ ] 4.2 Run the WASM/Vite UI smoke build and inspect the Skills view at representative viewport sizes to confirm icons, one-level indentation, collapsed startup, expand/collapse behavior, and full-height layout.
