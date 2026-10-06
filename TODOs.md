# TODOs

## Progress

- Overall: 96% (22 of 23 tasks)
- Current milestone: M201
- Current task: M201.2 (waiting for the release owner)

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), `docs/archive/TODOs.completed-20261005-m186.md` (M186), and `docs/archive/TODOs.completed-20261006-m187-m194.md` (M187 to M194, 0.2.0)

## Goals

- 0.3.0 takes the library from components to screens: menus, overlays, and layouts complete enough for a real app shell, docs that show the code, and copyable blocks built from the components.
- Maintenance costs less per change: one source of truth between crate modules and templates, and a release gate that checks the product instead of the finished first publish.

## Evidence (measured 2026-10-06 at `v0.2.0`)

- Adoption: 18 crates.io downloads, no stars, no issues. There is no outside demand signal yet, so priorities come from the library's own documented gaps and from building screens with it.
- Docs: component pages on the site have Preview and Code tabs per example, but no copy button, and link to GitHub markdown for props. The Installation page still shows a `dioxus-shadcn-0.1.0` source path. (Corrected 2026-10-06: the first assessment missed the Code tab.)
- Menus: Context Menu and Menubar have checkbox, radio, and shortcut items; Dropdown, the most used menu, has none. The release notes say menu submenus are not implemented, which shadcn/ui's Dropdown, Context Menu, and Menubar all have.
- Overlays and layout: the release notes exclude scroll lock (the page scrolls behind an open Dialog, Sheet, or Drawer), Sidebar mobile off-canvas and keyboard shortcut, multi-thumb sliders, and a pagination range helper.
- Themes: the release notes exclude a system color scheme default; the site forgets the chosen theme on reload. daisyUI ships a Theme Controller for this.
- daisyUI components with no counterpart and a use beyond layout: Theme Controller, Menu (vertical, nested), and Mockup (browser, window, code, phone).
- Template drift: all 21 commits between `v0.1.0` and `v0.2.0` that changed `crates/dioxus-shadcn/src` also had to edit `templates/`. The copies already differ in API, not only imports: `BadgeVariant` derives `Default` in the crate and not in the template.
- Release gate: about 95 steps. 43 of the 67 files in `docs/` and about 15 verifiers check the first publish's readiness (blockers, decision packets, handoffs), which was resolved on 2026-10-05.

## Scope Rules

- Every new component or block lands complete: crate module and feature (components), template, registry entry, docs page, site example, SSR tests, and a runtime check when it is interactive. What cannot meet this is cut, not stubbed.
- Prefer new props, parts, and components over changing existing ones. Breaking changes go in the 0.3.0 CHANGELOG Migration section.
- Retiring a gate requires that what it guarded is either finished or still checked by another gate; record which in the commit.
- Publishing 0.3.0 needs the release owner's confirmation.

## M195 Maintenance Groundwork

- DONE M195.1 Retire first-publish readiness docs and gates
  - Archive the readiness, blocker, decision, handoff, and follow-up docs under `docs/archive/`; remove their verifiers from `verify:release` and `package.json`; keep `docs/release.md`, the changelog, package contents, publish order, and Cargo metadata checks. Record the release gate's duration before and after.
  - Done: 46 docs in `docs/archive/first-publish/` with an index, 12 verifiers removed. The internal dependency version check moved into `verify:cargo-publish-metadata` and `verify:package-contents` now requires `LICENSE`, both reverse-verified. `verify:release`: 122 npm steps in 48.35 s before, 98 in 40.47 s after (cached builds). `docs/site.md` went from 4304 to 3457 lines.
- DONE M195.2 Template parity
  - Measure crate-to-template drift after normalizing imports, tests, and formatting; fix the API drift found; RFC 0066 decides between a parity gate and generating templates from the crate, from that measurement. The chosen check is reverse-verified with an injected divergence.
  - Done: RFC 0066 chose a gate: `tests/template_parity.rs` compares items with `syn` (160 differing items as committed, 97 after formatting and normalization). The drift included user-visible bugs: Select and Combobox templates lacked the 0.2.0 check mark, `CalendarDate::unchecked` (used by the docs) was missing, and 25 enums lacked `Default`. Templates are rustfmt-formatted; the crate's let chains became nested ifs. Generation is re-evaluated when `CRATE_ONLY` passes 10 entries. Reverse-verified four ways.

