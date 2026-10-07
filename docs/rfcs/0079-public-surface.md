# RFC 0079: Public Surface

- Status: Accepted
- Created: 2026-10-07

## Summary

0.6.0 keeps public only what a docs page, the site, a block, or the CLI
shows, and documents all of it. In `dioxus-shadcn` that makes 443 class
constants, 222 class functions, and 20 other items private and drops the
primitive re-exports nothing shows; `dioxus-shadcn-primitives` keeps a semver
promise of its own, loses the items nothing uses, and documents the rest.
Every library crate denies `missing_docs`.

## Current State

`node scripts/public-surface.mjs` lists every public item of the three
library crates from rustdoc JSON, with its kind, whether it has a doc
comment, and the first place outside its own file that names it: a
component page's text or example, the site, a block, the styled crate (for
the other crates), the CLI, an example app, a script, another module of the
same crate, or nowhere. A name in a page's "API Surface" list is counted
apart, as "listed", since a list promises an item whether or not anything
uses it. Measured at `v0.5.0`:

| Crate | Kind | Items | Undocumented | Undocumented parts | Listed | docs | site | blocks | styled | cli | examples | scripts | crate | nowhere |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| dioxus-shadcn | class constant | 444 | 441 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 444 |
| dioxus-shadcn | class function | 358 | 344 | 0 | 134 | 18 | 0 | 0 | 0 | 0 | 158 | 0 | 1 | 181 |
| dioxus-shadcn | component | 381 | 0 | 0 | 342 | 345 | 36 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| dioxus-shadcn | constant | 4 | 1 | 0 | 2 | 2 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| dioxus-shadcn | enum | 45 | 39 | 176 | 45 | 26 | 15 | 0 | 0 | 0 | 2 | 0 | 0 | 2 |
| dioxus-shadcn | function | 67 | 38 | 0 | 44 | 12 | 2 | 0 | 0 | 0 | 18 | 0 | 0 | 35 |
| dioxus-shadcn | module | 83 | 82 | 0 | 0 | 83 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| dioxus-shadcn | props | 381 | 0 | 1381 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 380 |
| dioxus-shadcn | re-export | 169 | 169 | 0 | 90 | 113 | 2 | 0 | 0 | 0 | 17 | 0 | 23 | 14 |
| dioxus-shadcn | struct | 4 | 1 | 17 | 2 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 3 |
| dioxus-shadcn-core | class function | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| dioxus-shadcn-core | enum | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| dioxus-shadcn-core | function | 1 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| dioxus-shadcn-core | struct | 3 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 1 | 0 | 0 | 0 | 2 |
| dioxus-shadcn-primitives | class function | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| dioxus-shadcn-primitives | enum | 36 | 14 | 153 | 12 | 30 | 1 | 0 | 3 | 0 | 0 | 0 | 0 | 2 |
| dioxus-shadcn-primitives | function | 67 | 65 | 0 | 26 | 49 | 0 | 0 | 11 | 0 | 0 | 0 | 0 | 7 |
| dioxus-shadcn-primitives | module | 20 | 20 | 0 | 0 | 18 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 2 |
| dioxus-shadcn-primitives | re-export | 1 | 1 | 0 | 0 | 1 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| dioxus-shadcn-primitives | struct | 58 | 30 | 308 | 21 | 39 | 0 | 0 | 8 | 0 | 0 | 0 | 0 | 11 |
| dioxus-shadcn-primitives | trait | 7 | 0 | 14 | 0 | 7 | 0 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |

