# RFC 0011: Toast Timer and Live Region

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Toast and Sonner real dismissal behavior: an auto-dismiss countdown that
pauses while the user is reading or interacting, dismiss callbacks that carry a
`ToastDismissReason`, and a persistent live region so screen readers announce
new notifications.

## Current State

As of M135:

- `ToastRoot` and `SonnerToast` render `open` state only. Nothing dismisses a
  toast after its duration; `ToastItem::duration_ms` and `toast_is_expired`
  exist but no component uses them.
- `ToastClose`, `ToastAction`, `SonnerClose`, and `SonnerAction` render
  buttons with no event handlers.
- Each toast carries its own `role="status"` and `aria-live`. A live region
  inserted into the page together with its content is often not announced,
  because assistive technology only reports changes to regions it already
  tracks.
- `TimerRuntime` and `LiveRegionRuntime` only have record-keeping example
  adapters.

## Decision

### Dismiss Callbacks

Toast and Sonner roots, close buttons, and action buttons gain an optional
`on_dismiss: Option<EventHandler<ToastDismissReason>>`. Action buttons also gain
`onclick: Option<EventHandler<MouseEvent>>`, which runs before
`on_dismiss(Action)`. Close buttons call `on_dismiss(Close)`. Queue state stays
with the app, which usually calls `toast_queue_dismiss` in the handler.

### Countdown

Roots gain `duration_ms: u64`, default `5000`, matching `ToastItem::new`. `0`
disables the timer, which suits loading toasts. While open, a page script
counts down the remaining time and pauses while the pointer is over the toast
or focus is inside it, then reports the timeout; Rust calls
`on_dismiss(Timeout)`. Pausing keeps time-limited content readable (WCAG 2.2.1)
and keeps the toast from vanishing under the pointer before someone clicks an
action.

The countdown runs in the page through `document::eval`, like the RFC 0010
overlay scripts, so Web, Desktop, and Mobile share one path and no async timer
dependency is added. The script exits when the toast is hidden or removed. The
template copy lives in `utils.rs` and a CLI test keeps it identical to the
crate copy.

### Live Region

`ToastViewport` and `SonnerViewport` become persistent
`role="region"` containers with `aria-label="Notifications"` and
`aria-live="polite"`. Apps render the viewport once and add toasts inside it,
so additions happen inside a region assistive technology already tracks. Each
toast keeps its `role="status"` and variant-based `aria-live`, so error and
warning toasts stay assertive.

## Scope

In scope:

- countdown with hover and focus pause, and `on_dismiss` reasons for Toast and
  Sonner
- persistent viewport live regions
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Toast queue state inside components | Apps already own the queue through `ToastQueue` helpers | A consumer needs toasts raised from outside the component tree |
| Swipe to dismiss | Needs pointer gesture wiring per renderer | The gesture runtime gains a real adapter |
| Stacking and exit animations | Styling concern; `data-state` already exposes hooks | A consumer reports missing transition hooks |
| Screen reader announcement testing | Browser automation cannot observe announcements | A portable screen reader test harness exists |

## Verification

- The Web preview renders real Toast and Sonner components.
- `npm run verify:runtime-interactions` asserts timeout dismissal, that hover
  pauses the countdown, close and action reasons, and the viewport live region
  attributes.
- Desktop and Mobile share the `document::eval` path but are not covered by an
  automated gate.
