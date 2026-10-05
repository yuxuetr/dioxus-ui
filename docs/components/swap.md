# Swap

Swap is a toggle button that switches between two elements, such as a menu
icon that becomes a close icon, or a sun that becomes a moon.

## Source Copy

```bash
dxui add swap
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["swap"] }
```

## API Surface

- `Swap`
- `SwapEffect`
- `swap_class`
- `swap_layer_class`

```rust
let mut open = use_signal(|| false);

rsx! {
  Swap {
    "aria-label": "Menu",
    effect: SwapEffect::Rotate,
    active: open(),
    on_active_change: move |active| open.set(active),
    on: rsx! { CloseIcon {} },
    off: rsx! { MenuIcon {} },
  }
}
```

`on` shows while `active` and `off` otherwise. A press calls
`on_active_change` with the new state. `SwapEffect` is `Fade` (the default),
`Rotate`, or `Flip`. Size and style the button with `class`.

## Accessibility Notes

Swap is a native button with `aria-pressed`, so assistive technology
announces it as a toggle. The hidden layer is `aria-hidden`; when the layers
are icons, name the button with `aria-label`. Motion stops under
`prefers-reduced-motion` (see [RFC 0060](../rfcs/0060-input-components.md)).
