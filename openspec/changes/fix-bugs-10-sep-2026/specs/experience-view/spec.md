## Purpose

Presents work history by company in a readable visual comparison that remains usable when company names are long or the available content width is constrained.

## ADDED Requirements

### Requirement: Readable company experience chart
The experience view MUST use the GPUI Kit chart component to compare each company by its calculated duration in months, using rounded bars and a common baseline. Company labels MUST remain readable and MUST NOT be forced into a vertically stacked layout that makes long names overlap or obscure the chart; the view MUST use a horizontal or side-by-side/in-bar label arrangement when the vertical arrangement cannot fit.

#### Scenario: Long company names in a wide view
- **WHEN** the experience view contains company names that do not fit beneath their vertical bars without overlap
- **THEN** the view presents the names in a readable horizontal, side-by-side, or in-bar arrangement while retaining the duration bars

#### Scenario: Company durations are compared
- **WHEN** the experience view contains multiple companies with different calculated month totals
- **THEN** each company is represented by a rounded bar whose size is proportional to its duration against the same chart maximum

#### Scenario: No company experience is available
- **WHEN** the resume contains no work history with chartable duration
- **THEN** the view shows an explicit empty-state message instead of an empty or misleading chart
