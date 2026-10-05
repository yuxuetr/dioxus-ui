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
- `SelectDismissBehavior`, `SelectSide`, `SelectAlign`
- `select_trigger_class`
- `select_item_class`

## Behavior

`open` and the selected value stay controlled by the app. Give the trigger an
`id` and pass it as the content's `anchor_id`:

```rust
let mut open = use_signal(|| false);
let mut value = use_signal(|| "system".to_string());

rsx! {
  SelectTrigger {
    id: "theme-trigger",
    open: open(),
    on_open_change: move |next| open.set(next),
    SelectValue { "{value}" }
  }
  SelectContent {
    open: open(),
    anchor_id: "theme-trigger",
    on_open_change: move |next| open.set(next),
    on_value_change: move |next| value.set(next),
    for theme in ["light", "dark", "system"] {
      SelectItem { key: "{theme}", value: theme, selected: value() == theme, "{theme}" }
    }
  }
}
```

- Clicking the trigger requests `!open`. ArrowDown or ArrowUp on a closed
  trigger requests open.
- With `anchor_id`, content is placed on `side` (default `Bottom`) with `align`
  (default `Start`) and `side_offset` (default `4`), flipping and shifting like
  Popover. The list is at least as wide as its trigger, and at least 8rem (see
  [RFC 0046](../rfcs/0046-listbox-width-follows-trigger.md)).
- Focus stays on the trigger. The selected option, or the first enabled one,
  is highlighted with `data-highlighted` and referenced by the trigger's
  `aria-activedescendant`. ArrowDown and ArrowUp move between enabled options
  without wrapping, Home and End jump to the ends, and typing a prefix
  highlights the next matching option.
- Enter, Space, or a click calls `on_value_change(value)` with the option's
  `value`, then `on_open_change(false)`. Disabled options are skipped and
  cannot be chosen.
- Escape and outside interactions request close per `dismiss` (default
  `DismissBehavior::popover_default()`).
- Without `anchor_id`, content renders in place with no keyboard handling.

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

The trigger uses select-only combobox semantics with `aria-haspopup="listbox"`
and `aria-activedescendant`; content uses listbox and option semantics. Apps
provide a visible or programmatic label for the trigger.

With the trigger's `id` as the content's `anchor_id`, the listbox renders
`id="{anchor_id}-content"` and takes the trigger's name through
`aria-labelledby`, and the trigger points `aria-controls` at it; a passed
`aria-controls` replaces the derived one. Disabled items set `aria-disabled`
(see [RFC 0055](../rfcs/0055-open-state-accessibility-audit.md)).

`SelectTrigger` passes through attributes such as `aria-labelledby` and
`aria-describedby` for a field description or error (see
[RFC 0038](../rfcs/0038-form-control-naming.md)).
