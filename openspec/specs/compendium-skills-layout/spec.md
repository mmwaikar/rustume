# compendium-skills-layout Specification

## Purpose

Define the screenshot-aligned expandable Skills presentation with compact level badges and readable keyword detail.

## Requirements

### Requirement: Skills hero composition
The Skills view MUST begin with a rounded hero panel using the current Rustume palette, containing a section eyebrow, the heading `Technical strengths`, and supporting copy describing the skill relationships.

#### Scenario: Skills screenshot match
- **WHEN** the Skills section is opened
- **THEN** the hero establishes the same title hierarchy and spacing as the reference screenshot without changing the current Rustume palette

### Requirement: Expandable skill rows
Each named skill MUST render as a row with a disclosure affordance, skill name, and level badge; activating a row MUST expand or collapse its keyword list.

#### Scenario: Expand a skill
- **WHEN** a visitor activates a collapsed skill row
- **THEN** its keywords appear as indented readable rows beneath the skill while other skill rows remain available

### Requirement: Skills density and responsiveness
The Skills list MUST use compact spacing and stable row dimensions matching the reference density, and MUST remain usable on narrow viewports.

#### Scenario: Narrow skills view
- **WHEN** the viewport is narrow
- **THEN** skill names, badges, and keywords wrap without clipping or overlap
