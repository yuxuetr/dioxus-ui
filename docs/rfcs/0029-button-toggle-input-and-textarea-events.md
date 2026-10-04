# RFC 0029: Button, Toggle, Input, And Textarea Events

- Status: Accepted
- Created: 2026-10-04

## Summary

Give `Button` an `onclick` callback, `Toggle` an `on_pressed_change`
callback, and `Input` and `Textarea` an `on_value_change` callback. All four
pass through global and element attributes such as `id`, `name`, `type`, and
`aria-*`.

## Current State

As of M153:

- `Button`, `Toggle`, `Input`, and `Textarea` render their element from a few
  props and expose no callback. A Button cannot run an action, a Toggle cannot
  change, and typing in an Input never reaches the app.
- None of them accepts `id`, `name`, `type`, or `aria-*`. A `Label` cannot
  name an `Input`, an icon-only Button cannot get an `aria-label`, and an
  Input cannot be `type="email"`.
- The demos style raw elements with `button_class` and similar helpers
  instead of using the components.
- RFC 0028 added `on_checked_change` and attribute spreading to Switch and
  Checkbox. A probe confirmed an `onclick` prop works next to
  `#[props(extends = GlobalAttributes, extends = button)]`, and that `r#type`
  and `name` pass through.

## Decision

### API

```rust
let mut email = use_signal(String::new);
let mut bold = use_signal(|| false);

rsx! {
  Label { r#for: "email", "Email" }
  Input {
    id: "email",
    r#type: "email",
    value: email(),
    on_value_change: move |value| email.set(value),
  }
  Toggle {
    "aria-label": "Bold",
    pressed: bold(),
    on_pressed_change: move |pressed| bold.set(pressed),
    "B"
  }
  Button { onclick: move |_| save(), "Save" }
}
```

| Component | Callback | Fired by |
| --- | --- | --- |
| Button | `onclick: Option<EventHandler<MouseEvent>>` | click, Enter, Space |
| Toggle | `on_pressed_change: Option<EventHandler<bool>>` with `!pressed` | click, Enter, Space |
| Input | `on_value_change: Option<EventHandler<String>>` | `input` events |
| Textarea | `on_value_change: Option<EventHandler<String>>` | `input` events |

- Button keeps the native `onclick` name and passes the event, so the app can
  read modifier keys or stop propagation.
- Toggle follows Switch: it sends the requested state and stays controlled.
- Input and Textarea send the new text on every `input` event and stay
  controlled through `value`. `on_value_change` matches the name RadioGroup
  and Tabs already use.
- `attributes: Vec<Attribute>` extends `GlobalAttributes` and the rendered
  element, and is spread after the explicit attributes. Explicit props such as
  `class`, `disabled`, and `value` keep their meaning.
- A disabled control fires no event.
- Button keeps the native `type`, which is `submit` inside a form. Apps pass
  `r#type: "button"` when a Button inside a form should not submit it.

## Scope

In scope:

- the callbacks and attribute spreading in the crate and the templates
- the Button, Toggle, Input, and Textarea docs pages
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Key, focus, and blur callbacks | Spreading covers attributes, not listeners, and each listener is another prop; apps can wrap the control in an element that listens | A consumer needs a listener on the control itself |
| A Button `type` default of `button` | Changing the native default would surprise apps that rely on submit | A consumer reports an accidental submit |
| Uncontrolled inputs | Every other component in this crate is controlled | A consumer needs an input without app state |
| Desktop and Mobile self-test scenarios | The components use plain events with no script | They gain script behavior |

## Verification

- The CLI generated fixture smoke keeps the templates compiling.
- The Web preview renders the four components, and
  `npm run verify:runtime-interactions` asserts:
  - a Button click and Enter call `onclick`;
  - Toggle flips `aria-pressed` by click and Space;
  - typing in a labelled Input and Textarea reaches app state;
  - passed attributes such as `type`, `name`, and `aria-label` render.
- Reverse checks: removing a callback, not spreading the attributes, or
  sending the current Toggle state instead of the requested one each make the
  verifier fail.
