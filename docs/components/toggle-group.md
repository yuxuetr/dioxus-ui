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

## Behavior

The pressed items stay controlled by the app. Pass `pressed` to each item and
handle `on_toggle`, which receives the toggled item's value:

```rust
let mut align = use_signal(|| Some("left".to_string()));

rsx! {
  ToggleGroup {
    on_toggle: move |value: String| {
      align.set(toggle_group_single_selection(align().as_deref(), &value))
    },
    ToggleGroupItem { value: "left", pressed: align().as_deref() == Some("left"), "Left" }
    ToggleGroupItem { value: "right", pressed: align().as_deref() == Some("right"), "Right" }
  }
}
```

- The items form one Tab stop: the item that last had focus, or the first
  pressed item, or the first enabled item.
- Arrow keys for `orientation` move focus between enabled items without
  pressing them, wrapping when `looping` is on; the default `Both` accepts all
  four arrows. Home and End jump to the first and last.
- In a right-to-left layout, ArrowLeft moves to the next item and ArrowRight
  to the previous one. Up and Down do not change.
- A click, Enter, or Space on an item calls `on_toggle` with its value. Use
  `toggle_group_single_selection` or `toggle_group_multiple_selection` to
  compute the next selection.

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
