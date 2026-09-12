## MODIFIED Requirements

### Requirement: Detailed skill groups
The Skills view MUST show every named skill with its level when present and its complete keyword list. Skill levels MUST use the compact bubble presentation so they remain visually subordinate to the skill name.

#### Scenario: Skill with level and keywords
- **WHEN** a skill has a name, level, and keywords
- **THEN** the view shows the name, level in a compact bubble, and readable keyword values

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

## ADDED Requirements

### Requirement: Distinct sub-skill nodes
The skills visualization MUST represent each distinct keyword/sub-skill belonging to a named skill as its own child node, with a relationship connecting that child to the parent skill. Repeated keywords differing only by letter case MUST remain one child for that parent.

#### Scenario: Skill with multiple sub-skills
- **WHEN** a skill has multiple keyword values
- **THEN** each distinct keyword is visible as a separate child node connected to that skill

#### Scenario: Duplicate keyword casing
- **WHEN** a skill contains the same keyword in different letter casing
- **THEN** the visualization shows one child node for that keyword under the skill