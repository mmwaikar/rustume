## Context

The Skills view now uses the gpui-kit tree component with nested tree items, but the current row renderer only displays text and the tree is initialized with expanded roots inside a fixed-height container. The tree API exposes entry depth, folder status, and expansion status, while the application already has an asset-backed icon helper and existing icon files.

## Goals / Non-Goals

**Goals:**

- Make folder hierarchy legible through state-aware parent icons and a leaf icon.
- Apply one tree-level indentation to child rows using the tree entry depth.
- Initialize every parent branch collapsed while preserving the tree component's expand/collapse interaction.
- Let the tree occupy the available Skills content height and avoid an unnecessarily constrained viewport.

**Non-Goals:**

- Changing skill graph identity, keyword normalization, or parent-child relationships.
- Adding new dependencies or replacing the gpui-kit tree component.
- Redesigning skill bubbles, labels, sidebar navigation, or other sections.

## Decisions

- Render icons from the existing asset-backed icon path helper. Use folder-open/folder-closed semantics for branch entries based on `is_expanded()`, and a stable file/skill glyph for leaves; this keeps icon loading consistent with the sidebar.
- Use `TreeEntry::depth()` to apply the child indentation at render time instead of encoding spaces into labels. This preserves selection, text measurement, and accessibility semantics.
- Build root `TreeItem`s without `.expanded(true)`, leaving all branches collapsed on initial state. The tree control remains responsible for expansion events and visible-entry flattening.
- Replace the fixed tree height with a flex-growing/minimum-height layout inside the existing scrollable content pane. The tree should fill the available section area while the outer content pane retains responsibility for overflow.

## Risks / Trade-offs

- [Risk] An icon asset name may not exist or may fail in the current WASM loader -> use an icon already present in `assets/icons` and verify both native compilation and the available UI smoke path.
- [Risk] Removing the fixed height may allow the tree to grow beyond short windows -> retain the outer content scroll container and set only layout constraints needed for flex growth.
- [Risk] Depth styling could double-indent if the tree component adds its own padding -> validate a parent/child pair visually and keep the renderer's added offset limited to one level.
