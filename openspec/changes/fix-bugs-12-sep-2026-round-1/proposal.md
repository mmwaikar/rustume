## Why

The 12-Sep-2026 pre-implementation review found that compact data labels are visually oversized relative to their role, and that the skills visualization does not expose each sub-skill as its own child node. These issues make the resume harder to scan and make the graph structure less useful for understanding individual skills and their relationships.

## What Changes

- Use a smaller, consistent compact bubble treatment for language fluency, skill level, and project tags while preserving the existing larger treatment for profile highlights and other primary labels.
- Represent every skill keyword/sub-skill as a distinct child node connected to its parent skill in the skills graph.
- Add focused regression coverage for compact bubble usage and one-node-per-sub-skill graph output, including duplicate keyword normalization behavior.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `skills-view`: Compact skill labels and the skills graph must present sub-skills as distinct child nodes.
- `profile-view`: Language fluency labels must use the compact bubble treatment.
- `projects-view`: Project tags must use the compact bubble treatment.

## Impact

- `src/app.rs`: shared bubble helpers and the skills section’s compact label usage.
- `src/resume.rs`: skills graph node and edge construction, plus graph regression tests.
- `openspec/specs/skills-view/spec.md`: updated behavioral requirements for compact labels and graph children.
