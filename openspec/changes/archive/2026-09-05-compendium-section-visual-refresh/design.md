## Context

See `proposal.md` and the four supplied references under `docs/screenshots/`. The current GPUI app has correct content models and section state, but `src/app.rs` renders Profile, Skills, Projects, and Blog Posts as simple stacked text. The existing browser host is only a WASM loader; the visual refresh must remain GPUI-owned.

## Goals / Non-Goals

**Goals:**

- Reproduce the screenshots' shared visual language across the four requested sections.
- Build reusable GPUI layout helpers for hero panels, bordered cards, badges, links, headings, and responsive grids.
- Preserve all existing resume data and interactions while improving hierarchy and scanability.
- Validate desktop and narrow viewport screenshots against the supplied references.

**Non-Goals:**

- Pixel-perfect recreation of Fluent UI or Blazor internals.
- Changes to JSON Resume parsing or analytics rules.
- Replacing GPUI with DOM/CSS section rendering.
- Adding live data fetching or new content.

## Decisions

### D1. Use a shared Compendium layout with the existing Rustume palette

Use the screenshot's geometry and hierarchy while retaining the current palette: dark green header/footer, sage sidebar surfaces, cream page background, amber active states, coral chart accents, warm white cards, dark text, modest shadows, rounded corners, and generous section spacing. Define these as Rust-side GPUI values/helpers rather than scattered literals where the API allows.

Alternative considered: adopt the screenshot's blue palette. Rejected because the user explicitly wants the current Rustume colors preserved.

### D2. Build each section from reusable GPUI composition helpers

Create helpers for hero panels, card panels, section eyebrows, metadata badges, link rows, and responsive grids. The four section renderers should differ in content, not structural styling.

Alternative considered: four independent render trees. Rejected because it would make screenshot tuning inconsistent and duplicate layout logic.

### D3. Model the screenshot interactions in entity state

Skills rows remain expandable through the existing selected-skill state, while navigation stays persistent. Projects, publications, network links, and blog links use GPUI Link elements. No browser route or JavaScript state is introduced.

### D4. Validate visual behavior at desktop and mobile sizes

Use the running WASM app and browser screenshots to check hero sizing, grid columns, wrapping, sidebar persistence, and card spacing. Native compilation and WASM compilation remain required gates.

## Risks / Trade-offs

- [GPUI layout primitives may not expose every CSS-like responsive behavior] -> Use flex wrapping and width constraints available in GPUI, with explicit narrow-layout checks.
- [Long real resume strings can distort screenshot proportions] -> Keep cards width-constrained, allow wrapping, and avoid fixed text widths.
- [Large screenshots encourage overfitting] -> Match composition, hierarchy, palette, spacing, and interactions while preserving accessible content.
- [Font and web renderer differences] -> Reuse the existing embedded IBM Plex Sans web font and validate in the actual browser build.

## Migration Plan

1. Add shared GPUI visual helpers and tokens.
2. Rebuild Profile, Skills, Projects, and Blog Posts views against the references.
3. Run native tests/checks and nightly WASM packaging.
4. Capture desktop/mobile screenshots and correct layout regressions.
5. Roll back by reverting the section renderer changes; resume models remain unchanged.
