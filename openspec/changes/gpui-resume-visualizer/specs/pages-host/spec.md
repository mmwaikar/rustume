## Purpose

Publish the resume visualizer as a static site on GitHub Pages so anyone with the URL can open it in a supported browser without a backend server.

## ADDED Requirements

### Requirement: Static GitHub Pages deployment
The system MUST be buildable into static assets (HTML, JavaScript, WebAssembly, and the resume JSON) that GitHub Pages can serve. A documented CI path MUST publish those assets on push to the default branch (or equivalent release workflow).

#### Scenario: Pages URL loads the visualizer
- **WHEN** a visitor opens the deployed GitHub Pages URL in a supported browser
- **THEN** the visualizer boots and, when the resume asset is present, shows the shell and resume views without calling a custom application server

### Requirement: Resume file ships with the site
The deployed site MUST include the JSON Resume file as a static asset loaded by the visualizer. Changing that file and redeploying MUST change what visitors see.

#### Scenario: Asset present
- **WHEN** the production build completes successfully
- **THEN** the published artifact contains the resume JSON used by the visualizer

### Requirement: Unsupported or failed boot is explained
If the browser cannot run the visualizer (missing required graphics or WASM support), the page MUST show a human-readable message rather than a blank canvas with no explanation.

#### Scenario: Incompatible browser
- **WHEN** the runtime cannot start the visualizer
- **THEN** the visitor sees an error or fallback message describing that the app could not start
