## Purpose

Defines a readable company experience chart that preserves newest-first ordering while making country differences visible through matching bar colors and an accurate chart legend.

## ADDED Requirements

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
