# Quickstart: Resume Visualization

## Prerequisites

- Rust toolchain with wasm support
- A modern browser with WebAssembly support
- A JSON Resume file to load as the visualization source

## Local development

1. Place the resume JSON in a static asset path such as `public/resume.json`.
2. Build and run the app with the Leptos development workflow.
3. Open the local URL shown by the dev server and confirm that the resume appears.

## Validation scenarios

- Open the app and verify that the experience view displays a bar-chart-style overview of companies and roles.
- Confirm that the graph view shows companies, countries, and skills as connected nodes.
- Check that the UI remains understandable when a country or skill relationship is missing.

## GitHub Pages deployment

1. Build the production static site.
2. Publish the generated static assets from the GitHub Actions workflow to the `gh-pages` branch.
3. Open the deployed GitHub Pages URL and confirm that the app renders correctly.
