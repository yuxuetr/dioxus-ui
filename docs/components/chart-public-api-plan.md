# Chart Public API Plan

This document defines the M35.1 first-party SVG Chart API plan. It narrows the
future public component surface before any `chart` registry entry, crate
feature, or generated source is added.

Status: Planned in M35.1.

## Decision

The first public Chart slice should be first-party SVG composition built from
existing pure chart primitives. It should support only:

- line charts
- bar charts
- area charts

The default generated source must not include Plotters, Charming, ECharts,
canvas, JavaScript interop, or any other rendering backend dependency. Those
remain app-owned or future opt-in adapters.

## Existing Foundation

The public component should reuse the primitives already shipped by
`dioxus-ui-primitives`:

```rust
ChartPoint
ChartSeries
ChartDomain
ChartScale
ChartColorToken
ChartFallbackRow
chart_series_x_domain
chart_series_y_domain
chart_scale_value
chart_color_class
chart_color_attribute
chart_summary
chart_series_label
chart_value_label
chart_fallback_rows
```

These helpers stay deterministic and rendering-agnostic. The styled component
layer owns markup composition only; apps still own data fetching, transforms,
number formatting, localization, units, and backend-specific rendering if they
choose an external chart library.

## Public Composition Surface

The first component should expose small parts instead of a full chart engine.
Names below describe the intended source-copy and crate-mode API.

```text
ChartRoot
ChartSvg
ChartTitle
ChartDescription
ChartLegend
ChartFallbackTable
ChartTooltipSlot
```

`ChartRoot` owns the outer layout, default aspect ratio, and stable chart
container classes. It should accept `class` and `children`, but it should not
measure DOM size or subscribe to resize events.

`ChartSvg` owns the semantic SVG wrapper. It should accept a `view_box`, title
ID, description ID, `class`, and `children`. It should render as an SVG with
`role="img"` and explicit accessible labeling.

`ChartTitle` renders the visible or screen-reader title for the chart. Apps own
the title text and ID value so multiple charts can coexist safely.

`ChartDescription` renders supporting context, summary text, or a screen-reader
description. Apps can pass `chart_summary(series)` when a concise generated
summary is sufficient.

`ChartLegend` renders series labels. Legend items must identify each series by
text and, when helpful, shape or token name. Color alone is not enough.

`ChartFallbackTable` renders rows from `chart_fallback_rows(series)`. It should
be available for non-trivial charts and should not be replaced by hover-only
tooltip content.

`ChartTooltipSlot` provides a controlled tooltip container for apps that already
know which datum is active. It should accept visibility and positioning props
only; pointer tracking, hit testing, keyboard cursor movement, and touch
exploration remain app/runtime-owned.

## First Chart Types

The first public rendering helpers should cover only these SVG series shapes:

- `ChartLineSeries` for paths built from present points
- `ChartBarSeries` for rectangular bars
- `ChartAreaSeries` for filled areas with a visible stroke

All three should take explicit dimensions, domains, and series data rather than
performing responsive measurement internally.

Deferred chart types:

- pie, donut, radial, radar, heatmap, candlestick, composed charts
- stacked and grouped domain helpers beyond simple app-owned calculations
- time-scale parsing and logarithmic scales
- streaming, virtualization, synchronized cursors, and dense-data rendering

## Accessibility Contract

The component must make accessible output the default path:

- `ChartSvg` uses `role="img"` with title and description references.
- Every chart has a title or equivalent accessible label.
- Non-trivial charts provide a fallback table or adjacent data table.
- Series labels include text, not only a color token.
- Missing values are represented in fallback rows.
- Tooltip content is supplemental and cannot be the only way to inspect data.
- Animated variants respect app-owned reduced-motion policy.

## Tailwind And Source-Copy Rules

Generated chart source should use complete Tailwind class tokens in static
strings. It must not build class names dynamically from runtime color names.

The generated `chart.rs` should be self-contained and source-copy friendly. It
may copy small data/scale helpers if needed, but it should not import internal
workspace crates from generated source.

## Platform Boundaries

| Target | M35.1 API position |
| --- | --- |
| Web | First-party SVG is the first public path after an example fixture proves sizing, fallback table output, and reduced-motion behavior. |
| Desktop | The same SVG markup should render in WebView, but Desktop claims stay gated until the fixture is checked there. |
| Mobile | The first slice should favor simple SVG, summaries, and fallback tables. Touch cursor exploration stays deferred. |

## Implementation Gates

M35 should continue in this order:

1. Build an example-only SVG chart fixture using the existing primitives.
2. Verify sizing, responsive container behavior, fallback table output, and
   reduced-motion policy.
3. Add crate-mode and source-copy composition parts only after the fixture is
   accepted.
4. Add `registry/chart.json`, docs, demo usage, and generated fixture coverage.

Before M35.3 starts, the project should still have no public `chart` registry
entry and no default Plotters or Charming adapter.

## Related Documents

- [Chart Strategy](chart-strategy.md)
- [Chart Follow-through](chart-follow-through.md)
- [Chart Recipes](chart-recipes.md)
- [Chart Backend Evaluation](chart-backend-evaluation.md)
