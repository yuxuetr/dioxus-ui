# File Input

File Input is a styled native file picker for uploads and imports.

## Source Copy

```bash
dxui add file-input
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["file-input"] }
```

## API Surface

- `FileInput`
- `file_input_class`

```rust
rsx! {
  Label { r#for: "avatar", "Avatar" }
  FileInput {
    id: "avatar",
    accept: "image/*",
    onchange: move |event: FormEvent| {
      for file in event.files() {
        println!("chose {}", file.name());
      }
    },
  }
}
```

The input binds no `value`, which a file input cannot take. `onchange`
passes the form event; read the chosen files with `event.files()`.
`accept`, `multiple`, and other attributes go to the input. `invalid` sets
the destructive border and `aria-invalid`. Upload transport stays app-owned,
as with Attachment.

## Accessibility Notes

The native file input supplies the button, the chosen file names, and the
keyboard behavior. Name it with a `Label` (see
[RFC 0060](../rfcs/0060-input-components.md)).
