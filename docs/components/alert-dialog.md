# Alert Dialog

Alert Dialog provides modal confirmation parts for destructive or high-impact
actions. It reuses dialog primitive configuration defaults.

## Source Copy

```bash
dxui add alert-dialog
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["alert-dialog"] }
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
`dioxus_ui::alert_dialog`.

## Accessibility Notes

Content uses `role="alertdialog"` and `aria-modal="true"`. Destructive actions
should be clearly labeled by the consuming app, and the cancel action should be
available before committing the destructive operation.

Focus trapping and automatic focus restoration are still runtime adapter work.
The primitive config documents the intended focus return and dismissal defaults.
