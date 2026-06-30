# Message Scroller Runtime Boundary Plan

This document defines the M34.1 boundary for Message Scroller before state
helpers or styled parts are implemented.

Status: Planned in M34.1. Pure state helpers implemented in M34.2. Controlled
composition parts implemented in M34.3. Web runtime assertion prerequisites
added in M34.4.

## Decision

M34 should split Message Scroller into three layers:

| Layer | Milestone | Owns | Does Not Own |
| --- | --- | --- | --- |
| Pure state helpers | M34.2 | bottom threshold math, unread marker visibility, scroll intent transitions | DOM measurement, scroll commands, async streams |
| Styled composition parts | M34.3 | viewport/content/anchor/marker/button markup, Tailwind classes, data attributes | imperative scrolling, virtualization, stream transport |
| Runtime verification | M34.4+ | browser assertions for scroll behavior and future adapter shape | default component behavior before verification |

The public component should be useful without a runtime adapter. It should show
the current following state, unread marker, and jump button based on controlled
props, while the app remains responsible for measuring scroll positions and
executing scroll commands.

## User-Facing Behavior

Message Scroller supports chat and transcript layouts where the app needs to:

- stay pinned to the latest message when the user is already near the bottom
- hold position when the user scrolls away from the latest message
- show an unread marker or jump button when new content arrives while held
- clear the unread marker when the app confirms the viewport reached the bottom
- avoid stealing focus from message actions, inputs, or selected text

The library can model these decisions, but it cannot know actual scroll
positions without renderer measurement.

## Pure Helper Inputs

M34.2 should use plain values so the helpers can be unit-tested without Dioxus,
DOM, WebView, or Mobile APIs.

```rust
pub struct MessageScrollerMetrics {
  pub scroll_top: f64,
  pub viewport_height: f64,
  pub content_height: f64,
}

pub enum MessageScrollerEvent {
  UserScrolled,
  MessageAppended,
  JumpRequested,
  ReachedBottom,
  Reset,
}

pub enum MessageScrollerIntent {
  Follow,
  Hold,
  JumpToLatest,
}
```

Required helper behavior:

- `message_scroller_distance_to_bottom(metrics)` returns a non-negative
  distance.
- `message_scroller_is_at_bottom(metrics, threshold)` returns true when the
  distance is within threshold.
- `message_scroller_should_follow(intent, metrics, threshold)` returns true for
  active following or explicit jump requests.
- `message_scroller_show_unread_marker(intent, appended_count)` returns true
  only when new content arrived while the user is held away from the bottom.
- `message_scroller_next_intent(intent, event, at_bottom)` resolves the next
  controlled intent without measuring the viewport itself.

M34.2 implements these helpers in
`crates/dioxus-ui-primitives/src/message_scroller.rs`.

## Runtime Boundary

The runtime layer may be planned later as a scroll command adapter, but M34.2
and M34.3 should not introduce it.

Runtime-owned responsibilities:

- reading scroll container metrics
- subscribing to scroll and resize invalidation
- scrolling to the bottom anchor
- preserving offset when older history is prepended
- restoring a saved message position
- coordinating smooth scroll with reduced-motion policy
- browser-level assertions for sticky-bottom and unread-marker behavior

App-owned responsibilities:

- async stream transport and cancellation
- message persistence and IDs
- virtualization and windowing
- loading older history
- deciding whether streaming tokens count as appended messages
- deciding when a jump request should be smooth or immediate
- routing to a specific message or citation

## Styled Component Surface

M34.3 should keep the component controlled and source-copy friendly:

```rust
MessageScroller {
  intent: MessageScrollerIntent,
  has_unread: bool,
  class: String,
  children: Element,
}

MessageScrollerViewport {
  class: String,
  children: Element,
}

MessageScrollerContent {
  class: String,
  children: Element,
}

MessageScrollerBottomAnchor {
  class: String,
}

MessageScrollerUnreadMarker {
  visible: bool,
  class: String,
  children: Element,
}

MessageScrollerJumpButton {
  visible: bool,
  class: String,
  children: Element,
}
```

The component may expose data attributes such as `data-intent`,
`data-following`, and `data-unread`, but it must not execute scroll commands.

M34.3 implements the controlled component, source-copy template, registry
entry, component docs page, catalog entry, and Web/Desktop demo states.

## Accessibility

Message Scroller must avoid noisy live announcements by default.

- Unread markers should be visible text, not only a live-region side effect.
- Jump buttons need explicit labels when icon-only.
- Streaming status announcements remain app-owned until live-region behavior is
  verified per renderer.
- Focus should remain on the currently focused control when messages append.
- Jump-to-latest should be app-triggered by button activation or explicit
  runtime policy, not automatic focus movement.

## Web, Desktop, And Mobile

| Target | M34 Position |
| --- | --- |
| Web | Browser assertions are required before claiming sticky-bottom behavior. |
| Desktop | WebView scroll, resize, and device-scale behavior remain deferred until a Desktop smoke path exists. |
| Mobile | Native scroll, visual viewport, safe area, keyboard viewport changes, and reduced motion remain documentation-only until repeatable tooling exists. |

## Quality Gates

M34.1 documentation changes should run:

```bash
git diff --check
```

M34.2 helper implementation should run:

```bash
cargo test -p dioxus-ui-primitives
cargo test --workspace --all-features --quiet
```

M34.3 component implementation should additionally run:

```bash
cargo test -q -p dioxus-ui-cli --test registry
scripts/generated-fixture-smoke.sh
scripts/feature-check.sh
```

M34.4 browser assertions should stay an expensive runtime gate and should not
be promoted into default release checks until stable.

M34.4 adds `node scripts/runtime-web-verify.mjs` as the separate expensive Web
runtime assertion prerequisite command. It validates fixture-visible
Message Scroller scroll-command states and pending browser assertion markers
without claiming real DOM scroll support.
