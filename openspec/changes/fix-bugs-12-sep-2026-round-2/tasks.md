## 1. Locate And Expose Skill Nodes

- [x] 1.1 Trace the Skills view from `skills_graph` to its visible tree/node presentation and identify the aggregation point that collapses multiple sub-skills; verify the chosen boundary with a representative skill containing at least two keywords.
- [x] 1.2 Update the Skills presentation to render one parent node and one separate child node per distinct normalized sub-skill using `gpui_kit::component::tree::tree`, preserving each label and parent-child relationship; verify the nested `TreeItem` count and labels for a multi-keyword skill.

## 2. Regression Coverage

- [x] 2.1 Add or strengthen a skills graph/presentation regression fixture with one parent skill and multiple sub-skills, asserting that each sub-skill has its own visible node and relationship rather than an aggregated label; verify with the focused skills test.
- [x] 2.2 Preserve case-insensitive deduplication, isolated skills, shared-keyword relationships, and valid edge endpoints; verify with the existing and new `cargo test skills_graph` coverage.

## 3. Integrated Validation

- [x] 3.1 Run the complete Rust test suite with `cargo test` and confirm no unrelated resume-view regressions.
- [ ] 3.2 Run the available WASM/Vite UI smoke build and inspect the Skills view to confirm a skill with multiple sub-skills shows one parent plus one node per sub-skill at representative viewport widths.
