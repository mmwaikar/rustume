# Research Notes: Resume Visualization

## 1. Resume data format

Decision: The application will ingest a JSON Resume document that follows the public JSON Resume schema and will map its fields into structured Rust models.

Rationale: The schema is well known, portable, and already suited to the requested views because it provides `basics`, `work`, `skills`, and related metadata without inventing a custom format.

Alternatives considered:
- Custom schema in a bespoke JSON file — rejected because it would be harder to reuse and less interoperable.
- Markdown-based resume source — rejected because the structure needed for charting and graphing would be less consistent.

## 2. Rendering strategy

Decision: Rust will own the data transformation and view-model preparation, while the UI will use Leptos for component composition and render the chart and graph through browser interop wrappers around Spectrum web components and Cytoscape.js.

Rationale: This keeps the core business logic in Rust and still allows the app to use the required third-party web visualization libraries in a browser-native way.

Alternatives considered:
- Implementing the visualizations entirely in JavaScript — rejected because it would weaken the Rust-native architecture and make the data shaping harder to test.
- Replacing Cytoscape with a custom renderer — rejected because it would not meet the requirement for a nodes-and-edges graph layout.

## 3. Deployment strategy

Decision: The site will be built as a static WASM web app and published to GitHub Pages through a GitHub Actions workflow.

Rationale: GitHub Pages supports static output and aligns with the repository deployment requirement without introducing a backend service.

Alternatives considered:
- Deploying to a SaaS host such as Vercel or Netlify — rejected because the requirement explicitly calls for GitHub Pages.
