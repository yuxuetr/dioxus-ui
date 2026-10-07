# Alert Dialog

Alert Dialog provides modal confirmation parts for destructive or high-impact
actions. It reuses dialog primitive configuration defaults.

## Source Copy

```bash
dxui add alert-dialog
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["alert-dialog"] }
```

## API Surface

- `AlertDialog`
- `AlertDialogTrigger`
- `AlertDialogOverlay`
- `AlertDialogContent`
- `AlertDialogHeader`
- `AlertDialogFooter`
- `AlertDialogTitle`
- `AlertDialogDescription`
- `AlertDialogAction`
- `AlertDialogCancel`
- `AlertDialogActionVariant`
- `AlertDialogPrimitiveConfig`
- `alert_dialog_overlay_class`
- `alert_dialog_content_class`
- `alert_dialog_action_class`

The module also re-exports `DialogPrimitiveConfig` for users importing from
`dioxus_shadcn::alert_dialog`.

## Behavior

`AlertDialog` owns whether the alert dialog is open and its parts read it,
so they must sit inside it (see
[RFC 0077](../rfcs/0077-component-owned-state.md)). `AlertDialogTrigger`
opens it; it renders an unstyled `button`, so style it with `class`. Pass
`open` and `on_open_change` to control it instead, or `default_open` to
start it open; `on_open_change` hears every change in both modes.

While open, the alert dialog locks page scroll and pads the root element for the
hidden scrollbar; nested modals share the lock, and the last to close
restores scrolling (see [RFC 0068](../rfcs/0068-modal-scroll-lock.md)).

```rust
rsx! {
  AlertDialog {
    AlertDialogTrigger {
      class: button_class(ButtonVariant::Destructive, ButtonSize::Md, use_density(), ""),
      "Delete project"
    }
    AlertDialogOverlay {}
    AlertDialogContent {
      AlertDialogTitle { "Delete project?" }
      AlertDialogCancel { "Cancel" }
      AlertDialogAction { onclick: move |_| delete_project(), "Delete" }
    }
  }
}
```

- Escape on the content closes it when `dismiss.escape_key` is set.
- The overlay never dismisses an alert dialog.
- `AlertDialogCancel` and `AlertDialogAction` run their `onclick` and then
  close it.
- Opening focuses the element marked `data-dxui-autofocus`, such as a
  keyboard-managed Calendar day, otherwise the first focusable element, or the
  content itself.
- Tab and Shift+Tab wrap inside the content while it is open, skipping
  `tabindex="-1"` elements.
- Closing, or removing the content from the tree, restores focus to the element
  that was focused before opening, unless focus had already moved to a control
  outside the content.

Render the cancel action before the destructive action so it receives initial
focus. Only the Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Content uses `role="alertdialog"` and `aria-modal="true"`. Destructive actions
should be clearly labeled by the consuming app, and the cancel action should be
available before committing the destructive operation.

Portal mounting and transitions remain app-owned.

`AlertDialogContent` takes its name from a `AlertDialogTitle` inside it and its description from a
`AlertDialogDescription`: the parts get generated ids, and the content points
`aria-labelledby` and `aria-describedby` at the ones that are mounted. A
passed `aria-label`, `aria-labelledby`, or `aria-describedby` on `AlertDialogContent`
replaces the generated value, so give a title-less one an `aria-label` (see
[RFC 0039](../rfcs/0039-dialog-names.md)).
