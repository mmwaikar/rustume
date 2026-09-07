## MODIFIED Requirements

### Requirement: WordPress publication derivation
The Blog Posts view MUST derive entries from publications whose publisher is `WordPress`, without fetching or inventing external post data.

#### Scenario: WordPress publications
- **WHEN** the resume contains WordPress publications with names and URLs
- **THEN** each appears as a blog post item with title, summary when present, source URL, and a date formatted as either `Mon Year` or `Month Year`

### Requirement: Blog source profile
When a WordPress profile exists in `basics.profiles`, the Blog Posts view MUST provide a labeled link to that profile or blog homepage, and the supporting introductory copy MUST remain a single readable line when the available width permits.

#### Scenario: WordPress profile link
- **WHEN** a profile has network `WordPress`
- **THEN** the view exposes its URL as the blog source link and presents the surrounding introductory sentence without unnecessary wrapping

### Requirement: Empty blog data
When no WordPress publications or WordPress profile exists, the Blog Posts destination MUST be omitted or show a clear no-blog-posts state, without calling a remote feed.

#### Scenario: No blog source
- **WHEN** the resume contains no WordPress-derived data
- **THEN** no fabricated blog posts appear and the app remains usable
