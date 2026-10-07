# Collapsible

Collapsible provides disclosure parts for content that can be shown or hidden.
The root owns whether it is open
([RFC 0077](../rfcs/0077-component-owned-state.md)).

## Source Copy

```bash
dxui add collapsible
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["collapsible"] }
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
rsx! {
  Collapsible {
    CollapsibleTrigger { "Details" }
    CollapsibleContent { "More information" }
  }
}
```

`Collapsible` starts closed, or open with `default_open: true`. Pass `open`
to control it, for example to show the state in the trigger label;
`on_open_change` hears every change the user makes either way. A click,
Enter, or Space on `CollapsibleTrigger` toggles the root. `disabled` on the
root or on the trigger disables the trigger.

`CollapsibleContent` renders nothing while closed. Set `force_mount: true`
(default `false`) to keep it in the DOM, hidden. All three parts pass other
attributes, such as `aria-label` and `data-*`, to their element. The trigger
and content must be inside `Collapsible`; otherwise they render nothing and
log which root they are missing.

## Accessibility Notes

`CollapsibleTrigger` renders a native button with `aria-expanded`, and with
`aria-controls` pointing at the content while it shows; the root generates
the ids. Animation timing and measured-height transitions stay app-owned.
