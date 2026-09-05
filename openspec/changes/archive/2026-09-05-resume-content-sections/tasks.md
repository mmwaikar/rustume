## 1. Resume Model Expansion

- [x] 1.1 Extend the typed JSON Resume model for nested basics location, profiles, education, publications, languages, interests, and full project fields; verify the checked-in `assets/resume.json` parses with populated values for each supported section
- [x] 1.2 Add derived selectors for available navigation destinations and WordPress publications, using case-insensitive publisher matching while preserving source labels and URLs; verify empty arrays omit their destinations and WordPress entries are selected without network access
- [x] 1.3 Add native model tests for profile links, education, publications, languages, interests, projects, and WordPress blog derivation; verify malformed or missing optional fields remain recoverable

## 2. Profile And Supporting Sections

- [x] 2.1 Add Profile content rendering for identity, headline, summary, location, contact details, and network links; verify all populated basics fields appear as usable labeled content
- [x] 2.2 Add Profile supporting groups for education, publications, languages, and interests with source dates, summaries, highlights, and links; verify empty subsections are omitted and populated subsections do not invent values
- [x] 2.3 Add conditional navigation entries for Profile, Education, Publications, Languages, and Network while preserving persistent header, sidebar, main pane, and footer; verify activating each entry replaces only main content

## 3. Skills And Projects

- [x] 3.1 Expand the Skills view to show every skill’s level and complete keywords while retaining relationship summaries and isolated skills; verify the actual resume’s skill groups and keywords are readable
- [x] 3.2 Add skill selection/detail state that reveals relationship context without losing application navigation; verify keyboard activation and repeated selection do not crash or reset the shell
- [x] 3.3 Add the Projects view with one GPUI item per project, descriptions, highlights, and preserved external URLs; verify all actual project entries render with their source links
- [x] 3.4 Add conditional Projects navigation and an explicit empty-project behavior; verify no project cards or links appear when the source array is empty

## 4. Blog Posts

- [x] 4.1 Add the Blog Posts view from WordPress publication entries with title, date, summary, and source link; verify the actual WordPress entries appear without any remote request
- [x] 4.2 Add the WordPress profile/blog source link when present and conditional Blog Posts navigation; verify no-blog data produces an omitted destination or clear empty state without fabricated posts

## 5. Validation And Delivery

- [x] 5.1 Keep the browser host limited to WASM loading and boot errors, with all new content rendered by GPUI Rust; verify no content rendering or extra UI dependency is added to `web/src/main.js` or `web/package.json`
- [x] 5.2 Run `cargo fmt --check`, `cargo test`, and `cargo check` against the expanded model and GPUI views; verify all existing analytics tests and new content tests pass
- [x] 5.3 Run `cargo +nightly check --lib --target wasm32-unknown-unknown` and the documented release WASM packaging plus `npm run build`; verify the browser artifact contains the generated GPUI WASM module and resume asset
