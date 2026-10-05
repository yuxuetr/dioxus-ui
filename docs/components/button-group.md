# Button Group

Button Group provides a grouped layout for related command buttons.

## Source Copy

```bash
dxui add button-group
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["button-group"] }
```

## API Surface

- `ButtonGroup`
- `ButtonGroupItem`
- `ButtonGroupOrientation`
- `button_group_class`
- `button_group_item_class`

## Accessibility Notes

Button Group renders a `role="group"` wrapper and native button items. Pass
`aria_label` when the group needs an accessible name. Apps own icon-only
accessible names, command behavior, pressed state, and any toolbar or
roving-focus semantics. `ButtonGroupItem` takes `onclick`, and other button
attributes, such as `aria-pressed` and `aria-label`, pass through. The group
reports its orientation as `data-orientation` only, since `role="group"` does
not support `aria-orientation`.
