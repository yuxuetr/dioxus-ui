# Menubar

Menubar provides controlled horizontal menu parts for application commands. It
reuses dropdown primitive defaults and shares item semantics with Context Menu.

## Source Copy

```bash
dxui add menubar
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["menubar"] }
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
- `MenubarPrimitiveConfig`
- `MenubarDismissBehavior`, `MenubarAlign`, `MenubarSide`
- `menubar_class`
- `menubar_trigger_class`
- `menubar_item_class`

The module also re-exports `DropdownPrimitiveConfig`, `DismissBehavior`,
`OverlayAlign`, and `OverlaySide` for users importing from
`dioxus_ui::menubar`.

## Behavior

The open menu stays controlled by the app. Keep the open menu's value, give
each `MenubarMenu` a `value`, give each trigger an `id`, and pass it as the
content's `anchor_id`:

```rust
let mut active = use_signal(|| None::<String>);
let is_open = move |value: &str| active().as_deref() == Some(value);

rsx! {
  Menubar {
    on_value_change: move |value: String| active.set(Some(value)),
    MenubarMenu {
      value: "file",
      MenubarTrigger {
        id: "file-trigger",
        open: is_open("file"),
        on_open_change: move |open: bool| active.set(open.then(|| "file".to_string())),
        "File"
      }
      MenubarContent {
        open: is_open("file"),
        anchor_id: "file-trigger",
        on_open_change: move |_| active.set(None),
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
  `side_offset` `4`), flips and shifts to stay in the viewport, and requests
  close on Escape, a pointer press outside, or focus moving outside, per
  `dismiss` (default `DismissBehavior::popover_default()`).
- An open menu behaves like Dropdown: the first enabled item takes focus,
  ArrowDown and ArrowUp wrap, Home, End, and typeahead jump, and Enter, Space,
  or a click on an enabled item calls its `onclick` and requests close.
- While a menu is open, Left and Right call `on_value_change` with the
  adjacent menu's value, and moving the pointer over another enabled trigger
  calls it with that trigger's menu value.
- In a right-to-left layout, ArrowLeft moves to the next trigger or menu and
  ArrowRight to the previous one.
- Closing returns focus to the open menu's trigger, unless focus already
  moved to another control. Tab moves focus out of the menu, which closes it.

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

Root uses `role="menubar"`, triggers use `role="menuitem"` with
`aria-haspopup="menu"` and `aria-expanded`, and content uses `role="menu"`.
The open menu moves DOM focus between items, so screen readers announce each
item as it receives focus. Submenus and ArrowUp opening on the last item are
not implemented (see [RFC 0015](../rfcs/0015-menubar-keyboard-behavior.md)).
