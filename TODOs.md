# TODOs

## Progress

- Overall: 60% (15 of 25 tasks)
- Current milestone: M221 (0.6.2, hardening)
- Current task: M221.4; M216 waits for the Stage 14 gate, and M218 and M219 follow `v0.7.0`

## Backup

- Completed plans: `docs/archive/TODOs.completed-20260628192208.md` (M1 to M29), `docs/archive/TODOs.completed-20261005.md` (M30 to M175), `docs/archive/TODOs.completed-20261005-m176-m180.md` (M176 to M180), `docs/archive/TODOs.completed-20261005-m181.md` (M181), `docs/archive/TODOs.completed-20261005-m182.md` (M182), `docs/archive/TODOs.completed-20261005-m183.md` (M183), `docs/archive/TODOs.completed-20261005-m184.md` (M184), `docs/archive/TODOs.completed-20261005-m185.md` (M185), `docs/archive/TODOs.completed-20261005-m186.md` (M186), `docs/archive/TODOs.completed-20261006-m187-m194.md` (M187 to M194, 0.2.0), `docs/archive/TODOs.completed-20261006-m195-m201.md` (M195 to M201, 0.3.0), `docs/archive/TODOs.completed-20261006-m202-m207.md` (M202 to M207, 0.4.0 to 0.4.2), `docs/archive/TODOs.completed-20261007-m208-m212.md` (M208 to M212, 0.5.0), and `docs/archive/TODOs.completed-20261007-m213-m214.md` (M213 to M214, 0.6.0)
- Previous roadmap: `docs/archive/roadmap-20261006-stages-0-10.md`

## Goals

- Follow [the roadmap](docs/roadmap.md) to 1.0. This plan covers 0.6.1 (M220, page scripts without eval), 0.6.2 (M221, hardening from the same audit), Stage 14 (0.7.0, Dioxus 0.8), and Stage 15 (1.0). M215 and M217 need no Dioxus release; M216 starts when the Stage 14 gate in Deferred exits 0, M218 when `v0.7.0` is tagged, and M219 when outside feedback on an `rc` is in and closed.

## Evidence (measured 2026-10-07 at `v0.6.0`, in a scratch worktree on `dioxus` and `dioxus-ssr` `=0.8.0-alpha.1`)

- Stage 14 gate exits 1: the newest Dioxus is `0.8.0-alpha.1` (2026-07-31), after `0.8.0-alpha.0` (2026-05-19); no release date is announced.
- No source change is needed on the alpha: the lockfile resolves one Dioxus (`0.8.0-alpha.1`); `cargo test --workspace --all-features` passes (all suites, 115 primitives and 333 styled tests); `cargo clippy --workspace --all-targets --all-features -D warnings` is clean, including the site and the Web, Desktop, and Mobile demos; the generated fixture with every template and block passes `cargo check` under `#![deny(warnings)]`.
- The browser checks fail on `dx` 0.8.0-alpha.1 as written: it enables Rust hot-patching unless told `--hot-patch false`, and the fat-binary link fails (`rust-lld: error: unknown file type: .../libdeps-*.a`), while `dx` 0.7.9 takes `--hot-patch` as a bare flag that is off by default. `dx` 0.8 also answers 200 with its build placeholder page before the app is built, so `serveDioxusWeb().ready()` returns early and the failure surfaces as a 30 s `page.goto` timeout instead of the build error. With `--hot-patch false`, `npm run verify:runtime-interactions` passes all 52 fixtures on the alpha.
- Not run on the alpha: the Desktop and Mobile self-tests, `verify:site`, `verify:preview`, and a fullstack hydration check.
- The fullstack hydration check exists only as a manual step: the M206 app (RFC 0075, `docs/release.md` 0.4.2 Publish) was run by hand, and Stage 14's exit names it.
- Five files pin or link Dioxus 0.7 outside the archive, RFCs, and CHANGELOG: `Cargo.toml`, `README.md`, `docs/quality-gates.md`, `docs/workspace.md`, `scripts/generated-fixture-smoke.sh`.
- Users: 67 to 130 downloads per crate, no issues.

## Stage 15 Evidence (measured 2026-10-07 at `086e7de`)

