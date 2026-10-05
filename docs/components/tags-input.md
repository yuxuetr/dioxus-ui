# Tags Input

Tags Input collects a list of short values, such as topics or email
recipients, shown as removable chips.

## Source Copy

```bash
dxui add tags-input
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["tags-input"] }
```

## API Surface

- `TagsInput`
- `tags_input_class`
- `tags_input_add`
- `tags_input_commit`
- `tags_input_remove`

```rust
let mut topics = use_signal(Vec::<String>::new);

rsx! {
  TagsInput {
    "aria-label": "Topics",
    placeholder: "Add a topic",
    tags: topics(),
    on_tags_change: move |tags| topics.set(tags),
  }
}
```

Enter or a comma adds the trimmed draft unless it is empty or already
present; pasted text with commas adds every complete segment. Backspace in an
empty input removes the last tag, and each chip has a remove button. Changes
call `on_tags_change` with the new list. `remove_label` prefixes the remove
buttons' names, "Remove" by default. Other attributes go to the text input.

## Accessibility Notes

The tags are a `ul`, so assistive technology hears how many there are, and
each remove button is named "Remove {tag}". Name the text input with a
`Label` or `aria-label`. Enter adds a tag instead of submitting a form (see
[RFC 0060](../rfcs/0060-input-components.md)).
