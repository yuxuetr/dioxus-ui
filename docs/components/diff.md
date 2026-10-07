# Diff

Diff compares two versions of an image or element side by side, split by a
divider the user drags.

## Source Copy

```bash
dxui add diff
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["diff"] }
```

## API Surface

- `Diff`
- `DiffBefore`
- `DiffAfter`
- `diff_class`
- `diff_layer_class`
- `diff_position`

```rust
let mut position = use_signal(|| 50.0);

rsx! {
  Diff {
    class: "aspect-video",
    position: position(),
    on_position_change: move |value| position.set(value),
    DiffBefore { img { src: "/before.jpg", alt: "Before retouching" } }
    DiffAfter { img { src: "/after.jpg", alt: "After retouching" } }
  }
}
```

`DiffBefore` shows left of the divider and `DiffAfter` right of it, at
`position` percent (default 50, clamped to 0 through 100). The component is
controlled: a move calls `on_position_change` and the app passes the value
back. Images inside the layers fill the frame; give the frame a size, such as
an aspect ratio.

## Accessibility Notes

A native range input, transparent and covering the frame, moves the divider,
so dragging, clicking, and the arrow, Home, and End keys work, and assistive
technology announces it as a slider. `label` names it, "Comparison position"
by default. The handle shows the focus ring. Give images `alt` text that says
which version each is (see [RFC 0059](../rfcs/0059-display-components.md)).
