# Sidebar

Sidebar provides controlled app shell and navigation composition parts. It does
not own routing, persistence, or keyboard shortcuts.

## Source Copy

```bash
dxui add sidebar
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["sidebar"] }
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

## Accessibility Notes

Collapsed state, side, active item state, and disabled state are exposed through
attributes, and link and button items mark the current page with
`aria-current`. Name the `aside` landmark with `aria-label`. Persistence and
keyboard shortcuts remain app-owned.
