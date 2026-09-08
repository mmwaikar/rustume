## MODIFIED Requirements

### Requirement: Profile supporting card grid
The Profile view MUST place available Education, Publications, Network, and Languages content in individually framed GroupBoxes arranged in this order: Education and Publications in the first row, then Network and Languages in the second row on wide screens. Each GroupBox MUST provide the only visible heading for its content, and the groups MUST stack without clipping on narrow panes.

#### Scenario: Four supporting groups present
- **WHEN** education, publications, network, and language data exist
- **THEN** the first row contains Education followed by Publications, the second row contains Network followed by Languages, and no group displays a duplicate inner heading

#### Scenario: Supporting content present
- **WHEN** education, publications, network, and language data exist
- **THEN** each group appears in a separate framed GroupBox with a clear heading and source links where available

#### Scenario: Partial supporting groups
- **WHEN** only some supporting arrays contain entries
- **THEN** the remaining groups retain the same relative row order without empty placeholder groups or misleading headings

## ADDED Requirements

### Requirement: Education data grid
The Education GroupBox MUST present each education entry in a readable data-grid-style layout with consistent columns or aligned fields for institution, study details, dates, and area, while preserving all available source values.

#### Scenario: Multiple education entries
- **WHEN** the resume contains one or more education entries
- **THEN** each entry appears as a distinct row with its institution, study type/details, date range, and area identifiable without parsing a concatenated paragraph

### Requirement: Network inline rows
The Network GroupBox MUST render each network name and its linked username or value on the same visual line whenever the pane has sufficient width, with the link remaining usable.

#### Scenario: Social profiles present
- **WHEN** the resume contains LinkedIn, GitHub, or other network profiles
- **THEN** each profile shows its network label and linked value inline in one row, without placing the value on a separate line

### Requirement: Language fluency bubbles
The Languages GroupBox MUST render each language name with its fluency as a small Bubble control, rather than as a plain hyphenated text string.

#### Scenario: Languages present
- **WHEN** the resume contains language entries with fluency values
- **THEN** each language is readable and its fluency is visually represented by a Bubble control

## MODIFIED Requirements

### Requirement: Profile responsive layout
The Profile hero and supporting groups MUST remain readable on narrow viewports without horizontal clipping or overlapping contact content.

#### Scenario: Mobile profile
- **WHEN** the viewport is narrow
- **THEN** hero columns stack, supporting groups become one column, network values may wrap gracefully, and all text remains readable
