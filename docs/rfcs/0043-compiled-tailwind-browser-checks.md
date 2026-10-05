# RFC 0043: Compiled Tailwind Browser Checks

- Status: Accepted
- Created: 2026-10-05

## Summary

Run the Web browser checks against compiled Tailwind and fix what that
shows. The runtime interaction verifier compiles the preview stylesheet with
the Tailwind Node API and serves the result in place of the uncompiled file.
Data-attribute variants match attribute values, Navigation Menu content opens
below its trigger, and a static guard rejects bare data variants.

## Current State

As of M167:

- The Web preview links `assets/preview.css`, which holds
  `@import "tailwindcss"` and `@source` lines. `dx serve` serves it as is, so
  the browser cannot resolve the import and no utility class applies. Every
  browser check has run without class-based layout; fixtures carry inline
  styles in its place, and some checks dispatch events instead of pressing
  where an element would be. The Slider thumb bug fixed in M167 went
  unnoticed for this reason.
- Components render boolean flags as `data-disabled="false"` or `"true"`,
  but their classes use Tailwind's bare `data-disabled:` variant, which
  matches whenever the attribute exists. Compiled, every enabled Select,
  Combobox, Command, menu, and Navigation Menu option or link gets
  `pointer-events: none` and half opacity, and every inactive item gets its
  active style. The same holds for `data-active`, `data-selected`,
  `data-invalid`, `data-collapsed`, `data-placeholder`, and `data-sort`,
  which renders `"none"` when unsorted.
- Value variants are written as `data-orientation-vertical:`,
  `data-state-open:`, and `data-side-left:`. Tailwind reads them as
  attributes named `data-orientation-vertical` and so on, which nothing
  renders, so Carousel, Resizable, Scroll Area, Toast, and Sidebar
  orientation, state, and side styles never apply.
- `NavigationMenuContent` is `md:absolute left-0 top-0` inside the
  `relative` item, so open content covers its own trigger.

## Decision

- `scripts/preview-tailwind.mjs` compiles a preview stylesheet with
  `@tailwindcss/node` and scans sources with `@tailwindcss/oxide`, as the
  Tailwind CLI does. The CLI package is not used: it pulls in a file watcher
  whose dependency has an open advisory, and the checks do not watch.
- `npm run verify:runtime-interactions` routes the request for the preview
  stylesheet to the compiled CSS, so the page loads it through its own
  `Stylesheet` link. It first asserts that an `sr-only` element is
  absolutely positioned, so a broken route fails instead of silently testing
  unstyled markup.
- Data variants match values: `data-[disabled=true]:`,
  `data-[orientation=vertical]:`, `data-[state=open]:`, `data-[side=left]:`,
  and `data-[sort=ascending]:` with `data-[sort=descending]:`. The rendered
  attributes stay as they are, so scripts, docs, and apps that read them are
  unaffected. The listbox's `data-highlighted` is set without a value and
  keeps the bare variant.
- `NavigationMenuContent` becomes `top-full mt-1.5`, so it opens below its
  trigger.
- `npm run verify:tailwind-static` fails on a bare `data-name:` variant
  unless the attribute is set without a value.

## Scope

In scope:

- the compile helper and the stylesheet route in the runtime verifier
- the data variants and the Navigation Menu content position in the crate
  and the templates
- the static guard
- removing inline layout workarounds from the interaction fixtures and
  replacing checks that worked around missing CSS with measured ones

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Utilities that conflict inside one composed class list, such as the vertical Slider's `w-full` with `w-auto` | Needs a per-element conflict check and fixes across components | M169 |
| Compiled CSS in the Desktop and Mobile self-tests | Their scenarios assert behavior, not layout | A WebView layout bug is reported |
| A compiled stylesheet served by `dx serve` for manual previews | `dx` downloads Tailwind from GitHub at build time, which the offline release gate cannot rely on | `dx` can use a local Tailwind binary |

## Verification

- `npm run verify:tailwind-static` passes on the rewritten classes and fails
  on the previous ones.
- `npm run verify:runtime-interactions` runs every fixture with compiled
  Tailwind, without inline layout styles on the Slider, Input OTP, Carousel,
  Resizable, and context menu fixtures. It measures that vertical tab
  triggers stack, that disabled links and items have
  `pointer-events: none`, that Carousel slides step by the viewport width
  plus the gap, and that a press on the dialog overlay outside the content
  closes it.
- Reverse checks: the bare `data-disabled:` variant, Navigation Menu content
  at `top-0`, or serving the uncompiled stylesheet each make the verifier
  fail.
