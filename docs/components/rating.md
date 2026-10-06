# Rating

Rating lets the user pick a score from one to five stars, or to another
maximum.

## Source Copy

```bash
dxui add rating
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["rating"] }
```

## API Surface

- `Rating`
- `rating_class`

```rust
let mut rating = use_signal(|| 0_u8);

rsx! {
  Rating {
    "aria-label": "Product rating",
    value: rating(),
    on_value_change: move |value| rating.set(value),
  }
}
```

`value` runs from 0 (no rating) to `max` (default 5); the stars up to it are
filled with the warning color. A change calls `on_value_change` with the
chosen star. `name` sets the radio group's form name; without it each Rating
gets a unique one. `disabled` disables every star.

## Accessibility Notes

Rating is a `role="radiogroup"` of native radio inputs, so Tab reaches it,
the arrow keys move between stars, and the value submits with a form. Each
star is named "{n} of {max}"; name the group with `aria-label` or
`aria-labelledby`. The focused star shows a ring around it (see
[RFC 0060](../rfcs/0060-input-components.md)).
