# profile-view Specification

## Purpose

Present the candidate's complete personal and professional profile from JSON Resume in a readable Compendium-style section without inventing or silently dropping source data.

## Requirements

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

### Requirement: Publication filtering
The Profile view MUST exclude WordPress publications from the ordinary Publications section while retaining the separate WordPress-derived Blog Posts destination behavior.

#### Scenario: WordPress publications present
- **WHEN** the resume contains a publication whose publisher is WordPress
- **THEN** that publication is omitted from the Profile Publications section and does not appear as a separate profile entry

### Requirement: Publication title as source link
When a publication contains a URL, the visible title MUST act as the link target instead of showing a separate action label or duplicate link.

#### Scenario: Linked publication title
- **WHEN** a non-WordPress publication includes a URL
- **THEN** the title itself opens the source and the UI shows no extra standalone publication link beside it

### Requirement: Publication summary in popover
The Profile view MUST show a publication summary in a popover when a summary exists, rather than rendering a fixed text block next to the title or date.

#### Scenario: Summary available
- **WHEN** a publication has a non-empty summary
- **THEN** the summary appears in a popover triggered by the title or card affordance, and the inline text remains compact and aligned

### Requirement: Metadata formatting stability
The publication metadata line MUST present publisher and date in a single line with a normalized human-readable date format such as `Mon Year` or `Month Year`.

#### Scenario: Publication metadata alignment
- **WHEN** the resume contains publication metadata
- **THEN** the publisher and date appear on the same line and the date remains readable without the center/left misalignment seen in the prior implementation

### Requirement: Supporting profile sections
Profile MUST present available education, publications, languages, and interests using their source fields, and MUST omit empty subsections rather than displaying fabricated placeholders. Education, Publications, Network, and Languages MUST be visually grouped in individually framed GroupBoxes.

#### Scenario: Education and publications
- **WHEN** the resume contains education and publication entries
- **THEN** Profile shows institution, study details, dates, publication title, publisher, date, summary, and source URL where present within their corresponding grouped sections

#### Scenario: Empty supporting data
- **WHEN** a supporting array is empty
- **THEN** its subsection is not rendered as populated content or an empty framed group
