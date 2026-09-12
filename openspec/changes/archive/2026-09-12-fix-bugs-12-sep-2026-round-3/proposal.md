## Why

The 12-Sep-2026 Round 3 review found that the new Skills tree is structurally correct but visually incomplete: nodes have no folder/file icons, child rows lack visible indentation, all branches start expanded, and the tree does not use the available content height. These issues make the hierarchy difficult to scan and force unnecessary scrolling.

## What Changes

- Show open/closed folder icons for skill nodes based on expansion state and a file-like or skill icon for sub-skill leaves.
- Indent child rows one tree level inside their parent while preserving readable labels and compact skill-level bubbles.
- Initialize all skill branches collapsed and retain normal tree interaction for expanding and collapsing them.
- Allow the Skills tree to fill the remaining content height so the view does not create an avoidable short fixed-height viewport.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills-view`: The tree presentation must communicate hierarchy with icons and indentation, start collapsed, and use the available vertical space.

## Impact

- `src/app.rs`: Skills tree item construction and tree row rendering, including icons, depth styling, expansion defaults, and layout sizing.
- `src/resume.rs`: Existing skill graph data remains the source for parent/child identity and relationships; no resume schema changes are expected.
- `openspec/specs/skills-view/spec.md`: Updated visible tree presentation requirements.
