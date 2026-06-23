# Sheet

Sheet provides a modal side panel for settings, secondary forms, and contextual
workflows. It reuses dialog primitive configuration defaults and adds side-based
content classes.

## Source Copy

```bash
dxui add sheet
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["sheet"] }
```

## API Surface

- `SheetOverlay`
- `SheetContent`
- `SheetHeader`
- `SheetFooter`
- `SheetTitle`
- `SheetDescription`
- `SheetClose`
- `SheetSide`
- `SheetPrimitiveConfig`
- `sheet_overlay_class`
- `sheet_content_class`

The module also re-exports `DialogPrimitiveConfig` for users importing from
`dioxus_ui::sheet`.

## Accessibility Notes

Content uses `role="dialog"` and `aria-modal="true"`. The first implementation
exposes `data-side` and `data-state` hooks but defers runtime focus trapping,
portal mounting, and transition orchestration.
