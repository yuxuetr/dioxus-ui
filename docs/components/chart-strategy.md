# Chart Strategy

Chart is deferred as a public component until the rendering backend, data API,
and accessibility contract are explicit. M16 records the strategy instead of
shipping a placeholder chart surface.

Status: Strategy documented in M16; M19 adds primitive helpers and docs-only
recipes while keeping the component deferred. M29.1 selects first-party SVG as
the preferred first rendering path, with Plotters as the first external Rust
backend candidate.

## Decision

Do not add `dxui add chart` or a `chart` crate feature yet.

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

M16 chooses the deferred docs-only recipe. M29.1 keeps the public component
deferred but narrows the backend direction: first-party SVG should be the first
implementation path when Chart work resumes, Plotters should be the first
external Rust backend candidate, and ECharts-backed approaches should remain
app-owned.

M19 follows through by adding shared data and accessibility primitives plus
[chart recipes](chart-recipes.md). Rendering remains app-owned.

For the backend evaluation decision, see
[Chart Backend Evaluation](chart-backend-evaluation.md).

## Future Minimum API

Before adding a public Chart component, define:

- chart types in scope: line, bar, area, pie, or composed charts
- data shape per chart type
- scale and domain behavior
- color token and series token strategy
- tooltip and legend composition
- responsive measurement behavior
- Web/Desktop support matrix
- generated source dependency policy

## Accessibility Contract

Future charts must support:

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

Still deferred:

- public Chart component
- registry entry
- crate feature
- rendering backend
- visual and keyboard verification for a specific backend

## M29 Backend Decision

M29.1 keeps `Chart` deferred as a public component. The first future slice
should be source-copy-friendly SVG composition, gated by measurement,
accessibility, fallback table, tooltip, and animation policies. Plotters remains
the preferred external Rust backend candidate for a later opt-in adapter.
