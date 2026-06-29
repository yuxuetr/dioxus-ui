# Direction

Direction provides a small scoped wrapper for native left-to-right and
right-to-left text direction.

## Source Copy

```bash
dxui add direction
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["direction"] }
```

## API Surface

- `Direction`
- `TextDirection`
- `direction_class`

## Accessibility Notes

Direction sets the native `dir` attribute and a matching `data-direction`
attribute. Apps still own locale detection, document-level direction policy,
text shaping, and mixed-direction content decisions. Nested direction wrappers
are acceptable when content requires them.
