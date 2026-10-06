# TODOs

## Progress

- Overall: 100% (7 of 7 tasks)
- Current milestone: none (0.4.0 published)
- Current task: none

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), `docs/archive/TODOs.completed-20261005-m186.md` (M186), `docs/archive/TODOs.completed-20261006-m187-m194.md` (M187 to M194, 0.2.0), and `docs/archive/TODOs.completed-20261006-m195-m201.md` (M195 to M201, 0.3.0)

## Goals

- 0.4.0 makes copy mode clean for the people who use it: a copied component brings only the code it needs and builds without warnings, and the CLI says what it did and what changed since the app copied a component.

## Evidence (measured 2026-10-06 at `v0.3.0`, against crates.io)

- Adoption: 18 downloads per crate across 0.1.0 to 0.3.0, no issues or pull requests. There is still no outside demand signal, so the plan covers only problems reproduced here.
- `dxui add button` in a new app copies `button.rs` (102 lines) and `utils.rs` (1435 lines: overlay placement, listbox, roving group, focus scope, dismiss timer, hover, media query, and submenu scripts). `cargo build` prints 66 warnings: 63 for unused `utils.rs` items, 3 for Button variants the app does not use. An app with the three blocks prints 229.
- `dxui add button` over an edited `button.rs` keeps the file and prints "Added button". 0.3.0 changed 97 template items; a copy user cannot see which of them their files lack.
- `dxui add dashboard login settings` fails (one name per call), and `dxui --version` is an unknown command.
- `cargo-semver-checks` against 0.2.0 found new props fields that the draft 0.3.0 notes did not list; no release step runs it.
- Dioxus: the newest release is 0.8.0-alpha.1 (2026-07-31); the 0.7.10 notes do not mention the iOS 27 launch failure.

## Scope Rules

- A task lands complete: CLI behavior, registry, templates, docs, and tests together. What cannot meet this is cut, not stubbed.
- Apps that copied 0.3.0 templates keep building after updating the CLI; anything they must change goes in the 0.4.0 CHANGELOG Migration section.
- No new components or blocks without an issue or a block that needs them.
- Publishing 0.4.0 needs the release owner's confirmation.

## M202 Copied Code

- DONE M202.1 Split the utils template along crate module boundaries
  - An RFC first. Each shared helper (class composition and attributes, anchored overlay, listbox, roving group, modal focus, dismiss timer, hover open, media query, submenus and menu marks, dialog labels) becomes its own template and registry entry, and each component depends on the ones it uses; `dxui add` copies dependencies transitively. The parity test compares each helper template with its crate module.
  - Exit: `dxui add button` copies `button.rs` and the base helper only; the generated fixture app with every component and block builds; an app holding a 0.3.0 `utils.rs` still builds after adding a component.
  - Done: RFC 0074; 13 helper templates with entries in `helpers/`, and a registry test that each template's `super::` imports are dependencies and each helper dependency is imported (reverse-verified both ways). `dxui add button` copies `button.rs` and a 27-line `utils.rs`; a new app prints 3 warnings (Button and density variants) instead of 66. The fixture app builds; the 0.3.0 copy app from the registry check builds after adding dropdown, tooltip, and dialog, and `dxui add` prints a note about its old `utils.rs`, since old and new helper copies can pick the same element ids.
