## Purpose

Expose the candidate's practical project work as a first-class Compendium-style destination with direct links to source repositories or project pages.

## ADDED Requirements

### Requirement: Project listing
The Projects view MUST show every named JSON Resume project with its name, description, highlights, and URL when present.

#### Scenario: Populated projects
- **WHEN** the resume contains project entries
- **THEN** each project is readable as a distinct item with its source link and highlights

### Requirement: Project links
Project URLs MUST be rendered as usable external links and MUST preserve the URL from the resume.

#### Scenario: Project repository
- **WHEN** a project has a GitHub URL
- **THEN** activating the project link opens that URL without changing the resume data

### Requirement: Empty projects
When the resume has no project entries, the Projects destination MUST be omitted from primary navigation or show a clear no-projects state, but MUST NOT display invented projects.

#### Scenario: No project data
- **WHEN** `projects` is empty
- **THEN** no fabricated project cards or links appear
