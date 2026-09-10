# visualizer-shell Specification

## Purpose

Define the shared Rustume application shell so navigation, resizing, scrolling, and persistent chrome behave consistently across every resume section.

## Requirements

### Requirement: Primary sidebar navigation
The visualizer MUST omit Overview from the primary sidebar, place Profile first, and preserve conditional Projects and Blog Posts destinations only when their source data is available.

#### Scenario: Sidebar destinations with optional content
- **WHEN** the resume contains profile data, projects, and WordPress blog data
- **THEN** the sidebar lists Profile first, followed by the remaining supported destinations, and does not list Overview

#### Scenario: Sidebar destinations without optional content
- **WHEN** the resume has no projects or WordPress blog data
- **THEN** the sidebar omits those optional destinations while keeping Profile first

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

### Requirement: Consistent default link styling
All user-facing links in the visualizer MUST use the default blue link styling consistently, including footer technology links, profile and network links, project links, publication links, and blog-post links. Link styling MUST remain visually distinct from surrounding selectable text while preserving each link's existing destination and activation behavior.

#### Scenario: Links appear across multiple views
- **WHEN** a visitor opens a profile, projects, publications, blog-post, or footer view containing links
- **THEN** every visible link uses the same default blue link color and remains distinguishable from non-link text

#### Scenario: Link destinations remain unchanged
- **WHEN** a visitor activates any styled link
- **THEN** the link opens its existing destination without changing URL or navigation behavior

### Requirement: Linked footer attribution
The visualizer shell MUST keep the footer in the StatusBar and show an attribution that says it is made with a heart using Rust and gpui-kit. The Rust name MUST link to `https://rust-lang.org/`, and the gpui-kit name MUST link to `https://gpui-kit.com`.

#### Scenario: Footer attribution is visible
- **WHEN** any resume section is displayed
- **THEN** the persistent footer shows the requested heart-marked attribution and both technology names are visible in the StatusBar

#### Scenario: Footer technology links are activated
- **WHEN** a visitor activates the Rust or gpui-kit name in the footer
- **THEN** the corresponding official URL is opened using the existing link behavior
