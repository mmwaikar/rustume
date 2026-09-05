## Purpose

Load a JSON Resume document, keep its meaning intact, and derive tenure, country, and skill relationships the visualizer can display without editing the source file.

## ADDED Requirements

### Requirement: JSON Resume document is the source of truth
The system MUST ingest a single static JSON document that conforms to the public JSON Resume schema for the fields it uses (`basics`, `work`, `skills`, and other present sections). The system MUST NOT invent resume facts that are absent from that document.

#### Scenario: Valid resume loads
- **WHEN** a well-formed JSON Resume file is available to the application
- **THEN** the visualizer presents the candidate identity from `basics` and the work and skill content from the file

#### Scenario: Invalid JSON is rejected
- **WHEN** the resume file is missing, not JSON, or not an object
- **THEN** the system MUST NOT render fabricated career data and MUST show a recoverable error state that the file could not be loaded

### Requirement: Work tenure is measured in whole months
The system MUST compute duration in whole months for each work entry from `startDate` and `endDate`. A missing `endDate` MUST be treated as ongoing through the current calendar month. Partial months MUST be counted using calendar month boundaries (inclusive start month through inclusive end month).

#### Scenario: Closed role
- **WHEN** a work entry has `startDate` 2020-01 and `endDate` 2020-03
- **THEN** the derived duration for that entry is 3 months

#### Scenario: Current role
- **WHEN** a work entry has a start date and no end date
- **THEN** duration includes every calendar month from the start month through the current month

### Requirement: Country is derived without dropping companies
The system MUST derive a country label from work location text or structured location when present. When country cannot be determined, the company and role MUST still appear in derived views with a explicit fallback country label of `Unknown`.

#### Scenario: Location present
- **WHEN** a work entry includes a location that names or implies a country
- **THEN** that country is associated with the company for geography views

#### Scenario: Location missing
- **WHEN** a work entry has a company name but no usable country
- **THEN** the company still appears in aggregations and graphs, tagged `Unknown`

### Requirement: Skills remain visible when isolated
The system MUST include every named skill from the resume in derived skill views. Skills with no inferred relationship to other skills MUST remain present as isolated nodes (no required edges).

#### Scenario: Isolated skill
- **WHEN** a skill has a name and no keywords or other skills to relate to
- **THEN** the skill still appears in the skill graph as a standalone node

### Requirement: Company tenure aggregation
The system MUST aggregate duration in months by company name so overlapping or sequential roles at the same employer contribute to that company’s total. Distinct companies MUST remain separate series.

#### Scenario: Two roles at one company
- **WHEN** two work entries share the same company name and have non-overlapping dates
- **THEN** the company total months equals the sum of both entries’ month durations
