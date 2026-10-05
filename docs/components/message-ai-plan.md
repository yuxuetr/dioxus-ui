# Message and AI-style API Plan

This document defines the M30.4 API plan for message and AI-style components.

Status: Planned in M30.4. Attachment, Bubble, Message, and Marker implemented
in M33. Message Scroller remains planned for M34.

## Sources

The current shadcn component catalog lists Attachment, Bubble, Marker, Message,
and Message Scroller as new components:

- <https://ui.shadcn.com/docs/components>
- <https://ui.shadcn.com/docs/components/radix/attachment>
- <https://ui.shadcn.com/docs/components/radix/bubble>
- <https://ui.shadcn.com/docs/components/radix/marker>
- <https://ui.shadcn.com/docs/components/radix/message>
- <https://ui.shadcn.com/docs/components/radix/message-scroller>

## Decision

M33 implemented Attachment, Bubble, Message, and Marker as provider-neutral
composition components.

M34 should handle Message Scroller separately because sticky bottom behavior,
turn anchoring, unread state, streaming append, restoring saved positions, and
jump-to-message behavior require runtime scroll and measurement verification.

These components must not depend on an AI SDK, model provider, upload service,
markdown parser, syntax highlighter, object URL manager, or virtualization
engine.

## Ownership Boundaries

The UI library owns:

- static layout and Tailwind class maps
- visual state variants
- slot composition
- accessible element guidance
- source-copy templates
- small pure helpers where behavior is deterministic without renderer access

The consuming app owns:

- file upload transport
- object URL lifecycle
- drag-drop behavior
- attachment preview loading and security policy
- markdown or rich text parsing
- syntax highlighting
- citation resolution
- model/provider integration
- stream transport and cancellation
- transcript persistence
- message IDs and routing
- virtualization
- actual scroll commands and DOM measurement until runtime adapters are stable

## Component Grouping

| Component | Milestone | Category | Rationale |
| --- | --- | --- | --- |
| Attachment | M33 | Static composition | File rows/cards can be styled without owning upload behavior. |
| Bubble | M33 | Static composition | Message surfaces are visual variants and alignment only. |
| Message | M33 | Static composition | Row layout can stay provider-neutral and compose Bubble, Avatar, and Marker. |
| Marker | M33 | Static composition | Status rows, separators, and notes are simple layout parts. |
| Message Scroller | M34 | Runtime-dependent | Scroll following, anchoring, restoration, and jump behavior need runtime verification. |

## Attachment API

Crate feature:

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["attachment"] }
```

Source-copy command:

```bash
dxui add attachment
```

Planned API:

```rust
Attachment {
  state: AttachmentState,
  size: AttachmentSize,
  orientation: AttachmentOrientation,
  class: String,
  children: Element,
}

AttachmentGroup {
  class: String,
  children: Element,
}

AttachmentMedia {
  variant: AttachmentMediaVariant,
  class: String,
  children: Element,
}

AttachmentContent {
  class: String,
  children: Element,
}

AttachmentTitle {
  class: String,
  children: Element,
}

AttachmentDescription {
  class: String,
  children: Element,
}

AttachmentActions {
  class: String,
  children: Element,
}

AttachmentAction {
  class: String,
  children: Element,
}

AttachmentTrigger {
  class: String,
  children: Element,
}

AttachmentState::{Idle, Uploading, Processing, Error, Done}
AttachmentSize::{Default, Sm, Xs}
AttachmentOrientation::{Horizontal, Vertical}
AttachmentMediaVariant::{Icon, Image}
```

Rules:

- `AttachmentAction` should map to Button-like styling, but source-copy output
  must remain self-contained or explicitly copy the needed class helpers.
- `AttachmentTrigger` should be a real button by default when implemented.
- Upload progress values are app-owned. The first API can represent state but
  should not own numeric progress or network transitions.
- Error state must be paired with text in `AttachmentDescription`, not color
  alone.
- `AttachmentGroup` may be horizontally scrollable, but it must not include
  programmatic scroll behavior in M33.

## Bubble API

Crate feature:

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["bubble"] }
```

Source-copy command:

```bash
dxui add bubble
```

Planned API:

```rust
Bubble {
  variant: BubbleVariant,
  align: BubbleAlign,
  class: String,
  children: Element,
}

BubbleGroup {
  class: String,
  children: Element,
}

BubbleContent {
  class: String,
  children: Element,
}

BubbleReactions {
  side: BubbleReactionSide,
  align: BubbleReactionAlign,
  class: String,
  children: Element,
}

BubbleVariant::{Default, Secondary, Muted, Tinted, Outline, Ghost, Destructive}
BubbleAlign::{Start, End}
BubbleReactionSide::{Top, Bottom}
BubbleReactionAlign::{Start, Center, End}
```

Rules:

- Bubble owns only the framed content surface and reaction placement.
- Avatar, sender name, timestamp, delivery state, and row-level actions belong
  to Message.
