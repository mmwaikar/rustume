## Context

The current UI has a full-size `bubble` helper and a `compact_bubble` helper, but project highlights still use the full-size helper. The skills graph already creates one keyword node per normalized keyword and emits parent-child edges, while the skills list separately renders keyword text. The change must preserve the existing JSON Resume model and graph payload shape while making the visual distinction and graph contract explicit.

## Goals / Non-Goals

**Goals:**

- Centralize the compact visual treatment for skill levels, language fluency, and project tags.
- Keep compact bubble labels on one readable horizontal line and prevent them from forcing surrounding layout growth.
- Preserve one child node and one parent-child relationship per case-insensitive keyword for each skill.
- Add regression coverage at the graph boundary and keep the existing isolated-skill and shared-keyword behavior intact.

**Non-Goals:**

- Redesigning the overall bubble theme, colors, spacing system, or profile highlight presentation.
- Changing resume parsing, JSON data structures, graph serialization, or unrelated visualizations.
- Changing the meaning of shared-keyword relationships between separate parent skills.

## Decisions

- Reuse the existing compact bubble helper for all three compact-label surfaces. This keeps sizing and text behavior consistent; adding separate per-section styles would make the reviewed visual bug easy to reintroduce.
- Keep the full-size bubble helper for profile headline/highlight content because those labels are primary identity content, while levels, fluency, and project tags are supporting metadata.
- Treat keyword identity case-insensitively within each parent skill and retain the first source spelling for display. This matches the existing graph normalization and avoids duplicate child nodes without changing source data.
- Validate graph behavior through the existing `skills_graph` unit tests, including multiple children, duplicate casing, valid endpoints, and isolated skills. A UI screenshot check can confirm compact sizing during implementation, but the planning contract remains observable behavior rather than a pixel value.

## Risks / Trade-offs

- [Risk] Very long fluency or tag text may still exceed the available width even with compact styling -> keep the existing no-wrap behavior for readable labels and validate representative resume content at narrow widths.
- [Risk] Changing helper usage could accidentally shrink profile headline bubbles -> explicitly retain full-size `bubble` calls for headline claims and verify the affected views separately.
- [Risk] Graph node IDs may collide for distinct punctuation or normalized values -> preserve the current parent-qualified graph ID strategy and add assertions that every edge endpoint resolves to a node.
