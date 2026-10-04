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
- Opening focuses the first enabled item. ArrowDown and ArrowUp move focus
  and wrap at the ends, skipping disabled items; Home and End jump to the
  first and last item; typing a prefix focuses the next matching item.
- Enter, Space, or a click on an enabled item calls the item's `onclick` and
  then requests close. Disabled items do not call `onclick`.
- Closing returns focus to the element focused before opening, usually the
  trigger, unless focus already moved to another control. Tab moves focus out
  of the menu, which closes it.

Only the Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Content uses menu semantics and items use menuitem semantics. The menu moves
DOM focus between items, so screen readers announce each item as it receives
focus. Destructive items need text that names the action, not only color.
