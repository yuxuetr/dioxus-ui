# File Input

File Input is a styled native file picker for uploads and imports, with a
drop area for files dragged onto the page.

## Source Copy

```bash
dxui add file-input
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["file-input"] }
```

## API Surface

- `FileInput`
- `FileDropzone`
- `file_input_class`
- `file_dropzone_class`

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

## Drop Area

`FileDropzone` takes files dragged onto it and passes them to `on_files` as
`Vec<FileData>` (`dioxus::html::FileData`: `name()`, `size()`,
`content_type()`, and the bytes):

```rust
rsx! {
  FileDropzone {
    "aria-label": "Upload area",
    on_files: move |files: Vec<FileData>| {
      for file in files {
        println!("dropped {}", file.name());
      }
    },
    "Drop files here"
    FileInput { class: "mt-3", multiple: true, onchange: move |event: FormEvent| upload(event.files()) }
  }
}
```

- It sets `data-dragging="true"` while files are over it, which the base
  classes show with a primary border and background; it counts entries and
  exits, so moving across its children does not clear it.
- A drop with no files, or onto a `disabled` area, calls nothing.
- `class` merges over the dashed border, padding, and centered text.

## Accessibility Notes

The native file input supplies the button, the chosen file names, and the
keyboard behavior. Name it with a `Label` (see
[RFC 0060](../rfcs/0060-input-components.md)).

Dropping needs a pointer, so put a `FileInput` or another picker inside or
beside a `FileDropzone`. Name the area with `aria-label` and, when it
stands alone, `role: "region"`.
