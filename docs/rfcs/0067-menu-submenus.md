# RFC 0067: Menu Submenus

- Status: Accepted
- Created: 2026-10-06

## Summary

Add `*Sub`, `*SubTrigger`, and `*SubContent` parts to Dropdown, Context Menu,
and Menubar: a menu item that opens a nested menu beside it. The shared
listbox script learns which items belong to which menu, so the nested menu
handles its own keys and the outer menu ignores them.

## Current State

The release notes say menu submenus are not implemented; shadcn/ui's
Dropdown Menu, Context Menu, and Menubar all have them. Menus run the listbox
script ([RFC 0014](0014-menu-keyboard-behavior.md)), which reads every
`menuitem` under the menu element, so a nested menu's items would join the
outer menu's arrow order and every key would be handled twice.

## Decision

### Parts

```rust
let mut share = use_signal(|| false);

rsx! {
  DropdownSub { on_open_change: move |open| share.set(open),
    DropdownSubTrigger { open: share(), "Share" }
    DropdownSubContent { open: share(),
      DropdownItem { onclick: move |_| copy_link(), "Copy link" }
    }
  }
}
```

- `*Sub` keeps `on_open_change` and an id for its trigger and content in
  context, as `HoverCard` does ([RFC 0023](0023-hover-card-hover-and-focus-opening.md)),
  and renders a `role="group"` wrapper. A nested `menu` may not be a direct
  child of a `menu`; a group may hold it.
- `*SubTrigger` is a `menuitem` with `aria-haspopup="menu"`,
  `aria-expanded`, and `aria-controls`, a chevron at its end (mirrored in
  right-to-left), and the accent background while open. A click asks to
  open.
- `*SubContent` is a `menu` placed beside the trigger, at its inline end
  with its start edges aligned, flipping when there is no room. It runs its
  own listbox script, anchored to the trigger.

`open` stays controlled by the app, passed to the trigger and the content.

### Ownership

An item belongs to the menu whose closest `[data-dxui-listbox]` ancestor is
that menu. Arrow keys, Home, End, typeahead, and pointer highlighting use
only the menu's own items, and a menu ignores key events from inside a
nested menu. A click on any item inside the menu, nested or not, still
closes it, so choosing a nested item closes every level.

### Keys and pointer

- ArrowRight (ArrowLeft in right-to-left), Enter, or Space on a sub trigger
  opens its submenu and focuses the first item; on an open one, the arrow
  focuses the first item.
- ArrowLeft (ArrowRight in right-to-left) or Escape inside a submenu closes
  that level and returns focus to its trigger; the outer menu stays open.
- Hovering a sub trigger opens its submenu without moving focus into it, so
  the pointer can travel to it. Highlighting another item of the outer menu
  moves focus there, which closes the submenu through the existing
  focus-outside dismissal.
- Closing a menu also asks its open submenus to close, so a menu reopens
  with them closed.

Moving the pointer diagonally across sibling items closes the submenu;
Radix's pointer grace area is not included.

## Alternatives

- **Script-only open state.** The script could show and hide submenus
  itself, but every other overlay here is controlled, and the app could not
  read or set the state.
- **Render submenus outside the menu.** That is a DOM portal
  ([RFC 0010](0010-overlay-interaction-behavior.md) chose none) and would
  break the containment the dismissal checks rely on.

## Verification

- SSR tests for the trigger and content attributes.
- A runtime check for the Dropdown: keys open, move within, and close one
  level; hover opens without focus; choosing a nested item closes both; a
  click on the outer menu's items still works; the closed menu reopens with
  the submenu closed.
- Context Menu and Menubar runtime checks for opening and choosing.
- The existing menu, Select, Combobox, and Command runtime checks and the
  Desktop scenarios still pass, since they share the script.
