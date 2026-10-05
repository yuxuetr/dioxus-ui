# RFC 0061: Mobile Navigation

- Status: Accepted
- Created: 2026-10-05

## Summary

Add Dock, a bottom navigation bar, and Fab, a floating action button that
can open a speed dial of actions: the daisyUI components for one-handed
phone layouts, which Sidebar and Navigation Menu do not cover.

## Current State

The Mobile examples run the same components as Web and Desktop
([RFC 0018](0018-mobile-interaction-verification.md)), but navigation is Sidebar
and Navigation Menu, both built for wide screens. A phone app builds its
bottom tab bar and its primary action button from raw markup.

## Decision

### Dock

`Dock` is a `nav` fixed to the bottom of the viewport, padded by
`env(safe-area-inset-bottom)` for phones with a home indicator. `fixed:
false` keeps it in the flow, for a preview frame. Name it with `aria-label`.
`DockItem` renders a link with `href` and a button without, takes an icon and
a `DockLabel` as children, and with `active` sets `aria-current="page"` and
shows a primary bar above the icon. Apps using the router's `Link` apply
`dock_item_class(active, class)` to it.

### Fab

`Fab` is a button fixed to the bottom inline-end corner. Without actions it
is a plain button: `onclick` runs the primary action, and `aria-label` names
it. With `FabAction` children it is a speed dial: the trigger sets
`aria-expanded` and `aria-controls`, `open` shows the actions stacked above
it, and a press calls `on_open_change`. Escape closes it and returns focus to
the trigger. The actions render only while open, so closed actions are not
in the tab order. `FabAction` is a button with a visible label beside a round
icon, so it needs no `aria-label`; the app closes the dial in the action's
`onclick`. `fixed: false` keeps it in the flow.

## Alternatives

- **Dock items only as buttons.** Bottom navigation usually changes the
  route; a link keeps open-in-new-tab and the URL on Web.
- **Rendering closed actions hidden with CSS.** They would stay in the tab
  order and the accessibility tree unless each one were also made inert.

## Verification

- SSR tests for both components' semantics.
- Site examples with `fixed: false`, audited in both themes and every preset.
- A runtime check that the Fab trigger opens and closes the dial, Escape
  returns focus, and an action runs.
