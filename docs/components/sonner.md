# Sonner

Sonner provides opinionated toast notification parts and pure queue helpers. It
does not own timers, promise orchestration, portal mounting, focus movement, or
live-region runtime.

## Source Copy

```bash
dxui add sonner
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["sonner"] }
```

## API Surface

- `SonnerViewport`
- `SonnerToast`
- `SonnerIcon`
- `SonnerContent`
- `SonnerTitle`
- `SonnerDescription`
- `SonnerAction`
- `SonnerClose`
- `SonnerItem`
- `SonnerQueue`
- `SonnerPlacement`
- `SonnerVariant`
- `sonner_queue_push`
- `sonner_queue_dismiss`
- `sonner_is_expired`

## Accessibility Notes

The toast root exposes status semantics and `aria-live` based on variant
urgency. Variant icons are decorative. Apps own announcement wording, timer
scheduling, promise state, portal placement, escape-key behavior, and focus
policy.
