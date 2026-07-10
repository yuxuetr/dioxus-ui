# Release and Package Strategy

## Goal

`dioxus-ui` supports two distribution modes:

- source-copy mode through `dxui add`
- crate mode through `dioxus-ui` feature flags

Source-copy mode remains the preferred early release path. Crate mode should be
published only after component APIs are stable enough that users can depend on
them without needing to edit internals.

## Package Set

Planned published crates:

```text
dioxus-ui-core
dioxus-ui-primitives
dioxus-ui
dioxus-ui-cli
```

Publishing order:

1. `dioxus-ui-core`
2. `dioxus-ui-primitives`
3. `dioxus-ui`
4. `dioxus-ui-cli`

## Feature Policy

The styled crate should keep `default = []`.

Users opt into components:

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["button", "dialog"] }
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
dialog = ["dioxus-ui-primitives/dialog"]
popover = ["dioxus-ui-primitives/popover"]
tooltip = ["dioxus-ui-primitives/tooltip"]
select = ["dioxus-ui-primitives/select"]
dropdown = ["dioxus-ui-primitives/dropdown"]
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
cargo test -p dioxus-ui-cli --test registry
cargo run -p dioxus-ui-cli -- list
scripts/example-smoke.sh
node scripts/web-preview-verify.mjs
node scripts/mobile-web-profile-verify.mjs
node scripts/desktop-preview-verify.mjs
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
node scripts/release-docs-verify.mjs
npm run verify:package-scripts
npm run verify:ci-docs
npm run verify:ci-plan
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
Package script wiring checks are also part of the release aggregate, but they
only inspect `package.json`; they do not execute the release gate recursively.
CI browser docs checks are read-only and do not create workflow files.
CI Plan checks are also read-only and validate documentation only.
Repository hygiene checks are read-only and report forbidden committed
artifacts without deleting files.
Registry metadata checks are part of `npm run verify:release` and validate only
registry JSON names, descriptions, source mappings, source-copy targets,
dependency references, and asset mappings.
Component status checks are part of `npm run verify:docs` and validate only the
local implementation snapshot and wiring coverage matrix derived from catalog
metadata.
Component docs structure checks are part of `npm run verify:docs` and validate
only required public component docs sections plus generated command and feature
snippets.
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
npm run verify:ci-docs
npm run verify:ci-plan
npm run verify:docs-status
npm run verify:docs-structure
npm run verify:docs-index
npm run verify:docs-links
npm run verify:docs-anchors
npm run verify:repo-hygiene
```

Browser-rendered Playwright smoke remains opt-in until a stable command is added
and proven.

Optional browser smoke:

```bash
npx playwright install chromium
npm run verify:mobile-browser
```

Local external Chrome fallback:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" npm run verify:mobile-browser
```

Optional local screenshot artifact:

```bash
DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

External Chrome plus screenshot artifact:

```bash
DIOXUS_UI_BROWSER_EXECUTABLE="/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT=1 npm run verify:mobile-browser
```

Generated screenshots match the ignored
`dioxus-ui-mobile-browser-preview-*.png` pattern.
When screenshot capture is enabled, the script validates the generated PNG
signature, nonzero byte size, and dimensions against the mobile viewport lower
bound.

This is not part of the release gate yet. It verifies mobile browser rendering
of the Web preview, not native Mobile behavior.

For CI setup options and non-blocking workflow policy, see
`docs/ci-browser-smoke.md` and `docs/ci-browser-workflow-template.md`. Do not
treat browser smoke as a required release gate until a reviewed workflow exists.
Workflow activation and required-gate promotion should follow
`docs/rfcs/0009-ci-browser-workflow-activation.md`.

Smoke commands:

```bash
cargo run -p dioxus-ui-cli -- init --root /tmp/dxui-release-smoke
cargo run -p dioxus-ui-cli -- add button --root /tmp/dxui-release-smoke
cargo run -p dioxus-ui-cli -- add dialog --root /tmp/dxui-release-smoke
```

Manual review:

- generated `assets/dioxus-ui.css` uses Tailwind CSS v4 syntax
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

The first CLI release can support:

```text
dxui init
dxui list
dxui add <component>
```

The CLI currently reads registry and templates from the repository layout. A
publish-ready CLI should either embed templates at compile time or package them
in a stable install location.

## Known Pre-1.0 Limitations

- Overlay primitives define state/config contracts but do not implement full
  focus trap, DOM portal, or positioning engines yet.
- Generated templates include a local `utils.rs` helper module and should not
  require `dioxus-ui-core` or `dioxus-ui-primitives` in source-copy mode.
- Web has a rendered preview shell and screenshot procedure. Desktop has a
  rendered preview shell and structural gate, but Desktop WebView screenshot
  capture is currently unsupported because the native preview window is not
  repeatable in local probes.
- Mobile has a Web profile structural gate for source-level mobile viewport and
  fallback markers, but no native device or emulator gate yet.
- Mobile browser smoke is documented as infeasible for release gates until
  browser automation dependencies are made portable.
