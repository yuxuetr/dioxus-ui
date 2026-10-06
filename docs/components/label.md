# Label

Label provides consistent text styling for form controls.

## Source Copy

```bash
dxui add label
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["label"] }
```

## API Surface

- `Label`
- `label_class`

## Accessibility Notes

Associate labels with their target control using `for` or equivalent Dioxus
attributes. Controls that are not labelable elements, such as `Slider`, take
`aria-labelledby` instead; give the label an `id`, which `Label` passes
through with other attributes. Avoid using label styling for unrelated helper
text.
