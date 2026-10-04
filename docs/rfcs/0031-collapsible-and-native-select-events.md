# RFC 0031: Collapsible And Native Select Events

- Status: Accepted
- Created: 2026-10-04

## Summary

Give `CollapsibleTrigger` an `on_open_change` callback and `NativeSelect` an
`on_value_change` callback. Both components, and the other Collapsible parts,
pass through global and element attributes.

## Current State

As of M155:

- `CollapsibleTrigger` renders a button with `aria-expanded`, `aria-controls`,
  and `data-state`, but no click handler. The docs say click handlers stay
  app-owned, yet the component gives the app nowhere to attach one.
- `NativeSelect` renders a native `select`. The browser changes the shown
  option, but the app is never told.
- Neither accepts `id`, `name`, or `aria-*`. A `Label` cannot name a Native
  Select, and a form cannot submit it by name.
- Dialog and Popover already call their open callback `on_open_change`, and
  RadioGroup, Tabs, Input, and Textarea report values through
  `on_value_change`.

## Decision

### API

```rust
let mut open = use_signal(|| false);
let mut size = use_signal(|| "md".to_string());

rsx! {
  Collapsible { open: open(),
    CollapsibleTrigger {
      open: open(),
      controls: "details",
      on_open_change: move |next| open.set(next),
      "Details"
    }
    CollapsibleContent { open: open(), id: "details", "More information" }
  }
  Label { r#for: "size", "Size" }
  NativeSelect {
    id: "size",
    name: "size",
    on_value_change: move |value| size.set(value),
    NativeSelectOption { value: "sm", selected: size() == "sm", "Small" }
    NativeSelectOption { value: "md", selected: size() == "md", "Medium" }
  }
}
```

- `CollapsibleTrigger` calls `on_open_change` on click with the requested
  state, `!open`. A native button turns Enter and Space into a click. The
  trigger stays controlled, like Switch and Toggle.
- `NativeSelect` calls `on_value_change` from `onchange` with the chosen
  option's `value`. Options stay controlled through `selected`.
- `Collapsible`, `CollapsibleTrigger`, and `CollapsibleContent` extend
  `GlobalAttributes` and their element. `NativeSelect` extends
  `GlobalAttributes` and `select`. The spread comes after the explicit
  attributes, so props such as `class`, `disabled`, and `id` keep their
  meaning.
- A disabled trigger or select fires no event.

## Scope

In scope:

- the callbacks and attribute spreading in the crate and the templates
- the Collapsible and Native Select docs pages
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| A root-level open context that links trigger and content ids | Each part takes `open` today, and `controls` and `id` already link them; a context is a larger API change | A consumer reports mismatched `open` values between parts |
| A `value` prop on Native Select | `selected` on each option already controls the choice | A consumer finds `selected` awkward for many options |
| Multiple selection | It needs a list value and a different change payload | A consumer needs a multi-select |
| Desktop and Mobile self-test scenarios | The components use plain click and change events with no script | They gain script behavior |

## Verification

- The CLI generated fixture smoke keeps the templates compiling.
- The Web preview renders a Collapsible and a labelled Native Select, and
  `npm run verify:runtime-interactions` asserts:
  - a click, Enter, and Space toggle `aria-expanded` and the content;
  - choosing an option reaches app state;
  - passed attributes such as `name` render, and the `Label` names the
    select.
- Reverse checks: removing a callback, not spreading the attributes, or
  sending the current open state instead of the requested one each make the
  verifier fail.
