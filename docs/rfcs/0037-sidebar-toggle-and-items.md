# RFC 0037: Sidebar Toggle And Items

- Status: Accepted
- Created: 2026-10-05

## Summary

Make Sidebar operable. `SidebarTrigger` reports the requested collapsed state
through `on_collapsed_change`. `SidebarItem` renders a link when it has an
`href` and a button when it has an `onclick`, marks the active one with
`aria-current="page"`, and makes a disabled one impossible to focus or
activate. The Sidebar parts pass through attributes.

## Current State

As of M161:

- `SidebarTrigger` renders a button with `aria-expanded` but no click
  handler, so pressing it cannot collapse or expand the sidebar.
- The trigger cannot take `aria-controls`, and `Sidebar` cannot take the
  `id` it would point at or an `aria-label` for the landmark.
- `SidebarItem` is a `div`. `active` only sets `data-active`, so assistive
  technology does not learn which item is the current page. `disabled` adds
  `aria-disabled` and `pointer-events-none` to the wrapper, but a link inside
  stays in the Tab order and still follows Enter.
- On Desktop, the Dioxus interpreter opens anchor clicks in the system
  browser, so a link inside an item cannot drive in-app navigation there.

## Decision

### API

```rust
let mut collapsed = use_signal(|| false);
let mut section = use_signal(|| "inbox");

rsx! {
  SidebarTrigger {
    "aria-controls": "app-sidebar",
    collapsed: collapsed(),
    on_collapsed_change: move |next| collapsed.set(next),
    "Toggle sidebar"
  }
  Sidebar { id: "app-sidebar", "aria-label": "Main", collapsed: collapsed(),
    SidebarContent {
      SidebarItem { active: section() == "inbox", onclick: move |_| section.set("inbox"), "Inbox" }
      SidebarItem { href: "/settings", "Settings" }
      SidebarItem { disabled: true, onclick: move |_| {}, "Archive" }
    }
  }
}
```

- `SidebarTrigger` gains `on_collapsed_change: Option<EventHandler<bool>>`,
  called with `!collapsed` on click. A disabled trigger is a native disabled
  button and reports nothing.
- `SidebarItem` gains `href: String` (default empty) and
  `onclick: Option<EventHandler<MouseEvent>>`:
  - a non-empty `href` renders `a href`; a disabled anchor drops `href` and
    keeps `aria-disabled="true"`;
  - otherwise an `onclick` renders `button type="button"`, natively disabled
    when `disabled`;
  - otherwise the item stays a `div` wrapper, as before, for apps that render
    their own link or button inside.
  - Anchors and buttons render `aria-current="page"` when active, and a
    disabled item does not call `onclick`.
- `Sidebar` extends `GlobalAttributes` and `aside`. `SidebarHeader`,
  `SidebarContent`, `SidebarFooter`, `SidebarGroup`, and `SidebarGroupLabel`
  extend `GlobalAttributes` and `div`. `SidebarItem` extends
  `GlobalAttributes` and `a`, and `SidebarTrigger` extends `GlobalAttributes`
  and `button`. The spread comes after the explicit attributes.

## Scope

In scope:

- the trigger callback, the item forms, and attribute spreading in the crate
  and the template
- the Sidebar docs page
- browser verification through `npm run verify:runtime-interactions`

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| A keyboard shortcut such as Ctrl+B or Cmd+B | Global shortcuts belong to the app and can clash with its own | A consumer asks for the shadcn shortcut |
| `SidebarRail` as a toggle button | The rail is decorative and hidden from assistive technology; the trigger already toggles | A consumer needs a pointer target on the edge |
| Hiding item labels while collapsed | Styling the collapsed icon view is app layout | A consumer reports overflowing labels |
| Mobile off-canvas behavior | Sheet already covers a modal side panel | A consumer needs the sidebar to become a sheet on small screens |
| Desktop and Mobile self-test scenarios | The parts use plain click events with no script | They gain script behavior |

## Verification

- The CLI generated fixture smoke keeps the template compiling.
- The Web preview renders a trigger and a Sidebar with button, link,
  disabled, and wrapper items, and `npm run verify:runtime-interactions`
  asserts:
  - a trigger click and Enter toggle `collapsed` and `aria-expanded`, and
    `aria-controls` names the sidebar landmark;
  - a button item click and Space change the active item and move
    `aria-current`;
  - a link item keeps its `href`, and a disabled item cannot take focus and
    ignores a forced press;
  - a wrapper item stays a `div`.
- Reverse checks: removing the trigger callback, removing the item callback,
  always rendering a wrapper, keeping `href` or calling `onclick` on a
  disabled item, or not spreading the attributes each make the verifier fail.
