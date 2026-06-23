# Command

Command provides controlled command palette and searchable action list parts. It
exposes active descendant semantics but leaves filtering and keyboard event
wiring to the consuming app.

## Source Copy

```bash
dxui add command
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["command"] }
```

## API Surface

- `Command`
- `CommandInput`
- `CommandList`
- `CommandEmpty`
- `CommandGroup`
- `CommandLabel`
- `CommandItem`
- `CommandSeparator`
- `CommandShortcut`
- `ActiveDescendantState`
- `command_class`
- `command_input_class`
- `command_item_class`
- `command_active_descendant_state`

## Accessibility Notes

Input uses combobox semantics, list uses listbox semantics, and items use option
semantics. Runtime filtering, keyboard navigation, and command execution are
owned by the consuming app in this phase.