- DONE M202.2 Copied components build without warnings
  - Exit: a new app that adds and uses one component builds with `RUSTFLAGS="-D warnings"`, checked for Button and for an overlay component in the generated fixture script, and the fixture app with every component and block builds with it too. Reverse-verify by reintroducing an unused helper.
  - Done: after M202.1 the remaining warnings were variants, props, and `pub use` re-exports the app does not use, which no split removes, so `dxui init` writes `#![allow(dead_code, unused_imports)]` at the top of `ui/mod.rs` and `dxui add` keeps it (and any other non-`mod` line, which it used to drop). The fixture script denies warnings with `#![deny(warnings)]` (same effect as `-D warnings` on the crate's own code, without rebuilding dependencies): the library fixture with every component and block, with the header removed, and a new app binary using Button, Dialog, and Popover, with it. Reverse-verified: an unused private helper in `button.rs` fails the library; the app without the header fails.

## M203 CLI for Copy Users

- DONE M203.1 Several names per add, and a version flag
  - `dxui add button dialog dashboard` adds each, components and blocks alike, and stops at the first unknown name before writing anything; `dxui --version` and `-V` print the version.
  - Done: names are checked against components and blocks before `init`, which used to run first and write the stylesheet and `ui/mod.rs` even for an unknown name; tests cover several names with a block, an unknown name in the middle leaving the root absent (reverse-verified by writing first), and both version flags.
- DONE M203.2 Report kept files and show template differences
  - `dxui add` reports each file as written, unchanged, or kept because it differs from the template, and suggests `dxui diff`. `dxui diff <name>...` prints a unified diff between the app's copies and the CLI's templates and exits 1 when any differ, so CI can check it.
  - Done: `add` and `diff` share one dependency resolution that returns entries as data; a file missing from the app also counts as a difference for `diff`. Unified diffs come from the `similar` crate (MIT/Apache-2.0, no further dependencies; `cargo deny` passes). Tests cover written, unchanged, kept, and overwrite statuses and a diff that fails on an edited and a missing copy. On the 0.3.0 copy app, `dxui add button sidebar` reports utils.rs and sidebar.rs as kept, and `dxui diff sidebar` exits 1 with their diffs.

## M204 0.4.0 Release

- DONE M204.1 Prepare 0.4.0
  - CHANGELOG with migration notes, versions bumped, `Cargo.lock` on the latest Dioxus 0.7. `cargo-semver-checks` against 0.3.0 becomes a step in `docs/release.md`, and its findings are in the notes. Release gate and publish dry run pass.
  - Done: versions 0.4.0; CHANGELOG [0.4.0]; `cargo update` within semver, Dioxus on 0.7.10 (newest 0.7; crates.io lists only 0.8 alphas above it). `cargo-semver-checks --release-type minor` against 0.3.0: 202 checks pass for each library crate (without `--release-type` every lint is skipped, which `docs/release.md` now says). `npm run verify:release` exits 0 (6m23s) and `cargo publish --workspace --dry-run` verifies all four crates.
- DONE M204.2 Publish 0.4.0
  - Only after the release owner confirms; then build fresh apps in both modes from crates.io.
  - Done: confirmed on 2026-10-06 once CI passed on `d064c6b`; the four crates are on crates.io at 0.4.0 in dependency order, tagged `v0.4.0`. The published `dxui` 0.4.0 copied Button, Dialog, Popover, and the dashboard block into a fresh app that built while denying warnings, and `dxui diff` reported every copy matching; a crate-mode app built against `dioxus-shadcn` 0.4.

## M205 0.4.1 Diff Everything

- DONE M205.1 `dxui diff` without names checks every copied entry
  - With no names, `dxui diff` reads the modules declared in `src/components/ui/mod.rs` and `src/blocks/mod.rs`, keeps those that name a component or block (helpers come in as dependencies, app modules are skipped), and diffs them; with nothing copied it says so and exits 1. `dxui add` without names still fails.
  - Exit: tests cover an app with an edited and a matching copy, an app module that is skipped, and an empty app; reverse-verify by dropping the blocks file.
  - Done: `copied_entries` reads both `mod.rs` files and keeps declared modules that name a component or block; `add` checks for a name itself now that option parsing allows none. A test covers an empty app, a skipped `my_widget` module, matching copies of Button and the dashboard block, and an edited block; it fails when the blocks file is not read. In a scratch app with Sidebar and the login block, an edited `utils.rs` is reported with exit 1.
- DONE M205.2 Publish 0.4.1
  - Confirmed by the release owner on 2026-10-06 once M205.1 is done and CI passes. CHANGELOG, versions, release gate, `cargo-semver-checks` against 0.4.0, then publish and check `dxui diff` from crates.io in a fresh app.
  - Done: CHANGELOG [0.4.1], versions 0.4.1; `npm run verify:release` exits 0, `cargo-semver-checks --release-type patch` against 0.4.0 reports no update required for each library crate, and the dry run verifies all four. CI passed on `311d6c3`; the four crates are on crates.io at 0.4.1, tagged `v0.4.1`. The published `dxui` 0.4.1 diffed a fresh app with Button, Dialog, and the dashboard block without names (14 entries match, exit 0) and exited 1 after an edit to `dialog.rs`.

## M206 Server-Rendered Element Ids

- DONE M206.1 Check element ids across server rendering and hydration
  - Components take element ids from process-wide counters, which grow across requests on a server and restart at 0 in the browser; Popover and menus find their anchor by id. Build a fullstack app, render two requests, open a Popover after hydration, and record whether it anchors. Fix if it fails, or document the verified case if it does not.
  - Done: reproduced on Dioxus 0.7.9 fullstack with Tabs (Popover anchors to an app-given id, so Tabs' roving keys were the observable case): three requests wrote `dxui-roving-group-1`, `-2`, `-3`, hydration keeps server attribute values (`dioxus-web` `hydrate.rs`), and ArrowRight did nothing; the same app rendered in the browser only moved focus. Fixed with `next_element_id()` on a counter in the root context, replacing 24 process-wide counters (RFC 0075; `ScopeId` was rejected since reused slots could keep old id-matching scripts alive). Copy mode gets an `element-id` helper on 23 entries. An SSR test renders a page twice and expects the same HTML (fails on the old counters); after the fix three requests write the same ids and ArrowRight moves focus and selection after hydration. Feature check, fixture smoke, clippy, and `npm run verify:release` pass. Not published: a release needs the owner's confirmation.

## M207 0.4.2 Release

- TODO M207.1 Publish 0.4.2
  - Confirmed by the release owner on 2026-10-06 after M206.1. CHANGELOG, versions, release gate, `cargo-semver-checks --release-type patch` against 0.4.1, CI, then publish and check from crates.io: a fullstack app on `dioxus-shadcn` 0.4.2 keeps the same ids across requests, and a copy app from `dxui` 0.4.2 gets the `element_id` helper.

## Deferred (re-evaluate when)

- Dioxus 0.8: when `cargo search dioxus --limit 1 --color never | grep -qE '^dioxus = "0\.8\.[0-9]+"'` exits 0 (a 0.8 release, not a pre-release). Checked 2026-10-06: exits 1 on 0.8.0-alpha.1.
- iOS 27 launch failure (Dioxus 0.7 lacks the UIScene lifecycle): when `gh release view -R DioxusLabs/dioxus --json body -q .body | grep -qi uiscene` exits 0.
- Form state and validation, chart tooltips, swipe gestures, DOM portal, Command fuzzy ranking, editing an Input OTP slot in the middle, and right-to-left Slider, Resizable, and Calendar keys: when `gh issue list -R yuxuetr/dioxus-ui --state all --search "<topic>" --json number -q length` prints more than 0, or a block needs one.
- More blocks: when an issue asks for a screen, by the same command.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
