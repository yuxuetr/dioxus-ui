# Select

Select provides primitive listbox configuration with styled trigger, value,
content, group, label, item, and separator parts.

## Source Copy

```bash
dxui add select
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["select"] }
```

## API Surface

- `SelectTrigger`
- `SelectValue`
- `SelectContent`
- `SelectGroup`
- `SelectLabel`
- `SelectItem`
- `SelectSeparator`
- `SelectPrimitiveConfig`
- `select_trigger_class`
- `select_item_class`

## Accessibility Notes

Select requires listbox semantics, active option tracking, keyboard navigation,
and a visible or programmatic label.
