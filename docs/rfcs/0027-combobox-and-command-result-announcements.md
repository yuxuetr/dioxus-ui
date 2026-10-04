# RFC 0027: Combobox And Command Result Announcements

- Status: Accepted
- Created: 2026-10-04

## Summary

Add `CommandStatus` and `ComboboxStatus`: a visually hidden, polite status
region that stays mounted while the app changes its text. The app decides the
wording and when to speak, for example "3 results" or "No results".

## Current State

As of M151:

- Combobox (RFC 0012) and Command (RFC 0024) keep focus in the input and
  highlight options through `aria-activedescendant`. A screen reader reads
  the highlighted option, but nothing says how many options the query left.
- `CommandEmpty` and `ComboboxEmpty` render plain `div` elements. Apps
  render them only when nothing matches, and screen readers do not announce
  content that arrives in a newly inserted element unless it is a live
  region that already existed.
- Toast and Sonner already use `aria-live` regions, but only for their own
  messages.
- `docs/components/accessibility.md` lists result count announcements as
  Planned for Combobox and Command.
- WAI-ARIA APG does not require a count. GOV.UK's accessible autocomplete
  and several design systems announce one through a polite status region.

## Decision

### Parts

```rust
let matches = items.iter().filter(|(_, label)| command_matches(label, &query())).count();

rsx! {
  Command {
    CommandInput { /* ... */ }
    CommandStatus {
      if query().is_empty() { "" }
      else if matches == 0 { "No results" }
      else if matches == 1 { "1 result" }
      else { "{matches} results" }
    }
    CommandList { /* ... */ }
  }
}
```

Both parts render:

```html
<div role="status" aria-live="polite" aria-atomic="true" class="sr-only">…</div>
```

- `role="status"` implies polite and atomic. The explicit attributes match
  Toast and help older assistive technology.
- `sr-only` keeps the region out of the layout but in the accessibility tree.
  A `class` prop is appended, so an app can show the text instead.
- The region must stay mounted. Render the part every time and change only
  its children. Rendering it conditionally recreates the element, and the
  new element's first text is not announced.

### Combobox Placement

`ComboboxContent` is `hidden` while closed, and content inside a hidden
element leaves the accessibility tree. Place `ComboboxStatus` next to
`ComboboxInput`, outside `ComboboxContent`. The docs show this layout, and
the browser check fails if the region is inside the popup.

### What Stays With The App

- The wording, its language, and pluralization.
- Whether to count disabled options.
- When to speak. An empty text says nothing, so apps can stay quiet for an
  empty query or a closed popup.

## Scope

In scope:

- `CommandStatus` and `ComboboxStatus` in the crate and the templates
- the Command and Combobox docs pages
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Built-in wording or a count helper | Wording and plural rules are locale-specific, and the app already has the count from its own filter | Two consumers write the same English helper |
| Debouncing the announcement | Screen readers replace or interrupt queued polite messages while the user types; a debounce needs a timer in every renderer | A screen reader user reports repeated counts while typing |
| Counting in the listbox script | The app renders the options, so it knows the count without reading the DOM | An app cannot count the options it renders |
| Desktop and Mobile self-test scenarios | The parts are plain markup with no script | The parts gain script behavior |
| Screen reader testing | The browser check covers the markup and the mounted element, not speech output | A screen reader test harness is added |

## Verification

- The CLI registry tests keep the templates compiling through the generated
  fixture smoke.
- The Web preview renders both parts, and
  `npm run verify:runtime-interactions` asserts:
  - `role="status"`, `aria-live="polite"`, and `aria-atomic="true"`;
  - the text follows the filtered count and says "No results" when nothing
    matches;
  - the same element stays mounted across query changes;
  - the Combobox region is outside the popup and stays in the accessibility
    tree while the popup is closed.
- Reverse checks: remounting the region on each change, removing the role or
  the live attribute, or moving the Combobox region inside the popup each
  make the verifier fail.
