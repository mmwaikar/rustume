## ADDED Requirements

### Requirement: Linked footer attribution
The visualizer shell MUST keep the footer in the StatusBar and show an attribution that says it is made with a heart using Rust and gpui-kit. The Rust name MUST link to `https://rust-lang.org/`, and the gpui-kit name MUST link to `https://gpui-kit.com`.

#### Scenario: Footer attribution is visible
- **WHEN** any resume section is displayed
- **THEN** the persistent footer shows the requested heart-marked attribution and both technology names are visible in the StatusBar

#### Scenario: Footer technology links are activated
- **WHEN** a visitor activates the Rust or gpui-kit name in the footer
- **THEN** the corresponding official URL is opened using the existing link behavior
