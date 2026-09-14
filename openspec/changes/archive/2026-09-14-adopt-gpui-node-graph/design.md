## Context

See proposal.md for motivation. Rustume builds one Rust `cdylib` for `wasm32-unknown-unknown` and starts its GPUI Kit application with `run_embedded`; Vite imports the generated Wasm bundle. The Geography section currently calculates `geography_graph` but renders only its node and edge counts plus a flat label list. Rustume already depends on `gpui-kit` 0.6.0, while its Pages workflow remains a scaffold that publishes `docs` rather than the Vite/Wasm bundle.

## Goals / Non-Goals

**Goals:**
- Render the existing country/company relationship data as collapsible graph state.
- Keep desktop and GitHub Pages browser behavior functionally equivalent.
- Exercise the published build rather than treating a local browser server as sufficient evidence.

**Non-Goals:**
- Replace the Skills tree or the Experience by Company chart.
- Add graph editing, connection creation, or persistence of graph viewport and selection state.
- Move the application away from static GitHub Pages hosting.

## Decisions

### Render a focused graph with existing GPUI Kit primitives

Implement the read-only, two-level graph directly in the existing Geography view using GPUI Kit layout elements, styled containers, and click handlers. The renderer positions persistent country nodes in a stable vertical column and lays out the expanded country’s company nodes in a responsive adjacent area. Connection lines are represented by the graph's own visual elements and stay aligned to their visible source and target nodes. This narrow layout directly serves the resume hierarchy without importing editing, routing, persistence, or browser-runtime requirements from an external graph library.

Alternative considered: adopt a GPUI graph library. Rejected because the candidates either require hosting features unavailable on GitHub Pages or do not document browser support.

### Model country expansion as application-owned view state

Keep `geography_graph` as the canonical resume-derived relationship source. Add transient state that records expanded country identifiers, then derive the graph nodes and edges presented to the view from that state. Country nodes are always visible; company nodes and their edges are included only for expanded countries. This prevents selection, pan, and layout state from leaking into the resume data model.

Alternative considered: persist expansion information in `resume.json`. Rejected because expansion is per-session presentation state, not resume content.

### Keep interaction state local and accessible

Country-node controls use ordinary GPUI interactive elements so pointer and keyboard activation share the same expand/collapse action. The visual state includes an explicit expanded/collapsed indicator and a textual company count. Company labels remain selectable and a readable empty state is shown when no country relationships exist.

Alternative considered: canvas-only hit testing. Rejected because ordinary interactive GPUI elements preserve the application's established input model and keep the limited interaction surface testable.

### Deploy the built web bundle

Update the Pages build job to compile the Wasm library, run `wasm-bindgen`, build Vite, and upload `web/dist`. The browser acceptance check must serve the same artifact with GitHub Pages-compatible behavior.

Alternative considered: continue publishing `docs`. Rejected because it does not contain the Vite/Wasm application.

## Risks / Trade-offs

- [The fixed two-level layout becomes crowded for many companies] -> Limit the visible detail to the active country, allow the content panel to scroll, and test a representative multi-country fixture.
- [Connection elements drift when responsive layouts reflow] -> Keep connection styling within the same country/company row layout as its endpoints and validate narrow and wide application windows.
- [Wasm packaging regresses while adding dependencies] -> Validate the generated Vite artifact and published-style browser startup in CI.

## Migration Plan

1. Add the country expansion projection and integrate the app-owned GPUI Kit renderer behind the Geography navigation item.
3. Change the Pages workflow to build and publish `web/dist`.
4. Validate desktop interaction, browser interaction, and the deployed artifact; retain the current text-only view as the rollback path until those checks pass.
5. Roll back by restoring the existing Geography content and Pages artifact configuration if any target fails.