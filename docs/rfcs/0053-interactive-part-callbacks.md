# RFC 0053: Interactive Part Callbacks

- Status: Accepted
- Created: 2026-10-05

## Summary

Give the parts that render a native button, label, or link the hooks an app
needs to act on them. The action buttons gain `onclick`, `FieldLabel` gains
`r#for`, and all of them pass through global and element attributes.

## Current State

As of M180:

- `ButtonGroupItem`, `InputGroupAction`, `AttachmentAction`,
  `AttachmentTrigger`, `ComboboxTrigger`, `MessageScrollerJumpButton`, and
  `TooltipTrigger` render a `button` with no click handler. Nothing happens
  when someone clicks them, and the app cannot change that. The site's
  Message Scroller example wraps the jump button in a `div onclick` so the
  click bubbles somewhere it can be handled.
- `MessageScrollerJumpButton` has no `type`, so inside a form it submits the
  form.
- `FieldLabel` renders a `label` with no `for`. The site's Field example
  labels each input with `aria-label` that repeats the visible label, and a
  click on the label does not focus the input.
- `BreadcrumbLink` and `HoverCardTrigger` render an `a` that accepts only
  `href` and `class`, so `id`, `target`, `rel`, and `title` cannot be set.
- None of these parts accepts `id`, `title`, `aria-*`, or `data-*`.
- Dioxus 0.7.9 `#[props(extends = ...)]` forwards attributes but not event
  listeners: `Label { onclick: ... }` does not compile. A callback has to be
  an explicit prop.

## Decision

Follow RFC 0028 to RFC 0036:

- `onclick: Option<EventHandler<MouseEvent>>` on `ButtonGroupItem`,
  `InputGroupAction`, `AttachmentAction`, `AttachmentTrigger`,
  `ComboboxTrigger`, `MessageScrollerJumpButton`, and `TooltipTrigger`. A
  disabled button does not fire it, as with any native button.
- `attributes` extends `GlobalAttributes` and `button` on those parts, and is
  spread after the explicit attributes, so the part's own `type`, `role`,
  `aria-expanded`, `id`, and `data-*` state attributes keep their values.
- `MessageScrollerJumpButton` renders `type="button"`.
- `FieldLabel` takes `r#for` and `attributes` extending `GlobalAttributes`
  and `label`, matching `Label`.
- `BreadcrumbLink` and `HoverCardTrigger` take `attributes` extending
  `GlobalAttributes` and `a`. `aria-current` on `BreadcrumbLink` and the
  hover trigger marker stay explicit.

```rust
let mut zoom = use_signal(|| 100);

rsx! {
  ButtonGroup { aria_label: "Zoom",
    ButtonGroupItem { "aria-label": "Zoom out", onclick: move |_| zoom -= 10, "-" }
    ButtonGroupItem { "aria-label": "Zoom in", onclick: move |_| zoom += 10, "+" }
  }
  Field {
    FieldLabel { r#for: "email", "Email" }
    Input { id: "email", r#type: "email" }
  }
}
```

## Scope

In scope:

- the props in the crate and the templates, with SSR tests for passed
  attributes
- preview fixtures and `npm run verify:runtime-interactions` assertions
- the site examples, which drop their workarounds
- the Attribute Forwarding section of `docs/component-api.md`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| `onclick` on close buttons, menu triggers, tabs, accordion and toggle group items, and calendar days | Their click already reports through `on_open_change`, `on_toggle`, `on_value_change`, or `on_select` | A consumer needs a side effect the component callback cannot carry |
| `onclick` on `BreadcrumbLink`, `NavigationMenuLink`, and `HoverCardTrigger` for in-app navigation | They are links with an `href`; a router app can put its own `Link` in the item | A router or Desktop consumer needs in-app navigation from these links |
| Attributes on every remaining part | Static wrappers have no reported need, and source-copy users can edit the template | A consumer reports a missing attribute on a specific part |
| Desktop and Mobile self-test scenarios | The parts use plain click events with no script | They gain script behavior |

## Verification

- The CLI generated fixture smoke keeps the templates compiling.
- SSR tests render a passed `id` or `aria-*` attribute on each part, and the
  part's own state attributes win over passed ones.
- The Web preview renders each action part with a click counter, and
  `npm run verify:runtime-interactions` asserts that a click runs the
  callback, that a disabled part does not, that the jump button does not
  submit its form, and that a click on a `FieldLabel` focuses its input.
- Reverse checks: dropping a part's `onclick`, `type="button"` on the jump
  button, or `for` on `FieldLabel` each make the verifier fail.
