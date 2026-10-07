# Combobox

Combobox provides searchable selection parts. It combines combobox
trigger/input semantics, listbox content, and popover primitive defaults.

## Source Copy

```bash
dxui add combobox
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["combobox"] }
```

## API Surface

- `Combobox`
- `ComboboxTrigger`
- `ComboboxInput`
- `ComboboxContent`
- `ComboboxList`
- `ComboboxEmpty`
- `ComboboxStatus`
- `ComboboxGroup`
- `ComboboxValue`
- `ComboboxItem`
- `ComboboxPrimitiveConfig`
- `ComboboxDismissBehavior`, `ComboboxSide`, `ComboboxAlign`
- `combobox_trigger_class`
- `combobox_input_class`
- `combobox_item_class`
- `combobox_active_descendant_state` (crate only; the source-copy template
  omits it)

## Behavior

`Combobox` owns the chosen value, or values with `multiple`, and whether the
list is open, as `Select` does, and its parts read it, so they must sit
inside it (see [RFC 0077](../rfcs/0077-component-owned-state.md)). The typed
text stays with the app, which filters the items it renders:

```rust
let fruits = ["Apple", "Banana", "Blueberry", "Cherry"];
let mut query = use_signal(String::new);

rsx! {
  Combobox {
    id: "fruit-input",
    on_value_change: move |value| query.set(value),
    ComboboxInput {
      value: query(),
      placeholder: "Search fruit",
      oninput: move |event: FormEvent| query.set(event.value()),
    }
    ComboboxContent {
      ComboboxList {
        for fruit in fruits.iter().filter(|fruit| fruit.to_lowercase().contains(&query().to_lowercase())) {
          ComboboxItem { key: "{fruit}", value: *fruit, "{fruit}" }
        }
      }
    }
  }
}
```

Pass `value` (`values`) or `open` with their change callbacks to control
them, or `default_value` (`default_values`) and `default_open` to start them;
`on_value_change` hears every choice in both modes.

- A combobox has one combobox element: its `ComboboxInput`, or without one a
  `ComboboxTrigger`. The root's `id` names it, for a `Label`, and the list
  anchors to it; without `id` the ids are generated. Use one or the other: a
  trigger next to an input would be a second combobox for the same list.
- Typing opens the list and reaches the app through `oninput`. ArrowDown on
  a closed input opens it, and a click on the trigger toggles it.
- Content is placed on `side` (default `Bottom`) with `align` (default
  `Start`) and `side_offset` (default `4`), flipping and shifting like
  Popover. The list is at least as wide as its input, and at least 8rem (see
  [RFC 0046](../rfcs/0046-listbox-width-follows-trigger.md)).
- Focus stays in the input. No option is highlighted until ArrowDown or
  ArrowUp; the highlighted option gets `data-highlighted` and the input's
  `aria-activedescendant`. Arrows skip disabled options without wrapping, and
  Home, End, Space, and printable keys stay with the input.
- When filtering removes the highlighted option, the highlight clears.
- Enter or a click chooses the option, which becomes the value, and closes
  the list.
- Escape and outside interactions close it per `dismiss` (default
  `DismissBehavior::popover_default()`).

To announce how many options match, render `ComboboxStatus` next to the
input, outside `ComboboxContent`, which is hidden while closed. Keep it
mounted and change only its text; an empty text says nothing:

```rust
let matches = fruits.iter().filter(|fruit| fruit.to_lowercase().contains(&query().to_lowercase())).count();

rsx! {
  ComboboxInput { /* ... */ }
  ComboboxStatus {
    // `open` is the app's signal, passed to the root as `open`.
    if !open() { "" }
    else if matches == 0 { "No results" }
    else if matches == 1 { "1 result" }
    else { "{matches} results" }
  }
  ComboboxContent { /* ... */ }
}
```

`ComboboxStatus` renders a visually hidden `role="status"` region with
`aria-live="polite"` and `aria-atomic="true"`. Rendering it conditionally
recreates the element, and its first text is then not announced.

Only the Web renderer is covered by `npm run verify:runtime-interactions`.

### Multiple selection

`multiple` on `Combobox` makes a choice toggle the value in `values`, keeps
the list open, and sets `aria-multiselectable="true"` on `ComboboxList`, as
with Select; the app decides whether to clear the query. Selected options
show a check mark (see [RFC 0062](../rfcs/0062-multi-select.md)).

## Accessibility Notes

Trigger and input use combobox semantics, list uses listbox semantics, and items
use option semantics. The input keeps focus and references the highlighted
option through `aria-activedescendant`. `ComboboxStatus` announces the text
the app gives it; the wording and when to speak stay with the app (see
[RFC 0027](../rfcs/0027-combobox-and-command-result-announcements.md)). Async loading is owned by the consuming app.

`ComboboxList` renders `id="{id}-list"` and takes the combobox element's
name through `aria-labelledby`, and `ComboboxInput` points `aria-controls` at
it, as does an open `ComboboxTrigger`; a passed `aria-controls` replaces the
derived one (see [RFC 0055](../rfcs/0055-open-state-accessibility-audit.md)).

`ComboboxInput` passes through attributes such as `name`, `aria-labelledby`,
and `aria-describedby` (see
[RFC 0038](../rfcs/0038-form-control-naming.md)).
`ComboboxTrigger` toggles the list, calls its `onclick`, and passes
through attributes such as `aria-label` and `aria-controls`; its `role`,
`aria-expanded`, and `aria-invalid` come from its props (see
[RFC 0053](../rfcs/0053-interactive-part-callbacks.md)).
