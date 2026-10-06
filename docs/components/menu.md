# Menu

Menu is a vertical navigation list: section titles, link or button items
that mark the current page, and collapsible groups of nested items.

## Source Copy

```bash
dxui add menu
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.2", default-features = false, features = ["menu"] }
```

## API Surface

- `Menu`
- `MenuTitle`
- `MenuItem`
- `MenuGroup`
- `menu_class`
- `menu_item_class`

```rust
let mut components_open = use_signal(|| true);

rsx! {
  nav { "aria-label": "Documentation",
    Menu {
      MenuTitle { "Guides" }
      MenuItem { href: "/docs/start", active: true, "Getting started" }
      MenuItem { href: "/docs/theming", "Theming" }
      MenuGroup {
        label: rsx! { "Components" },
        open: components_open(),
        on_open_change: move |open| components_open.set(open),
        MenuItem { href: "/components/button", "Button" }
        MenuItem { href: "/components/dialog", "Dialog" }
      }
    }
  }
}
```

## Behavior

See [RFC 0072](../rfcs/0072-menu-and-mockup.md).

- `Menu` is a `ul`, `MenuTitle` and `MenuItem` are `li` elements, and a
  `MenuGroup` is an `li` with a button and a nested `ul`.
- An item with an `href` renders a link and one with an `onclick` a button;
  otherwise it wraps the app's own link or button. A disabled link drops
  its `href` and a disabled button is `disabled`, so neither takes focus or
  calls `onclick`.
- `MenuGroup`'s `open` stays controlled: its button calls
  `on_open_change(!open)`, and the nested list is hidden while closed. Its
  chevron points down when closed and up when open.
- Groups nest: a `MenuGroup` can hold another.

Unlike `Dropdown` or `Menubar`, Menu has no `role="menu"`; it is a list of
links, read and moved through like any other list, with Tab.

## Accessibility Notes

Put the menu in a `nav` with an `aria-label`. The active link or button has
`aria-current="page"`. Group buttons use `aria-expanded` and `aria-controls`
for the nested list. Titles are text only; name a group of links with its
title by wrapping it in a `MenuGroup` when it should collapse.
