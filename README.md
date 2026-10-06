# dioxus-shadcn

`dioxus-shadcn` is a shadcn/ui-style component library for Dioxus 0.7:

- headless primitives for behavior, accessibility, state, and composition
- Tailwind CSS styled components as the default visual layer
- a CLI that copies component source into user projects
- a packaged crate for users who prefer dependency-based usage

It ships 82 components, the `dxui` CLI, and a
[component site](https://yuxuetr.github.io/dioxus-ui/). Version 0.2.0 is
on crates.io as [`dioxus-shadcn`](https://crates.io/crates/dioxus-shadcn) and
[`dioxus-shadcn-cli`](https://crates.io/crates/dioxus-shadcn-cli).

## Product Direction

The library is not a pure Tailwind component package. Its shape is:

```text
Dioxus headless/primitive logic layer
+ Tailwind default style layer
+ CLI component source generator
```

This gives users two workflows:

1. Add source code into an app and customize it freely.
2. Depend on a crate and enable only the component features they need.

Both workflows use the same component API. Source copy suits apps that want to
edit components; the crate suits apps that want updates through Cargo.

## Repository Layout

```text
dioxus-ui/
├─ crates/
│  ├─ dioxus-shadcn-core/     # shared types, class merging, theme tokens
│  ├─ dioxus-shadcn-primitives/   # unstyled logic components
│  ├─ dioxus-shadcn/          # styled public components
│  └─ dioxus-shadcn-cli/      # dxui init / add / list
│     ├─ registry/            # component metadata embedded in the CLI
│     └─ templates/           # source templates copied by the CLI
├─ examples/
│  ├─ preview-states/         # preview fixtures shared by the demos
│  ├─ web-demo/
│  ├─ desktop-demo/
│  ├─ mobile-demo/
│  ├─ runtime-web-verification/
│  └─ runtime-desktop-verification/
├─ site/                      # component site (dx serve --package dioxus-ui-site)
└─ docs/
   └─ rfcs/
```

The published crates show [crates/README.md](crates/README.md) on crates.io;
it is the short guide for using them. This README documents development.

Example run commands are documented in [examples/README.md](examples/README.md).

## Usage

### Source-Copy Mode

```bash
cargo install dioxus-shadcn-cli

dxui init
dxui add button
dxui add dialog
dxui add input
```

`dxui add` keeps existing component files by default. Use
`dxui add button --overwrite` when you intentionally want to replace a
previously generated component file.

Expected output in a Dioxus app:

```text
src/components/ui/mod.rs
src/components/ui/button.rs
src/components/ui/dialog.rs
src/components/ui/input.rs
src/components/ui/utils.rs
assets/dioxus-shadcn.css
```

For Tailwind CSS v4, `assets/dioxus-shadcn.css` should be an input stylesheet, not
a precompiled full Tailwind output:

```css
@import "tailwindcss";

@custom-variant dark (&:is(.dark *));

:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.141 0.005 285.823);
  --primary: oklch(0.21 0.006 285.885);
  --primary-foreground: oklch(0.985 0 0);
  /* ...the rest of the token set */
}

.dark {
  color-scheme: dark;
  --background: oklch(0.141 0.005 285.823);
  /* ... */
}

@theme inline {
  --color-background: var(--background);
  --color-primary: var(--primary);
  /* ... */
}
```

The user's Dioxus app build should produce the final CSS after scanning the app
source and generated component files.

The stylesheet defines the shadcn/ui semantic color tokens in the shadcn/ui
v4 layout ([RFC 0051](docs/rfcs/0051-semantic-color-tokens.md)):
`--background`, `--foreground`, `--card`, `--popover`, `--primary`,
`--secondary`, `--muted`, `--accent`, `--destructive`, each with a
`-foreground` pair where text sits on it, plus `--border`, `--input`,
`--ring`, `--chart-1` to `--chart-5`, the `--sidebar-*` group, and
`--radius`. On top of shadcn/ui it adds `--destructive-foreground` and the
`--success`, `--warning`, and `--info` status colors with their
`-foreground` pairs. `@theme inline` turns
each token into a Tailwind color, so `bg-primary` and `text-muted-foreground`
work in app code too. To rebrand, redefine a token in `:root` and `.dark`,
for example a blue `--primary` and `--ring`. Component classes use only the
tokens, plus `bg-black/50` for modal overlays.

The generated stylesheet also holds an opt-in dark theme. Add the `dark` class
to the app's top-level element to turn it on:

```rust
div { class: "dark min-h-screen bg-background text-foreground", App {} }
```

`.dark` redefines only the tokens, so your own palette classes, such as
`bg-white` or `text-zinc-600`, keep their colors there; use the token classes
for app surfaces that should follow the theme. The
`@custom-variant` line makes an app's own `dark:` utilities follow the same
class. To follow the system preference instead, wrap the `.dark` blocks in
`@media (prefers-color-scheme: dark)` and change their selector to `:root`.
The Web, Desktop, and Mobile previews have a "Dark theme" toggle in their
header that shows the components under the block.

For other palettes, `dxui theme add <name>...` appends theme presets ported
from daisyUI, and `dxui theme list` prints all 33 with their color scheme
([RFC 0057](docs/rfcs/0057-theme-presets.md)). A preset is one
`[data-theme="<name>"]` rule, so setting the attribute themes that element's
subtree, and dark presets set `color-scheme: dark` without the `dark` class:

```rust
div { "data-theme": "nord", class: "min-h-screen bg-background text-foreground", App {} }
```

Crate-mode apps get the same stylesheet by running `dxui init`, which writes
only `assets/dioxus-shadcn.css` and an empty `src/components/ui/mod.rs`. They
also add an `@source` line for the crate's source after the import, since
Tailwind generates only the classes it finds in scanned files (see
[crates/README.md](crates/README.md#depend-on-the-crate)). The token blocks in
`examples/web-demo/assets/preview.css`, after its repository-relative
`@source` lines, match the generated stylesheet; `npm run verify:css-inputs`
checks that.

### Crate Mode

```toml
[dependencies]
dioxus-shadcn = { version = "0.2", default-features = false, features = ["button", "input", "dialog"] }
```

```rust
use dioxus_shadcn::{Button, DialogContent, DialogTitle, Input};
```

## Component Scope

The 82 components are listed in
[docs/components/catalog.md](docs/components/catalog.md) and by `dxui list`.
Known pre-1.0 limitations are in
[docs/release.md](docs/release.md#known-pre-10-limitations).

## Tailwind Rule

Tailwind class names must appear as complete source tokens. Runtime selection is
allowed, but runtime class construction is not.

Use this:

```rust
match variant {
  ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
  ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
}
```

Avoid this:

```rust
format!("bg-{}-500", color)
```

## Component Site

`site/` is a Dioxus Web app that browses the catalog in the manner of the
shadcn/ui site: a sidebar grouped by category, a page per component with its
install commands and live examples (each with a Preview and a Code tab
showing the file that runs), installation and theming guides, a dark
theme toggle, and a menu of the theme presets
([RFC 0052](docs/rfcs/0052-component-site.md),
[RFC 0057](docs/rfcs/0057-theme-presets.md)). To add an
example, put a file with a `Demo` component under `site/src/examples/` and
list it in `site/src/examples/mod.rs`. Run it locally with:

```bash
dx serve --package dioxus-ui-site
```

After changing classes the site uses, run `npm run css:site`; after changing
the registry, catalog, or theme presets, run `npm run site:catalog`. The release gate fails
when either generated file is stale, and `npm run verify:site` checks every
route in a browser.

Every push to `main` deploys the site to
<https://yuxuetr.github.io/dioxus-ui/> through `.github/workflows/pages.yml`:
it builds with `--base-path dioxus-ui`, and `npm run site:pages` copies
`index.html` to each route and to `404.html`, since GitHub Pages serves files
only.

## Documentation

- [Design Overview](docs/design.md)
- [Roadmap](docs/roadmap.md)
- [Workspace Specification](docs/workspace.md)
- [Component API Specification](docs/component-api.md)
- [Release and Package Strategy](docs/release.md)
- [Internal Trial Developer Guide](docs/internal-trial-developer-guide.md)
- [Release Gate Failure Triage Runbook](docs/release-gate-failure-triage-runbook.md)
- [Quality Gates](docs/quality-gates.md)
- [CI Browser Smoke Guide](docs/ci-browser-smoke.md)
- [CI Browser Workflow Template](docs/ci-browser-workflow-template.md)
- [Component Catalog](docs/components/README.md)
- [Documentation Site Plan](docs/site.md)
- [TODO Plan](TODOs.md)
- [RFC 0001: Project Architecture](docs/rfcs/0001-project-architecture.md)
- [RFC 0002: CLI Registry and Code Generation](docs/rfcs/0002-cli-registry-and-code-generation.md)
- [RFC 0003: Tailwind Styling Contract](docs/rfcs/0003-tailwind-styling-contract.md)
- [RFC 0004: Benchmark and CSS Output Strategy](docs/rfcs/0004-benchmark-and-css-output.md)
- [RFC 0005: Modules and Platform Profiles](docs/rfcs/0005-modules-and-platform-profiles.md)
- [RFC 0006: Focus and Portal Primitives](docs/rfcs/0006-focus-and-portal-primitives.md)
- [RFC 0007: Keyboard Navigation Primitives](docs/rfcs/0007-keyboard-navigation-primitives.md)
- [RFC 0008: Overlay Positioning and Portals](docs/rfcs/0008-overlay-positioning-and-portals.md)
- [RFC 0009: CI Browser Workflow Activation](docs/rfcs/0009-ci-browser-workflow-activation.md)
- [RFC 0010: Overlay Interaction Behavior](docs/rfcs/0010-overlay-interaction-behavior.md)
- [RFC 0011: Toast Timer and Live Region](docs/rfcs/0011-toast-timer-and-live-region.md)
- [RFC 0012: Listbox Overlay Behavior](docs/rfcs/0012-listbox-overlay-behavior.md)
- [RFC 0013: Date Picker and Calendar Keyboard Behavior](docs/rfcs/0013-date-picker-calendar-keyboard.md)
- [RFC 0014: Menu Keyboard Behavior](docs/rfcs/0014-menu-keyboard-behavior.md)
- [RFC 0015: Menubar Keyboard Behavior](docs/rfcs/0015-menubar-keyboard-behavior.md)
- [RFC 0016: Navigation Menu Interaction Behavior](docs/rfcs/0016-navigation-menu-interaction.md)
- [RFC 0017: Desktop Interaction Verification](docs/rfcs/0017-desktop-interaction-verification.md)
- [RFC 0018: Mobile Interaction Verification](docs/rfcs/0018-mobile-interaction-verification.md)
- [RFC 0019: Roving Group Interaction](docs/rfcs/0019-roving-group-interaction.md)
- [RFC 0020: Android Interaction Verification](docs/rfcs/0020-android-interaction-verification.md)
- [RFC 0021: Accordion Interaction](docs/rfcs/0021-accordion-interaction.md)
- [RFC 0022: Tooltip Hover And Focus Opening](docs/rfcs/0022-tooltip-hover-and-focus-opening.md)
- [RFC 0023: Hover Card Hover And Focus Opening](docs/rfcs/0023-hover-card-hover-and-focus-opening.md)
- [RFC 0024: Command Keyboard And Filtering](docs/rfcs/0024-command-keyboard-and-filtering.md)
- [RFC 0025: Right-To-Left Arrow Mirroring](docs/rfcs/0025-right-to-left-arrow-mirroring.md)
- [RFC 0026: Tabs Manual Activation And Vertical Orientation](docs/rfcs/0026-tabs-manual-activation-and-vertical-orientation.md)
- [RFC 0027: Combobox And Command Result Announcements](docs/rfcs/0027-combobox-and-command-result-announcements.md)
- [RFC 0028: Switch And Checkbox Change Events](docs/rfcs/0028-switch-and-checkbox-change-events.md)
- [RFC 0029: Button, Toggle, Input, And Textarea Events](docs/rfcs/0029-button-toggle-input-and-textarea-events.md)
- [RFC 0030: Slider Keyboard And Pointer Input](docs/rfcs/0030-slider-keyboard-and-pointer-input.md)
- [RFC 0031: Collapsible And Native Select Events](docs/rfcs/0031-collapsible-and-native-select-events.md)
- [RFC 0032: Input OTP Value Changes](docs/rfcs/0032-input-otp-value-changes.md)
- [RFC 0033: Pagination Page Changes](docs/rfcs/0033-pagination-page-changes.md)
- [RFC 0034: Carousel Slide Changes](docs/rfcs/0034-carousel-slide-changes.md)
- [RFC 0035: Control Label Overrides](docs/rfcs/0035-control-label-overrides.md)
- [RFC 0036: Resizable Handle Input](docs/rfcs/0036-resizable-handle-input.md)
- [RFC 0037: Sidebar Toggle And Items](docs/rfcs/0037-sidebar-toggle-and-items.md)
- [RFC 0038: Form Control Naming](docs/rfcs/0038-form-control-naming.md)
- [RFC 0039: Dialog Names](docs/rfcs/0039-dialog-names.md)
- [RFC 0040: Composite Widget Names](docs/rfcs/0040-composite-widget-names.md)
- [RFC 0041: Checkbox Indeterminate State](docs/rfcs/0041-checkbox-indeterminate-state.md)
- [RFC 0042: Slider Thumb Position And Vertical Orientation](docs/rfcs/0042-slider-thumb-position-and-vertical-orientation.md)
- [RFC 0043: Compiled Tailwind Browser Checks](docs/rfcs/0043-compiled-tailwind-browser-checks.md)
- [RFC 0044: Tailwind Utility Conflicts](docs/rfcs/0044-tailwind-utility-conflicts.md)
- [RFC 0045: Drawn Checkbox](docs/rfcs/0045-drawn-checkbox.md)
- [RFC 0046: Listbox Width Follows Trigger](docs/rfcs/0046-listbox-width-follows-trigger.md)
- [RFC 0047: Opt-in Dark Theme](docs/rfcs/0047-opt-in-dark-theme.md)
- [RFC 0048: Phone-width Preview Layout](docs/rfcs/0048-phone-width-preview-layout.md)
- [RFC 0049: Compiled Preview Stylesheet](docs/rfcs/0049-compiled-preview-stylesheet.md)
- [RFC 0050: Preview Theme Toggle](docs/rfcs/0050-preview-theme-toggle.md)
- [RFC 0051: Semantic Color Tokens](docs/rfcs/0051-semantic-color-tokens.md)
- [RFC 0052: Component Site](docs/rfcs/0052-component-site.md)
- [RFC 0053: Interactive Part Callbacks](docs/rfcs/0053-interactive-part-callbacks.md)
- [RFC 0054: Automated Accessibility Audit](docs/rfcs/0054-automated-accessibility-audit.md)
- [RFC 0055: Open State Accessibility Audit](docs/rfcs/0055-open-state-accessibility-audit.md)
- [RFC 0056: Published Crate Names](docs/rfcs/0056-published-crate-names.md)
- [RFC 0057: Theme Presets](docs/rfcs/0057-theme-presets.md)
- [RFC 0058: Status Variants](docs/rfcs/0058-status-variants.md)
- [RFC 0059: Display Components](docs/rfcs/0059-display-components.md)
- [RFC 0060: Input Components](docs/rfcs/0060-input-components.md)
- [RFC 0061: Mobile Navigation](docs/rfcs/0061-mobile-navigation.md)
- [RFC 0062: Multi-Select](docs/rfcs/0062-multi-select.md)
- [RFC 0063: Navigation Menu Submenus](docs/rfcs/0063-navigation-menu-submenus.md)
- [RFC 0064: Typed Date Input](docs/rfcs/0064-typed-date-input.md)
- [RFC 0065: Pie and Donut Charts](docs/rfcs/0065-pie-and-donut-charts.md)
- [RFC 0066: Template Parity](docs/rfcs/0066-template-parity.md)
- [RFC 0067: Menu Submenus](docs/rfcs/0067-menu-submenus.md)
- [RFC 0068: Modal Scroll Lock](docs/rfcs/0068-modal-scroll-lock.md)
- [RFC 0069: Off-Canvas Sidebar and Shortcut](docs/rfcs/0069-off-canvas-sidebar.md)
- [RFC 0070: Range Slider](docs/rfcs/0070-range-slider.md)
- [RFC 0071: Theme Controller](docs/rfcs/0071-theme-controller.md)
- [RFC 0072: Menu and Mockup](docs/rfcs/0072-menu-and-mockup.md)

## Verification Shortcuts

The repository includes `package.json` metadata for Node-based verification
aliases and opt-in browser smoke tests.

`.github/workflows/ci.yml` runs on GitHub Actions: rustfmt, Clippy, and the
default CI set on pull requests, and `npm run verify:release` on pushes to
`main`. The browser smoke workflow (`browser-smoke.yml`) stays inactive by
policy (RFC 0009).

Install JavaScript dependencies:

```bash
npm install
```

Run the default deterministic local gate:

```bash
npm run verify
```

This wraps preview/example smoke plus docs metadata checks. It does not run full
Rust workspace tests, browser automation that needs an installed browser,
screenshots, or release-only gates.

Run the local release gate before publishing:

```bash
npm run verify:release
```

This runs Rust workspace checks, CLI registry/list smoke, the default local
gate, Cargo workspace metadata checks, Cargo lock metadata checks, pre-commit
metadata checks, script metadata checks, feature checks, generated source-copy
fixture smoke, release docs consistency checks, package script wiring checks,
and CI browser docs checks. It also checks CI Plan documentation while keeping
browser installation and screenshots opt-in, verifies browser artifact policy
metadata, then checks repository hygiene for forbidden generated
artifacts and an active browser smoke workflow file. It also
checks Cargo publish metadata for the library and CLI crates without
packaging or publishing them, published package contents, the publish order,
and changelog metadata. Rendered component coverage metadata is part of this gate
and stays read-only.

Run deterministic preview and example gates only:

```bash
npm run verify:smoke
```

This wraps existing local gates for Web preview, Mobile Web profile, Desktop
preview, and example smoke output. It does not require Playwright browser
binaries.

Verify the docs-site catalog metadata contract only:

```bash
npm run verify:docs
```

For focused debugging, the aggregate command expands to:

```bash
npm run verify:docs-catalog
npm run verify:docs-catalog-page
npm run verify:docs-status
npm run verify:docs-structure
npm run verify:docs-routes
npm run verify:docs-source-preview
npm run verify:rfcs
npm run verify:readme
npm run verify:docs-index
npm run verify:docs-links
npm run verify:docs-anchors
```

This builds the catalog in memory from registry entries, templates, component
docs, crate features, and crate modules. The page check also verifies
`docs/components/catalog.md` matches the shared catalog builder output. The
status check verifies `docs/components/component-status.md` matches the local component
implementation surface. The structure check verifies public component docs keep
the required title, install, API, and accessibility sections. The route check
verifies site route metadata. The source preview check verifies template
metadata for the Code tab source previews. The README check verifies
verification shortcut command discoverability and summary coverage. The index
check verifies README and docs/README keep the required project entry points.
The link target check verifies tracked Markdown files do not reference missing
local files. The anchor check verifies local Markdown fragments match headings
or explicit anchors.

Verify focused metadata gates only:

```bash
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
npm run verify:rendered-component-dom
npm run verify:examples-metadata
npm run verify:css-inputs
npm run verify:rfcs
npm run verify:changelog
npm run verify:readme
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:ci-workflow-template
npm run verify:browser-artifact-policy
npm run verify:repo-hygiene
```

These checks validate committed metadata and documentation wiring. They do not
replace command behavior checks such as Rust tests, source-copy fixture smoke,
or browser-rendered verification.

Verify README verification shortcuts only:

```bash
npm run verify:readme
```

This checks command discoverability and summary coverage in this README. It
does not execute the referenced commands, crawl external links, render docs, or
rewrite prose.

Verify changelog metadata only:

```bash
npm run verify:changelog
```

This checks project-owned `CHANGELOG.md` structure, the Unreleased section,
Keep a Changelog and Conventional Commits references, and stale template-link
bans. It does not generate release notes, run git-cliff, derive changes from
Git history, create tags, publish releases, or rewrite commit history.

Verify component implementation status only:

```bash
npm run verify:docs-status
```

This checks that the generated component status snapshot still matches local
registry, template, docs, crate feature, crate module, source preview, and
source-copy target metadata. The coverage matrix is local wiring status, not
runtime visual parity.

Verify public component docs structure only:

```bash
npm run verify:docs-structure
```

This checks required component docs sections and generated command/feature
snippets. It does not score prose quality or assert runtime visual parity.

Verify documentation index consistency only:

```bash
npm run verify:docs-index
```

This checks that the root README and docs README link the required quality,
release, CI, site, RFC, and TODO entry points. It also checks that every
`package.json` `verify` and `verify:*` alias is documented in
`docs/quality-gates.md`, and that the quality gate release block matches the
direct `verify:release` command segments in order.

Verify local Markdown link targets only:

```bash
npm run verify:docs-links
```

This checks local relative links in tracked Markdown files. It does not validate
external URLs or heading fragments.

Verify local Markdown anchors only:

```bash
npm run verify:docs-anchors
```

This checks same-file and relative Markdown fragments in tracked Markdown files.
It does not validate external URL anchors or generated docs runtime routes.

Verify release documentation consistency only:

```bash
npm run verify:release-docs
```

This checks that release docs still distinguish the local aggregate command,
cover every direct `verify:release` command segment in order, reject extra
expanded release gate commands, and keep the opt-in browser smoke boundary
explicit.

Verify registry metadata only:

```bash
npm run verify:registry
```

This checks registry entry names, descriptions, source mappings, source-copy
targets, dependency references, and asset mappings. It does not execute CLI
commands or replace the Rust registry tests.

Verify Tailwind static token safety only:

```bash
npm run verify:tailwind-static
```

This checks shipped Rust source and templates for dynamic Tailwind utility
tokens such as `bg-{...}` and for bare data variants such as `data-disabled:`,
which match any attribute value. It does not compile Tailwind CSS or assert
visual parity.

Verify that class functions never join conflicting Tailwind utilities:

```bash
npm run verify:tailwind-conflicts
```

This compiles each utility with the Tailwind Node API and fails when a class
function joins a base class with a state class that sets the same property
under the same variant, where the stylesheet order would pick the winner.

Verify that the committed preview stylesheet matches the current classes:

```bash
npm run verify:preview-css
```

The Web, Desktop, and Mobile previews link
`examples/preview-states/assets/preview.generated.css`, compiled Tailwind for
the shared preview page. After changing a component or fixture class, run
`npm run css:preview` to regenerate it; this check fails until you do.

Verify npm verification alias wiring only:

```bash
npm run verify:package-scripts
```

This checks `package.json` script relationships and local script target
existence without executing Cargo, browser automation, generated fixture smoke,
or release commands.

Verify npm lockfile metadata only:

```bash
npm run verify:package-lock
```

This checks that committed `package-lock.json` root metadata matches
`package.json` without running npm install, contacting the registry, or
rewriting the lockfile.

Verify Cargo workspace metadata only:

```bash
npm run verify:cargo-workspace
```

This checks that `crates/` package manifests keep inheriting root workspace
package metadata and that `cargo metadata` resolves the same values. It does
not publish crates, package crates, contact crates.io, or check dependency
freshness.

Verify Cargo publish metadata only:

```bash
npm run verify:cargo-publish-metadata
```

This checks planned published crate descriptions, shared
README/keywords/categories metadata, example `publish = false` boundaries, and
release wiring, and that internal workspace dependencies declare the workspace
version alongside their local paths. It does not run `cargo publish`, run
`cargo package`, contact crates.io, replace repository URLs, or create package
archives.

Verify published package contents only:

```bash
npm run verify:package-contents
```

This runs `cargo package --list` for each publishable crate and checks that
every package ships its `LICENSE` and the CLI package contains every registry entry and template the CLI embeds. It does
not build package archives, run `cargo publish`, or contact crates.io.

Verify publish order metadata only:

```bash
npm run verify:publish-order
```

This checks the planned crate publish order across release docs and Cargo
publish metadata. It
does not create package archives, run `cargo package`, run `cargo publish`,
contact crates.io, check registry ownership, inspect credentials, change
dependency versions, or authorize a release.

Verify Cargo lockfile metadata only:

```bash
npm run verify:cargo-lock
```

This checks that committed `Cargo.lock` workspace package entries match Cargo
metadata without running `cargo update`, rewriting the lockfile, contacting
crates.io, or checking dependency freshness.

Verify pre-commit metadata only:

```bash
npm run verify:pre-commit
```

This checks committed `.pre-commit-config.yaml` local hook wiring and supporting
config files without installing hook environments, executing hooks, or checking
remote hook freshness.

Verify script metadata only:

```bash
npm run verify:scripts
```

This checks committed script shebangs, executable bits, and package-referenced
script targets without executing scripts, linting shell syntax, or rewriting
file modes.

Verify gitignore metadata only:

```bash
npm run verify:gitignore
```

This checks required generated directory, browser state, and screenshot artifact
patterns in `.gitignore`, plus the repository hygiene policy fragments that
reject tracked generated artifacts. It does not delete local files, inspect
ignored artifact contents, validate global Git excludes, or replace repository
hygiene tracking checks.

Verify preview state metadata only:

```bash
npm run verify:preview-state-metadata
```

This checks that the shared Web/Desktop preview state inventory, preview
binaries, structural preview verifiers, Tailwind CSS v4 source inputs, and
release wiring stay aligned. It does not launch browsers, run `dx serve`,
compile Tailwind output, inspect screenshots, or assert visual parity.

Verify mobile browser smoke metadata only:

```bash
npm run verify:mobile-browser-metadata
```

This checks the opt-in mobile browser smoke script, package alias, localhost
target, `390x844` viewport, selector contract, screenshot artifact pattern,
browser environment variables, and CI browser guidance. It does not launch
Playwright, start `dx serve`, install browsers, write screenshots, or validate
rendered output.

Verify rendered component coverage metadata only:

```bash
npm run verify:rendered-component-coverage
```

This checks `docs/components/rendered-coverage.json` against the docs catalog
and confirms `examples/preview-states` exposes stable
`data-component-preview` targets for every public component. It does not start
a server, launch a browser, write screenshots, update generated docs, change
component APIs, edit templates, or claim visual parity.

Verify rendered component browser DOM targets only:

```bash
npm run verify:rendered-component-dom
```

This opt-in Playwright command starts the Web preview and checks every
`data-component-preview` target from `docs/components/rendered-coverage.json`
in a real browser DOM. It requires Playwright Chromium or
`DIOXUS_UI_BROWSER_EXECUTABLE`, does not write screenshots or traces, and does
not claim visual parity or interaction coverage.

Verify Web preview screenshot smoke only:

```bash
npm run verify:web-screenshot-smoke
```

This opt-in Playwright command starts the Web preview and checks representative
desktop and mobile screenshot targets without writing screenshots by default.
It requires Playwright Chromium or `DIOXUS_UI_BROWSER_EXECUTABLE`, validates
stable preview panels, chart SVG output, fallback rows, overlay content,
interaction fixtures, and viewport dimensions, and does not compare pixels or
claim visual parity.

Verify runtime interaction fixtures only:

```bash
npm run verify:runtime-interactions
```

This opt-in Playwright command starts the Web preview and exercises the
`data-interaction-*` fixture targets for disclosure, overlay, selection,
keyboard-visible state, and scroll-status behavior, plus the real Dialog
and Alert Dialog components (Escape, overlay click, close and action buttons,
initial focus, Tab wrap, focus restore) the real Popover component
(anchored placement, flip near the viewport edge, Escape, outside click), and
the real Tooltip component (top placement, Escape, outside presses ignored),
real Toast and Sonner components (countdown dismissal, hover pause, dismiss
reasons, live region viewport), and real Select and Combobox components
(anchored listbox, arrow and typeahead navigation, selection), and a real
Date Picker with Calendar (focus entry, day keyboard movement, focus return),
real Dropdown and Context Menu components (menu navigation, activation,
point placement), a real Menubar (roving triggers, menu switching, focus
return), a real Navigation Menu (click, keyboard, and hover disclosure,
dismissal), real Tabs, Radio Group, and Toggle Group components (one Tab
stop, arrow movement, selection following focus), a real Accordion
(toggle reporting, trigger arrow movement, region links), Tooltip and
Hover Card hover and focus opening, a real Command (highlight movement,
query resets, choosing from the input), right-to-left arrow mirroring in
the roving groups and menus, vertical, manually activated Tabs, Combobox
and Command result status regions, Switch and Checkbox change events,
Button, Toggle, Input, and Textarea events, Slider keyboard and pointer
input, Collapsible and Native Select events, Input OTP typing, Pagination
page changes, Carousel slide changes, Resizable handle input, Sidebar
toggle and items, form control names, dialog names, composite widget names, the Checkbox mixed
state and drawn marks, Select and Combobox list widths, and Slider thumb position and vertical sliders, all with compiled
Tailwind, no conflicting utilities in any rendered class list, readable text contrast in the light and opt-in dark themes, the preview theme toggle, and a 375px layout with no sideways scroll. It requires Playwright
Chromium or `DIOXUS_UI_BROWSER_EXECUTABLE`, does not write screenshots or
traces, and does not claim full accessibility certification, native Desktop or
Mobile coverage, or visual parity.

`npm run verify:desktop-interactions` runs the same kinds of interactions in the
Desktop WebView. The Desktop preview runs an in-app self-test of ten
scenarios and exits with the result (RFC 0017). It opens a window and needs a
GUI session. `npm run verify:mobile-interactions` runs the same scenarios in an
iOS Simulator build (RFC 0018) and needs Xcode.
`npm run verify:android-interactions` runs them in an Android emulator
(RFC 0020) and needs the Android SDK, NDK, and an AVD.

Run all local browser smoke commands serially:

```bash
npm run verify:browser-local
```

This opt-in aggregate runs mobile browser smoke, rendered component DOM
verification, Web screenshot smoke, and runtime interaction verification in
order. It intentionally runs the browser commands serially because each command
starts its own Web preview server; do not parallelize these commands. The
aggregate keeps screenshots disabled by default and stays outside
`npm run verify` and `npm run verify:release`.

Verify repository hygiene only:

```bash
npm run verify:repo-hygiene
```

This checks that inactive browser workflow files, generated directories such as
`node_modules/` and `target/`, and known generated artifacts are not committed.
It reports drift but does not remove files.

Verify CI browser smoke documentation only:

```bash
npm run verify:ci-docs
```

This checks that the opt-in CI browser smoke guide and workflow template still
reference the current local gates without creating active workflow files.

Verify CI Plan documentation only:

```bash
npm run verify:ci-plan
```

This checks that the quality gate CI Plan still references the current PR,
release, and opt-in browser verification boundaries.

Verify CI browser workflow template metadata only:

```bash
npm run verify:ci-workflow-template
```

This checks the documented browser smoke workflow template, RFC 0009 activation
policy, CI browser guide, package alias, and absence of
`.github/workflows/browser-smoke.yml`. It does not create workflows, run GitHub
Actions, install browsers, upload artifacts, or change browser smoke rollout
policy.

Verify browser artifact policy metadata only:

```bash
npm run verify:browser-artifact-policy
```

This checks CI browser artifact guidance, workflow template upload fields, RFC
0009 artifact policy, `.gitignore` screenshot patterns, repository hygiene
boundaries, and release wiring. It does not launch browser automation, upload
artifacts, delete local files, enforce remote retention, or validate screenshot
pixels.

Browser-rendered Playwright smoke is opt-in and needs an explicit browser
install:

```bash
npx playwright install chromium
```

After Chromium is installed, run the opt-in mobile browser smoke:

```bash
npm run verify:mobile-browser
npm run verify:browser-local
```

Run the rendered component DOM verifier the same way:

```bash
npm run verify:rendered-component-dom
npm run verify:web-screenshot-smoke
npm run verify:runtime-interactions
npm run verify:site
```

If Playwright-managed Chromium is unavailable but local Chrome is installed,
use an explicit executable path:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:browser-local
```

For rendered component DOM verification with local Chrome:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:rendered-component-dom
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:web-screenshot-smoke
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:runtime-interactions
```

To save an ignored local mobile browser screenshot after the assertions pass,
add:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

To save ignored local Web preview screenshots after the assertions pass, add:

```bash
DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

For local Chrome plus screenshot capture:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_WEB_SCREENSHOT=1 npm run verify:web-screenshot-smoke
```

Screenshots use the ignored `dioxus-ui-mobile-browser-preview-*.png` pattern.
When screenshot capture is enabled, the smoke also validates the generated PNG
signature, byte size, and dimensions against the `390x844` mobile viewport
lower bound.
Web preview screenshots use the ignored `dioxus-ui-web-preview-*.png` pattern
and validate PNG signature, byte size, and dimensions against the desktop and
mobile viewport lower bounds.

For manual release-candidate screenshot review, use the
[Release Candidate Browser Review Runbook](docs/components/release-candidate-browser-review-runbook.md),
[Release Screenshot Review Checklist](docs/components/release-screenshot-review-checklist.md)
and copy review evidence into the
[Release Screenshot Review Notes Template](docs/components/release-screenshot-review-notes-template.md).

That browser smoke starts the Web preview, checks a mobile browser viewport, and
cleans up the server. It is not part of the release gate and is not part of
default release gates.

For CI setup options and non-blocking workflow policy, see the
[CI Browser Smoke Guide](docs/ci-browser-smoke.md) and
[CI Browser Workflow Template](docs/ci-browser-workflow-template.md).

## References

- Dioxus RSX and UI documentation: <https://dioxuslabs.com/learn/0.7/essentials/ui/rsx/>
- Dioxus components direction: <https://github.com/DioxusLabs/dioxus-components>
- Tailwind class detection: <https://tailwindcss.com/docs/detecting-classes-in-source-files>
- Tailwind v4 installation: <https://tailwindcss.com/docs/installation>
- Dioxus component macro docs: <https://docs.rs/dioxus/latest/dioxus/prelude/attr.component.html>
