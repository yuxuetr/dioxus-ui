# Radio Group

Radio Group provides a controlled single-choice group with radio semantics and
roving focus helpers.

## Source Copy

```bash
dxui add radio-group
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["radio-group"] }
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

## Behavior

`RadioGroup` owns the checked value
([RFC 0077](../rfcs/0077-component-owned-state.md)). Start it with
`default_value`, or control it with `value`; `on_value_change` hears every
change the user makes either way. Items compare their `value` with the
group's:

```rust
rsx! {
  RadioGroup { default_value: "small",
    RadioGroupItem { value: "small", "aria-label": "Small" }
    RadioGroupItem { value: "large", "aria-label": "Large" }
  }
}
```

- The items form one Tab stop: the checked item, or the first enabled item
  when none is checked.
- Arrow keys for `orientation` move focus between enabled items, wrapping when
  `looping` is on; the default `Both` accepts all four arrows. Home and End
  jump to the first and last. Moving focus checks the focused item.
- In a right-to-left layout, ArrowLeft moves to the next item and ArrowRight
  to the previous one. Up and Down do not change.
- A click or Space on an item checks it.
- A `RadioGroupItem` outside a `RadioGroup` renders nothing and logs which
  root it is missing.

The Web renderer is covered by `npm run verify:runtime-interactions`.
`radio_group_move_value` and `radio_group_item_tabindex` remain for apps that
render their own items.

## Accessibility Notes

Radio Group renders `role="radiogroup"` and items render `role="radio"` with
`aria-checked`. Arrow keys follow the text direction (see
[RFC 0025](../rfcs/0025-right-to-left-arrow-mirroring.md)).

Items have no text of their own, so name each one. `RadioGroup` and
`RadioGroupItem` pass through attributes such as `id`, `aria-label`, and
`aria-labelledby` (see [RFC 0038](../rfcs/0038-form-control-naming.md)):

```rust
rsx! {
  Label { id: "size-label", "Size" }
  RadioGroup { "aria-labelledby": "size-label", default_value: "small",
    RadioGroupItem { id: "size-small", value: "small" }
    Label { r#for: "size-small", "Small" }
  }
}
```
