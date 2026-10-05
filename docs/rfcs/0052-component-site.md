# RFC 0052: Component Site

- Status: Accepted
- Created: 2026-10-05

## Summary

Add a component site to the repository: a Dioxus Web app, built on the
published components, where a visitor browses the catalog and sees each
component's live examples, source, and install commands in the light and
dark themes, in the manner of the shadcn/ui site.

## Current State

As of M178:

- The components use the shadcn/ui semantic color tokens (RFC 0051), so a
  page that toggles the `dark` class shows both themes.
- `scripts/docs-catalog-builder.mjs` already joins the registry entries,
  templates, component docs, crate features, and eight catalog categories
  (Actions, Forms, Overlays, Navigation, Layout, Data Display, Feedback,
  Messaging) into one catalog of 64 components. It feeds the generated
  `docs/components/catalog.md` and `routes.md`, which plan
  `/components/{slug}` routes "for a future Dioxus docs runtime".
- The previews (RFC 0049, RFC 0050) show fixtures for checks, not examples
  for readers, and have no navigation.
- The locked Dioxus is 0.7.9. Its router is behind the `router` feature of
  the `dioxus` crate.

## Decision

### Crate

- A `site/` workspace member, package `dioxus-ui-site`, `publish = false`.
  It depends on `dioxus` with the `web` and `router` features and on
  `dioxus-shadcn` by path with every component feature, so it renders what a
  crate-mode app gets.
- The site uses the library's own components (Button, Toggle, Sheet, Tabs,
  and so on) for its chrome. A component that cannot build the site is a gap
  in the library.
- Run it with `dx serve --package dioxus-ui-site`.

### Routes

| Route | Page |
| --- | --- |
| `/` | Home: what the library is, the two install modes, links into the catalog |
| `/docs/getting-started` | Source-copy and crate-mode setup, the generated stylesheet |
| `/docs/theming` | The token list, rebranding, the opt-in dark theme |
| `/components/:slug` | One page per catalog component |
| anything else | Not found page |

A `/components/:slug` for a slug outside the catalog renders the not found
page.

### Catalog data

- `npm run site:catalog` writes `site/src/catalog.rs` from the docs catalog
  builder: each category in order, and each component's slug, title,
  description, and crate feature. The sidebar and the component pages read
  it, so the site, `catalog.md`, and `routes.md` come from one source.
- `npm run verify:site-catalog`, in the release gate, fails when the file is
  stale.
- The install commands are derived from the slug: `dxui add <slug>` for
  source-copy apps and `features = ["<feature>"]` for crate-mode apps.

### Examples

- Each example is one Rust file under `site/src/examples/<slug>/`, holding
  one component function. The page renders the function and shows the same
  file with `include_str!`, so the shown source is the code that runs.
- Examples import from `dioxus_shadcn`, as a crate-mode app does. The source tab
  shows that import.
- M180 adds the page template and the examples; M179 builds the shell.

### Theme

- The header has a dark theme toggle that adds the `dark` class to the site
  root, like the preview toggle (RFC 0050). The site starts light.

### Stylesheet

- `site/assets/site.css` is a Tailwind input: the import, `@source` roots
  for the site and the component crate, and the CLI stylesheet body copied
  verbatim. `npm run verify:css-inputs` covers it like the preview inputs.
- `npm run css:site` compiles it to `site/assets/site.generated.css`, which
  the site links, and `npm run verify:site-css`, in the release gate, fails
  when the compiled file is stale, as `verify:preview-css` does for the
  previews (RFC 0049).

### Layout

- A header, a sidebar with the catalog grouped by category, and the page.
  Below the `md` breakpoint the sidebar moves into a Sheet opened from a
  header menu button, so a 375px viewport has no sideways scroll.

### Verification

`npm run verify:site` (M179.3) serves the site with `dx serve`, visits every
route, and fails on a console error, a route that renders the not found page,
text below 4.5:1 in either theme, or a sideways scroll at 375px. It needs a
browser, like `verify:runtime-interactions`, and is not part of the release
gate.

## Scope

In scope:

- the crate, routes, sidebar, home, getting started, theming, and component
  pages
- the catalog generator, the site stylesheet, and their drift gates
- the browser check
- examples for every catalog component (M180)

Out of scope, with reevaluation conditions:

| Item | Reason | Reevaluate when |
| --- | --- | --- |
| Hosting and deployment | Publishing a site is a release-owner decision, like crates.io | The release owner picks a host |
| Search | 64 components fit in a grouped sidebar | The catalog passes about 100 entries or a reader asks for it |
| Theme editor or base color picker | One zinc theme; RFC 0051 documents rebranding | A consumer asks for more than one shipped theme |
| Server-side rendering or static generation | The site is a client app served by `dx serve` | Hosting needs pages without WebAssembly |
| Remembering the theme across reloads | Starting light keeps the checks deterministic, as in RFC 0050 | A reader asks for it |
| Rendering the component docs Markdown | The pages link the Markdown docs instead | Docs and pages drift in a way the check misses |

## Verification

- `cargo check -p dioxus-ui-site --target wasm32-unknown-unknown` and the
  release gate pass with the site in the workspace.
- `npm run verify:site-catalog` and `npm run verify:site-css` fail on a
  stale file and pass after regenerating.
- `npm run verify:site` passes, and fails on a broken route and on a
  low-contrast class.
