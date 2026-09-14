## 1. Country graph model and layout

- [x] 1.1 Add unit-tested projection state that retains country nodes and derives visible company nodes and edges from the set of expanded country identifiers; verify multi-country and unknown-country fixtures produce the specified nodes and relationships.
- [x] 1.2 Add a focused, responsive layout model for the two-level country/company graph using only Rustume-owned code; verify wide and narrow layout tests keep country and company labels readable.

## 2. GPUI Kit graph interaction

- [x] 2.1 Replace the Geography text summary with styled GPUI Kit country and company node elements plus their connecting visual elements; verify selecting Experience by Country displays a distinct node-and-edge graph rather than textual graph counts or labels.
- [x] 2.2 Wire accessible country-node controls to expand and collapse company nodes and their connections without mutating resume data; verify focused interaction tests cover both transitions and the rendered control indicates its state.
- [x] 2.3 Add an explicit empty state and responsive scrolling for unavailable or dense country data; verify an empty resume and a representative multi-country resume remain readable.

## 3. Browser delivery

- [x] 3.1 Update the GitHub Pages workflow to build the release Wasm library, generate the wasm-bindgen wrapper, run the Vite build, and publish `web/dist`; verify the workflow artifact contains the HTML, JavaScript, and Wasm application bundle.
- [x] 3.2 Add a deployed-artifact browser startup and interaction check using GitHub Pages-compatible response behavior; verify the country graph loads and a country can be expanded without browser security configuration.

## 4. Validation and documentation

- [x] 4.1 Run Rust unit tests and the native build after integration; verify `cargo test` and the production desktop build succeed.
- [x] 4.2 Run the release Wasm and Vite build, then exercise the built browser artifact; verify `just cargo-build`, `just gen-wasm`, and `just web-build` complete and the graph interactions work.
- [x] 4.3 Document that the country graph uses the existing GPUI Kit stack without an external graph-rendering dependency and describe the supported GitHub Pages deployment path; verify the documentation matches the validated implementation outcome.