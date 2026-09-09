## MODIFIED Requirements

### Requirement: Detailed skill groups
The Skills view MUST show every named skill with its level when present and its complete keyword list. The level indicator MUST remain compact and sized to its label rather than expanding to the size of the skill name.

#### Scenario: Compact skill level indicator
- **WHEN** a skill has a name, level, and keywords
- **THEN** the view shows the name, a compact readable level bubble sized to the level text, and readable keyword values

#### Scenario: Skill with level and keywords
- **WHEN** a skill has a name, level, and keywords
- **THEN** the view shows the name, level, and readable keyword values

## ADDED Requirements

### Requirement: Skill hierarchy
The Skills view MUST represent each keyword or sub-skill as a separate child node beneath its parent skill instead of joining sub-skills into one delimiter-separated text node.

#### Scenario: Skill with sub-skills
- **WHEN** a named skill contains multiple keywords
- **THEN** the view shows one parent skill node with one distinct child node per keyword, and each child remains individually readable and selectable

#### Scenario: Skill without sub-skills
- **WHEN** a named skill has no keywords
- **THEN** the view shows the skill as a standalone node without an empty child collection