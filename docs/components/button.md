# Button

Button is the primary command component for actions and form submission.

## Source Copy

```bash
dxui add button
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["button"] }
```

## API Surface

- `Button`
- `ButtonVariant`
- `ButtonSize`
- `button_class`
- `DensityProvider`, `use_density`, `UiDensity`
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

## Density

`Button` takes its density from the nearest `DensityProvider`
([RFC 0078](../rfcs/0078-touch-density.md)), `Comfortable` without one. Under
`Touch` it is at least 48 CSS pixels high and 44 wide, a touch target; at
`Comfortable` the size alone sets the height (`Sm` is 32 pixels):

```rust
rsx! {
  DensityProvider { density: UiDensity::Touch,
    Button { "Save" }
  }
}
```

An element styled as a button, such as an overlay trigger, passes the same
density to `button_class(variant, size, use_density(), "")`. Every
interactive component follows the provider the same way.

## Accessibility Notes

Use `disabled` for unavailable actions. Icon-only buttons should provide an
accessible label, such as `aria-label`, through the consuming app.
