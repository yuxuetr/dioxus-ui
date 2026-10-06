# Context Menu

Context Menu provides controlled menu parts for application commands. It reuses
dropdown primitive defaults and adds checkbox, radio, and shortcut parts.

## Source Copy

```bash
dxui add context-menu
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["context-menu"] }
```

## API Surface

- `ContextMenu`
- `ContextMenuTrigger`
- `ContextMenuContent`
- `ContextMenuGroup`
- `ContextMenuLabel`
- `ContextMenuItem`
- `ContextMenuCheckboxItem`
- `ContextMenuRadioGroup`
- `ContextMenuRadioItem`
- `ContextMenuSeparator`
- `ContextMenuShortcut`
- `ContextMenuSub`, `ContextMenuSubTrigger`, `ContextMenuSubContent`
- `ContextMenuPrimitiveConfig`
- `ContextMenuDismissBehavior`, `ContextMenuSide`, `ContextMenuAlign`
- `context_menu_content_class`
- `context_menu_item_class`
- `context_menu_checkbox_item_class`
- `context_menu_radio_item_class`
- `context_menu_sub_trigger_class`

The module also re-exports `DropdownPrimitiveConfig` for users importing from
`dioxus_shadcn::context_menu`.

## Behavior

`ContextMenu` owns whether the menu is open and its parts read it, so they
must sit inside it (see
[RFC 0077](../rfcs/0077-component-owned-state.md)). `ContextMenuTrigger` is
the area that opens it: a right click inside opens the menu at the pointer.
Pass `open` and `on_open_change` to control it, or `default_open` to start it
open. A `ContextMenuRadioGroup` owns its value with `value`,
`default_value`, and `on_value_change`, and its items take a `value`; a
checkbox item keeps `checked`, which the app flips in its `onclick`. Checked
checkbox items show a check mark and checked radio items a dot in their
inset:

```rust
rsx! {
  ContextMenu {
    ContextMenuTrigger { class: "h-32 border border-dashed", "Right-click here" }
    ContextMenuContent {
      ContextMenuItem { onclick: move |_| copy(), "Copy" }
    }
  }
}
```

- The menu is placed with its top-left corner at the pointer (`side`
  `Bottom`, `align` `Start`, `side_offset` `0`), flipping and shifting to stay
  8 pixels inside the viewport. A context menu key opens it at the focused
  element.
- Opening focuses the first enabled item. ArrowDown and ArrowUp move focus and
  wrap at the ends, skipping disabled items; Home and End jump to the first and
  last item; typing a prefix focuses the next matching item.
- Enter, Space, or a click on an enabled item, including checkbox and radio
  items, calls the item's `onclick` and then closes the menu. Disabled items do
  not call `onclick`.
- Escape, a pointer press outside, and focus moving outside close it per
  `dismiss` (default `DismissBehavior::popover_default()`). Closing returns
  focus to the element focused before opening, unless focus already moved to
  another control.

Only the Web renderer is covered by `npm run verify:runtime-interactions`.


### Submenus

`ContextMenuSub` holds a nested menu
([RFC 0067](../rfcs/0067-menu-submenus.md)) and owns whether it is open; pass
`open` and `on_open_change` to control it:

```rust
rsx! {
  ContextMenuSub {
    ContextMenuSubTrigger { "Share" }
    ContextMenuSubContent {
      ContextMenuItem { onclick: move |_| copy_link(), "Copy link" }
    }
  }
}
```

- ArrowRight (ArrowLeft in right-to-left), Enter, Space, or a click on the
  trigger opens the submenu and focuses its first item; hovering the trigger
  opens it without moving focus.
- ArrowLeft (ArrowRight in right-to-left) or Escape inside the submenu closes
  that level and returns focus to the trigger.
- Choosing an item inside closes every level. Highlighting another item of
  the outer menu, or closing the outer menu, closes the submenu.
- The submenu opens at the trigger's inline end and flips when there is no
  room. Moving the pointer diagonally across sibling items closes it; there
  is no pointer grace area.

## Accessibility Notes

Content uses `role="menu"`. Items use `menuitem`, `menuitemcheckbox`, or
`menuitemradio` roles, and the menu moves DOM focus between them. Sub
triggers use `aria-haspopup="menu"`, `aria-expanded`, and `aria-controls`,
and each submenu sits in a `group` and takes its name from its trigger.
