<!--
Sync Impact Report
- Version change: 0.0.0 → 1.0.0
- Modified principles: Initial constitution introduced
- Added sections: Visualization Standards, Development Workflow
- Removed sections: None
- Templates requiring updates: .specify/templates/plan-template.md ✅ updated, .specify/templates/spec-template.md ✅ updated, .specify/templates/tasks-template.md ✅ updated
- Follow-up TODOs: None
-->

# Rustume Constitution

## Core Principles

### I. Resume-First Fidelity
Every feature MUST preserve a visually faithful, narrative-rich resume experience that is comparable to the Compendium reference. The visualization MUST communicate the candidate's story clearly through structure, emphasis, and relationships between roles, skills, and achievements. Rationale: the core product value is the quality of the story told by the visualization, not merely the presence of data.

### II. Rust-Native Performance
Rendering, data shaping, and analytics MUST remain fast, deterministic, and browser-safe. Rust is the source of truth for preparing resume data and derived payloads; Leptos and WASM MUST be used in a way that preserves simplicity, reliability, and predictable behavior. Rationale: the experience must feel immediate and stable even as the data model grows.

### III. Testable Visualization
Visual and interaction changes MUST be validated through automated checks where feasible and through explicit review against the reference experience. Data transformations, layout decisions, and content rules MUST be testable without relying on manual guesswork. Rationale: visual quality degrades quickly when changes are not verified.

### IV. Accessibility and Readability
The resume visualization MUST remain legible, navigable, and understandable across devices and assistive technologies. Contrast, hierarchy, text scaling, and keyboard support are mandatory; ornamental effects MUST NOT compromise comprehension. Rationale: a beautiful visualization is not successful if it is inaccessible or hard to parse.

### V. Incremental Delivery
Features MUST be delivered as small, independently testable slices that can be reviewed and deployed without breaking the core visualization. Rework is allowed only when it improves fidelity, maintainability, or performance. Rationale: the visualization must evolve safely while preserving a coherent user experience.

## Visualization Standards
The project MUST target a polished, high-information-density resume experience inspired by the Compendium reference. Implementations MUST preserve clear relationships between experiences, skills, accomplishments, and supporting context, and they MUST remain responsive for desktop and mobile viewing. All visual choices MUST be driven by structured resume data and documented in the feature spec.

## Development Workflow
All work MUST begin from a concrete resume data model and a defined visual goal. Any change to layout, interaction, or content prioritization MUST be documented in the spec or plan and reviewed for fidelity, accessibility, and performance. Before merge, contributors MUST verify the change with relevant tests and a manual review of the rendered output.

## Governance
This constitution supersedes ad hoc design preferences for Rustume. Any amendment MUST include a documented rationale, a version bump, and review against the current principles before implementation proceeds. Pull requests and planning artifacts MUST confirm compliance with these principles, especially visual fidelity, performance, and accessibility.

**Version**: 1.0.0 | **Ratified**: 2026-08-10 | **Last Amended**: 2026-08-10
