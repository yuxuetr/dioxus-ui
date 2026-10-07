# Menubar

Menubar provides controlled horizontal menu parts for application commands. It
reuses dropdown primitive defaults and shares item semantics with Context Menu.

## Source Copy

```bash
dxui add menubar
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["menubar"] }
```

## API Surface

- `Menubar`
- `MenubarMenu`
- `MenubarTrigger`
- `MenubarContent`
- `MenubarItem`
- `MenubarCheckboxItem`
- `MenubarRadioGroup`
- `MenubarRadioItem`
- `MenubarLabel`
- `MenubarSeparator`
- `MenubarShortcut`
- `MenubarSub`, `MenubarSubTrigger`, `MenubarSubContent`
- `MenubarPrimitiveConfig`
- `MenubarDismissBehavior`, `MenubarAlign`, `MenubarSide`
- `menubar_class`
- `menubar_trigger_class`
- `menubar_item_class`
- `menubar_checkbox_item_class`
- `menubar_radio_item_class`
- `menubar_sub_trigger_class`

The module also re-exports `DropdownPrimitiveConfig`, `DismissBehavior`,
`OverlayAlign`, and `OverlaySide` for users importing from
`dioxus_shadcn::menubar`.

## Behavior

`Menubar` owns which menu is open, by the `value` of its `MenubarMenu`, or
the empty string while none is, as Radix does (see
[RFC 0077](../rfcs/0077-component-owned-state.md)). Each menu's trigger and
content read it and link their ids themselves. Pass `value` and
`on_value_change` to control it, or `default_value` to start with a menu
open. A `MenubarMenu` without a `value` gets a generated one.

A `MenubarRadioGroup` owns its value with `value`, `default_value`, and
`on_value_change`, and its items take a `value`; a `MenubarCheckboxItem`
keeps `checked`, which the app flips in its `onclick`. Checked checkbox items
show a check mark and checked radio items a dot in their inset.

```rust
rsx! {
  Menubar { "aria-label": "Editor",
    MenubarMenu { value: "file",
      MenubarTrigger { "File" }
      MenubarContent {
        MenubarItem { onclick: move |_| new_file(), "New" }
      }
    }
  }
}
```

- The triggers form one Tab stop. Left and Right move focus between enabled
  triggers and wrap; Home and End jump to the first and last trigger.
- A click toggles a menu. ArrowDown, Enter, or Space on a closed trigger opens
  it.
- Content is placed below its trigger (`side` `Bottom`, `align` `Start`,
  `side_offset` `4`), flips and shifts to stay in the viewport, and closes on
  Escape, a pointer press outside, or focus moving outside, per
  `dismiss` (default `DismissBehavior::popover_default()`).
- An open menu behaves like Dropdown: the first enabled item takes focus,
  ArrowDown and ArrowUp wrap, Home, End, and typeahead jump, and Enter, Space,
  or a click on an enabled item calls its `onclick` and closes the menu.
- While a menu is open, Left and Right open the adjacent menu, and moving the
  pointer over another enabled trigger opens that trigger's menu.
- In a right-to-left layout, ArrowLeft moves to the next trigger or menu and
  ArrowRight to the previous one.
- Closing returns focus to the open menu's trigger, unless focus already
  moved to another control. Tab moves focus out of the menu, which closes it.

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.


### Submenus

`MenubarSub` holds a nested menu ([RFC 0067](../rfcs/0067-menu-submenus.md))
and owns whether it is open; pass `open` and `on_open_change` to control it:

```rust
rsx! {
  MenubarSub {
    MenubarSubTrigger { "Share" }
    MenubarSubContent {
      MenubarItem { onclick: move |_| copy_link(), "Copy link" }
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

Root uses `role="menubar"`, triggers use `role="menuitem"` with
`aria-haspopup="menu"` and `aria-expanded`, and `aria-controls` while open, and content uses `role="menu"`.
The open menu moves DOM focus between items, so screen readers announce each
item as it receives focus. Sub triggers use `aria-haspopup="menu"`,
`aria-expanded`, and `aria-controls`, and each submenu sits in a `group` and
takes its name from its trigger (see
[RFC 0067](../rfcs/0067-menu-submenus.md)). ArrowUp opening on the last item
is not implemented (see [RFC 0015](../rfcs/0015-menubar-keyboard-behavior.md)).

`Menubar` passes through attributes, so name the menu bar with `aria-label` (see
[RFC 0040](../rfcs/0040-composite-widget-names.md)).
