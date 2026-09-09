## Purpose

Present work history as a compact visual comparison that preserves company names and makes relative experience duration easy to scan.

## ADDED Requirements

### Requirement: Vertical experience chart
The Experience view MUST render each company with a bottom-aligned vertical bar whose height represents its relative number of months, with rounded corners and a visible month value.

#### Scenario: Companies with varied durations
- **WHEN** the resume contains work entries for multiple companies with different calculated durations
- **THEN** the view shows one vertical bar per company aligned to a common baseline, with taller bars representing longer durations and rounded bar corners

### Requirement: Readable company labels
The Experience view MUST display every company name associated with a bar and MUST keep long names readable by allowing diagonal label orientation when horizontal space is insufficient.

#### Scenario: Long company name
- **WHEN** a company name is too long for the available chart label width
- **THEN** its full name remains available as a diagonally tilted readable label without overlapping another company label

### Requirement: Selectable chart text
Company names, month values, and the chart heading MUST be selectable text.

#### Scenario: Copy chart data
- **WHEN** a visitor drags across an experience label or value
- **THEN** the selected chart text can be copied