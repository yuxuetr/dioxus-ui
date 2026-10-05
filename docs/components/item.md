# Item

Item provides generic list or result composition parts. It is useful for
settings rows, search results, command results, and navigation rows when the
consuming app owns the collection behavior.

## Source Copy

```bash
dxui add item
```

This generates:

```text
src/components/ui/item.rs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["item"] }
```

## API Surface

- `Item`
- `ItemMedia`
- `ItemContent`
- `ItemTitle`
- `ItemDescription`
- `ItemActions`
- class helpers for every part

## Accessibility Notes

- `Item` exposes selected and disabled state through data attributes.
- `Item` does not add listbox, menu, table, or link semantics.
- Apps own click handlers, navigation, collection roles, keyboard behavior, and
  `aria-selected` where a specific widget pattern requires it.
