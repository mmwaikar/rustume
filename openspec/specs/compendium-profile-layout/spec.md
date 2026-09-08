# compendium-profile-layout Specification

## Purpose

Define the screenshot-aligned visual composition for the Profile section while preserving candidate data and links supplied by JSON Resume.

## Requirements

### Requirement: Profile hero composition
The Profile view MUST present a rounded hero panel using the current Rustume palette with a section eyebrow, large candidate name, headline claims rendered as Bubble controls, a responsive two-column identity/contact block, and a readable summary area that wraps within its container.

#### Scenario: Populated profile screenshot match
- **WHEN** the Profile section is opened with the actual resume
- **THEN** the candidate name is visually dominant, each headline is a separate Bubble control, contact/location information is grouped beside the identity on wide screens, and the summary remains readable without clipping

### Requirement: Supporting group arrangement
The Profile view MUST present available Education, Publications, Network, and Languages content in two responsive rows on wide screens: Education and Publications in the first row, and Network and Languages in the second row.

#### Scenario: Supporting groups visible
- **WHEN** education, publications, network, and language data exist
- **THEN** the layout arranges the groups in the requested order and keeps them readable without overlapping or reordering unexpectedly

### Requirement: Group identity and frame clarity
Each supporting section MUST appear inside a single GroupBox frame with one clear visible title and no duplicated internal label text.

#### Scenario: Section headings
- **WHEN** the profile renders Education, Publications, Network, or Languages
- **THEN** each section has one visible heading and does not display a second, redundant title inside the content area

### Requirement: Education table readability
The Education section MUST render as a compact data-grid-style layout with labels for institution, study, dates, and area, followed by aligned row entries for each education record.

#### Scenario: Education rows
- **WHEN** the resume contains one or more education entries
- **THEN** the values remain aligned and source-derived, while missing fields remain empty rather than being invented or duplicated

### Requirement: Inline network and language metadata
Network and language content MUST remain visually compact and readable, with network label/value pairs on the same line and language fluency rendered using the app’s Bubble control.

#### Scenario: Inline metadata
- **WHEN** the resume contains network or language entries
- **THEN** each network label and value stay on a single visible row and each fluency value appears as a small bubble matching the app’s existing styling

### Requirement: Profile responsive layout
The Profile hero and supporting groups MUST remain readable on narrow viewports without horizontal clipping or overlapping contact content.

#### Scenario: Mobile profile
- **WHEN** the viewport is narrow
- **THEN** hero columns stack, supporting groups become one column, and all text remains readable
