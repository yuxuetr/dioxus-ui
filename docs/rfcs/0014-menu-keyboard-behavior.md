# RFC 0014: Menu Keyboard Behavior

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Dropdown and Context Menu the menu keyboard pattern: opening focuses the
first item, arrows move focus with wrapping, Home, End, and typeahead jump,
Enter, Space, or click activates an item and closes the menu, and focus
returns to where it was before opening. Context Menu gains a point anchor so
it opens at the pointer.

## Current State

As of M138:

- `DropdownContent` is anchored and dismissable (RFC 0010), but its items are
  not focusable, have no click handler, and the menu has no keyboard handling.
- `ContextMenuContent` renders `open` state only, with no placement and no
  dismissal. RFC 0010 deferred it because it anchors to a pointer position.
- Context Menu items, including checkbox and radio items, have no click
  handler.
- The listbox script from RFC 0012 already enumerates enabled items, runs
  typeahead, and reports choices, but it keeps focus on an anchor through
  `aria-activedescendant`, which is the listbox pattern, not the menu pattern.

## Decision

### Menu Mode

The listbox script gains a third mode, `menu`, instead of a second script.
Item enumeration, disabled filtering, typeahead, pointer highlighting, and the
Rust hook are shared; the menu mode differs where the WAI-ARIA menu pattern
differs from the listbox pattern:

- Items are `menuitem`, `menuitemcheckbox`, or `menuitemradio` elements. The
  script gives them `tabindex="-1"` and moves DOM focus to the highlighted
  item instead of setting `aria-activedescendant`.
- Keys are read from the menu itself, since focus is inside it.
- Opening focuses the first enabled item. ArrowDown and ArrowUp wrap at the
  ends. Home, End, and typeahead work as in Select.
- Enter and Space click the focused item. A click on an enabled item, from the
  pointer or the keyboard, runs the item's `onclick` and requests close.
- When the menu closes, focus returns to the element focused before it opened,
  unless focus already moved to another control (the RFC 0013 rule).
- Tab is not handled. Moving focus out of the menu triggers the anchored
  overlay's focus-outside dismissal.

Menu items gain `onclick: Option<EventHandler<MouseEvent>>`, which is not
called for disabled items.

### Context Menu Point Anchor

`AnchoredPlacement` gains `anchor_point: Option<(f64, f64)>`, viewport
coordinates used when no `anchor_id` is given. The anchored overlay script
measures a zero-size rectangle at that point and applies the same flip and
shift rules. `ContextMenuContent` gains `anchor_point`, `side` (default
Bottom), `align` (default Start), `side_offset` (default 0), `dismiss`
(default `popover_default()`), and `on_open_change`, so the menu's top-left
corner sits at the pointer and flips when there is no room.

The app opens the menu from `oncontextmenu`: prevent the default menu, store
`event.client_coordinates()`, and set `open`. A keyboard context menu key
fires the same event with coordinates at the focused element.

## Scope

In scope:

- Dropdown and Context Menu focus entry, arrow, Home, End, and typeahead
  navigation, activation, Escape and outside dismissal, and focus return
- Context Menu placement at a point
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Submenus | No submenu parts exist yet | Submenu parts are added |
| Menubar cross-menu navigation | Needs left and right movement between menus and trigger roving focus | Planned as the next milestone |
| Checkbox and radio state | State stays app-controlled through `checked` and the new `onclick` | A consumer needs uncontrolled menu item state |
| Keeping a menu open after activation | All enabled items close the menu | A consumer needs a multi-toggle menu |
| Opening a Dropdown on ArrowUp with the last item focused | Dropdown has no trigger part; the app owns the trigger | A Dropdown trigger part is added |

## Verification

- A CLI parity test keeps the template scripts identical to the crate scripts.
- The Web preview renders real Dropdown and Context Menu components, and
  `npm run verify:runtime-interactions` asserts focus entry, wrapping
  navigation that skips disabled items, typeahead, activation with close and
  focus return, Escape, Tab, and point placement.
- Desktop and Mobile share the `document::eval` path but are not covered by an
  automated gate.
