# RFC 0019: Roving Group Interaction

- Status: Accepted
- Created: 2026-10-04

## Summary

Tabs, Radio Group, and Toggle Group are groups where the whole group takes
one Tab stop and arrow keys move between items. Give them one shared page
script that:

- keeps one Tab stop per group;
- moves focus with arrow keys, Home, and End, skipping disabled items;
- selects the focused item in Tabs and Radio Group;
- reports clicks to Rust.

Tabs also gains a root that links each trigger to its panel by id.

## Current State

As of M143:

- `TabsTrigger` is a `button` with `role="tab"` and no click handler. An app
  cannot switch tabs without wrapping the trigger itself.
- No item in the three components handles arrow keys.
- `RadioGroupItem` and `ToggleGroupItem` render `tabindex="0"` only on a
  checked or pressed item. When nothing is selected, every item has
  `tabindex="-1"` and the group cannot be reached with Tab.
- `TabsTrigger` and `TabsContent` have no ids, so triggers have no
  `aria-controls` and panels have no `aria-labelledby`.
- `radio_group_move_value` and `toggle_group_move_value` compute the next
  value from a `RovingFocusItem` list, but no component calls them. An app
  would have to build the item list and move DOM focus itself.

## Decision

### State

State stays controlled. The app keeps the selected value and passes
`active`, `checked`, or `pressed` to each item, as today. Each group reports
one request:

| Component | Request | Value |
| --- | --- | --- |
| `Tabs` | `on_value_change(String)` | the tab to show |
| `RadioGroup` | `on_value_change(String)` | the item to check |
| `ToggleGroup` | `on_toggle(String)` | the item the user toggled |

Toggle Group reports the toggled item rather than the next selection,
because the root does not hold the current selection.
`toggle_group_single_selection` and `toggle_group_multiple_selection` compute
the next selection from it.

### Roving Group Script

The group root runs one page script for its lifetime, like Menubar
(RFC 0015). Items are read from the DOM on every event. Items inside a nested
group belong to that group.

The root sets three attributes:

- orientation: `horizontal`, `vertical`, or `both`;
- whether movement wraps;
- whether selection follows focus.

Tab stop:

- Exactly one enabled item has `tabindex="0"`, and the others have `-1`.
- The focused item becomes the Tab stop.
- In groups where selection follows focus, the Tab stop is:
  1. the selected enabled item;
  2. otherwise the last focused item;
  3. otherwise the first enabled item.
- In Toggle Group the order is:
  1. the last focused item;
  2. otherwise the first pressed item;
  3. otherwise the first enabled item.
- The script recomputes the Tab stop when selection, `disabled`, or the item
  list changes.
- The Rust components stop rendering `tabindex` on items, so the script is
  the only owner.

Keyboard:

| Orientation | Next | Previous |
| --- | --- | --- |
| `horizontal` | ArrowRight | ArrowLeft |
| `vertical` | ArrowDown | ArrowUp |
| `both` | ArrowRight, ArrowDown | ArrowLeft, ArrowUp |

- Movement skips disabled items. It wraps when looping is on. At the ends,
  with looping off, focus stays put and the key is still consumed.
- Home and End move to the first and last enabled item.
- Keys with Alt, Control, or Meta held are ignored.
- In Tabs and Radio Group, a move that changes focus also reports the newly
  focused item's value. This is the WAI-ARIA automatic activation for tabs
  and the standard radio group behavior.
- In Toggle Group, arrows only move focus.

Pointer and activation:

- A click on an enabled item reports its value. Enter and Space click the
  button natively, so they use the same path.

### Components

- `Tabs` is a new root that provides a base id and `on_value_change` through
  context.
  - `TabsList` runs the script with horizontal orientation, looping on, and
    selection following focus.
  - `TabsTrigger` renders `id` and `aria-controls`; `TabsContent` renders
    `id`, `aria-labelledby`, and `tabindex="0"`.
  - Ids are built from the base id, the part, and the value, with characters
    other than ASCII letters, digits, `-`, and `_` replaced by `-`.
  - Without `Tabs`, the parts keep keyboard movement but report nothing and
    render no ids.
- `RadioGroup` runs the script with its `orientation` and `looping`, with
  selection following focus.
- `ToggleGroup` runs the script with its `orientation` and `looping`. Focus
  moves without pressing.
- `radio_group_item_tabindex`, `toggle_group_item_tabindex`, and the
  `*_move_value` helpers stay exported for apps that render their own items.

The script and hook live in a crate module shared by the three features, and
in the template `utils.rs`. A CLI parity test keeps the two scripts
identical.

## Scope

In scope:

- one Tab stop per group, and arrow, Home, and End movement
- selection following focus in Tabs and Radio Group
- click reporting in all three components
- Tabs trigger and panel id links
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Manual tab activation | Radix and shadcn/ui default to automatic activation | A consumer needs tabs whose panels are expensive to show |
| Vertical tabs | `TabsList` has no orientation prop | An orientation prop is added to `TabsList` |
| Right-to-left arrow mirroring | Left and Right follow visual order only in LTR, as in Menubar | A consumer reports RTL group movement |
| Accordion | Accordion triggers are each a Tab stop, and its trigger and panel ARIA links are a separate gap | Accordion behavior is designed |
| Desktop and Mobile self-test scenarios | Desktop and Mobile already run the shared `document::eval` path through eight scenarios; the script uses only focus and click | The script relies on behavior that differs between WebViews |

## Verification

- A CLI parity test keeps the template script identical to the crate script.
- Unit tests cover Tabs id building.
- The Web preview renders real Tabs, Radio Group, and Toggle Group, and
  `npm run verify:runtime-interactions` asserts:
  - Tab stops: the selected tab, the first enabled radio when nothing is
    checked, and the last focused toggle;
  - arrow movement that skips disabled items and wraps;
  - Home and End;
  - selection following focus in Tabs and Radio Group;
  - focus without pressing in Toggle Group;
  - clicks;
  - Tabs id links, with Tab moving from the selected trigger into its panel.
