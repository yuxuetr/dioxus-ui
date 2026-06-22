# Spinner

Spinner provides a compact loading status indicator.

## Source Copy

```bash
dxui add spinner
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["spinner"] }
```

## API Surface

- `Spinner`
- `SpinnerSize`
- `spinner_class`

## Accessibility Notes

Spinner renders with `role="status"` and an accessible label. Keep the default
label for generic loading states, or pass a more specific label when the
surrounding UI has multiple asynchronous regions.
