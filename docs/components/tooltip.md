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

## Accessibility Notes

Tooltips should be discoverable by hover and focus. Do not put essential
interactive content inside a tooltip.
