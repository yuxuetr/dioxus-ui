# RFC 0015: Menubar Keyboard Behavior

- Status: Accepted
- Created: 2026-10-04

## Summary

Give Menubar the WAI-ARIA menubar pattern. The triggers form one Tab stop
with Left, Right, Home, and End movement. ArrowDown, Enter, Space, or a click
opens a menu, which then uses the menu mode from RFC 0014. While a menu is
open, Left and Right open the adjacent menu, and hovering another trigger
switches to its menu. Closing a menu returns focus to its trigger.

## Current State

As of M139:

- `MenubarTrigger` and `MenubarContent` render `open` state only. Content has
  no placement, no dismissal, and no focus handling, and items have no click
  handler.
- Every trigger is a separate Tab stop, and arrow keys do nothing.
- RFC 0010 deferred Menubar until cross-menu roving focus existed.
- The listbox script's menu mode (RFC 0014) already handles focus entry,
  vertical movement, typeahead, activation, and focus return within one menu.
  It leaves Left and Right unhandled.

## Decision

### State

Open state stays app-controlled, like every other part in the library. The
app keeps the open menu's value. It passes `open` to each trigger and its
content, and it handles three requests:

- trigger `on_open_change(bool)`, from a click or ArrowDown;
- content `on_open_change(false)`, from activation, Escape, or an outside
  interaction;
- `Menubar` `on_value_change(String)`, from Left, Right, or hover switching.
  The string is the `value` of the `MenubarMenu` to open.

### Menubar Script

`Menubar` runs one page script for the whole bar. It starts when the bar
mounts and ends when the bar is removed. Triggers are read from the DOM on
every event, and disabled triggers are skipped. The script:

- gives the trigger that last had focus `tabindex="0"` and all other triggers
  `tabindex="-1"`, starting with the first enabled trigger, so the bar is one
  Tab stop;
- on a focused trigger, moves focus with Left and Right (wrapping), Home, and
  End;
- inside an open menu, sends the adjacent menu's value on Left and Right,
  wrapping. Keys that the menu mode already handled are ignored;
- while a menu is open, sends the hovered trigger's menu value when the
  pointer moves over another enabled trigger.

### Parts

- `Menubar` gains `on_value_change` and renders `data-dxui-menubar`.
- `MenubarMenu` gains `value`, which marks the menu for the script.
- `MenubarTrigger` gains `id` and `on_open_change`. A click requests `!open`,
  and ArrowDown on a closed trigger requests `true`.
- `MenubarContent` gains the props `DropdownContent` has: `anchor_id`, `side`
  (default Bottom), `align` (default Start), `side_offset` (default 4),
  `dismiss` (default `popover_default()`), and `on_open_change`. It runs the
  anchored overlay and the listbox menu mode.
- `MenubarItem`, `MenubarCheckboxItem`, and `MenubarRadioItem` gain `onclick`,
  which is not called for disabled items.

### Focus Return

In menu mode, the listbox script now returns focus to the anchor when one is
given. Without an anchor, it returns focus to the element focused before
opening.

The change is needed because of how switching works. When Right switches
menus, the old menu restores focus while the new one opens. The element
focused before the new menu opened is the old menu's trigger, but Escape must
return to the new menu's trigger. A Dropdown's anchor is normally the element
that opened it, so this does not change Dropdown behavior. Context Menu has
no anchor and keeps the previous rule.

## Scope

In scope:

- trigger roving focus
- opening from the keyboard and the pointer
- adjacent-menu switching with Left and Right
- hover switching
- activation, Escape, outside dismissal, and focus return
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Submenus | No submenu parts exist yet | Submenu parts are added |
| ArrowUp opening with the last item focused | The menu mode always starts on the first item | A consumer needs ArrowUp opening |
| Right-to-left arrow mirroring | Left and Right follow visual order only in LTR | A consumer reports RTL menubar navigation |
| Typeahead across triggers | The WAI-ARIA pattern marks it optional | A consumer needs trigger typeahead |

## Verification

- A CLI parity test keeps the template scripts identical to the crate scripts.
- The Web preview renders a real Menubar. `npm run verify:runtime-interactions`
  asserts:
  - roving focus that skips a disabled trigger;
  - opening with ArrowDown;
  - switching with Left and Right;
  - hover switching;
  - activation with close and focus return to the trigger;
  - Escape and Tab.
- Desktop and Mobile share the `document::eval` path, but no automated gate
  covers them.
