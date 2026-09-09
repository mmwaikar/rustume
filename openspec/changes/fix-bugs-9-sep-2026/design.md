## Context

The shell currently renders navigation and footer content with plain GPUI elements, uses placeholder geometry for icons, installs IBM Plex Sans during web startup, and keeps the experience chart as horizontal width-scaled bars. Skills are currently modeled as skill nodes with keyword text joined by delimiters; `resume.rs` already owns the graph payload and is the narrowest place to introduce child-node relationships. The application has separate shell/content rendering paths, so selection behavior must be applied at the shared text-bearing boundaries rather than only in Profile.

## Goals / Non-Goals

**Goals:**

- Make the shell and all visible resume content selectable, consistently themed, and readable at launch.
- Replace placeholder icons and footer chrome with GPUI Kit asset/control integrations.
- Preserve existing JSON Resume input and duration calculations while changing only the visual representation of experience.
- Represent skill keywords as explicit parent/child data so the view can render each child independently.
- Keep content scrolling independent from persistent shell chrome.

**Non-Goals:**

- No changes to the JSON Resume schema or external data sources.
- No redesign of unrelated Profile, Projects, Geography, or Blog Posts behavior beyond shared selection/theme inheritance.
- No new navigation destinations or removal of existing conditional destinations.

## Decisions

1. **Centralize shared presentation behavior in the shell helpers.** Apply text-selection wrappers and theme-consistent text styling at the common shell/content construction points, then cover specialized generated text such as chart labels and skill children explicitly. This avoids relying on every individual label call site while preserving control over interactive links and bubbles.

2. **Use GPUI Kit assets and controls rather than drawing replacements.** Resolve the documented asset/icon API and use the repository's bundled icon assets for navigation. Use the documented StatusBar and text-selection controls so keyboard/mouse behavior follows the kit. The existing hand-drawn `nav_icon` helper and plain footer are implementation details to remove or replace.

3. **Bundle Inter and configure both theme font roles.** Add the Inter font resource using the same compile-time bundling approach as the current font, register it with the text system, and update the shared theme's normal and monospaced families before synchronizing the base theme. Keep the change compatible with the web startup path and verify the desktop path uses the same initialization contract.

4. **Build the experience chart from normalized duration data.** Continue using `company_months` as the source of truth, compute a stable baseline and proportional heights from the maximum duration, and render a fixed-width column per company with a rounded bottom-aligned bar. Keep the company name and month count as separate selectable text elements; use diagonal label styling only when the name needs it.

5. **Make skills hierarchical in the graph payload.** Emit one parent node per named skill and one child node per keyword with deterministic IDs and parent-child edges. Preserve isolated parent skills, case-normalize IDs consistently, and retain any existing shared-keyword relationships only where they remain meaningful after child nodes are introduced. Render the level bubble as a compact content-sized control beside the parent label.

6. **Maximize at window creation.** Set the initial window options before opening the root view, using the platform-supported maximize option rather than resizing the content manually. This keeps layout sizing responsive and leaves pane resizing owned by the dock.

## Risks / Trade-offs

- [Risk] GPUI Kit APIs or asset names may differ from the linked documentation or installed `gpui-kit` version. -> Confirm the exact 0.6.0 API from the dependency source before implementation and keep the integration isolated in small helpers.
- [Risk] Rotated labels can overlap at narrow window sizes. -> Use stable chart column dimensions, constrain label area, and fall back to a readable compact layout when diagonal placement cannot fit.
- [Risk] Broad selection wrappers can interfere with links or interactive skill rows. -> Apply selection to text-bearing elements while preserving interactive ancestors and validate copying in each major view.
- [Risk] Inter font packaging can increase WASM or desktop bundle size. -> Bundle only the required regular/font weights and verify both native compilation and the web build.
- [Risk] Changing graph node semantics can break existing relationship assertions. -> Update graph tests to assert parent/child structure and preserve the existing isolated-skill behavior explicitly.

## Migration Plan

1. Implement the shared shell/theme/assets/window changes, then update graph data and view renderers.
2. Run Rust formatting, unit tests, and the native/web build checks available in the repository.
3. Launch the application and manually verify selection, maximized startup, icons, footer persistence, chart orientation, and skills hierarchy at wide and narrow sizes.
4. Roll back by reverting the change files only; resume data remains backward compatible because no input schema changes are introduced.