- `verify:semver` lets an `rc` break the API: in a scratch crate, removing a public function between `1.0.0-rc.1` and `1.0.0-rc.2` is checked as a "major change" and passes ("no semver update required"); with `--release-type minor` the same change fails (exit 100). Stage 15's "no breaking change during the `rc` period" has no gate yet.
- The templates are covered by the crate check: the template parity test compares every template item with the crate module token by token (`CRATE_ONLY`: 1 entry), so a template API change is a crate API change.
- No crate declares `rust-version`. The published crates need Rust 1.85 (edition 2024; the highest dependency floor is 1.85 in both the `dioxus-shadcn` and `dxui` trees), and CI builds only on stable (1.99 locally), so nothing shows whether the code itself needs more.
- No place for outside feedback: no `.github/ISSUE_TEMPLATE`, no labels for it, and the docs do not say what 1.x promises.
- Users: 67 to 130 downloads per crate, 0 issues. The roadmap's exit needs an app outside this repository on an `rc`; only the community call can bring one.

## M220 Evidence (measured 2026-10-08 at `9bc826b`)

- An app migrating to the crate serves `script-src 'self' 'wasm-unsafe-eval'` and reported that the interactive components fail there (FB-02, with FB-13 and FB-14): 15 modules start a page script with `document::eval`, which `dioxus-web` 0.7.10 runs through `Function::new_with_args`; `0.8.0-alpha.1` does the same. Dialog, Dropdown, Command, and Navigation Menu block its migration.
- A probe app on Dioxus 0.7.10 under that policy: `document::eval` is refused, and a `#[wasm_bindgen(inline_js = ...)]` function runs in a release build and under `dx serve`. Design and the rest of the probe: [RFC 0080](docs/rfcs/0080-page-scripts-without-eval.md).
- The preview app the browser checks serve uses eval itself: `document::Title` in `examples/web-demo/src/bin/preview.rs` and the `lang` eval in `PreviewSurface`.

## M221 Evidence (checked 2026-10-08 at `9bc826b`)

- The same app's audit (FB-01 to FB-16) found no script injection path. Confirmed in the code:
  - FB-03: no component checks a URL's scheme. `href` reaches the DOM unchanged in `BreadcrumbLink`, `NavigationMenuLink`, the `Pagination` links, `HoverCardTrigger`, `SidebarMenuButton`, `MenuItem`, and `DockItem`, so a `javascript:` URL from app data runs on a click. `AvatarImage` takes `src`, which cannot run script.
  - FB-06: a disabled `NavigationMenuLink` keeps its `href` (`navigation_menu.rs:462`); Menu, Pagination, and Sidebar already drop it (`(!disabled).then_some(href)`).
  - FB-04: `theme_init_script` writes `storage_key` into an inline script with `{:?}` (`theme_controller.rs:114`), which escapes quotes but not `</script>`.
  - FB-05: the theme controller and the init script set any stored value as `data-theme` (`theme_controller.rs:63`, `:82`).
  - FB-07: the anchored overlay script closes on every Escape on `document` without checking `defaultPrevented`, and Dialog, Sheet, Drawer, and Alert Dialog close on any Escape that bubbles, so one Escape in a popover inside a dialog closes both. Submenus already stop theirs (`listbox.rs:106`).
  - FB-08: the Sidebar shortcut (Ctrl/Cmd+B) fires and prevents default inside inputs and editable content, which takes bold from a rich-text editor.
  - FB-11: `dxui` writes with `fs::write` and `create_dir_all` and never checks for symlinks, so a symlinked `src/components` writes outside `--root`.
  - FB-01: `dxui init` writes the crate's `@source` as an absolute path from `cargo metadata`, which breaks when the app builds in another directory, such as Docker or CI.
  - FB-15: Table rows and footer use `border-b` and `border-t` without `border-border`, and the stylesheet has no base border color, so Tailwind 4 draws them in the text color.
  - FB-10: no doc says what the components do and do not sanitize.
- Not taken: FB-09 (Button keeps the native `submit` type in a form on purpose, documented at `button.rs:93`; M221.6 states it), FB-12 (the template parity test already compares every template with its crate module), FB-16 (the class-merge report runs in debug builds only, at `debug` level, once per distinct message).

## Scope Rules

