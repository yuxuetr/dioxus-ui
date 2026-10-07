# Sidebar

Sidebar provides app shell and navigation composition parts under a
`SidebarProvider` that owns the collapsed and mobile-open state, an opt-in
off-canvas panel for phones, and an opt-in keyboard shortcut. It does
not own routing or persistence.

## Source Copy

```bash
dxui add sidebar
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["sidebar"] }
```

## API Surface

- `SidebarProvider`
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

`SidebarProvider` owns whether the sidebar is collapsed
([RFC 0077](../rfcs/0077-component-owned-state.md)) and shares it with
`Sidebar`, `SidebarRail`, and `SidebarTrigger`, which may sit anywhere inside
it, such as the sidebar beside the page and the trigger in its header. It
renders no element of its own. `SidebarItem` renders a link or a button that
marks the current page:

```rust
let mut section = use_signal(|| "inbox");

rsx! {
  SidebarProvider {
    SidebarTrigger { "Toggle sidebar" }
    Sidebar { "aria-label": "Main",
      SidebarContent {
        SidebarItem { active: section() == "inbox", onclick: move |_| section.set("inbox"), "Inbox" }
        SidebarItem { href: "/settings", "Settings" }
        SidebarItem { disabled: true, onclick: move |_| {}, "Archive" }
      }
    }
  }
}
```

- The provider starts expanded, or collapsed with `default_collapsed: true`;
  pass `collapsed` to control it, for example to hide labels while
  collapsed. `on_collapsed_change` hears every change the user makes.
- The trigger toggles it and points `aria-controls` at the sidebar, whose id
  the provider generates. When you give the sidebar an `id`, give the
  trigger the same `aria-controls`. A disabled trigger does nothing.
- `Sidebar`, `SidebarRail`, and `SidebarTrigger` must be inside
  `SidebarProvider`; otherwise they render nothing and log which root they
  are missing.
- An item with an `href` renders an anchor for router or server pages. An
  item with an `onclick` and no `href` renders a button; use it for in-app
  sections, since Desktop opens anchors in the system browser. An item with
  neither stays a wrapper for a link or button you render inside.
- Anchors and buttons render `aria-current="page"` when `active`. A disabled
  button uses the native `disabled`, and a disabled anchor drops its `href`,
  so neither takes focus or calls `onclick`.
- Global and element attributes pass through every part except the rail, so
  the sidebar can take an `aria-label`.

## Off-Canvas And Shortcut

Set `off_canvas` on the provider to make the sidebar off-canvas below
`SIDEBAR_MOBILE_QUERY` (768px), and pass `shortcut` to the sidebar for Ctrl or
Command and a key (see [RFC 0069](../rfcs/0069-off-canvas-sidebar.md)):

```rust
let mut mobile_open = use_signal(|| false);

rsx! {
  SidebarProvider {
    off_canvas: true,
    mobile_open: mobile_open(),
    on_mobile_open_change: move |next| mobile_open.set(next),
    SidebarTrigger { "Toggle sidebar" }
    Sidebar { "aria-label": "Main", shortcut: 'b',
      SidebarContent {
        SidebarItem { onclick: move |_| mobile_open.set(false), "Inbox" }
      }
    }
  }
}
```

- Below the breakpoint the sidebar is a modal `dialog` panel at its side,
  hidden until opened, named by the sidebar's `aria-label`. It traps
  focus, locks page scroll, and asks to close on Escape or a press on the
  overlay; closing returns focus to the trigger. To close it from items
  too, since choosing a page usually should, control `mobile_open` as above.
- The trigger toggles the panel below the breakpoint and the collapsed state
  above it; `aria-expanded` follows whichever it toggles. `shortcut` does
  the same.
- The viewport is read after the first render, so a phone renders the wide
  layout for a frame first, as server-side rendering does.
- Without `off_canvas` the sidebar renders the same `aside` at every width.

## Accessibility Notes

Collapsed state, side, active item state, and disabled state are exposed through
attributes, and link and button items mark the current page with
`aria-current`. Name the `aside` landmark with `aria-label`; the off-canvas
dialog takes the same name. Persistence remains app-owned.
