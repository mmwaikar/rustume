# compendium-projects-layout Specification

## Purpose

Define the screenshot-aligned project hero and responsive project-card grid while preserving source descriptions, highlights, and links.

## Requirements

### Requirement: Projects hero composition
The Projects view MUST begin with a rounded hero panel using the current Rustume palette, containing a section eyebrow, the heading `Selected works`, and supporting copy describing the project collection.

#### Scenario: Projects screenshot match
- **WHEN** the Projects section is opened with project data
- **THEN** the hero uses the reference title hierarchy and leaves a clear transition into the project grid

### Requirement: Project card grid
Each project MUST render as a rounded white card in a two-column wide-screen grid, with a prominent linked project name, highlight badges, readable description, and preserved external source link behavior.

#### Scenario: Populated project grid
- **WHEN** the resume contains multiple projects
- **THEN** projects are distributed across the grid without losing any name, highlight, description, or link

### Requirement: Projects responsive behavior
The project grid MUST collapse to one column on narrow viewports and keep card content readable without horizontal scrolling.

#### Scenario: Mobile projects
- **WHEN** the viewport is narrow
- **THEN** project cards stack vertically and long descriptions wrap within their cards
