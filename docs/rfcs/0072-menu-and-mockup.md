# RFC 0072: Menu and Mockup

- Status: Accepted
- Created: 2026-10-06

## Summary

Port two more daisyUI components: Menu, a vertical navigation list with
collapsible groups, and Mockup, frames that show content as a browser, a
window, a terminal, or a phone.

## Current State

0.2.0 ported the daisyUI components that had no shadcn/ui counterpart
([RFC 0059](0059-display-components.md) to
[RFC 0061](0061-mobile-navigation.md)). Of the rest, these two have uses
beyond layout: a docs or settings sidebar without the full `Sidebar`, and
product or documentation pages that show an app inside a frame. Hero,
Footer, Navbar, and List are layouts the app writes with a few classes;
Filter and Validator overlap Toggle Group and Field.

## Decision

### Menu

- `Menu` is a `ul`, without `role="menu"`: it lists links, which assistive
  technology reads as a list and keyboards move through with Tab, unlike
  the app menus of `Dropdown` and `Menubar`.
- `MenuTitle` is a text-only `li` for a section label.
- `MenuItem` renders a link with an `href`, a button with an `onclick`, or
  wraps the app's own content, as `SidebarItem` does
  ([RFC 0037](0037-sidebar-toggle-and-items.md)); `active` sets
  `aria-current="page"`.
- `MenuGroup` takes a `label` element and a controlled `open`; its button
  has `aria-expanded` and `aria-controls` for the nested list, which is
  hidden while closed. The chevron points down when closed and up when open,
  which needs no mirroring in right-to-left. Groups nest.

### Mockup

- `MockupBrowser` frames content with a toolbar of three dots and an
  address field showing `url`; `MockupWindow` frames it with the dots only.
- `MockupCode` is a terminal-styled `pre` of `MockupCodeLine`s, each with an
  optional `prefix` (such as `$`) shown before the line and hidden from
  copying and from assistive technology, and an optional `highlight`.
- `MockupPhone` is a rounded device frame with a camera notch and a display
  area sized like a phone screen.
- The frames are decorative: the dots, notch, and prefixes are
  `aria-hidden`, and the content inside keeps its own semantics.

## Alternatives

- **Native `details` for menu groups.** The open state would live in the
  DOM, unlike every other disclosure here, and the summary's marker differs
  across WebViews.

## Verification

- SSR tests for each part.
- A Menu runtime check: the group's button opens and closes the nested
  list, items move `aria-current`, a disabled item is disabled, and the
  keyboard reaches and toggles the group; reverse-verified with a list that
  stays visible.
- Site examples for both, audited in both themes and every preset.
