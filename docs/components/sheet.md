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
dioxus-shadcn = { version = "0.1", default-features = false, features = ["sheet"] }
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
`dioxus_shadcn::sheet`.

## Behavior

`open` stays controlled by the app. Pass the same `on_open_change` handler to
`SheetOverlay`, `SheetContent`, and `SheetClose` to receive close requests.

- Escape on the content requests close when `dismiss.escape_key` is set.
- A click on the overlay requests close when `dismiss.outside_pointer` is set.
  The default `DismissBehavior::dialog_default()` leaves it off.
- `SheetClose` always requests close.
- Opening focuses the element marked `data-dxui-autofocus`, such as a
  keyboard-managed Calendar day, otherwise the first focusable element, or the
  content itself.
- Tab and Shift+Tab wrap inside the content while it is open, skipping
  `tabindex="-1"` elements.
- Closing, or removing the content from the tree, restores focus to the element
  that was focused before opening, unless focus had already moved to a control
  outside the content.

Sheet shares the Dialog focus scope; the browser smoke covers it through Dialog
and Alert Dialog.

## Accessibility Notes

Content uses `role="dialog"` and `aria-modal="true"`. Content
exposes `data-side` and `data-state` hooks. Portal mounting and transition
orchestration remain app-owned.

`SheetContent` takes its name from a `SheetTitle` inside it and its description from a
`SheetDescription`: the parts get generated ids, and the content points
`aria-labelledby` and `aria-describedby` at the ones that are mounted. A
passed `aria-label`, `aria-labelledby`, or `aria-describedby` on `SheetContent`
replaces the generated value, so give a title-less one an `aria-label` (see
[RFC 0039](../rfcs/0039-dialog-names.md)).
