# Collapsible

Collapsible provides controlled disclosure parts for content that can be shown
or hidden by the consuming app.

## Source Copy

```bash
dxui add collapsible
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["collapsible"] }
```

## API Surface

- `Collapsible`
- `CollapsibleTrigger`
- `CollapsibleContent`
- `collapsible_class`
- `collapsible_trigger_class`
- `collapsible_content_class`

## Accessibility Notes

`CollapsibleTrigger` renders a native button with `aria-expanded`. Pass matching
`controls` and content `id` values when the app needs explicit trigger/content
association. Open state, click handlers, animation timing, and measured-height
transitions stay app-owned.
