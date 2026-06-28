# Web Runtime Verification Harness

This document defines the M25.2 plan for verifying Web runtime adapters before
they become default behavior. It is a harness specification, not an
implementation.

Status: Planned in M25.2.

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

## Candidate Commands

Future implementation can add commands similar to:

```bash
cargo run -p dioxus-ui-runtime-web-verification
node scripts/runtime-web-verify.mjs
```

The exact command should wait until the fixture and browser driver are added.
Until then, M25.2 only defines the expected harness shape.

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
