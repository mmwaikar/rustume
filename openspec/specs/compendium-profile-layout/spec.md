# compendium-profile-layout Specification

## Purpose

Define the screenshot-aligned visual composition for the Profile section while preserving candidate data and links supplied by JSON Resume.

## Requirements

### Requirement: Profile hero composition
The Profile view MUST present a rounded hero panel using the current Rustume palette with a section eyebrow, large candidate name, headline pills, a right-side contact/location block, and a readable summary area.

#### Scenario: Populated profile screenshot match
- **WHEN** the Profile section is opened with the actual resume
- **THEN** the candidate name is visually dominant, each headline is a separate pill, contact/location information is grouped on the right, and the summary is readable within the palette-preserving hero panel

### Requirement: Profile supporting card grid
The Profile view MUST place available Education, Publications, Network, and Languages content in individually framed cards arranged in a responsive two-column layout on wide screens.

#### Scenario: Supporting content present
- **WHEN** education, publications, network, and language data exist
- **THEN** each group appears in a separate rounded white card with a clear heading and source links where available

### Requirement: Profile responsive layout
The Profile hero and supporting cards MUST remain readable on narrow viewports without horizontal clipping or overlapping contact content.

#### Scenario: Mobile profile
- **WHEN** the viewport is narrow
- **THEN** hero columns stack and supporting cards become one column while all text remains readable
