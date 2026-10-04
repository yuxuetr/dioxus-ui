# RFC 0038: Form Control Naming

- Status: Accepted
- Created: 2026-10-05

## Summary

Let apps name Radio Group, Progress, the Select trigger, and the Combobox
input. Each passes through global and element attributes, so a `Label` can
point at an item with `for`, a group or progress bar can take
`aria-labelledby` or `aria-label`, and a trigger or input can take
`aria-describedby` for a field description.

## Current State

As of M162:

- `RadioGroupItem` renders a radio button whose only child is an
  `aria-hidden` indicator, and it accepts no `id`, `aria-label`, or
  `aria-labelledby`. Every radio has an empty accessible name. The browser
  verifier has to find radios by `data-value` because a role query with a
  name finds none.
- `RadioGroup` cannot take `aria-labelledby`, so the group is unnamed too.
- `Progress` renders `role="progressbar"` with values but cannot take a name
  or `aria-valuetext`. ARIA requires a name for a progress bar.
- `SelectTrigger` and `ComboboxInput` take an `id`, so a `Label` with `for`
  names them, but they cannot take `aria-describedby` for a Field
  description or error, or `aria-labelledby` when the label is not a
  `label` element.

## Decision

- `RadioGroup` extends `GlobalAttributes` and `div`. `RadioGroupItem`
  extends `GlobalAttributes` and `button`, so `Label { r#for: "size-small" }`
  names `RadioGroupItem { id: "size-small" }`.
- `Progress` extends `GlobalAttributes` and `div`.
- `SelectTrigger` extends `GlobalAttributes` and `button`, and
  `ComboboxInput` extends `GlobalAttributes` and `input`. Their existing `id`
  props stay.
- The spread comes after the explicit attributes, so the roles, states, and
  roving and listbox hooks the components render stay in place.

## Scope

In scope:

- attribute spreading on these six parts in the crate and the template
- the Radio Group, Progress, Select, and Combobox docs pages
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Attribute spreading on the remaining parts of Select, Combobox, and other components | Those parts are named by their content or are presentational | A consumer needs to name or identify one |
| A visible label child on `RadioGroupItem` | Radix and shadcn pair items with a separate `Label`; a child would change the item layout | A consumer asks for a built-in label |
| A Progress `aria-valuetext` default | The app knows how to word the value | A consumer reports a confusing announcement |
| Desktop and Mobile self-test scenarios | The change adds attributes and no behavior | The parts gain script behavior |

## Verification

- The CLI generated fixture smoke keeps the templates compiling.
- The Web preview labels the radio group, its items, a progress bar, the Select
  trigger, and the Combobox input, and `npm run verify:runtime-interactions`
  asserts their accessible names and descriptions through role queries, and
  that roving focus and listbox behavior still work.
- Reverse checks: not spreading the attributes of the group, an item,
  Progress, the Select trigger, or the Combobox input each make the verifier
  fail.
