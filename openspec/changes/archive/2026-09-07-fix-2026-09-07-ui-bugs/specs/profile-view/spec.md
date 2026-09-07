## MODIFIED Requirements

### Requirement: Profile identity and summary
The Profile view MUST show the candidate name, headline claims, summary, location, and available contact details from `basics`. Headline claims MUST be presented as distinct badge-like controls, and the summary MUST wrap within the available content width.

#### Scenario: Populated profile
- **WHEN** the resume contains name, label, summary, location, and contact details
- **THEN** Profile shows the name and headline claims as the primary identity block, displays each claim as a separate badge-like control, places contact/location beside the identity on wide screens, and wraps the summary without clipping

### Requirement: Profile links
The Profile view MUST render available website, email, phone, and social profile links as usable links, preserving their labels and URLs.

#### Scenario: Network profiles
- **WHEN** `basics.profiles` contains LinkedIn, GitHub, or other networks
- **THEN** Profile shows each available network as a labeled link in the Network supporting group

### Requirement: Supporting profile sections
Profile MUST present available education, publications, languages, and interests using their source fields, and MUST omit empty subsections rather than displaying fabricated placeholders. Education, Publications, Network, and Languages MUST be visually grouped in individually framed GroupBoxes.

#### Scenario: Education and publications
- **WHEN** the resume contains education and publication entries
- **THEN** Profile shows institution, study details, dates, publication title, publisher, date, summary, and source URL where present within their corresponding grouped sections

#### Scenario: Empty supporting data
- **WHEN** a supporting array is empty
- **THEN** its subsection is not rendered as populated content or an empty framed group
