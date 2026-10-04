# Command

Command provides command palette and searchable action list parts. Focus stays
in the input while arrow keys move the highlight, and Enter or a click chooses
an item. Filtering stays with the app.

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
- `CommandStatus`
- `CommandGroup`
- `CommandLabel`
- `CommandItem`
- `CommandSeparator`
- `CommandShortcut`
- `ActiveDescendantState`
- `command_class`
- `command_input_class`
- `command_item_class`
- `command_matches`
- `command_active_descendant_state`

## Behavior

The query and the chosen item stay with the app. Keep the query, filter the
items you render, and handle `Command` `on_select`:

```rust
let mut query = use_signal(String::new);
let items = [("calendar", "Calendar"), ("settings", "Settings")];

rsx! {
  Command {
    on_select: move |value: String| run(&value),
    CommandInput {
      value: query(),
      placeholder: "Type a command...",
      oninput: move |event: FormEvent| query.set(event.value()),
    }
    CommandList {
      for (value, label) in items.into_iter().filter(|(_, label)| command_matches(label, &query())) {
        CommandItem { key: "{value}", id: "command-{value}", value, "{label}" }
      }
    }
  }
}
```

- The selected option, or else the first enabled option, starts highlighted.
  The input's `aria-activedescendant` names it and the item has
  `data-highlighted`.
- ArrowDown and ArrowUp move to the next or previous enabled option and stop
  at the ends. Home and End highlight the first and last enabled option.
- Typing, including Space, goes to the input. When the query changes, the
  highlight goes back to the first enabled option of the re-rendered list.
- Enter or a click on an option calls `on_select` with its `value`, or its
  `id` without one. Focus stays in the input. Disabled items are skipped and
  cannot be chosen.
- Moving the pointer over an option highlights it.
- When nothing matches, render `CommandEmpty`; the input then has no
  `aria-activedescendant` and Enter does nothing.
- `command_matches(label, query)` is true when the trimmed query is empty or
  the label contains it, ignoring case.
- Inside `Command`, the input has an id and `aria-controls` naming the list.
- Without `Command`, the parts render state only and the app wires keys
  itself with `active_id`, `active`, and `selected`.

To announce how many items match, render `CommandStatus` every time and
change only its text; an empty text says nothing:

```rust
let matches = items.iter().filter(|(_, label)| command_matches(label, &query())).count();

rsx! {
  CommandStatus {
    if query().is_empty() { "" }
    else if matches == 0 { "No results" }
    else if matches == 1 { "1 result" }
    else { "{matches} results" }
  }
}
```

`CommandStatus` renders a visually hidden `role="status"` region with
`aria-live="polite"` and `aria-atomic="true"`. `CommandEmpty` is usually
rendered only when nothing matches, so it is not announced on its own.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

Input uses combobox semantics, list uses listbox semantics, and items use option
semantics. `CommandStatus` announces the text the app gives it (see
[RFC 0027](../rfcs/0027-combobox-and-command-result-announcements.md)). Fuzzy ranking, looping, and Ctrl key bindings are not
implemented (see [RFC 0024](../rfcs/0024-command-keyboard-and-filtering.md)).
