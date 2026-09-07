## Context

The current GPUI view builds a `DockArea` entity but renders the sidebar and main content as ordinary sibling elements, so the intended split-pane behavior is absent. Sidebar labels contain hand-written text glyphs, Overview is first even though Profile is the useful identity entry point, and the content region has no independent overflow policy. Profile and Blog Posts are also rendered directly in `src/app.rs`: Profile partially places contact data beside identity but repeats contact content below, while Blog Posts emits raw release-date strings and an intro assembled from multiple children.

The existing `Resume` model already contains all required source data for Profile and WordPress publications. The change should remain a rendering and interaction correction, with no JSON schema, persistence, or dependency contract changes.

## Goals / Non-Goals

**Goals:**

- Make the application shell satisfy the new `visualizer-shell` contract across every section.
- Reuse gpui-kit controls for Dock resizing, scrolling, icons, Bubble headline claims, and GroupBox supporting sections.
- Keep Profile content complete while removing duplicated contact presentation and preserving responsive wrapping.
- Normalize publication dates at the view boundary with deterministic handling of supported partial/full date strings.
- Add focused checks for pure formatting/order decisions and a manual or browser validation path for GPUI layout behavior.

**Non-Goals:**

- Changing the JSON Resume data model or the sample resume contents.
- Removing the Overview view implementation if it remains useful outside primary navigation.
- Redesigning unrelated Experience, Skills, Projects, or graph behavior.
- Adding a new icon or UI dependency before confirming gpui-kit already exposes the needed primitives.

## Decisions

### Use the existing Dock entity as the shell owner

Make the Dock entity the parent of the sidebar and right content pane, configure minimum pane sizes, and keep header/footer outside the split so they remain persistent. This fixes the current ownership mismatch instead of introducing a second layout abstraction. A plain flex row was considered but rejected because it cannot provide the requested user-adjustable divider.

### Keep navigation data-driven and Profile-first

Build the sidebar from a fixed ordered list beginning with Profile, followed by Experience, Skills, Geography, and conditionally Projects and Blog Posts. Keep Overview out of this list while retaining its section enum only if other code still needs it. Each item will carry a stable identifier, accessible label, and gpui-kit icon element so clicking the icon or label uses the same section-selection handler. Hand-written glyph prefixes were considered but rejected because their rendering varies by font and they are the source of the reported icon defect.

### Put overflow on the right pane only

Wrap the active section content in the gpui-kit scrolling element supported by the pinned version, with a full-height/flexible parent and padding inside the scroll viewport. The sidebar remains fixed within the Dock. A page-level overflow rule was considered but rejected because it would scroll the shell and make navigation harder to use.

### Use native Bubble and GroupBox controls

Replace the local `badge` helper for Profile headline claims with gpui-kit Bubble controls, and use GroupBox controls for Education, Publications, Network, and Languages. Preserve the existing reusable card helper for unrelated project/graph content unless the pinned API requires a small compatibility adjustment. This follows the requested control vocabulary and avoids maintaining a duplicate badge abstraction.

### Format dates with one shared view helper

Add a small pure formatter near the Blog Posts rendering code that accepts the existing optional release-date string and returns `Mon Year` or `Month Year`. It will support ISO year-month and full ISO date inputs, use the source month name without a timezone conversion, and fall back to the original value only when parsing is not possible. A data-model migration was considered but rejected because this is presentation-only behavior.

### Make Profile layout responsive through wrapping and minimum widths

Keep identity and contact in a flexible row with a minimum width only where needed, allow the summary and headline controls to wrap, and place supporting GroupBoxes in a flexible two-column arrangement that collapses naturally when the pane narrows. Contact data will be rendered once in the identity block; the separate duplicate Contact card will be removed. This preserves source completeness without forcing horizontal scrolling.

## Risks / Trade-offs

- [Risk] The exact gpui-kit control names or scroll API may differ from the current pinned release. -> Mitigation: inspect the crate's exported component modules before implementation, then add the smallest adapter or use the closest supported primitive without changing the behavior contracts.
- [Risk] GPUI layout sizing may allow a pane to collapse or prevent the scroll viewport from receiving available height. -> Mitigation: set explicit Dock minimum sizes and validate with a long Profile fixture at narrow and wide window sizes.
- [Risk] Resume release dates may contain formats outside ISO year/month/day forms. -> Mitigation: preserve the original value as a fallback and add tests for supported formats plus malformed input.
- [Risk] Existing users may rely on Overview being directly navigable. -> Mitigation: keep the view implementation available and document that only the primary sidebar destination has changed; the dated bug contract explicitly removes the sidebar link.

## Migration Plan

1. Implement the shell and rendering changes in `src/app.rs`, adding focused pure helpers/tests where practical.
2. Run `cargo fmt --check` and focused `cargo test`, then `cargo check` for the native target.
3. Build the web host and inspect the shell at wide and narrow viewport sizes, checking Dock resize, content scrolling, icons, Profile groups, and Blog date output.
4. If the layout change is not viable, revert only the shell composition while retaining the independently testable date and content-formatting fixes; no data migration or rollback script is required.
