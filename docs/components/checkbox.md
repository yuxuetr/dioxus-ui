# Checkbox

Checkbox provides a controlled boolean input with styled checked and unchecked
states.

## Source Copy

```bash
dxui add checkbox
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["checkbox"] }
```

## API Surface

- `Checkbox`
- `checkbox_class`

## Accessibility Notes

Pair checkboxes with a visible label or accessible name. Keep checked state in
application state so form and keyboard behavior remain predictable.
