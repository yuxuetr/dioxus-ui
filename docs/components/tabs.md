# Tabs

Tabs provides controlled styled list, trigger, and content parts.

## Source Copy

```bash
dxui add tabs
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["tabs"] }
```

## API Surface

- `TabsList`
- `TabsTrigger`
- `TabsContent`
- `tabs_list_class`
- `tabs_trigger_class`
- `tabs_content_class`

## Accessibility Notes

Tabs should expose tablist, tab, and tabpanel roles once full ARIA wiring is
added. Keep active state controlled and make keyboard activation predictable.
