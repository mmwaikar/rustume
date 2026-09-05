## Why

The actual resume now contains the candidate's profile, detailed skills, projects, publications, education, languages, interests, and network links, but the GPUI visualizer only exposes a summary, experience, skills summary, and geography summary. The app needs the same content-forward section structure as Compendium so visitors can explore the resume rather than seeing a partial data projection.

## What Changes

- Add a Profile destination that presents identity, headline, summary, contact links, location, education, publications, languages, interests, and network profiles from the loaded JSON Resume.
- Expand Skills into a navigable, grouped view with skill level, keywords, and relationships while preserving isolated skills.
- Add a Projects destination with project descriptions, highlights, and external links.
- Add a Blog Posts destination derived from JSON Resume publication entries whose publisher is WordPress; link entries to their source URLs and show the WordPress profile link when available.
- Add conditional sidebar destinations for Projects, Blog Posts, Education, Publications, Languages, and Network based on available resume data, following Compendium's content hierarchy.
- Keep all content rendering in GPUI/gpui-kit Rust views; retain the browser JavaScript host only for loading the generated WASM module and displaying boot errors.

## Capabilities

### New Capabilities
- `profile-view`: Candidate identity, summary, contact, education, publications, languages, interests, and social/network presentation.
- `skills-view`: Detailed skill groups with levels, keywords, and relationship-aware navigation.
- `projects-view`: Resume project cards/list with descriptions, highlights, and source links.
- `blog-posts-view`: WordPress publication-derived blog post listing with source links and fallback profile access.

### Modified Capabilities
- None. The existing OpenSpec change is still implementing the visualizer foundation; this change adds content destinations without rewriting its requirements.

## Impact

- **Rust models**: Extend `src/resume.rs` beyond the current basics/work/skills subset to cover profiles, education, publications, languages, interests, and projects, while preserving the actual resume JSON as the source of truth.
- **GPUI UI**: Extend `src/app.rs` navigation state and content views; no DOM-based section renderer is introduced.
- **Web host**: No new browser framework or visualization dependency; `web/src/main.js` remains a minimal WASM loader.
- **Tests**: Add native tests for parsing the actual resume sections, conditional navigation, WordPress publication filtering, and empty-section behavior.
- **Compatibility**: Existing overview, experience, and geography behavior remains available; sections with no source data are not shown as populated content.
