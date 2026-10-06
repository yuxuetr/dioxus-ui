# Collapsible

Collapsible provides controlled disclosure parts for content that can be shown
or hidden by the consuming app.

## Source Copy

```bash
dxui add collapsible
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["collapsible"] }
```

## API Surface

- `Collapsible`
- `CollapsibleTrigger`
- `CollapsibleContent`
- `collapsible_class`
- `collapsible_trigger_class`
- `collapsible_content_class`

## Open Events

```rust
let mut open = use_signal(|| false);

rsx! {
  Collapsible { open: open(),
    CollapsibleTrigger {
      open: open(),
      controls: "details",
      on_open_change: move |next| open.set(next),
      "Details"
    }
    CollapsibleContent { open: open(), id: "details", "More information" }
  }
}
```

`CollapsibleContent` renders nothing while closed. Set `force_mount: true`
(default `false`) to keep it in the DOM, hidden, for example so the trigger's
`aria-controls` always points at an element.

A click, Enter, or Space on `CollapsibleTrigger` calls `on_open_change` with
the requested state, `!open`. Pass the value back as `open` to every part. A
disabled trigger does not call it. All three parts pass other attributes, such
as `aria-label` and `data-*`, to their element.

## Accessibility Notes

`CollapsibleTrigger` renders a native button with `aria-expanded`. Pass matching
`controls` and content `id` values when the app needs explicit trigger/content
association. Open state, animation timing, and measured-height transitions stay
app-owned.
