# Tabs

Tabs provides controlled styled root, list, trigger, and content parts.

## Source Copy

```bash
dxui add tabs
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["tabs"] }
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

`Tabs` owns which tab is selected and links its parts (see
[RFC 0077](../rfcs/0077-component-owned-state.md)). Start it with
`default_value`, or control it with `value`; `on_value_change` hears every
change either way:

```rust
rsx! {
  Tabs { default_value: "account",
    TabsList {
      TabsTrigger { value: "account", "Account" }
      TabsTrigger { value: "billing", "Billing" }
    }
    TabsContent { value: "account", "Account settings" }
    TabsContent { value: "billing", "Billing settings" }
  }
}
```

Control it when the app changes the tab itself, such as on a failed save:

```rust
let mut tab = use_signal(|| "account".to_string());

rsx! {
  Tabs { value: tab(), on_value_change: move |next| tab.set(next),
    // the same parts
  }
}
```

- The triggers form one Tab stop on the selected trigger. Tab from it moves
  into the visible panel, which has `tabindex="0"`.
- Left and Right move focus between enabled triggers and wrap; Home and End
  jump to the first and last. Moving focus selects the focused trigger
  (automatic activation).
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
- A click, Enter, or Space on a trigger selects it.
- `Tabs` links each trigger to its panel with `aria-controls` and
  `aria-labelledby`. A part outside a `Tabs` renders nothing, and Dioxus logs
  that it must be inside one.

The Web renderer is covered by `npm run verify:runtime-interactions`.

## Accessibility Notes

The list uses `role="tablist"` with `aria-orientation` matching
`orientation`, triggers use `role="tab"` with `aria-selected`, and panels use
`role="tabpanel"`.
Activation and orientation follow
[RFC 0026](../rfcs/0026-tabs-manual-activation-and-vertical-orientation.md).

`TabsList` passes through attributes, so name the tab list with `aria-label` or
`aria-labelledby` (see [RFC 0040](../rfcs/0040-composite-widget-names.md)).