## M196 Menus

- DONE M196.1 Dropdown checkbox, radio, and shortcut items
  - Same parts and behavior as Context Menu: `DropdownCheckboxItem`, `DropdownRadioGroup`, `DropdownRadioItem`, `DropdownShortcut`; runtime check.
  - Done: the four parts, `inset` on `DropdownItem`, and a shared `menu_marks` helper. Context Menu and Menubar checkable items drew no mark at all (checked state only in `aria-checked`); all three menus now draw a check or dot. Runtime fixture reverse-verified; site example checked in a screenshot in both themes.
- DONE M196.2 Menu submenus
  - RFC: `*Sub`, `*SubTrigger`, and `*SubContent` for Dropdown, Context Menu, and Menubar; ArrowRight opens (ArrowLeft in right-to-left), ArrowLeft and Escape close one level, hover opens with a short delay. Runtime check for each menu; the Desktop menu scenarios still pass.
  - Done: RFC 0067. Hover opens at once without focus, as Radix does, rather than after a delay; no pointer grace area. The shared listbox script now scopes items by their closest `[data-dxui-listbox]`, so Select, Combobox, and Command run the changed script too and their checks still pass. Runtime checks for all three menus, reverse-verified; RTL checked in a scratch run.

## M197 Overlays and Layout

- DONE M197.1 Scroll lock for modal overlays
  - Dialog, Alert Dialog, Sheet, and Drawer stop page scroll while open and restore it, scrollbar width compensated, nested modals counted. Runtime check of `scrollY` under an open dialog.
  - Done: RFC 0068; the lock lives in the modal focus scope script behind a flag (Date Picker passes false). Runtime check reverse-verified; the nested count is exercised only by code review, since no fixture nests modals.
- DONE M197.2 Sidebar off-canvas and shortcut
  - Below a breakpoint the Sidebar opens as a Sheet; an opt-in keyboard shortcut (Ctrl/Cmd+B in shadcn/ui) toggles it. Runtime check at phone width.
  - Done: RFC 0069; a `use_media_query` helper, and an opt-in off-canvas mode whose wrapper is a modal dialog below 768px (axe rejects a dialog role on `aside`). The viewport is known after the first render, so phones show the wide layout for one frame. Runtime check at both widths, reverse-verified.
- DONE M197.3 Range slider
  - Two thumbs on one track with `value: (f64, f64)` and a minimum gap, keyboard and pointer, without changing `Slider`'s API. Runtime check.
  - Done: RFC 0070; `RangeSlider` with each thumb a WAI-ARIA slider bounded by the other, and `range_slider_values` with unit tests. Runtime check reverse-verified; site example "Price range".
- DONE M197.4 Pagination range helper
  - `pagination_range(current, total, siblings)` returning pages and ellipses; unit tests, docs, and the site example switched to it.
  - Done: constant-length rows with an ellipsis only for gaps of two or more pages (the first draft hid a single page behind one; a test caught it). The old site example never showed pages 4 to 9.

## M198 daisyUI Components

