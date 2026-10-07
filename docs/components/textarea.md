# Textarea

Textarea is a styled multi-line text field with invalid and density-aware class
support.

## Source Copy

```bash
dxui add textarea
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["textarea"] }
```

## API Surface

- `Textarea`
- `textarea_class`

## Change Events

```rust
let mut notes = use_signal(String::new);

rsx! {
  Label { r#for: "notes", "Notes" }
  Textarea {
    id: "notes",
    value: notes(),
    on_value_change: move |value| notes.set(value),
  }
}
```

`Textarea` is controlled. Each `input` event calls `on_value_change` with the new
text, and the app passes it back as `value`. Other attributes, such as
`id`, `name`, `rows`, and `aria-describedby`, are passed to the textarea.

## Accessibility Notes

Pair textareas with `Label` and set `invalid` when the field is invalid; it
renders `aria-invalid` and the destructive border.