- A task lands complete: crate, templates, registry, docs, site, and tests together. What cannot meet this is cut, not stubbed.
- M215 changes scripts, examples, and docs only; it needs no crate release.
- The Dioxus 0.8 move is the only breaking change in 0.7.0, with a Migration note in the CHANGELOG.
- No new components or blocks without an issue or a block that needs them.
- Publish 0.7.0 once every task before M216.3 is done (the release owner's direction of 2026-10-07).
- During the `rc` period only bug and docs changes land; a needed breaking change ends the period (another `rc` after it, and the exit's "no breaking change" restarts).
- Posting the community call is outward-facing: the release owner posts it or approves the exact text first.
- M220 changes no public API and ships as 0.6.1 from `main`; M216 then moves the same code to Dioxus 0.8.

## M220 0.6.1 Page Scripts Without Eval (RFC 0080)

- DONE M220.1 Strict-CSP browser run
  - `npm run verify:csp` serves the web preview with `script-src 'self' 'wasm-unsafe-eval'` (the served page's own inline scripts allowed by hash) and runs the runtime interaction checks, failing on any `securitypolicyviolation` or page error. The preview gets `lang` and its title from a web demo `index.html` instead of an eval and `document::Title`, so what fails is the components.
  - Exit: it fails on the current components and names the eval; without the policy the same run passes. Not yet in CI or `verify:release` (it stays red until M220.4).
  - Done (eeaef10): `--csp` mode of the interaction checks: a route serves each document with the policy plus hashes of its inline scripts (dx serve's toast script), an init script records `securitypolicyviolation`, and refusals and page errors fail the run and are appended to the failure that a refused script causes. On the current components it fails at the theme toggle with `script-src refused eval (.../wasm/preview.js:1134)` and the wasm `unreachable` panic; without the policy `verify:browser-local` passes all four checks, including the title checks against the `Dioxus.toml` title. axe is now evaluated, not added as an inline script tag.
- DONE M220.2 `script` helper
  - `component_script!` and `Script` (RFC 0080) in `dioxus-shadcn`, with `serde` and, on `wasm32`, `serde_json` and `wasm-bindgen`; the `script.rs` template and `script` helper entry; `dxui add` names the crates the app's `Cargo.toml` lacks when it writes the helper; the generated fixture declares them.
  - Exit: unit tests for the eval source and the export check; in a browser test page a script receives what Rust sends, its messages arrive in order, and `recv` returns `Finished` when it ends; the parity test and the generated fixture smoke pass.
  - Done (6876e90): `script.rs` in the crate and as a template, `helpers/script.json`, and `serde` plus `wasm32` `serde_json` and `wasm-bindgen` in `dioxus-shadcn`. The helper had no user without a component, so Checkbox's script moved onto it here (out of M220.4); its browser check is the "receives what Rust sends" page: with `send` made a no-op the `indeterminate` check fails, restored it passes. Ordered messages and `Finished` were shown in the RFC probe and come with M220.3's scripts that send. Each macro module carries a `source_exports_run` test, and `eval_source` has a unit test. The parity test keys a macro invocation by macro and first token (`component_script!(checkbox_indeterminate_script)`), and changing the template's script alone fails it; its class lint now ignores an indented `#[cfg(test)]`. `dxui add` reads the app's dependencies with `cargo metadata --no-deps` (canonical paths, since macOS names `/var` through a symlink) and prints the missing lines; the fixture smoke declares them, checks that nothing is named, checks that an app without a manifest is told `wasm-bindgen = "0.2"`, and runs `cargo check --target wasm32-unknown-unknown` on the fixture. `verify:browser-local`, the Desktop interaction self-test, Clippy, `cargo deny`, the feature check, and the docs checks pass.
- DONE M220.3 Overlay, focus, and roving scripts on `Script`
  - Modal Focus, Anchored Overlay, Listbox, Hover Open, Dismiss Timer, and Roving Group, in the crate and the templates, with their entries listing `script`. These carry Dialog, Sheet, Drawer, Alert Dialog, Popover, Dropdown, Select, Tooltip, Command, Combobox, Hover Card, Toast, Tabs, Toggle Group, and Accordion.
  - Exit: `verify:csp` passes the checks of those components; `verify:browser-local`, the Desktop interaction self-test, and the fullstack hydration check pass.
  - Done (4872447): the six scripts became `component_script!` modules (body wrapped in `export async function run(dioxus)`, call sites start the module and send as before); Modal Focus receives `[scopeId, locksScroll]` instead of `__SCOPE_ID__` and `__LOCK_SCROLL__`, so its string-building helper and its test went away. The six helper entries list `script`, `mod script` is built for every feature that uses one, and the old raw-string comparison in the registry test lost the six entries the parity test now covers. `verify:browser-local` (52 interaction fixtures, which run these scripts as snippets), the Desktop interaction self-test (the eval transport: dialog, popover, select, dropdown, toast, date picker, menubar, navigation menu), the fullstack hydration check (Tabs and Select after hydration), Clippy on both targets, and the workspace tests pass. `verify:csp` cannot yet show these components: a script still on eval runs at mount, its refusal panics the wasm module (`unreachable`), and the page stops before the first fixture; it is measured with M220.4.
- DONE M220.4 The other scripts on `Script`
  - Checkbox, Input OTP, Media Query, Menubar, Navigation Menu, Resizable, Sidebar, Slider, and Theme Controller; no `document::eval` is left in the crate or the templates. `verify:csp` joins `verify:release` and CI.
  - Exit: `verify:csp` passes; a `document::eval` put back into one component fails it (reverse-verify); `grep -rn 'document::eval' crates/dioxus-shadcn/src crates/dioxus-shadcn-cli/templates` prints nothing; the Desktop self-test and the browser checks pass.
  - Done (2019138, with b7e9694): the eight modules (nine scripts; Checkbox went in M220.2) run on `component_script!`. `Script::recv` takes `&self` (on Desktop it copies the `Eval` handle), so the theme controller keeps one `Rc<Script>` for later sends and its receive loop, as it kept a copyable `Eval`. `verify:csp` passes all 52 fixtures under the strict policy and is in `verify:release` (so CI) and the release and quality-gate docs; with `document::eval("void 0")` put back into Checkbox it fails with `script-src refused eval`. The grep prints only `script.rs`, whose eval is the Desktop and Mobile transport. `verify:browser-local`, the Desktop interaction self-test, the fullstack hydration check, the generated fixture smoke (wasm32 included), Clippy on both targets, the feature check, and the docs checks pass. On the way: `mod script`'s feature list in M220.3 had been read too widely (a regex spanning several `cfg` blocks) and now lists the 27 features whose modules start a script. Two faults in the browser checks were found and fixed in b7e9694: a `dx serve` left by a crashed run kept port 45239 and answered later runs with its older build (now `serveDioxusWeb` refuses a port that already answers), and `dx serve` 0.7.9 holds the first `Accept: text/html` request after a build open for over 90 s (the CSP route retries with a 5 s limit).
- DONE M220.5 CSP docs
  - README and `docs/component-api.md`: the policy the components need, the theme init script's hash or nonce, the copy-mode crates, and that app code using `document::eval` or `document::Title` still needs `'unsafe-eval'`. CHANGELOG `[Unreleased]`.
  - Exit: the docs checks pass.
  - Done (dfc3cc1): README "Content Security Policy" (the policy, the theme init script, app eval and `Title` with `[web.app] title` as the alternative, and Dioxus fullstack's per-request inline hydration scripts, which carry no nonce in 0.7.10 (`dioxus-server` `ssr.rs:736`; measured: three inline scripts on the fullstack example's page)), the copy-mode crate lines next to the `dxui add` output (whose file list was refreshed from a real run; it had been missing six helpers), "Page Scripts" in `docs/component-api.md`, a CSP note on the theme controller page, and CHANGELOG `Fixed` and `Changed`. The docs, changelog, and README checks pass.
- DONE M220.6 Publish 0.6.1
  - Versions, CHANGELOG, release gate with `verify:semver` against `v0.6.0`, publish in dependency order, annotated tag `v0.6.1`, then a fresh web app from crates.io served with the strict policy opens a Dialog and switches Tabs with no violation.
  - Done (3d04850, tag `v0.6.1`; record 79024b5): the release gate's `verify:csp` first failed because `dx serve` held the page request (three tries, then with `Accept: */*` the app never started), so the check now serves a `dx build` from a static server (5058245); the rest of the gate, `verify:semver` against `v0.6.0` (no change), the dry run, and CI on 3d04850 passed. `cargo publish --workspace` timed out waiting for the index after `dioxus-shadcn-core`; primitives, CLI, and `dioxus-shadcn` were then published one by one. A fresh app on crates.io 0.6.1 under the strict policy opened a Dialog (focus inside, Escape closed it) and moved Tabs with ArrowRight with nothing refused; on 0.6.0 the same check fails with the refused eval. The published `dxui` 0.6.1 copied Dialog and Tabs, named the three missing crates, and the app built under `#![deny(warnings)]` and passed the same check. Recorded in `docs/release.md` (0.6.1 Publish).

## M221 0.6.2 Hardening (starts when `git tag -l v0.6.1 | grep -q .` exits 0)

- DONE M221.1 URL props allow safe schemes only
  - One helper keeps relative URLs, fragments, and `http:`, `https:`, `mailto:`, and `tel:`, and turns anything else into no `href`; every component in FB-03 uses it, in the crate and the templates. A disabled `NavigationMenuLink` drops its `href` (FB-06).
  - Exit: unit tests for the schemes (including `JavaScript:`, leading spaces, and control characters); an SSR test renders each FB-03 component with `javascript:alert(1)` and finds no such `href`.
  - Done (df77955): `safe_href` (crate module and `safe-url` helper template) strips surrounding spaces and control characters and every tab and line break, reads a scheme as a letter followed by letters, digits, `+`, `-`, or `.`, and keeps the URL when it has no scheme or one of `http`, `https`, `mailto`, `tel` (case-insensitive). Unit tests keep 11 forms (empty, `#`, `/a:b`, `HTTP://`, `mailto:`, `tel:`) and drop 8 (`JavaScript:`, leading spaces, `\u{1}`, a tab or newline inside `javascript`, `data:`, `vbscript:`). The SSR test renders all seven components with `javascript:alert(1)` plus a disabled Navigation Menu link and finds no `javascript:` and no `/retired`; with Breadcrumb passing `href` through it fails. Disabled links that drop `href` also get `role="link"`, as in Menu. The workspace tests, parity, Clippy, the feature check, the generated fixture smoke, and `verify:browser-local` pass.
- DONE M221.2 Theme storage
  - `theme_init_script` refuses a `storage_key` outside `[A-Za-z0-9_-]` (FB-04); the controller and the init script apply a stored theme only when it is `system`, `light`, `dark`, or a preset the app lists, else `system` (FB-05).
  - Exit: unit tests for both; the theme controller browser check passes, and a stored `x"><script>` leaves `data-theme` unset.
  - Done (226e562): rather than refusing a key, `theme_init_script` writes it as a JavaScript string in which everything but ASCII letters, digits, `-`, and `_` is a `\u` escape, so every key still works and none can close the script (unit test: `</script><script>alert("x")//` plus U+2028 leaves no `<` and no U+2028). Rather than a list of the app's presets, which needs a new `ThemeController` prop, both scripts apply a stored value only when it matches `^[A-Za-z0-9_-]{1,64}$` (every preset name does), so a stored `x"><script>` is ignored: the interaction check stores it, remounts the controller, and finds `data-theme-choice` still `system` and no `data-theme`; without the check in the controller script it reports `x\"><script>`. The site's `index.html` carries the new init script (its test compares it). The workspace tests, parity, Clippy, and `verify:runtime-interactions` pass.
- DONE M221.3 One Escape closes one layer
  - The anchored overlay and Navigation Menu scripts skip an Escape whose `defaultPrevented` is set and prevent the one they act on; Dialog, Sheet, Drawer, and Alert Dialog skip a prevented Escape (FB-07). The Sidebar shortcut ignores inputs, `select`, and editable content (FB-08).
  - Exit: browser checks: Escape in a Popover inside a Dialog closes only the Popover, a second Escape closes the Dialog; Ctrl+B in a text input leaves the Sidebar as it was. Both fail before the change.
  - Done (ec8784f): the `defaultPrevented` route does not work: Dioxus handles `onkeydown` at its root element, before the scripts' `document` listeners run, so the Dialog closes before a popover could mark the event. Instead a `layer` helper (crate and template) keeps the open overlays of a virtual DOM in opening order; Dialog, Sheet, Drawer, and Alert Dialog close on Escape only when they are the last opened (`Layer::is_top`), and anchored overlays register so a dialog under them waits. Anchored overlays themselves keep their Escape: with the check applied to them too, the dropdown check "Escape with a hovered submenu closes the whole menu" failed, and menus already share Escape through their scripts. Navigation Menu was left out: its script reports every close the same way, and it is not modal. The Sidebar shortcut skips a prevented event and targets inside `input`, `textarea`, `select`, or editable content. A nested-layers fixture (Dialog with a Popover) joins the preview (53 fixtures). With `is_top` always true the first Escape closes the Dialog too, and without the editing check Ctrl+B in "Contact email" collapses the Sidebar; both checks fail that way and pass with the change. The workspace tests, the feature check, the fixture smoke, `verify:browser-local`, the Desktop self-test, and `verify:csp` pass.
- TODO M221.4 `dxui` writes stay in the app
  - Every write and directory creation refuses a symlink on the path under `--root` (FB-11); `@source` is written relative to the stylesheet when the crate is under the app's directory or Cargo home is shared, with the absolute path kept and a note otherwise (FB-01).
  - Exit: CLI tests: a symlinked `src/components` fails with the path named and writes nothing; `@source` in a fixture is relative and Tailwind still finds the crate's classes (`verify:css-inputs`).
- TODO M221.5 Table border color
  - Table row, header, and footer borders use `border-border` (FB-15), crate and template.
  - Exit: a rendered DOM check reads the row border color as the `--border` token in both themes.
- TODO M221.6 Security notes
  - A Security section in the README and `docs/component-api.md`: the URL schemes components keep, that children and rich content are not sanitized, the CSP the components need (RFC 0080), the theme storage key rule, and Button's native `submit` type in a form (FB-09, FB-10).
  - Exit: the docs checks pass.
- TODO M221.7 Publish 0.6.2
  - Versions, CHANGELOG, release gate with `verify:semver` against `v0.6.1`, publish in dependency order, annotated tag `v0.6.2`.

## M215 Dioxus 0.8 Readiness

- DONE M215.1 Browser checks on both `dx` lines
  - `serveDioxusWeb` turns hot-patching off in the form the installed `dx` accepts (a bare flag stays off on 0.7; `--hot-patch false` on 0.8), and `ready()` waits until the server answers with the app rather than the CLI's build placeholder, failing with `dx`'s output when the build fails.
  - Exit: `npm run verify:runtime-interactions` passes with `dx` 0.7.9 on the main tree and with `dx` 0.8.0-alpha.1 on a scratch 0.8 tree; a build that fails under `dx` 0.8 makes `ready()` throw with the build error instead of a `page.goto` timeout (reverse-verify by leaving hot-patching on).
  - Done (f005db4): both `dx` lines answer 200 with a placeholder ("dx is not serving a web app") before the first build, so `ready()` now waits for the page that loads `/wasm/`; `dx --version` picks `--hot-patch false` on 0.8 and nothing on 0.7. The screenshot, rendered DOM, and mobile browser smokes dropped their own `dx serve` copies for `serveDioxusWeb`. All four browser checks (`verify:browser-local`) pass on `dx` 0.7.9 and, from a cold `dx` output directory, on `dx` 0.8.0-alpha.1 with Dioxus 0.8.0-alpha.1; with hot-patching left on, `ready()` fails after 5 s with `dx serve could not build` and dx's `Build failed` output.
- DONE M215.2 Fullstack hydration check in the release gate
  - A fullstack example renders Tabs and one more component with generated ids on the server; a script serves it, compares the ids across three requests, hydrates the page in Chromium, and checks that ArrowRight moves focus between tabs. `npm run verify:release` and CI run it; the Deferred entry for it is removed.
  - Exit: the check passes on Dioxus 0.7, and fails when `next_element_id()` is replaced with a process-wide counter (RFC 0075's failing case).
  - Done (2b9c64e): `examples/fullstack-hydration` renders Tabs and Select; `npm run verify:fullstack-hydration` compares 19 generated ids across three requests, then after hydration ArrowRight moves focus to and selects the second tab and the Select list opens under its trigger, with no console errors. It runs in `verify:release` (which passes) and in CI, which installs `wasm32-unknown-unknown` and `dioxus-cli` at the `Cargo.lock` Dioxus version through cargo-binstall. `deny.toml` allows `webpki-roots` (CDLA-Permissive-2.0) and `xxhash-rust` (BSL-1.0), which only `dioxus-fullstack` brings. With a process-wide counter in `next_element_id()`, request 2 writes `dxui-tabs-10-*` against request 1, and with the request check skipped the hydrated ArrowRight no longer moves focus.
- DONE M215.3 `npm run verify:dioxus-next`
  - A script copies the tree into a scratch worktree, pins `dioxus` and `dioxus-ssr` to the newest 0.8 pre-release on crates.io, and runs the workspace tests, Clippy, the generated fixture smoke, the fullstack check, and the browser interactions with a `dx` of the same version, which it names when it is missing. `docs/release.md` records each run's result.
  - Exit: it passes on `0.8.0-alpha.1`, fails with the compiler error when a template is given a type error (reverse-verify), and leaves the main tree unchanged.
  - Done (e2a6a85): the newest 0.8 version comes from the crates.io versions list; a `dx` of another version fails with an install hint and `DIOXUS_NEXT_DX`; builds go to `<target>/dioxus-next` so dx never serves a 0.7 bundle. On `0.8.0-alpha.1` all five steps pass (recorded in `docs/release.md`, Dioxus Next). A type error added to the Button template alone fails the template parity test; added to the crate module and the template, it fails the workspace tests with `error[E0308]`. Each run removes its worktree and leaves `git status` unchanged.

## M216 0.7.0 Dioxus 0.8 (starts when the Stage 14 gate exits 0)

- TODO M216.1 Move to Dioxus 0.8
  - Workspace, `dioxus-ssr`, the generated fixture, examples, site, and the five files that pin or link 0.7 move to the 0.8 release; fix what `verify:dioxus-next` reports on it.
  - Exit: Stage 14's criteria: the release gate, browser checks, Desktop and Mobile self-tests, and the fullstack hydration check pass on Dioxus 0.8.
- TODO M216.2 Prepare 0.7.0
  - CHANGELOG with the Dioxus 0.8 Migration note and any API change 0.8 forced, versions, release gate (with `verify:semver` against `v0.6.0`), publish dry run.
- TODO M216.3 Publish 0.7.0
  - Push, CI, publish in dependency order, annotated tag `v0.7.0`, then build fresh apps in both modes from crates.io on Dioxus 0.8 that use a component, a class override, and a state callback.

## M217 1.0 Readiness (needs no Dioxus release)

- DONE M217.1 `verify:semver` holds `rc` versions to no breaking change
  - When the workspace version is a pre-release of the same major as the baseline tag, or the baseline is a pre-release and the version is its release (`1.0.0-rc.N` to `rc.N+1` or to `1.0.0`), run cargo-semver-checks with `--release-type minor`; other bumps keep the derived type.
  - Exit: in a scratch worktree tagged `v1.0.0-rc.1`, removing a public item and bumping to `1.0.0-rc.2` fails the gate and an added item passes; `0.6.0` to `0.7.0` still checks as a minor bump before 1.0 (breaking allowed).
  - Done (014320c): `semver-verify.mjs` reads the `[workspace.package]` version and passes `--release-type minor` when the major is at least 1, equals the tag's, and either side is a pre-release. In a local clone tagged `v1.0.0-rc.1`: removing a public function at `rc.2` fails ("1.0.0-rc.2 must not break the API of v1.0.0-rc.1"), adding one passes, and removing one at `1.0.0` fails; from `v0.6.0`, a new public field passes at `0.7.0` and at `1.0.0-rc.1` and fails without a bump. A deliberate break between `rc`s needs `SEMVER_RC_BREAK=1` (the Scope Rules' restart), which lets it through. The main tree still passes against `v0.6.0`.
- DONE M217.2 State the 1.x promise and the Rust floor
  - A "Compatibility" section (README and `docs/release.md`) says what 1.x keeps: the public API of the three library crates, the templates through parity, the `dxui` commands and flags, and how a new Dioxus line or Rust floor is released. `rust-version` is set to the lowest Rust that builds the published crates, and CI checks them with that toolchain.
  - Exit: `cargo +<floor> check` of the four published crates passes in CI, and one Rust below fails locally (reverse-verify); the docs checks pass.
  - Done (025ea13, a31c17d): the floor is 1.88, not 1.85: Dioxus 0.7.10's `const-serialize-macro` uses let chains, so 1.87 fails with E0658 inside it, and 1.88 builds all four crates with the locked dependencies. `rust-version = "1.88"` is in `[workspace.package]`, each published crate inherits it (the publish metadata check requires that), and CI's "Check the published crates on the Rust floor" step reads it from `cargo metadata` and passes. On 1.87 cargo now stops with "requires rustc 1.88". Clippy's `incompatible_msrv` then flagged `floor_char_boundary` (1.91) in the parity test, fixed in a31c17d. `docs/release.md` and the README have a Compatibility section: the three crate APIs, templates through parity, the `dxui` commands; a new Dioxus line is a new major version, and a minor release may raise the Rust floor.
- DONE M217.3 Feedback intake
  - GitHub issue forms for a bug and for `rc` feedback (crate version, crate or copy mode, platform, Dioxus version, what was built), an `rc-feedback` label, and a triage rule in `docs/release.md`: fix in the next `rc`, or defer with a reason in Deferred.
  - Exit: a test issue filed through the form gets the label and is closed with the triage note, and the M219 gate command counts it only when it is not the owner's.
  - Done (16ffdd8): `.github/ISSUE_TEMPLATE` has `bug.yml` (label `bug`), `rc-feedback.yml` (label `rc-feedback`: rc version, mode, platform, Dioxus version, what was built, what got in the way), and `config.yml`; all three validate against the SchemaStore issue-form schemas, and a dropdown without options fails them. The `rc-feedback` label exists. `docs/release.md` "Release Candidate Feedback" gives the three closing notes and the two gate commands. GitHub's GraphQL `issueTemplates` lists only Markdown templates, so the forms were checked by schema, not by filing through the web form: test issue #1 was filed with the label and the form's sections and closed as Deferred, and the gate then counts 1 `rc-feedback` issue, 0 from outside, 0 open (the search index lagged about 20 s after closing).

## M218 1.0.0 Release Candidates (starts when `git tag -l v0.7.0 | grep -q .` exits 0)

- TODO M218.1 Publish `1.0.0-rc.1`
  - Versions, CHANGELOG (no API change from 0.7.0), release gate with `verify:semver` against `v0.7.0`, publish in dependency order, annotated tag, then fresh apps in both modes from crates.io.
- TODO M218.2 Call for feedback
  - Draft the call (what 1.0 promises, how to try the `rc`, where to report) in `docs/release.md`; the release owner posts it in the Dioxus community. Each later `rc` repeats M218.1 with only bug and docs changes.

## M219 1.0.0 (starts when outside `rc` feedback exists and none is open)

- TODO M219.1 Publish 1.0.0
  - Starts when `gh issue list -R yuxuetr/dioxus-ui --label rc-feedback --state all --json author -q '[.[]|select(.author.login!="yuxuetr")]|length'` prints more than 0 and the same query with `--state open` prints 0.
  - Release from the last `rc` with no API change (`verify:semver` passes under M217.1's rule), CHANGELOG entry closing the `rc` period, publish, annotated tag `v1.0.0`, roadmap Stage 15 marked complete.

## Deferred (re-evaluate when)

- Dioxus 0.8 (Stage 14, M216): when `cargo search dioxus --limit 1 --color never | grep -qE '^dioxus = "0\.8\.[0-9]+"'` exits 0 (a 0.8 release, not a pre-release). Checked 2026-10-07: exits 1 on 0.8.0-alpha.1.
- iOS 27 launch failure (Dioxus 0.7 lacks the UIScene lifecycle): when `gh api repos/DioxusLabs/dioxus/releases --jq '.[0:10][].body' | grep -qiE 'pull/5893|uiscene'` exits 0. The fix is DioxusLabs/dioxus#5893 (open since 2026-10-05, against `v0.7`), titled "launch with the iOS 27 SDK (... scene manifest ...)" without the word UIScene, so the old check (`gh release view` body contains `uiscene`) would have missed its release; it also read only the newest release. Checked 2026-10-07: exits 1; with a released PR number (5570) in place of 5893 it exits 0.
- Form state and validation, chart tooltips, swipe gestures, DOM portal, Command fuzzy ranking, editing an Input OTP slot in the middle, and right-to-left Slider, Resizable, and Calendar keys: when `gh issue list -R yuxuetr/dioxus-ui --state all --search "<topic>" --json number -q length` prints more than 0, or a block needs one.
- More blocks: when an issue asks for a screen, by the same command.
- Generating templates from the crate: when `CRATE_ONLY` in `crates/dioxus-shadcn-cli/tests/template_parity.rs` passes 10 entries, or M210 needs a template difference the parity rules cannot express. Checked 2026-10-07: 1 entry.

## Status Rules

- Change `TODO` to `DONE` only after implementation, validation, and commit.
- Preserve task wording unless the user explicitly requests planning changes.
- One task at a time: implement, validate, commit, then update TODO status in a separate commit.
- Source-copy components must remain self-contained and must not import internal crates.
