## Why

Rustume needs a public resume visualizer that tells a career story the way [Compendium](https://www.codionics.com/Compendium/) does—structured shell, tenure charts, and relationship graphs—while keeping JSON Resume as the source of truth. The current Leptos/WASM scaffold and SpecKit plan do not match the chosen UI stack: [gpui-kit](https://github.com/longbridge/gpui-kit) compiled to the browser (as on [the gpui-kit gallery](https://gpui-kit.com/gallery/)) and hosted on GitHub Pages.

## What Changes

- **BREAKING**: Remove Leptos as the UI runtime. Rustume becomes a GPUI + gpui-kit application with a WASM web host. Native desktop remains out of scope for this change unless the same crate happens to build natively as a side effect.
- Load a static JSON Resume document ([schema](https://jsonresume.org/schema)) and shape it in Rust into chart and graph view models.
- Present a Compendium-inspired layout: header, footer, left sidebar, and main content, with gpui-kit **Dock** splitting sidebar and main.
- Show company tenure as a bar chart of **months**.
- Show experience-by-country and experience-by-skills as node–edge graphs. Prefer [gpui-flow](https://github.com/pacifio/gpui-flow); if it cannot pin to gpui-kit’s GPUI or cannot target WASM, use Cytoscape.js in the JS host with Rust structs that match Cytoscape element JSON.
- Replace the placeholder GitHub Pages workflow with a `story-web`-style WASM production build and deploy.

## Capabilities

### New Capabilities

- `json-resume`: Ingest, validate, and transform a JSON Resume file into typed Rust models and derived analytics (tenure months, country from work location, skill relationships).
- `visualizer-shell`: Application chrome (header, footer, Dock left sidebar vs main pane) and section navigation inspired by Compendium.
- `experience-chart`: Bar-chart view of professional experience duration in months, aggregated by company.
- `relationship-graph`: Node–edge visualizations for companies/countries and for skills, including the gpui-flow vs Cytoscape fallback contract.
- `pages-host`: Static GitHub Pages hosting of the GPUI WASM visualizer (build, headers/assets, sample resume file).

### Modified Capabilities

- None. `openspec/specs/` has no main specs yet. Existing SpecKit docs under `specs/001-resume-visualization/` remain historical and are superseded by this change’s stack; they are not OpenSpec main specs.

## Impact

- **Code**: Replace `leptos` CSR entry (`src/main.rs`, `src/app.rs`, `src/lib.rs`) with a gpui-kit `Root` + Dock workspace and a WASM bootstrap modeled on gpui-kit `story-web`.
- **Data**: Add `assets/resume.json` (sample JSON Resume) until a real resume is substituted.
- **Dependencies**: Drop Leptos/wasm-bindgen-as-UI; add `gpui-kit` (pins GPUI), optional `gpui-flow` after a compatibility spike, `serde`/`serde_json` retained; JS host may add Cytoscape only on fallback.
- **CI**: Rewrite `.github/workflows/pages.yml` to compile `wasm32-unknown-unknown`, package the web host, and deploy static output.
- **Constitution / SpecKit**: Constitution still names Leptos/WASM; this change’s behavior is GPUI WASM. Updating the constitution is out of scope unless required for apply; call out the conflict in design.
- **Non-goals**: Resume editing, multi-profile switching, pixel-perfect Compendium cloning of Adobe Spectrum/Blazor internals, and a separate Leptos site.
