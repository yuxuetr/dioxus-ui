# RFC 0047: Opt-in Dark Theme

- Status: Superseded by [RFC 0051](0051-semantic-color-tokens.md)
- Created: 2026-10-05

> Since M178, the palette remap below is replaced: `.dark` redefines only the
> RFC 0051 semantic color tokens, and app palette classes keep their colors
> under `.dark`. The opt-in `dark` class and the contrast checks remain.

## Summary

Add a dark theme that apps turn on with a `dark` class. The theme is a block
of palette variables in the generated stylesheet. Component classes do not
change. A browser check measures text contrast in both themes.

## Current State

As of M171:

- Component classes use fixed Tailwind palette utilities: about 340 zinc,
  100 blue, 80 red, and 70 white utilities, plus a few green, amber, and
  emerald ones. There is no dark theme, and RFC 0003 deferred one until the
  light theme was stable. M168 to M171 verified the light theme against
  compiled Tailwind.
- Tailwind v4 compiles each color utility to a palette variable, such as
  `background-color: var(--color-zinc-900)` for `bg-zinc-900` and
  `var(--color-white)` for `bg-white`. Redefining those variables restyles
  every component without touching a class.
- An experiment with compiled CSS swapped each scale end for end (50 with
  950, 100 with 900, and so on, 500 unchanged) and white with zinc-950. Text
  contrast held everywhere except two cases:
  - `text-zinc-500` muted text reached only 4.12:1 on the dark surface;
  - the white Checkbox tick on `blue-400` reached about 2.6:1.
  In the light theme, no rendered text was below the WCAG AA minimum.

## Decision

- The CLI default `assets/dioxus-ui.css` gains a `.dark` block that sets
  `color-scheme: dark` and redefines these palette variables:
  - white becomes zinc-950;
  - each zinc step takes its mirror step (950 for 50, 900 for 100, and so
    on), except that 500 and 600 both take 400, so muted text stays at least
    4.5:1;
  - each blue, red, green, amber, and emerald step takes its mirror step,
    except that 500 takes 400 and 600 takes 500. Fills stay saturated enough
    for the white Checkbox marks to keep 3:1, and dark labels on them keep
    4.5:1.
- The values are Tailwind's literal palette values. They cannot refer to the
  variables they replace, because the swaps would form reference cycles.
- The theme is opt-in. An app adds `class: "dark"` to an ancestor, usually
  the root element. The docs show how to follow the system preference by
  wrapping the block in `@media (prefers-color-scheme: dark)`.
- The Web and Desktop preview stylesheets carry the same block, and
  `npm run verify:css-inputs` fails when they drift from the CLI block.
  Crate-mode apps copy the block from the theming docs.
- The block remaps the palette for everything under `.dark`, app classes
  included. An app that uses zinc for its own surfaces goes dark with the
  components.

## Scope

In scope:

- the `.dark` block in the CLI default stylesheet and the preview
  stylesheets, and its drift check
- browser contrast checks in both themes
- theming docs

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Semantic color tokens (`bg-background`, `bg-primary`) in component classes | Rewrites about 700 color utilities in the crate and the templates. Crate-mode apps that do not define the tokens would lose every color. The palette remap gives dark mode without either cost | A consumer needs a brand palette that remapping the palette variables cannot express |
| Dark theme on by default through `prefers-color-scheme` | Apps with light page surfaces would get dark components with no change on their side | A consumer asks for it, or most generated apps adopt the dark class |
| App `text-white` over images or app-owned fills | White becomes zinc-950 under `.dark`, so white text on an image turns dark | A consumer reports unreadable text under `.dark` |
| Non-text contrast of the Switch thumb on its track | The light theme has the same low thumb-to-track contrast; the thumb position and shadow carry the state | A user reports that the Switch state is unclear |

## Verification

- `npm run verify:css-inputs` passes when the Web and Desktop preview blocks
  match the CLI block, and fails when one differs.
- `npm run verify:runtime-interactions` compiles the preview stylesheet with
  the block. In the light theme, then with `dark` on the root element, it
  checks every visible element that holds text. The text must reach 4.5:1
  against its composited background, or 3:1 for large text. Disabled and
  faded elements are skipped. The check runs after the first render and again
  after the interactions, with menus and dialogs open. It also checks that a
  card surface is dark under `.dark`.
- Reverse checks: a pure end-for-end inversion, a stylesheet without the
  block, or a block without the white remap each make the verifier fail.
