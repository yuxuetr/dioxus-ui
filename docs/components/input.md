# Input

Input is a styled single-line text field with invalid and density-aware class
support.

## Source Copy

```bash
dxui add input
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["input"] }
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

Pair inputs with `Label` and expose validation state with `aria-invalid` when
the field is invalid.
