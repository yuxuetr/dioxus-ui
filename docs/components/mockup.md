# Mockup

Mockup frames content as a browser window, an application window, a
terminal, or a phone, for product and documentation pages.

## Source Copy

```bash
dxui add mockup
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["mockup"] }
```

## API Surface

- `MockupBrowser`
- `MockupWindow`
- `MockupCode`
- `MockupCodeLine`
- `MockupPhone`
- `mockup_frame_class`
- `mockup_code_line_class`

```rust
rsx! {
  MockupBrowser { url: "https://example.com", content_class: "p-8", "Hello" }
  MockupCode {
    MockupCodeLine { prefix: "$", "cargo install dioxus-shadcn-cli" }
    MockupCodeLine { prefix: "$", "dxui add mockup" }
    MockupCodeLine { highlight: true, "Added mockup" }
  }
  MockupPhone { div { class: "grid h-full place-items-center", "App" } }
}
```

## Behavior

See [RFC 0072](../rfcs/0072-menu-and-mockup.md).

- `MockupBrowser` shows three window dots and, with a `url`, an address
  field; `MockupWindow` shows the dots only. Content goes in a bordered area
  below, styled with `content_class`.
- `MockupCode` stacks `MockupCodeLine`s in a terminal-styled block that
  scrolls sideways when lines are long. A line's `prefix` is shown before it
  and excluded from text selection; `highlight` marks the line.
- `MockupPhone` is a rounded frame with a camera notch around a display area
  with a phone screen's proportions, styled with `display_class`.

## Accessibility Notes

The dots, notch, and line prefixes are `aria-hidden`, so a screen reader
reads only the content. The address field is text, not a link. When a mockup
shows an image of an app rather than live content, give the image its own
alternative text.
