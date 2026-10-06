# Switch

Switch provides a controlled on/off control with styled track and thumb parts.

## Source Copy

```bash
dxui add switch
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["switch"] }
```

## API Surface

- `Switch`
- `switch_class`
- `switch_state`
- `switch_thumb_class`

## Change Events

```rust
let mut wifi = use_signal(|| false);

rsx! {
  Label { r#for: "wifi", "Wi-Fi" }
  Switch {
    id: "wifi",
    checked: wifi(),
    on_checked_change: move |checked| wifi.set(checked),
  }
}
```

`Switch` is controlled. A click, Space, Enter, or a click on its `Label`
calls `on_checked_change` with the requested state, `!checked`, and the app
passes it back as `checked`. A disabled switch does not call it.

Other attributes, such as `id`, `name`, `aria-label`, and
`aria-describedby`, are passed to the button. The button and its thumb render
`data-state="checked"` or `"unchecked"` for `data-[state=checked]:` variants.

## Accessibility Notes

Use switch semantics for settings that take effect immediately. Use checkbox
semantics instead when the value is submitted as part of a form.

Give every switch an accessible name, either a `Label` whose `for` matches the
switch `id` or an `aria-label`. A switch is a button, so it is not submitted
with a native form; render a hidden input from the same state if you need one.
