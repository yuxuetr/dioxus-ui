# Dialog

Dialog combines primitive configuration with styled overlay, content, title,
description, and close parts.

## Source Copy

```bash
dxui add dialog
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["dialog"] }
```

## API Surface

- `DialogOverlay`
- `DialogContent`
- `DialogTitle`
- `DialogDescription`
- `DialogClose`
- `DialogPrimitiveConfig`
- `DismissBehavior` (the `dismiss` prop)
- `dialog_overlay_class`
- `dialog_content_class`

## Behavior

`open` stays controlled by the app. Pass the same `on_open_change` handler to
`DialogOverlay`, `DialogContent`, and `DialogClose` to receive close requests:

While open, the dialog locks page scroll and pads the root element for the
hidden scrollbar; nested modals share the lock, and the last to close
restores scrolling (see [RFC 0068](../rfcs/0068-modal-scroll-lock.md)).

```rust
let mut open = use_signal(|| false);

rsx! {
  DialogOverlay { open: open(), on_open_change: move |next| open.set(next) }
  DialogContent {
    open: open(),
    on_open_change: move |next| open.set(next),
    DialogTitle { "Rename project" }
    DialogClose { on_open_change: move |next| open.set(next), "Cancel" }
  }
}
```

- Escape on the content requests close when `dismiss.escape_key` is set.
- A click on the overlay requests close when `dismiss.outside_pointer` is set.
  The default `DismissBehavior::dialog_default()` leaves it off.
- `DialogClose` always requests close.
- Opening focuses the element marked `data-dxui-autofocus`, such as a
  keyboard-managed Calendar day, otherwise the first focusable element, or the
  content itself.
- Tab and Shift+Tab wrap inside the content while it is open, skipping
  `tabindex="-1"` elements.
- Closing, or removing the content from the tree, restores focus to the element
  that was focused before opening, unless focus had already moved to a control
  outside the content.

Focus handling runs through `document::eval`, so it works in the Web, Desktop,
and Mobile renderers. The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

Dialogs trap focus, restore focus on close, and dismiss according to the
configured escape-key and outside-interaction behavior. Always render a
`DialogTitle`.

`DialogContent` takes its name from a `DialogTitle` inside it and its description from a
`DialogDescription`: the parts get generated ids, and the content points
`aria-labelledby` and `aria-describedby` at the ones that are mounted. A
passed `aria-label`, `aria-labelledby`, or `aria-describedby` on `DialogContent`
replaces the generated value, so give a title-less one an `aria-label` (see
[RFC 0039](../rfcs/0039-dialog-names.md)).
