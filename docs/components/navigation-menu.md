# Navigation Menu

Navigation Menu provides controlled navigation-oriented parts. It keeps
navigation semantics instead of forcing command-menu roles.

## Source Copy

```bash
dxui add navigation-menu
```

## Crate Feature

```toml
dioxus-shadcn = { version = "0.6", default-features = false, features = ["navigation-menu"] }
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
- `NavigationMenuOrientation`
- `NavigationMenuPrimitiveConfig`
- `navigation_menu_class`
- `navigation_menu_trigger_class`
- `navigation_menu_link_class`

The module also re-exports `PopoverPrimitiveConfig` for users importing from
`dioxus_shadcn::navigation_menu`.

## Behavior

`NavigationMenu` owns which item is open, by the `value` of its
`NavigationMenuItem`, or the empty string while none is, as Radix does; each
item's trigger and content compare it with their item's (see
[RFC 0077](../rfcs/0077-component-owned-state.md)). Pass `value` and
`on_value_change` to control it, or `default_value` to start with an item
open; `on_value_change` hears every change, the empty string on close. An
item without a `value` gets a generated one:

```rust
rsx! {
  NavigationMenu { "aria-label": "Product",
    NavigationMenuList {
      NavigationMenuItem { value: "docs",
        NavigationMenuTrigger { "Docs" }
        NavigationMenuContent {
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
- In a right-to-left layout, ArrowLeft moves to the next top-level item and
  ArrowRight to the previous one.
- ArrowDown on a trigger focuses the first link of its content, opening it
  first when closed. Inside content, ArrowDown and ArrowUp move between links
  and wrap; Home and End jump to the first and last link.
- Escape closes the open content and, when focus was in the menu, moves focus
  to its trigger. A pointer press outside the menu, focus moving outside it,
  or a click on a content link closes it.

Content is laid out with CSS, not anchored. `NavigationMenuViewport` and
`NavigationMenuIndicator` show while any item is open.

Content opens from the app's state, so in a server-rendered page it opens
only after hydration; until then top-level `NavigationMenuLink`s work as
plain links and contents stay `hidden`. Showing contents by CSS hover before
hydration would leave two sources of truth afterwards (a hovered content
that ignores Escape and shows while its trigger says `aria-expanded="false"`),
so a menu that must work without Wasm is better written with plain links and
CSS of the app's own.

### Submenus

Nest a `NavigationMenu` with `orientation: NavigationMenuOrientation::Vertical`
in a content for a mega menu: its triggers form a column, and its contents sit
beside its list, each with its item's `value`:

```rust
NavigationMenuContent {
  NavigationMenu {
    orientation: NavigationMenuOrientation::Vertical,
    "aria-label": "Solution areas",
    // Keep a panel open: ignore the empty value a close sends.
    value: area(),
    on_value_change: move |value: String| if !value.is_empty() { area.set(value) },
    NavigationMenuList {
      NavigationMenuItem { value: "web", NavigationMenuTrigger { "Web" } }
      NavigationMenuItem { value: "mobile", NavigationMenuTrigger { "Mobile" } }
    }
    NavigationMenuContent { value: "web", /* links */ }
    NavigationMenuContent { value: "mobile", /* links */ }
  }
}
```

Each menu's script acts only on its own items. In the vertical menu, a click
or hover opens a panel at once, ArrowDown and ArrowUp move between triggers,
ArrowRight enters the panel and ArrowLeft returns, and the panel stays open
when the pointer leaves, so keep one panel open, such as the first when the
outer item opens. Escape closes both menus and returns focus to the outer
trigger (see [RFC 0063](../rfcs/0063-navigation-menu-submenus.md)).

The Web renderer is covered by `npm run verify:runtime-interactions` and the
Desktop renderer by `npm run verify:desktop-interactions`, and the iOS
Simulator by `npm run verify:mobile-interactions`, and an Android emulator by
`npm run verify:android-interactions`.

## Accessibility Notes

Use Navigation Menu for navigation destinations. Use Menubar or Context Menu for
application commands. Navigation Menu follows the disclosure navigation
pattern: triggers are buttons with `aria-expanded`, content holds links, and
no `menu` roles are used. Viewport size measurement and motion are not
implemented (see [RFC 0016](../rfcs/0016-navigation-menu-interaction.md)).

`NavigationMenu` passes through attributes, so give the `nav` landmark an
`aria-label` when the page has more than one (see [RFC
0040](../rfcs/0040-composite-widget-names.md)).
