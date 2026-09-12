## Context

The resume model already normalizes keywords case-insensitively and the `skills_graph` payload already creates parent-qualified child nodes. The reported behavior therefore points to the presentation path: the current application exposes graph summaries and skill rows, but any node-list or tree renderer must preserve the graph node boundaries instead of joining all child labels into one aggregate display value.

## Goals / Non-Goals

**Goals:**

- Ensure the user-visible Skills tree renders one parent node and one child node per distinct sub-skill.
- Preserve readable labels, parent-child edges, isolated skills, and case-insensitive deduplication.
- Add a regression test at the presentation/data boundary that would fail if multiple child labels are collapsed into one displayed node.

**Non-Goals:**

- Changing the JSON Resume schema or the meaning of skill keywords.
- Changing shared-keyword relationships between separate parent skills.
- Redesigning bubble sizing, sidebar navigation, or unrelated graph views.

## Decisions

- Treat the graph payload as the source of truth for node identity. The renderer should iterate distinct graph nodes and their relationships rather than reconstructing a single child from the raw keyword list.
- Keep parent-qualified, case-insensitive child identity. This preserves the existing graph contract and avoids duplicate nodes for casing-only variations without changing displayed source spelling.
- Keep the existing parent skill presentation and add separate child presentation beneath or connected to it. An aggregate string such as `A * B` is not an acceptable substitute because it erases node identity and relationship boundaries.
- Test both a skill with multiple keywords and a skill with case-duplicate keywords. The first verifies separate visible children; the second verifies stable deduplication.

## Risks / Trade-offs

- [Risk] The current UI path may not expose a dedicated graph renderer -> identify the actual node rendering boundary during implementation and add the smallest renderer needed to preserve graph identities.
- [Risk] A renderer may accidentally show duplicate children when it combines raw keywords with graph nodes -> use normalized graph node identities for the rendered child collection.
- [Risk] Longer sub-skill labels may crowd the tree -> preserve readable labels and use the existing scrollable content surface rather than truncating or aggregating nodes.
