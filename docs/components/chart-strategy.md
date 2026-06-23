# Chart Strategy

Chart is deferred as a public component until the rendering backend, data API,
and accessibility contract are explicit. M16 records the strategy instead of
shipping a placeholder chart surface.

Status: Strategy documented in M16; component deferred.

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

M16 chooses the deferred docs-only recipe. A later milestone can introduce an
adapter once the target backend and component set are clear.

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

## Future Milestone Seed

A future chart milestone should start with:

1. backend evaluation
2. accessible data model design
3. one narrow chart type
4. generated source dependency review
5. visual and keyboard verification in Web and Desktop examples
