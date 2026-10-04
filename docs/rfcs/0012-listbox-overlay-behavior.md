# RFC 0012: Listbox Overlay Behavior

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Select and Combobox real listbox behavior: the trigger or input opens
anchored listbox content, keyboard focus stays on the trigger or input while
`aria-activedescendant` tracks the highlighted option, Select supports
typeahead, and choosing an option reports its value and requests close.

## Current State

As of M136:

- `SelectTrigger` and `ComboboxTrigger` render `aria-expanded` but have no
  event handlers; the app wires every open and close path by hand.
- `SelectContent` and `ComboboxContent` render `open` state only, with no
  anchor and no placement.
- Options have no click handling, no keyboard navigation, no highlighted
  option, and no typeahead. `TypeaheadState` and `ActiveDescendantState` exist
  in `dioxus-ui-primitives`, but no component uses them.
- `ComboboxInput` has no `oninput`, so the app cannot read typed text through
  the component, and its `aria-expanded` is always `"true"`.
- RFC 0010 deferred Select and Combobox until listbox keyboard wiring could be
  verified together with placement.

## Decision

### API

All new props are optional, so existing `open`-only usage renders the same.

- `SelectTrigger` gains `id` and `on_open_change`. Click requests
  `!open`; ArrowDown or ArrowUp on a closed trigger requests open. Enter and
  Space keep their native button click. The trigger also renders
  `aria-haspopup="listbox"`.
- `ComboboxInput` gains `id`, `open` (default `true`, the current
  `aria-expanded` output), `placeholder`, `oninput`, and `on_open_change`.
  ArrowDown on a closed input requests open. Filtering stays with the app,
  which re-renders the matching items from `oninput`.
- `SelectContent` and `ComboboxContent` gain the RFC 0010 anchoring props
  (`anchor_id`, `side` default Bottom, `align` default Start, `side_offset`
  default 4), `dismiss` (default `popover_default()`), `on_open_change`, and
  `on_value_change: Option<EventHandler<String>>`.

`anchor_id` names both the placement anchor and the element that keeps focus
and receives the listbox keys: the Select trigger or the Combobox input. One
prop keeps the two from disagreeing.

### Listbox Script

When content opens with an `anchor_id`, the anchored overlay script from
RFC 0010 places it, and a second page script handles the listbox:

1. Collect enabled options (`role="option"` without `data-disabled="true"` or
   `aria-disabled="true"`) from the DOM on every key, so items the app
   re-renders while filtering are picked up. Options without an `id` get one
   derived from the scope id.
2. Highlight one option by setting `data-highlighted` on it and
   `aria-activedescendant` on the anchor, and scroll it into view. Select
   starts on the selected option or the first enabled one. Combobox starts
   with no highlight until ArrowDown or ArrowUp, so Enter in a fresh input
   does not pick an option the user never moved to.
3. ArrowDown and ArrowUp move between enabled options without wrapping. Select
   also handles Home, End, Space, and typeahead: printable characters build a
   prefix buffer that resets after 500 ms. Repeating one letter searches for
   that letter after the highlighted option, cycling through matches; a longer
   prefix keeps the highlighted option while it still matches.
4. Enter (and Space in Select) chooses the highlighted option. Pointer
   movement highlights an option, and click chooses it. `pointerdown` inside
   the content is prevented so focus stays on the anchor.
5. When filtering removes the highlighted option, Select moves to the selected
   or first option and Combobox clears the highlight.
6. Choosing sends the option's `data-value`; Rust calls
   `on_value_change(value)` and then `on_open_change(false)`.

A new `data-highlighted` attribute is used instead of the existing
`data-active` attribute, which `ComboboxItem` renders from its controlled
`active` prop; the script would otherwise fight the Rust render. Item classes
gain `data-highlighted:` styles.

Typeahead runs in the page because a round trip per keystroke would add an IPC
hop on Desktop and Mobile and would need a copy of the item labels in Rust.
The script mirrors the `match_typeahead` rules (skip disabled, search after
the active option, case-insensitive prefix).

### Source-Copy Templates

The listbox script and its hook join the RFC 0010 helpers in the shared
`utils.rs` template, and the CLI parity test covers the new script.

## Scope

In scope:

- Select and Combobox opening, anchored placement, keyboard navigation,
  pointer selection, Escape and outside dismissal
- Select typeahead
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Multi-select | Choosing closes the content; no multi-value API exists | A consumer needs a multi-value Select or Combobox |
| Async loading states | Apps own item fetching and already render `ComboboxEmpty` | A consumer needs a loading part with announcements |
| Combobox with the input inside the content | Needs initial focus into the content and focus return to the trigger | The interaction smoke gains a trigger-plus-inner-input fixture |
| Command keyboard navigation | Command is not an overlay and has its own docs plan | A consumer reports Command keyboard gaps |
| Grouped option headings in typeahead | Labels are read from option text only | A consumer reports group labels matched by typeahead |

## Verification

- The CLI parity test keeps the template script identical to the crate
  script.
- The Web preview renders real Select and Combobox components, and
  `npm run verify:runtime-interactions` asserts placement, arrow navigation
  skipping disabled options, typeahead, Enter and click selection, focus
  staying on the anchor, filter resets, and Escape.
- Desktop and Mobile share the `document::eval` path but are not covered by an
  automated gate.
