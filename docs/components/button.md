# Button

Button is the primary command component for actions and form submission.

## Source Copy

```bash
dxui add button
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["button"] }
```

## API Surface

- `Button`
- `ButtonVariant`
- `ButtonSize`
- `button_class`

## Accessibility Notes

Use `disabled` for unavailable actions. Icon-only buttons should provide an
accessible label through the consuming app.
