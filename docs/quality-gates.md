# Quality Gates

This document defines local and CI verification commands for `dioxus-shadcn`.

## Default Local Gate

Run before committing implementation changes:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
```

These commands should stay fast enough for regular development.

## Source-Copy Gate

Run after changes to CLI, registry, templates, or component dependencies:

```bash
cargo test -p dioxus-shadcn-cli --test registry
scripts/generated-fixture-smoke.sh
```

This verifies:

- public registry component names match `dioxus-shadcn` crate feature names.
- public registry components have docs pages and catalog entries.
- registry template source and target paths match generated module names.
- every template file is registered exactly once, including `utils`.
- `scripts/feature-check.sh` covers every public component feature.
- `dxui list` returns public components.
- `dxui init` creates the generated project structure.
- `dxui add` can add every public component.
- generated `mod.rs` includes every public component and `utils`.
- generated code does not import `dioxus-shadcn-core` or `dioxus-shadcn-primitives`.
- generated source compiles with only `dioxus = "0.7"`.

## Feature Gate

Run after changes to `crates/dioxus-shadcn/Cargo.toml`, crate exports, or feature
gating:

```bash
scripts/feature-check.sh
```

This verifies:

- every public `dioxus-shadcn` feature compiles independently.
- static component feature combinations compile.
- primitive-backed feature combinations compile.
- all `dioxus-shadcn` features compile together.

This command invokes Cargo many times and is intentionally separated from the
default local gate.

## Example Smoke Gate

Run after changes to Web or Desktop examples, component demo output, or expanded
parity coverage:

```bash
scripts/example-smoke.sh
```

This verifies that both command-line demo crates expose representative states
for low-risk composition, form-specific, message, scroller, direction,
collapsible, and chart components. It is not a screenshot or visual parity gate.

## Web Preview Gate

Run after changes to the rendered Web preview shell:

```bash
node scripts/web-preview-verify.mjs
```

This verifies the Dioxus Web preview binary, Tailwind CSS v4 source input, stable
`data-preview-*` screenshot targets, and representative shared inventory output.
It is the structural prerequisite for browser screenshots.

For the Playwright screenshot procedure, see
`docs/components/web-preview-screenshot-verification.md`.

## Mobile Web Profile Gate

Run after changes to the rendered Web preview shell, Mobile verification docs,
or mobile-profile source markers:

```bash
node scripts/mobile-web-profile-verify.mjs
```

This verifies that the Web preview exposes the shared `mobile-profile` panel,
keeps the documented `390x844` mobile viewport target, and preserves
source-level markers for touch targets, hover alternatives, safe-area ownership,
reduced-motion policy, and visible status text.

This is not a native Mobile gate. It does not verify software keyboard behavior,
safe-area inset measurement, mobile assistive technology, or native WebView
gesture arbitration.

## Release Gate

Run before publishing:

```bash
npm install
npm run verify:release
```

The release aggregate expands to the required local release gates:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
cargo test -p dioxus-shadcn-cli --test registry
cargo run -p dioxus-shadcn-cli -- list
npm run verify:cargo-workspace
npm run verify:cargo-publish-metadata
npm run verify:package-contents
npm run verify:publish-order
npm run verify:cargo-lock
npm run verify:pre-commit
npm run verify:scripts
npm run verify:gitignore
npm run verify:preview-state-metadata
npm run verify:mobile-browser-metadata
npm run verify:rendered-component-coverage
npm run verify:examples-metadata
npm run verify:css-inputs
npm run verify:registry
npm run verify:tailwind-static
npm run verify:tailwind-conflicts
npm run verify:preview-css
npm run verify:site-css
npm run verify:site-catalog
npm run verify:theme-presets
npm run verify
npm run verify:changelog
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:package-lock
npm run verify:ci-docs
npm run verify:ci-plan
npm run verify:ci-workflow-template
npm run verify:browser-artifact-policy
npm run verify:repo-hygiene
```

Use the smaller deterministic local alias before routine handoff when full
release checks are not needed:

```bash
npm run verify
```

The local alias does not require Playwright browser binaries and does not
replace feature, source-copy fixture, registry, or release documentation gates.
Future browser-rendered smoke commands should document
`npx playwright install chromium` separately.

