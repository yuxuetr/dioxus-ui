# Radio Group

Radio Group provides a controlled single-choice group with radio semantics and
roving focus helpers.

## Source Copy

```bash
dxui add radio-group
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["radio-group"] }
```

## API Surface

- `RadioGroup`
- `RadioGroupItem`
- `radio_group_class`
- `radio_group_item_class`
- `radio_group_move_value`
- `NavigationOrientation`
- `RovingFocusItem`
- `FocusMove`

## Accessibility Notes

Radio Group renders `role="radiogroup"` and items render `role="radio"` with
`aria-checked`. Use the roving focus helpers to move across enabled items and
skip disabled choices when handling arrow keys.
