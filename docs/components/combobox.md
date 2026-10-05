# Combobox

Combobox provides controlled searchable selection parts. It combines combobox
trigger/input semantics, listbox content, and popover primitive defaults.

## Source Copy

```bash
dxui add combobox
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.1", default-features = false, features = ["combobox"] }
```

## API Surface

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

`open`, the typed text, and the chosen value stay controlled by the app, which
also filters the items it renders. Give the input an `id` and pass it as the
content's `anchor_id`:

```rust
let fruits = ["Apple", "Banana", "Blueberry", "Cherry"];
let mut open = use_signal(|| false);
let mut query = use_signal(String::new);

rsx! {
  ComboboxInput {
    id: "fruit-input",
    value: query(),
    open: open(),
    placeholder: "Search fruit",
    oninput: move |event: FormEvent| {
      query.set(event.value());
      open.set(true);
    },
    on_open_change: move |next| open.set(next),
  }
  ComboboxContent {
    open: open(),
    anchor_id: "fruit-input",
    on_open_change: move |next| open.set(next),
    on_value_change: move |value| query.set(value),
    ComboboxList {
      for fruit in fruits.iter().filter(|fruit| fruit.to_lowercase().contains(&query().to_lowercase())) {
        ComboboxItem { key: "{fruit}", value: *fruit, "{fruit}" }
      }
    }
  }
}
```

- `oninput` reports typed text. ArrowDown on a closed input requests open.
  `open` sets the input's `aria-expanded` and defaults to `true` for the
  existing input-inside-content usage.
- With `anchor_id`, content is placed on `side` (default `Bottom`) with `align`
  (default `Start`) and `side_offset` (default `4`), flipping and shifting like
  Popover. The list is at least as wide as its input, and at least 8rem (see
  [RFC 0046](../rfcs/0046-listbox-width-follows-trigger.md)).
- Focus stays in the input. No option is highlighted until ArrowDown or
  ArrowUp; the highlighted option gets `data-highlighted` and the input's
  `aria-activedescendant`. Arrows skip disabled options without wrapping, and
  Home, End, Space, and printable keys stay with the input.
- When filtering removes the highlighted option, the highlight clears.
- Enter or a click calls `on_value_change(value)` with the option's `value`,
  then `on_open_change(false)`.
- Escape and outside interactions request close per `dismiss` (default
  `DismissBehavior::popover_default()`).
- Without `anchor_id`, content renders in place with no keyboard handling.

To announce how many options match, render `ComboboxStatus` next to the
input, outside `ComboboxContent`, which is hidden while closed. Keep it
mounted and change only its text; an empty text says nothing:

```rust
let matches = fruits.iter().filter(|fruit| fruit.to_lowercase().contains(&query().to_lowercase())).count();

rsx! {
  ComboboxInput { /* ... */ }
  ComboboxStatus {
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

## Accessibility Notes

Trigger and input use combobox semantics, list uses listbox semantics, and items
use option semantics. The input keeps focus and references the highlighted
option through `aria-activedescendant`. `ComboboxStatus` announces the text
the app gives it; the wording and when to speak stay with the app (see
[RFC 0027](../rfcs/0027-combobox-and-command-result-announcements.md)). Async loading is owned by the consuming app.

With the input's `id` as the content's `anchor_id`, `ComboboxList` renders
`id="{anchor_id}-list"` and takes the input's name through `aria-labelledby`,
and `ComboboxInput` points `aria-controls` at it; a passed `aria-controls`
replaces the derived one (see [RFC 0055](../rfcs/0055-open-state-accessibility-audit.md)).

`ComboboxInput` passes through attributes such as `name`, `aria-labelledby`,
and `aria-describedby` (see
[RFC 0038](../rfcs/0038-form-control-naming.md)).
`ComboboxTrigger` takes `onclick`, where the app opens the list, and passes
through attributes such as `aria-label` and `aria-controls`; its `role`,
`aria-expanded`, and `aria-invalid` come from its props (see
[RFC 0053](../rfcs/0053-interactive-part-callbacks.md)).