- Markdown parsing and code rendering stay app-owned and are passed as children.
- Destructive and status meanings must be visible in text, not color alone.

## Message API

Crate feature:

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["message"] }
```

Source-copy command:

```bash
dxui add message
```

Planned API:

```rust
Message {
  align: MessageAlign,
  class: String,
  children: Element,
}

MessageGroup {
  class: String,
  children: Element,
}

MessageAvatar {
  class: String,
  children: Element,
}

MessageContent {
  class: String,
  children: Element,
}

MessageHeader {
  class: String,
  children: Element,
}

MessageFooter {
  class: String,
  children: Element,
}

MessageAlign::{Start, End}
```

Rules:

- Message owns row layout, alignment, avatar placement, header placement, and
  footer placement.
- Bubble owns the visible message surface and should be composed inside
  `MessageContent`.
- Message should not define app-specific roles such as `User`, `Assistant`, or
  `Tool` in the first API. Those are data-model concerns.
- AI-specific examples may show assistant reasoning or tool status by composing
  Marker, Bubble, and Message, but no AI SDK dependency is allowed.
- Icon-only footer actions need accessible labels supplied by the app.

## Marker API

Crate feature:

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["marker"] }
```

Source-copy command:

```bash
dxui add marker
```

Planned API:

```rust
Marker {
  variant: MarkerVariant,
  class: String,
  children: Element,
}

MarkerIcon {
  decorative: bool,
  class: String,
  children: Element,
}

MarkerContent {
  class: String,
  children: Element,
}

MarkerVariant::{Default, Border, Separator}
```

Rules:

- Marker owns inline status, bordered row, and labeled separator layouts.
- For streaming or in-progress states, apps should set `role="status"` or use a
  live-region strategy explicitly.
- Icons should be decorative by default unless the app gives them accessible
  meaning.
- Marker must not own search indexing, citation resolution, tool execution, or
  transcript compaction behavior.

## Message Scroller API Boundary

Message Scroller should not be implemented with the M33 static components.
M34.1 defines pure state helpers and runtime boundaries in the
[Message Scroller Runtime Boundary Plan](message-scroller-plan.md).

Planned source-copy command after M34 planning:

```bash
dxui add message-scroller
```

Controlled API surface after runtime planning:

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

Pure helper area:

```rust
MessageScrollerIntent::{Follow, Hold, JumpToLatest}

message_scroller_distance_to_bottom(...)
message_scroller_is_at_bottom(...)
message_scroller_should_follow(...)
message_scroller_show_unread_marker(...)
message_scroller_next_intent(...)
```

Deferred until M34.4 or later runtime work:

- actual scroll commands
- scroll position restoration
- viewport measurement
- streaming append measurement
- load-history anchoring
- virtualized rendering
- browser-level runtime assertions

## Accessibility

M33 components should follow these requirements:

- Attachment icon-only actions need explicit labels.
- Attachment error states must include text, not color-only status.
- Attachment groups that are scrollable and presentational need group labeling
  if focusable.
- Bubble variants must not be the only signal for message meaning.
- Message footer icon-only actions need labels.
- Marker status rows should use explicit status semantics when announcing
  progress matters.
- Message Scroller must preserve keyboard focus and avoid noisy announcements,
  but those claims are deferred until runtime verification exists.

## Mobile and Desktop

- Static message components should be responsive by class maps and not branch by
  renderer.
- Attachment actions need touch-friendly button sizes.
- Horizontal attachment rows should remain usable with touch scrolling and
  keyboard tab order.
- Message Scroller requires separate Web/Desktop/Mobile verification because
  scroll behavior and viewport measurement differ by renderer.

## Tailwind Constraints

- all classes must be complete static Tailwind tokens
- variants must use match-based class maps
- no dynamic color, width, gap, or side class generation
- source-copy templates must remain Tailwind v4 compatible
- runtime-specific animation utilities should be optional and documented

## Source-copy Policy

Generated message component sources must remain self-contained:

- no imports from `dioxus-shadcn-core`
- no imports from `dioxus-shadcn-primitives`
- no AI SDK imports
- no upload client imports
- no markdown parser imports
- no runtime adapter imports for M33 static components

If components need shared enums or class helpers in source-copy mode, copy them
into the generated file for that component.

## M33 Implementation Result

M33 added crate features, source-copy templates, registry entries, component
docs, catalog coverage, and Web/Desktop demo states for Attachment, Bubble,
Message, and Marker.

The implementation keeps upload transport, object URL lifecycle, markdown,
syntax highlighting, citation lookup, streaming, provider integration, and
virtualization app-owned.

## Quality Gates

M33 implementation was validated with:

```bash
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
git diff --check
```

M33 completion required updates to:

- `registry/*.json`
- `templates/*.rs`
- `crates/dioxus-shadcn/src/*.rs`
- component docs pages
- component catalog
- parity matrix
- accessibility checklist

M34 must add runtime-specific gates before claiming Message Scroller behavior.
