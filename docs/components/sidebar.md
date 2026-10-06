# Sidebar

Sidebar provides controlled app shell and navigation composition parts, an
opt-in off-canvas panel for phones, and an opt-in keyboard shortcut. It does
not own routing or persistence.

## Source Copy

```bash
dxui add sidebar
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.3", default-features = false, features = ["sidebar"] }
```

## API Surface

- `Sidebar`
- `SidebarRail`
- `SidebarHeader`
- `SidebarContent`
- `SidebarFooter`
- `SidebarGroup`
- `SidebarGroupLabel`
- `SidebarItem`
- `SidebarTrigger`
- `SidebarSide`
- `SidebarState`
- `sidebar_toggle`
- `SIDEBAR_MOBILE_QUERY`
- `sidebar_mobile_panel_class`, `sidebar_mobile_class`, `sidebar_overlay_class`

## Toggle And Items

`SidebarTrigger` reports the requested collapsed state, and `SidebarItem`
renders a link or a button that marks the current page:

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

- The trigger calls `on_collapsed_change` with `!collapsed`. A disabled
  trigger reports nothing.
- An item with an `href` renders an anchor for router or server pages. An
  item with an `onclick` and no `href` renders a button; use it for in-app
  sections, since Desktop opens anchors in the system browser. An item with
  neither stays a wrapper for a link or button you render inside.
- Anchors and buttons render `aria-current="page"` when `active`. A disabled
  button uses the native `disabled`, and a disabled anchor drops its `href`,
  so neither takes focus or calls `onclick`.
- Global and element attributes pass through every part except the rail, so
  the sidebar can take an `id` and an `aria-label` and the trigger
  `aria-controls`.

## Off-Canvas And Shortcut

Pass `on_mobile_open_change` to make the sidebar off-canvas below
`SIDEBAR_MOBILE_QUERY` (768px), and `shortcut` for Ctrl or Command and a key
(see [RFC 0069](../rfcs/0069-off-canvas-sidebar.md)):

```rust
let mut collapsed = use_signal(|| false);
let mut mobile_open = use_signal(|| false);

rsx! {
  SidebarTrigger {
    "aria-controls": "app-sidebar",
    collapsed: collapsed(),
    on_collapsed_change: move |next| collapsed.set(next),
    mobile_open: mobile_open(),
    on_mobile_open_change: move |next| mobile_open.set(next),
    "Toggle sidebar"
  }
  Sidebar {
    id: "app-sidebar",
    "aria-label": "Main",
    collapsed: collapsed(),
    on_collapsed_change: move |next| collapsed.set(next),
    mobile_open: mobile_open(),
    on_mobile_open_change: move |next| mobile_open.set(next),
    shortcut: 'b',
    SidebarContent { /* items */ }
  }
}
```

- Below the breakpoint the sidebar is a modal `dialog` panel at its side,
  hidden unless `mobile_open`, named by the sidebar's `aria-label`. It traps
  focus, locks page scroll, and asks to close on Escape or a press on the
  overlay; closing returns focus to the trigger. Close it from items too,
  since choosing a page usually should.
- The trigger toggles `mobile_open` below the breakpoint and `collapsed`
  above it; `aria-expanded` follows whichever it toggles.
- `shortcut` toggles the panel off-canvas and `collapsed` otherwise, through
  the Sidebar's `on_collapsed_change`.
- The viewport is read after the first render, so a phone renders the wide
  layout for a frame first, as server-side rendering does.
- Without `on_mobile_open_change` the sidebar renders the same `aside` at
  every width.

## Accessibility Notes

Collapsed state, side, active item state, and disabled state are exposed through
attributes, and link and button items mark the current page with
`aria-current`. Name the `aside` landmark with `aria-label`; the off-canvas
dialog takes the same name. Persistence remains app-owned.
