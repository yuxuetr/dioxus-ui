# Navigation Menu

Navigation Menu provides controlled navigation-oriented parts. It keeps
navigation semantics instead of forcing command-menu roles.

## Source Copy

```bash
dxui add navigation-menu
```

## Crate Feature

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["navigation-menu"] }
```

## API Surface

- `NavigationMenu`
- `NavigationMenuList`
- `NavigationMenuItem`
- `NavigationMenuTrigger`
- `NavigationMenuContent`
- `NavigationMenuLink`
- `NavigationMenuViewport`
- `NavigationMenuIndicator`
- `NavigationMenuPrimitiveConfig`
- `navigation_menu_class`
- `navigation_menu_trigger_class`
- `navigation_menu_link_class`

The module also re-exports `PopoverPrimitiveConfig` for users importing from
`dioxus_ui::navigation_menu`.

## Behavior

The open item stays controlled by the app. Keep the open item's value, give
each `NavigationMenuItem` a `value`, and set the value from
`on_value_change`, which receives an empty string to close:

```rust
let mut active = use_signal(String::new);
let is_open = move |value: &str| active() == value;

rsx! {
  NavigationMenu {
    on_value_change: move |value: String| active.set(value),
    NavigationMenuList {
      NavigationMenuItem {
        value: "docs",
        NavigationMenuTrigger { open: is_open("docs"), "Docs" }
        NavigationMenuContent {
          open: is_open("docs"),
          NavigationMenuLink { href: "/docs/install", "Install" }
        }
      }
      NavigationMenuItem {
        NavigationMenuLink { href: "/blog", "Blog" }
      }
    }
  }
}
```

- A click, Enter, or Space on a trigger toggles its content.
- With a mouse, resting on a trigger opens it after 200 ms, or immediately
  while another item is open. Moving off the open item's trigger and content
  closes it after 300 ms. A trigger closed by a click does not reopen until
  the pointer leaves it. Touch uses clicks only.
- Every trigger and link stays in the Tab order, and content follows its
  trigger, so Tab from an open trigger enters its links.
- Left and Right move focus between top-level triggers and links and wrap;
  Home and End jump to the first and last. Disabled triggers and links with
  `disabled` are skipped.
- ArrowDown on a trigger focuses the first link of its content, opening it
  first when closed. Inside content, ArrowDown and ArrowUp move between links
  and wrap; Home and End jump to the first and last link.
- Escape closes the open content and, when focus was in the menu, moves focus
  to its trigger. A pointer press outside the menu, focus moving outside it,
  or a click on a content link closes it.

Content is laid out with CSS, not anchored. `NavigationMenuViewport` and
`NavigationMenuIndicator` only reflect the `open` value the app passes.

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`; Mobile is not
covered by an automated check.

## Accessibility Notes

Use Navigation Menu for navigation destinations. Use Menubar or Context Menu for
application commands. Navigation Menu follows the disclosure navigation
pattern: triggers are buttons with `aria-expanded`, content holds links, and
no `menu` roles are used. Viewport size measurement, submenus, motion, and
right-to-left arrow mirroring are not implemented (see
[RFC 0016](../rfcs/0016-navigation-menu-interaction.md)).
