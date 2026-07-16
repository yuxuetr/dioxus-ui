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
npm run verify:cargo-workspace
npm run verify:cargo-publish-metadata
npm run verify:publish-readiness-blockers
npm run verify:cargo-lock
npm run verify:pre-commit
npm run verify:scripts
npm run verify:gitignore
npm run verify:preview-state-metadata
npm run verify:mobile-browser-metadata
npm run verify:examples-metadata
npm run verify:css-inputs
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
npm run verify:ci-workflow-template
npm run verify:browser-artifact-policy
npm run verify:release-warning-inventory
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
Publish readiness blocker checks are read-only and validate only the documented
placeholder repository URL, pre-1.0 API stability, changelog ownership, CLI
template packaging, and crates.io review blockers; they do not replace
repository URLs, check registries, run `cargo package`, run `cargo publish`,
stabilize APIs, generate changelogs, or package CLI templates.
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
guide, package alias, and absence of an active workflow file; they do not run
GitHub Actions, install browsers, upload artifacts, or change rollout policy.
Browser artifact policy metadata checks are also read-only and validate only
committed artifact guidance, screenshot upload patterns, `.gitignore` coverage,
repository hygiene boundaries, and release wiring; they do not launch browser
automation, upload artifacts, delete local files, enforce remote retention, or
validate screenshot pixels.
Release warning inventory metadata checks are also read-only and validate only
the documented `block` `0.1.6` Rust future-incompatibility warning inventory,
Cargo lock evidence, quality gate notes, docs-site notes, and release wiring;
they do not run Cargo, parse compiler output, upgrade dependencies, or suppress warnings.
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
npm run verify:publish-readiness-blockers
npm run verify:cargo-lock
npm run verify:pre-commit
npm run verify:scripts
npm run verify:gitignore
npm run verify:examples-metadata
npm run verify:css-inputs
npm run verify:rfcs
npm run verify:readme
npm run verify:tailwind-static
npm run verify:ci-docs
npm run verify:ci-plan
npm run verify:ci-workflow-template
npm run verify:browser-artifact-policy
npm run verify:release-warning-inventory
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
bound of `390x844`.

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

Cargo publish metadata is tracked by `npm run verify:cargo-publish-metadata`.
That check keeps descriptions and shared README/keywords/categories metadata
reviewable, but it does not replace a later publish-readiness review.
Known blockers for that review are tracked by
`npm run verify:publish-readiness-blockers`.

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
- The release aggregate currently reports the known `block` `0.1.6` Rust
  future-incompatibility warning. It is tracked by the release warning
  inventory metadata gate until the dependency graph changes.
