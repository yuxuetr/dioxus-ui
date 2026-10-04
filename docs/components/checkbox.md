# Checkbox

Checkbox provides a controlled boolean input with styled checked and unchecked
states.

## Source Copy

```bash
dxui add checkbox
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["checkbox"] }
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
passed to the input. An indeterminate state is not supported yet.

## Accessibility Notes

Pair checkboxes with a visible label or accessible name. Keep checked state in
application state so form and keyboard behavior remain predictable.
