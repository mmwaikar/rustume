## Context

The current GPUI app already uses `DockArea` for a resizable left panel and main content pane. `SidebarPanel` and `ContentPanel` register dock panel names for the dock API, while their render methods supply the actual visible elements. Profile supporting sections currently add an inner heading inside each GroupBox, place all groups in one wrapping container, and render education, network, language, and publication content as free-form rows. The existing `Bubble`, `GroupBox`, and `Link` controls are already used in the application.

See `proposal.md` and the three delta specs for the user-visible behavior being changed.

## Goals / Non-Goals

**Goals:**

- Keep dock registration, resizing, independent content scrolling, and existing navigation behavior intact while removing redundant labels.
- Make the sidebar pane itself fill available height with the established sidebar color.
- Make Profile's four supporting groups deterministic and scannable at wide sizes, with responsive stacking.
- Reuse gpui-kit primitives for bubbles, links, grouping, and the publication description interaction.
- Preserve JSON Resume source values and existing WordPress classification logic.
- Add focused testable helpers for publication filtering and date/row data where a pure Rust assertion is useful.

**Non-Goals:**

- Changing the JSON Resume schema or resume asset.
- Reworking the header, footer, navigation destinations, charts, graph views, or overall color palette.
- Adding a new UI dependency before confirming the installed gpui-kit version cannot provide the required popover behavior.
- Changing publication or profile URLs, except for the visible way they are presented.

## Decisions

### Keep dock panel registrations but control visible chrome

Retain the `BasePanel::panel_name` values because the dock needs stable panel identity for layout and resizing. Register the entities with `panel_handle(...)` and `DockLayout::panel_view(..., cx)` so gpui-kit receives their presentation trait instead of falling back to `panel_name`, and implement an empty presentation title for these shell panels. Wrap the sidebar render root in a full-height element with the sidebar background, rather than relying on the navigation content's height to paint the pane.

**Alternative considered:** Removing the panel names or replacing Dock with a manual flex split. Rejected because the names are part of dock identity and the existing Dock behavior satisfies the resize requirement.

### Build Profile rows in explicit order

Construct two row containers from the conditionally available groups: the first contains Education then Publications, and the second contains Network then Languages. Use flexible children with a minimum width so each row forms two columns when the pane permits and stacks naturally when it does not. `supporting_group` remains responsible for the GroupBox frame and title; content renderers return only rows, tables, or controls and no longer add duplicate section headings.

**Alternative considered:** One flex-wrap list sorted by insertion order. Rejected because wrapping can move groups between rows unpredictably as widths change and cannot guarantee the requested pairing.

### Represent education as aligned rows, not a new data dependency

Keep education data in the existing `Education` model and render a lightweight grid-style layout from GPUI containers: a header row for the available fields followed by one row per entry, with consistent flex/grid widths and graceful wrapping for long values. Preserve missing fields as empty cells rather than inventing values.

**Alternative considered:** Add a table crate or convert the model to a new tabular abstraction. Rejected as unnecessary for a fixed, small resume section and risky for the existing WASM target.

### Use existing primitives for inline metadata and fluency

Build network rows as one flex row containing the network label and link. Build language rows as a flex row containing the language and the existing `bubble` helper around the fluency. Allow the row to wrap only at narrow widths so labels remain readable without horizontal overflow.

**Alternative considered:** Keep hyphenated strings and add CSS-like text formatting. Rejected because it does not satisfy the requested visual distinction or reuse the established Bubble control.

### Filter and present publications with a pure selection boundary

Derive the profile publication list by excluding entries recognized by the existing WordPress publication helper/classification rule. Only render the Publications GroupBox when the filtered list is non-empty. For each remaining item, use the title as the link when a URL exists, preserve publisher/date metadata, and attach a popover trigger only when a non-empty summary exists. Verify the exact popover builder available in the pinned gpui-kit version during implementation before choosing its concrete API shape.

**Alternative considered:** Duplicate the WordPress predicate inside the view or keep a separate visible `Open publication` link. Rejected because duplicated classification can drift and the separate action is the bug being removed.

The pinned `gpui-kit` 0.6.0 exposes `gpui_kit::component::popover::Popover` with `trigger(...)` and `content(...)`; use that native component for publication summaries rather than adding a dependency.

## Risks / Trade-offs

- [The dock skin may expose panel names through an API not visible in `src/app.rs`] -> Inspect the pinned gpui-kit source/examples during implementation and change the smallest supported dock configuration; keep panel names registered for persistence.
- [The pinned gpui-kit release may not expose a popover primitive with the expected builder API] -> Confirm the dependency API before editing. If unavailable, use the closest existing gpui-kit disclosure/tooltip primitive and document the compatibility limitation rather than adding an unrelated UI library.
- [A fixed two-column row can become cramped at intermediate pane widths] -> Use minimum widths and flex wrapping/stacking, and validate both wide and narrow dock sizes.
- [Publication classification may remove an entry unexpectedly] -> Reuse the existing `wordpress_publications` semantics and add a focused test with mixed ordinary and WordPress entries.
- [Education rows may be difficult to inspect in a narrow pane] -> Permit cell wrapping and verify the full profile content remains reachable through the existing scrollable content pane.

## Migration Plan

1. Inspect gpui-kit dock and popover APIs pinned by `Cargo.toml`.
2. Update the shell and Profile rendering in `src/app.rs`, keeping the current data models and navigation state.
3. Add or update focused native tests for publication filtering and pure formatting/selection helpers.
4. Run `cargo fmt` and `cargo test`, then build or launch the supported GPUI target for a visual check at wide and narrow pane sizes.
5. Roll back by reverting the single app change if the dock or popover API is incompatible; no data migration or asset migration is required.

## Open Questions

- The exact gpui-kit popover API name and trigger semantics should be confirmed from the checked-in dependency source before implementation; this does not alter the requested behavior or task breakdown.
