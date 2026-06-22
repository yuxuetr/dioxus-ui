# Label

Label provides consistent text styling for form controls.

## Source Copy

```bash
dxui add label
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["label"] }
```

## API Surface

- `Label`
- `label_class`

## Accessibility Notes

Associate labels with their target control using `for` or equivalent Dioxus
attributes. Avoid using label styling for unrelated helper text.
