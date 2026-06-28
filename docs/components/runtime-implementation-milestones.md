# Runtime Implementation Milestone Seeds

This document defines the M25.4 implementation seeds that follow renderer
verification planning. It does not mark any renderer runtime adapter as stable.

Status: Planned in M25.4.

## Implementation Order

| Milestone | Goal | Default Behavior |
| --- | --- | --- |
| M26 Web runtime verification fixture | Add a dedicated Web fixture and browser assertion harness for runtime contracts. | No component defaults change. |
| M27 Web feedback and overlay runtime adapters | Add experimental Web focus, portal, timer, and live-region adapters behind explicit opt-in wiring. | Source-copy templates remain runtime-free. |
| M28 Web measurement, pointer, and gesture adapters | Add experimental Web measurement, pointer, and gesture adapters after the fixture proves fallback behavior. | Gesture-heavy behavior remains opt-in. |
| M29 Desktop and Mobile verification follow-through | Add Desktop smoke fixture and Mobile checklist or device harness when tooling is selected. | Desktop/Mobile support remains conservative. |
| M30 Chart backend evaluation | Revisit chart rendering backend after measurement and fallback-table requirements are concrete. | No `chart` component ships before backend choice. |

## Priority Rationale

1. Verification fixture comes before adapter implementation because runtime
   behavior depends on browser and WebView APIs.
2. Feedback and overlay adapters come before gestures because their fallback
   behavior is easier to assert and affects many components.
3. Measurement comes before chart rendering because chart sizing and tooltip
   behavior depend on reliable rectangles.
4. Pointer and gesture support comes after measurement and pointer capture are
   proven.
5. Desktop and Mobile remain separate verification tracks because passing Web
   tests does not prove WebView or touch behavior.

## M26 Seed

M26 should implement the Web verification fixture described in
[Web Runtime Verification Harness](runtime-web-verification-harness.md).

Initial tasks:

- create `examples/runtime-web-verification`
- add fixture panels for focus, portal, timer, live-region, measurement,
  pointer, and gesture runtime families
- add stable `data-testid` hooks and visible fallback status output
- add a browser assertion script only after the fixture can run locally
- document the command as an expensive runtime check, not a default release gate

M26 ships the compile-checked fixture and Rust command first. Browser automation
remains a later step.

## Later Adapter Slices

M27 and M28 should not start until M26 can prove both success and fallback
states.

Recommended adapter order:

1. Web timer adapter and live-region adapter
2. Web portal adapter and non-modal focus return
3. Web modal focus trap
4. Web measurement adapter
5. Web pointer adapter
6. Web gesture adapter

Each slice should add tests before changing component docs from planned to
implemented runtime behavior.

## Documentation Updates

When each milestone lands, update:

- [Parity Matrix](parity.md)
- [Accessibility Contract Checklist](accessibility.md)
- [Complex Component Batches](complex-batches.md)
- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
