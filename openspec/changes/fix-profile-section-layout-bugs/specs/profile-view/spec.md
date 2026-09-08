## MODIFIED Requirements

### Requirement: Supporting profile sections
Profile MUST present available education, publications, languages, and interests using their source fields, MUST omit empty subsections rather than displaying fabricated placeholders, and MUST visually group Education, Publications, Network, and Languages in individually framed GroupBoxes. The Publications group MUST exclude entries identified as WordPress blog publications.

#### Scenario: Education and filtered publications
- **WHEN** the resume contains education entries, ordinary publications, and WordPress blog publications
- **THEN** Profile shows the education entries and only the ordinary publications in their corresponding grouped sections, preserving publisher, date, summary, and source URL data for each displayed publication

#### Scenario: Empty or blog-only publications
- **WHEN** the publication collection is empty after excluding WordPress blog entries
- **THEN** the Profile view omits the Publications GroupBox instead of showing an empty or blog-only group

#### Scenario: Education and publications
- **WHEN** the resume contains education and publication entries
- **THEN** Profile shows institution, study details, dates, publication title, publisher, date, summary, and source URL where present within their corresponding grouped sections

#### Scenario: Empty supporting data
- **WHEN** a supporting array is empty
- **THEN** its subsection is not rendered as populated content or an empty framed group

## ADDED Requirements

### Requirement: Publication title links and descriptions
Each displayed publication with a source URL MUST render its title as the usable source link. Its description or summary MUST be available through a popover associated with the publication, and MUST NOT require a separate visible `Open publication` action or permanently consume the row's primary layout space.

#### Scenario: Publication with URL and summary
- **WHEN** an ordinary publication has a title, URL, and summary
- **THEN** activating the title opens the publication URL, and the description can be revealed through the associated popover

#### Scenario: Publication without URL or summary
- **WHEN** an ordinary publication lacks a URL or summary
- **THEN** the title remains plain when no URL exists, no broken link is rendered, and no empty popover affordance is shown when no description exists

### Requirement: Profile links
The Profile view MUST render available website, email, phone, and social profile links as usable links, preserving their labels and URLs.

#### Scenario: Network profiles
- **WHEN** `basics.profiles` contains LinkedIn, GitHub, or other networks
- **THEN** Profile shows each available network as a labeled link in the Network supporting group
