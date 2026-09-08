# profile-view Specification

## Purpose

Define the expected publication and supporting-content behavior for the Profile page so resume data is shown clearly and without redundant or misaligned UI elements.

## MODIFIED Requirements

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
