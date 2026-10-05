# Menu System API Plan

This document defines the M13 menu system APIs before implementation. The goal
is to reuse existing keyboard, typeahead, dismissal, and placement primitives
without pretending that all menu-like components have the same semantics.

Status: Implemented in M13.

## Scope

M13 covers:

- Context Menu: implemented
- Menubar: implemented
- Navigation Menu: implemented

These components should ship in both crate mode and source-copy mode. Crate mode
can reuse `dioxus-shadcn-core` and `dioxus-shadcn-primitives`; generated templates must
remain self-contained and must not import internal crates.

## Shared Rules

Menu systems use controlled state first:

- `open: bool` for overlay content
- explicit `checked`, `disabled`, and `selected` props where applicable
- `class: String` on every styled part
- `children: Element` for composition slots

All Tailwind classes must be complete static tokens in source. Runtime selection
between predefined strings is allowed; runtime construction of class names is
not allowed.

Primitive reuse:

- roving focus for item movement
- typeahead for enabled item matching
- dismissal behavior for Escape and outside interaction
- overlay placement for floating menu content

Deferred runtime work:

- DOM focus commands
- nested submenu focus handoff
- pointer intent delay
- collision-aware runtime measurement
- portal mounting

## Context Menu

Context Menu is an application command menu opened from a pointer or keyboard
context action. It should use menu semantics, not navigation semantics.

Planned crate API:

```rust
ContextMenuContent { open, class, children }
ContextMenuGroup { class, children }
ContextMenuLabel { class, children }
ContextMenuItem { disabled, inset, destructive, class, children }
ContextMenuCheckboxItem { checked, disabled, class, children }
ContextMenuRadioGroup { value, class, children }
ContextMenuRadioItem { value, checked, disabled, class, children }
ContextMenuSeparator { class }
ContextMenuShortcut { class, children }
```

Behavior defaults:

- popover/dropdown-like dismissal
- vertical roving focus
- typeahead over enabled item labels
- `role="menu"` for content
- `role="menuitem"` for regular items
- `role="menuitemcheckbox"` and `role="menuitemradio"` for checked items

M13 should not implement pointer-position anchoring yet. The first version can
be controlled content with placement metadata and static classes.

## Menubar

Menubar is a persistent horizontal command surface with menu content attached to
top-level triggers.

Planned crate API:

```rust
Menubar { class, children }
MenubarMenu { class, children }
MenubarTrigger { open, disabled, class, children }
MenubarContent { open, class, children }
MenubarItem { disabled, inset, destructive, class, children }
MenubarCheckboxItem { checked, disabled, class, children }
MenubarRadioGroup { value, class, children }
MenubarRadioItem { value, checked, disabled, class, children }
MenubarLabel { class, children }
MenubarSeparator { class }
MenubarShortcut { class, children }
```

Behavior defaults:

- horizontal roving focus for top-level triggers
- vertical roving focus inside menu content
- typeahead inside open content
- Escape dismissal enabled
- outside pointer dismissal enabled

Nested submenus are planned but should ship as placeholders or documented
deferred parts until focus handoff behavior is designed and tested.

## Navigation Menu

Navigation Menu is for site or app navigation. It may look like a menu, but it
should keep navigation semantics instead of forcing command-menu roles.

Planned crate API:

```rust
NavigationMenu { class, children }
NavigationMenuList { class, children }
NavigationMenuItem { class, children }
NavigationMenuTrigger { open, disabled, class, children }
NavigationMenuContent { open, class, children }
NavigationMenuLink { active, disabled, class, children }
NavigationMenuViewport { open, class, children }
NavigationMenuIndicator { open, class }
```

Behavior defaults:

- root/list use navigation-friendly structure
- links remain link-like content owned by the app
- roving focus may support trigger movement
- content can reuse popover-like placement and dismissal

Use Navigation Menu for navigation destinations. Use Menubar or Context Menu for
commands and application actions.

## Platform Defaults

| Target | Menu systems |
| --- | --- |
| Web | Use standard roles and keyboard events; keep runtime focus adapters deferred. |
| Desktop | Match web keyboard behavior where WebView support allows. |
| Mobile | Avoid hover-only opening; prefer tap-controlled disclosure or Sheet for complex navigation. |

## Implementation Order

Completed order:

1. Context Menu
2. Menubar
3. Navigation Menu
4. documentation, examples, and parity updates

This order starts with the menu semantics that are closest to Dropdown, then
adds persistent top-level triggers, then separates navigation-specific behavior.

M13 shipped controlled styled parts for all three menu systems. Runtime roving
focus commands, typeahead event wiring, context-trigger anchoring, nested
submenu handoff, and viewport measurement remain adapter work.

## Quality Gates

Each M13 component should include:

- crate-mode class composition tests
- primitive helper tests when roving focus or typeahead helpers are exposed
- registry entry and self-contained template
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking each task done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
