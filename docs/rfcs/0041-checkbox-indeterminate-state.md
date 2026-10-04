# RFC 0041: Checkbox Indeterminate State

- Status: Accepted
- Created: 2026-10-05

## Summary

Add the mixed state to `Checkbox`. An `indeterminate` prop sets the native
input's `indeterminate` property, which browsers expose to assistive
technology as a mixed checkbox. A change from the mixed state requests
`true`, and the component restores the property when the app keeps the
checkbox mixed after a click.

## Current State

As of M165:

- `Checkbox` renders a native checkbox with `checked` and reports
  `on_checked_change(!checked)`. It has no mixed state, so a "select all"
  checkbox over a partial selection can only show checked or unchecked.
- The native mixed state is the `indeterminate` DOM property. HTML has no
  attribute for it, Dioxus 0.7 has no `indeterminate` attribute either, and
  ARIA in HTML asks authors not to put `aria-checked` on a native checkbox.
- A click on an indeterminate checkbox clears the property in the browser
  before any handler runs. If the app keeps the checkbox mixed, its props do
  not change, Dioxus patches nothing, and the property stays cleared.

## Decision

- `Checkbox` gains `indeterminate: bool` (default `false`) and renders
  `data-state` as `checked`, `unchecked`, or `indeterminate`, with the checked
  colors for the mixed state.
- An effect sets the input's `indeterminate` property through
  `document::eval` whenever the prop changes. A checkbox that has never been
  mixed starts no script.
- A change while mixed calls `on_checked_change(true)`, so the next state is
  checked, as in Radix.
- The change handler also bumps a counter the effect depends on, so the
  effect runs again after the app's next render and puts the property back
  when the app kept the checkbox mixed.

## Scope

In scope:

- the prop, the property sync, the requested state, and `data-state` in the
  crate and the template
- the Checkbox docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Restoring `checked` when the app ignores a change | Every controlled checkbox has that drift; it is not specific to the mixed state | A consumer reports a checkbox that ignores changes |
| A mixed state on Switch or menu checkbox items | Switch has no mixed state in ARIA; menu items are not native inputs | A consumer needs a mixed menu item |
| Server-rendered mixed state | The property can only be set in the browser | A consumer needs the mixed state before hydration |
| Desktop and Mobile self-test scenarios | The eval sets one property and the WebViews already run longer page scripts | A WebView reports a different checkbox behavior |

## Verification

- The CLI generated fixture smoke keeps the template compiling.
- The Web preview renders a "Select all" checkbox over two items and a
  checkbox that stays mixed, and `npm run verify:runtime-interactions`
  asserts:
  - a partial selection makes the select-all input indeterminate with
    `data-state="indeterminate"`;
  - a click on it checks every item and clears the mixed state, and
    unchecking one item makes it mixed again;
  - a click on the checkbox that stays mixed leaves its input indeterminate.
- Reverse checks: not setting the property, requesting `!checked` while
  mixed, or not re-syncing after a change each make the verifier fail.
