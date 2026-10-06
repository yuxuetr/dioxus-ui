# Avatar

Avatar displays a user or entity image with a fallback part for initials or
short labels.

## Source Copy

```bash
dxui add avatar
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["avatar"] }
```

## API Surface

- `Avatar`
- `AvatarImage`
- `AvatarFallback`
- `avatar_class`
- `avatar_image_class`
- `avatar_fallback_class`

## Accessibility Notes

Use meaningful `alt` text when the avatar communicates identity. Use an empty
`alt` value when the surrounding text already names the person or object.
