# Status

Status is a small colored dot for presence or health, such as online, away,
or an outage.

## Source Copy

```bash
dxui add status
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["status"] }
```

## API Surface

- `Status`
- `StatusVariant`
- `StatusSize`
- `status_class`

```rust
rsx! {
  Status { variant: StatusVariant::Success, label: "Online" }
  span { class: "flex items-center gap-2", Status { variant: StatusVariant::Warning }, "Degraded" }
}
```

`StatusVariant` is `Neutral` (the default), `Primary`, `Success`, `Warning`,
`Info`, or `Destructive`; `StatusSize` is `Sm`, `Md` (the default), or `Lg`.

## Accessibility Notes

With `label`, Status renders `role="img"` with that `aria-label`. Without a
label it is `aria-hidden`, for a dot beside text that already states the
status. Do not rely on the color alone: either label the dot or put the state
in nearby text (see [RFC 0059](../rfcs/0059-display-components.md)).
