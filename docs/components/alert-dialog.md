# Alert Dialog

Alert Dialog provides modal confirmation parts for destructive or high-impact
actions. It reuses dialog primitive configuration defaults.

## Source Copy

```bash
dxui add alert-dialog
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["alert-dialog"] }
```

## API Surface

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

`open` stays controlled by the app. Pass the same `on_open_change` handler to
`AlertDialogContent`, `AlertDialogCancel`, and `AlertDialogAction`:

```rust
let mut open = use_signal(|| false);

rsx! {
  AlertDialogOverlay { open: open() }
  AlertDialogContent {
    open: open(),
    on_open_change: move |next| open.set(next),
    AlertDialogTitle { "Delete project?" }
    AlertDialogCancel { on_open_change: move |next| open.set(next), "Cancel" }
    AlertDialogAction {
      onclick: move |_| delete_project(),
      on_open_change: move |next| open.set(next),
      "Delete"
    }
  }
}
```

- Escape on the content requests close when `dismiss.escape_key` is set.
- The overlay never dismisses an alert dialog.
- `AlertDialogCancel` requests close; `AlertDialogAction` runs `onclick` and
  then requests close.
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
