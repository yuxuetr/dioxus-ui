# TODOs

## Progress

- Overall: 73% (8 of 11 tasks)
- Current milestone: M212 (0.5.0 release)
- Current task: M212.1 (M208.3, the 0.4.3 publish, waits for the release owner)

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), `docs/archive/TODOs.completed-20261005-m186.md` (M186), `docs/archive/TODOs.completed-20261006-m187-m194.md` (M187 to M194, 0.2.0), `docs/archive/TODOs.completed-20261006-m195-m201.md` (M195 to M201, 0.3.0), and `docs/archive/TODOs.completed-20261006-m202-m207.md` (M202 to M207, 0.4.0 to 0.4.2)
- Previous roadmap: `docs/archive/roadmap-20261006-stages-0-10.md`

## Goals

- Follow [the roadmap](docs/roadmap.md) from 0.4.2 to 1.0. This plan covers Stage 11 (0.4.3) and Stage 12 (0.5.0); Stages 13 to 15 get tasks once 0.5.0 ships, since their scope depends on the 0.5.0 API.

## Evidence (measured 2026-10-06 at `v0.4.2`)

- Adoption: 18 to 30 downloads per crate and no issues or pull requests, so breaking changes cost nothing yet and cost much more after 1.0.
- Crate mode: `crates/README.md` tells apps to write `@source "/path/to/dioxus-shadcn-0.4.2/src"`. After a crate upgrade the line still names the old version's directory, which stays in Cargo's cache, so Tailwind keeps scanning old source and misses classes the new version added.
- Theme: the known limitations in `docs/release.md` said Checkbox marks are white images that follow only the default light and dark `--primary-foreground`. Corrected in M208.2: the marks have followed `--primary-foreground` since 0.2.0.
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
- DONE M208.2 Checkbox marks follow the theme
  - The check and indeterminate marks take the color of the token the checked box uses for its foreground, in the crate and the template, instead of fixed white and dark images.
  - Exit: `npm run verify:theme-presets` checks mark contrast against the checked background for every preset and fails on the old white mark under a light-primary preset such as `cupcake`; the browser check sees the mark color change with `data-theme`.
  - Done: no code change needed. The premise came from a stale known limitation: since 0.2.0 (`87f4ffb`) the marks are masks filled with `--primary-foreground` in the crate and the template. The browser check in `verify:runtime-interactions` already asserts the mark color equals `--primary-foreground` under light, dark, and an overridden token, and `verify:theme-presets` checks `primary-foreground` on `primary` for all 33 presets. Corrected the known limitation in `docs/release.md`.
- TODO M208.3 Publish 0.4.3
  - After M208.1 and M208.2 and the release owner's confirmation: CHANGELOG, versions, release gate, `cargo-semver-checks --release-type patch` against 0.4.2, CI, publish, then check both modes from crates.io.
  - Cut from the local `release/0.4` branch (`117c4a4`), since `main` carries the 0.5.0 class merge; bumping it to 0.4.3 matches the crate README's `@source` example, then merge the `[0.4.3]` notes back into `main`.

## M209 User Class Overrides (RFC 0076)

- DONE M209.1 Choose the merge by the ground-truth gate
  - Build the RFC 0076 corpus from the utilities `verify:tailwind-conflicts` enumerates, compute expected removals with `utilityConflicts`, and run `tw_merge` and a generated merge table against it. Measure merge cost per render over the site's component pages. Record results and the choice in RFC 0076; no component changes.
  - Exit: the gate script exists and fails for the three RFC 0076 reverse cases; RFC 0076 lists each candidate's false removals, missed removals, unknown tokens, and cost.
  - Done: `scripts/class-merge-gate.mjs` (ground truth from Tailwind and Chrome, cascade model checked in Chrome, three reverse merges rejected) and `examples/class-merge-gate`. The table generated by `scripts/class-merge-table.mjs` passed: 0 false removals, 0 lost overrides, every Tailwind utility in the corpus classified, 0.92 µs per call with `mt-4`. `tw_merge` failed: 196 false removals (`group/item` removes `group`, `border-b-primary` removes `border-b`, `text-sm` removes `leading-6`, typos remove colors) and 484 lost overrides. The gate also reports 412 partial overlaps no merge can fix (logical over physical sides). Building the corpus found `verify:tailwind-conflicts` blind to token colors; fixed in `5876777` with the Menu active text it hid.
- DONE M209.2 Merge user classes in every component
  - One merge helper used by every class function in the crate and the templates; unknown user tokens left in place and reported in debug builds; the gate and the browser check join `npm run verify:release`. Update the `class` rules in `docs/component-api.md`, mark the RFC 0044 override rules superseded, update the known limitations, and drop the important modifier from the fixtures.
  - Exit: gate and browser check pass; a parity test or registry test fails if a class function appends a user class without the helper; the generated fixture app builds while denying warnings.
  - Done: `merge_classes` in `dioxus-shadcn-core` and the `class-merge` and `class-merge-table` helpers (brought by `utils`); 720 class function call sites in crate and templates. The table stores names reversed and the generator fails if Tailwind would generate CSS from a copied helper (it first added 2,673 lines to the preview CSS). The gate, now on the shipped merge with arbitrary values on every root, passes: 0 false removals, 0 lost overrides, every named Tailwind utility classified, 0.58 µs per call with `mt-4`. `class_functions_merge_the_user_class` and the preview's `overflow-auto` check fail when a site appends again. `verify:tailwind-conflicts` now sees single-word utilities and found the Message Scroller jump button that never hid. `verify:release`, `verify:browser-local`, and `verify:site` pass.

