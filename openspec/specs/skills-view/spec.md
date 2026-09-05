# skills-view Specification

## Purpose

Make the candidate's named skills useful to inspect by exposing levels, keywords, and relationships in a dedicated view.

## Requirements

### Requirement: Detailed skill groups
The Skills view MUST show every named skill with its level when present and its complete keyword list.

#### Scenario: Skill with level and keywords
- **WHEN** a skill has a name, level, and keywords
- **THEN** the view shows the name, level, and readable keyword values

### Requirement: Isolated skills remain visible
Skills with no keywords or no inferred relationship MUST remain visible as standalone skill entries.

#### Scenario: Isolated skill
- **WHEN** a skill has no relationship to another skill
- **THEN** it remains visible and the view does not fail or hide it

### Requirement: Skill navigation
Selecting a skill or skill group MUST reveal its related keywords and relationship context without replacing the persistent application chrome.

#### Scenario: Inspect a skill
- **WHEN** a visitor activates a skill entry
- **THEN** its details are shown in the main content area and sidebar navigation remains available
