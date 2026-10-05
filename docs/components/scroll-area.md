# Scroll Area

Scroll Area wraps native scrolling with styled composition parts. It does not
replace browser or platform scroll behavior in the first implementation.

## Source Copy

```bash
dxui add scroll-area
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["scroll-area"] }
```

## API Surface

- `ScrollArea`
- `ScrollAreaViewport`
- `ScrollAreaContent`
- `ScrollAreaScrollbar`
- `ScrollAreaThumb`
- `ScrollAreaCorner`
- `ScrollAreaOrientation`
- `scroll_area_orientation_attribute`

## Accessibility Notes

The viewport keeps native scrolling behavior and is a Tab stop
(`tabindex="0"`) with a focus ring, so keyboard users can scroll it; WebKit
never makes scrollers focusable on its own. Name it with `role: "region"` and
`aria-label`, or pass `tabindex: "-1"` when its content already has a
focusable element. Styled scrollbar parts are presentational hooks and are
hidden from assistive technology. Apps remain
responsible for focus management, scroll restoration, and platform-specific
scrollbar behavior.
