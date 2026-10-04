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
npm run verify:release-notes-readiness
npm run verify:license-readiness
npm run verify:repository-identity-readiness
npm run verify:api-stability-readiness
npm run verify:cli-template-packaging-readiness
npm run verify:package-contents
npm run verify:registry-availability-readiness
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:publish-order
npm run verify:workspace-dependency-publish-readiness
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
npm run verify:release-warning-inventory
npm run verify:release-candidate-handoff
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
Publish readiness blocker checks are read-only and validate only the documented
placeholder repository URL, pre-1.0 API stability, release notes readiness,
root license file readiness, crates.io review, and workspace dependency
publish readiness blockers; they do not replace repository URLs, check
registries, run `cargo package`, run `cargo publish`, stabilize APIs, generate
changelogs, generate license text, change embedded CLI template delivery, or
change dependency versions.
Release notes readiness checks are read-only and validate only that
project-owned changelog structure exists and its Unreleased section records
first publish included scope, excluded scope, and known warnings; they do not
generate release notes, run git-cliff, derive changes from Git history, create
tags, or publish releases.
[Release Notes Readiness Preparation Plan](release-notes-readiness-preparation-plan.md)
defines the repository-safe decision pass that preceded the first-publish
release notes recorded in `CHANGELOG.md`.
[Release Notes Evidence Checklist](release-notes-evidence-checklist.md)
records included scope, excluded scope, known warnings, owners, and migration
note evidence before local changelog follow-up.
[Release Notes Local Follow-up Map](release-notes-local-follow-up-map.md)
maps approved, blocked, and deferred release-note decisions to local files and
gates without writing final notes.
License readiness checks are read-only and validate only workspace MIT license
metadata and committed root `LICENSE` file; they do not choose different
license terms, generate replacement license text, change copyright holders, run
`cargo package`, run `cargo publish`, or contact crates.io.
[License Decision Preparation Plan](license-decision-preparation-plan.md)
defines the repository-safe decision pass before maintainers accept, block, or
defer root license file readiness.
[License Decision Record Template](license-decision-record-template.md)
provides copyable license file, copyright holder, expression confirmation, file
commit approval, and rollback evidence fields.
[License Local Follow-up Map](license-local-follow-up-map.md) maps license
decision outcomes to local license file, metadata, and documentation follow-up
without generating or committing license text by itself.
Repository identity readiness checks are read-only and validate only that the
approved repository URL remains in workspace metadata; they do not choose a
different repository owner, change repository metadata, check crates.io
availability, run `cargo package`, or run `cargo publish`.
[Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md)
defines the repository-safe decision pass before maintainers accept, block, or
defer the canonical repository owner and URL.
[Repository Identity Decision Record Template](repository-identity-decision-record-template.md)
provides copyable owner, URL, remote availability, metadata approval, and
rollback evidence fields.
[Repository Identity Local Follow-up Map](repository-identity-local-follow-up-map.md)
maps identity decision outcomes to local metadata and documentation follow-up
without replacing the placeholder URL by itself.
API stability readiness checks are read-only and validate only workspace
version `0.1.0` and the accepted `0.1.x` first-publish API policy; they do not
stabilize component APIs, change crate versions, change the pre-`1.0`
breaking-change policy, generate migration guides, run `cargo package`, or run
`cargo publish`.
The [Public API Surface Inventory](public-api-surface-inventory.md) documents
the component, primitive, core, feature, registry, and source-copy surfaces
accepted for first publish.
[API Stability Review Checklist](api-stability-review-checklist.md) provides
the maintainer review steps to use for future API changes.
[API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md)
defines the repository-safe decision pass for accepting, blocking, or deferring
the current `0.1.x` API surface.
[API Stability Decision Record Template](api-stability-decision-record-template.md)
provides the copyable maintainer record for that decision without resolving the
blocker by itself.
[API Stability Local Follow-up Map](api-stability-local-follow-up-map.md)
maps approved, blocked, and deferred outcomes to local files and gates without
applying API changes.
CLI template packaging readiness checks are read-only. They validate that the
CLI embeds registry and template assets at compile time; they do not run
`cargo package`, run `cargo publish`, install the CLI, contact crates.io,
create package archives, or change embedded template contents.
`npm run verify:package-contents` runs `cargo package --list` for each
publishable crate and checks that the CLI package contains
every registry entry and template the CLI embeds; it lists package contents only and does not build
package archives, run `cargo publish`, or contact crates.io.
Registry availability readiness checks are read-only. They validate that the
crates.io name and ownership review blocker remains documented as deferred.
Deferral blocks crates.io publishing but not local release readiness, and the
checks themselves stay local; they do not contact crates.io, check crate name availability, check ownership, inspect
credentials, run `cargo package`, run `cargo publish`, or create package
archives.
Publish readiness coverage checks are read-only. They validate that every
current publish blocker has a focused readiness gate and resolved readiness
items keep their focused gates; they do not resolve
blockers, replace repository URLs, stabilize APIs, generate release notes,
generate license text, change embedded CLI template delivery, change
dependency versions, contact registries, inspect credentials, run
`cargo package`, run `cargo publish`, or create package archives.
Publish readiness runbook checks are read-only. They validate manual
resolution evidence for every current publish blocker; they do not resolve
blockers, replace repository URLs, stabilize APIs, generate release notes,
generate license text, change embedded CLI template delivery, change
dependency versions, contact registries, inspect credentials, run
`cargo package`, run `cargo publish`, or create package archives.
The [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
records required maintainer decisions, evidence, local follow-up files, and
safe validation commands before any blocker is resolved.
[First Publish Readiness Plan](first-publish-readiness-plan.md) defines the
repository-safe order for resolving those blockers without authorizing package
or publish commands.
[Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
records the minimum maintainer evidence required before blocker-specific local
follow-up starts.
[First Publish Local Implementation Map](first-publish-local-implementation-map.md)
maps approved blocker decisions to local files, gates, and rollback
considerations.
Publish order checks are read-only. They validate the planned crate publish
order; they do not create package archives, run `cargo package`, run
`cargo publish`, contact crates.io, check registry ownership, inspect
credentials, change dependency versions, or authorize a release.
Workspace dependency publish readiness checks are read-only. They validate
that internal workspace dependencies declare versions matching the workspace
version alongside local paths; they do not change dependency versions, run
`cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, create package archives, or authorize a
release.
[Workspace Dependency Publish Readiness Preparation Plan](workspace-dependency-publish-readiness-preparation-plan.md)
defines the repository-safe decision pass before any internal dependency
version metadata changes.
[Workspace Dependency Evidence Checklist](workspace-dependency-evidence-checklist.md)
records the maintainer evidence required before dependency metadata follow-up.
[Workspace Dependency Local Follow-up Map](workspace-dependency-local-follow-up-map.md)
maps approved dependency strategies to local manifest, metadata, and validation
follow-up without applying changes.
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
Release candidate handoff metadata checks are also read-only and validate only
the handoff checklist sections, release gate evidence, optional browser review
evidence, publish readiness blockers, warning inventory, artifact hygiene
boundaries, and discoverability links; they do not run release gates, launch
browser automation, capture screenshots, create artifacts, create Git tags,
publish packages, activate CI workflows, generate docs output, change component
APIs, or rewrite templates.
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
npm run verify:release-notes-readiness
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
npm run verify:release-warning-inventory
npm run verify:release-candidate-handoff
npm run verify:docs-status
npm run verify:docs-structure
npm run verify:docs-index
npm run verify:docs-links
npm run verify:docs-anchors
npm run verify:runtime-interactions
npm run verify:repo-hygiene
```

Browser-rendered Playwright smoke remains opt-in until a stable command is added
and proven.

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

## Release Candidate Handoff Checklist Plan

M118 should add a final handoff checklist for a release candidate after the
deterministic release gates and optional browser review have completed. The
checklist should be a maintainer-facing summary, not another automation layer.
The checklist lives at
[`release-candidate-handoff-checklist.md`](release-candidate-handoff-checklist.md).
Repository-root path: `docs/release-candidate-handoff-checklist.md`.

Required handoff evidence:

- release candidate identifier and branch or commit
- `npm run verify:release` result or focused gate results
- `npm run verify:browser-artifact-policy` result
- `npm run verify:repo-hygiene` result
- optional browser review runbook result
- screenshot review notes location if screenshots were captured
- screenshot retention outcome
- publish readiness blocker status
- release warning inventory status
- unresolved follow-up tasks
- final `git status --short` result

The checklist should link to:

- `docs/components/release-candidate-browser-review-runbook.md`
- `docs/components/release-screenshot-review-notes-template.md`
- `docs/components/screenshot-artifact-retention.md`
- `docs/publish-readiness-blockers.md`
- `docs/publish-readiness-resolution-runbook.md`
- `docs/release-warning-inventory-metadata.md`
- `docs/quality-gates.md`

Repository-safe boundaries:

- no package publishing
- no Git tags
- no release artifact creation
- no committed screenshots
- no artifact uploads
- no CI workflow activation
- no generated docs output
- no component API changes
- no source-copy template rewrites

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

The CLI embeds registry and template assets at compile time. Installed CLI
commands can read component metadata and generated source without relying on
the `crates/dioxus-ui-cli/registry/` or `crates/dioxus-ui-cli/templates/`
directories at runtime. Both directories live inside the CLI crate so
`cargo package` includes them in the published crate.

Cargo publish metadata is tracked by `npm run verify:cargo-publish-metadata`.
That check keeps descriptions and shared README/keywords/categories metadata
reviewable, but it does not replace a later publish-readiness review.
Known blockers for that review are tracked by
`npm run verify:publish-readiness-blockers`. The resolved CLI template
packaging strategy remains covered by
`npm run verify:cli-template-packaging-readiness`.

## First Publish Dry Run

M134 ran `cargo publish --workspace --dry-run` on 2026-10-04 with
cargo 1.98.1. Cargo packaged and compile-verified every crate from its
extracted archive, then aborted each upload:

| Crate | Packaged Files | Compressed Size |
| --- | --- | --- |
| `dioxus-ui-core` | 7 | 11.8 KiB |
| `dioxus-ui-primitives` | 26 | 35.0 KiB |
| `dioxus-ui` | 70 | 66.3 KiB |
| `dioxus-ui-cli` | 139 | 55.8 KiB |

The only warnings were `aborting upload due to dry run`. Package archives stay
in the Cargo target directory and are never committed.

`npm run verify:package-contents` keeps the package file lists checked in
`npm run verify:release`; rerun the dry run before an actual publish because it
also resolves dependencies against the live crates.io index.

## First Publish Steps

The actual publish is a release-owner action. It requires the crates.io
evidence in
[Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md#deferral)
first.

1. Record the crates.io evidence and mark registry availability resolved.
2. Rename the `CHANGELOG.md` Unreleased section to `[0.1.0]` with the release
   date and commit it.
3. Authenticate with `cargo login`; never commit or paste the token into the
   repository.
4. Run `cargo publish --workspace --dry-run` and confirm all four crates
   verify.
5. Run `cargo publish --workspace`. Cargo publishes each crate after its
   dependencies; the dry run uploaded `dioxus-ui-core`, `dioxus-ui-cli`,
   `dioxus-ui-primitives`, then `dioxus-ui`, which satisfies the same
   constraints as the publishing order above.
6. Optionally tag the release commit as `v0.1.0`.

## Known Pre-1.0 Limitations

- Dialog, Alert Dialog, Sheet, and Drawer implement Escape and overlay
  dismissal, initial focus, Tab wrap, and focus restore; Popover, Dropdown,
  Hover Card, and Tooltip implement anchored placement and dismissal. Only the
  Web renderer is browser-verified. Select and Combobox implement anchored
  listbox keyboard navigation and selection (see RFC 0012); multi-select and
  the input-inside-content Combobox layout are not supported. Date Picker,
  Navigation Menu, Context Menu, and Menubar stay controlled-only, and there is
  no DOM portal (see RFC 0010).
- Toast and Sonner dismiss themselves after a countdown that pauses on hover
  and focus, inside persistent live region viewports (see RFC 0011). Screen
  reader announcements are not automated, and swipe to dismiss is not
  implemented.
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