Parts are fields, variants, inherent methods, and trait items. The counts
match `RUSTFLAGS="-W missing_docs" cargo check -p <crate> --all-features`
exactly for the items both see: 1139 in `dioxus-shadcn` and 605 in
`dioxus-shadcn-primitives` (the lint files a module's warning under
`lib.rs` and an impl's under its own file, which only moves counts between
files). Props fields are the exception: rustdoc shows the 1381 fields
`#[component]` generates, and the lint does not see them, since their spans
come from the macro. (The 0.6.0 plan first gave 1599 for `dioxus-shadcn`;
that build also printed the primitives crate's warnings.)

What the counts say:

- Class constants are a component's own pieces: 444 of 444 are named only in
  their own file. One, `CHART_COLOR_CLASSES`, is listed on the Chart page.
- Class functions split. 136 are listed or used in docs text, the kind of
  use shadcn/ui's `buttonVariants` serves (styling a link as a button). The
  other 222 are named only by the desktop and web demos, which print class
  strings as a smoke check, or nowhere.
- Other functions are mostly state and attribute helpers: 50 are listed or
  shown; 17 are named only by examples or nowhere.
- Of 169 primitive re-exports, 136 are listed or shown; the 17 names that
  are not include calendar math (`days_in_month`, `is_leap_year`), clamps
  (`carousel_clamp_index`, `data_table_clamp_page`), and Sonner attribute
  helpers.
- In the primitives crate, 126 of 190 items are named by the styled crate.
  The runtime adapter traits (`FocusRuntime`, `PortalRuntime`, and the
  others) are named by the runtime verification apps and two component
  pages; 22 items, among them placement and typeahead types, are named
  nowhere outside their file.

## Decision

An item stays public when a component page lists it, or docs text, the
site, a block, or the CLI names it. Each kind follows from that:

| Kind | Rule |
| --- | --- |
| Component, props, and the enums props take | Public. Every component is listed on its page's API Surface. |
| Module | Public, with a `//!` line that says what the component is. |
| Class constant | Private (`const`, or `pub(crate)` when another module reads it), except a listed one. |
| Class function | Public when listed or shown; otherwise private or `pub(crate)`. |
| State and attribute function | Public when listed or shown and still needed: a helper whose work a root does since RFC 0077 (such as stepping a Carousel the root already steps) is removed even when listed, with a Migration note. |
| Primitive re-export | Kept when listed or shown, or when a public signature names the type; otherwise removed. |
| Primitives crate item | Public when the styled crate, an example, or a page names it; removed when nothing does. |

Templates take the same visibility as their crate module, so the template
parity test (RFC 0066) keeps them identical; in copy mode the change is
invisible, since copies live in the app. Demos that print class strings
print the rendered class from the component instead.

Every remaining public item gets a doc comment, and each library crate's
`lib.rs` denies `missing_docs`, so the compiler, not a review, keeps new
public items documented. Props fields are documented on the component's
page, as they are today; the lint cannot see them.

`dioxus-shadcn-primitives` keeps its own semver promise. It is published,
the crates README offers it as the unstyled layer, and the runtime adapter
traits exist for apps to implement. A promise that covered only the items
the styled crate re-exports would be a rule no check enforces: the semver
check in the release gate (M213.4) compares whole crates.

Reverse the rule for any narrowed item when an issue asks to use it from an
app: make it public with a doc comment and list it. Reconsider the
primitives promise if keeping it blocks a change the styled crate needs; the
signal is a release whose Migration section is mostly primitives items.

## Alternatives

- Keep everything public and document it: about 1100 doc comments in the
  styled crate, most of them on constants that no app has a use for, and
  every class string change would become a breaking change.
- Hide narrowed items with `#[doc(hidden)]`: they would stay reachable and
  covered by the semver check, which is the cost this RFC removes.
- Merge the primitives crate into the styled crate: removes a crate from the
  promise, but the runtime traits and the copy-mode layout depend on the
  split. Not justified by anything measured here.

## Impact

Breaking for apps that name a narrowed item; each goes in the 0.6.0
Migration section. Components, props, and the class functions the pages
show do not change.

## Validation

- `cargo check --all-features` passes with `#![deny(missing_docs)]` in each
  library crate.
- The per-feature build, the generated fixture app, the site, the examples,
  and the template parity test pass.
- `node scripts/public-surface.mjs` reports no class constant other than
  the listed one and no class function or re-export that is neither listed
  nor shown.
