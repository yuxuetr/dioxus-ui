# Dropdown

Dropdown provides primitive menu configuration with styled content, group,
label, item, and separator parts.

## Source Copy

```bash
dxui add dropdown
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["dropdown"] }
```

## API Surface

- `DropdownContent`
- `DropdownGroup`
- `DropdownLabel`
- `DropdownItem`
- `DropdownSeparator`
- `DropdownPrimitiveConfig`
- `dropdown_content_class`
- `dropdown_item_class`

## Accessibility Notes

Dropdown menus need roving focus, keyboard navigation, escape dismissal, and
clear disabled or destructive item states.