`npm run verify` runs `npm run verify:smoke` and `npm run verify:docs`.

`npm run verify:release` runs Rust workspace checks, CLI registry/list smoke,
Cargo workspace metadata checks, Cargo lock metadata checks, pre-commit
metadata checks, script metadata checks, gitignore metadata checks, examples
metadata checks, CSS input metadata checks, `npm run verify`, component feature
checks, generated fixture smoke, release documentation consistency checks,
package script wiring checks, and CI browser documentation checks. It also
checks CI Plan documentation and repository hygiene.

`npm run verify:smoke` runs rendered preview structural checks and example smoke
output.

`npm run verify:preview` runs Web preview, Mobile Web profile, and Desktop
preview structural checks.

`npm run verify:web-preview` checks rendered Web preview screenshot
prerequisites without launching a browser.

`npm run verify:web-screenshot-smoke` starts the Web preview and checks
representative desktop and mobile screenshot targets in a real browser. It is
opt-in, requires Playwright Chromium or `DIOXUS_UI_BROWSER_EXECUTABLE`, and is
not part of default or release gates. It writes screenshots only when
`DIOXUS_UI_WEB_SCREENSHOT=1` is set, validates PNG metadata after capture, and
does not compare pixels, certify visual parity, update generated docs, change
component APIs, or edit templates.

`npm run verify:mobile-web-profile` checks source-level Mobile Web profile
coverage and fallback markers. It is not a native Mobile device or emulator
gate.

`npm run verify:desktop-preview` checks Desktop preview structural coverage
without launching a native WebView screenshot run.

`npm run verify:examples` runs the example smoke script for Web and Desktop demo
entry points.

`npm run verify:examples-metadata` checks that example Cargo manifests,
workspace membership, `examples/README.md`, preview verification scripts, and
example package aliases stay aligned. It is read-only and does not run examples,
compile generated fixtures, launch previews, install browser dependencies, or
rewrite documentation.

`npm run verify:css-inputs` checks that CLI default CSS and rendered preview CSS
inputs keep Tailwind CSS v4 syntax, the RFC 0051 semantic color token blocks,
required preview `@source` roots, everything the CLI stylesheet holds after its
import (the light and dark token blocks) copied verbatim into each preview
input, no redefined Tailwind palette variables, and no Tailwind CSS v3
directives. It is read-only and
does not compile Tailwind, inspect generated CSS output, launch previews, or
assert visual parity.

`npm run verify:docs` runs all docs metadata and Markdown drift checks. Use the
individual commands below when isolating a specific failure.

`npm run verify:docs-catalog` checks the docs-site catalog contract in memory
from registry entries, templates, component docs, crate features, and crate
modules. It also verifies every public component has static catalog grouping
metadata. It must not write generated catalog artifacts.

`npm run verify:docs-catalog-page` checks that
`docs/components/catalog.md` matches the Markdown rendered from the shared
catalog builder, including grouped sections and the full flat index.

`npm run verify:docs-status` checks that `docs/components/component-status.md` matches
the local implementation status derived from registry entries, templates, docs,
crate features, crate modules, source preview metadata, and source-copy targets.
It verifies wiring coverage, not live upstream shadcn/ui parity or runtime
visual parity.

`npm run verify:docs-structure` checks that every public component docs page
keeps the required title, Source Copy, Crate Feature, API Surface, and
Accessibility Notes structure. It is read-only and does not score prose quality,
validate rendered HTML, or assert runtime visual parity.

`npm run verify:docs-routes` checks that `docs/components/routes.md` matches the
route manifest rendered from the shared catalog builder.

`npm run verify:docs-source-preview` checks that
`docs/components/source-preview.md` matches the source preview manifest rendered
from the shared catalog builder.

`npm run verify:rfcs` checks that RFC filenames, first headings, numbering, and
README index links remain aligned. It is read-only and does not review RFC
prose, decide acceptance status, validate implementation status, render docs,
or rewrite index files.

`npm run verify:changelog` checks project-owned changelog structure,
Unreleased section, Keep a Changelog and Conventional Commits references, and
stale template-link bans. It is read-only and does not generate release notes,
run git-cliff, derive changes from Git history, create tags, publish releases,
or rewrite commit history.

