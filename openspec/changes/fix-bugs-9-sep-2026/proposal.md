## Why

The 9-Sep-2026 review found several remaining usability and visual regressions in the resume visualizer: text cannot be selected, the shell chrome and typography do not match the intended GPUI Kit presentation, sidebar icons are placeholders, and the experience and skills views do not communicate their data clearly. This change consolidates those fixes so the application is readable, inspectable, and consistent when launched.

## What Changes

- Replace the custom footer with a StatusBar-style footer and apply the intended Inter font/theme configuration.
- Make user-facing text selectable throughout the visualizer.
- Render real GPUI Kit icon assets in sidebar navigation instead of geometric placeholders.
- Render experience as a bottom-aligned vertical bar chart with rounded bars and readable diagonal company labels.
- Render skills as a hierarchy where each sub-skill is a separate child node, with compact level bubbles sized to their labels.
- Start the desktop application with its window maximized.

## Capabilities

### New Capabilities

- `experience-view`: Presents company experience as an inspectable vertical bar chart with readable labels.

### Modified Capabilities

- `visualizer-shell`: Requires selectable shell/content text, themed Inter typography, visible asset-backed sidebar icons, StatusBar footer chrome, and maximized startup behavior.
- `profile-view`: Requires profile text and supporting profile content to remain selectable.
- `skills-view`: Requires compact level indicators and a separate child node for each sub-skill.

## Impact

- Affects the GPUI application shell and shared rendering helpers in `src/app.rs` and startup/theme initialization in `src/lib.rs`.
- Affects skills graph data shaping in `src/resume.rs` and the Skills view presentation.
- Uses the GPUI Kit controls/assets and bundled font resources; no resume JSON schema or external API changes are intended.
- Adds or updates focused unit/rendering checks for graph structure, formatting, and shell behavior where the current test harness permits.