# RFC 0077: Component-Owned State

- Status: Accepted
- Created: 2026-10-06

## Summary

A compound component's root owns its state and shares it with its parts
through context. An app either leaves the state to the root, starting from a
default, or controls it with a value and a change callback; either way the
parts read it from the root, and the root links their element ids itself.
Parts used outside their root fail at render with a message that names the
root, instead of rendering inert.

## Current State

Measured on 2026-10-06 at `v0.4.2`:

- Stateful compound components are fully controlled and their parts share
  nothing. The Select doc example keeps `open` and `value` in two signals,
  passes `open` and `on_open_change` to both `SelectTrigger` and
  `SelectContent`, names the trigger with `id` and repeats it as
  `anchor_id`, and computes `selected` for every `SelectItem`. A multiple
  Select also toggles the chosen value in the app.
- `Tabs` provides a context, but only for element ids and the change
  callback; the app computes `active` for every `TabsTrigger` and every
  `TabsContent`, and the parts render without a `Tabs` around them.
- 25 crate modules take `open: bool` and 25 take per-part `active`,
  `selected`, `checked`, or `pressed`.

Every app repeats the same wiring, and a mismatch, such as an `anchor_id`
that names no trigger or a `selected` computed from the wrong signal, renders
without an error.

## Decision

### State

A root takes its state in three props, as Radix and shadcn/ui do:

| Prop | Meaning |
| --- | --- |
| `value` | The app controls the state: the root shows this value and never changes it itself |
| `default_value` | The root owns the state, starting from this value |
| `on_value_change` | Called with every change the user makes, in both modes |

Overlays take `open`, `default_open`, and `on_open_change` the same way. A
root is controlled exactly when `value` is set; `value` is a
`ReadSignal<Option<T>>`, so an app passes a plain value or a signal and the
parts follow it as it changes.

One helper, `use_controllable`, implements the rule for every root, in the
crate and as a copy-mode helper, so the controlled and uncontrolled behavior
cannot drift per component.

### Context and ids

The root provides a context holding the current state as a `Memo`, the
callbacks that change it, and a base id from `next_element_id()` (RFC 0075).
Parts derive their ids from the base id, so `aria-controls`,
`aria-labelledby`, and the anchor of an anchored overlay link without the app
naming them, and a server and the browser hydrating it render the same ids.

### Parts outside their root

A part reads its root's context with a helper that panics with a message
naming the part and the root, such as "`SelectItem` must be inside a
`Select` (RFC 0077)". Dioxus 0.7 catches a panic in a component: it logs the
message, in the browser console on the Web, and renders nothing for the part
while the rest of the page renders. The part is missing on screen and the log
says why; rendering it without its state would show a control that does
nothing. A compile error is not possible for `rsx!` children. Each moved
component has a test that renders a part alone and expects nothing for it,
and the helper's message has its own test.

### Select

`Select` is a new root. It takes `value`, `default_value`, and
`on_value_change` for one value, or with `multiple` set, `values`,
`default_values`, and `on_values_change`; and `open`, `default_open`, and
`on_open_change`. Choosing an item sets the value and closes the list, or in
a multiple Select toggles the value and keeps the list open. `SelectValue`
renders the chosen value, or the chosen values joined with commas, or else
its `placeholder`. `SelectTrigger` loses `id`, `open`, and `on_open_change`;
`SelectContent` loses `open`, `anchor_id`, `on_open_change`,
`on_value_change`, and `multiple`; `SelectItem` loses `selected`.
`SelectValue` takes no children: to show labels other than the values, put
them in the `SelectTrigger` instead. `Select` takes an optional `id` that
names the trigger, so a `Label` can point at it; without one the ids are
generated. Showing the chosen item's own text automatically would need the
items to register with the root as they render, after the trigger, so a
server render would show the raw value until hydration; it waits for an
issue that asks for it.

### Tabs

`Tabs` takes `value`, `default_value`, and `on_value_change`.
`TabsTrigger` and `TabsContent` lose `active` and compare their `value` with
the root's.

