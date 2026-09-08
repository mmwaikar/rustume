## Context

The current GPUI app already owns a resizable dock shell and a profile view composed from helper methods in `src/app.rs`. The shell registers `SidebarPanel` and `ContentPanel` with the dock and then renders the visible navigation and content separately. The profile supporting groups currently use a single wrapping container and several per-item renderers that do not match the requested layout or typography.

See `proposal.md` and the delta specs for the approved behavior. This design focuses on the implementation approach only.

## Goals / Non-Goals

**Goals:**

- Keep the existing sidebar/content split and resizable dock semantics intact.
- Ensure the sidebar icon and text remain visible and the full sidebar pane is painted with the configured background color.
- Make the Profile supporting section more readable and consistent at both wide and narrow sizes.
- Reuse the existing gpui-kit controls and data structures instead of introducing new dependencies.
- Keep publication filtering and display semantics aligned with the current WordPress classification logic.

**Non-Goals:**

- Changing the JSON Resume schema or asset data.
- Reworking unrelated sections such as Skills, Experience, or Geography.
- Broad refactoring beyond the layout and presentation fixes requested in round 2.

## Decisions

### Keep the dock shell but remove user-visible identity labels

The dock API still needs stable panel identifiers to keep the shell resizable and to preserve the left/right split. The fix should therefore keep the panel registrations and `panel_handle(...)` wiring, while removing the redundant visible name labels from the rendered user-facing shell. This keeps behavior stable while addressing the visual bug without rearchitecting the app shell.

**Alternative considered:** Replacing Dock with a custom flex split. Rejected because the current dock is already the intended shell abstraction and its resizing behavior is a known requirement.

### Use explicit row grouping for the Profile supporting cards

The supporting content should be composed in two logical rows: Education/Publications on the first row and Network/Languages on the second row. This produces a stable order and lets the layout remain consistent when the pane is wide. When the pane becomes narrow, the row children should stack naturally instead of forcing clipped or overlapping content.

**Alternative considered:** One large wrapping container with arbitrary order. Rejected because this makes the behavior inconsistent and does not guarantee the requested pairing.

### Render education as a structured data grid

The current education helper creates a free-form row-by-row layout with repeated columns but without the visual hierarchy expected in the profile section. A grid-like layout with a header row followed by each entry keeps the data aligned without changing the underlying resume model. Missing values should remain empty, not fabricated.

**Alternative considered:** Converting to a table widget or a third-party grid component. Rejected because the app already has sufficient layout primitives and a small grid is enough to satisfy the requirement without adding new dependencies.

### Use the existing Bubble and link primitives for network and language items

Network entries should remain a single inline row of label + link. Language fluency should continue to use the existing Bubble style so the badge behavior matches the rest of the app. This keeps the profile visually consistent and avoids custom badge logic.

**Alternative considered:** Build one-off custom styling for fluency marks. Rejected because the app already has the `Bubble` primitive and this bug specifically asks to use it.

### Preserve publication filtering and fix presentation at the display boundary

The profile should continue to derive publication data through the existing WordPress exclusion logic, then render only the remaining data. The bug fix belongs in the render path: the title should be the link itself, publisher/date metadata should stay neatly on one line with a month-year date format, and the summary should appear only through a popover when present.

**Alternative considered:** Duplicate the WordPress filter or add a separate visible action link. Rejected because that would drift from the canonical logic and reproduce the exact issue the review calls out.

## Risks / Trade-offs

- [Sidebar icon visibility may differ by theme or icon asset availability] → Verify the icon assets and the active/inactive color pair in the shell before finalizing the layout.
- [Wide layout with explicit rows may compress on intermediate pane widths] → Use minimum widths and wrapping so the grouping still remains readable in the mid-range layout.
- [Publication summary popovers may require a specific gpui-kit API shape] → Confirm the exact `Popover` builder in the installed dependency version before finalizing the view code.
- [Education rows may still be too dense for narrow widths] → Keep the header row and values compact and allow wrapping to preserve read-through.

## Migration Plan

1. Inspect the current gpui-kit primitives used in the shell and profile views.
2. Update the shell presentation in `src/app.rs` so the sidebar is fully painted and the visible labels are removed.
3. Rework the supporting-group layout and helper methods for Education, Network, Languages, and publications.
4. Validate the publication behavior using the existing pure helper tests and the app’s test suite.
5. Run focused visual verification at wide and narrow dock widths, then revert only the affected file if the chosen API contract is incompatible.

## Open Questions

- The exact gpui-kit `Popover` trigger/content API should be verified against the checked-in dependency before implementation so the summary behavior matches the real control contract.
