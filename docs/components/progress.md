# Progress

Progress displays completion for a bounded task.

## Source Copy

```bash
dxui add progress
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["progress"] }
```

## API Surface

- `Progress`
- `progress_class`
- `progress_indicator_class`
- `progress_percent`

## Accessibility Notes

Progress uses `role="progressbar"` with value attributes. Provide surrounding
text when users need to know what task is progressing.

ARIA requires a progress bar to have a name. `Progress` passes through
attributes such as `aria-label`, `aria-labelledby`, and `aria-valuetext`
(see [RFC 0038](../rfcs/0038-form-control-naming.md)).
