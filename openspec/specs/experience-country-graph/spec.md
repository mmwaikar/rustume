# experience-country-graph Specification

## Purpose

Defines an interactive, country-grouped experience visualization that remains usable in Rustume's desktop and published browser applications.

## Requirements

### Requirement: Country-grouped experience graph
The application MUST present Experience by Country as a node-and-edge visualization whose initial view identifies every country represented by the resume's work entries. Each country node MUST display a readable country label, and the visualization MUST distinguish country nodes from company nodes.

#### Scenario: Resume contains entries from multiple countries
- **WHEN** the user opens Experience by Country for a resume whose work entries belong to multiple countries
- **THEN** the visualization displays a distinct, readable node for each represented country

#### Scenario: Work entry has no usable country
- **WHEN** a work entry has no usable location country
- **THEN** the visualization groups it under a readable `Unknown` country node

### Requirement: Expandable country companies
The application MUST let the user expand a country node to reveal its associated companies as connected company nodes, and MUST let the user collapse the node to hide those company nodes while preserving the country node.

#### Scenario: Expand a country
- **WHEN** the user activates a collapsed country node
- **THEN** the application displays the companies associated with that country and their connecting edges

#### Scenario: Collapse a country
- **WHEN** the user activates an expanded country node
- **THEN** the application hides that country's company nodes and their connecting edges while retaining the country node

### Requirement: Desktop and browser availability
The country graph and its expand/collapse interaction MUST be available in both the desktop application and the browser application published for Rustume. The browser application MUST render an equivalent usable graph without requiring users to change browser security settings or run a separate local host.

#### Scenario: Use the desktop application
- **WHEN** the application runs on a supported desktop platform
- **THEN** the user can view and expand or collapse the Experience by Country graph

#### Scenario: Use the published browser application
- **WHEN** the user opens the published Rustume site in a supported current browser
- **THEN** the user can view and expand or collapse the Experience by Country graph