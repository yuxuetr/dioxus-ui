# Toast

Toast provides controlled notification composition parts and pure queue helpers.
It does not own timers, portal mounting, focus movement, or live-region runtime.

## Source Copy

```bash
dxui add toast
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["toast"] }
```

## API Surface

- `ToastViewport`
- `ToastRoot`
- `ToastTitle`
- `ToastDescription`
- `ToastAction`
- `ToastClose`
- `ToastItem`
- `ToastQueue`
- `ToastPlacement`
- `ToastVariant`
- `ToastDismissReason`
- `toast_queue_push`
- `toast_queue_dismiss`
- `toast_is_expired`

## Accessibility Notes

The root exposes status semantics and `aria-live` based on variant urgency.
Close and action controls are native buttons. Apps own announcement wording,
timer scheduling, portal placement, escape-key behavior, and whether focus
should move into a toast action.
