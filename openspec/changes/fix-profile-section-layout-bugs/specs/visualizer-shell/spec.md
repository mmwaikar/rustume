## MODIFIED Requirements

### Requirement: Resizable shell panes
The visualizer MUST retain the registered dock panels needed for sidebar resizing, but MUST NOT render the dock panel names `sidebar` or `content` as visible labels in the application shell.

#### Scenario: Shell renders without duplicate panel labels
- **WHEN** the application shell is opened
- **THEN** the sidebar and main pane show their navigation and resume content without standalone `sidebar` or `content` labels

#### Scenario: Resize the sidebar
- **WHEN** a visitor drags the sidebar/content divider
- **THEN** the relative pane widths change while both panes remain visible and usable

### Requirement: Sidebar icon controls
The visualizer MUST apply the sidebar background across the full visible sidebar pane, including unused vertical space below navigation items and the area exposed when the main content is shorter.

#### Scenario: Sidebar fills its dock pane
- **WHEN** the sidebar is displayed beside the main content
- **THEN** every visible part of the sidebar pane uses the sidebar background, while navigation controls remain readable and the pane remains resizable

#### Scenario: Navigate using an icon
- **WHEN** a visitor activates a destination icon
- **THEN** the corresponding section becomes active and the navigation remains visible

### Requirement: Primary sidebar navigation
The visualizer MUST omit Overview from the primary sidebar, place Profile first, and preserve conditional Projects and Blog Posts destinations only when their source data is available.

#### Scenario: Sidebar destinations with optional content
- **WHEN** the resume contains profile data, projects, and WordPress blog data
- **THEN** the sidebar lists Profile first, followed by the remaining supported destinations, and does not list Overview

#### Scenario: Sidebar destinations without optional content
- **WHEN** the resume has no projects or WordPress blog data
- **THEN** the sidebar omits those optional destinations while keeping Profile first

### Requirement: Scrollable content pane
The right content pane MUST scroll independently when the active section exceeds the available viewport height, while the header, footer, and sidebar remain part of the application shell.

#### Scenario: Long Profile content
- **WHEN** Profile content is taller than the viewport
- **THEN** the visitor can scroll through all Profile content in the right pane without losing access to the shell navigation
