## Context

See `proposal.md` for motivation. The current Rust model only deserializes basics name/label/summary, work, skills, and placeholder project/education values. The actual resume now includes structured profiles, a nested basics location, education, publications, languages, interests, projects, and WordPress links. The current GPUI app has a persistent shell and section state but only renders Overview, Experience, Skills, and Geography.

## Goals / Non-Goals

**Goals:**

- Preserve the actual JSON Resume as the only content source.
- Add typed models for the resume sections required by the new views.
- Extend GPUI section state and navigation while keeping the header, sidebar, main pane, and footer persistent.
- Render profile, detailed skills, projects, and WordPress-derived blog posts entirely with gpui-kit elements.
- Keep empty sections predictable and free of fabricated content.
- Cover model transforms and content-selection rules with native tests.

**Non-Goals:**

- Live WordPress/RSS/API fetching.
- Resume editing, filtering, search, or multi-profile support.
- Replacing the existing experience and geography analytics contract.
- DOM-based rendering or adding a client-side UI framework.

## Decisions

### D1. Extend the existing hand-rolled JSON Resume model

Add typed serde structs for profiles, nested basics location, education, publications, languages, interests, and projects. Keep flexible optional fields for schema data not currently displayed. This preserves the existing crate boundary and avoids introducing a schema crate that could conflict with the GPUI toolchain.

Alternative considered: deserialize `serde_json::Value` everywhere. Rejected because typed fields make view behavior and tests explicit.

### D2. Use one GPUI section enum with conditional destinations

Extend the existing section state with Profile, Skills detail, Projects, Blog Posts, Education, Publications, Languages, and Network as needed. Primary navigation is built from non-empty source data; Profile and the existing analytics destinations remain available. Main content swaps within the same GPUI entity so chrome persists.

Alternative considered: browser routes or separate windows. Rejected because the current design uses entity state and a persistent Dock-style shell.

### D3. Treat Blog Posts as a deterministic publication projection

Filter publication entries by publisher value `WordPress` and expose their existing title/date/summary/URL. Use the WordPress profile as an optional source link. Do not fetch a remote feed, because the JSON Resume is the source of truth and network data would make the static app nondeterministic.

Alternative considered: fetch WordPress at runtime. Rejected because of CORS, availability, privacy, and reproducibility risks.

### D4. Render rich content as reusable GPUI rows/cards

Use small GPUI view helpers for identity rows, metadata, links, keyword lists, project items, education items, publication items, and language/interest groups. Keep visual hierarchy dense and content-forward, matching Compendium without copying its Blazor or Fluent UI implementation.

Alternative considered: move sections into HTML templates. Rejected because the browser UI must remain GPUI-owned.

### D5. Derive sidebar visibility from data availability

Projects, Blog Posts, Education, Publications, Languages, and Network destinations appear only when corresponding source data exists. Profile remains the default destination even when optional fields are absent; each empty subsection is omitted or represented by a clear empty state according to its spec.

## Risks / Trade-offs

- [Actual resume fields vary from the minimal JSON Resume subset] -> Use optional serde fields and test against the checked-in resume before changing UI code.
- [Large work history makes the profile visually dense] -> Keep Profile focused on identity and supporting sections; retain detailed work history in Experience.
- [Publisher spelling/casing changes] -> Use a documented case-insensitive match for `WordPress` while preserving source text for display.
- [GPUI text/link ergonomics differ from DOM controls] -> Build links as keyboard-focusable GPUI interactive elements and validate native compilation after each view slice.

## Migration Plan

1. Extend resume models and tests against the current `assets/resume.json`.
2. Add derived selectors for WordPress publications and conditional navigation.
3. Add GPUI content views and keyboard-accessible navigation entries.
4. Run native tests, native Cargo checks, nightly WASM checks, and the minimal Vite host build.
5. Roll back by reverting the model/view commit; the source resume remains unchanged.
