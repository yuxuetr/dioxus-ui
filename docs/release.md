# Release and Package Strategy

## Goal

`dioxus-shadcn` supports two distribution modes:

- source-copy mode through `dxui add`
- crate mode through `dioxus-shadcn` feature flags

Both modes ship in 0.1.0, 0.2.0, 0.3.0, 0.4.0, 0.4.1, and 0.4.2. The `0.4.x` API surface is accepted;
before `1.0`, a breaking change bumps the minor version and comes with a
changelog migration note.

## Package Set

Published crates (0.1.0 and 0.2.0 on crates.io since 2026-10-05, 0.3.0,
0.4.0, 0.4.1, and 0.4.2 since 2026-10-06):

```text
dioxus-shadcn-core
dioxus-shadcn-primitives
dioxus-shadcn
dioxus-shadcn-cli
```

Publishing order:

1. `dioxus-shadcn-core`
2. `dioxus-shadcn-primitives`
3. `dioxus-shadcn`
4. `dioxus-shadcn-cli`

## Feature Policy

The styled crate should keep `default = []`.

Users opt into components:

```toml
dioxus-shadcn = { version = "0.4", default-features = false, features = ["button", "dialog"] }
```

Feature names should match registry names where possible:

```text
button
input
textarea
label
checkbox
switch
tabs
accordion
dialog
popover
tooltip
select
dropdown
```

Overlay components may enable primitive features:

```toml
dialog = ["dioxus-shadcn-primitives/dialog"]
popover = ["dioxus-shadcn-primitives/popover"]
tooltip = ["dioxus-shadcn-primitives/tooltip"]
select = ["dioxus-shadcn-primitives/select"]
dropdown = ["dioxus-shadcn-primitives/dropdown"]
```

## Versioning Policy

Before `1.0`, API changes are allowed but should still be documented in the
changelog.

Breaking changes include:

- renaming components or props
- changing feature names
- changing generated file paths
- changing registry schema fields
- changing class helper semantics
- removing public primitive config fields

Patch releases should be limited to:

- bug fixes
- documentation corrections
- non-breaking class additions
- registry metadata fixes

Before each release, compare the three library crates with the last
published version, here 0.3.0, after the version bump:

```bash
cargo binstall cargo-semver-checks   # once
for crate in dioxus-shadcn-core dioxus-shadcn-primitives dioxus-shadcn; do
  cargo semver-checks -p "$crate" --baseline-version 0.3.0 --all-features --release-type minor
done
```

`--release-type minor` runs the breaking-change lints even though a pre-1.0
minor bump allows breaking changes; without it every lint is skipped. Each
finding goes in the changelog's Migration section. The check compares crate
APIs only: copy-mode changes such as template files and the `dxui` command
line need their own notes. For 0.3.0 it found new props fields in
`dioxus-shadcn`; for 0.4.0 it found none. A patch release compares against
the previous patch with `--release-type patch`; for 0.4.1 against 0.4.0 and 0.4.2
against 0.4.1 it found none.

## Release Gates

Before publishing any crate:

```bash
npm install
npm run verify:release
```

The release aggregate runs the required local release gates in command-chain
order. For manual review or focused failure isolation, the expanded gate set is:

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

For routine local handoff before release-specific gates, this deterministic
convenience alias is available:

```bash
npm install
npm run verify
```