`npm run verify:readme` checks that README verification shortcuts mention the
primary local, docs, smoke, and release aliases, focused metadata aliases, and
the direct `verify:docs` command segments from package metadata. It is
read-only and does not execute commands, crawl external links, render the docs
site, or rewrite README prose.

`npm run verify:docs-index` checks that README and docs/README keep required
quality, release, CI, site, RFC, and TODO entry points discoverable. It also
checks that this document lists every verification alias and that the release
gate block matches direct `verify:release` command segments in order. It is
read-only and does not crawl external links, expand nested aliases, execute
release commands, or generate navigation artifacts.

`npm run verify:docs-links` checks that tracked Markdown files do not reference
missing local relative file targets. It is read-only and does not validate
external URLs, heading fragments, or generated docs runtime routes.

`npm run verify:docs-anchors` checks that local Markdown fragments point to
target document headings or explicit anchors. It is read-only and does not
validate external URL anchors, rendered HTML anchors, or generated docs runtime
routes.

`npm run verify:release-docs` checks that release documentation still mentions
the local aggregate alias, the exact ordered expanded release gate block for
every direct `verify:release` command segment, source-copy fixture smoke,
feature checks, and opt-in browser smoke boundary. It is read-only and does not
generate release artifacts.

`npm run verify:registry` checks that registry entries have stable metadata,
existing sources, valid source-copy targets, known dependency references, and
well-formed asset mappings. It is read-only and does not execute CLI commands,
compile Rust crates, or replace CLI registry tests.

`npm run verify:tailwind-static` checks shipped Rust source and source-copy
templates for dynamic Tailwind utility token interpolation, for bare data
variants such as `data-disabled:`, which match any attribute value, and, since
M178, for Tailwind palette color utilities such as `bg-blue-600` outside test
modules; components use the RFC 0051 semantic tokens, and only the
`bg-black/50` modal overlay keeps a palette color. It is
read-only and does not compile Tailwind CSS, validate user-provided classes, or
assert visual parity.

`npm run verify:tailwind-conflicts` compiles each utility of the shipped class
functions with the Tailwind Node API and fails when a function joins a base
class with a state class, helper, or method class that sets the same property
under the same variant (see
[RFC 0044](rfcs/0044-tailwind-utility-conflicts.md)). It does not validate
user-provided classes or conflicts between two state classes.

`npm run verify:preview-css` compiles the preview stylesheet input and fails
when `examples/preview-states/assets/preview.generated.css`, which every
preview target links, differs from the result (see
[RFC 0049](rfcs/0049-compiled-preview-stylesheet.md)). Run
`npm run css:preview` to regenerate it after a class change.

`npm run verify:site-css` does the same for the component site
(see [RFC 0052](rfcs/0052-component-site.md)): it fails when
`site/assets/site.generated.css` differs from a fresh compile of
`site/assets/site.css`. Run `npm run css:site` to regenerate it.
`npm run verify:site-catalog` fails when `site/src/catalog.rs`, the site's
sidebar and page data, differs from what the docs catalog builder produces;
run `npm run site:catalog` to regenerate it. Both are read-only and do not
build or serve the site.

`npm run verify:theme-presets` checks the theme presets in
`crates/dioxus-shadcn-cli/themes/` and the default theme
([RFC 0057](rfcs/0057-theme-presets.md)): each preset sets exactly the default
theme's tokens under its `/* dxui theme: <name> */` marker and
`[data-theme="<name>"]` rule and keeps daisyUI's MIT notice, and every text
pair components render meets 4.5:1, computed from the OKLCH values. Run
`npm run theme:presets <daisyui themes dir>` to regenerate the presets.

`npm run verify:package-scripts` checks that `package.json` still exposes the
required verification aliases, that aggregate aliases reference the expected
focused commands, and that local `scripts/` targets exist. It is read-only and
does not execute release commands.

`npm run verify:package-lock` checks that committed `package-lock.json` root
metadata matches `package.json`, including package name, version, lockfile
version, and root devDependencies. It is read-only and does not run npm install,
contact the registry, or rewrite lockfiles.

`npm run verify:cargo-workspace` checks that committed Rust workspace package
metadata stays consistent across root and `crates/` manifests. It verifies
workspace inheritance and `cargo metadata` output without publishing crates,
packaging crates, contacting crates.io, or checking dependency freshness.

