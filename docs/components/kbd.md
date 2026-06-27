# Kbd

Kbd provides a styled inline keyboard shortcut hint.

## Source Copy

```bash
dxui add kbd
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["kbd"] }
```

## API Surface

- `Kbd`
- `KbdSize`
- `kbd_class`

## Accessibility Notes

Kbd uses native `kbd` semantics. Apps own platform-specific shortcut wording
and should avoid using shortcut hints as the only way to discover an action.
