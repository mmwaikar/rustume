## ADDED Requirements

### Requirement: Consistent default link styling
All user-facing links in the visualizer MUST use the default blue link styling consistently, including footer technology links, profile and network links, project links, publication links, and blog-post links. Link styling MUST remain visually distinct from surrounding selectable text while preserving each link's existing destination and activation behavior.

#### Scenario: Links appear across multiple views
- **WHEN** a visitor opens a profile, projects, publications, blog-post, or footer view containing links
- **THEN** every visible link uses the same default blue link color and remains distinguishable from non-link text

#### Scenario: Link destinations remain unchanged
- **WHEN** a visitor activates any styled link
- **THEN** the link opens its existing destination without changing URL or navigation behavior
