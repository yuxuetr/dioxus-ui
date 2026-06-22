# Input

Input is a styled single-line text field with invalid and density-aware class
support.

## Source Copy

```bash
dxui add input
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["input"] }
```

## API Surface

- `Input`
- `input_class`

## Accessibility Notes

Pair inputs with `Label` and expose validation state with `aria-invalid` when
the field is invalid.
