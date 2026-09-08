# visualizer-shell Specification

## Purpose

Define the shared Rustume application shell so navigation chrome, dock behavior, and sidebar readability remain consistent while the profile and publication content are corrected.

## MODIFIED Requirements

### Requirement: Sidebar icon visibility
The visualizer MUST keep sidebar destinations readable and distinguishable in the left dock, including the icon glyph and the destination label, without exposing redundant shell identity labels.

#### Scenario: Sidebar icons are unreadable
- **WHEN** the resume shell loads and the sidebar is visible
- **THEN** the navigation icon, its label, and the current selection remain clearly visible and consistent with the app’s palette and sizing

### Requirement: Full sidebar pane painting
The sidebar pane MUST use the configured background color across the full docked area, even when the content area inside the sidebar is shorter than the pane height.

#### Scenario: Sidebar pane background
- **WHEN** the left dock pane is taller than its child content
- **THEN** the pane background fills the full available height and does not stop at the content bounds

### Requirement: Shell resizing without layout breakage
The docked sidebar and content pane MUST keep their resize behavior and minimum usable widths while the shell chrome remains minimal and readable.

#### Scenario: Resize the sidebar
- **WHEN** a visitor adjusts the left/right split in the shell
- **THEN** the sidebar and content pane remain visible, usable, and internally consistent without zero-width collapse
