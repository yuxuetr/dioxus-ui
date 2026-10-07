# Indicator

Indicator places a badge, dot, or other small element on a corner of another
element, such as an unread count on a button or a presence dot on an avatar.

## Source Copy

```bash
dxui add indicator
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["indicator"] }
```

## API Surface

- `Indicator`
- `IndicatorItem`
- `IndicatorPlacement`
- `indicator_class`
- `indicator_item_class`

```rust
rsx! {
  Indicator {
    IndicatorItem { Badge { "3" } }
    Button { variant: ButtonVariant::Outline, "Inbox" }
  }
}
```

`IndicatorPlacement` is `TopEnd` (the default), `TopStart`, `BottomEnd`, or
`BottomStart`. Placements use logical sides, so `TopEnd` sits on the top left
in a right-to-left layout.

## Accessibility Notes

Indicator adds no semantics. A count badge is read where it sits in source,
so put the `IndicatorItem` after the element when the count belongs to it, or
include the count in the element's accessible name, such as
`aria-label: "Inbox, 3 unread"` (see
[RFC 0059](../rfcs/0059-display-components.md)).
