# RFC 0028: Switch And Checkbox Change Events

- Status: Accepted
- Created: 2026-10-04

## Summary

Give `Switch` and `Checkbox` an `on_checked_change` callback and let them pass
through global and element attributes such as `id`, `name`, and `aria-*`.
`Switch` also renders `data-state`.

## Current State

As of M152:

- `Switch` renders `button role="switch"` with `aria-checked` from its
  `checked` prop. It has no click handler and no callback, so clicking it does
  nothing.
- `Checkbox` renders a native checkbox from its `checked` prop. The browser
  toggles the box on click, but the app is never told, so its state and the
  box drift apart.
- Neither component accepts `id`, `name`, or `aria-*`. `Label { for: ... }`
  cannot point at them, and a Switch has no way to get an accessible name.
- `docs/components/accessibility.md` lists Switch as Planned with "Needs
  explicit switch semantics before stability".
- Radix Switch and Checkbox take `checked` and `onCheckedChange`, pass other
  props to the rendered element, and render `data-state`.
- Dioxus 0.7.9 supports `#[props(extends = GlobalAttributes, extends = button)]`
  with `..attributes` in `rsx!`. A probe confirmed it renders `id`, `name`,
  and `aria-label` next to explicit `class` and `disabled` props.

## Decision

### API

```rust
let mut wifi = use_signal(|| false);

rsx! {
  Label { r#for: "wifi", "Wi-Fi" }
  Switch {
    id: "wifi",
    checked: wifi(),
    on_checked_change: move |checked| wifi.set(checked),
  }
}
```

- `on_checked_change: Option<EventHandler<bool>>` receives the requested
  state, which is `!checked`. Both components stay controlled: the app sets
  `checked` from the value it receives.
- `Switch` calls it on click. A native button turns Space, Enter, and a click
  on its `Label` into a click, so no key handler is needed.
- `Checkbox` calls it from `onchange`, which a click, Space, and a label click
  fire.
- A disabled control fires neither event.
- `attributes: Vec<Attribute>` extends `GlobalAttributes` and `button` for
  Switch and `input` for Checkbox, and is spread after the explicit
  attributes.
- `Switch` renders `data-state="checked"` or `"unchecked"`, so apps can style
  it with `data-[state=checked]:` variants. Checkbox already has `:checked`.

### Why The Requested State

Sending `!checked` instead of reading the DOM keeps the value tied to the
prop. If an app ignores the callback, the next click asks for the same state
again instead of following a box the browser toggled.

## Scope

In scope:

- the callback, attribute spreading, and `data-state` in the crate and the
  templates
- the Switch and Checkbox docs pages
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| An indeterminate checkbox | It needs a tri-state type and the DOM `indeterminate` property, which has no attribute | A consumer needs a "select all" checkbox |
| A hidden form input for Switch | A button is not submitted with a form; apps can render their own input from the same state | A consumer submits a Switch in a native form |
| The same change for Button, Toggle, Input, and Textarea | Each has its own event shape; this milestone covers the two components the accessibility table blocks | The next milestone picks them up |
| Keeping a native checkbox in step when the app ignores the callback | The browser toggles the box before Dioxus sees the event | A consumer reports the drift |
| Desktop and Mobile self-test scenarios | The components use plain click and change events with no script | They gain script behavior |

## Verification

- Unit tests cover the `data-state` value.
- The CLI generated fixture smoke keeps the templates compiling.
- The Web preview renders a labelled Switch and Checkbox, and
  `npm run verify:runtime-interactions` asserts:
  - the accessible name comes from `Label`;
  - a click, Space, and a label click toggle `aria-checked` or `checked`;
  - `data-state` follows the Switch state;
  - a disabled control does not change.
- Reverse checks: removing the callback, not spreading the attributes, or
  sending the current state instead of the requested one each make the
  verifier fail.
