## Why

The 7-Sep-2026 bug list identifies several visible regressions in the Rustume visualizer: navigation does not match the intended section order, sidebar icons are unreliable, long content cannot be scrolled, and Profile and Blog Posts do not present their data in the required readable layout. Addressing these together keeps the shared shell and content sections consistent across all pages.

## What Changes

- Remove Overview from the primary sidebar and make Profile the first destination.
- Replace text-based sidebar glyphs with reliable icon controls and host the sidebar/main split in a resizable Dock.
- Make the right content pane independently scrollable while preserving the header, footer, and navigation shell.
- Format Blog Posts dates as `Mon Year` or `Month Year` and keep the introductory copy on one readable line where space permits.
- Render Profile headline claims as Bubble controls, place identity/contact details in a responsive two-column block, allow the summary to wrap, and group Education, Publications, Network, and Languages in GroupBoxes.
- Remove the custom badge helper where the existing Bubble control provides the intended behavior.

## Capabilities

### New Capabilities

- `visualizer-shell`: Define the sidebar ordering, icon treatment, resizable Dock composition, and independently scrollable content pane.

### Modified Capabilities

- `profile-view`: Require the corrected Profile identity, contact, headline, and supporting-section presentation.
- `blog-posts-view`: Require normalized human-readable publication dates and non-wrapping introductory copy behavior.
- `compendium-profile-layout`: Update the Profile composition to use Bubble controls, a responsive two-column identity block, and GroupBoxes.
- `compendium-blog-posts-layout`: Update the Blog Posts hero/supporting copy and publication date presentation.

## Impact

- **Code**: Update `src/app.rs` shell, sidebar, Profile, and Blog Posts rendering; reuse gpui-kit Dock, Bubble, icon, scrolling, and GroupBox controls where available.
- **Data**: No JSON Resume schema or source-data changes; date formatting is a view concern.
- **Dependencies**: No new dependency is expected; confirm the pinned gpui-kit API for the required controls during implementation.
- **Validation**: Add or update focused Rust/UI checks for navigation order, control composition, date formatting, scrolling/resizing behavior, and responsive Profile content.