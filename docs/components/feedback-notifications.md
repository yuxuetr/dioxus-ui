# Feedback Notifications API Plan

This document defines the M18 Toast and Sonner APIs before implementation. The
goal is to provide controlled feedback notification composition parts backed by
pure queue state primitives, without taking ownership of timers, portal
mounting, promise orchestration, DOM focus, or live-region announcement runtime
too early.

Status: Planned in M18.

## Scope

M18 covers:

- Toast
- Sonner

These components should ship in crate mode and source-copy mode. Crate mode can
reuse `dioxus-ui-core` and `dioxus-ui-primitives`; generated templates must
remain self-contained and must not import internal crates.

## Shared Primitive Strategy

M18 should add pure feedback state helpers:

```rust
ToastItem { id, title, description, variant, duration_ms, dismissible }
ToastQueue { items, limit }
ToastPlacement::{TopLeft, TopCenter, TopRight, BottomLeft, BottomCenter, BottomRight}
ToastVariant::{Default, Success, Info, Warning, Error, Loading}
ToastDismissReason::{Action, Close, Timeout, Programmatic}
```

Planned helpers:

```rust
toast_placement_attribute(placement) -> &'static str
toast_variant_attribute(variant) -> &'static str
toast_dismiss_reason_attribute(reason) -> &'static str
toast_queue_push(queue, item) -> ToastQueue
toast_queue_dismiss(queue, id) -> ToastQueue
toast_queue_limit(items, limit) -> Vec<ToastItem>
toast_is_expired(elapsed_ms, duration_ms) -> bool
```

Rules:

- helpers are deterministic and pure
- no timer ownership in primitives
- no portal mounting in primitives
- no DOM focus ownership in primitives
- no live-region runtime or announcement queue in primitives
- IDs, async operations, timer scheduling, and persisted history are app-owned

## Toast

Toast provides controlled notification composition parts. It does not own queue
lifetime, timers, portal mounting, or live announcements in the first
implementation.

Planned crate API:

```rust
ToastViewport { placement, class, children }
ToastRoot { variant, open, class, children }
ToastTitle { class, children }
ToastDescription { class, children }
ToastAction { disabled, class, children }
ToastClose { disabled, class, children }
```

Behavior defaults:

- viewport placement maps to data attributes and static class helpers
- root variant maps to static class helpers
- open state is controlled by the app
- action and close are native buttons
- app owns when items enter and leave the queue
- app owns live-region announcements and focus decisions

## Sonner

Sonner provides an opinionated toast list composition surface for common
feedback variants. It should remain a styled layer over the same primitive
queue helpers rather than a separate runtime.

Planned crate API:

```rust
SonnerViewport { placement, class, children }
SonnerToast { variant, class, children }
SonnerTitle { class, children }
SonnerDescription { class, children }
SonnerAction { disabled, class, children }
SonnerClose { disabled, class, children }
```

Behavior defaults:

- variants include default, success, info, warning, error, and loading
- loading is visual state only; async promise orchestration is app-owned
- timers and dismiss scheduling are app-owned
- app owns announcement wording and priority
- source-copy templates keep all variant class tokens static

## Accessibility Defaults

| Concern | Default |
| --- | --- |
| Urgency | Default and success should generally use polite announcements; destructive or error feedback may use assertive announcements in the app. |
| Focus | Toasts do not steal focus by default. Action buttons remain reachable when the app renders them in the tab order. |
| Dismissal | Close buttons are native buttons; escape-key bulk dismissal is app-owned. |
| Timing | Auto-dismiss must be app-owned so apps can respect reduced motion and user timing preferences. |
| Mobile | Prefer bottom placement and larger tap targets; do not rely on hover-only affordances. |

## Platform Defaults

| Target | Feedback defaults |
| --- | --- |
| Web | Use app-owned portal placement, live-region node, and timer scheduler. |
| Desktop | Avoid assuming browser notification APIs or DOM portal semantics. |
| Mobile | Prefer tap controls, larger targets, and fewer simultaneous visible items. |

## Implementation Order

1. Feedback notification API plan
2. Feedback state primitives
3. Toast
4. Sonner
5. documentation, examples, and parity updates

This order starts with deterministic queue behavior, then adds the lower-level
Toast composition parts before the more opinionated Sonner presentation.

## Quality Gates

Each M18 component should include:

- primitive unit tests for queue and state helpers
- crate-mode class composition tests
- registry entry and self-contained template
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking implementation tasks done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