The alias runs preview smoke, example smoke, docs metadata, and Markdown drift
checks. It does not install browser binaries and does not replace the full Rust,
source-copy, feature, or release documentation gate list above.
Preview and example focused checks are reached through the direct
`npm run verify` release segment rather than listed as separate release command
segments.
Changelog metadata checks are read-only and validate only project-owned
changelog structure, Unreleased section, Keep a Changelog and Conventional
Commits references, and stale template-link bans; they do not generate release
notes, run git-cliff, derive changes from Git history, create tags, publish
releases, or rewrite commit history.
Package script wiring checks are also part of the release aggregate, but they
only inspect `package.json` and local script target existence; they do not
execute the release gate recursively.
Package lock metadata checks are also part of the release aggregate, but they
only compare committed `package.json` and `package-lock.json` root metadata;
they do not run `npm install` or contact the npm registry.
Cargo workspace metadata checks are also part of the release aggregate, but
they only compare committed workspace manifest metadata and `cargo metadata`
output; they do not publish crates or contact crates.io.
Cargo publish metadata checks are read-only and validate only planned published
crate descriptions, shared README/keywords/categories metadata, example
`publish = false` boundaries, and release wiring; they do not run
`cargo publish`, run `cargo package`, contact crates.io, replace repository URLs, or create package archives.
They also require internal workspace dependencies
to declare the workspace version alongside their local paths, since published
crates resolve them from crates.io by that version.
The [Public API Surface Inventory](public-api-surface-inventory.md) documents
the component, primitive, core, feature, registry, and source-copy surfaces
accepted for first publish.
`npm run verify:package-contents` runs `cargo package --list` for each
publishable crate and checks that each package contains its `LICENSE` and the CLI package contains
every registry entry and template the CLI embeds; it lists package contents only and does not build
package archives, run `cargo publish`, or contact crates.io.
Publish order checks are read-only. They validate the planned crate publish
order; they do not create package archives, run `cargo package`, run
`cargo publish`, contact crates.io, check registry ownership, inspect
credentials, change dependency versions, or authorize a release.
Cargo lock metadata checks are also part of the release aggregate, but they
only compare committed `Cargo.lock` workspace package metadata and Cargo
metadata output; they do not update the lockfile or contact crates.io.
Pre-commit metadata checks are also part of the release aggregate, but they
only validate committed local hook wiring and supporting config files; they do
not install or execute pre-commit hooks.
Script metadata checks are also part of the release aggregate, but they only
validate committed script shebangs, executable bits, and package-referenced
script targets; they do not execute scripts.
Gitignore metadata checks are also part of the release aggregate, but they only
validate `.gitignore` patterns and repository hygiene policy fragments; they do
not delete local artifacts or inspect ignored file contents.
Preview state metadata checks are also part of the release aggregate, but they
only validate shared preview inventory wiring, Web/Desktop preview binaries,
structural verifier state lists, Tailwind CSS v4 preview inputs, and release
wiring; they do not launch browsers, run `dx serve`, compile Tailwind output,
inspect screenshots, or assert visual parity.
Mobile browser smoke metadata checks are also part of the release aggregate,
but they only validate the opt-in browser smoke script, package alias,
localhost target, `390x844` viewport, selector contract, screenshot artifact
pattern, browser environment variables, and CI browser guidance; they do not
launch Playwright, start `dx serve`, install browsers, write screenshots, or
validate rendered output.
Rendered component coverage checks are read-only. They validate stable rendered
preview target metadata for public components against the docs catalog and
confirm those `data-component-preview` ids are present in the shared preview
state source; they do not start a server, launch a browser, write screenshots,
update generated docs, change component APIs, edit templates, or claim visual
parity.
Examples metadata checks are also part of the release aggregate, but they only
validate example workspace membership, package script wiring, smoke script
references, preview verifier references, and examples documentation; they do
not run examples or launch previews.
CSS input metadata checks are also part of the release aggregate, but they only
validate Tailwind CSS v4 input syntax, preview source roots, and CLI default
CSS tokens; they do not compile Tailwind or inspect generated CSS output.
CI browser docs checks are read-only and do not create workflow files.
CI Plan checks are also read-only and validate documentation only.
CI workflow template metadata checks are also read-only and validate only the
documented browser workflow template, RFC 0009 activation policy, CI browser
guide, package alias, and absence of an active browser smoke workflow file
(`.github/workflows/browser-smoke.yml`); they do not run
GitHub Actions, install browsers, upload artifacts, or change rollout policy.
Browser artifact policy metadata checks are also read-only and validate only
committed artifact guidance, screenshot upload patterns, `.gitignore` coverage,
repository hygiene boundaries, and release wiring; they do not launch browser
automation, upload artifacts, delete local files, enforce remote retention, or
validate screenshot pixels.
Repository hygiene checks are read-only and report forbidden committed
artifacts such as generated directories, screenshots, and inactive workflow
files without deleting files.
Registry metadata checks are part of `npm run verify:release` and validate only
registry JSON names, descriptions, source mappings, source-copy targets,
dependency references, and asset mappings.
Tailwind static token checks are part of `npm run verify:release` and validate
only that shipped Rust source and templates avoid dynamic Tailwind utility
interpolation.
Component status checks are part of `npm run verify:docs` and validate only the
local implementation snapshot and wiring coverage matrix derived from catalog
metadata.
Component docs structure checks are part of `npm run verify:docs` and validate
only required public component docs sections plus generated command and feature
snippets.
RFC metadata checks are part of `npm run verify:docs` and validate only RFC
filename numbering, first headings, contiguous sequence, and README index
coverage.
README verification summary checks are part of `npm run verify:docs` and
validate only verification shortcut command discoverability and summary
coverage in `README.md`.
Documentation index checks are part of `npm run verify:docs` and validate only
README entry-point links.
Markdown link target checks are part of `npm run verify:docs` and validate only
local relative file targets in tracked Markdown files.
Markdown anchor checks are part of `npm run verify:docs` and validate only
local fragments in tracked Markdown files.