## M210 Component-Owned State

- DONE M210.1 RFC and prototype on Select and Tabs
  - An RFC for roots that own state through context: `default_value` for uncontrolled use, `value` with `on_value_change` for controlled use, `open` likewise for overlays, and ids and anchors from `next_element_id()`. Parts read the root; a part outside its root fails to compile or is documented as unsupported, not silently inert. Prototype Select and Tabs in the crate and templates.
  - Exit: the Select and Tabs doc examples have no `anchor_id`, no per-item `selected` or `active`, and one `open`; existing browser checks for both pass unchanged in behavior; SSR renders the same ids twice; the RFC lists every stateful component and the batch it moves in.
  - Done: RFC 0077; `Select` root (value or values, open, optional trigger `id` for a `Label`) and root-owned `Tabs`, through `use_controllable` in `root_state` (crate) and the `root-state` helper (copy mode). Parts outside their root panic with a message naming it; Dioxus 0.7 logs it and renders nothing for the part, which the tests assert. Site, settings block, and fixtures migrated; the fixtures cover controlled and uncontrolled. `verify:release`, `verify:browser-local`, and `verify:site` pass. The multiple Select browser check now waits for the list to be ready before its first key, as the single Select check did. Not run: the Desktop self-test fails at its `theme` scenario on this machine at `v0.4.2` too, because the system appearance is Dark, so no later scenario ran in Desktop; the iOS and Android self-tests were not run.
- DONE M210.2 Move the overlay components
  - The overlays in the RFC's batch list (dialogs, sheets, drawers, popovers, menus, tooltips, hover cards, combobox, date picker, and the rest the RFC names) take the root-owned API.
  - Exit: their browser and SSR checks pass; the site and blocks use the new API; parity test passes.
  - Done: roots with `open`, `default_open`, and `on_open_change` for Dialog, Alert Dialog, Sheet, Drawer, Popover, Tooltip, Hover Card, Fab, Dropdown, Context Menu, and Date Picker, through a shared `overlay_root` helper; new trigger parts (unstyled buttons that take `class`, recorded in RFC 0077) and a `ContextMenuTrigger` area. Menubar and Navigation Menu own the open item's `value`; menu radio groups (`menu_radio`) and submenus own their state; Combobox owns value(s) and open through a `choice` helper shared with Select. Date Picker keeps the date with the app and menu checkbox items keep `checked` (RFC keep table). Fixed in passing: `Controllable::get` returned a stale controlled value in event handlers (c294e5b; its message wrongly blames the menubar flake, which was a missing focus wait, fixed in 72ee72f). `verify:release`, `verify:runtime-interactions`, and `verify:site` pass; Desktop, iOS, and Android self-tests not run (Desktop blocked as in M210.1).
- DONE M210.3 Move the group and disclosure components
  - The groups in the RFC's batch list (accordion, collapsible, radio group, toggle group, menubar, navigation menu, carousel, and the rest the RFC names) take the root-owned API.
  - Exit: as M210.2, and no crate module still takes a per-part state prop the RFC did not keep on purpose.
  - Done: Accordion (value or values with `multiple`; `Choice::toggle` clears a single choice), Collapsible (overlay root, `aria-controls` only while the content renders), and `MenuGroup` (`default_open`) in c34336d; Radio Group and Toggle Group in 3227879; Carousel (`count`, `index`, items and indicators by `index`), Command (highlight already in its script; per-part highlight props removed), and Input OTP (`length`, code on the root, slots by `index`) in 015a68a; Sidebar gained `SidebarProvider` with `off_canvas` (1571a15), added to the RFC batch table since the trigger and sidebar duplicated four props. The RFC keep table now also names Rating, the Command and Combobox query, and toast `open`; a resurvey of the crate finds no other per-part state prop. Fixed in passing: roving-group tests did not build with radio-group or toggle-group alone (30bba72); 3227879 dropped the toggle-group template's inlined primitives (bc95e22, caught by generated-fixture-smoke); per-feature clippy had been checked through a grep that ANSI colors blinded, and a clean rerun passes for every feature. `verify:release`, `verify:runtime-interactions` (one reverse check went red), and `verify:site` pass; Desktop, iOS, and Android self-tests not run.

## M211 Density Decision

- DONE M211.1 Measure touch targets and decide
  - In the Mobile self-test (iOS Simulator, RFC 0018), record the rendered size of every interactive control at defaults. If any is below 44 by 44 CSS pixels, density comes from a root context and the interactive components take it; otherwise `UiDensity` and Button's `density` prop are removed. Either way the design docs say what was measured.
  - Exit: the measurement table is in the decision record; the chosen path is implemented in crate and templates with tests, or removed with a Migration note.
  - Done: on an iPhone 17 Pro simulator (iOS 26.3) 124 of 134 controls were under 44 by 44 at defaults (table in RFC 0078, f2cf88e), so density comes from `DensityProvider` and the interactive components read it (1ef4575): a Touch height floor for controls with text, a centered `after:` hit area for controls drawn small, Button without its `density` prop. The Mobile preview renders under Touch and the Mobile self-test fails on any control under 44 by 44 (inline text links exempt); it reports 0 of 131 now and went red at 52, 17, and 1 while classes were missing. `verify:release`, `verify:runtime-interactions`, and `verify:mobile-interactions` pass; Desktop and Android self-tests not run.

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
