## Purpose

Define the screenshot-aligned Blog Posts hero and compact publication-card grid using only WordPress publication data already present in JSON Resume.

## ADDED Requirements

### Requirement: Blog Posts hero composition
The Blog Posts view MUST begin with a rounded hero panel using the current Rustume palette, containing a section eyebrow, the heading `Musings on experiences in life and programming.`, supporting copy, and a usable WordPress source link.

#### Scenario: Blog hero screenshot match
- **WHEN** the Blog Posts section is opened with WordPress publications
- **THEN** the hero communicates the blog purpose and exposes the WordPress source link in the supporting copy

### Requirement: Blog post card grid
Each WordPress publication MUST render as a compact rounded white card in a two-column wide-screen grid, with a prominent linked title and readable publication date.

#### Scenario: Populated blog grid
- **WHEN** WordPress publications exist
- **THEN** every publication appears once with its title, source URL, and release date

### Requirement: Empty blog layout
When no WordPress-derived data exists, the section MUST preserve the shared hero treatment and show a clear empty state without fabricated cards.

#### Scenario: No blog posts
- **WHEN** the resume has no WordPress publications or profile
- **THEN** the view shows no invented post cards and remains visually coherent
