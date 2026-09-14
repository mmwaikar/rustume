## Why

The Experience by Country view currently exposes only a text summary of the generated geography graph, rather than the expandable node-and-edge visualization requested for the resume visualizer. The graph must be implemented with Rustume's existing GPUI Kit stack so it remains deployable to GitHub Pages without external graph-library runtime constraints.

## What Changes

- Replace the Geography view's textual node and relationship summary with an interactive Experience by Country node-and-edge graph.
- Render the graph with existing GPUI Kit elements and Rustume-owned layout and interaction state, without adding a graph-rendering dependency.
- Validate the graph on the desktop target and the existing `wasm32-unknown-unknown` browser build published to GitHub Pages.
- Support expanding a country node to reveal its associated company nodes and connections.

## Capabilities

### New Capabilities
- `experience-country-graph`: Renders and interacts with the country-grouped company experience graph across supported desktop and browser targets.

### Modified Capabilities
- `experience-view`: Expands the experience visualizer requirements to include the interactive country view alongside the company duration chart.

## Impact

- Affected code: `src/app.rs`, `src/resume.rs`, `src/lib.rs`, and the Web/Vite deployment configuration.
- Dependencies: No graph-rendering dependency is added; the implementation uses the existing GPUI Kit dependency.
- Deployment: The built browser application must be published as a GitHub Pages-compatible static artifact.