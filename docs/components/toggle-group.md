# Toggle Group

Toggle Group provides grouped pressed controls with single or multiple
selection helpers and roving focus helpers.

## Source Copy

```bash
dxui add toggle-group
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["toggle-group"] }
```

## API Surface

- `ToggleGroup`
- `ToggleGroupItem`
- `ToggleGroupType`
- `toggle_group_class`
- `toggle_group_item_class`
- `toggle_group_move_value`
- `NavigationOrientation`
- `RovingFocusItem`
- `FocusMove`

## Behavior

`ToggleGroup` owns which items are pressed
([RFC 0077](../rfcs/0077-component-owned-state.md)). In the default single
mode one item is pressed, named by `default_value` or, to control it,
`value`; the empty string means none. `on_value_change` hears every change,
including `""` when the pressed item is released:

```rust
rsx! {
  ToggleGroup { "aria-label": "Alignment", default_value: "left",
    ToggleGroupItem { value: "left", "Left" }
    ToggleGroupItem { value: "right", "Right" }
  }
}
```

With `selection_type: ToggleGroupType::Multiple`, any number of items are
pressed, and the group takes `values`, `default_values`, and
`on_values_change` instead.

- The items form one Tab stop: the item that last had focus, or the first
  pressed item, or the first enabled item.
- Arrow keys for `orientation` move focus between enabled items without
  pressing them, wrapping when `looping` is on; the default `Both` accepts all
  four arrows. Home and End jump to the first and last.
- In a right-to-left layout, ArrowLeft moves to the next item and ArrowRight
  to the previous one. Up and Down do not change.
- A click, Enter, or Space on an item presses it, or releases it when
  pressed.
- A `ToggleGroupItem` outside a `ToggleGroup` renders nothing and logs which
  root it is missing.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Toggle Group renders `role="group"` and items render buttons with
`aria-pressed`. Use single mode when only one item can be active, and multiple
mode when each item is independently toggleable. Arrow keys follow the text
direction (see [RFC 0025](../rfcs/0025-right-to-left-arrow-mirroring.md)).

`ToggleGroup` passes through attributes, so name the group with `aria-label` or
`aria-labelledby` (see [RFC 0040](../rfcs/0040-composite-widget-names.md)).
The group reports its orientation as `data-orientation` only, since
`role="group"` does not support `aria-orientation`.
