# RFC 0050: Preview Theme Toggle

- Status: Accepted
- Created: 2026-10-05

## Summary

Add a dark theme toggle to the shared preview page, so a maintainer can see
the opt-in dark theme in the Web, Desktop, and Mobile previews. The browser
check and the in-app self-test press the toggle and check that the page turns
dark.

## Current State

As of M174:

- The opt-in `.dark` block (RFC 0047) is in the compiled preview stylesheet
  (RFC 0049), but nothing on the preview page sets the `dark` class. Only
  `npm run verify:runtime-interactions` turns the theme on, by adding the
  class to the root element from the test script.
- A maintainer running a preview by hand sees only the light theme. The
  Desktop, iOS, and Android WebViews never render the dark theme, so a WebView
  that resolves the remapped palette variables differently from Chrome would
  go unnoticed.
- The page root is `main` with `min-h-screen bg-white`. Overlays render
  inline (`PortalTarget::Inline`), so they are descendants of `main`.

## Decision

- `PreviewSurface` keeps a `dark_theme` signal, off by default, and adds
  `dark` to the class of its `main` root when it is on. Every fixture,
  including the inline overlays, is under `main`, so the whole page switches.
  The light theme stays the default, so existing checks see the same page.
- The page header gets a `Toggle` labelled "Dark theme" with
  `id="preview-theme-toggle"`. It uses the library's own component, and its
  `aria-pressed` state reports the theme.
- `npm run verify:runtime-interactions` presses the toggle after the first
  render. It checks that `main` has the `dark` class, that its background is
  dark, and that its `color-scheme` is `dark`. It then presses the toggle
  again and checks that the light theme returns. The contrast checks keep
  setting the class on the root element, because the dialog check runs while
  a modal dialog covers the toggle.
- The in-app self-test gains a `theme` scenario after `stylesheet`. It
  presses the toggle, waits for `color-scheme: dark` and a changed background
  on `main`, then presses it again. The Desktop, iOS, and Android checks then
  report ten scenarios.

## Scope

In scope:

- the toggle and the `main` class in `PreviewSurface`
- the browser check and the self-test scenario
- docs for the preview toggle

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Following the system color scheme in the previews | The previews show both themes on demand; RFC 0047 keeps the theme opt-in | RFC 0047 makes the system preference the default |
| Remembering the choice across reloads | The previews are for inspection; a reload starting light keeps the checks deterministic | A maintainer asks for it |
| Contrast checks in the Desktop and Mobile self-tests | The Web check covers contrast of the same page and stylesheet | A WebView renders a palette color differently from Chrome |

## Verification

- `npm run verify:runtime-interactions` passes the toggle check in both
  directions.
- `npm run verify:desktop-interactions` passes ten scenarios, including
  `theme`. The iOS and Android checks do the same where their toolchains are
  available.
- Reverse checks: a toggle that does not add the `dark` class fails the
  browser check and the Desktop `theme` scenario.
