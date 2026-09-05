## Context

See `proposal.md` for motivation. The repo today is a Leptos CSR stub (`leptos` 0.7, `src/app.rs` shell with Profile / Experience / Skills / Projects / Blog Posts) plus a Pages workflow that uploads `docs/` without a WASM build. SpecKit artifacts under `specs/001-resume-visualization/` still assume Leptos, Spectrum 2, and Cytoscape as the primary graph. Constitution principle II names Leptos; this design uses GPUI WASM instead and does not amend the constitution in this change.

Visual reference: [Compendium](https://www.codionics.com/Compendium/). Web UI proof that gpui-kit runs in-browser: [gpui-kit gallery](https://gpui-kit.com/gallery/) via `crates/story-web` in [longbridge/gpui-kit](https://github.com/longbridge/gpui-kit).

## Goals / Non-Goals

**Goals:**

- One GPUI + gpui-kit UI tree for the visualizer, compiled to `wasm32-unknown-unknown` and served statically.
- Dock-based left/main split; header and footer outside or around the dock area.
- Rust-owned JSON Resume parse and derived payloads; UI only renders those payloads.
- Deterministic month math and graph construction covered by `cargo test` on native (and WASM where practical).

**Non-Goals:**

- Keeping Leptos as a parallel renderer.
- Shipping a first-class native installer in this change.
- Exact Blazor/Spectrum internals from Compendium.
- Live resume editing or multiple concurrent profiles.

## Decisions

### D1. Web host follows gpui-kit `story-web`, not Leptos CSR

**Choice:** Add a small JS/Vite (or equivalent) host that loads the rustume WASM the way `story-web` loads the gallery: wasm-bindgen, canvas/WebGPU, production `base` path for GitHub Pages.

**Why:** gpui-kit components are not DOM web components; they are GPUI elements. Mixing Leptos HTML with gpui-kit Dock is not supported. The gallery is the documented browser path.

**Alternatives:** Trunk-only GPUI web backends; Leptos wrapping a canvas — rejected as extra frameworks and contrary to dropping Leptos.

### D2. Single crate with `lib` + dual entry, plus `web/` host

**Choice:** Keep package name `rustume`. `src/lib.rs` holds models, transforms, and GPUI views. Native `src/main.rs` may remain a thin desktop launch for local iteration if gpui-kit allows it with low extra cost; WASM entry is compiled from the same lib (cfg or a `web` binary). Static host lives in `web/` (index.html, Vite, resume.json copy). `assets/resume.json` is the source resume checked into the repo.

**Why:** Matches current single-crate layout; Pages only needs the `web/` dist output.

**Alternatives:** Workspace with `rustume-core` / `rustume-ui` / `rustume-web` — deferred until the spike shows compile times or cfg noise that justify a split.

### D3. Shell = gpui-kit `Root` + header/footer + `DockArea`

**Choice:** `gpui_kit::init`; window content is `Root`. Vertical stack: header bar, `DockArea` filling remaining height, footer bar. Dock center is an `h_split` (or left `DockPlacement`) with a nav panel and a content panel. Nav items: Overview, Experience, Skills, Geography (country graph), plus Projects / Education when those JSON Resume arrays are non-empty. Content panel swaps on nav selection (entity state, not browser routes). Minimum pane widths enforced in panel size constraints.

**Why:** Meets the Dock splitter requirement and Compendium-like IA; gpui-kit Dock is the supported splitter.

**Alternatives:** Manual `div` flex + resize handles — more code, worse persistence. Browser CSS splitter — would leave GPUI.

### D4. JSON Resume models and month rules

**Choice:** `serde` structs aligned with [JSON Resume](https://jsonresume.org/schema): `Resume`, `Basics`, `Work` (`name`, `position`, `start_date`/`end_date` via `startDate`/`endDate` rename), `location` as `String` or object; `Skills` with `keywords`. Duration: inclusive calendar months from parsed year-month (day default 1) through end month (or now). Country: parse `work.location` string (last comma segment or country name) or `location.countryCode` if a nested object is present; else `Unknown`. Company aggregation: case-sensitive `work.name` as in the file (do not fuzzy-merge).

**Why:** Matches existing SpecKit data-model intent while staying schema-faithful.

**Alternatives:** jsonresume crate from crates.io — only if it matches schema version without fighting gpui-kit’s edition; otherwise hand-rolled structs are enough.

### D5. Experience bars use gpui-kit charts

**Choice:** Render `ExperienceBar` rows with gpui-kit’s chart primitives (bar), not a JS charting library.

**Why:** Stays in the GPUI tree; no extra JS for the required chart.

**Alternatives:** Custom painted bars; Chart.js — extra host complexity.

### D6. Graphs: spike gpui-flow, then Cytoscape fallback with Cy element structs

**Choice:** Implementation order:

1. Spike: depend on `gpui-flow` against **the same GPUI revision gpui-kit pins**, target `wasm32-unknown-unknown`, render a 3-node graph in the Dock content panel.
2. If the spike fails (GPUI git mismatch, no WASM, handle/event issues), do **not** force gpui-flow. Use Cytoscape.js in the JS host: WASM exports JSON of Cytoscape elements; host mounts a canvas/div overlay or reserved DOM sibling for the two graph views only.

**Cytoscape element model (Rust, serde):** match Cytoscape’s `elements` array:

- `CyElement { group: CyGroup, data: CyData, position: Option<CyPosition> }`
- `CyGroup`: `nodes` | `edges`
- `CyData` for nodes: `id`, `label`, optional `parent`, plus `kind` / `group` as data fields (`company` | `country` | `skill`)
- `CyData` for edges: `id`, `source`, `target`, optional `label`

Builders in Rust produce two graphs (geography, skills) as `Vec<CyElement>`. gpui-flow path maps the same logical nodes/edges to `FlowNode`/`FlowEdge` without duplicating tenure/country rules.

**Why:** User preference is gpui-flow; gpui-flow’s `Cargo.toml` currently tracks `zed-industries/zed` git GPUI and macOS example deps — likely incompatible with gpui-kit’s pinned GPUI. Cytoscape remains the proven graph for Compendium-like exploration and already fits a JS host.

**Alternatives:** Custom GPUI painter for graphs — high cost. Always Cytoscape — skip only if the spike succeeds.

### D7. GitHub Pages CI

**Choice:** Replace the placeholder job in `.github/workflows/pages.yml`: rustup `wasm32-unknown-unknown`, wasm-bindgen-cli, Node/Bun for `web/` production build, upload `web/dist` (or documented output dir). Set Vite `base` for project Pages (`/rustume/`) vs user Pages (`/`) via a workflow input or repo variable. If GPUI web requires COOP/COEP, document that **GitHub Pages cannot set those headers**; then either confirm gpui-kit gallery works without them on Pages (follow their exact flags) or add a short `404`/index note that visitors need a Chromium WebGPU build. Prefer copying whatever `story-web` `make build-prod` uses.

**Why:** Existing workflow already has Pages permissions; it only lacks a real build.

**Alternatives:** Cloudflare Pages for COOP/COEP — out of scope unless the spike proves GitHub Pages cannot boot GPUI.

### D8. Drop Leptos in the same change as the first GPUI window

**Choice:** Remove `leptos` from `Cargo.toml` when the gpui-kit hello-window + Dock compiles. Do not leave a dual UI.

**Why:** Confirmed product decision.

## Risks / Trade-offs

- [gpui-flow / gpui-kit GPUI mismatch] → Spike early; fallback Cytoscape with shared `CyElement` payloads.
- [WASM binary size / Pages limits] → Release LTO, wasm-opt if needed; keep sample resume small.
- [WebGPU / browser support] → Fallback message in `index.html` if WASM init throws; document Chrome/Edge.
- [COOP/COEP vs GitHub Pages] → Mirror gpui-kit gallery deployment; if isolation is required and Pages cannot provide it, record as apply-time blocker and keep static host otherwise identical.
- [Constitution still says Leptos] → Follow this design for implementation; constitution update is a later change.
- [Compendium pixel drift] → Use live Compendium for IA and density during apply; do not block on Spectrum tokens.

## Migration Plan

1. Land planning artifacts (this change). Apply implements crate swap, `web/` host, resume asset, Pages build.
2. Visitors of the existing Pages site (if any) see a new GPUI app after the first successful deploy; no data migration.
3. Rollback: revert the deploy commit / restore previous Pages artifact; Leptos tree is gone after apply, so rollback is git revert of the apply commits.
4. SpecKit folder `specs/001-resume-visualization/` is historical; do not keep implementing it.

## Open Questions

- Exact GitHub Pages `base` path (user vs project site) — set from the repo’s Pages configuration at apply time.
- Whether gpui-kit gallery’s WASM flags already avoid COOP/COEP — copy empirically during the host spike.
