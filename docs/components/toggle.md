# Toggle

Toggle provides a controlled pressed button state for compact commands and
formatting controls.

## Source Copy

```bash
dxui add toggle
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["toggle"] }
```

## API Surface

- `Toggle`
- `ToggleSize`
- `ToggleVariant`
- `toggle_class`

## Accessibility Notes

Toggle renders a button with `aria-pressed`. Use it for binary commands where
the pressed state changes the command itself, not for navigation between views.
