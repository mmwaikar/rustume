# profile-view Specification

## Purpose

Present the candidate's complete personal and professional profile from JSON Resume in a readable Compendium-style section without inventing or silently dropping source data.

## Requirements

### Requirement: Profile identity and summary
The Profile view MUST show the candidate name, headline, summary, location, and available contact details from `basics`.

#### Scenario: Populated profile
- **WHEN** the resume contains name, label, summary, and location
- **THEN** Profile shows those values together as the primary identity block

### Requirement: Profile links
The Profile view MUST render available website, email, phone, and social profile links as usable links, preserving their labels and URLs.

#### Scenario: Network profiles
- **WHEN** `basics.profiles` contains LinkedIn, GitHub, or other networks
- **THEN** Profile shows each available network as a labeled link

### Requirement: Supporting profile sections
Profile MUST present available education, publications, languages, and interests using their source fields, and MUST omit empty subsections rather than displaying fabricated placeholders.

#### Scenario: Education and publications
- **WHEN** the resume contains education and publication entries
- **THEN** Profile shows institution, study details, dates, publication title, publisher, date, summary, and source URL where present

#### Scenario: Empty supporting data
- **WHEN** a supporting array is empty
- **THEN** its subsection is not rendered as populated content
