## Purpose

Define the shared Rustume application shell so navigation, resizing, scrolling, and persistent chrome behave consistently across every resume section.

## ADDED Requirements

### Requirement: Primary sidebar navigation
The visualizer MUST omit Overview from the primary sidebar, place Profile first, and preserve conditional Projects and Blog Posts destinations only when their source data is available.

#### Scenario: Sidebar destinations with optional content
- **WHEN** the resume contains profile data, projects, and WordPress blog data
- **THEN** the sidebar lists Profile first, followed by the remaining supported destinations, and does not list Overview

#### Scenario: Sidebar destinations without optional content
- **WHEN** the resume has no projects or WordPress blog data
- **THEN** the sidebar omits those optional destinations while keeping Profile first

### Requirement: Sidebar icon controls
Each sidebar destination MUST display a reliable, distinguishable icon together with its accessible text label, and activating either the icon or label MUST select the destination.

#### Scenario: Navigate using an icon
- **WHEN** a visitor activates a destination icon
- **THEN** the corresponding section becomes active and the navigation remains visible

### Requirement: Resizable shell panes
The visualizer MUST place the sidebar and right content pane in a resizable split, with minimum usable widths for both panes and no zero-width collapse.

#### Scenario: Resize the sidebar
- **WHEN** a visitor drags the sidebar/content divider
- **THEN** the relative pane widths change while both panes remain visible and usable

### Requirement: Scrollable content pane
The right content pane MUST scroll independently when the active section exceeds the available viewport height, while the header, footer, and sidebar remain part of the application shell.

#### Scenario: Long Profile content
- **WHEN** Profile content is taller than the viewport
- **THEN** the visitor can scroll through all Profile content in the right pane without losing access to the shell navigation
