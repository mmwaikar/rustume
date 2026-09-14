# Implementation Plan

1. Scaffold the Rust repo and gpui app.
2. Port the resume schema and analytics from the Blazor app.
3. Rebuild the navigation shell and page layout.
4. Render the country/company graph with Rust-built GPUI Kit elements, application-owned layout, and expand/collapse state.
5. Add Spectrum 2-oriented styling.
6. Configure GitHub Pages deployment to publish the Vite bundle in `web/dist`, including the generated WebAssembly files.
7. Validate route coverage and output parity.

## Experience By Country

The Experience by Country view uses Rustume's existing GPUI Kit stack and adds no external graph-rendering library. Country nodes remain visible; selecting a country expands its connected company nodes and connector elements. The browser build is a static Vite artifact and does not require COOP or COEP response headers, so it can be published on GitHub Pages.
