# Switch

Switch provides a controlled on/off control with styled track and thumb parts.

## Source Copy

```bash
dxui add switch
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["switch"] }
```

## API Surface

- `Switch`
- `switch_class`
- `switch_thumb_class`

## Accessibility Notes

Use switch semantics for settings that take effect immediately. Use checkbox
semantics instead when the value is submitted as part of a form.
