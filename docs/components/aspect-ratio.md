# Aspect Ratio

Aspect Ratio provides a fixed-ratio slot for media or custom content. It does
not own media loading, object-fit behavior, captions, or measurement.

## Source Copy

```bash
dxui add aspect-ratio
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["aspect-ratio"] }
```

## API Surface

- `AspectRatio`
- `aspect_ratio_class`
- `aspect_ratio_style`
- `aspect_ratio_value`

## Accessibility Notes

Aspect Ratio does not add semantics. Apps should provide accessible labels,
captions, and alternative text for the media rendered inside it.
