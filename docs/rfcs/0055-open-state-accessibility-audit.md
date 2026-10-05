# RFC 0055: Open State Accessibility Audit

- Status: Accepted
- Created: 2026-10-05

## Summary

Audit the runtime preview while each overlay, menu, and popup is open, and fix
what a probe of those states found: an unnamed and unlinked Select listbox,
disabled Select and Dropdown items that do not say so, an unnamed DatePicker
dialog, and a Hover Card that claims a dialog role.

## Current State

RFC 0054 audits the preview at its contrast points: the first render, an open
Dialog, and after the interactions. Those points left this out of scope until
"a component fails an audit in a state the points miss". A one-off probe
opened each of the preview's 24 popup triggers, the Context Menu, the Tooltip,
and the Hover Card, and ran the same audit in both themes:

| Rule | Where | Cause |
| --- | --- | --- |
| `aria-input-field-name` (serious) | open `SelectContent` | the `listbox` has no name and takes no attributes, so an app cannot give it one |
| `aria-required-attr` (critical) | open `SelectTrigger` | an expanded `role="combobox"` needs `aria-controls`, and the content has no `id` to point at |
| `color-contrast` (serious) | disabled `SelectItem`, `DropdownItem` | they set `data-disabled` but not `aria-disabled`, so assistive technology does not hear that they are disabled and axe does not exempt their faded text |
| `aria-input-field-name`, `aria-required-attr` | open `ComboboxList`, `ComboboxInput` | the same gap as Select; the probe never opened the Combobox, and the runtime audit found it |
| `aria-dialog-name` (serious) | open `DatePickerContent` | a `role="dialog"` with no name and no attributes |
| `aria-dialog-name` (serious) | open `HoverCardContent` | a `role="dialog"` with no name |

The other states the probe opened passed. Context Menu, Menubar, Command, and
Combobox items already set `aria-disabled`.

## Decision

### Select

`SelectTrigger` already asks the app to pass its `id` as the content's
`anchor_id`. Both parts derive the content id from that one value:

- `SelectContent` with an `anchor_id` renders `id="{anchor_id}-content"` and
  `aria-labelledby="{anchor_id}"`, so the listbox takes the trigger's name.
- `SelectTrigger` with an `id` renders `aria-controls="{id}-content"`.

No new prop is needed, and a Select without ids renders neither attribute, as
before. A passed `aria-controls` replaces the derived one; the shared
`default_attribute` helper now takes any value type for this.

### Combobox

The listbox is `ComboboxList`, inside `ComboboxContent`, so the content passes
its `anchor_id` to the list through context:

- `ComboboxList` inside content with an `anchor_id` renders
  `id="{anchor_id}-list"` and `aria-labelledby="{anchor_id}"`.
- `ComboboxInput` with an `id` renders `aria-controls="{id}-list"`, unless
  one is passed. `ComboboxTrigger` keeps taking `aria-controls` from the app.

### Disabled items

`SelectItem` and `DropdownItem` render `aria-disabled` next to
`data-disabled`, like the Context Menu and Menubar items. The listbox script
already skips either attribute, so keyboard behavior does not change.

### DatePicker

`DatePickerContent` with an `anchor_id` renders `aria-labelledby="{anchor_id}"`,
so the dialog takes the trigger's name, which is the field's label or chosen
date. The Calendar grid inside keeps its own month caption.

### Hover Card

`HoverCardContent` drops `role="dialog"`. The Radix Hover Card that shadcn/ui
wraps renders no role, since the card previews a link for sighted users and
the link stays the accessible element. The content keeps its
`data-dxui-hover-content` marker, which the hover script and the checks use.

### Audit

`npm run verify:runtime-interactions` runs the RFC 0054 audit, in both themes,
while each of these fixtures is open: Popover, Select, Combobox, Command,
DatePicker, Dropdown, Menubar, Navigation Menu, Context Menu, Tooltip, Hover
Card, Toast, Sonner, and Alert Dialog, next to the existing open Dialog.

## Scope

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Auditing open states on the site | Site examples render closed, and the preview fixtures open every overlay component | A site example opens an overlay that no preview fixture covers |
| Desktop and Mobile audits | axe rules read the DOM, which is the same in each renderer; Playwright WebKit is not installed, and the Desktop and iOS previews use the system WKWebView, not Playwright WebKit; the self-tests have no result channel | A WebKit-only accessibility defect is reported |
| A separate name prop on `SelectContent` or `DatePickerContent` | The trigger's name describes the popup in every example | An app needs a popup name that differs from its trigger |

## Verification

- SSR tests: `SelectTrigger` and `ComboboxInput` render `aria-controls`,
  `SelectContent` and `ComboboxList` render the matching `id` and
  `aria-labelledby`, a passed `aria-controls` replaces the derived one, disabled `SelectItem` and
  `DropdownItem` render `aria-disabled="true"`, `DatePickerContent` renders
  `aria-labelledby`, and `HoverCardContent` renders no `role`.
- The runtime check keeps its Select, Dropdown, DatePicker, and Hover Card
  keyboard and dismissal assertions.
- Reverse checks: restoring the Hover Card dialog role, or dropping
  `aria-disabled` from `SelectItem`, makes the runtime check fail.
