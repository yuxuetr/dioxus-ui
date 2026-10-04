# Context Menu

Context Menu provides controlled menu parts for application commands. It reuses
dropdown primitive defaults and adds checkbox, radio, and shortcut parts.

## Source Copy

```bash
dxui add context-menu
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["context-menu"] }
```

## API Surface

- `ContextMenuContent`
- `ContextMenuGroup`
- `ContextMenuLabel`
- `ContextMenuItem`
- `ContextMenuCheckboxItem`
- `ContextMenuRadioGroup`
- `ContextMenuRadioItem`
- `ContextMenuSeparator`
- `ContextMenuShortcut`
- `ContextMenuPrimitiveConfig`
- `ContextMenuDismissBehavior`, `ContextMenuSide`, `ContextMenuAlign`
- `context_menu_content_class`
- `context_menu_item_class`

The module also re-exports `DropdownPrimitiveConfig` for users importing from
`dioxus_ui::context_menu`.

## Behavior

`open`, the pointer position, and checkbox or radio state stay controlled by
the app. Open the menu from `oncontextmenu` and pass the position as
`anchor_point`:

```rust
let mut open = use_signal(|| false);
let mut point = use_signal(|| (0.0, 0.0));

rsx! {
  div {
    oncontextmenu: move |event| {
      event.prevent_default();
      let position = event.client_coordinates();
      point.set((position.x, position.y));
      open.set(true);
    },
    "Right-click here"
  }
  ContextMenuContent {
    open: open(),
    anchor_point: point(),
    on_open_change: move |next| open.set(next),
    ContextMenuItem { onclick: move |_| copy(), "Copy" }
  }
}
```

- With `anchor_point`, the menu is placed with its top-left corner at the point
  (`side` `Bottom`, `align` `Start`, `side_offset` `0`), flipping and shifting
  to stay 8 pixels inside the viewport. A context menu key fires
  `oncontextmenu` with coordinates at the focused element.
- Opening focuses the first enabled item. ArrowDown and ArrowUp move focus and
  wrap at the ends, skipping disabled items; Home and End jump to the first and
  last item; typing a prefix focuses the next matching item.
- Enter, Space, or a click on an enabled item, including checkbox and radio
  items, calls the item's `onclick` and then requests close. Disabled items do
  not call `onclick`.
- Escape, a pointer press outside, and focus moving outside request close per
  `dismiss` (default `DismissBehavior::popover_default()`). Closing returns
  focus to the element focused before opening, unless focus already moved to
  another control.

Only the Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Content uses `role="menu"`. Items use `menuitem`, `menuitemcheckbox`, or
`menuitemradio` roles, and the menu moves DOM focus between them. Nested
submenus are not implemented.
