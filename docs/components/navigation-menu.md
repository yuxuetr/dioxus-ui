# Navigation Menu

Navigation Menu provides controlled navigation-oriented parts. It keeps
navigation semantics instead of forcing command-menu roles.

## Source Copy

```bash
dxui add navigation-menu
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["navigation-menu"] }
```

## API Surface

- `NavigationMenu`
- `NavigationMenuList`
- `NavigationMenuItem`
- `NavigationMenuTrigger`
- `NavigationMenuContent`
- `NavigationMenuLink`
- `NavigationMenuViewport`
- `NavigationMenuIndicator`
- `NavigationMenuPrimitiveConfig`
- `navigation_menu_class`
- `navigation_menu_trigger_class`
- `navigation_menu_link_class`

The module also re-exports `PopoverPrimitiveConfig` for users importing from
`dioxus_ui::navigation_menu`.

## Accessibility Notes

Use Navigation Menu for navigation destinations. Use Menubar or Context Menu for
application commands. Runtime trigger roving focus, viewport measurement, and
portal positioning remain adapter work.
