# TODOs

## Progress

- Overall: 50% (3 of 6 tasks)
- Current milestone: M213 (0.6.0 public surface)
- Current task: M213.4

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), `docs/archive/TODOs.completed-20261005-m186.md` (M186), `docs/archive/TODOs.completed-20261006-m187-m194.md` (M187 to M194, 0.2.0), `docs/archive/TODOs.completed-20261006-m195-m201.md` (M195 to M201, 0.3.0), `docs/archive/TODOs.completed-20261006-m202-m207.md` (M202 to M207, 0.4.0 to 0.4.2), and `docs/archive/TODOs.completed-20261007-m208-m212.md` (M208 to M212, 0.5.0)
- Previous roadmap: `docs/archive/roadmap-20261006-stages-0-10.md`

## Goals

- Follow [the roadmap](docs/roadmap.md) to 1.0. This plan covers Stage 13 (0.6.0). Stage 14 waits for a Dioxus 0.8 release (see Deferred), and Stage 15 needs Stage 14 and outside use, so neither gets tasks yet.

## Evidence (measured 2026-10-07 at `v0.5.0`)

- Undocumented public items (`RUSTFLAGS="-W missing_docs" cargo check --all-features`): 0 in `dioxus-shadcn-core`, 605 in `dioxus-shadcn-primitives`, 1139 in `dioxus-shadcn` (442 constants, 382 functions, 125 variants, 82 modules, 51 methods, 39 enums; first recorded as 1599, which included the primitives crate's warnings).
- `dioxus-shadcn` declares 451 `pub const`s; 10 are named anywhere outside its `src` (site, examples, docs, scripts, CLI tests), 7 of those only in RFC prose. It declares 424 snake-case `pub fn`s, 359 of them `*_class`.
- `dioxus-shadcn` re-exports 196 primitive items from 28 modules; the demos and fixtures name many of them (calendar math, carousel and chart helpers, primitive configs), several of which predate root-owned state (RFC 0077), such as `carousel_next`, `SidebarState`, and `ActiveDescendantState`.
- `cargo-semver-checks` is a manual step in `docs/release.md`, not part of `npm run verify:release`.
- Dioxus: the newest release is 0.8.0-alpha.1, so the Stage 14 gate exits 1.

## Scope Rules

- A task lands complete: crate, templates, registry, docs, site, and tests together. What cannot meet this is cut, not stubbed.
- Narrowing never breaks a documented use: anything a docs page, the site, or a block shows stays public, or the docs change in the same task with a Migration note.
- Breaking changes go to 0.6.0, each with a Migration note in the CHANGELOG.
- No new components or blocks without an issue or a block that needs them.
- Publish 0.6.0 once every task before M214.2 is done (the release owner's direction of 2026-10-07).

## M213 Public Surface (RFC 0079)

- DONE M213.1 Measure the surface and decide what stays public
  - A script lists every public item of the three library crates from rustdoc JSON with its kind and where it is named outside its own module (other crate modules, templates, site, examples, docs pages, scripts). RFC 0079 sets a rule per kind (component, props, class function, class constant, state helper, primitive re-export, primitive crate item) from those counts, and decides whether `dioxus-shadcn-primitives` keeps a semver promise of its own.
  - Exit: the script's counts are in RFC 0079 and match `missing_docs` within the items rustdoc and the lint both see; every rule names the evidence behind it and the condition that would reverse it.
  - Done (40b724b): `scripts/public-surface.mjs` matches `missing_docs` exactly (1139 styled, 605 primitives; rustdoc also shows 1381 macro-generated props fields the lint cannot see). RFC 0079 keeps public what a page lists or docs, the site, a block, or the CLI show: in `dioxus-shadcn` 443 class constants, 222 class functions, 17 functions, 2 constants, and 1 struct go private, and unlisted primitive re-exports go unless a public signature needs them; the primitives crate keeps its own promise, drops the 22 items nothing names, and documents the rest; every library crate denies `missing_docs`.
- DONE M213.2 Narrow and document `dioxus-shadcn`
  - Apply RFC 0079 to the styled crate and its templates (same visibility, so the parity test stays green), drop re-exports the rule removes, move demos and fixtures off them, and give every remaining public item a doc comment or a docs page. `#![deny(missing_docs)]` in `lib.rs`.
  - Exit: `cargo check -p dioxus-shadcn --all-features` passes under the deny; the per-feature build, the generated fixture app, the site, and the examples build; class merge, conflict, and parity scripts still find every class function (reverse-verify by hiding one).
  - Done (30120db): 443 class constants, 164 unlisted class functions, and 46 listed-only or unused helpers private; 9 unlisted re-exports dropped; 7 helpers orphaned by RFC 0077 and `message_scroller_class`'s unused parameter removed. 491 doc comments (written per module, copied to templates by anchor); `#![deny(missing_docs)]` passes. `public-surface.mjs` now finds no unlisted class constant, class function, or re-export and nothing undocumented. Demos dropped narrowed prints; the preview chart uses Chart components. The merge check went red on a now-private class function appending `class` and green when restored; the conflict check still sees 688 class functions. Release gate, site, runtime interactions (52), preview, examples, per-feature clippy, and fixture smoke pass.
- DONE M213.3 Narrow and document `dioxus-shadcn-primitives`
  - Same for the primitives crate under RFC 0079's decision on its promise; the styled crate keeps compiling against it.
  - Exit: `#![deny(missing_docs)]` passes; the crates README and `docs/public-api-surface-inventory.md` state what the primitives crate promises.
  - Done (efec746): the compiler showed which of the 22 unnamed items nothing calls; the `dismissal` and `typeahead` modules, the toast/Sonner runtime request helpers, `slider_snap`, and `slider_percent` are removed, three crate-only helpers are private, and the rest stay as types of called functions. 569 doc comments; primitives and core deny `missing_docs`, `cargo doc -D warnings` passes, and 288 primitive doc blocks were copied into the templates that inline them. The crates README and the inventory state the primitives crate's own semver promise. Release gate, site, runtime interactions, and fixture smoke pass.
- TODO M213.4 `cargo-semver-checks` in the release gate
  - `npm run verify:release` compares the three library crates against the last published version, with the release type taken from the version bump, and `docs/release.md` drops the manual step.
  - Exit: the gate fails when a public function is removed without a minor bump and passes with the bump (reverse-verify both on a scratch branch); CI runs it.

## M214 0.6.0 Release

- TODO M214.1 Prepare 0.6.0
  - CHANGELOG with a Migration note per item M213 made private or removed, versions, release gate, publish dry run.
- TODO M214.2 Publish 0.6.0
  - Push, CI, publish in dependency order, annotated tag `v0.6.0`, then build fresh apps in both modes from crates.io that use a component, a class override, and a state callback.

## Deferred (re-evaluate when)

- Dioxus 0.8 (Stage 14): when `cargo search dioxus --limit 1 --color never | grep -qE '^dioxus = "0\.8\.[0-9]+"'` exits 0 (a 0.8 release, not a pre-release). Checked 2026-10-07: exits 1 on 0.8.0-alpha.1.
- iOS 27 launch failure (Dioxus 0.7 lacks the UIScene lifecycle): when `gh release view -R DioxusLabs/dioxus --json body -q .body | grep -qi uiscene` exits 0.
- Form state and validation, chart tooltips, swipe gestures, DOM portal, Command fuzzy ranking, editing an Input OTP slot in the middle, and right-to-left Slider, Resizable, and Calendar keys: when `gh issue list -R yuxuetr/dioxus-ui --state all --search "<topic>" --json number -q length` prints more than 0, or a block needs one.
- More blocks: when an issue asks for a screen, by the same command.
- Generating templates from the crate: when `CRATE_ONLY` in `crates/dioxus-shadcn-cli/tests/template_parity.rs` passes 10 entries, or M210 needs a template difference the parity rules cannot express.
- A fullstack hydration app in CI: when Stage 14 starts, a fullstack issue is filed, or a component generates ids without `next_element_id()`.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
