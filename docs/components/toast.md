# Toast

Toast provides controlled notification composition parts and pure queue helpers.
It does not own timers, portal mounting, focus movement, or live-region runtime.

## Source Copy

```bash
dxui add toast
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["toast"] }
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

## Behavior

The app owns the queue. Pass one `on_dismiss` handler to `ToastRoot`,
`ToastAction`, and `ToastClose`; it receives the `ToastDismissReason`:

```rust
let dismiss = move |reason: ToastDismissReason| {
  queue.set(toast_queue_dismiss(queue(), &id));
  log::info!("toast dismissed: {}", toast_dismiss_reason_attribute(reason));
};

rsx! {
  ToastRoot {
    on_dismiss: dismiss,
    ToastTitle { "Changes saved" }
    ToastAction { onclick: move |_| undo(), on_dismiss: dismiss, "Undo" }
    ToastClose { on_dismiss: dismiss, "×" }
  }
}
```

- `ToastRoot` calls `on_dismiss(Timeout)` after `duration_ms` (default `5000`)
  of open time. The countdown pauses while the pointer is over the toast or
  focus is inside it, and resumes with the remaining time. `0` disables it,
  which suits loading toasts.
- `ToastAction` runs `onclick`, then calls `on_dismiss(Action)`.
- `ToastClose` calls `on_dismiss(Close)`. It is a 28px icon button named
  "Close notification", so give it a glyph or icon rather than a word.

The countdown runs through `document::eval`, so it works in the Web, Desktop,
and Mobile renderers. The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

`ToastViewport` is a persistent `role="region"` labelled `Notifications` with
`aria-live="polite"`. Render it once and add toasts inside it, so assistive
technology announces additions to a region it already tracks.

The root exposes status semantics and `aria-live` based on variant urgency.
Close and action controls are native buttons. The countdown pauses on hover and
focus so the message stays readable. Apps own announcement wording, portal
placement, escape-key behavior, and whether focus should move into a toast
action.