### Overlays and triggers

An overlay root takes `open`, `default_open`, and `on_open_change`. An
uncontrolled overlay needs a part that opens it, so every overlay has a
trigger part, as in shadcn/ui: `DialogTrigger`, `AlertDialogTrigger`,
`SheetTrigger`, `DrawerTrigger`, and `PopoverTrigger` are new, and the
existing triggers read the root. A trigger renders a `button`, toggles the
root's `open`, and links itself to the content with `aria-haspopup`,
`aria-expanded`, and `aria-controls`; an anchored content anchors to it.
Dioxus has no `asChild`, so a trigger has no styles of its own and takes
`class` and the button's attributes, such as `class: button_class(..)`.
A controlled overlay may leave the trigger out and open from any app button;
an anchored one, such as a Popover, keeps its trigger as the anchor. The
content and close parts lose `open`, `on_open_change`, and `anchor_id`.

### Menus

`Dropdown` is a new root with a `DropdownTrigger`. `ContextMenu` is a new
root with a `ContextMenuTrigger`, an area that opens the menu at the pointer
on a right click, so the app no longer records the point. `Menubar` takes
`value`, `default_value`, and `on_value_change` for the `value` of the open
`MenubarMenu`, the empty string while none is open, as Radix does; each menu's
trigger and content compare it with their menu's.

In every menu, a radio group owns its value (`value`, `default_value`,
`on_value_change`) and its radio items take a `value` instead of `checked`,
and a submenu root owns its open state the same way as an overlay root. A
checkbox item keeps `checked`: each item is its own state, as a Checkbox is.

### Which components move

| Batch | Components | State the root takes |
| --- | --- | --- |
| M210.1 | Select, Tabs | value (values), open |
| M210.2 | Dialog, Alert Dialog, Sheet, Drawer, Popover, Hover Card, Tooltip, Fab | open |
| M210.2 | Dropdown, Context Menu, Menubar | open; radio group value |
| M210.2 | Combobox, Navigation Menu | open; value (the open item's for Navigation Menu) |
| M210.2 | Date Picker | open |
| M210.3 | Accordion, Collapsible, Menu groups | open items |
| M210.3 | Radio Group, Toggle Group, Carousel, Command, Input OTP | value, selected or active item |

Kept as they are, on purpose:

| Components | Why |
| --- | --- |
| Checkbox, Switch, Toggle, Swap, Slider, Number Input, Input, Textarea, Native Select, Resizable | One element holds the state, as with native form controls; there are no parts to share it with |
| Menu checkbox items (Dropdown, Context Menu, Menubar) | Each item is its own state, as a Checkbox is |
| Date Picker's date | The app builds the Calendar inside from it, as with Calendar days |
| Breadcrumb, Pagination, Sidebar, Dock, Menu items, Navigation Menu links, Data Table rows, Item, Calendar days | The current item follows the app's route or data, which the component cannot know |
| Chart, Progress, Radial Progress | They display a value the app owns |

A component keeps a per-part state prop only if this table says so; M210.3
ends when no other crate module takes one.

## Alternatives

| Option | Why not |
| --- | --- |
| Keep every component controlled | Every app repeats the wiring, and wiring mistakes render without an error |
| Pass a `Signal` the app owns to the root | Still makes the app create state for components it never reads, and does not cover a plain value |
| Separate controlled and uncontrolled components | Doubles the components for one difference in one prop |
| Render parts outside their root as before | A part that silently ignores its state is the hidden failure this release removes |

## Impact

- Breaking for every app that uses a moved component: per-part state props
  and hand-matched ids go away, and parts must sit inside their root. Each
  batch lists its changes in the 0.5.0 Migration section.
- Copy mode gains a `controllable` helper.
- The site, blocks, and fixtures use the new API.

## Validation

M210.1 is done when the Select and Tabs doc examples have no `anchor_id`, no
per-item `selected` or `active`, and one `open`; the existing browser checks
for both pass unchanged in behavior; and two server renders of the same app
write the same ids.
