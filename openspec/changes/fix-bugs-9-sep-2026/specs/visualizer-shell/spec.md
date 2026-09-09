## MODIFIED Requirements

### Requirement: Sidebar icon visibility
The visualizer MUST render a recognizable asset-backed icon for every sidebar destination, alongside its label and current selection, without exposing redundant shell identity labels.

#### Scenario: Sidebar icons are visible
- **WHEN** the resume shell loads and the sidebar is visible
- **THEN** every displayed navigation destination has a recognizable icon glyph, readable label, and visible current selection consistent with the app palette and sizing

#### Scenario: Sidebar icons are unreadable
- **WHEN** the resume shell loads and the sidebar is visible
- **THEN** the navigation icon, its label, and the current selection remain clearly visible and consistent with the app's palette and sizing

## ADDED Requirements

### Requirement: Selectable visualizer text
User-facing text throughout the shell and active content view MUST be selectable, including headings, labels, summaries, metadata, chart labels, and skill names.

#### Scenario: Select text in any view
- **WHEN** a visitor drags across visible text in the header, sidebar, footer, or active content view
- **THEN** the text can be selected and copied without requiring a separate export mode

### Requirement: Themed application typography
The visualizer MUST use the configured Inter font for normal and monospaced text roles and MUST apply the shared application theme consistently across shell and content controls.

#### Scenario: Inter theme is loaded
- **WHEN** the application starts with bundled font assets available
- **THEN** visible shell and content text uses Inter with the configured theme sizing and does not fall back to the previous oversized default typography

### Requirement: Status footer
The application shell MUST present its persistent footer using StatusBar-style chrome and MUST keep the footer visible as part of the shell while the content pane scrolls independently.

#### Scenario: Scroll content with footer present
- **WHEN** active content exceeds the viewport and the visitor scrolls the content pane
- **THEN** the footer remains shell chrome outside the scrolling content and presents the application status text in a StatusBar-style layout

### Requirement: Maximized startup
The desktop visualizer MUST open its initial application window maximized.

#### Scenario: Launch application
- **WHEN** the desktop application is launched
- **THEN** its initial window opens maximized while retaining usable sidebar and content panes