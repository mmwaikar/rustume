# Data Model: Resume Visualization

## Core entities

### ResumeProfile
Represents the full resume that will be rendered.

- `basics`: person metadata such as name, label, email, phone, url, location
- `work`: list of professional experience entries
- `skills`: list of competencies
- `projects`, `education`, `languages`, `awards`, `publications`, `references`, `volunteer`: optional supporting data for future expansion

### ExperienceEntry
Represents an individual role or employment period.

- `company`: employer name
- `position`: role title
- `start_date` / `end_date`: parsed timeline values
- `summary`: short description
- `highlights`: bullet-like achievements
- `location`: optional location or country value
- `country`: derived from location when available

### Company
Represents a unique employer in the resume.

- `name`
- `country`
- `experience_count`
- `total_duration_months`
- `roles`: linked experience entries

### Country
Represents a geographic context used to relate companies and experience.

- `name`
- `companies`: linked company entities
- `experience_count`

### Skill
Represents a capability or proficiency area.

- `name`
- `level`: optional proficiency value
- `keywords`: optional related terms
- `related_skills`: derived relationships used by the graph view

## Derived view models

### ExperienceBar
Used by the bar-chart visualization.

- `company_name`
- `position`
- `duration_months`
- `start_month`
- `end_month`
- `country`

### GraphNode
Used by the Cytoscape graph.

- `id`
- `label`
- `group` (`company`, `country`, `skill`)
- `size`

### GraphEdge
Used by the Cytoscape graph.

- `source`
- `target`
- `label`
- `weight`

## Validation rules

- The resume must contain a valid JSON object with `basics` and at least one `work` entry or one `skills` entry.
- Each work experience should have a company or organization name when possible.
- If a country is missing, the system should still render the entity and assign a neutral fallback label such as `Unknown`.
- The graph must remain valid even when some skills are isolated and have no edges.
