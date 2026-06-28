# Chart Recipes

These recipes show how to use chart primitives with app-owned rendering. They
are intentionally docs-only: there is no `dxui add chart`, no `chart` registry
entry, and no `chart` feature in `dioxus-ui`.

Use these helpers from `dioxus-ui-primitives` when an app wants shared data and
accessibility behavior without taking a rendering dependency from this library.

## Shared Data Setup

```rust
use dioxus_ui_primitives::{
  chart_fallback_rows, chart_scale_value, chart_series_x_domain,
  chart_series_y_domain, chart_summary, ChartColorToken, ChartDomain,
  ChartPoint, ChartSeries,
};

let series = vec![ChartSeries::new(
  "revenue",
  "Revenue",
  vec![
    ChartPoint::new(0.0, 12.0),
    ChartPoint::new(1.0, 18.0),
    ChartPoint::missing(2.0),
  ],
)];

let x_domain = chart_series_x_domain(&series);
let y_domain = chart_series_y_domain(&series);
let summary = chart_summary(&series);
let rows = chart_fallback_rows(&series);
```

Apps own number formatting, localized labels, units, titles, descriptions, and
the final rendering surface.

## Line Chart Recipe

Use the primitive domains to map each point to an app-owned SVG or canvas
coordinate.

```rust
let x = chart_scale_value(point.x, x_domain, ChartDomain::new(0.0, width));
let y = chart_scale_value(point.y.unwrap_or(0.0), y_domain, ChartDomain::new(height, 0.0));
```

Rendering guidance:

- skip missing points or split the path at missing values
- identify each series by label and shape, not color alone
- expose `chart_summary` near the chart
- render `chart_fallback_rows` as an adjacent table for non-trivial data

## Bar Chart Recipe

Use the x domain for band placement and the y domain for bar height. The
primitive helpers intentionally do not calculate band widths because layout and
padding are renderer-owned.

Rendering guidance:

- keep bar labels available in text or table form
- use `ChartColorToken` as a semantic token, not the only identifier
- include zero in the app-owned y domain when that is meaningful for the data
- avoid hover-only tooltips on mobile

## Area Chart Recipe

Use the same scale helpers as line charts, then close the app-owned path to a
baseline. The baseline decision is domain-specific and remains app-owned.

Rendering guidance:

- keep line stroke visible in addition to fill
- provide a table fallback for cumulative or dense data
- avoid animation-only state changes
- respect reduced-motion settings in the app

## Backend Evaluation Checklist

Before adding a public chart component or adapter, validate:

- rendering backend works in Web and Desktop targets
- source-copy dependency impact is explicit
- accessible title, description, summary, and fallback table are supported
- tooltips are keyboard reachable when interactive
- color-independent series identification is built in
- responsive measurement is reliable without hard-coded browser assumptions

M29.1 selects first-party SVG as the preferred first future rendering path and
keeps Plotters as the first external Rust backend candidate. See
[Chart Backend Evaluation](chart-backend-evaluation.md).

## Deferred Public API

The following remain intentionally deferred:

- `dxui add chart`
- `dioxus-ui` `chart` feature
- first-party SVG chart components
- canvas/WebView backend adapters
- tooltip cursor runtime
- chart animation runtime
