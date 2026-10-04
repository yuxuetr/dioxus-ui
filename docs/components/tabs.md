# Tabs

Tabs provides controlled styled root, list, trigger, and content parts.

## Source Copy

```bash
dxui add tabs
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["tabs"] }
```

## API Surface

- `Tabs`
- `TabsList`
- `TabsTrigger`
- `TabsContent`
- `TabsActivation`
- `TabsOrientation`
- `tabs_class`
- `tabs_list_class`
- `tabs_trigger_class`
- `tabs_content_class`

## Behavior

The selected tab stays controlled by the app. Keep its value, pass `active` to
each trigger and panel, and handle `Tabs` `on_value_change`:

```rust
let mut tab = use_signal(|| "account".to_string());

rsx! {
  Tabs {
    on_value_change: move |value: String| tab.set(value),
    TabsList {
      TabsTrigger { value: "account", active: tab() == "account", "Account" }
      TabsTrigger { value: "billing", active: tab() == "billing", "Billing" }
    }
    TabsContent { value: "account", active: tab() == "account", "Account settings" }
    TabsContent { value: "billing", active: tab() == "billing", "Billing settings" }
  }
}
```

- The triggers form one Tab stop on the selected trigger. Tab from it moves
  into the visible panel, which has `tabindex="0"`.
- Left and Right move focus between enabled triggers and wrap; Home and End
  jump to the first and last. Moving focus calls `on_value_change` with the
  focused trigger's value (automatic activation).
- With `activation: TabsActivation::Manual`, keys only move focus, and Enter,
  Space, or a click selects.
- With `orientation: TabsOrientation::Vertical`, Up and Down move focus
  instead of Left and Right, the list renders `aria-orientation="vertical"`,
  and the root, list, and panels render `data-orientation="vertical"` so the
  list sits beside the panels.
- When focus leaves the list, the selected trigger becomes the Tab stop
  again, so Shift+Tab back into the list lands on it.
- In a right-to-left horizontal list, ArrowLeft moves to the next trigger and
  ArrowRight to the previous one.
- A click, Enter, or Space on a trigger calls `on_value_change` with its value.
- `Tabs` links each trigger to its panel with `aria-controls` and
  `aria-labelledby`. Without `Tabs`, the parts keep keyboard movement but
  report nothing and render no ids.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

The list uses `role="tablist"` with `aria-orientation` matching
`orientation`, triggers use `role="tab"` with `aria-selected`, and panels use
`role="tabpanel"`.
Activation and orientation follow
[RFC 0026](../rfcs/0026-tabs-manual-activation-and-vertical-orientation.md).
