## 1. Compact Bubble Presentation

- [x] 1.1 Confirm and, if needed, refine the shared compact bubble styling so skill levels, language fluency, and project highlight tags are smaller than primary profile bubbles, remain horizontally readable, and pass a focused `cargo test` or UI smoke check.
- [x] 1.2 Update the Projects view to use the compact bubble treatment for each project highlight while retaining the existing project link, description, and full-size profile highlight behavior; verify the rendered Projects view contains compact tags and unchanged project content.

## 2. Skills Graph Contract

- [x] 2.1 Verify or adjust skills graph construction so every case-insensitive distinct keyword is represented by one parent-qualified child node and one parent-child relationship, while isolated skills, shared-keyword relationships, and valid edge endpoints remain intact; verify with focused `skills_graph` unit tests.
- [x] 2.2 Add or strengthen regression tests for multiple sub-skills and duplicate keyword casing, asserting separate child nodes, normalized deduplication, and corresponding `sub-skill` edges; verify with `cargo test skills_graph`.

## 3. Integrated Validation

- [x] 3.1 Run the complete Rust test suite with `cargo test` and confirm the modified graph and resume-view behavior introduces no regressions.
- [x] 3.2 Run the application or its available UI smoke path and inspect Profile, Skills, and Projects at representative widths to confirm compact bubbles are horizontal and subordinate while each skill sub-skill appears separately in the visualization.