- DONE M198.1 Theme Controller
  - Sets `data-theme` and the dark class on the document, defaults to the system color scheme, remembers the choice, and applies it before first paint. The site's theme menu uses it.
  - Done: RFC 0071; `ThemeController` and `theme_init_script` (the site's new `index.html` carries it). The header menu replaced the dark toggle with System, Light, and Dark entries; site-verify checks reload persistence and the system scheme.
- DONE M198.2 Menu
  - Vertical navigation list with titles, active items, and nested collapsible groups.
  - Done: RFC 0072 (shared with M198.3); a `ul` list rather than `role="menu"`, controlled `MenuGroup`s. Runtime check reverse-verified; screenshot in both themes.
- DONE M198.3 Mockup
  - Browser, Window, Code, and Phone frames.
  - Done: RFC 0072; five parts with hidden decorations, static (no runtime fixture), screenshots in both themes. The frames use semantic tokens, so the terminal and phone invert in the dark theme.

## M199 Site Documentation

- DONE M199.1 Copy example source on the site
  - The existing Code tab gets a copy button, as do the install commands. Fix the stale `0.1.0` path on the Installation page. (Narrowed 2026-10-06: Preview and Code tabs already exist.)
  - Done: copy buttons on all code blocks with a status announcement; versions derived from `CARGO_PKG_VERSION`. Clipboard check in site-verify, reverse-verified. Also found and cancelled a Pages run stuck in the queue for 16 hours, which had blocked every deploy since 2026-10-05 11:58; the live site is current again.
- DONE M199.2 API reference on the site
  - Each component page renders its docs page's API, behavior, and accessibility sections instead of linking to GitHub.
  - Done: rendered at build time by `site/build.rs` (pulldown-cmark build dependency); component links stay on the site under the runtime base path. site-verify checks every page and a cross link.

## M200 Blocks

- DONE M200.1 Block registry and CLI
  - RFC: a block is a registry item of kind `block` that copies one screen into `src/blocks/` and adds the components it uses; `dxui list` and `dxui add` handle blocks; a generated fixture app builds every block.
  - Done: RFC 0073 chose a separate `blocks/` directory over a `kind` field, so component tooling and `RegistryComponent` are unchanged; `dxui list blocks` keeps `dxui list` script-stable. The login block (planned for M200.3) landed here to exercise the CLI end to end, and showed `FieldError` could not take an `id`.
- DONE M200.2 Dashboard block
  - Sidebar (with off-canvas), header, stats, chart, and data table.
  - Done: `DashboardBlock` with search and sortable orders; compiles in the fixture app. The visual and interaction check is on the site in M200.4.
- DONE M200.3 Login and settings blocks
  - A sign-in form and a settings page with tabs and fields.
  - Done: the login block came with M200.1; `SettingsBlock` with tabs, checked fields, switches, and save and reset compiles in the fixture app.
- DONE M200.4 Blocks on the site
  - A Blocks page with a full-width preview, source, and the `dxui add` command for each block, audited in both themes and every preset.
  - Done: index and per-block pages compiled from the copied sources through a `components::ui` shim; site-verify audits them (dashboard with every preset) and exercises each block. Screenshots found two library bugs, fixed in their own commits: every chart drew upside down (`ChartScale` normalized its range), and an off-canvas Sidebar did not stretch on wide screens.

## M201 0.3.0 Release

- DONE M201.1 Prepare 0.3.0
  - CHANGELOG with migration notes, versions bumped, `Cargo.lock` on the latest Dioxus 0.7, release gate and publish dry run pass.
  - Done: versions 0.3.0, snippets at 0.3, `[0.3.0]` notes with three migration entries, Dioxus 0.7.10 (works with the 0.7.9 CLI in every gate). `cargo publish --workspace --dry-run` packages and verifies all four crates. `cargo search` shows Dioxus 0.8.0-alpha.1, a pre-release, so the 0.8 item stays deferred.
- TODO M201.2 Publish 0.3.0
  - Only after the release owner confirms.

## Deferred (re-evaluate when)

- Form state and validation library: when the settings block needs more than `Field` with per-field errors.
- Chart tooltips and hit testing: when the dashboard block's chart needs exact values on hover.
- Swipe gestures for Carousel, Toast, and Drawer: when touch input can be checked by a script rather than the Mobile checklist.
- DOM portal: when an anchored overlay is clipped in a runtime check; fixed positioning has not been clipped so far.
- Dioxus 0.8: when `cargo search dioxus` lists a 0.8 release.
- Command fuzzy ranking, editing an Input OTP slot in the middle, and right-to-left Slider, Resizable, and Calendar keys: when an issue asks for them.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
