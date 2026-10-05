# Direction

Direction provides a small scoped wrapper for native left-to-right and
right-to-left text direction.

## Source Copy

```bash
dxui add direction
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["direction"] }
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

Tabs, Radio Group, Toggle Group, Menubar, and Navigation Menu read the
computed direction when an arrow key is pressed. Inside a right-to-left
wrapper, ArrowLeft moves to the next item and ArrowRight to the previous one
(see [RFC 0025](../rfcs/0025-right-to-left-arrow-mirroring.md)).
