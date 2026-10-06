# Separator

Separator divides content visually and can optionally expose separator
semantics.

## Source Copy

```bash
dxui add separator
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["separator"] }
```

## API Surface

- `Separator`
- `SeparatorOrientation`
- `separator_class`

## Accessibility Notes

Use decorative separators for purely visual division. Use semantic separators
when the divider conveys structure to assistive technology.
