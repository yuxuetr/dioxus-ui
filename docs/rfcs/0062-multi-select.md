# RFC 0062: Multi-Select

- Status: Accepted
- Created: 2026-10-05

## Summary

Let Select and Combobox choose several options: `multiple` on the content
keeps the listbox open after a choice and marks it `aria-multiselectable`,
and selected options show a check mark, so they stay distinct from the
keyboard highlight.

## Current State

The 0.1.0 release notes exclude multi-select. Select and Combobox already
leave the selection to the app: each option takes `selected`, and a choice
calls `on_value_change` with its value, so an app can keep a set of values.
Two things stop it. The listbox requests close after every choice, and a
selected option and the highlighted one both draw `bg-accent`, so in a list
with several selected options the highlight cannot be seen.

## Decision

### `multiple`

`SelectContent` and `ComboboxContent` take `multiple: bool`. With it, the
content passes no close handler to the listbox hook, so a choice calls
`on_value_change` and the listbox stays open; Escape and outside
interactions still close it through the anchored overlay, which keeps its
own handler. The listbox sets `aria-multiselectable="true"`. Combobox's
listbox is `ComboboxList`, which reads `multiple` from the content's context
with the anchor id. The app toggles the chosen value in its set and renders
the trigger or input text, such as "Rust, Go" or "2 selected".

No listbox code changes: choosing already sends the value and leaves the
script running until the listbox hides.

### Check marks

Selected options show a check mark at the inline end, a masked `::after`
filled with the text color, and no longer draw `bg-accent`; the highlight
keeps `bg-accent`. This matches shadcn/ui's Select and applies to single
selection too. Options reserve the space with `pe-8`.

## Migration

Single-select options look different: the selected option shows a check
mark instead of the accent background. Apps that styled selected options
through `class` may need to update.

## Verification

- SSR tests for `aria-multiselectable` and the selected classes.
- A runtime check that a multiple Select stays open across two choices,
  reports both, shows both checked, and closes on Escape; and that a
  multiple Combobox stays open after a click.
- Site examples for both, audited in both themes and every preset.
