## Why

The 12-Sep-2026 Round 2 review still shows each skill as having only two visible tree nodes: one parent skill and one aggregate child containing all sub-skills. This hides the individual relationships users need to inspect and makes the skill tree misleading when a skill has several keywords.

## What Changes

- Render each distinct sub-skill as its own child node beneath its parent skill.
- Preserve the parent skill node, the readable sub-skill labels, and the existing relationship semantics.
- Add regression coverage that distinguishes multiple child nodes from a single aggregated child and preserves case-insensitive duplicate handling.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills-view`: The visible skill tree must expose one node for the parent and one node for each distinct sub-skill.

## Impact

- `src/app.rs`: the user-visible Skills section and any node-list rendering derived from skill graph data.
- `src/resume.rs`: skill graph payload construction and regression tests.
- `openspec/specs/skills-view/spec.md`: updated requirement for distinct visible child nodes.
