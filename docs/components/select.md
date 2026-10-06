# Select

Select provides a root that owns the chosen value and the open state, with
styled trigger, value, content, group, label, item, and separator parts.

## Source Copy

```bash
dxui add select
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["select"] }
```

## API Surface

- `Select`
- `SelectTrigger`
- `SelectValue`
- `SelectContent`
- `SelectGroup`
- `SelectLabel`
- `SelectItem`
- `SelectSeparator`
- `SelectPrimitiveConfig`
- `SelectDismissBehavior`, `SelectSide`, `SelectAlign`
- `select_trigger_class`
- `select_item_class`

## Behavior

`Select` owns the chosen value and whether the list is open, and links its
parts (see [RFC 0077](../rfcs/0077-component-owned-state.md)). Start it with
`default_value`, or control it with `value`; `on_value_change` hears every
choice either way:

```rust
rsx! {
  Select { default_value: "system",
    SelectTrigger { SelectValue { placeholder: "Theme" } }
    SelectContent {
      for theme in ["light", "dark", "system"] {
        SelectItem { key: "{theme}", value: theme, "{theme}" }
      }
    }
  }
}
```

To show a label other than the value, control the value and put the label in
the trigger:

```rust
let mut fruit = use_signal(|| "banana".to_string());

rsx! {
  Select { id: "fruit", value: fruit(), on_value_change: move |next| fruit.set(next),
    Label { r#for: "fruit", "Fruit" }
    SelectTrigger { span { class: "truncate", "{label_of(&fruit())}" } }
    SelectContent {
      SelectItem { value: "apple", "Apple" }
      SelectItem { value: "banana", "Banana" }
    }
  }
}
```

- `open`, `default_open`, and `on_open_change` work the same way for the list.
- `id` names the trigger, for a `Label`; without it the ids are generated.
- Clicking the trigger toggles the list. ArrowDown or ArrowUp on a closed
  trigger opens it.
- The list is placed on `side` (default `Bottom`) with `align` (default
  `Start`) and `side_offset` (default `4`), flipping and shifting like
  Popover. The list is at least as wide as its trigger, and at least 8rem (see
  [RFC 0046](../rfcs/0046-listbox-width-follows-trigger.md)).
- Focus stays on the trigger. The selected option, or the first enabled one,
  is highlighted with `data-highlighted` and referenced by the trigger's
  `aria-activedescendant`. ArrowDown and ArrowUp move between enabled options
  without wrapping, Home and End jump to the ends, and typing a prefix
  highlights the next matching option.
- Enter, Space, or a click chooses the option and closes the list. Disabled
  options are skipped and cannot be chosen.
- Escape and outside interactions close the list per `dismiss` (default
  `DismissBehavior::popover_default()`).
- `SelectValue` shows the chosen value, or the chosen values joined with
  commas, or its `placeholder`, with `data-placeholder="true"` while empty.
- The selected option shows a check mark at the inline end; the highlighted
  one takes the accent background.
- A part outside a `Select` renders nothing, and Dioxus logs that it must be
  inside one.

### Multiple selection

With `multiple`, a choice toggles the value and the list stays open, and it
sets `aria-multiselectable="true"`. `values`, `default_values`, and
`on_values_change` take the chosen values:

```rust
rsx! {
  Select { multiple: true, default_values: vec!["rust".to_string()],
    SelectTrigger { SelectValue { placeholder: "Languages" } }
    SelectContent {
      SelectItem { value: "rust", "Rust" }
      SelectItem { value: "go", "Go" }
    }
  }
}
```

Escape and outside interactions still close it (see
[RFC 0062](../rfcs/0062-multi-select.md)).

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

The trigger uses select-only combobox semantics with `aria-haspopup="listbox"`
and `aria-activedescendant`; content uses listbox and option semantics. Apps
provide a visible or programmatic label for the trigger.

The listbox renders `id="{trigger id}-content"` and takes the trigger's name
through `aria-labelledby`, and the trigger points `aria-controls` at it; a
passed `aria-controls` replaces the derived one. Disabled items set `aria-disabled`
(see [RFC 0055](../rfcs/0055-open-state-accessibility-audit.md)).

`SelectTrigger` passes through attributes such as `aria-labelledby` and
`aria-describedby` for a field description or error (see
[RFC 0038](../rfcs/0038-form-control-naming.md)).
