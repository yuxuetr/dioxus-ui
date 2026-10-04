# Hover Card

Hover Card shows controlled rich preview content for a link. It opens on
hover and keyboard focus, and reuses popover primitive placement and
dismissal defaults.

## Source Copy

```bash
dxui add hover-card
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["hover-card"] }
```

## API Surface

- `HoverCard`
- `HoverCardTrigger`
- `HoverCardContent`
- `HoverCardHeader`
- `HoverCardTitle`
- `HoverCardDescription`
- `HoverCardSide`
- `HoverCardAlign`
- `HoverCardPrimitiveConfig`
- `hover_card_content_class`
- `hover_card_side_attribute`
- `hover_card_align_attribute`

The module also re-exports `OverlaySide`, `OverlayAlign`, and
`PopoverPrimitiveConfig` for users importing from `dioxus_ui::hover_card`.

## Behavior

`open` stays controlled by the app. Keep it, pass it to `HoverCardContent`,
and handle `HoverCard` `on_open_change`:

```rust
let mut open = use_signal(|| false);

rsx! {
  HoverCard {
    on_open_change: move |next| open.set(next),
    HoverCardTrigger { href: "/users/dioxus", "@dioxus" }
    HoverCardContent { open: open(), "Fullstack app framework for Rust." }
  }
}
```

- Hovering the trigger requests open after `open_delay_ms` (default `700`).
  Keyboard focus on the trigger requests open at once.
- The card stays open while the pointer or focus is on the trigger or the
  card, so the pointer can reach links inside it. Leaving both with the
  pointer requests close after `close_delay_ms` (default `300`). Focus leaving
  both requests close at once, unless the pointer is over them.
- A press on the trigger follows the link and keeps the card open.
- Touch pointers do not open the card.
- `HoverCardTrigger` renders an `a` with the required `href`.
- Inside `HoverCard`, the content anchors to `HoverCardTrigger` and uses the
  root's `on_open_change` for dismissal. `anchor_id` and `on_open_change` on
  `HoverCardContent` override them.
- Content is placed next to its anchor using fixed positioning, flips to the
  opposite side when the preferred side lacks room, shifts to stay inside the
  viewport, and follows resize and scroll. `side_offset` defaults to `4`
  pixels. Without an anchor, content renders in place.
- `side` defaults to `Bottom` and `align` to `Center`. The rendered
  `data-side` switches to the flipped side while placed.
- Escape, a pointer press outside, and focus moving outside request close per
  `dismiss` (default `DismissBehavior::popover_default()`).
- Without `HoverCard`, the app wires its own trigger and passes `anchor_id`.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

The card is a preview for sighted pointer and keyboard users; the trigger has
no ARIA link to it. Mobile behavior should not rely on hover; use tap/click
disclosure or prefer Popover/Sheet for critical actions. Skipping the delay
between adjacent cards, touch opening, and triggers other than links are not
implemented (see
[RFC 0023](../rfcs/0023-hover-card-hover-and-focus-opening.md)).
