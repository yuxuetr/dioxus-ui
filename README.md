# dioxus-ui

`dioxus-ui` aims to be a shadcn/ui-style component system for Dioxus:

- headless primitives for behavior, accessibility, state, and composition
- Tailwind CSS styled components as the default visual layer
- a CLI that copies component source into user projects
- an optional packaged crate for users who prefer dependency-based usage

The project is intentionally documentation-first. The initial goal is to define
the architecture and development sequence before implementing components.

## Product Direction

The library should not be a pure Tailwind component package. The target shape is:

```text
Dioxus headless/primitive logic layer
+ Tailwind default style layer
+ CLI component source generator
```

This gives users two workflows:

1. Add source code into an app and customize it freely.
2. Depend on a crate and enable only the component features they need.

The source-copy workflow is the priority for early versions because Dioxus and
the component APIs are expected to evolve quickly.

## Planned Repository Layout

```text
dioxus-ui/
├─ crates/
│  ├─ dioxus-ui-core/         # shared types, class merging, theme tokens
│  ├─ dioxus-ui-primitives/   # unstyled logic components
│  ├─ dioxus-ui/              # styled public components
│  └─ dioxus-ui-cli/          # dxui init / dxui add
├─ registry/                  # component metadata used by the CLI
├─ templates/                 # source templates copied by the CLI
├─ examples/
│  ├─ web-demo/
│  └─ desktop-demo/
└─ docs/
   └─ rfcs/
```

Example run commands are documented in [examples/README.md](examples/README.md).

## Planned Usage

### Source-Copy Mode

```bash
cargo install dioxus-ui-cli

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
src/components/ui/button.rs
src/components/ui/dialog.rs
src/components/ui/input.rs
src/components/ui/utils.rs
assets/dioxus-ui.css
```

For Tailwind CSS v4, `assets/dioxus-ui.css` should be an input stylesheet, not
a precompiled full Tailwind output:

```css
@import "tailwindcss";

@theme {
  --color-background: var(--dxui-background);
  --color-foreground: var(--dxui-foreground);
}
```

The user's Dioxus app build should produce the final CSS after scanning the app
source and generated component files.

### Crate Mode

```toml
[dependencies]
dioxus-ui = { version = "0.1", default-features = false, features = ["button", "input", "dialog"] }
```

```rust
use dioxus_ui::{Button, Dialog, Input};
```

Crate mode comes after the copied source API stabilizes.

## Component Scope

Early components:

- Button
- Input
- Textarea
- Label
- Checkbox
- Switch
- Badge
- Card
- Alert
- Avatar
- Separator
- Tabs
- Accordion

Later primitive-first components:

- Dialog
- Dropdown
- Popover
- Tooltip
- Toast
- Select
- Command

The later group needs stronger accessibility and interaction design, including
focus management, keyboard navigation, ARIA attributes, portal behavior,
outside-click handling, and positioning.

## Tailwind Rule

Tailwind class names must appear as complete source tokens. Runtime selection is
allowed, but runtime class construction is not.

Use this:

```rust
match variant {
  ButtonVariant::Primary => "bg-blue-600 text-white hover:bg-blue-700",
  ButtonVariant::Secondary => "bg-zinc-100 text-zinc-900 hover:bg-zinc-200",
}
```

Avoid this:

```rust
format!("bg-{}-500", color)
```

## Documentation

- [Design Overview](docs/design.md)
- [Roadmap](docs/roadmap.md)
- [Workspace Specification](docs/workspace.md)
- [Component API Specification](docs/component-api.md)
- [Release and Package Strategy](docs/release.md)
- [Release Candidate Handoff Checklist](docs/release-candidate-handoff-checklist.md)
- [Internal Trial Developer Guide](docs/internal-trial-developer-guide.md)
- [First Publish Readiness Plan](docs/first-publish-readiness-plan.md)
- [API Stability Decision Preparation Plan](docs/api-stability-decision-preparation-plan.md)
- [API Stability Decision Record Template](docs/api-stability-decision-record-template.md)
- [API Stability Local Follow-up Map](docs/api-stability-local-follow-up-map.md)
- [Release Notes Readiness Preparation Plan](docs/release-notes-readiness-preparation-plan.md)
- [Release Notes Evidence Checklist](docs/release-notes-evidence-checklist.md)
- [Release Notes Local Follow-up Map](docs/release-notes-local-follow-up-map.md)
- [Blocker Resolution Evidence Checklist](docs/blocker-resolution-evidence-checklist.md)
- [Workspace Dependency Publish Readiness Preparation Plan](docs/workspace-dependency-publish-readiness-preparation-plan.md)
- [Workspace Dependency Evidence Checklist](docs/workspace-dependency-evidence-checklist.md)
- [Workspace Dependency Local Follow-up Map](docs/workspace-dependency-local-follow-up-map.md)
- [First Publish Local Implementation Map](docs/first-publish-local-implementation-map.md)
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

