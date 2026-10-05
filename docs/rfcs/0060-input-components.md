# RFC 0060: Input Components

- Status: Accepted
- Created: 2026-10-05

## Summary

Add Rating, Number Input, Tags Input, File Input, and Swap: the daisyUI and
shadcn/ui ecosystem inputs that have no counterpart here. Each builds on a
native control, so keyboard, form, and assistive technology support come
from the browser, and each is controlled like the existing inputs.

## Current State

Apps collecting a star rating, a quantity, a list of labels, an upload, or a
two-state icon toggle build them from raw markup. `Input` binds a controlled
`value`, which a file input cannot take, and has no `file:` styles. daisyUI
has Rating, File Input, and Swap; number and tag inputs are the most
requested shadcn/ui ecosystem additions.

## Decision

### Rating

`Rating` is a `role="radiogroup"` of native radio inputs drawn as stars with
a CSS mask, filled up to `value` with `--warning`. `value` is 0 for no rating
up to `max` (default 5). A change calls `on_value_change` with the chosen
star; the arrow keys move between stars as in any radio group. Each radio is
named "{n} of {max}", and `aria-label` on the group names the rating.
`name` defaults to a generated unique name so two ratings do not share a
group. `disabled` disables every star.

### Number Input

`NumberInput` is a text input with `inputmode="decimal"` and
`role="spinbutton"`, between decrement and increment buttons. A native
`type="number"` input reports `value` as an empty string while its text is
incomplete, such as "-" or "1.", so a controlled render would wipe what the
user is typing; owning the text avoids that. `value` and `step` are `f64`,
and `min` and `max` are optional. The input keeps the text being typed and
calls `on_value_change` only with a number that parses. On blur it clamps
to `min` and `max` and rounds to the step's decimals. ArrowUp and ArrowDown
step, Home and End go to `min` and `max`, and the input sets
`aria-valuenow`, `aria-valuemin`, and `aria-valuemax`. The buttons step and
clamp; they are `tabindex="-1"`, since the keys do the same for keyboard
users, and are named by `decrement_label` and `increment_label`, "Decrease"
and "Increase" by default.

### Tags Input

`TagsInput` shows `tags: Vec<String>` as removable chips before a text
input. Enter or a comma adds the trimmed draft unless it is empty or already
present; Backspace in an empty input removes the last tag; each chip's
remove button is named "Remove {tag}". Changes call `on_tags_change` with the
new list. The chips are a `ul`, so assistive technology hears the count.
`id`, `placeholder`, and ARIA attributes go to the text input.

### File Input

`FileInput` is a native `type="file"` input styled through `file:`, with no
`value` binding. `onchange` passes the form event, whose `files()` the app
reads; `accept` and `multiple` pass through. Upload transport stays app-owned
as in Attachment.

### Swap

`Swap` is a toggle button that shows its `on` element when `active` and its
`off` element otherwise, such as a menu icon that becomes a close icon. The
two are element props rather than child parts, so Swap computes each layer's
classes from the effect and state without stacked group selectors. The hidden
layer is `aria-hidden`. It sets `aria-pressed`, takes `aria-label`, and calls
`on_active_change`. `SwapEffect` is `Fade` (default), `Rotate`, or `Flip`;
motion stops under `prefers-reduced-motion`.

## Alternatives

- **Custom star buttons for Rating.** Radios give the group semantics, arrow
  keys, and form value without code.
- **A native `type="number"` input for Number Input.** It has the spinbutton
  role and arrow keys built in, but its empty `value` for incomplete text
  breaks a controlled input.

## Verification

- SSR tests for each component's semantics.
- Unit tests for the Tags Input and Number Input state functions.
- A site example per component, audited in both themes and every preset.
- Runtime checks for Rating arrow keys, Number Input typing and clamping,
  Tags Input adding and removing, and Swap pressing.
