# Input Group

Input Group provides grouped layout parts for addons, controls, and actions
around native inputs.

## Source Copy

```bash
dxui add input-group
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["input-group"] }
```

## API Surface

- `InputGroup`
- `InputGroupAddon`
- `InputGroupControl`
- `InputGroupAction`
- `InputGroupAddonPosition`
- `input_group_class`
- `input_group_addon_class`
- `input_group_control_class`
- `input_group_action_class`

## Accessibility Notes

Input Group preserves native input semantics by leaving the actual input element
app-owned. Pair grouped inputs with `Label`; use text or `aria-hidden` for
decorative addons as appropriate. `InputGroupAction` renders a native button
that takes `onclick` and passes through other attributes; icon-only actions
need an `aria-label` from the app.
