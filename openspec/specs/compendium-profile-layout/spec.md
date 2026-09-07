# compendium-profile-layout Specification

## Purpose

Define the screenshot-aligned visual composition for the Profile section while preserving candidate data and links supplied by JSON Resume.

## Requirements

### Requirement: Profile hero composition
The Profile view MUST present a rounded hero panel using the current Rustume palette with a section eyebrow, large candidate name, headline claims rendered as Bubble controls, a responsive two-column identity/contact block, and a readable summary area that wraps within its container.

#### Scenario: Populated profile screenshot match
- **WHEN** the Profile section is opened with the actual resume
- **THEN** the candidate name is visually dominant, each headline is a separate Bubble control, contact/location information is grouped beside the identity on wide screens, and the summary remains readable without clipping

### Requirement: Profile supporting card grid
The Profile view MUST place available Education, Publications, Network, and Languages content in individually framed GroupBoxes arranged in a responsive two-column layout on wide screens.

#### Scenario: Supporting content present
- **WHEN** education, publications, network, and language data exist
- **THEN** each group appears in a separate framed GroupBox with a clear heading and source links where available

### Requirement: Profile responsive layout
The Profile hero and supporting groups MUST remain readable on narrow viewports without horizontal clipping or overlapping contact content.

#### Scenario: Mobile profile
- **WHEN** the viewport is narrow
- **THEN** hero columns stack, supporting groups become one column, and all text remains readable
