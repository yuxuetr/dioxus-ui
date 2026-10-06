# Hover Card

Hover Card shows rich preview content for a link. It opens on
hover and keyboard focus, and reuses popover primitive placement and
dismissal defaults.

## Source Copy

```bash
dxui add hover-card
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["hover-card"] }
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
`PopoverPrimitiveConfig` for users importing from `dioxus_shadcn::hover_card`.

## Behavior

`HoverCard` owns whether the card is open and its parts read it, so they must
sit inside it (see [RFC 0077](../rfcs/0077-component-owned-state.md)):

```rust
rsx! {
  HoverCard {
    HoverCardTrigger { href: "/users/dioxus", "@dioxus" }
    HoverCardContent { "Fullstack app framework for Rust." }
  }
}
```

Pass `open` and `on_open_change` to control it, or `default_open` to start it
open; `on_open_change` hears every change in both modes.

- Hovering the trigger opens it after `open_delay_ms` (default `700`).
  Keyboard focus on the trigger opens it at once.
- The card stays open while the pointer or focus is on the trigger or the
  card, so the pointer can reach links inside it. Leaving both with the
  pointer closes it after `close_delay_ms` (default `300`). Focus leaving
  both closes it at once, unless the pointer is over them.
- A press on the trigger follows the link and keeps the card open.
- Touch pointers do not open the card.
- `HoverCardTrigger` renders an `a` with the required `href` and passes
  through anchor attributes such as `target` and `rel`.
- The content anchors to `HoverCardTrigger`.
- Content is placed next to its anchor using fixed positioning, flips to the
  opposite side when the preferred side lacks room, shifts to stay inside the
  viewport, and follows resize and scroll. `side_offset` defaults to `4`
  pixels.
- `side` defaults to `Bottom` and `align` to `Center`. The rendered
  `data-side` switches to the flipped side while placed.
- Escape, a pointer press outside, and focus moving outside close it per
  `dismiss` (default `DismissBehavior::popover_default()`).

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

The card is a preview for sighted pointer and keyboard users; the trigger has
no ARIA link to it, and the content renders no role, as in the Radix Hover
Card (see [RFC 0055](../rfcs/0055-open-state-accessibility-audit.md)). Mobile behavior should not rely on hover; use tap/click
disclosure or prefer Popover/Sheet for critical actions. Skipping the delay
between adjacent cards, touch opening, and triggers other than links are not
implemented (see
[RFC 0023](../rfcs/0023-hover-card-hover-and-focus-opening.md)).
