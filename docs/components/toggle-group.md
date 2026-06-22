# Toggle Group

Toggle Group provides grouped pressed controls with single or multiple
selection helpers and roving focus helpers.

## Source Copy

```bash
dxui add toggle-group
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["toggle-group"] }
```

## API Surface

- `ToggleGroup`
- `ToggleGroupItem`
- `ToggleGroupType`
- `toggle_group_class`
- `toggle_group_item_class`
- `toggle_group_single_selection`
- `toggle_group_multiple_selection`
- `toggle_group_move_value`
- `NavigationOrientation`
- `RovingFocusItem`
- `FocusMove`

## Accessibility Notes

Toggle Group items render buttons with `aria-pressed`. Use single mode when
only one item can be active, and multiple mode when each item is independently
toggleable. Use the roving focus helpers to keep arrow-key movement consistent
across enabled items.