## Verification Shortcuts

The repository includes `package.json` metadata for Node-based verification
aliases and future browser smoke tests.

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
metadata, validates the release warning inventory, validates release candidate
handoff metadata, then checks repository hygiene for forbidden generated
artifacts and inactive workflow files. It also
checks Cargo publish metadata for the planned library and CLI crates without
packaging or publishing them, then validates the publish readiness blocker
inventory, release notes readiness metadata, license readiness metadata,
repository identity readiness metadata, API stability readiness metadata, and
changelog metadata. Rendered component coverage metadata is part of this gate
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
status check verifies `docs/components/status.md` matches the local component
implementation surface. The structure check verifies public component docs keep
the required title, install, API, and accessibility sections. The route check
verifies future docs runtime route metadata. The source preview check verifies
template metadata for future source preview routes. The README check verifies
verification shortcut command discoverability and summary coverage. The index
check verifies README and docs/README keep the required project entry points.
The link target check verifies tracked Markdown files do not reference missing
local files. The anchor check verifies local Markdown fragments match headings
or explicit anchors.

Verify focused metadata gates only:

```bash
npm run verify:cargo-workspace
npm run verify:cargo-publish-metadata
npm run verify:publish-readiness-blockers
npm run verify:release-notes-readiness
npm run verify:license-readiness
npm run verify:repository-identity-readiness
npm run verify:api-stability-readiness
npm run verify:cli-template-packaging-readiness
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
npm run verify:release-warning-inventory
npm run verify:release-candidate-handoff
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
tokens such as `bg-{...}`. It does not compile Tailwind CSS or assert visual
parity.

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
release wiring. It does not run `cargo publish`, run `cargo package`, contact
crates.io, replace repository URLs, or create package archives.

Verify publish readiness blockers only:

```bash
npm run verify:publish-readiness-blockers
```

This checks the documented placeholder repository URL, pre-1.0 API stability,
release notes readiness, root license file readiness, crates.io review, and
workspace dependency publish readiness blockers. It does not replace
repository URLs, check registries, run `cargo package`, run `cargo publish`,
stabilize APIs, generate changelogs, generate license text, change embedded
CLI template delivery, or change dependency versions.

Verify release notes readiness metadata only:

```bash
npm run verify:release-notes-readiness
```

This checks that project-owned changelog structure exists while publish-ready
release notes are still unresolved. It does not generate release notes, run
git-cliff, derive changes from Git history, create tags, publish releases, or
decide release contents.
Use [Release Notes Blocker Handoff](docs/release-notes-blocker-handoff.md)
to consolidate first-publish scope, owner, warning, migration-note, rollback,
and validation evidence before resolving the release notes blocker.

Verify license readiness metadata only:

```bash
npm run verify:license-readiness
```

This checks workspace license metadata and missing root `LICENSE-MIT` and
`LICENSE-APACHE` files. It does not choose license terms, generate license
text, change copyright holders, run `cargo package`, run `cargo publish`, or
contact crates.io.
Use [License Decision Preparation Plan](docs/license-decision-preparation-plan.md)
for the maintainer decision pass before committing root license files.
[License Decision Record Template](docs/license-decision-record-template.md)
captures approved, blocked, and deferred outcomes.
[License Local Follow-up Map](docs/license-local-follow-up-map.md) maps
approved license decisions to local file, metadata, and documentation updates.
[License Blocker Handoff](docs/license-blocker-handoff.md) consolidates the
required license file evidence, rollback expectations, and validation commands
for the missing root license files blocker.

Verify repository identity readiness metadata only:

```bash
npm run verify:repository-identity-readiness
```

This checks that the approved repository URL remains in workspace metadata. It
does not choose a different repository owner, change repository metadata, check
crates.io availability, run `cargo package`, or run `cargo publish`.
Use [Repository Identity Decision Preparation Plan](docs/repository-identity-decision-preparation-plan.md)
for the maintainer decision pass that approved the canonical URL.
[Repository Identity Decision Record Template](docs/repository-identity-decision-record-template.md)
captures approved, blocked, and deferred outcomes.
[Repository Identity Local Follow-up Map](docs/repository-identity-local-follow-up-map.md)
maps approved identity decisions to local metadata and documentation updates.
[Repository Identity Blocker Handoff](docs/repository-identity-blocker-handoff.md)
consolidates the required owner evidence, rollback expectations, and validation
commands for the repository identity readiness item.

Verify API stability readiness metadata only:

```bash
npm run verify:api-stability-readiness
```

This checks workspace version `0.1.0` and the unresolved pre-`1.0` API
stability blocker. It does not stabilize component APIs, change crate versions,
decide semantic versioning policy, generate migration guides, run
`cargo package`, or run `cargo publish`.
The current API review surface is documented in
[Public API Surface Inventory](docs/public-api-surface-inventory.md).
Use [API Stability Review Checklist](docs/api-stability-review-checklist.md)
for maintainer review before resolving the API stability blocker.
[API Stability Blocker Handoff](docs/api-stability-blocker-handoff.md)
consolidates the public surface review inputs, rollback expectations, and
validation commands for the pre-`1.0` API stability blocker.

Verify CLI template packaging readiness metadata only:

```bash
npm run verify:cli-template-packaging-readiness
```

This checks that CLI registry and template assets are embedded at compile time.
It does not run `cargo package`, run `cargo publish`, install the CLI, contact
crates.io, create package archives, or change embedded template contents.

Verify registry availability readiness metadata only:

```bash
npm run verify:registry-availability-readiness
```

This checks that planned publishable crate names remain documented and that
the unresolved crates.io name and ownership review blocker remains documented.
It does not contact crates.io, check crate name availability, check ownership,
inspect credentials, run `cargo package`, run `cargo publish`, or create
package archives.
Use [Registry Availability Blocker Handoff](docs/registry-availability-blocker-handoff.md)
to consolidate crate names, owner, credential, publish-order, rollback, and
validation evidence before resolving the registry availability blocker.

Verify publish readiness coverage metadata only:

```bash
npm run verify:publish-readiness-coverage
```

This checks that every current publish blocker has a focused readiness gate,
metadata doc, README mention, package script, and release wiring. It does not
resolve blockers, replace repository URLs, stabilize APIs, generate release
notes, generate license text, change embedded CLI template delivery, contact
registries, inspect credentials, run `cargo package`, run `cargo publish`, or
create package archives.

Verify publish readiness runbook metadata only:

```bash
npm run verify:publish-readiness-runbook
```

This checks manual resolution evidence and follow-up update targets for every
current publish blocker. It does not resolve blockers, replace repository
URLs, stabilize APIs, generate release notes, generate license text, change
embedded CLI template delivery, contact registries, inspect credentials, run
`cargo package`, run `cargo publish`, or create package archives.

Use [Publish Readiness Decision Matrix](docs/publish-readiness-decision-matrix.md)
to record the maintainer decision, evidence, local follow-up files, and focused
validation commands for each blocker before resolving it.
Use [Publish Blocker Resolution Tracker](docs/publish-blocker-resolution-tracker.md)
to keep the six current blockers in one shared handoff view while evidence is
collected and local follow-up remains gated.
Use [First Publish Decision Packet](docs/first-publish-decision-packet.md) when
the release owner needs one copyable review surface for all six blockers.
Use [Approved Publish Blocker Resolution Plan](docs/approved-publish-blocker-resolution-plan.md)
for the approved local resolution sequence and the remaining crates.io evidence
boundary.
Use
[First Publish Maintainer Handoff Template](docs/first-publish-maintainer-handoff-template.md)
when those decisions need a copyable release-candidate note.

Verify publish order metadata only:

```bash
npm run verify:publish-order
```

This checks the planned crate publish order across release docs, Cargo publish
metadata, registry availability metadata, and the publish readiness runbook. It
does not create package archives, run `cargo package`, run `cargo publish`,
contact crates.io, check registry ownership, inspect credentials, change
dependency versions, or authorize a release.

Verify workspace dependency publish readiness metadata only:

```bash
npm run verify:workspace-dependency-publish-readiness
```

This checks that path-only internal workspace dependencies remain documented
as unresolved publish readiness work. It does not change dependency versions,
run `cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, create package archives, or authorize a
release.
Use [Workspace Dependency Blocker Handoff](docs/workspace-dependency-blocker-handoff.md)
to consolidate internal dependency graph, version policy, local development,
publish-order, rollback, and validation evidence before resolving the workspace
dependency publish readiness blocker.

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
keyboard-visible state, and scroll-status behavior. It requires Playwright
Chromium or `DIOXUS_UI_BROWSER_EXECUTABLE`, does not write screenshots or
traces, and does not claim full accessibility certification, native Desktop or
Mobile coverage, or visual parity.

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

Verify release warning inventory metadata only:

```bash
npm run verify:release-warning-inventory
```

This checks the documented `block` `0.1.6` Rust future-incompatibility warning
inventory, Cargo lock evidence, release docs, quality gate notes, docs-site
notes, and release wiring. It does not run Cargo, parse live compiler output,
execute `cargo report`, upgrade dependencies, or suppress warnings.

Verify release candidate handoff metadata only:

```bash
npm run verify:release-candidate-handoff
```

This checks the final handoff checklist sections, release gate evidence,
optional browser review evidence, publish readiness blockers, warning
inventory, artifact hygiene boundaries, and discoverability links. It does not
run release gates, launch browser automation, capture screenshots, create
artifacts, create Git tags, publish packages, activate CI workflows, generate
docs output, change component APIs, or rewrite templates.

Future browser-rendered Playwright smoke will require an explicit browser
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
For final maintainer handoff, use the
[Release Candidate Handoff Checklist](docs/release-candidate-handoff-checklist.md).

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
