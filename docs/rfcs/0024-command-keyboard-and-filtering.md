# RFC 0024: Command Keyboard And Filtering

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Command a root that reports chosen items, and an input that reports
typed text and controls the list. Add a command mode to the listbox script
(RFC 0012). In that mode:

- the first option is highlighted;
- Up, Down, Home, and End move the highlight;
- the highlight goes back to the first match when the query changes;
- Enter or a click chooses an item while focus stays in the input.

Filtering stays app-owned, with a small matching helper.

## Current State

As of M148:

- `CommandInput` renders `role="combobox"` with a static
  `aria-activedescendant`, but has no `oninput` or `placeholder`. An app
  cannot read what the user types without wrapping its own input.
- No part handles arrow keys or Enter. The app has to track the highlighted
  item, pass `active_id` to the input and list, and set `active` on each item
  itself.
- `CommandItem` takes a required `id` but no value, so nothing can report
  which item was chosen.
- The input has no `aria-controls` and the list has no id.
- Select and Combobox already use the listbox script, which keeps focus on an
  anchor input and highlights options through `aria-activedescendant`.
  Combobox mode starts with no highlight and has no Home or End, which suits
  a popup that opens on typing but not a palette that is always open.
- `docs/components/accessibility.md` lists "Needs keyboard event and
  filtering integration verification" as Planned for Command.

## Decision

### State

The query and the chosen item stay with the app, as with Combobox. The app
keeps the query, filters the items it renders, and handles `Command`
`on_select(String)`:

```rust
let mut query = use_signal(String::new);
let items = [("calendar", "Calendar"), ("settings", "Settings")];

rsx! {
  Command {
    on_select: move |value: String| run(&value),
    CommandInput {
      value: query(),
      placeholder: "Type a command...",
      oninput: move |event: FormEvent| query.set(event.value()),
    }
    CommandList {
      for (value, label) in items.into_iter().filter(|(_, label)| command_matches(label, &query())) {
        CommandItem { key: "{value}", id: "command-{value}", value, "{label}" }
      }
    }
  }
}
```

`command_matches(label, query)` returns true when the trimmed query is empty
or the label contains it, ignoring case.

### Components

- `Command` provides a base id through context and runs the listbox script
  in command mode on its root, with the input as the anchor. A chosen item
  calls `on_select` with its value.
- `CommandInput` gains `placeholder` and `oninput`. Inside `Command` it
  renders an id and `aria-controls` naming the list.
- `CommandList` renders an id inside `Command`.
- `CommandItem` gains an optional `value`, rendered as `data-value`. Without
  it, the item's `id` is the value. Its class highlights `data-highlighted`,
  as Combobox items do.
- The `active_id`, `active`, and `selected` props stay, so apps that track
  the highlight themselves keep working. Without `Command`, the parts work as
  today.

### Command Mode

Focus stays in the input. The script reads keys from the input and
highlights options with `data-highlighted` and the input's
`aria-activedescendant`.

| Key or event | Behavior |
| --- | --- |
| Script start | Highlight the selected option, or else the first enabled option |
| ArrowDown, ArrowUp | Move to the next or previous enabled option; stop at the ends |
| Home, End | Highlight the first or last enabled option |
| Enter | Choose the highlighted option |
| Typing, including Space | Goes to the input; no typeahead |
| Query change | Highlight the first enabled option once the app re-renders the list |
| Pointer move over an option | Highlight it |
| Click on an option | Choose it; focus stays in the input |

- When filtering leaves no enabled options, the input has no
  `aria-activedescendant` and Enter does nothing.
- Disabled items are skipped and cannot be chosen.
- Home and End move the highlight instead of the caret, as in cmdk.
- The highlight resets after an `input` event on the anchor. The script
  waits for the next DOM change in the list, which is the app's re-render.
  It also resets at once, so a query that changes nothing still starts from
  the first option.

Select, Combobox, and menu modes do not change.

## Scope

In scope:

- the `Command` root, item values, and choice reporting
- input text reporting and the input-to-list link
- command mode in the listbox script
- the matching helper
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Fuzzy matching and ranking | The app renders the items, so it can sort them; cmdk's scorer is a separate design | A consumer needs ranked results |
| Looping at the ends | cmdk does not loop by default | A consumer asks for looping |
| Ctrl+N, Ctrl+P, Ctrl+J, and Ctrl+K bindings | cmdk's vim bindings are optional and can clash with app shortcuts | A consumer asks for them |
| A command dialog part | Dialog and Command compose; Escape and focus already come from Dialog | A consumer reports a gap when composing them |
| Announcing the result count | Needs a live region policy shared with Combobox | Combobox result announcements are designed |
| Desktop and Mobile self-test scenarios | The listbox script already runs in the WebViews for Select | The script relies on behavior that differs between WebViews |

## Verification

- The CLI parity test keeps the template script identical to the crate
  script.
- Unit tests cover the matching helper and the item value fallback.
- The Web preview renders a real Command, and
  `npm run verify:runtime-interactions` asserts:
  - the input controls the list;
  - the first option starts highlighted;
  - arrows skip the disabled item and stop at the ends;
  - Home and End;
  - typed text, including a space, reaches the input;
  - the highlight resets to the first match when the query changes;
  - the empty state and no active descendant when nothing matches;
  - pointer highlight;
  - Enter and click choose an item while focus stays in the input.