`npm run verify:cargo-publish-metadata` checks published crate
descriptions, shared README/keywords/categories metadata, example `publish =
false` boundaries, and release wiring. It is read-only and does not run
`cargo publish`, run `cargo package`, contact crates.io, replace repository URLs, or create package archives.

It also requires internal workspace dependencies to declare the workspace
version alongside their local paths, since published crates resolve them
from crates.io by that version.

[Public API Surface Inventory](public-api-surface-inventory.md) records the
component, primitive, core, feature, registry, and source-copy surfaces
accepted for first publish.

`npm run verify:package-contents` runs `cargo package --list` for each
publishable crate and checks that every package ships its manifest, README,
`LICENSE`, and entry point, and that the CLI package contains every registry entry and template the CLI embeds. It
lists package contents only; it does not build or compress package archives,
run `cargo publish`, or contact crates.io.

`npm run verify:publish-order` checks the planned crate publish order. It is
read-only and does not create package archives, run `cargo package`, run
`cargo publish`, contact crates.io, check registry ownership, inspect
credentials, change dependency versions, or authorize a release.

`npm run verify:cargo-lock` checks that committed `Cargo.lock` workspace package
entries match `cargo metadata --locked --no-deps` workspace members. It is
read-only and does not run `cargo update`, rewrite the lockfile, contact
crates.io, or check dependency freshness.

`npm run verify:pre-commit` checks that committed `.pre-commit-config.yaml`
local hook metadata and supporting config files remain wired. It is read-only
and does not install pre-commit environments, execute hooks, contact remote hook
repositories, or check remote hook freshness.

`npm run verify:scripts` checks committed script shebangs, executable bits, and
package-referenced script targets. It is read-only and does not execute scripts,
lint shell syntax, install dependencies, or rewrite file modes.

`npm run verify:gitignore` checks that `.gitignore` keeps required generated
directory, browser state, and screenshot artifact patterns aligned with
repository hygiene policy. It is read-only and does not delete files, inspect
ignored artifact contents, validate global excludes, or replace repository
hygiene tracking checks.

`npm run verify:preview-state-metadata` checks that the shared preview state
inventory, Web/Desktop preview binaries, structural preview verifiers, Tailwind
CSS v4 preview inputs, and release wiring stay aligned. It is read-only and
does not launch browsers, run `dx serve`, compile Tailwind output, inspect
screenshots, or assert visual parity.

`npm run verify:mobile-browser-metadata` checks that the opt-in mobile browser
smoke script, package alias, localhost target, `390x844` viewport, selector
contract, screenshot artifact pattern, browser environment variables, and CI
browser guidance stay aligned. It is read-only and does not launch Playwright,
start `dx serve`, install browsers, write screenshots, or validate rendered
output.

`npm run verify:rendered-component-coverage` checks stable rendered preview
target metadata for public components against the docs catalog and confirms
those `data-component-preview` ids are present in the shared preview state
source. It is read-only and does not start a server, launch a browser, write
screenshots, update generated docs, change component APIs, edit templates, or
claim visual parity.

`npm run verify:rendered-component-dom` starts the Web preview and checks every
rendered coverage target in a real browser DOM. It is opt-in, requires
Playwright Chromium or `DIOXUS_UI_BROWSER_EXECUTABLE`, and is not part of
default or release gates. It does not write screenshots or traces, assert
runtime interactions, update generated docs, change component APIs, edit
templates, or claim visual parity.

