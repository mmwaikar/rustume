## 1. Experience Color Mapping

- [x] 1.1 Refactor the experience presentation data so country-to-color assignments are deterministic, shared by every bar and legend item, and include an explicit `Unknown` mapping when needed; verify with focused data tests covering multiple countries and missing locations.
- [x] 1.2 Update the BarChart fill path so each company bar uses its assigned country color while preserving newest-first ordering and duration scaling; verify focused tests assert ordering and distinct country colors.
- [x] 1.3 Move the country legend into the chart frame and render only countries represented by plotted entries, with matching swatches including `Unknown`; verify the rendered structure or a focused legend-data test confirms colocated content and complete mappings.

## 2. Shared Link Styling

- [x] 2.1 Apply the default blue link style at the shared link helper and retain equivalent styling for separately composed footer links; verify all link-bearing views reuse the same color decision without changing their existing URLs.
- [x] 2.2 Add focused assertions for link color constants/helper configuration and verify footer, profile, network, project, publication, and blog link paths remain independently activatable.

## 3. Validation

- [x] 3.1 Run `cargo fmt --check` and the full library test suite, fixing formatting or regressions in the touched paths.
- [x] 3.2 Run the native/wasm and Vite build checks, then perform browser smoke checks at wide and narrow viewports for country-colored bars, in-frame legend placement, unknown-country mapping, and blue links; record any environment-only canvas limitations. Browser startup and WebGL initialization passed; detailed visual assertions remain limited because GPUI renders to canvas and emits environment-specific wasm asset-loader warnings.
