# Dock

Dock is a bottom navigation bar for phone layouts, with an icon and a label
per destination.

## Source Copy

```bash
dxui add dock
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.5", default-features = false, features = ["dock"] }
```

## API Surface

- `Dock`
- `DockItem`
- `DockLabel`
- `dock_class`
- `dock_item_class`
- `dock_label_class`

```rust
rsx! {
  Dock { "aria-label": "Main",
    DockItem { href: "/", active: true, HomeIcon {}, DockLabel { "Home" } }
    DockItem { href: "/search", SearchIcon {}, DockLabel { "Search" } }
    DockItem { href: "/profile", UserIcon {}, DockLabel { "Profile" } }
  }
}
```

The Dock is fixed to the bottom of the viewport and padded by
`env(safe-area-inset-bottom)`, so it clears the home indicator; leave room
for it at the bottom of the page. `fixed: false` keeps it in the flow.
`DockItem` renders a link with `href` and a button without; `active` marks
the current destination with a primary bar. With the router, apply
`dock_item_class(active, "")` to a `Link`.

## Accessibility Notes

Dock is a `nav` landmark; name it with `aria-label`. The active item sets
`aria-current="page"`, and each item's `DockLabel` gives it a visible name, so
keep the labels even when the icons are familiar (see
[RFC 0061](../rfcs/0061-mobile-navigation.md)).