`npm run verify:runtime-interactions` starts the Web preview and exercises the
focused `data-interaction-*` fixture targets in a real browser. It is opt-in,
requires Playwright Chromium or `DIOXUS_UI_BROWSER_EXECUTABLE`, and is not part
of default or release gates. It checks representative click, keyboard, focus,
ARIA, visible text, and `data-state` transitions for disclosure, overlay,
selection, keyboard-visible state, and scroll-status behavior, plus real
Dialog, Alert Dialog, Popover, and Tooltip components: Escape and outside
dismissal, initial focus, Tab wrap, focus restore, and anchored placement with
flip; and real Toast and Sonner components: countdown dismissal, hover pause,
dismiss reasons, and viewport live regions; and real Select and Combobox
components: anchored listbox placement, arrow and typeahead navigation, and
selection with focus kept on the trigger or input; and a real Date Picker
with Calendar: focus entry, day keyboard movement across months, Tab wrap,
and focus return; and real Dropdown and Context Menu components: focus entry,
wrapping navigation, typeahead, activation with focus return, and point
placement; and a real Menubar: roving trigger focus, Left and Right and hover
menu switching, and focus return to the open menu's trigger; and a real
Navigation Menu: click toggling, top-level and content arrow movement, hover
open and close timing, and Escape, outside, and focus-out dismissal; and real
Tabs, Radio Group, and Toggle Group components: one Tab stop, arrow movement
past disabled items, selection following focus in Tabs and Radio Group, and
Tabs trigger and panel id links; and a real Accordion: toggle reporting, Up
and Down trigger movement with every trigger a Tab stop, and trigger and
region id links; Tooltip hover delay, content hover, keyboard focus
opening, press closing, and `aria-describedby`; and Hover Card open and close
delays, card hover and focus, and presses that keep it open; and Command
highlight movement, query resets, and choosing from the input; and Left and
Right swapped under `dir="rtl"` in Tabs, Radio Group, Toggle Group, Menubar,
and Navigation Menu; and vertical, manually activated Tabs; and Combobox and
Command result status regions; and labelled Switch and Checkbox toggling by
click, Space, and label; and Button, Toggle, Input, and Textarea events and
passed attributes; and Slider keys, clamping, pointer press and drag, and
disabled guards; and Collapsible toggling and Native Select choices; and
Input OTP typing, filtering, and Backspace after a rejected character; and
Pagination page changes and disabled anchors; and Carousel slide changes and
arrow keys; and Resizable handle keys and drags; and Sidebar toggles and
items; and Radio Group, Progress, Select, and Combobox names; and dialog names
and page-wide id references; and tab list, toggle group, menu bar, navigation,
and calendar grid names; and the Checkbox mixed state and drawn marks; and
Select and Combobox list widths; and the Slider thumb
position and vertical sliders, all with compiled Tailwind and no conflicting
utilities in any rendered class list; and text contrast of at least WCAG AA
in the light and opt-in dark themes, measured on the RFC 0051 token colors,
with the Checkbox tick switching to its dark stroke under `.dark`; and a 375px
layout with no sideways scroll and no element outside its fixture card; and
action part callbacks and `FieldLabel` `for`; and no axe-core violation of the
WCAG 2.1 A and AA and best-practice rules at the first render, an open Dialog,
each open overlay, menu, and popup, and after the interactions in both themes,
with `landmark-unique` disabled because the preview repeats landmark components
([RFC 0054](rfcs/0054-automated-accessibility-audit.md),
[RFC 0055](rfcs/0055-open-state-accessibility-audit.md)). It does not
write screenshots or traces, update generated docs, change component APIs, edit
templates, certify full accessibility, verify native Desktop or Mobile
behavior, or claim visual parity.

`npm run verify:site` serves the component site with `dx serve` (see
[RFC 0052](rfcs/0052-component-site.md)) and visits the home, installation,
and theming pages and every catalog component page. It fails on a console
error, a route that renders the not found page or the wrong component, text
below the WCAG AA contrast minimum in the light or dark theme, an axe-core
violation of the WCAG 2.1 A and AA and best-practice rules in either theme
([RFC 0054](rfcs/0054-automated-accessibility-audit.md)), or a sideways
scroll at 375px. Each component page must render exactly the examples
`site/src/examples/mod.rs` lists for it, each with a drawn preview and a Code
tab whose source defines the rendered `Demo`. Every catalog component must have at
least one example, and no example may name a component outside the catalog. It also checks that unknown
routes render the not found page,
that the header toggle switches the theme, and that the catalog menu opens and
navigates below `md`. It needs Playwright Chromium and is not part of the
release gate.

`npm run verify:desktop-interactions` builds the Desktop preview and runs its
in-app self-test (RFC 0017). Ten scenarios cover the compiled preview
stylesheet (RFC 0049), the preview theme toggle (RFC 0050), the dialog focus
scope, anchored overlay, listbox, menu mode, toast timer, calendar focus following,
Menubar, and Navigation Menu inside the Desktop WebView. The command exits
with the self-test status. It opens a window, needs a GUI session, and is not
part of default or release gates.

