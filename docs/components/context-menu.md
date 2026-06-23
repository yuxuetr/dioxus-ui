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
- `context_menu_content_class`
- `context_menu_item_class`

The module also re-exports `DropdownPrimitiveConfig` for users importing from
`dioxus_ui::context_menu`.

## Accessibility Notes

Content uses `role="menu"`. Items use `menuitem`, `menuitemcheckbox`, or
`menuitemradio` roles. Roving focus, typeahead, context-trigger anchoring, and
nested submenu focus handoff remain runtime adapter work.
