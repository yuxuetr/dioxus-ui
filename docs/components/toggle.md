# Toggle

Toggle provides a controlled pressed button state for compact commands and
formatting controls.

## Source Copy

```bash
dxui add toggle
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["toggle"] }
```

## API Surface

- `Toggle`
- `ToggleSize`
- `ToggleVariant`
- `toggle_class`

## Change Events

```rust
let mut bold = use_signal(|| false);

rsx! {
  Toggle {
    "aria-label": "Bold",
    pressed: bold(),
    on_pressed_change: move |pressed| bold.set(pressed),
    "B"
  }
}
```

`Toggle` is controlled. A click, Enter, or Space calls `on_pressed_change`
with the requested state, `!pressed`, and the app passes it back as
`pressed`. A disabled toggle does not call it. Other attributes, such as
`aria-label`, are passed to the button.

## Accessibility Notes

Toggle renders a button with `aria-pressed`. Use it for binary commands where
the pressed state changes the command itself, not for navigation between views.
