# Dropdown

Dropdown provides primitive menu configuration with styled content, group,
label, item, checkbox, radio, submenu, separator, and shortcut parts.

## Source Copy

```bash
dxui add dropdown
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["dropdown"] }
```

## API Surface

- `Dropdown`
- `DropdownTrigger`
- `DropdownContent`
- `DropdownGroup`
- `DropdownLabel`
- `DropdownItem`
- `DropdownCheckboxItem`
- `DropdownRadioGroup`
- `DropdownRadioItem`
- `DropdownSeparator`
- `DropdownShortcut`
- `DropdownSub`, `DropdownSubTrigger`, `DropdownSubContent`
- `DropdownPrimitiveConfig`
- `DropdownDismissBehavior`, `DropdownSide`, `DropdownAlign`
- `dropdown_content_class`
- `dropdown_item_class`
- `dropdown_inset_item_class`
- `dropdown_checkbox_item_class`
- `dropdown_radio_item_class`
- `dropdown_shortcut_class`
- `dropdown_sub_trigger_class`

## Behavior

`Dropdown` owns whether the menu is open and its parts read it, so they must
sit inside it (see [RFC 0077](../rfcs/0077-component-owned-state.md)).
`DropdownTrigger` toggles it and anchors the menu; it renders an unstyled
`button`, so style it with `class`. Pass `open` and `on_open_change` to
control it, or `default_open` to start it open. A `DropdownRadioGroup` owns
its value the same way, with `value`, `default_value`, and
`on_value_change`, and its items take a `value`. A checkbox item keeps
`checked`: its `onclick` reports activation, and the app flips it:

```rust
let mut status_bar = use_signal(|| true);

rsx! {
  Dropdown {
    DropdownTrigger {
      class: button_class(ButtonVariant::Outline, ButtonSize::Md, use_density(), ""),
      "View"
    }
    DropdownContent {
      DropdownCheckboxItem {
        checked: status_bar(),
        onclick: move |_| status_bar.toggle(),
        "Status bar"
        DropdownShortcut { "Ctrl+/" }
      }
      DropdownRadioGroup { default_value: "bottom",
        DropdownRadioItem { value: "bottom", "Panel bottom" }
        DropdownRadioItem { value: "right", "Panel right" }
      }
    }
  }
}
```

Checkbox items show a check mark and radio items a dot while checked, in an
inset before the label; pass `inset: true` to plain items to line them up.

- The menu is placed next to the trigger using fixed positioning, flips to
  the opposite side when the preferred side lacks room, shifts to stay inside
  the viewport, and follows resize and scroll. `side_offset` defaults to `4`
  pixels.
- `side` defaults to `Bottom` and `align` to `End`, matching
  `DropdownPrimitiveConfig`.
- Escape, a pointer press outside the content and trigger, and focus moving
  outside close it per `dismiss` (default
  `DismissBehavior::popover_default()`).
- Opening focuses the first enabled item. ArrowDown and ArrowUp move focus
  and wrap at the ends, skipping disabled items; Home and End jump to the
  first and last item; typing a prefix focuses the next matching item.
- Enter, Space, or a click on an enabled item, including checkbox and radio
  items, calls the item's `onclick` and then closes the menu. Disabled items
  do not call `onclick`.
- Closing returns focus to the trigger, unless focus already moved to another
  control. Tab moves focus out
  of the menu, which closes it.

### Submenus

`DropdownSub` holds a nested menu ([RFC 0067](../rfcs/0067-menu-submenus.md))
and owns whether it is open; pass `open` and `on_open_change` to control it:

```rust
rsx! {
  DropdownSub {
    DropdownSubTrigger { "Share" }
    DropdownSubContent {
      DropdownItem { onclick: move |_| copy_link(), "Copy link" }
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

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

Content uses menu semantics, and items use `menuitem`, `menuitemcheckbox`, or
`menuitemradio` semantics with `aria-checked`, so the checked state is
announced as well as drawn. Sub triggers use `aria-haspopup="menu"`,
`aria-expanded`, and `aria-controls`, and each submenu sits in a `group` and
takes its name from its trigger. The menu moves
DOM focus between items, so screen readers announce each item as it receives
focus. Destructive items need text that names the action, not only color.
Disabled items set `aria-disabled` next to `data-disabled` (see
[RFC 0055](../rfcs/0055-open-state-accessibility-audit.md)).
