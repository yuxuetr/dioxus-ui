# Checkbox

Checkbox provides a controlled boolean input with styled checked and unchecked
states.

## Source Copy

```bash
dxui add checkbox
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["checkbox"] }
```

## API Surface

- `Checkbox`
- `checkbox_class`

## Change Events

```rust
let mut terms = use_signal(|| false);

rsx! {
  Checkbox {
    id: "terms",
    name: "terms",
    checked: terms(),
    on_checked_change: move |checked| terms.set(checked),
  }
  Label { r#for: "terms", "Accept terms" }
}
```

`Checkbox` is controlled. A click, Space, or a click on its `Label` calls
`on_checked_change` with the requested state, `!checked`, and the app passes
it back as `checked`. A disabled checkbox does not call it.

Other attributes, such as `id`, `name`, `value`, and `aria-describedby`, are
passed to the input.

## Indeterminate State

```rust
let mut items = use_signal(|| [true, false]);
let all = items().iter().all(|item| *item);
let some = items().iter().any(|item| *item);

rsx! {
  Checkbox {
    "aria-label": "Select all",
    checked: all,
    indeterminate: some && !all,
    on_checked_change: move |checked| items.set([checked, checked]),
  }
}
```

`indeterminate` sets the input's native `indeterminate` property, which
browsers announce as a mixed checkbox, and renders
`data-state="indeterminate"`. A change while mixed requests `true`. A click
clears the property in the browser; when the app keeps the checkbox mixed,
the component sets it again after the next render. The property is set in the
browser, so server-rendered HTML exposes the checkbox as unchecked until it
hydrates (see [RFC 0041](../rfcs/0041-checkbox-indeterminate-state.md)).

The input draws its own box with `appearance-none`: the checked state shows a
tick and the mixed state a dash, both on the blue fill. The marks are keyed
off `data-state`, so server-rendered HTML already shows them (see
[RFC 0045](../rfcs/0045-drawn-checkbox.md)).

## Accessibility Notes

Pair checkboxes with a visible label or accessible name. Keep checked state in
application state so form and keyboard behavior remain predictable.
