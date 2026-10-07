# Skeleton

Skeleton provides an animated placeholder for loading UI.

## Source Copy

```bash
dxui add skeleton
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["skeleton"] }
```

## API Surface

- `Skeleton`
- `skeleton_class`

## Accessibility Notes

Skeletons are hidden from assistive technology by default. Keep loading state
announcements in the surrounding application flow when they are needed.
