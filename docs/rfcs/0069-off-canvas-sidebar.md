# RFC 0069: Off-Canvas Sidebar and Shortcut

- Status: Accepted
- Created: 2026-10-06

## Summary

Below 768px a Sidebar can become an off-canvas panel that opens over the page
as a modal, and an opt-in keyboard shortcut toggles the sidebar. Both follow
the viewport through a small media query helper.

## Current State

The release notes for Sidebar ([RFC 0037](0037-sidebar-toggle-and-items.md))
exclude keyboard shortcuts and mobile off-canvas behavior. On a phone the
sidebar keeps its width, or the app hides it and builds a Sheet with a
second copy of the navigation. shadcn/ui's Sidebar renders as a sheet on
mobile and toggles with Ctrl or Command and B.

## Decision

### Viewport

`use_media_query(query, enabled)` runs a script that reports whether
`matchMedia(query)` matches, and again on each change, until its element is
removed. It answers after the first render, so a phone first renders the
desktop layout for a frame, as does server-side rendering.
`SIDEBAR_MOBILE_QUERY` is `(max-width: 767px)`, shadcn/ui's breakpoint.

### Off-canvas

Off-canvas is opt-in: a Sidebar without `on_mobile_open_change` renders the
same `aside` as before at every width. With it, the `aside` sits in a wrapper
that is `display: contents` on wide viewports and, below the breakpoint:

- is a `dialog` with `aria-modal`, named by the `aside`, fixed to the side,
  and hidden unless `mobile_open`;
- traps focus, locks page scroll ([RFC 0068](0068-modal-scroll-lock.md)),
  and returns focus on close, through the modal focus scope;
- asks to close on Escape or a press on the overlay rendered next to it.

`collapsed` does not apply off-canvas; the panel has its full width.
`SidebarTrigger` with `on_mobile_open_change` toggles `mobile_open` below the
breakpoint and `collapsed` above it, and its `aria-expanded` follows the one
it toggles. A `dialog` role is not allowed on an `aside`, hence the wrapper.

### Shortcut

`shortcut: Some('b')` makes Ctrl or Command and B toggle the sidebar: the
off-canvas panel when it is off-canvas, otherwise `collapsed` through the
Sidebar's new `on_collapsed_change`. The key is not combined with Alt or
Shift, and the browser's default for it is prevented.

## Alternatives

- **Document a Sheet composition.** It needs the navigation rendered twice,
  once per layout, and two triggers.
- **CSS-only off-canvas.** A translated panel stays in the tab order and
  cannot trap focus or lock scroll without knowing the viewport.

## Verification

- SSR tests: an off-canvas Sidebar renders the desktop `aside` before the
  page answers, and the panel classes per side.
- A runtime check of the fixture at 1280px (trigger and Ctrl+B collapse) and
  at 375px (closed panel hidden, trigger opens a named modal dialog at the
  left edge with focus inside and scroll locked, Escape and the overlay
  close it and return focus, Ctrl+B toggles it), with the axe audit open.
  Reverse-verified with a trigger that ignores the viewport.
