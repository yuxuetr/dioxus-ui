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

## Accessibility Notes

Hover Card is controlled-only in this phase. Mobile behavior should not rely on
hover; use tap/click disclosure or prefer Popover/Sheet for critical actions.
