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

## Accessibility Notes

Content uses `role="dialog"` and `aria-modal="true"`. Runtime focus trapping,
portal mounting, touch gestures, and drag-to-dismiss behavior are deferred.
