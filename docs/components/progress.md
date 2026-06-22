# Progress

Progress displays completion for a bounded task.

## Source Copy

```bash
dxui add progress
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["progress"] }
```

## API Surface

- `Progress`
- `progress_class`
- `progress_indicator_class`
- `progress_percent`

## Accessibility Notes

Progress uses `role="progressbar"` with value attributes. Provide surrounding
text when users need to know what task is progressing.
