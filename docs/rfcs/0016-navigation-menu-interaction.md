# RFC 0016: Navigation Menu Interaction Behavior

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Navigation Menu the WAI-ARIA disclosure navigation pattern, plus hover
opening as in shadcn/ui:

- Triggers are buttons that toggle their content.
- Every trigger and link stays in the Tab order.
- Arrow keys move between top-level items and between the links of open
  content.
- Hovering opens content after a delay, and leaving closes it.
- Escape, outside presses, and focus leaving the menu close it.

## Current State

As of M140:

- `NavigationMenuTrigger` and `NavigationMenuContent` render `open` state
  only. They have no click, keyboard, hover, or dismissal handling.
- RFC 0010 deferred Navigation Menu because its content is laid out with CSS
  below the list rather than anchored to a trigger. That reason still holds
  for placement. Interaction behavior was never designed.
- Navigation Menu is a navigation landmark of links, not an application menu.
  It must not use `menu` roles or move every item out of the Tab order, unlike
  Menubar (RFC 0015).

## Decision

### State

The app keeps the value of the open item, empty when none is open. It passes
`open` to each trigger, content, and optional viewport or indicator. It
handles one request: `NavigationMenu` `on_value_change(String)`, with the item
value to open, or an empty string to close. `NavigationMenuItem` gains a
`value` prop that names the item. Triggers need no handlers of their own.

### Navigation Menu Script

`NavigationMenu` runs one page script for its lifetime, like Menubar. Items,
triggers, content, and links are read from the DOM on every event.

Pointer:

- A click on a trigger requests its value, or an empty string when its content
  is open. Enter and Space click the button natively, so they use the same
  path.
- When the mouse moves onto a trigger, its content opens after 200 ms. If
  another item is already open, it opens immediately. The script listens for
  pointer movement rather than `pointerover`, because Chrome sends
  `pointerover` when the layout shifts under a resting cursor (found in M140).
- When the mouse moves off the open item's trigger and content, the content
  closes after 300 ms, unless the mouse comes back first.
- A trigger closed by a click does not reopen on hover until the mouse leaves
  it.
- Touch and pen pointers use clicks only.

Keyboard:

- Left and Right move focus between top-level items and wrap. Top-level
  items are triggers and links outside content. Home and End jump to the
  first and last top-level item.
- ArrowDown on a trigger focuses the first link of its content, opening the
  content first when it is closed.
- Inside content, ArrowDown and ArrowUp move between its links and wrap.
  Home and End jump to the first and last link.
- Tab keeps the document order. Content follows its trigger in the DOM, so Tab
  from an open trigger enters its links.

Dismissal:

- Escape while content is open closes it. If focus was inside the menu, focus
  moves to the trigger of the closed content.
- A pointer press outside the menu closes open content.
- Focus moving outside the menu closes open content.

Disabled triggers and links with `aria-disabled="true"` are skipped.

## Scope

In scope:

- click, keyboard, and hover opening
- top-level and content keyboard movement
- Escape, outside, and focus-out dismissal, with focus return
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Viewport size measurement | `NavigationMenuViewport` keeps its CSS variables app-set; content renders in its item | A consumer needs content rendered inside a shared animated viewport |
| Anchored placement | Content is laid out with CSS below the list (RFC 0010) | A consumer reports Navigation Menu content clipping |
| Submenus | No nested list parts exist | Nested navigation parts are added |
| Open and close motion | No motion tokens exist for overlays | Overlay motion is designed |
| Right-to-left arrow mirroring | Left and Right follow visual order only in LTR | A consumer reports RTL navigation menu movement |
| Closing when the pointer leaves the window | The script only sees movement inside the page | A consumer reports content staying open after the pointer leaves the window |

## Verification

- A CLI parity test keeps the template script identical to the crate script.
- The Web preview renders a real Navigation Menu, and
  `npm run verify:runtime-interactions` asserts:
  - click toggling;
  - top-level arrow movement that skips a disabled trigger;
  - ArrowDown content entry and link movement;
  - Escape with focus return;
  - hover open delay, switching, and close after leaving;
  - outside press and focus-out dismissal.
- Desktop and Mobile share the `document::eval` path but are not covered by an
  automated gate.
