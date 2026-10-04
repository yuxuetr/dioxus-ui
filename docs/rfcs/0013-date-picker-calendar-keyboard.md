# RFC 0013: Date Picker and Calendar Keyboard Behavior

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Calendar days keyboard navigation and selection hooks, and give Date
Picker real popup behavior: anchored placement next to the trigger, focus entry
on the focused day, Tab containment, Escape and outside dismissal, and focus
return to the trigger.

## Current State

As of M137:

- `CalendarDay` renders a `gridcell` button with state attributes but no
  click, keyboard, or focus handling. Every day button is in the Tab order.
- `CalendarNavButton` has no click handler.
- `calendar_move_date` and `CalendarKeyMove` exist in `dioxus-ui-primitives`,
  but nothing maps keys to them.
- `DatePickerTrigger` and `DatePickerContent` render `open` state only, with
  static `data-side` and `data-align` attributes and no anchor, focus entry, or
  dismissal.
- RFC 0010 deferred Date Picker until Popover anchored placement passed the
  browser smoke, which it did in M135.

## Decision

### Calendar

The visible month, the focused date, and the selected date stay controlled by
the app. Calendar parts gain optional props:

- `CalendarDay`:
  - `focused: bool`. When `on_key_move` is set, the focused day gets
    `tabindex="0"` and the others `tabindex="-1"`, so the grid is one Tab stop
    (roving tabindex). After a day handles a navigation key, the day that
    becomes focused takes DOM focus through `MountedData::set_focus`, so
    moving the focused date with the keyboard moves focus without page
    JavaScript. This works whether Dioxus reuses the day element or mounts a
    new one for days keyed by date. Changing `focused` without a key press
    does not move focus, so an always-visible calendar never steals focus.
  - `on_key_move: Option<EventHandler<CalendarKeyMove>>`. ArrowLeft and
    ArrowRight move by a day, ArrowUp and ArrowDown by a week, Page Up and Page
    Down by a month (by a year with Shift), and Home and End to the start and
    end of the week. Handled keys prevent the default scroll.
  - `on_select: Option<EventHandler<CalendarDate>>`, called on click. Enter
    and Space trigger the native button click.
- `CalendarNavButton` gains `onclick`.
- `calendar_key_move(key, shift) -> Option<CalendarKeyMove>` exposes the key
  mapping.

The focused day also renders `data-dxui-autofocus`, which a focus scope uses
for focus entry (see below).

The app applies the move with `calendar_move_date` and updates the visible
month when the new date falls outside it. Source-copy templates do not carry
date arithmetic, matching the current template where apps build the month grid
themselves; the template gains `CalendarKeyMove` and `calendar_key_move` only.

### Date Picker

- `DatePickerTrigger` gains `id` and `on_open_change`; click requests `!open`.
- `DatePickerContent` gains the RFC 0010 anchoring props (`anchor_id`,
  existing `side` and `align`, `side_offset` default 4), `dismiss` (default
  `popover_default()`), and `on_open_change`. It runs the anchored overlay
  script for placement and Escape or outside dismissal, and the modal focus
  scope for focus entry, Tab wrap, and focus return.

The content stays a `dialog`. Selecting a date is app code in `on_select`:
store the value and request close.

### Focus Scope Adjustments

Three changes to the shared focus scope script, which also apply to Dialog,
Alert Dialog, Sheet, and Drawer:

1. Focus entry prefers an element with `data-dxui-autofocus` over the first
   focusable element.
2. Tab wrapping ignores `tabindex="-1"` elements. Before this change, the last
   focusable element in a roving grid was an unreachable day, so Tab could
   leave the scope.
3. Focus returns to the previously focused element only when focus is on the
   document body or inside the scope at close. When an outside click moved
   focus to another control, focus stays there.

## Scope

In scope:

- Calendar day keyboard movement, roving tabindex, focus following, selection
  and navigation callbacks
- Date Picker placement, focus entry, Tab wrap, dismissal, and focus return
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Date arithmetic in source-copy templates | Templates already leave month grid generation to the app | A template consumer reports needing `calendar_move_date` |
| Typed date parsing | Needs locale formats and validation policy | A consumer needs a text input Date Picker |
| Range selection keyboard flow | Range anchors and hover preview need extra state | A consumer needs keyboard range selection |
| Right-to-left arrow mirroring | Arrow keys map to visual left and right only in LTR | A consumer reports RTL calendar navigation |
| Disabled day skipping | Moving onto a disabled date keeps focus on a disabled button | A consumer reports focus loss on disabled dates |

## Verification

- A unit test covers the key mapping.
- The Web preview renders a real Date Picker with a Calendar, and
  `npm run verify:runtime-interactions` asserts placement, focus entry, key
  movement across months, Tab wrap, selection with focus return, Escape, and
  outside dismissal.
- Desktop and Mobile share the same code paths but are not covered by an
  automated gate.
