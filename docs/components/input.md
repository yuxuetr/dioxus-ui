# Input

Input is a styled single-line text field with an invalid state.

## Source Copy

```bash
dxui add input
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["input"] }
```

## API Surface

- `Input`
- `input_class`

## Change Events

```rust
let mut notes = use_signal(String::new);

rsx! {
  Label { r#for: "notes", "Notes" }
  Input {
    id: "notes",
    value: notes(),
    on_value_change: move |value| notes.set(value),
  }
}
```

`Input` is controlled. Each `input` event calls `on_value_change` with the new
text, and the app passes it back as `value`. Other attributes, such as
`id`, `name`, `type`, `autocomplete`, and `aria-describedby`, are passed to the input.

## Accessibility Notes

Pair inputs with `Label` and set `invalid` when the field is invalid; it renders
`aria-invalid` and the destructive border.
