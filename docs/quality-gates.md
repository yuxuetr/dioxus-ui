# Quality Gates

This document defines local and CI verification commands for `dioxus-ui`.

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
cargo test -p dioxus-ui-cli --test registry
scripts/generated-fixture-smoke.sh
```

This verifies:

- public registry component names match `dioxus-ui` crate feature names.
- public registry components have docs pages and catalog entries.
- registry template source and target paths match generated module names.
- every template file is registered exactly once, including `utils`.
- `scripts/feature-check.sh` covers every public component feature.
- `dxui list` returns public components.
- `dxui init` creates the generated project structure.
- `dxui add` can add every public component.
- generated `mod.rs` includes every public component and `utils`.
- generated code does not import `dioxus-ui-core` or `dioxus-ui-primitives`.
- generated source compiles with only `dioxus = "0.7"`.

## Feature Gate

Run after changes to `crates/dioxus-ui/Cargo.toml`, crate exports, or feature
gating:

```bash
scripts/feature-check.sh
```

This verifies:

- every public `dioxus-ui` feature compiles independently.
- static component feature combinations compile.
- primitive-backed feature combinations compile.
- all `dioxus-ui` features compile together.

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
cargo test -p dioxus-ui-cli --test registry
cargo run -p dioxus-ui-cli -- list
npm run verify:cargo-workspace
npm run verify:cargo-lock
npm run verify:registry
npm run verify:tailwind-static
npm run verify
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:package-lock
npm run verify:ci-docs
npm run verify:ci-plan
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
Cargo workspace metadata checks, `npm run verify`, component feature checks,
generated fixture smoke, release documentation consistency checks, package
script wiring checks, and CI browser documentation checks. It also checks CI
Plan documentation and repository hygiene.

`npm run verify:smoke` runs rendered preview structural checks and example smoke
output.

`npm run verify:preview` runs Web preview, Mobile Web profile, and Desktop
preview structural checks.

`npm run verify:web-preview` checks rendered Web preview screenshot
prerequisites without launching a browser.

`npm run verify:mobile-web-profile` checks source-level Mobile Web profile
coverage and fallback markers. It is not a native Mobile device or emulator
gate.

`npm run verify:desktop-preview` checks Desktop preview structural coverage
without launching a native WebView screenshot run.

`npm run verify:examples` runs the example smoke script for Web and Desktop demo
entry points.

`npm run verify:docs` runs all docs metadata and Markdown drift checks. Use the
individual commands below when isolating a specific failure.

`npm run verify:docs-catalog` checks the docs-site catalog contract in memory
from registry entries, templates, component docs, crate features, and crate
modules. It also verifies every public component has static catalog grouping
metadata. It must not write generated catalog artifacts.

`npm run verify:docs-catalog-page` checks that
`docs/components/catalog.md` matches the Markdown rendered from the shared
catalog builder, including grouped sections and the full flat index.

`npm run verify:docs-status` checks that `docs/components/status.md` matches
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
templates for dynamic Tailwind utility token interpolation. It is read-only and
does not compile Tailwind CSS, validate user-provided classes, or assert visual
parity.

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

`npm run verify:cargo-lock` checks that committed `Cargo.lock` workspace package
entries match `cargo metadata --locked --no-deps` workspace members. It is
read-only and does not run `cargo update`, rewrite the lockfile, contact
crates.io, or check dependency freshness.

`npm run verify:repo-hygiene` checks that inactive workflow files, generated
directories such as `node_modules/`, `target/`, `dist/`, and `build/`, and known
generated artifacts are not committed. It is read-only and reports drift without
removing files.

`npm run verify:ci-docs` checks that the opt-in CI browser smoke guide and
workflow template still reference the current local verification aliases and do
not add an active browser smoke workflow.

## Mobile Browser Smoke Gate

Run only when Playwright Chromium has been installed:

```bash
npx playwright install chromium
npm run verify:mobile-browser
```

For local machines with a supported Chrome executable, the command also accepts:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
```

Optional local screenshot capture:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

With an external Chrome executable:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

The screenshot artifact uses the ignored
`dioxus-ui-mobile-browser-preview-*.png` pattern and should not be committed.
When screenshot capture is enabled, the script validates the generated PNG
signature, nonzero byte size, and dimensions against the mobile viewport lower
bound.

This opt-in command starts the rendered Web preview, uses a mobile browser
viewport, asserts the Mobile Web profile and representative preview panels, and
cleans up the preview server. It remains outside default release gates until CI
or local release stability is proven.

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
cargo test -p dioxus-ui-cli --test registry
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
