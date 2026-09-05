## 1. GPUI Foundation

- [x] 1.1 Replace the Leptos dependency and entry points with a GPUI + gpui-kit crate configuration that supports native test compilation and the `wasm32-unknown-unknown` target; verify `cargo check` and the WASM dependency graph resolve without Leptos
- [ ] 1.2 Add the GPUI web bootstrap and static host skeleton modeled on gpui-kit `story-web`, including a production base-path setting and a human-readable boot failure message; verify the host can load the generated WASM entry in a local production build
- [x] 1.3 Add the sample `assets/resume.json` source document and make the web build copy it into the published asset directory; verify the file exists in the production artifact at the path consumed by the app

## 2. Resume Data And Analytics

- [x] 2.1 Implement serde JSON Resume models for basics, work, locations, skills, and the sections used by navigation; verify a valid sample document parses and invalid, missing, non-object, or malformed input returns a recoverable error instead of fabricated data
- [x] 2.2 Implement calendar-month parsing and inclusive whole-month duration rules, treating an open end date as the current calendar month; verify closed, ongoing, partial-date, and equal-tenure cases with deterministic unit tests
- [x] 2.3 Implement company tenure aggregation and country derivation, preserving companies with missing locations under `Unknown`; verify repeated company entries sum their durations and distinct company names remain separate
- [x] 2.4 Implement Rust-owned graph payload models and builders for geography and skills, including stable node IDs, company/country/skill groups, valid edge endpoints, shared-keyword relationships, and isolated skill nodes; verify graph unit tests cover known and unknown countries, related skills, and isolated skills

## 3. Visualizer Shell

- [x] 3.1 Build the GPUI application shell with header, footer, persistent left navigation, and main content pane using gpui-kit `Root` and Dock splitting; verify the first view shows all chrome and identifies the candidate or product from valid resume data
- [ ] 3.2 Add keyboard-accessible section selection and entity state for Overview, Experience, Skills, Geography, and conditional Projects/Education destinations; verify activating each destination replaces only the main pane while navigation remains visible
- [ ] 3.3 Configure Dock splitter minimum sizes and resizing behavior so sidebar and main pane remain usable; verify a resize changes relative widths without allowing either pane to collapse to zero

## 4. Resume Visualizations

- [ ] 4.1 Implement the experience view with gpui-kit chart primitives, one labeled bar per company, month counts, monotonic proportional lengths, and an explicit no-work-history empty state; verify unequal and equal month totals render the expected relative bar lengths
- [ ] 4.2 Run the gpui-flow compatibility spike against gpui-kit’s pinned GPUI revision and the WASM target using a three-node graph; verify the result compiles and renders in the web host or record the concrete incompatibility that selects the fallback
- [x] 4.3 Implement geography and skills graph views using gpui-flow when the spike succeeds, otherwise Cytoscape.js consuming only the Rust-generated graph payload; verify both views render companies, countries, skills, and isolated nodes without re-parsing resume JSON in JavaScript
- [x] 4.4 Add graph pan, zoom, and safe node selection behavior for the chosen renderer; verify interaction preserves topology and selecting a node does not crash the visualizer

## 5. Pages Delivery

- [ ] 5.1 Replace the placeholder Pages workflow with Rust WASM compilation, wasm-bindgen packaging, web dependency installation, production host build, and Pages artifact upload; verify the workflow contains a documented push-to-default-branch deployment path
- [ ] 5.2 Make the GitHub Pages base path configurable for project pages versus user pages, defaulting safely for an unconfigured remote, and document any required browser/WebGPU or COOP/COEP constraints; verify generated HTML and asset URLs match the selected base path
- [ ] 5.3 Add a supported-browser startup fallback for missing WASM/graphics support and document local build/run steps; verify an initialization failure displays an explanatory message rather than a blank canvas

## 6. Integration Validation

- [x] 6.1 Add or update native tests covering parsing, duration, aggregation, country fallback, and graph payload contracts; verify `cargo test` passes
- [ ] 6.2 Build the release web artifact with the documented commands and inspect the output for HTML, JavaScript, WASM, and resume JSON; verify the static output can be served without a backend and boots the shell with the sample resume
