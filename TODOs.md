# TODOs

## Progress

- Overall: 9% (1 of 11 tasks)
- Current milestone: M208 (0.4.3)
- Current task: M208.2

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), `docs/archive/TODOs.completed-20261005-m186.md` (M186), `docs/archive/TODOs.completed-20261006-m187-m194.md` (M187 to M194, 0.2.0), `docs/archive/TODOs.completed-20261006-m195-m201.md` (M195 to M201, 0.3.0), and `docs/archive/TODOs.completed-20261006-m202-m207.md` (M202 to M207, 0.4.0 to 0.4.2)
- Previous roadmap: `docs/archive/roadmap-20261006-stages-0-10.md`

## Goals

- Follow [the roadmap](docs/roadmap.md) from 0.4.2 to 1.0. This plan covers Stage 11 (0.4.3) and Stage 12 (0.5.0); Stages 13 to 15 get tasks once 0.5.0 ships, since their scope depends on the 0.5.0 API.

## Evidence (measured 2026-10-06 at `v0.4.2`)

- Adoption: 18 to 30 downloads per crate and no issues or pull requests, so breaking changes cost nothing yet and cost much more after 1.0.
- Crate mode: `crates/README.md` tells apps to write `@source "/path/to/dioxus-shadcn-0.4.2/src"`. After a crate upgrade the line still names the old version's directory, which stays in Cargo's cache, so Tailwind keeps scanning old source and misses classes the new version added.
- Theme: Checkbox marks are white images that follow only the default light and dark `--primary-foreground` (known limitations, `docs/release.md`).
- Overrides: 352 call sites in 81 crate modules append the user class, and a user class does not win by position (RFC 0076). The maintainer decided on 2026-10-06 that the last class wins and that the merge must not cause hidden bugs.
- State: the Select doc example passes `open` twice, matches `anchor_id` to the trigger id by hand, and computes `selected` per item; Tabs computes `active` per trigger and per panel. 25 crate modules take `open: bool` and 25 take per-part `active`, `selected`, `checked`, or `pressed`.
- Density: `UiDensity` is public, and Button is the only component of 82 that takes it.
- Dioxus: the newest release is 0.8.0-alpha.1.

## Scope Rules

