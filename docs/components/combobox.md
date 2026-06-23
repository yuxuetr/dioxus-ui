# Combobox

Combobox provides controlled searchable selection parts. It combines combobox
trigger/input semantics, listbox content, and popover primitive defaults.

## Source Copy

```bash
dxui add combobox
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["combobox"] }
```

## API Surface

- `ComboboxTrigger`
- `ComboboxInput`
- `ComboboxContent`
- `ComboboxList`
- `ComboboxEmpty`
- `ComboboxGroup`
- `ComboboxValue`
- `ComboboxItem`
- `ComboboxPrimitiveConfig`
- `combobox_trigger_class`
- `combobox_input_class`
- `combobox_item_class`
- `combobox_active_descendant_state`

## Accessibility Notes

Trigger and input use combobox semantics, list uses listbox semantics, and items
use option semantics. Runtime filtering, keyboard navigation, and async loading
are owned by the consuming app in this phase.
