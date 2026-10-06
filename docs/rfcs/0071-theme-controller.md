# RFC 0071: Theme Controller

- Status: Accepted
- Created: 2026-10-06

## Summary

Add `ThemeController`, which applies a theme to the document root, follows
the system's color scheme by default, remembers the choice, and with
`theme_init_script` applies it before the app's first paint.

## Current State

The dark theme is an opt-in `.dark` class
([RFC 0047](0047-opt-in-dark-theme.md)) and presets are `data-theme` values
([RFC 0057](0057-theme-presets.md)); the release notes exclude a
system-preference default. Every app writes the same code to toggle the
class, read `prefers-color-scheme`, and persist the choice, and the site
forgot its theme on reload. daisyUI ships a Theme Controller for this.

## Decision

- `Theme` is `System`, `Light`, `Dark`, or `Preset(name)`, stored as
  `system`, `light`, `dark`, or the name.
- `ThemeController { theme, storage_key, on_theme_change }` runs a script
  for as long as it is mounted. It sets `data-theme` on the root for a
  preset and the `dark` class for the dark scheme, and while the theme is
  `System`, follows `prefers-color-scheme` through its change event.
- Each theme the app passes is written to `localStorage` under
  `storage_key` (`dxui-theme` by default, empty for none). On mount, a
  stored theme that differs from the app's is applied and reported through
  `on_theme_change`, so `theme` stays controlled while the app adopts the
  remembered choice. Nothing stored means `System`, the requested default.
- `theme_init_script(storage_key)` returns a one-line script for the page's
  `<head>` that does the same before the app loads, so a stored dark or
  preset theme does not flash light first. The site's `index.html` uses it,
  and a site test keeps the two equal.
- The controller writes to the document root because the `dark` variant
  and presets apply to everything inside the element that carries them, and
  only the root covers overlays and the page background.

The control that picks a theme stays the app's: a select, a toggle group,
or daisyUI's checkbox. The site's header menu and its Theme Controller
example share one controller through context.

## Alternatives

- **A hook.** A component makes the script's lifetime visible in the tree
  and works the same in source-copy templates.
- **Cookies.** They let a server render the right theme, but this library
  has no server rendering path; `localStorage` needs no server.

## Verification

- Unit tests for `Theme`'s stored form and the init script; an SSR test for
  the marker.
- A runtime check: the system scheme followed live, each theme on the root
  and in storage, and a controller mounted with a stored theme applying and
  reporting it; reverse-verified without the report.
- `verify:site` checks the header menu, persistence across a reload with the
  init script, the system scheme, and every preset on the root.
