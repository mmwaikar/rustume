## MODIFIED Requirements

### Requirement: Profile identity and summary
The Profile view MUST show the candidate name, headline claims, summary, location, and available contact details from `basics`. Headline claims MUST be presented as distinct badge-like controls, the summary MUST wrap within the available content width, and all displayed profile text MUST be selectable.

#### Scenario: Select populated profile text
- **WHEN** the resume contains name, claims, summary, location, and contact details and a visitor drags across the Profile view
- **THEN** the visible identity, claims, summary, and metadata can be selected and copied without clipping or replacing the profile layout

#### Scenario: Populated profile
- **WHEN** the resume contains name, label, summary, location, and contact details
- **THEN** Profile shows the name and headline claims as the primary identity block, displays each claim as a separate badge-like control, places contact/location beside the identity on wide screens, and wraps the summary without clipping

### Requirement: Profile links
The Profile view MUST render available website, email, phone, and social profile links as usable links, preserving their labels and URLs, while keeping their visible labels selectable.

#### Scenario: Select network labels
- **WHEN** `basics.profiles` contains LinkedIn, GitHub, or other networks
- **THEN** Profile shows each available network as a labeled link in the Network supporting group and the label text can be selected

#### Scenario: Network profiles
- **WHEN** `basics.profiles` contains LinkedIn, GitHub, or other networks
- **THEN** Profile shows each available network as a labeled link in the Network supporting group