# Chart Follow-through

This document defines the M19 chart follow-through scope and completion status.
The goal is to add pure chart data and accessibility primitives while keeping
rendering backends, generated chart components, DOM measurement, pointer
interaction, and animation runtime out of the public component surface.

Status: Implemented in M19; M35 later adds first-party SVG Chart composition.

## Scope

M19 covers:

- chart data primitives
- chart scale helpers
- chart accessibility helpers
- docs-only rendering recipes

M19 did not add:

- `dxui add chart`
- a `chart` feature in `dioxus-ui`
- SVG or canvas rendering components
- a dependency on a charting backend

M35 later adds `dxui add chart`, a `chart` feature, and source-copy SVG
composition parts while keeping external backend adapters deferred.

## Primitive Strategy

M19 added pure chart primitives:

```rust
ChartPoint { x, y }
ChartSeries { id, label, points }
ChartDomain { min, max }
ChartScale { domain, range }
ChartColorToken::{Primary, Secondary, Success, Warning, Destructive, Neutral}
ChartFallbackRow { series_id, series_label, x_label, y_label, missing }
```

Shipped helpers:

```rust
chart_domain(values) -> ChartDomain
chart_series_x_domain(series) -> ChartDomain
chart_series_y_domain(series) -> ChartDomain
chart_scale_value(value, domain, range) -> f64
chart_color_class(token) -> &'static str
chart_summary(series) -> String
chart_series_label(series, token) -> String
chart_value_label(series_label, x_label, y) -> String
chart_fallback_rows(series) -> Vec<ChartFallbackRow>
```

Rules:

- helpers are deterministic and pure
- no rendering backend in primitives
- no DOM measurement in primitives
- no pointer event ownership in primitives
- no locale-specific formatting in primitives
- labels, units, localization, data fetching, and rendering are app-owned

## Data Boundaries

Chart data primitives should support the first recipe set without pretending to
cover every visualization type.

Initial scope:

- line chart data
- bar chart data
- area chart data
- positive and negative numeric values
- empty series and missing points
- color-independent series labels

Deferred:

- pie and radial charts
- stacked domain helpers
- time scale parsing
- logarithmic scales
- streaming data
- virtualization

## Accessibility Boundaries

M19 helpers should make accessible chart output easier without generating final
markup.

Helpers should support:

- visible chart title and description expectations
- series count and point count summary
- color-independent fallback rows
- deterministic labels for missing data
- data table recipe guidance

Apps still own:

- localized number and date formatting
- announcement timing
- keyboard interaction for tooltips or cursor exploration
- visible table rendering
- reduced-motion animation decisions

## Recipe Boundaries

M19 added docs-only recipes for:

- line chart
- bar chart
- area chart

Recipe docs should show how to combine primitives with first-party SVG
composition, app-owned SVG or Canvas, or backend rendering. M35 adds
`registry/chart.json`; external backend rendering remains app-owned.

## Platform Defaults

| Target | Chart follow-through defaults |
| --- | --- |
| Web | Prefer app-owned SVG or an explicit backend adapter after measurement and accessibility validation. |
| Desktop | Verify WebView rendering and measurement before stabilizing any backend adapter. |
| Mobile | Prefer compact summaries, tabular fallback, and fewer interactive-only affordances. |

## Implementation Order

1. Chart follow-through API plan
2. Chart data primitives
3. Chart accessibility helpers
4. Docs-only chart recipes
5. documentation, parity, and batch updates

This order clarifies the data and accessibility contract before any rendering
surface is introduced.

## Quality Gates

M19 implementation tasks should include:

- primitive unit tests for data and accessibility helpers
- docs updates for chart strategy and recipes
- parity and accessibility checklist updates
- release quality gates before marking the milestone complete

Before marking implementation tasks done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
