# Sidebar

Sidebar provides controlled app shell and navigation composition parts. It does
not own routing, persistence, or keyboard shortcuts.

## Source Copy

```bash
dxui add sidebar
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["sidebar"] }
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

## Accessibility Notes

Collapsed state, side, active item state, and disabled state are exposed through
attributes. Use links or buttons inside sidebar items according to app routing
needs. Persistence and keyboard shortcuts remain app-owned.
