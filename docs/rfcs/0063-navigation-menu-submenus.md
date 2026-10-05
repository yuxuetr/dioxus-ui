# RFC 0063: Navigation Menu Submenus

- Status: Accepted
- Created: 2026-10-05

## Summary

Nest a vertical `NavigationMenu` inside `NavigationMenuContent` for a
submenu: a column of triggers whose panels open beside it, as in a mega
menu. Each menu's script acts only on its own items, and the vertical
orientation changes the arrow keys and removes the hover delay.

## Current State

The 0.1.0 release notes exclude Navigation Menu submenus
([RFC 0016](0016-navigation-menu-interaction.md) covers one level). The
script finds triggers, contents, and links anywhere under its root, so a
nested menu's trigger would also be read as the outer menu's: a click on it
would send the inner item's value to the outer menu and open nothing.

## Decision

### Ownership

An element belongs to the menu whose root is its closest
`[data-dxui-navigation-menu]` ancestor. Every lookup in the script, triggers,
items, contents, links, and the open item, keeps only the elements its own
root owns. The inner menu handles keys inside it first and prevents their
default, and the outer menu already skips prevented events. A click on a
link anywhere in the content still closes the outer menu, since following a
link leaves the page.

### Orientation

`NavigationMenu` takes `orientation: NavigationMenuOrientation`,
`Horizontal` by default, and sets `data-orientation`. A vertical menu:

- lays its list out as a column and places each item's content beside the
  list, through the `navigation-menu` group, instead of below it;
- moves between triggers with ArrowDown and ArrowUp, enters content with
  ArrowRight (ArrowLeft in right-to-left), and returns to the trigger from
  content with ArrowLeft;
- opens on hover without the 200 ms delay, since the pointer is already in
  the outer menu.

Escape closes every menu and returns focus to the outer trigger.

### Values

The inner menu is controlled like the outer one: it has its own `value` and
`on_value_change`. Apps usually open its first item when the outer item
opens, so the panel is not empty.

## Verification

- SSR tests for `data-orientation` and the vertical classes.
- A runtime check that hovering and the arrow keys switch the inner panels,
  a click on an inner trigger does not change the outer value, and Escape
  closes both.
- The Desktop navigation menu scenario still passes.
- A site example of a mega menu, audited in both themes and every preset.
