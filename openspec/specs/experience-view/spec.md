# experience-view Specification

## Purpose

Defines a readable company experience chart that preserves newest-first ordering while making country differences visible through matching bar colors and an accurate chart legend.

## Requirements

### Requirement: Country-colored experience bars and legend
The experience view MUST render each company bar using the color assigned to that company's country, and MUST show a legend within the chart presentation that maps every displayed country, including unknown locations, to the same color used by its bars. The legend MUST be colocated with the chart rather than presented as an unrelated section below it.

#### Scenario: Companies from multiple countries
- **WHEN** the experience data contains companies associated with different countries
- **THEN** bars for different countries use visibly different colors and the chart legend identifies each country with its matching color

#### Scenario: Unknown country is present
- **WHEN** a company has no usable country or location value
- **THEN** its bar uses the dedicated unknown-country color and the legend includes an `Unknown` entry with that same color

#### Scenario: Newest-first ordering is retained
- **WHEN** the experience view contains companies with different latest work start dates
- **THEN** companies appear from newest latest start date to oldest while their country colors and legend mapping remain correct

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

### Requirement: Navigation to country experience visualization
The experience area MUST provide an Experience by Country view in addition to the Experience by Company chart. Selecting the country view MUST present the country-grouped experience visualization rather than only a textual count or list of graph records.

#### Scenario: Open country experience view
- **WHEN** the user selects Experience by Country from the application navigation
- **THEN** the application displays the interactive country-grouped experience visualization
