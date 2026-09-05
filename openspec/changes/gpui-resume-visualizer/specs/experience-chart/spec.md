## Purpose

Make relative time spent at each employer obvious by showing company tenure as a bar chart measured in months.

## ADDED Requirements

### Requirement: Bar chart of months by company
The experience visualization MUST present one bar per company whose length is proportional to that company’s aggregated duration in months. Each bar MUST be labeled with the company name and the month count (or an equivalent accessible text equivalent).

#### Scenario: Multiple companies
- **WHEN** the resume contains work at two companies with different month totals
- **THEN** the longer-tenure company has a visibly longer bar and both month counts are readable

### Requirement: Empty work history
When the resume has no work entries, the experience chart MUST show an empty state that explains there is no employment history to chart, and MUST NOT draw a misleading zero-company chart that looks like a failure of the renderer.

#### Scenario: Skills-only resume
- **WHEN** the loaded document has skills but an empty `work` array
- **THEN** the experience chart area states that no work history is available

### Requirement: Chart does not distort tenure
Bar lengths MUST map monotonically to month totals. Equal month totals MUST produce equal bar lengths. The chart MUST NOT use decorative scaling that reverses or hides relative tenure.

#### Scenario: Equal tenure
- **WHEN** two companies each total 12 months
- **THEN** their bars are the same length
