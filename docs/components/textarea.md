# Textarea

Textarea is a styled multi-line text field with invalid and density-aware class
support.

## Source Copy

```bash
dxui add textarea
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["textarea"] }
```

## API Surface

- `Textarea`
- `textarea_class`

## Accessibility Notes

Pair textareas with `Label` and expose validation state with `aria-invalid`
when the field is invalid.
