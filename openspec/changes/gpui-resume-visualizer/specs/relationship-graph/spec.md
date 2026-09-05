## Purpose

Show how companies relate to countries and how skills relate to each other as node–edge graphs so visitors can see geography and capability structure at a glance.

## ADDED Requirements

### Requirement: Country graph
The system MUST provide a node–edge view whose nodes include companies and countries, with edges connecting a company to the country derived for its work. Companies tagged `Unknown` MUST still appear.

#### Scenario: Company in a known country
- **WHEN** a company has a derived country of Germany
- **THEN** the graph contains a company node, a country node labeled Germany, and an edge between them

#### Scenario: Unknown country
- **WHEN** a company has fallback country `Unknown`
- **THEN** that company node remains visible and is connected to an `Unknown` country node (or labeled equivalently) without omitting the company

### Requirement: Skills graph
The system MUST provide a node–edge view of skills. Edges MUST represent relationships inferred from the resume (shared keywords, shared work context, or explicit keyword overlap). Isolated skills MUST appear as nodes without edges.

#### Scenario: Related skills
- **WHEN** two skills share a keyword or other documented relationship rule
- **THEN** both nodes appear and an edge connects them

#### Scenario: Isolated skill remains
- **WHEN** a skill has no inferred relationship
- **THEN** it still appears as a node and the rest of the graph still renders

### Requirement: Graph payload contract
Rust MUST own a portable graph payload: nodes with stable `id`, display `label`, and `group` (`company`, `country`, or `skill`), and edges with `source`, `target`, and optional `label`. Any renderer (native graph canvas or a JavaScript graph library) MUST consume this payload rather than re-parsing the raw resume in the host language.

#### Scenario: Payload completeness
- **WHEN** the resume has at least one company and one skill
- **THEN** the derived payload contains those entities as nodes and only edges whose endpoints exist in the node set

### Requirement: Interactive inspection
Visitors MUST be able to pan and zoom the graph (or equivalent inspection) so overlapping labels can be read. Selecting a node MUST not crash the visualizer; if details are shown, they MUST come from the resume-derived payload.

#### Scenario: Pan and zoom
- **WHEN** the visitor pans or zooms a populated graph
- **THEN** nodes and edges remain in a consistent topology and stay visible according to the viewport
