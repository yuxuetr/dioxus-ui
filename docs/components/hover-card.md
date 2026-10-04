# Hover Card

Hover Card provides controlled rich preview content for a trigger. It reuses
popover primitive placement and dismissal defaults, while hover timing and
pointer intent remain runtime adapter work.

## Source Copy

```bash
dxui add hover-card
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["hover-card"] }
```

## API Surface

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

`open` stays controlled by the app, which decides when hover or focus opens the
card. Pass the trigger id as `anchor_id` and `on_open_change` to
`HoverCardContent`.

- With `anchor_id`, content is placed next to the element with that id using
  fixed positioning, flips to the opposite side when the preferred side lacks
  room, shifts to stay inside the viewport, and follows resize and scroll.
  `side_offset` defaults to `4` pixels. Without `anchor_id`, content renders in
  place.
- `side` defaults to `Bottom` and `align` to `Center`. The rendered
  `data-side` switches to the flipped side while placed.
- Escape, a pointer press outside, and focus moving outside request close per
  `dismiss` (default `DismissBehavior::popover_default()`).

## Accessibility Notes

Hover Card stays controlled by the app. Mobile behavior should not rely on
hover; use tap/click disclosure or prefer Popover/Sheet for critical actions.
