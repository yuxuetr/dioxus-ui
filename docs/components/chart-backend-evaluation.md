# Chart Backend Evaluation

This document records the M29.1 chart backend evaluation after Web, Desktop,
and Mobile runtime verification planning. M35 followed this direction by adding
first-party SVG Chart composition without adding external backend adapters.

Status: Decided in M29.1; first-party SVG composition implemented in M35.

## Decision

M35 adds `dxui add chart`, a `chart` registry entry, and a `dioxus-ui` `chart`
feature for first-party SVG composition only.

When chart rendering starts, prefer a first-party, source-copy-friendly SVG
recipe and adapter shape for the first public slice. Keep Plotters as the first
external Rust backend candidate for future opt-in rendering. Keep Charming or
other ECharts-backed approaches app-owned.

This means the project now ships:

- pure chart data primitives
- scale helpers
- color token helpers
- summary and fallback-row helpers
- first-party SVG composition parts for line, bar, and area charts

It still does not ship:

- canvas rendering adapters
- ECharts/JavaScript interop wrappers
- tooltip cursor runtime
- chart animation runtime

## Evaluated Options

| Backend | Fit | Strengths | Risks | Decision |
| --- | --- | --- | --- | --- |
| First-party SVG in Dioxus RSX | Implemented first public path | Source-copy friendly, accessible markup is controllable, works naturally across Web/Desktop WebView, no heavy dependency | More rendering work for axes, ticks, paths, and stacked charts | Implemented in M35 for line, bar, and area composition |
| Canvas with app-owned drawing | Useful for dense data | Better for many points and custom drawing | Weaker built-in semantics, needs explicit fallback table and hit testing, Web/Desktop/Mobile verification is heavier | Keep as recipe or future opt-in adapter, not default |
| Plotters adapter | Strong Rust-native candidate | Mature Rust plotting API, supports multiple backends including SVG, bitmap, and WASM/canvas paths | Dependency surface, Dioxus event integration, accessibility and source-copy policy need adapter design | First external Rust backend candidate after first-party SVG contract is proven |
| Charming/ECharts adapter | Strong app-owned option | Rich ECharts feature set, HTML/WASM/SSR renderer options | JS/ECharts rendering model, optional SSR/WASM feature split, generated source complexity, accessibility policy not controlled by this library | Keep app-owned, not a default `dioxus-ui` backend |

## Source Notes

- Plotters docs describe it as a Rust plotting library with multiple backends,
  including bitmap, vector, and WebAssembly, plus HTML5 canvas support through a
  WASM backend: <https://docs.rs/plotters/latest/plotters/>.
- Charming docs describe an Apache ECharts-backed Rust library with HTML,
  image/SSR, and WASM renderers: <https://docs.rs/charming/latest/charming/>.

## Gates Used Before The Public Chart

Before the public Chart component was added, the project required:

- measurement behavior verified for Web and Desktop, and a Mobile visual
  viewport policy
- accessible title, description, summary, and fallback table requirements
- tooltip ownership model, including keyboard and touch alternatives
- color-independent series labeling
- reduced-motion and animation policy
- source-copy dependency policy
- chart type scope limited to the first supported set

The first supported set should be:

- line chart
- bar chart
- area chart

Still deferred:

- pie, radial, radar, heatmap, candlestick, composed charts
- streaming and virtualized data
- logarithmic and time-scale parsing
- cursor exploration runtime
- synchronized multi-chart interactions

## Implemented First Public Slice

M35 used this order:

1. Add first-party SVG recipe components in an example-only fixture.
2. Verify measurement, responsive sizing, and fallback table output.
3. Add keyboard-reachable tooltip or explicitly ship without interactive
   tooltips.
4. Add generated source-copy docs explaining that chart rendering remains
   source-owned.
5. Add `dxui add chart` and a `chart` feature after the fixture passed.

The first public component exposes composition parts rather than a large
all-in-one chart engine:

```text
ChartRoot
ChartSvg
ChartTitle
ChartDescription
ChartLegend
ChartFallbackTable
ChartTooltipSlot
```

Apps should still own:

- data fetching and transforms
- number/date formatting
- units and localization
- tooltip content
- animation timing
- backend-specific rendering if they choose Plotters or Charming

## Platform Decision

| Target | Backend Position |
| --- | --- |
| Web | First-party SVG is the preferred first path because semantics and fallback markup stay in Dioxus. Canvas and ECharts stay opt-in. |
| Desktop | First-party SVG should be verified inside WebView before any stable claim. Plotters bitmap/SVG can be evaluated as an opt-in backend later. |
| Mobile | Prefer summaries, fallback tables, and simple SVG. Avoid dense hover-only charts and gesture-heavy cursor exploration until the Mobile checklist has repeatable tooling. |

## Parity Impact

The shadcn-style `Chart` component is implemented as a first-party SVG
composition surface. M35 keeps external backend adapters, cursor exploration,
hit testing, animation runtime, and complex chart types deferred or app-owned.

## Relationship To Other Plans

- [Chart Strategy](chart-strategy.md)
- [Chart Follow-through](chart-follow-through.md)
- [Chart Recipes](chart-recipes.md)
- [Chart Public API Plan](chart-public-api-plan.md)
- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
- [Mobile Runtime Verification Checklist](runtime-mobile-verification-checklist.md)