`npm run verify:mobile-interactions` runs the same scenarios in an iOS
Simulator build of `examples/mobile-demo` (RFC 0018). It boots an iPhone
simulator on iOS 26 or older when none is running, and passes only when the
app console reports success. It needs Xcode and is not part of default or
release gates.

`npm run verify:android-interactions` runs the same scenarios in an Android
emulator build of `examples/mobile-demo` (RFC 0020). It boots the first AVD
headless when no emulator is running, requests the self-test through the
`debug.dioxus_shadcn.self_test` system property, and passes only when logcat
reports success. It reports an NDK install whose symbolic links were stored as
text files before building. It needs the Android SDK, NDK, and an AVD and is
not part of default or release gates.

`npm run verify:browser-local` runs the opt-in browser-backed checks serially:
mobile browser smoke, rendered component DOM verification, Web screenshot
smoke, and runtime interaction verification. It is not part of default or
release gates. Do not parallelize these Web preview browser commands; each
command starts and cleans up its own `dx serve` process, and parallel runs can
produce duplicate preview roots or duplicate component targets.

`npm run verify:repo-hygiene` checks that inactive workflow files, generated
directories such as `node_modules/`, `target/`, `dist/`, and `build/`, and known
generated artifacts are not committed. It is read-only and reports drift without
removing files.

`npm run verify:ci-docs` checks that the opt-in CI browser smoke guide and
workflow template still reference the current local verification aliases and do
not add an active browser smoke workflow.

`npm run verify:ci-workflow-template` checks that the documented browser smoke
workflow template, RFC 0009 activation policy, CI browser guide, package alias,
and active workflow absence stay aligned. It is read-only and does not create
workflow files, run GitHub Actions, install browsers, upload artifacts, or
change rollout policy.

`npm run verify:browser-artifact-policy` checks that CI browser artifact
guidance, workflow template upload fields, RFC 0009 artifact policy,
`.gitignore` screenshot patterns, repository hygiene boundaries, and release
wiring stay aligned. It is read-only and does not launch browser automation,
upload artifacts, delete local files, enforce remote retention, or validate
screenshot pixels.

## Mobile Browser Smoke Gate

Run only when Playwright Chromium has been installed:

```bash
npx playwright install chromium
npm run verify:mobile-browser
npm run verify:rendered-component-dom
npm run verify:web-screenshot-smoke
npm run verify:runtime-interactions
npm run verify:browser-local
```

