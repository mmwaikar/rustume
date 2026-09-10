## Context

The current experience renderer builds `ExperienceEntry` values with country and color metadata, but the chart fill callback must explicitly use that metadata for each datum. The legend is currently rendered as a separate child after the chart, which makes it read as unrelated content and makes mapping failures harder to notice. The shared `App::link` helper creates every profile, network, project, publication, and blog link, while footer links are composed separately.

## Goals / Non-Goals

**Goals:**

- Preserve newest-first experience ordering while making the chart fill and legend use one deterministic country-to-color mapping.
- Place the legend in the chart container so it remains visually associated with the plotted bars and includes every country represented by those bars.
- Reserve a stable, explicit color for `Unknown` and avoid relying on map iteration or missing metadata to choose colors.
- Apply one default blue link style at the shared link boundary, including footer links, without changing URL behavior.
- Add focused tests for country color consistency, unknown-country coverage, ordering, legend data, and shared link styling decisions.

**Non-Goals:**

- No changes to resume JSON parsing or the meaning of location values.
- No changes to company duration calculations, navigation, link destinations, or external dependencies.
- No redesign of the geography graph or unrelated color tokens.

## Decisions

1. **Use one country palette map for both bars and legend.** Build a deterministic country-to-color map once from the experience entries, with `Unknown` always present when needed, and pass the same assigned value to the chart fill callback and legend swatch. This avoids independently recomputing colors in two render paths.

2. **Keep the chart data as the source of legend membership.** Derive legend entries from the countries actually present in the chart data, not from all resume locations. This prevents unused countries from appearing and guarantees every legend item has a plotted counterpart.

3. **Colocate the legend in the chart frame.** Render the legend in a compact row or side region inside the chart's bordered container. A side region is preferred when the horizontal chart has room; a wrapped in-frame row is the fallback for narrow widths. Moving it outside the frame is not considered sufficient because it weakens the visual association.

4. **Centralize link color in the shared helper and explicitly match footer links.** Set the default blue color on the reusable link construction path and retain the same explicit color for footer links because footer attribution is composed separately. Keep selectable text inside links so selection behavior remains intact.

5. **Test behavior at pure-data boundaries where possible.** Add assertions around the country palette and sorted entries, and use stable constants for link styling/URLs. Runtime rendering remains a final visual check because canvas output cannot be fully asserted through DOM text.

## Risks / Trade-offs

- [Risk] A side legend may reduce chart width on small windows. -> Use a wrapped in-frame legend or horizontal scrolling while keeping it inside the chart boundary.
- [Risk] Country labels may differ in case or formatting. -> Reuse the existing normalized `country_for_location` result and assign colors after deterministic sorting.
- [Risk] GPUI link styling can be overridden by nested selectable text. -> Apply the blue text style at the link and verify the selectable child inherits it in native and web rendering.

## Migration Plan

1. Refactor the experience render model so palette assignment is shared by bars and legend, then move the legend into the chart frame.
2. Apply the default blue style to the shared link helper and footer link elements.
3. Add focused tests and run formatting, Rust tests, wasm compilation, and the web build.
4. Run the browser smoke check at wide and narrow viewports and inspect chart colors, legend placement, and links.
5. Roll back by reverting the scoped app and test changes; resume input data remains compatible.
