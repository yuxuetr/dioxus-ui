# Tooltip

Tooltip provides primitive configuration with a styled content part for short
supplemental descriptions.

## Source Copy

```bash
dxui add tooltip
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["tooltip"] }
```

## API Surface

- `TooltipContent`
- `TooltipPrimitiveConfig`
- `tooltip_content_class`

## Behavior

`open` stays controlled by the app, which opens the tooltip on hover and focus.
Pass the trigger id as `anchor_id` and `on_open_change` to `TooltipContent`.

- With `anchor_id`, content is placed next to the element with that id using
  fixed positioning, flips to the opposite side when the preferred side lacks
  room, shifts to stay inside the viewport, and follows resize and scroll.
  `side_offset` defaults to `4` pixels. Without `anchor_id`, content renders in
  place.
- `side` defaults to `Top` and `align` to `Center`.
- Escape requests close per `dismiss` (default
  `DismissBehavior::tooltip_default()`); outside pointer presses do not.
- Hover delays remain app-owned.

## Accessibility Notes

Tooltips should be discoverable by hover and focus. Do not put essential
interactive content inside a tooltip.
