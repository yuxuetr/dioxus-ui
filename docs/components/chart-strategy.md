# Chart Strategy

Chart was deferred as a public component until the rendering backend, data API,
and accessibility contract were explicit. M16 recorded the strategy instead of
shipping a placeholder chart surface; M35 implements the first public SVG
composition slice.

Status: Strategy documented in M16; M19 adds primitive helpers and docs-only
recipes while keeping the component deferred. M29.1 selects first-party SVG as
the preferred first rendering path, with Plotters as the first external Rust
backend candidate. M35 implements the first public SVG composition surface.

## Decision

Add `dxui add chart` and a `chart` crate feature only for first-party SVG
composition.

Reasons:

- chart rendering needs a backend decision, not only Tailwind classes
- different backends have different Web/Desktop behavior
- accessible charts require more than SVG paths and colors
- generated source should not force a large dependency on every project
- data modeling differs by chart type

## Backend Policy

The project should evaluate chart backends before committing to an API:

| Backend shape | Fit | Risk |
| --- | --- | --- |
| First-party SVG primitives | Source-copy friendly and dependency-light | Easy to underbuild accessibility, scales poorly across chart types |
| Adapter traits over external crates | Flexible and Rust-native | More abstraction before proven demand |
| Web canvas/SVG JS interop | Mature chart ecosystem | Cross-platform and source-copy complexity |
| Deferred docs-only recipe | Lowest risk now | No turnkey chart component yet |

M16 chooses the deferred docs-only recipe. M29.1 narrows the backend direction:
first-party SVG should be the first implementation path when Chart work resumes,
Plotters should be the first external Rust backend candidate, and ECharts-backed
approaches should remain app-owned. M35 implements the first-party SVG path.

M19 follows through by adding shared data and accessibility primitives plus
[chart recipes](chart-recipes.md). M35 adds first-party SVG composition for
line, bar, and area charts. External rendering backends remain app-owned.

For the backend evaluation decision, see
[Chart Backend Evaluation](chart-backend-evaluation.md).

## Minimum API

The public Chart component defines:

- chart types in scope: line, bar, and area
- data shape per chart type
- scale and domain behavior
- color token and series token strategy
- tooltip and legend composition
- generated source dependency policy

Still future:

- responsive measurement behavior beyond explicit dimensions and viewBox
- Web/Desktop/Mobile backend adapter support matrix
- pie, radial, radar, heatmap, candlestick, and composed charts

## Accessibility Contract

Charts must support:

- visible title or label
- optional description
- keyboard-reachable data summary when interactive
- tabular fallback or adjacent data table for non-trivial datasets
- color-independent series identification
- reduced-motion behavior for animated charts

Charts must not rely on color alone to communicate state.

## Source-Copy Constraints

Generated chart code must not silently pull in a heavy rendering stack. If a
future chart component needs dependencies, the registry entry should document
them clearly and the CLI should make the generated dependency expectation
obvious.

## Platform Defaults

| Target | Chart policy |
| --- | --- |
| Web | Prefer SVG or a proven web backend after adapter design. |
| Desktop | Verify WebView rendering, measurement, and pointer behavior before stability. |
| Mobile | Prefer responsive summaries and avoid dense interactive-only charts. |

## M19 Follow-through

M19 adds:

- pure chart data primitives
- pure scale and color token helpers
- accessible summary and fallback-row helpers
- docs-only line, bar, and area recipes

M35 implements:

- public Chart component
- registry entry
- crate feature
- source-copy SVG composition parts

Still deferred:

- external rendering backend adapters
- cursor exploration runtime
- animation runtime
- visual and keyboard verification for backend-specific adapters

## M29 Backend Decision

M29.1 kept `Chart` deferred as a public component and selected
source-copy-friendly SVG composition for the first future slice. M35 implements
that slice after fixture, fallback table, tooltip ownership, animation policy,
and source-copy gates. Plotters remains the preferred external Rust backend
candidate for a later opt-in adapter.

## M35 Public API Preparation

M35 defines and implements the first-party SVG composition surface:

```text
ChartRoot
ChartSvg
ChartTitle
ChartDescription
ChartLegend
ChartFallbackTable
ChartTooltipSlot
```

The first chart types stay limited to line, bar, and area charts. External
backends remain out of the default generated source. See
[Chart Public API Plan](chart-public-api-plan.md).
