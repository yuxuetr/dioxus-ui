# Sonner

Sonner provides opinionated toast notification parts and pure queue helpers.
Each toast dismisses itself after a countdown that pauses on hover and focus.
It does not own promise orchestration, portal mounting, or focus movement.

## Source Copy

```bash
dxui add sonner
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["sonner"] }
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
- `SonnerDismissReason`
- `sonner_dismiss_reason_attribute`

## Behavior

The app owns the queue and renders one `SonnerToast` per item. Pass the same
`on_dismiss` handler to `SonnerToast`, `SonnerAction`, and `SonnerClose`; it
receives a `SonnerDismissReason`, usually followed by `sonner_queue_dismiss`.

- `SonnerToast` calls `on_dismiss(Timeout)` after `duration_ms` (default
  `5000`) of open time, pausing while the pointer is over the toast or focus is
  inside it. `0` disables the countdown, which suits loading toasts.
- `open` defaults to `true`; set it to `false` to hide a toast without
  unmounting it.
- `SonnerAction` runs `onclick`, then calls `on_dismiss(Action)`.
- `SonnerClose` calls `on_dismiss(Close)`. It is a 28px icon button named
  "Close notification", so give it a glyph or icon rather than a word.

Sonner shares the Toast countdown script; only the Web renderer is covered by
`npm run verify:runtime-interactions`.

## Accessibility Notes

`SonnerViewport` is a persistent `role="region"` labelled `Notifications` with
`aria-live="polite"`. Render it once and add toasts inside it, so assistive
technology announces additions to a region it already tracks.

The toast root exposes status semantics and `aria-live` based on variant
urgency. Variant icons are decorative. The countdown pauses on hover and focus
so the message stays readable. Apps own announcement wording, promise state,
portal placement, escape-key behavior, and focus policy.
