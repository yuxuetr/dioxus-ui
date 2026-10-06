# Dialog

Dialog is a `Dialog` root that owns whether the dialog is open, with
styled trigger, overlay, content, title, description, and close parts.

## Source Copy

```bash
dxui add dialog
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["dialog"] }
```

## API Surface

- `Dialog`
- `DialogTrigger`
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

`Dialog` owns whether the dialog is open and its parts read it, so they
must sit inside it (see
[RFC 0077](../rfcs/0077-component-owned-state.md)). `DialogTrigger` opens
it; it renders an unstyled `button`, so style it with `class`:

```rust
rsx! {
  Dialog {
    DialogTrigger {
      class: button_class(ButtonVariant::Outline, ButtonSize::Md, UiDensity::Comfortable, ""),
      "Rename project"
    }
    DialogOverlay {}
    DialogContent {
      DialogTitle { "Rename project" }
      DialogClose { "Cancel" }
    }
  }
}
```

`default_open` starts it open. To control it, pass `open` and
`on_open_change`; then any app button can open or close it, and the trigger
is optional:

```rust
let mut open = use_signal(|| false);

rsx! {
  Dialog { open: open(), on_open_change: move |next| open.set(next),
    DialogOverlay {}
    DialogContent {
      DialogTitle { "Rename project" }
      Button { onclick: move |_| open.set(false), "Save" }
    }
  }
}
```

`on_open_change` hears every change the user makes, in both modes. The
trigger points `aria-controls` at the content and sets `aria-expanded`.

While open, the dialog locks page scroll and pads the root element for the
hidden scrollbar; nested modals share the lock, and the last to close
restores scrolling (see [RFC 0068](../rfcs/0068-modal-scroll-lock.md)).

- Escape on the content closes it when `dismiss.escape_key` is set.
- A click on the overlay closes it when `dismiss.outside_pointer` is set.
  The default `DismissBehavior::dialog_default()` leaves it off.
- `DialogClose` always closes it.
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