- A task lands complete: crate, templates, registry, docs, site, and tests together. What cannot meet this is cut, not stubbed.
- No hidden failures: code that drops, rewrites, or ignores what an app passed must be checked against ground truth (Tailwind's output, a browser) with reverse checks, and must leave the input unchanged when it cannot decide. A wrong result that fails loudly is acceptable; one that fails quietly is not.
- Breaking changes go to 0.5.0 only, each with a Migration note in the CHANGELOG; 0.4.3 stays compatible and passes `cargo-semver-checks --release-type patch`.
- No new components or blocks without an issue or a block that needs them.
- Each publish needs the release owner's confirmation.

## M208 0.4.3 Crate-Mode Setup and Theme Marks

- DONE M208.1 `dxui` keeps the crate's `@source` line current
  - In an app that depends on `dioxus-shadcn`, `dxui init` writes the `@source` line for the resolved crate's `src` from `cargo metadata`, and running it again after an upgrade replaces a line that names another version. An app that does not depend on the crate gets no line. `crates/README.md` drops the manual `cargo metadata | jq` step.
  - Exit: tests cover a new line, a replaced stale line, a current line left alone, and an app without the crate; in a scratch crate-mode app, bumping the version and rerunning `dxui init` makes a class only the new version uses appear in the compiled CSS. Reverse-verify by leaving the stale line.
  - Done: `dxui init` reads `cargo metadata` and replaces crate `@source` lines (`dioxus-shadcn/src` or `dioxus-shadcn-<version>/src`) in `assets/dioxus-shadcn.css`; an unreadable manifest fails the command. Unit tests cover the four cases plus line matching and this workspace's metadata. In a scratch app on 0.2.0, the stale line after bumping to 0.3.0 compiled without `rounded-b-xl` and `w-28` (0.3.0-only Mockup classes); after `dxui init` both appeared, and a rerun changed nothing.
- TODO M208.2 Checkbox marks follow the theme
  - The check and indeterminate marks take the color of the token the checked box uses for its foreground, in the crate and the template, instead of fixed white and dark images.
  - Exit: `npm run verify:theme-presets` checks mark contrast against the checked background for every preset and fails on the old white mark under a light-primary preset such as `cupcake`; the browser check sees the mark color change with `data-theme`.
- TODO M208.3 Publish 0.4.3
  - After M208.1 and M208.2 and the release owner's confirmation: CHANGELOG, versions, release gate, `cargo-semver-checks --release-type patch` against 0.4.2, CI, publish, then check both modes from crates.io.

## M209 User Class Overrides (RFC 0076)

- TODO M209.1 Choose the merge by the ground-truth gate
  - Build the RFC 0076 corpus from the utilities `verify:tailwind-conflicts` enumerates, compute expected removals with `utilityConflicts`, and run `tw_merge` and a generated merge table against it. Measure merge cost per render over the site's component pages. Record results and the choice in RFC 0076; no component changes.
  - Exit: the gate script exists and fails for the three RFC 0076 reverse cases; RFC 0076 lists each candidate's false removals, missed removals, unknown tokens, and cost.
- TODO M209.2 Merge user classes in every component
  - One merge helper used by every class function in the crate and the templates; unknown user tokens left in place and reported in debug builds; the gate and the browser check join `npm run verify:release`. Update the `class` rules in `docs/component-api.md`, mark the RFC 0044 override rules superseded, update the known limitations, and drop the important modifier from the fixtures.
  - Exit: gate and browser check pass; a parity test or registry test fails if a class function appends a user class without the helper; the generated fixture app builds while denying warnings.

## M210 Component-Owned State

- TODO M210.1 RFC and prototype on Select and Tabs
  - An RFC for roots that own state through context: `default_value` for uncontrolled use, `value` with `on_value_change` for controlled use, `open` likewise for overlays, and ids and anchors from `next_element_id()`. Parts read the root; a part outside its root fails to compile or is documented as unsupported, not silently inert. Prototype Select and Tabs in the crate and templates.
  - Exit: the Select and Tabs doc examples have no `anchor_id`, no per-item `selected` or `active`, and one `open`; existing browser checks for both pass unchanged in behavior; SSR renders the same ids twice; the RFC lists every stateful component and the batch it moves in.
- TODO M210.2 Move the overlay components
  - The overlays in the RFC's batch list (dialogs, sheets, drawers, popovers, menus, tooltips, hover cards, combobox, date picker, and the rest the RFC names) take the root-owned API.
  - Exit: their browser and SSR checks pass; the site and blocks use the new API; parity test passes.
- TODO M210.3 Move the group and disclosure components
  - The groups in the RFC's batch list (accordion, collapsible, radio group, toggle group, menubar, navigation menu, carousel, and the rest the RFC names) take the root-owned API.
  - Exit: as M210.2, and no crate module still takes a per-part state prop the RFC did not keep on purpose.

## M211 Density Decision

- TODO M211.1 Measure touch targets and decide
  - In the Mobile self-test (iOS Simulator, RFC 0018), record the rendered size of every interactive control at defaults. If any is below 44 by 44 CSS pixels, density comes from a root context and the interactive components take it; otherwise `UiDensity` and Button's `density` prop are removed. Either way the design docs say what was measured.
  - Exit: the measurement table is in the decision record; the chosen path is implemented in crate and templates with tests, or removed with a Migration note.

## M212 0.5.0 Release

- TODO M212.1 Prepare 0.5.0
  - CHANGELOG with a Migration note per breaking change from M209 to M211, versions, release gate, `cargo-semver-checks --release-type minor` against 0.4.3, publish dry run.
- TODO M212.2 Publish 0.5.0
  - Only after the release owner confirms; then build fresh apps in both modes from crates.io, including a class override and an uncontrolled Select.

## Deferred (re-evaluate when)

- Dioxus 0.8 (Stage 14): when `cargo search dioxus --limit 1 --color never | grep -qE '^dioxus = "0\.8\.[0-9]+"'` exits 0 (a 0.8 release, not a pre-release). Checked 2026-10-06: exits 1 on 0.8.0-alpha.1.
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
