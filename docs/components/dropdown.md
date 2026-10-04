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

## Behavior

`open` stays controlled by the app. Give the trigger an `id`, pass it as
`anchor_id`, and pass `on_open_change` to `DropdownContent`.

- With `anchor_id`, content is placed next to the element with that id using
  fixed positioning, flips to the opposite side when the preferred side lacks
  room, shifts to stay inside the viewport, and follows resize and scroll.
  `side_offset` defaults to `4` pixels. Without `anchor_id`, content renders in
  place.
- `side` defaults to `Bottom` and `align` to `End`, matching
  `DropdownPrimitiveConfig`.
- Escape, a pointer press outside the content and anchor, and focus moving
  outside request close per `dismiss` (default
  `DismissBehavior::popover_default()`).
- Item roving focus and arrow-key navigation remain app-owned.

## Accessibility Notes

Dropdown menus need roving focus, keyboard navigation, escape dismissal, and
clear disabled or destructive item states.
