## ADDED Requirements

### Requirement: Responsive sidebar collapse
The visualizer MUST collapse the primary sidebar on narrow viewports so the content pane receives the full available width, while keeping the sidebar open with a 240px width on wider viewports. The visitor MUST be able to reopen the collapsed sidebar on narrow viewports using the shell's dock toggle affordance.

#### Scenario: Narrow viewport loads with sidebar collapsed
- **WHEN** the window width is below the responsive breakpoint of 768px
- **THEN** the primary sidebar is closed by default and the content pane spans the full window width

#### Scenario: Narrow viewport reopens the sidebar
- **WHEN** the window width is below 768px and the visitor activates the dock toggle
- **THEN** the sidebar reopens over the content pane and remains reachable through the dock

#### Scenario: Wide viewport keeps the sidebar open
- **WHEN** the window width is at or above 768px
- **THEN** the primary sidebar is open at 240px and the content pane is laid out beside it as before

#### Scenario: Resize crosses the breakpoint
- **WHEN** the window is resized across the 768px breakpoint
- **THEN** the sidebar collapses when crossing below it and reopens when crossing above it, without requiring a reload