Focused npm aliases are also available:

```bash
npm run verify:smoke
npm run verify:docs
npm run verify:release
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:registry
npm run verify:cargo-workspace
npm run verify:cargo-publish-metadata
npm run verify:cargo-lock
npm run verify:pre-commit
npm run verify:scripts
npm run verify:gitignore
npm run verify:examples-metadata
npm run verify:css-inputs
npm run verify:rfcs
npm run verify:changelog
npm run verify:readme
npm run verify:tailwind-static
npm run verify:ci-docs
npm run verify:ci-plan
npm run verify:ci-workflow-template
npm run verify:browser-artifact-policy
npm run verify:docs-status
npm run verify:docs-structure
npm run verify:docs-index
npm run verify:docs-links
npm run verify:docs-anchors
npm run verify:runtime-interactions
npm run verify:repo-hygiene
```

Browser-rendered Playwright smoke remains opt-in by policy (RFC 0009).

Optional browser smoke:

```bash
npx playwright install chromium
npm run verify:mobile-browser
npm run verify:rendered-component-dom
npm run verify:web-screenshot-smoke
npm run verify:runtime-interactions
npm run verify:browser-local
```

Local external Chrome fallback:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:rendered-component-dom
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:web-screenshot-smoke
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:runtime-interactions
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:browser-local
```

Optional local screenshot artifact:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

External Chrome plus screenshot artifact:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

Generated screenshots match the ignored
`dioxus-ui-mobile-browser-preview-*.png` pattern.
When screenshot capture is enabled, the script validates the generated PNG
signature, nonzero byte size, and dimensions against the mobile viewport lower
bound of `390x844`.

This is not part of the release gate yet. It verifies mobile browser rendering
of the Web preview, not native Mobile behavior.

`npm run verify:rendered-component-dom` is also opt-in and outside release
gates. It verifies all 64 public `data-component-preview` targets exist,
remain visible, contain text, and have non-empty bounding boxes in a browser
DOM. It does not write screenshots or traces, assert interactions, or claim
visual parity.

`npm run verify:runtime-interactions` is also opt-in and outside release gates.
It starts the Web preview and checks focused `data-interaction-*` fixtures for
representative click, keyboard, focus, ARIA, visible text, and `data-state`
transitions. It does not write screenshots or traces, certify full
accessibility, verify native Desktop or Mobile behavior, or claim visual parity.

`npm run verify:site` is opt-in and outside release gates too. It serves the
component site and checks every route for console errors, the right page,
text contrast and an axe-core audit in both themes, and a 375px layout (see
RFC 0052 and RFC 0054). The release
gate runs `npm run verify:site-css` and `npm run verify:site-catalog`, which
need no browser. `.github/workflows/pages.yml` deploys the site to GitHub
Pages on every push to `main`.

`npm run verify:web-screenshot-smoke` is also opt-in and outside release gates.
It starts the Web preview and checks desktop and mobile screenshot readiness
without writing screenshots by default. When `DIOXUS_UI_WEB_SCREENSHOT=1` is
set, it writes ignored `dioxus-ui-web-preview-*.png` files and validates PNG
metadata. It does not compare pixels, claim visual parity, or replace manual
visual review.

`npm run verify:browser-local` is also opt-in and outside release gates. It
runs the browser-backed commands serially so their `dx serve` processes do not
overlap. It does not enable screenshots by default, activate CI workflows, or
replace release gates.

For manual release-candidate screenshot review, use
`docs/components/release-screenshot-review-checklist.md`. The checklist records
what to capture, what to inspect, and how to clean local screenshot artifacts
without turning visual review into an automated release gate.
The retention decision in
`docs/components/screenshot-artifact-retention.md` keeps screenshots
local-first: copy command output and PNG metadata into review notes, do not
commit PNG files, and defer uploads or GitHub release attachments until a
separate release owner defines that process.
Use `docs/components/release-screenshot-review-notes-template.md` for a
copyable review note format that records environment, screenshot metadata,
observed issues, decision, retention outcome, and cleanup evidence.
Use `docs/components/release-candidate-browser-review-runbook.md` for the
full manual sequence that combines deterministic gates, opt-in browser smoke,
optional screenshot capture, review notes, retention, and cleanup.
Use `docs/release-gate-failure-triage-runbook.md` when a release aggregate
command fails and the maintainer needs to isolate the first failing focused
gate without automatic repair behavior.

## Release Smoke Checks

For CI setup options and non-blocking workflow policy, see
`docs/ci-browser-smoke.md` and `docs/ci-browser-workflow-template.md`. Do not
treat browser smoke as a required release gate until a reviewed workflow exists.
Workflow activation and required-gate promotion should follow
`docs/rfcs/0009-ci-browser-workflow-activation.md`.

Smoke commands:

```bash
cargo run -p dioxus-shadcn-cli -- init --root /tmp/dxui-release-smoke
cargo run -p dioxus-shadcn-cli -- add button --root /tmp/dxui-release-smoke
cargo run -p dioxus-shadcn-cli -- add dialog --root /tmp/dxui-release-smoke
```

Manual review:

- generated `assets/dioxus-shadcn.css` uses Tailwind CSS v4 syntax
- generated `src/components/ui/mod.rs` is deterministic
- registry entries point to existing templates
- component features compile individually and in representative combinations
- command-line Web and Desktop examples expose representative parity states
- Web preview screenshots cover desktop and mobile viewports
- Mobile Web profile gate passes; native Mobile device, simulator, keyboard,
  safe-area, and assistive behavior remain unclaimed
- Mobile browser smoke was probed in M41 but is not a release gate until a
  deterministic Playwright dependency and browser binary strategy exists
- Desktop preview structural gate passes; Desktop WebView screenshot capture
  was locally probed in M39 but is not repeatable yet, so it should not be
  claimed or added to release gates

`scripts/feature-check.sh` is more expensive than a normal workspace check
because it invokes Cargo once per public component feature. Run it before
release and after feature-gating changes.

See [Quality Gates](quality-gates.md) for local, CI, source-copy, feature, and
release verification tiers.

## CLI Release Notes

The 0.1.0 CLI supports, one component per `add`:

```text
dxui init [--root <path>]
dxui add <component> [--root <path>] [--overwrite]
dxui list
```

The CLI embeds registry and template assets at compile time. Installed CLI
commands can read component metadata and generated source without relying on
the `crates/dioxus-shadcn-cli/registry/` or `crates/dioxus-shadcn-cli/templates/`
directories at runtime. Both directories live inside the CLI crate so
`cargo package` includes them in the published crate.

Cargo publish metadata is tracked by `npm run verify:cargo-publish-metadata`.
That check keeps descriptions and shared README/keywords/categories metadata
reviewable, and `npm run verify:package-contents` checks that the CLI package
contains every embedded registry entry and template.

## First Publish Dry Run

M184 ran `cargo publish --workspace --dry-run` on 2026-10-05 with
cargo 1.99.0, after the RFC 0056 rename. Cargo packaged and compile-verified
every crate from its extracted archive, then aborted each upload:

| Crate | Packaged Files | Compressed Size |
| --- | --- | --- |
| `dioxus-shadcn-core` | 8 | 5.1 KiB |
| `dioxus-shadcn-primitives` | 27 | 28.3 KiB |
| `dioxus-shadcn` | 79 | 100.7 KiB |
| `dioxus-shadcn-cli` | 140 | 84.3 KiB |

Each package includes `README.md`, from `crates/README.md`, and `LICENSE`.
The only warnings were `aborting upload due to dry run`. Package archives stay
in the Cargo target directory and are never committed.

`npm run verify:package-contents` keeps the package file lists checked in
`npm run verify:release`; rerun the dry run before an actual publish because it
also resolves dependencies against the live crates.io index.

## First Publish Steps

The release owner published 0.1.0 on 2026-10-05 with these steps, tagged as
`v0.1.0`. The first attempt failed before any upload because the crates.io
account had no verified email; verify it at crates.io/settings/profile before
publishing. The steps require the crates.io evidence in
[Registry Availability Readiness Metadata](archive/first-publish/registry-availability-readiness-metadata.md#resolution)
first.

1. Record the crates.io evidence and mark registry availability resolved.
2. Rename the `CHANGELOG.md` Unreleased section to `[0.1.0]` with the release
   date and commit it.
3. Authenticate with `cargo login`; never commit or paste the token into the
   repository.
4. Run `cargo publish --workspace --dry-run` and confirm all four crates
   verify.
5. Run `cargo publish --workspace`. Cargo publishes each crate after its
   dependencies; 0.1.0 uploaded `dioxus-shadcn-core`, then
   `dioxus-shadcn-cli` and `dioxus-shadcn-primitives`, then `dioxus-shadcn`,
   which satisfies the same constraints as the publishing order above.
6. Tag the release commit, as `v0.1.0`.

After publishing, `cargo install dioxus-shadcn-cli` installed a `dxui` that
lists all 64 components, and a fresh app built with both a copied component
and the `dioxus-shadcn` crate from crates.io.

## 0.2.0 Publish

The release owner confirmed 0.2.0 on 2026-10-05, published with the same
steps and tagged `v0.2.0`: the version bump and `[0.2.0]` notes landed first
with the release gate and dry run passing, then `cargo publish --workspace`
uploaded the four crates in dependency order. The published `dxui` lists 79
components and 33 theme presets, and a fresh app built against
`dioxus-shadcn` 0.2 from crates.io with copied Tags Input and Date Picker
templates and an added `nord` preset.

## 0.3.0 Publish

The release owner confirmed 0.3.0 on 2026-10-06 after a review of the
release: the release gate and dry run passed, and `cargo-semver-checks`
against 0.2.0 found no breaking change in `dioxus-shadcn-core` or
`dioxus-shadcn-primitives` and only new props fields in `dioxus-shadcn`,
which the migration notes list. The review also fixed the crate README's
`@source` path, which still named 0.1.0. The four crates were published one
at a time in dependency order and tagged `v0.3.0`. The published `dxui`
lists 82 components and 3 blocks; a fresh app built with the dashboard,
login, and settings blocks copied, and another built against
`dioxus-shadcn` 0.3 from crates.io with Theme Controller, Mockup, Menu, and
`RangeSlider`.

## 0.4.0 Publish

The release owner confirmed 0.4.0 on 2026-10-06 once CI passed on the
release commit. The release gate and dry run passed, and `cargo-semver-checks`
against 0.3.0 found no breaking change in the three library crates; the
migration notes cover apps that copied components with 0.3. The four crates
were published one at a time in dependency order and tagged `v0.4.0`. The
published `dxui` 0.4.0 copied Button, Dialog, Popover, and the dashboard
block into a fresh app that built while denying warnings, and `dxui diff`
reported that every copy matched; another app built against `dioxus-shadcn`
0.4 from crates.io.

## 0.4.1 Publish

The release owner confirmed 0.4.1 on 2026-10-06, to follow the `dxui diff`
change once CI passed. The release gate and dry run passed, and
`cargo-semver-checks --release-type patch` against 0.4.0 required no version
change in the three library crates. The four crates were published in
dependency order and tagged `v0.4.1`. In a fresh app with Button, Dialog, and
the dashboard block, the published `dxui` 0.4.1 ran `dxui diff` without names
and reported all 14 entries matching, then exited 1 after `dialog.rs` was
edited.

## Known Pre-1.0 Limitations

- Dialog, Alert Dialog, Sheet, and Drawer implement Escape and overlay
  dismissal, initial focus, Tab wrap, and focus restore; Popover, Dropdown,
  Hover Card, and Tooltip implement anchored placement and dismissal. Only the
  Web renderer is browser-verified. Select and Combobox implement anchored
  listbox keyboard navigation and selection (see RFC 0012) and multiple
  selection (see RFC 0062); the input-inside-content Combobox layout is not
  supported. Command
  highlights and chooses items from its input (see RFC 0024); fuzzy ranking
  is not implemented. Combobox and Command announce result counts through
  status parts whose wording the app provides (see RFC 0027). Switch and
  Checkbox report changes through `on_checked_change` (see RFC 0028); a form
  input for Switch is not included. Button,
  Toggle, Input, and Textarea report events through `onclick`,
  `on_pressed_change`, and `on_value_change` (see RFC 0029); key, focus, and
  blur callbacks are not included. Slider responds to keys and the pointer
  (see RFC 0030), and `RangeSlider` has two thumbs (see RFC 0070); right-to-left sliders are not included. Collapsible and Native Select report changes through
  `on_open_change` and `on_value_change` (see RFC 0031); multiple selection is
  not included. Input OTP reports the cleaned code through `on_value_change`
  (see RFC 0032); editing a slot in the middle is not included. Pagination
  controls report clicks through `onclick` and render buttons without an
  `href` (see RFC 0033); `pagination_range` lays out page numbers and
  ellipses. Carousel shows
  the selected index and reports clicks and arrow keys (see RFC 0034); swipe
  gestures and autoplay are not included. A passed `aria-label` replaces the
  English default on Pagination and Carousel controls in the browser and in
  SSR (see RFC 0035). Resizable handles report keyboard and pointer resizes
  through `on_resize` (see RFC 0036); right-to-left groups and keyboard
  collapse are not included. Sidebar triggers report toggles and items render
  links or buttons with `aria-current` (see RFC 0037); an opt-in off-canvas
  panel and keyboard shortcut follow the viewport (see RFC 0069). Radio Group, Progress, the Select
  trigger, and the Combobox input pass through naming attributes (see RFC
  0038); the other parts of those components do not. Dialog, Alert Dialog,
  Sheet, Drawer, and Popover content take their names from their titles (see
  RFC 0039); server-rendered HTML gets the link after hydration. Tab lists,
  toggle groups, menu bars, navigation menus, and calendar grids take names
  through passed attributes (see RFC 0040). Checkbox has a native mixed state
  (see RFC 0041); it is set after hydration. Slider places its thumb on the
  value and supports a vertical orientation (see RFC 0042), and `RangeSlider`
  has two thumbs (see RFC 0070); right-to-left sliders are not included. Browser checks run with compiled
  Tailwind and data variants match attribute values (see RFC 0043); compiled
  CSS in the Desktop and Mobile self-tests is not covered. Class functions do
  not join conflicting utilities (see RFC 0044); a user class that sets a
  property the component sets needs Tailwind's important modifier. Checkbox
  draws its box and marks (see RFC 0045); the marks are white images, so
  custom mark colors and forced-colors marks are not included. Select and
  Combobox lists are at least as wide as their trigger (see RFC 0046); other
  anchored content sizes to its content. Component classes use the shadcn/ui
  semantic color tokens (see RFC 0051), which crate-mode apps must define;
  the Checkbox marks follow the default light and dark
  `--primary-foreground` only. The dark theme is an opt-in `.dark`
  class that redefines only the tokens (see RFC 0047 and RFC 0051);
  `ThemeController` defaults it to the system preference and remembers the
  choice (see RFC 0071), and app palette classes do not follow it. Pagination content wraps in narrow containers (see
  RFC 0048); a Pagination that drops pages to fit is not included. The
  previews link a committed compiled stylesheet (see RFC 0049) that must be
  regenerated with `npm run css:preview` after class changes; dioxus-shadcn
  itself still ships no compiled Tailwind output. The previews have a dark
  theme toggle (see RFC 0050); they do not follow the system color scheme or
  remember the choice. Date Picker and
  Calendar implement anchored placement, focus entry, and day keyboard
  navigation (see RFC 0013), and Date Picker Input parses typed dates (see
  RFC 0064); source-copy date arithmetic is not included. Dropdown and Context Menu implement menu keyboard
  navigation and activation, and Context Menu opens at a point (see RFC 0014);
  all three menus have submenus (see RFC 0067) without a pointer grace area. Menubar implements roving triggers and menu
  switching with Left, Right, and hover (see RFC 0015). Navigation Menu
  implements click, keyboard, and hover disclosure with dismissal and
  vertical submenus (see RFC 0016 and RFC 0063), but not viewport size
  measurement. Tabs, Radio Group, and Toggle
  Group keep one Tab stop and move focus with arrow keys, and Tabs and Radio
  Group select the focused item (see RFC 0019); Tabs also supports manual
  activation and a vertical orientation (see RFC 0026). These groups, Menubar, and Navigation
  Menu swap Left and Right in right-to-left layouts (see RFC 0025); Calendar
  grid keys do not. Accordion reports
  toggles and moves focus between triggers with Up and Down (see RFC 0021);
  an item that cannot collapse is not implemented. Tooltip opens on hover
  after a delay and on keyboard focus (see RFC 0022), and Hover Card does the
  same with a close delay (see RFC 0023); skipping delays between adjacent
  roots and touch opening are not implemented. There is no DOM portal (see
  RFC 0010); modal overlays lock page scroll while open (see RFC 0068).
- Toast and Sonner dismiss themselves after a countdown that pauses on hover
  and focus, inside persistent live region viewports (see RFC 0011). Screen
  reader announcements are not automated, and swipe to dismiss is not
  implemented.
- Generated templates include the local helper modules they use and should not
  require `dioxus-shadcn-core` or `dioxus-shadcn-primitives` in source-copy mode.
- Web has a rendered preview shell and screenshot procedure. Desktop has a
  rendered preview shell and structural gate, but Desktop WebView screenshot
  capture is currently unsupported because the native preview window is not
  repeatable in local probes.
- Desktop interaction behavior is checked by `npm run verify:desktop-interactions`,
  an in-app self-test of ten scenarios in the Desktop WebView (see
  RFC 0017), starting with checks that the compiled preview stylesheet
  applies (see RFC 0049) and that the theme toggle works (see RFC 0050). It needs a GUI session, runs locally on macOS only, and is not
  part of `npm run verify:release`. It does not exercise native default
  actions such as Tab movement.
- Mobile has a Web profile structural gate for source-level mobile viewport and
  fallback markers. `npm run verify:mobile-interactions` runs the ten
  scenarios in an iOS Simulator build (see RFC 0018). It needs
  Xcode and is not part of `npm run verify:release`.
  `npm run verify:android-interactions` runs them in an Android emulator
  (see RFC 0020); it needs the Android SDK, NDK, and an AVD and is not part of
  `npm run verify:release` either. Physical devices and touch gestures are not
  automated.
- Dioxus 0.7 iOS apps stop at launch on iOS 27, because they do not adopt
  the UIScene lifecycle. This affects every Dioxus 0.7 app; the Mobile
  self-test uses iOS 26 or older by default.
- Mobile browser smoke is documented as infeasible for release gates until
  browser automation dependencies are made portable.
- The release aggregate currently reports the known `block` `0.1.6` Rust
  future-incompatibility warning. It is tracked by the release warning
  inventory metadata gate until the dependency graph changes.
