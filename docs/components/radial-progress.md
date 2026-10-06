# Radial Progress

Radial Progress shows a value from 0 to 100 as a ring, with the percentage
or a custom label in the middle.

## Source Copy

```bash
dxui add radial-progress
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["radial-progress"] }
```

## API Surface

- `RadialProgress`
- `RadialProgressSize`
- `radial_progress_class`
- `radial_progress_value`
- `radial_progress_geometry`

```rust
rsx! {
  RadialProgress { value: 72.0, "aria-label": "Storage used" }
  RadialProgress { value: 3.0 / 5.0 * 100.0, "aria-label": "Tasks done", "3/5" }
}
```

`value` is clamped to 0 through 100. Children replace the centered
percentage. `RadialProgressSize` is `Sm`, `Md` (the default), or `Lg`. Color
the ring with a class such as `[&_circle:last-of-type]:stroke-success`.

## Accessibility Notes

The component has `role="progressbar"` with `aria-valuemin="0"`,
`aria-valuemax="100"`, and the rounded `aria-valuenow`; the ring is
`aria-hidden`. Give it a name with `aria-label` or `aria-labelledby` (see
[RFC 0059](../rfcs/0059-display-components.md)).
