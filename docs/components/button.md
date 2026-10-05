# Button

Button is the primary command component for actions and form submission.

## Source Copy

```bash
dxui add button
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["button"] }
```

## API Surface

- `Button`
- `ButtonVariant`
- `ButtonSize`
- `button_class`

`ButtonVariant::Link` draws foreground-colored text with a primary underline
on hover, so link buttons stay readable in theme presets with a light primary
color ([RFC 0057](../rfcs/0057-theme-presets.md)).

## Events

```rust
rsx! {
  Button { onclick: move |_| save(), "Save" }
  Button { r#type: "button", "aria-label": "Close", onclick: move |_| close(), "×" }
}
```

A click, Enter, or Space calls `onclick` with the mouse event. Other
attributes, such as `type`, `name`, and `aria-label`, are passed to the
button. Inside a form the button keeps the native `submit` type; pass
`r#type: "button"` for a button that should not submit.

## Accessibility Notes

Use `disabled` for unavailable actions. Icon-only buttons should provide an
accessible label, such as `aria-label`, through the consuming app.
