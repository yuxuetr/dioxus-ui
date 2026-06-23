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
- `menubar_class`
- `menubar_trigger_class`
- `menubar_item_class`

The module also re-exports `DropdownPrimitiveConfig` for users importing from
`dioxus_ui::menubar`.

## Accessibility Notes

Root uses `role="menubar"` and content uses `role="menu"`. Items use menu item
roles. Roving focus, typeahead, nested submenu behavior, and runtime focus
handoff remain adapter work.
