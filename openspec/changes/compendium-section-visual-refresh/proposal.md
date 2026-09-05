## Why

The current GPUI content sections contain the correct resume data but present it as plain stacked text, while the supplied Compendium screenshots establish a much richer visual language and information hierarchy. This change will make Profile, Skills, Projects, and Blog Posts visibly match those references while keeping all rendering in GPUI Rust.

## What Changes

- Rework the Profile view into a Compendium-style hero panel with section eyebrow, large candidate name, headline pills, right-aligned contact/location block, summary, and supporting two-column cards.
- Rework Skills into a hero plus expandable skill rows with level badges, chevrons, and indented keyword rows matching the reference density.
- Rework Projects into a hero plus a responsive two-column grid of rounded project cards with linked names, highlight badges, descriptions, and preserved source URLs.
- Rework Blog Posts into a hero with a WordPress source link and a responsive two-column grid of compact linked post cards with publication dates.
- Align the four views with the screenshots' composition and hierarchy while preserving the current Rustume palette: dark green application header, sage navigation surfaces, cream page background, amber active states, coral accents, warm white cards, restrained shadows, rounded corners, icon-led navigation, and consistent spacing.
- Preserve the existing JSON Resume source data, GPUI-only browser rendering, keyboard navigation, external links, and empty-data behavior.

## Capabilities

### New Capabilities
- `compendium-profile-layout`: Screenshot-aligned Profile composition and supporting card layout.
- `compendium-skills-layout`: Screenshot-aligned expandable skills presentation.
- `compendium-projects-layout`: Screenshot-aligned project card grid.
- `compendium-blog-posts-layout`: Screenshot-aligned blog post hero and card grid.

### Modified Capabilities
- None. The existing content contracts remain valid; this change specifies their user-visible composition and visual behavior.

## Impact

- **GPUI UI**: Primarily `src/app.rs`, including shared visual helpers, section renderers, navigation icons, and interaction state.
- **Resume models**: No schema changes expected; existing profile, skill, project, publication, and link data are reused.
- **Assets**: The four reference screenshots under `docs/screenshots/` become visual acceptance references; no screenshot is shipped to users.
- **Web host**: No additional browser UI framework or content renderer; the minimal WASM loader remains unchanged.
- **Validation**: Add runtime screenshot checks at desktop and mobile sizes plus native/WASM compilation checks.
