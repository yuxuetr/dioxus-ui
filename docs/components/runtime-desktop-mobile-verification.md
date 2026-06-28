# Desktop And Mobile Runtime Verification Strategy

This document defines the M25.3 strategy for Desktop and Mobile runtime
adapter verification. It complements the Web harness plan and keeps
renderer-specific behavior out of default components until each target is
verified.

Status: Desktop strategy planned in M25.3. M28.1 adds the Desktop runtime smoke
fixture scaffold.

## Decision

Desktop should get a focused WebView smoke fixture before runtime adapters are
enabled by default. Mobile should remain documentation-first until stable
project tooling exists for touch, viewport, and assistive verification.

Rationale:

- Desktop WebView behavior can differ from browser behavior for focus, portal
  stacking, pointer capture, timers, and device scale
- Mobile gesture behavior must preserve native scrolling and account for safe
  areas, visual viewport changes, reduced motion, and hover absence
- the current workspace has Web and Desktop examples, but no dedicated runtime
  verification fixtures
- unsupported fallbacks are safer than claiming runtime support without target
  evidence

## Desktop Verification

Planned location:

```text
examples/runtime-desktop-verification/
```

M28.1 adds the fixture as a workspace package:

```bash
cargo test -p dioxus-ui-runtime-desktop-verification
cargo run -p dioxus-ui-runtime-desktop-verification
```

The first fixture is a compile-checked smoke scaffold. It exposes stable
Desktop-oriented status output for focus, portal stacking, timers, visible live
status, measurement, pointer capture, and conservative gesture checks. It does
not start a real WebView window or claim renderer support.

The Desktop fixture should reuse the same runtime panels as the Web harness but
verify WebView-specific behavior:

| Runtime Family | Desktop Smoke Checks |
| --- | --- |
| Focus | Initial focus, modal trap, focus return after close, escape behavior, and missing target fallback. |
| Portal | Inline, body, and named target stacking inside the app shell; missing target fallback. |
| Timer | Schedule, cancel, disabled timer, cleanup on close, and behavior while the window is hidden if tooling supports it. |
| Live region | Visible status mirror and manual assistive check notes; do not claim screen-reader support from WebView smoke alone. |
| Measurement | Anchor, overlay, viewport, scroll, resize, and device scale consistency. |
| Pointer | Pointer down/move/up/cancel, capture release, and lost-capture behavior where WebView exposes it. |
| Gesture | Conservative carousel swipe checks only after pointer behavior is proven. |

Desktop support should graduate one runtime family at a time. Focus and portal
can be verified before gestures; gestures should wait until pointer capture is
stable.

## Mobile Verification

Mobile should be treated as a target profile with stricter interaction rules,
not as a separate component tree.

Mobile verification must cover:

| Area | Required Checks |
| --- | --- |
| Touch targets | Controls meet touch density expectations and do not rely on hover-only affordances. |
| Safe areas | Overlays, sheets, drawers, and notification viewports avoid unsafe screen edges. |
| Visual viewport | Measurement adapts when browser chrome or software keyboard changes viewport size. |
| Native scroll | Carousel and drawer gestures preserve vertical scrolling and cancel below threshold. |
| Reduced motion | Timer, autoplay, and gesture defaults can be disabled or softened by app policy. |
| Live status | Critical announcements have visible text, not only live-region side effects. |

Until stable mobile automation exists in this project, Mobile runtime adapters
should remain opt-in or documentation-only. Source-copy components should
continue to expose controlled parts that apps can wire to platform-specific
behavior.

## Documentation-Only Targets

These areas should stay documentation-only until tooling or target behavior is
proven:

- mobile screen-reader live-region reliability
- mobile visual viewport measurement across browser chrome and keyboard states
- Desktop background timer behavior across operating systems
- Desktop WebView pointer capture quirks
- gesture physics beyond deterministic next, previous, and cancel outcomes
- chart rendering backend behavior on high-density Desktop and Mobile displays

## Candidate Commands

Future Desktop implementation can add commands similar to:

```bash
cargo run -p dioxus-ui-runtime-desktop-verification
```

Mobile verification may begin as a manual checklist attached to the Web fixture
or as a dedicated target fixture after Dioxus Mobile tooling is selected.

## Graduation Criteria

Desktop runtime support can move from planning to implementation when:

- a Desktop smoke fixture exists
- focus, portal, timer, measurement, and pointer checks can be run locally
- unsupported fallback states are visible
- live-region support is documented as manual until assistive behavior is
  verified

Mobile runtime support can move from documentation-only to implementation when:

- target device or emulator commands are defined
- touch, safe-area, visual viewport, and native scroll checks are repeatable
- unsupported or cancel fallbacks are visible
- hover-only components have explicit mobile alternatives

## Relationship To Other Plans

- [Runtime Renderer Verification Matrix](runtime-renderer-verification.md)
- [Web Runtime Verification Harness](runtime-web-verification-harness.md)
- [Runtime Adapter Plan](runtime-adapters.md)
