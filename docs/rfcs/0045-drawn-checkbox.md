# RFC 0045: Drawn Checkbox

- Status: Accepted
- Created: 2026-10-05

## Summary

Make the Checkbox follow its classes. The input drops its native appearance
with `appearance-none`, and the checked and mixed states draw a tick and a
dash as background images keyed off `data-state`, on the blue fill.

## Current State

As of M169:

- `Checkbox` renders `<input type="checkbox">` with native appearance.
  Chrome paints the native control and ignores the border and background
  classes: a checked input computes a white border and paints the browser's
  own blue, whatever the classes say. Browser checks with compiled Tailwind
  showed this while checking the checked border color.
- The mixed state comes only from the native `indeterminate` property, so
  its look is the browser's, and server-rendered HTML shows nothing until
  hydration sets the property.

## Decision

- `CHECKBOX_BASE_CLASS` adds `appearance-none`, `bg-center`, and
  `bg-no-repeat`, so the border, fill, and rounded corners the classes set
  are what the input paints.
- `data-[state=checked]:` adds a white tick and `data-[state=indeterminate]:`
  a white dash, each an inline SVG data URI in a complete Tailwind token.
  The mixed state also takes `border-blue-600` and `bg-blue-600`, since it
  can be mixed while `checked` is false.
- The marks are keyed off the rendered `data-state`, so server-rendered HTML
  shows them before hydration. The native `indeterminate` property, which
  assistive technology reads, is still set after hydration as in RFC 0041.
- The element stays an `input`, so labels, `form` submission, and passed
  attributes work as before.

## Scope

In scope:

- the appearance, marks, and mixed fill in the crate and the template
- the Checkbox docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Custom mark colors | A data URI cannot read `currentColor`; the marks are white on the blue fill | A theme needs a fill on which white marks are unreadable |
| A wrapper element with an icon | Changes the structure that labels and passed attributes rely on | A mark needs more than a background image |
| Forced-colors styling | Background images are removed in forced colors mode; the native checked state still reaches assistive technology | A forced-colors user reports a missing mark |

## Verification

- `npm run verify:tailwind-conflicts` and `npm run verify:tailwind-static`
  pass with the new tokens, and the compiled preview stylesheet contains both
  marks.
- `npm run verify:runtime-interactions` asserts that a checked Checkbox has
  `appearance: none`, the `bg-blue-600` color, and an SVG background image,
  that an unchecked one has no background image, and that a mixed one has
  the blue fill and an SVG mark different from the tick.
- Reverse checks: removing `appearance-none`, the tick, or the dash each make
  the verifier fail.
