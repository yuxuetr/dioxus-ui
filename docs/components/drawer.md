# Drawer

Drawer provides a mobile-oriented bottom modal for task flows. It is a separate
public component from Sheet because it has bottom-first sizing and mobile usage
guidance, but it reuses dialog primitive configuration defaults.

## Source Copy

```bash
dxui add drawer
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["drawer"] }
```

## API Surface

- `DrawerOverlay`
- `DrawerContent`
- `DrawerHeader`
- `DrawerFooter`
- `DrawerTitle`
- `DrawerDescription`
- `DrawerClose`
- `DrawerPrimitiveConfig`
- `drawer_overlay_class`
- `drawer_content_class`

The module also re-exports `DialogPrimitiveConfig` for users importing from
`dioxus_ui::drawer`.

## Behavior

`open` stays controlled by the app. Pass the same `on_open_change` handler to
`DrawerOverlay`, `DrawerContent`, and `DrawerClose` to receive close requests.

- Escape on the content requests close when `dismiss.escape_key` is set.
- A click on the overlay requests close when `dismiss.outside_pointer` is set.
  The default `DismissBehavior::dialog_default()` leaves it off.
- `DrawerClose` always requests close.
- Opening focuses the first focusable element, or the content itself.
- Tab and Shift+Tab wrap inside the content while it is open.
- Closing, or removing the content from the tree, restores focus to the element
  that was focused before opening.

Drawer shares the Dialog focus scope; the browser smoke covers it through Dialog
and Alert Dialog.

## Accessibility Notes

Content uses `role="dialog"` and `aria-modal="true"`. Portal mounting,
touch gestures, and drag-to-dismiss behavior are deferred.
