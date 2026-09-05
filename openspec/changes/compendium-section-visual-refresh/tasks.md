## 1. Shared Visual Language

- [x] 1.1 Define reusable GPUI visual tokens and helpers for the screenshot composition while preserving the current dark-green, sage, cream, amber, coral, and warm-white palette; verify native `cargo check` passes and helpers are used by at least one refreshed section
- [x] 1.2 Add icon-led sidebar presentation and active-section styling without changing navigation destinations or keyboard activation; verify the persistent shell remains visible across refreshed views
- [x] 1.3 Add responsive GPUI layout helpers for wide two-column grids and narrow one-column stacking; verify layout constraints prevent horizontal clipping at desktop and mobile viewport sizes

## 2. Profile Refresh

- [x] 2.1 Recompose Profile into the screenshot-aligned hero with eyebrow, large candidate name, headline pills, right-side contact/location block, and summary; verify all actual basics values remain visible and the hero matches the supplied profile reference
- [x] 2.2 Recompose Education, Publications, Network, and Languages into shared bordered cards with headings, readable metadata, badges, and GPUI links; verify populated supporting data is preserved and cards form a responsive two-column layout
- [x] 2.3 Capture desktop and mobile Profile screenshots and compare hero hierarchy, card arrangement, wrapping, and sidebar persistence against `docs/screenshots/profile.png`

## 3. Skills Refresh

- [x] 3.1 Add the screenshot-aligned Skills hero with `Technical strengths` heading and supporting copy; verify the hero uses the shared current-palette treatment
- [x] 3.2 Rebuild skill rows with disclosure affordances, level badges, compact spacing, and indented keyword rows; verify expanding and collapsing a real skill preserves all keywords and isolated skills remain visible
- [x] 3.3 Capture desktop and mobile Skills screenshots and compare row density, badge placement, keyword indentation, and wrapping against `docs/screenshots/skills.png`

## 4. Projects Refresh

- [x] 4.1 Add the screenshot-aligned Projects hero with `Selected works` heading and supporting copy; verify empty project data still produces the specified empty state
- [x] 4.2 Rebuild project items as a responsive two-column card grid with linked names, highlight badges, descriptions, and preserved source links; verify all actual projects render once without clipped long descriptions
- [x] 4.3 Capture desktop and mobile Projects screenshots and compare card grid columns, hero spacing, badges, links, and responsive stacking against `docs/screenshots/projects.png`

## 5. Blog Posts Refresh

- [x] 5.1 Add the screenshot-aligned Blog Posts hero with the supplied heading, supporting copy, and WordPress source link; verify no remote feed is introduced
- [x] 5.2 Rebuild WordPress publications as compact two-column linked cards with title and release date; verify every actual WordPress publication appears once and empty data remains coherent
- [x] 5.3 Capture desktop and mobile Blog Posts screenshots and compare hero composition, card density, dates, links, and responsive stacking against `docs/screenshots/blog-posts.png`

## 6. Integration Validation

- [x] 6.1 Run `cargo fmt --check`, `cargo test`, and `cargo check`; verify existing resume analytics and content tests remain green
- [x] 6.2 Run `cargo +nightly check --lib --target wasm32-unknown-unknown`, regenerate wasm-bindgen output, and run `npm run build`; verify the browser artifact remains GPUI-owned with no section renderer added to JavaScript
- [x] 6.3 Serve the production browser artifact and inspect all four refreshed views at desktop and mobile sizes; verify no black canvas, font panic, overflow, or overlapping content remains
