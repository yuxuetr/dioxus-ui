# Web Runtime Verification Harness

This document defines the M25.2 plan for verifying Web runtime adapters before
they become default behavior. It is a harness specification, not an
implementation.

Status: Fixture command documented in M26.5. M27.2 adds fixture-local timer and
live-region success-path adapters. M27.3 adds fixture-local focus and portal
success-path adapters.

## Decision

The first Web verification harness should be a dedicated Dioxus Web fixture
that exercises runtime adapters directly. It should not modify generated
component templates and should not make runtime adapters a default dependency
of styled components.

Rationale:

- adapter behavior needs browser APIs that pure Rust tests cannot verify
- source-copy components must remain stable without runtime imports
- each runtime family needs explicit unsupported fallback checks
- browser automation should prove behavior before Web defaults change

## Planned Fixture Shape

Planned location:

```text
examples/runtime-web-verification/
```

M26.1 adds the fixture crate and compile-checked runtime family metadata. M26.2
adds focus and portal panel status output for initial focus, focus trap, focus
return, escape close, inline portal, body portal, named target, and missing
target fallback. M26.3 adds timer, live-region, and measurement status output
for scheduling, cancellation, disabled timers, cleanup, polite/assertive
announcements, duplicate suppression, empty messages, node measurement,
viewport measurement, scroll/resize invalidation, and missing measurement
targets. Browser automation and concrete Web adapter implementations remain
deferred. M26.4 adds pointer and gesture status output for pointer start, move,
end, cancel, capture release, Carousel next, previous, cancel, unsupported
gesture fallback, and native-scroll escape placeholders.

M27.2 keeps the same fixture boundary and adds in-memory `WebTimerRuntime` and
`WebLiveRegionRuntime` adapters. The visible status output now includes
successful timer scheduling, cancellation, live-region queueing, duplicate
suppression, and the explicit unsupported fallback lines. Browser timeout APIs,
DOM live-region mutation, and cleanup assertions are still deferred to the
browser automation layer.

M27.3 adds in-memory `WebFocusRuntime` and `WebPortalRuntime` adapters. The
visible status output now includes successful initial focus, focus trap, focus
return, body portal mounting, named portal mounting, missing target fallbacks,
and explicit unsupported fallback lines. Browser focus commands, DOM portal
mounting, tab wrapping, and z-index policy remain deferred.

The fixture should expose one route or panel per runtime family:

| Panel | Purpose |
| --- | --- |
| Focus | Open modal and non-modal overlays, then verify initial focus, trap behavior, escape close, and focus return. |
| Portal | Mount content inline, in body, and in a named target; verify fallback for a missing target. |
| Timer | Schedule, cancel, disable, and unmount timers; verify no stale callback mutates state after cleanup. |
| Live region | Queue polite and assertive messages; verify duplicate suppression and empty-message fallback. |
| Measurement | Measure anchor, overlay, viewport, and scrolled content rectangles; verify missing target fallback. |
| Pointer | Emit down, move, up, cancel, and lost-capture paths; verify deltas without mutating app state directly. |
| Gesture | Simulate carousel swipe next, previous, and cancel; verify native scroll escape behavior is preserved. |

Each panel should expose stable `data-testid` attributes for automation and a
small visible status region for manual inspection.

## Browser Assertions

The first browser automation layer can use Playwright or an equivalent browser
driver. The important assertions are behavioral rather than tool-specific.

| Runtime Family | Required Assertions |
| --- | --- |
| Focus | Active element enters the expected target, tabbing stays inside modal scope, shift-tab wraps, escape closes when configured, and trigger focus is restored. |
| Portal | Body target receives mounted content, named target receives mounted content, inline mode stays local, and missing target reports fallback state. |
| Timer | Scheduled callback fires once, cancellation prevents callback, disabled delay reports disabled, and unmount cleanup prevents later updates. |
| Live region | Polite and assertive regions receive messages, consecutive duplicate suppression works, empty messages are ignored, and cleanup removes stale text. |
| Measurement | Rectangles are non-empty for mounted targets, viewport rect is available, scroll/resize invalidates stale values, and missing nodes report missing. |
| Pointer | Pointer move reports deterministic deltas, cancel stops further movement, release clears active capture, and unsupported mode leaves controlled state unchanged. |
| Gesture | Negative horizontal swipe commits next, positive horizontal swipe commits previous, below-threshold swipe cancels, and vertical native scroll is not blocked. |

## Failure Modes

The harness must make failures visible through explicit adapter results:

| Failure | Expected Result |
| --- | --- |
| Missing focus target | `FocusCommandResult::MissingTarget` or `Unsupported` |
| Missing portal target | `PortalMountResult::MissingTarget` or `Unsupported` |
| Disabled timer policy | `TimerRuntimeResult::Disabled` |
| Empty announcement | `LiveRegionRuntimeResult::EmptyMessage` |
| Missing measured node | `MeasurementRuntimeResult::Missing` |
| Pointer adapter unavailable | `PointerRuntimeResult::Unsupported` |
| Gesture below threshold | `GestureOutcome::Cancel` |

Browser tests should assert these visible result states rather than relying on
panics or console errors.

## Fixture Boundaries

The fixture may import `dioxus-ui-primitives` with the `runtime` feature and may
contain local experimental Web adapter code.

The fixture must not:

- add runtime imports to generated component templates
- change crate-mode component defaults
- require Web runtime adapters for non-Web examples
- hide app-owned policy inside styled component class helpers
- depend on Tailwind runtime class generation

## Verification Commands

Current fixture commands:

```bash
cargo run -p dioxus-ui-runtime-web-verification
cargo test -p dioxus-ui-runtime-web-verification
```

Future browser implementation can add a command similar to:

```bash
node scripts/runtime-web-verify.mjs
```

The Rust fixture command exists as of M26.1 and is documented as an expensive
runtime check in M26.5. The browser assertion command should wait until the
fixture renders under a Web runtime and a browser driver is added.

## Graduation Criteria

The Web runtime adapter slice can move from planning to implementation when:

- this fixture exists and can run locally
- browser assertions cover all runtime families or explicitly defer a family
- unsupported fallback states are visible and tested
- CI can run the Web checks separately from default Rust checks
- docs state that source-copy components remain runtime-free by default

## Relationship To Other Plans

- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
- [Runtime Adapter Plan](runtime-adapters.md)
- [Focus And Portal Contract Implementation Plan](focus-portal-contracts.md)
- [Timer And Live Region Contract Implementation Plan](timer-live-region-contracts.md)
- [Measurement Pointer Gesture Contract Implementation Plan](measurement-pointer-gesture-contracts.md)
