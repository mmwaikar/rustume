## Purpose

Give visitors a Compendium-like reading frame: persistent chrome, a navigable left sidebar, and a main pane that shows one resume section or visualization at a time.

## ADDED Requirements

### Requirement: Persistent application chrome
The visualizer MUST show a header, a footer, a left sidebar, and a main content area on the primary desktop layout. Header MUST identify the product or candidate at a glance. Footer MUST remain visible with supporting attribution or copyright-style text.

#### Scenario: First open
- **WHEN** a visitor opens the visualizer with a valid resume
- **THEN** they see header, footer, left sidebar, and main content together without opening extra windows

### Requirement: Sidebar navigation of resume sections
The left sidebar MUST list primary destinations that cover at least profile/overview, experience (including the tenure chart), skills (including the skills graph), and geography/company-country graph. Selecting a destination MUST replace the main pane with that destination’s content. The sidebar MUST remain visible while the main pane changes.

#### Scenario: Switch from overview to experience
- **WHEN** the visitor selects Experience in the sidebar
- **THEN** the main pane shows experience content including the tenure chart and the sidebar stays on screen

### Requirement: Resizable split between sidebar and main
Sidebar and main content MUST be separated by a vertical splitter that the visitor can drag to change relative widths. Neither pane MUST collapse to zero width through normal splitter use; a usable minimum width MUST remain for both.

#### Scenario: Drag splitter
- **WHEN** the visitor drags the splitter toward the main pane
- **THEN** the sidebar grows, the main pane shrinks, and both remain usable

### Requirement: Visual fidelity to Compendium
The layout MUST be recognizably similar to [Compendium](https://www.codionics.com/Compendium/): dense professional chrome, left navigation, and a content-forward main column. The system MUST NOT require the visitor to use a plain linear resume dump as the only view.

#### Scenario: Scan in ten seconds
- **WHEN** a visitor opens a populated resume
- **THEN** they can identify whose resume it is and reach the tenure or graph views without reading a wall of unstructured text first

### Requirement: Keyboard access to primary navigation
Primary sidebar destinations MUST be reachable with keyboard focus and activation. Focus MUST not be trapped in the sidebar after activation.

#### Scenario: Keyboard section change
- **WHEN** a keyboard user focuses a sidebar destination and activates it
- **THEN** the main pane shows that destination and focus remains usable in the window