For local machines with a supported Chrome executable, the command also accepts:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:rendered-component-dom
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:web-screenshot-smoke
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:runtime-interactions
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:browser-local
```

Optional local screenshot capture:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

With an external Chrome executable:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

The screenshot artifact uses the ignored
`dioxus-ui-mobile-browser-preview-*.png` pattern and should not be committed.
When screenshot capture is enabled, the script validates the generated PNG
signature, nonzero byte size, and dimensions against the mobile viewport lower
bound.

This opt-in command starts the rendered Web preview, uses a mobile browser
viewport, asserts the Mobile Web profile and representative preview panels, and
cleans up the preview server. It is not part of the release gate and remains
outside default release gates until CI or local release stability is proven.

The Web screenshot smoke command also remains outside default release gates. It
checks the rendered Web preview at `1280x900` and `390x844`, and screenshot
capture stays opt-in through `DIOXUS_UI_WEB_SCREENSHOT=1`.
For manual release-candidate screenshot review, see
`docs/components/release-screenshot-review-checklist.md`.
For screenshot cleanup and retention, see
`docs/components/screenshot-artifact-retention.md`; screenshot files are
temporary local review aids by default and should not be committed or uploaded
by normal local verification.
For copyable manual review evidence, use
`docs/components/release-screenshot-review-notes-template.md`.
For the full manual release-candidate browser review sequence, use
`docs/components/release-candidate-browser-review-runbook.md`.
For release aggregate failure isolation, use
`docs/release-gate-failure-triage-runbook.md`.

For CI setup, artifact upload, and non-blocking job policy, see
`docs/ci-browser-smoke.md` and `docs/ci-browser-workflow-template.md`. This
repository does not add a browser workflow until that workflow is reviewed
separately.

Manual release review:

- Tailwind CSS v4 input stylesheet remains `@import "tailwindcss";`.
- generated templates remain self-contained.
- component docs and registry entries remain in sync.
- accessibility contract changes are reflected in component docs.

## CI Plan

Default pull request CI should run:

```bash
cargo check --workspace --all-features
cargo test --workspace --all-features
cargo test -p dioxus-shadcn-cli --test registry
scripts/generated-fixture-smoke.sh
npm run verify
```

This keeps pull request CI on deterministic Rust, source-copy, preview, example,
docs metadata, and Markdown drift checks without requiring browser installation.

Scheduled or release CI should run the full local release aggregate:

```bash
npm run verify:release
```

The release aggregate includes the feature gate, generated fixture smoke,
release documentation checks, package script wiring checks, and CI documentation
checks. The feature gate recompiles the same crate many times, so keeping it in
scheduled or release CI is the conservative default:

```bash
scripts/feature-check.sh
```

`.github/workflows/ci.yml` runs this plan on GitHub Actions: the default set,
with `cargo fmt --check` and Clippy, on each pull request, and
`npm run verify:release` on each push to `main`.

Browser smoke remains opt-in and should follow the policy in
`docs/ci-browser-smoke.md` and `docs/ci-browser-workflow-template.md`.

## Runtime Web Verification Gate

Run after changes to runtime verification fixtures or experimental Web runtime
adapters:

```bash
cargo test -p dioxus-ui-runtime-web-verification
cargo run -p dioxus-ui-runtime-web-verification
node scripts/runtime-web-verify.mjs
```

This currently verifies compile-checked runtime panel metadata and visible
fallback status output for focus, portal, timer, live-region, measurement,
scroll command, pointer, and gesture contracts. The Node command is the
separate expensive browser-assertion prerequisite check; it asserts stable
fixture output for focus, portal, timers, live-region, measurement, pointer,
gesture, and Message Scroller prerequisites before a real Web runtime browser
driver is promoted.

Do not promote runtime Web verification into the default release gate until the
fixture renders under a Web runtime and the browser assertions are stable.

## Runtime Desktop Verification Gate

Run after changes to Desktop runtime verification fixtures or Desktop-specific
runtime adapter plans:

```bash
cargo test -p dioxus-ui-runtime-desktop-verification
cargo run -p dioxus-ui-runtime-desktop-verification
```

This currently verifies compile-checked Desktop WebView smoke metadata and
visible fallback status output for focus, portal stacking, timers, live status,
measurement, pointer capture, and conservative gesture checks.

Do not promote runtime Desktop verification into the default release gate until
the fixture starts a real Desktop WebView and the smoke checks are stable.

## Desktop Preview Gate

Run after changes to the rendered Desktop preview shell:

```bash
node scripts/desktop-preview-verify.mjs
```

This verifies the Desktop preview binary, Tailwind CSS v4 source input, stable
`data-preview-*` selectors, and representative command-line Desktop smoke
output. Desktop WebView screenshot capture remains separate until repeatable.

## Desktop WebView Screenshot Feasibility

M39 probed local Desktop WebView screenshot capture and did not add a screenshot
smoke gate. macOS capture tooling is present, but the Desktop preview does not
currently produce a stable native window to select: `dx serve --platform
desktop` can build and launch before the macOS app exits with `SIGBUS`, and the
direct Cargo preview binary also exits without a repeatable window.

Do not add `scripts/desktop-webview-screenshot-smoke.sh` to local, CI, or
release gates until a local GUI session can launch the Desktop preview
consistently and select it by stable title, process name, or window id. Continue
to use `node scripts/desktop-preview-verify.mjs` as the supported Desktop gate.

## Runtime Mobile Verification Gate

Mobile verification now has a Mobile Web profile structural gate, but native
Mobile verification remains documentation-only until a repeatable device or
emulator command exists.

After changes to Mobile runtime plans, review:

```text
docs/components/mobile-web-profile-verification.md
docs/components/runtime-mobile-verification-checklist.md
docs/components/runtime-desktop-mobile-verification.md
docs/components/runtime-renderer-verification.md
```

Do not add Mobile runtime adapters or default component behavior until touch,
safe area, visual viewport, native scroll, reduced motion, and visible status
checks can be repeated and fallback states are visible.
