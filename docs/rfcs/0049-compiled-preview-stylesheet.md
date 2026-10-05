# RFC 0049: Compiled Preview Stylesheet

- Status: Accepted
- Created: 2026-10-05

## Summary

Give every preview target compiled Tailwind. A generated stylesheet is
committed next to the shared preview page and linked from `PreviewSurface`. A
release gate keeps it in sync with the component classes. The Desktop and
Mobile self-tests check that it applies.

## Current State

As of M173:

- The Web and Desktop preview binaries link their own `assets/preview.css`.
  That file is a Tailwind input: `@import "tailwindcss"`, `@source` lines,
  and the dark theme block. `dx` serves it as is, and the browser or WebView
  cannot resolve the import, so no utility class applies. The Mobile preview
  links no stylesheet at all.
- Only `npm run verify:runtime-interactions` sees compiled Tailwind, because
  it answers the stylesheet request with CSS compiled by
  `scripts/preview-tailwind.mjs` (RFC 0043). A maintainer who runs a preview
  by hand gets an unstyled page, and the Desktop, iOS, and Android
  self-tests run their scenarios without layout.
- RFC 0043 left compiled CSS out of the Desktop and Mobile self-tests until a
  WebView layout bug was reported. None has been, but the unstyled manual
  previews are a present problem of their own.
- The compiled preview CSS is about 58KB. Compiling the same sources twice
  gives identical output.

## Decision

- `npm run css:preview` compiles `examples/web-demo/assets/preview.css` and
  writes `examples/preview-states/assets/preview.generated.css`. The
  generated file is committed.
- `PreviewSurface` links the generated file with `asset!`, so the Web,
  Desktop, and Mobile previews all load it. The Web and Desktop binaries no
  longer link their uncompiled inputs. The inputs stay as the compile source
  and keep their `npm run verify:css-inputs` checks.
- `npm run verify:preview-css`, part of the release gate, compiles the input
  and fails when the committed file differs from the result. Its message says
  to run `npm run css:preview`. A class change in a component or a fixture
  now needs that command before the gate passes.
- The in-app self-test starts with a `stylesheet` scenario that waits for an
  `sr-only` element to be absolutely positioned. The Desktop, iOS, and
  Android checks then report nine scenarios.
- `npm run verify:runtime-interactions` still answers the stylesheet request
  with a fresh compile. It keeps testing the current classes even when the
  committed file is stale, and the drift gate reports the stale file.
- RFC 0003 still holds for library users: dioxus-ui ships no compiled
  Tailwind output. The generated file belongs to the previews only.

## Scope

In scope:

- the regeneration script, the generated file, and its drift gate
- the `PreviewSurface` link and the removal of the binary links
- the self-test stylesheet scenario

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| dx's Tailwind integration | It downloads the Tailwind binary at build time, which the offline release gate cannot rely on | `dx` can use a local Tailwind binary |
| Layout assertions in the Desktop and Mobile self-tests | The scenarios assert behavior; the Web check covers layout of the same page | A WebView renders the page differently from Chrome |
| Regenerating the file in a pre-commit hook | The hook would need Node and the npm dependencies on every commit | Stale-file failures become frequent in practice |

## Verification

- `npm run verify:preview-css` passes on the committed file. It fails when
  the file is stale, for example after a component class changes without
  `npm run css:preview`.
- `npm run verify:desktop-interactions` passes nine scenarios, starting with
  `stylesheet`. The iOS and Android checks do the same where their toolchains
  are available.
- Reverse checks: removing the `PreviewSurface` link fails the Desktop
  `stylesheet` scenario, and a stale generated file fails the drift gate.
