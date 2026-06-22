# Dialog

Dialog combines primitive configuration with styled overlay, content, title,
description, and close parts.

## Source Copy

```bash
dxui add dialog
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["dialog"] }
```

## API Surface

- `DialogOverlay`
- `DialogContent`
- `DialogTitle`
- `DialogDescription`
- `DialogClose`
- `DialogPrimitiveConfig`
- `dialog_overlay_class`
- `dialog_content_class`

## Accessibility Notes

Dialogs should trap focus, restore focus on close, expose a title, and dismiss
according to the configured escape-key and outside-interaction behavior.
