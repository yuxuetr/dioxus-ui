# Slider

Slider provides a controlled horizontal numeric value with range and thumb
styling.

## Source Copy

```bash
dxui add slider
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["slider"] }
```

## API Surface

- `Slider`
- `SliderState`
- `SliderKeyMove`
- `slider_state`
- `slider_percent`
- `slider_range_style`
- `slider_thumb_style`
- `slider_aria_attributes`

## Accessibility Notes

Slider renders `role="slider"` with horizontal orientation and ARIA value
attributes derived from the clamped, stepped value. Keyboard event handling
should use the exported slider state primitive so Arrow, Home, End, Page Up,
and Page Down behavior stays consistent.
