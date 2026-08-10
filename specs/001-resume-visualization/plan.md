# Implementation Plan: Resume Visualization

**Branch**: `001-resume-visualization` | **Date**: 2026-08-10 | **Spec**: [spec.md](spec.md)

**Input**: Feature specification from `/specs/001-resume-visualization/spec.md`

## Summary

Build a Leptos-based resume visualizer that loads a JSON Resume document, derives experience and skill relationships in Rust, and presents them through a Spectrum 2-inspired shell with a bar-chart view for company experience and a Cytoscape-based nodes-and-edges graph view for companies, countries, and skills. The app will be deployable as a static GitHub Pages site.

## Technical Context

**Language/Version**: Rust 1.85+ (edition 2021)

**Primary Dependencies**: Leptos, serde, serde_json, wasm-bindgen, Cytoscape.js via browser interop, Spectrum 2 web components via WASM/browser integration

**Storage**: Static JSON resume file served with the app (for the initial version)

**Testing**: `cargo test` for Rust unit tests, plus manual smoke checks in the browser

**Target Platform**: WebAssembly in the browser, deployed to GitHub Pages

**Project Type**: Web application

**Performance Goals**: Render the overview and graph within 2 seconds on a typical desktop browser for a single resume profile

**Constraints**: Must stay compatible with a client-side WASM build; external visualization libraries must be loaded in a browser-safe way; GitHub Pages deployment must use static assets only

**Scale/Scope**: Single resume profile with multiple experience entries, companies, countries, and skills

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- The feature MUST preserve a visually faithful resume experience comparable to the reference visualization.
- Rendering and data shaping MUST remain Rust-native, deterministic, and performant.
- Accessibility, readability, and testability requirements MUST be addressed before implementation is considered complete.

## Project Structure

### Documentation (this feature)

```text
specs/001-resume-visualization/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
└── tasks.md
```

### Source Code (repository root)

```text
src/
├── app.rs
├── lib.rs
├── main.rs
├── models/
│   ├── resume.rs
│   ├── views.rs
│   └── graph.rs
├── services/
│   ├── resume_loader.rs
│   ├── resume_transform.rs
│   └── graph_builder.rs
├── ui/
│   ├── components/
│   │   ├── shell.rs
│   │   ├── experience_chart.rs
│   │   └── skill_graph.rs
│   └── styles.rs
└── assets/
    └── resume.json
```

**Structure Decision**: The feature will stay in a single Rust/Leptos web app with a dedicated `models` layer for the JSON Resume schema and derived visualization payloads, a `services` layer for data loading and transformation, and a `ui` layer for the interactive experience.

## Complexity Tracking

No constitution violations are expected for this feature.
