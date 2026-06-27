# Timer And Live Region Contract Implementation Plan

This document defines the M23.1 implementation plan for timer and live-region
adapter contracts. It turns the M21 timer/live-region plan into a narrow
primitive-layer code surface before any renderer-specific runtime is added.

Status: Planned in M23.

## Decision

Timer and live-region adapter contracts should extend
`dioxus-ui-primitives/src/runtime.rs` behind the existing `runtime` feature.

Rationale:

- focus and portal contracts already use `dioxus-ui-primitives/runtime`
- feedback queues and expiration helpers already live in `dioxus-ui-primitives`
- timer and live-region contracts are runtime command boundaries, not styling
  utilities
- a separate module would split closely related optional runtime contracts too
  early
- a separate crate would add workspace complexity before renderer-specific
  implementations exist

The first implementation should add contracts only. It should not schedule real
timers, create DOM live regions, mutate Toast/Sonner queues, or own async
promise orchestration.

## Module Location

Planned file:

```text
crates/dioxus-ui-primitives/src/runtime.rs
```

Initial public exports:

```rust
TimerRuntimeRequest
TimerRuntimeResult
TimerRuntime
TimerRuntimeUnsupported
AnnouncementPriority
DuplicateAnnouncementPolicy
LiveRegionRuntimeRequest
LiveRegionRuntimeResult
LiveRegionRuntime
LiveRegionRuntimeUnsupported
```

The implementation may refine names, but it should preserve these concepts:

- explicit request structs for deterministic tests
- explicit unsupported results
- no queue mutation from runtime contracts
- no product-copy generation in live-region contracts
- no renderer-specific timer or DOM handle type

## Feature Strategy

Use the existing feature:

```toml
runtime = []
```

Do not add a separate `feedback-runtime` feature in M23. Runtime contracts are
still inert without a concrete runtime implementation, so splitting feature
flags before there is code size or dependency pressure would add complexity
without value.

Styled Toast and Sonner components should continue to compile and behave as
controlled composition parts without runtime contracts.

## Timer Contract Shape

```rust
pub enum TimerRuntimeResult<TimerId> {
  Scheduled(TimerId),
  Cancelled,
  Missing,
  Disabled,
  Unsupported,
}

pub struct TimerRuntimeRequest {
  pub delay_ms: u64,
  pub reason: TimerReason,
}

pub enum TimerReason {
  ToastDismiss,
  SonnerDismiss,
  TooltipDelay,
  HoverCardDelay,
  CarouselAutoplay,
}

pub trait TimerRuntime {
  type TimerId;

  fn schedule_once(&self, request: &TimerRuntimeRequest) -> TimerRuntimeResult<Self::TimerId>;
  fn cancel(&self, id: &Self::TimerId) -> TimerRuntimeResult<Self::TimerId>;
}
```

The exact enum shape may be adjusted during implementation. The key boundary is
that timer runtimes report scheduling results; they do not remove items from
Toast or Sonner queues.

## Live Region Contract Shape

```rust
pub enum AnnouncementPriority {
  Polite,
  Assertive,
}

pub enum DuplicateAnnouncementPolicy {
  Allow,
  SuppressConsecutive,
}

pub enum LiveRegionRuntimeResult {
  Queued,
  SuppressedDuplicate,
  EmptyMessage,
  Unsupported,
}

pub struct LiveRegionRuntimeRequest {
  pub message: String,
  pub priority: AnnouncementPriority,
  pub duplicate_policy: DuplicateAnnouncementPolicy,
}

pub trait LiveRegionRuntime {
  fn announce(&self, request: &LiveRegionRuntimeRequest) -> LiveRegionRuntimeResult;
}
```

Live-region contracts receive already-authored text. Apps own localization,
copy tone, error severity policy, and whether an announcement should be made at
all.

## Toast And Sonner Mapping

The first examples should be pure mappings from existing feedback primitives to
runtime requests:

```rust
let timer = TimerRuntimeRequest::toast_dismiss(item.duration_ms);
let announcement = LiveRegionRuntimeRequest::polite(format!(
  "{} {}",
  item.title,
  item.description.unwrap_or_default(),
));
```

Toast and Sonner pass request metadata to runtimes, but controlled queue updates
remain app-owned.

## Source-Copy Policy

M23 should not change default generated component output.

Runtime contracts should not be copied when a user runs:

```bash
dxui add toast
dxui add sonner
```

Future opt-in commands can be planned separately:

```bash
dxui add runtime-timers
dxui add runtime-live-region
```

Until then, source-copy components remain controlled styled parts with app-owned
timer and announcement behavior.

## Platform Fallbacks

| Target | Timer Fallback | Live Region Fallback |
| --- | --- | --- |
| Web | return `Unsupported` until a Web timer runtime is installed | return `Unsupported` until a DOM live-region runtime is installed |
| Desktop | return `Unsupported` until Dioxus/WebView timer behavior is verified | return `Unsupported` until WebView assistive behavior is verified |
| Mobile | allow app policy to disable or lengthen timers | prefer explicit visible status text and avoid noisy announcements |

Fallback behavior must be testable without a renderer.

## M23 Implementation Order

1. Extend `runtime` contracts with timer request/result/trait types.
2. Extend `runtime` contracts with live-region request/result/trait types.
3. Add Toast and Sonner mapping examples in pure tests.
4. Update runtime planning docs after the contract surface is proven.

## Quality Gates

Before marking M23 implementation tasks done:

```bash
cargo test -p dioxus-ui-primitives --features runtime
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
```

Generated fixture smoke is only required when registry/templates change.
