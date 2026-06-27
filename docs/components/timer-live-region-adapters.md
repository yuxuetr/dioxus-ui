# Timer And Live Region Adapter Plan

This document defines the M21.3 timer and live-region adapter contracts. It
builds on the [runtime adapter plan](runtime-adapters.md) and the M18
[feedback notifications plan](feedback-notifications.md).

Status: Planned in M21.

## Goals

- Define timer scheduling and cancellation boundaries for transient UI.
- Define live-region announcement queueing, urgency, and duplicate handling.
- Keep Toast and Sonner queue state controlled and deterministic.
- Avoid taking ownership of async promise orchestration or product copy.

## Non-Goals

- Implement timers.
- Implement a live-region DOM node.
- Add uncontrolled Toast or Sonner APIs.
- Own persisted notification history.
- Translate or generate product-specific announcement text.

## Timer Adapter

Required by:

- Toast auto-dismiss
- Sonner auto-dismiss
- Tooltip delay, if implemented later
- Hover Card delay, if implemented later
- Carousel autoplay, if implemented later

Responsibilities:

- schedule one-shot callbacks
- cancel scheduled callbacks
- expose elapsed time or deadline metadata for tests
- allow app policy to disable or lengthen timers
- avoid panics when the platform does not provide a timer runtime

The timer adapter should not mutate component queues directly. It should notify
the consuming app, and the app should update controlled state.

## Timer Contract Shape

The exact Rust API should wait for implementation, but the boundary should stay
small:

```rust
pub enum TimerResult {
  Scheduled,
  Cancelled,
  Missing,
  Unsupported,
}

pub trait TimerRuntime {
  type TimerId;

  fn schedule_once(&self, delay_ms: u64) -> Option<Self::TimerId>;
  fn cancel(&self, id: Self::TimerId) -> TimerResult;
}
```

Callbacks may need a Dioxus-specific shape later. The important contract is
that timers are runtime commands, while expiration checks such as
`toast_is_expired` remain pure helpers.

## Live Region Adapter

Required by:

- Toast announcements
- Sonner announcements
- Carousel slide change announcements, if implemented later
- async loading status, if implemented later
- Data Table loading or empty status, if implemented later

Responsibilities:

- enqueue announcement text
- choose polite or assertive channel
- suppress duplicate announcements when configured
- expose announcement history for tests and examples
- preserve ordering where the platform supports it

Live-region adapters should receive already-authored text. They should not
construct product copy from component internals.

## Live Region Contract Shape

```rust
pub enum AnnouncementPriority {
  Polite,
  Assertive,
}

pub enum DuplicatePolicy {
  Allow,
  SuppressConsecutive,
}

pub trait LiveRegionRuntime {
  fn announce(
    &self,
    message: &str,
    priority: AnnouncementPriority,
    duplicate_policy: DuplicatePolicy,
  ) -> bool;
}
```

`false` or an explicit unsupported result must be acceptable on platforms where
live-region behavior cannot be verified yet.

## Feedback Component Mapping

| Component | Timer Need | Live Region Need | App-Owned Policy |
| --- | --- | --- | --- |
| Toast | optional auto-dismiss per item | announce title/description/action result | duration, copy, urgency, queue mutation |
| Sonner | optional auto-dismiss per item | announce title/description/promise result | promise state, duration, copy, urgency |
| Tooltip | optional open/close delay later | none by default | hover/focus timing policy |
| Hover Card | optional open/close delay later | none by default | pointer intent policy |
| Carousel | optional autoplay later | optional slide change announcement | autoplay, pause rules, slide wording |

## Accessibility Defaults

| Concern | Default |
| --- | --- |
| Urgency | success/info/default should generally be polite; destructive errors may be assertive when app policy chooses it. |
| Duplicate messages | suppress consecutive duplicates by default for live regions. |
| Focus | notifications should not steal focus by default. |
| Timing | apps should be able to pause, lengthen, or disable timers for user preference and reduced-motion policy. |
| Actions | action result announcements are app-authored, not inferred by the component. |

## Platform Defaults

| Target | Timer Default | Live Region Default |
| --- | --- | --- |
| Web | browser timer runtime after example verification | DOM live region after announcement tests |
| Desktop | Dioxus/runtime timer if available | verify WebView screen-reader behavior before claiming support |
| Mobile | timer support with conservative durations | avoid noisy announcements and prefer explicit status text |

## Source-Copy Strategy

Generated Toast and Sonner components should remain controlled styled parts.
Future runtime helpers can be added separately, for example:

```bash
dxui add runtime-timers
dxui add runtime-live-region
```

That keeps users who only want static source-copy components from inheriting a
runtime policy they did not choose.

## Implementation Order

1. Keep Toast and Sonner queues controlled.
2. Add timer adapter contracts without wiring them into components by default.
3. Add live-region adapter contracts with testable history.
4. Build a Web demo proving timer cancellation and polite/assertive channels.
5. Decide whether Toast/Sonner should provide optional helper hooks later.

M23 turns this plan into concrete primitive-layer contracts. See the
[timer and live-region contract implementation plan](timer-live-region-contracts.md).
