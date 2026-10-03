# Documentation Site Plan

## Current Site Shape

The documentation site is currently Markdown-first:

```text
docs/
├─ README.md
├─ components/
│  └─ README.md
├─ component-api.md
├─ design.md
├─ release.md
├─ roadmap.md
├─ workspace.md
└─ rfcs/
```

This keeps the project easy to review before choosing a docs runtime.

## Future Site Runtime

The site should eventually be a Dioxus Web app or static site that uses the same
component crate and registry metadata as the CLI.

Required views:

- component catalog
- component detail page
- source-copy command
- crate feature usage
- accessibility notes
- Web/Desktop preview tabs
- generated source preview

## Preview Contract

Each component preview should show:

- default state
- variant or role-specific states
- disabled state when applicable
- invalid state when applicable
- density differences when applicable
- keyboard and ARIA notes for interactive components

Overlay previews should also show:

- open state
- closed state
- primitive config defaults
- mobile/desktop behavior notes

## Data Sources

The docs site should derive its catalog from:

- `crates/dioxus-ui-cli/registry/*.json`
- `crates/dioxus-ui-cli/templates/*.rs`
- public crate features
- component docs in `docs/components`

Duplicating component metadata manually should be avoided once the docs site
becomes executable.

## M53 Catalog Data Contract

Before building a visual docs runtime, the project should expose a deterministic
catalog contract that can be derived from local source files.

Required catalog fields:

| Field | Source | Notes |
| --- | --- | --- |
| `name` | `crates/dioxus-ui-cli/registry/*.json` | Stable CLI/source-copy component id. |
| `description` | `crates/dioxus-ui-cli/registry/*.json` | Short catalog summary. |
| `registry_path` | filesystem | Path to the registry entry. |
| `template_path` | registry file list | Primary source-copy template path. |
| `docs_path` | `docs/components/{name}.md` | Component detail markdown page. |
| `crate_feature` | `crates/dioxus-ui/Cargo.toml` | Feature users enable for crate mode. |
| `crate_module` | `crates/dioxus-ui/src/{name}.rs` | Styled crate module path. |
| `source_copy_target` | registry file list | Generated target path for `dxui add`. |

Derived catalog fields:

- `slug`: same as `name`
- `title`: title-cased `name`
- `crate_import`: module name with dashes converted to underscores
- `source_copy_command`: `dxui add {name}`
- `crate_feature_toml`: `dioxus-ui = { features = ["{name}"] }`

Intentional exceptions:

- `utils` remains a source-copy helper, not a catalog component page.
- Planning and strategy markdown files under `docs/components` are not component
  detail pages unless their basename matches a registry component.

Deferred visual-runtime fields:

- preview image paths
- rendered Web/Desktop route ids
- screenshot artifact paths
- visual state matrix ids
- interactive examples

Those fields should be added only after the catalog metadata check is stable.

## Catalog Verification

The current catalog contract is verified by:

```bash
npm run verify:docs-catalog
```

The command builds the catalog in memory and fails on missing required fields or
surface drift. It verifies:

- public component registry entries
- source-copy template paths and generated targets
- component docs pages
- crate feature names
- styled crate module paths
- `lib.rs` public module exports

The command intentionally does not write a generated catalog artifact. A future
docs runtime should either call the same source-reading logic or introduce a
generated artifact only after the contract is stable and reviewed.

## M53 Final Result

M53 completed the docs-site catalog data contract and verification gate.

Validation ran:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
npm run verify:docs-catalog
git diff --check
```

All commands passed. `npm run verify:docs-catalog` reported:

```text
publicComponents: 64
registryEntries: 65
sourceCopyHelpers: ["utils"]
templates: 65
crateModules: 64
crateFeatures: 64
componentDocs: 64
```

No generated catalog artifact was written. The next docs-site milestone can
extract shared catalog-building logic for a future runtime or start a minimal
static catalog view using the same contract.

## M54 Shared Builder Plan

The catalog-building logic should be reusable before a visual docs runtime
exists.

Extraction boundary:

- shared builder module: read local sources, normalize component names, build
  in-memory catalog records, and return summary counts
- verification script: call the shared builder, enforce required fields, print
  summary output, and fail on drift
- future docs runtime: consume the shared builder or a reviewed generated
  artifact, but should not duplicate registry/docs/feature parsing logic

Out of scope for M54:

- writing `catalog.json`
- adding a visual docs route
- changing component APIs
- adding preview image or route ids to catalog records

The existing `npm run verify:docs-catalog` behavior should remain stable after
the extraction.

## M54 Shared Builder Usage

The shared catalog builder lives at:

```text
scripts/docs-catalog-builder.mjs
```

It exports `buildDocsCatalog(options)`. The default call reads from the current
repository root and returns:

- `catalog`: normalized public component records
- `registryNames`, `templateNames`, `crateModuleNames`, `docsNames`,
  `featureNames`, and `libModuleNames`: source inventories used for drift checks
- `publicComponentNames`: registry entries excluding source-copy helpers
- `sourceCopyHelpers`: helper entries such as `utils`
- `summary`: stable counts for reporting

The verification entry point remains:

```bash
npm run verify:docs-catalog
```

`scripts/docs-catalog-verify.mjs` consumes the shared builder, then owns the
validation rules and failure messages. Future docs runtime code should reuse the
builder output directly, or move the builder behind a package boundary if the
runtime needs to import it from Rust/Dioxus tooling. It should not reimplement
registry, template, docs, feature, or module parsing.

The builder is intentionally read-only. M54 still does not create
`catalog.json`, preview route ids, screenshot artifact paths, or rendered docs
pages.

## M54 Final Result

M54 extracted the docs catalog source-reading and record-building logic into a
shared JavaScript module while keeping validation behavior in the existing
verification entry point.

Validation ran:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
npm run verify:docs-catalog
git diff --check
```

All commands passed. `npm run verify:docs-catalog` reported:

```text
publicComponents: 64
registryEntries: 65
sourceCopyHelpers: ["utils"]
templates: 65
crateModules: 64
crateFeatures: 64
componentDocs: 64
```

No generated catalog JSON artifact was written, and no preview server remained
listening on port 45237.

## M55 Static Catalog View Plan

The next docs-site step is a deterministic Markdown catalog view generated from
the shared builder. This gives reviewers and users a readable component index
before a routed Dioxus docs runtime exists.

Static page target:

```text
docs/components/catalog.md
```

The page should include one row per public component with:

- component title and docs link
- registry description
- `dxui add` source-copy command
- crate feature name
- template path
- source-copy target path

Generation boundary:

- `scripts/docs-catalog-builder.mjs` remains the only source-reading path
- a small renderer script may convert builder output to Markdown
- a verification script should fail when `docs/components/catalog.md` drifts
  from current builder output

Out of scope for M55:

- visual docs routes
- preview screenshots or image paths
- generated JSON catalog artifacts
- component API changes
- runtime preview tabs

The existing `npm run verify:docs-catalog` command should continue to verify
metadata coverage. The new static-page verification should be additive.

## M55 Final Result

M55 added a deterministic static component catalog page generated from the
shared docs catalog builder:

```text
docs/components/catalog.md
```

The Markdown renderer lives at:

```text
scripts/docs-catalog-markdown.mjs
```

The drift gate lives at:

```text
scripts/docs-catalog-markdown-verify.mjs
```

Validation ran:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
npm run verify:docs-catalog
npm run verify:docs-catalog-page
git diff --check
```

All commands passed. The static page lists 64 public components and remains a
Markdown review artifact, not a generated JSON catalog or visual docs route.

## M56 Static Catalog Grouping Plan

The flat static catalog is accurate but hard to scan. M56 should add a small
category taxonomy to the static catalog output before any visual docs navigation
exists.

Initial category taxonomy:

- Actions: buttons, toggles, command-style controls, and keyboard hints
- Forms: form controls, field composition, labels, and date entry
- Overlays: dialogs, drawers, popovers, menus, tooltips, and hover cards
- Navigation: breadcrumbs, pagination, tabs, sidebar, and navigation menus
- Layout: cards, separators, scroll areas, resizable panels, and structural
  primitives
- Data Display: tables, charts, progress, avatars, badges, empty states, and
  typography
- Feedback: alerts, toast/sonner, skeleton, and spinner
- Messaging: attachment, bubble, message, marker, and message scroller

Metadata ownership:

- Keep grouping metadata near the docs catalog builder for M56.
- Do not add category fields to every registry entry until the taxonomy has
  proven useful.
- The builder should fail on missing grouping metadata so new public components
  cannot silently disappear from grouped docs.

Out of scope for M56:

- visual docs navigation
- rendered route generation
- registry schema changes
- component API changes
- screenshot or preview asset fields

The grouped catalog should still preserve the flat table because it remains the
fastest way to audit CLI commands, feature names, templates, and source-copy
targets.

## M56 Final Result

M56 added static catalog grouping metadata and rendered grouped sections into:

```text
docs/components/catalog.md
```

The shared builder now returns `category` and `category_label` for each public
component, plus a `catalogCategories` summary count. The verification gate fails
when a public component has missing or unknown grouping metadata.

Current category count:

```text
catalogCategories: 8
```

Validation ran:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
npm run verify:docs-catalog
npm run verify:docs-catalog-page
git diff --check
```

All commands passed. The grouping metadata remains owned by the docs catalog
builder for now; registry schema changes remain deferred until the taxonomy is
proven useful.

## M57 Docs Route Manifest Plan

The static catalog now has enough metadata to define future docs runtime routes
without rendering a routed app yet. M57 should add a deterministic route
manifest derived from the shared builder.

Static route fields:

- `docs_route`: canonical future runtime route for a component detail page
- `markdown_path`: current markdown source for that route
- `category_route`: grouped catalog anchor for the component category
- `category_anchor`: stable anchor id derived from the category id
- `source_route`: future source-copy preview route placeholder

Initial route shape:

```text
/components
/components/{slug}
/components#category-{category}
/components/{slug}/source
```

Only the route manifest should be committed in M57. A Dioxus router, visual
navigation, rendered source preview, screenshots, and browser route assertions
remain out of scope.

The manifest should be generated from `scripts/docs-catalog-builder.mjs` and
verified for drift so future route work does not duplicate component metadata.

## M57 Final Result

M57 added static route metadata to the docs catalog builder and rendered a
reviewable route manifest:

```text
docs/components/routes.md
```

The route manifest renderer lives at:

```text
scripts/docs-route-manifest-markdown.mjs
```

The drift gate lives at:

```text
scripts/docs-route-manifest-verify.mjs
```

Validation ran:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
npm run verify:docs-catalog
npm run verify:docs-catalog-page
npm run verify:docs-routes
git diff --check
```

All commands passed. The manifest defines 64 component routes and 8 category
routes. It remains a static planning artifact; no Dioxus router, rendered route,
browser route assertion, or generated JSON route file was added.

## M58 Source Preview Manifest Plan

M57 reserved `/components/{slug}/source` routes. M58 should define the static
source preview metadata those routes need before rendering source previews in a
Dioxus docs runtime.

Static source preview fields:

- `source_preview_route`: same route as the component `source_route`
- `source_preview_path`: template file used by `dxui add`
- `source_preview_target`: source-copy target path generated into user projects
- `source_preview_language`: language hint for future syntax highlighting
- `source_preview_lines`: deterministic line count for the template
- `source_preview_bytes`: deterministic byte count for the template

Out of scope for M58:

- embedding full template source in docs pages
- syntax highlighting
- rendered source preview routes
- browser route assertions
- generated JSON source manifests
- component API changes

The source preview manifest should be generated from
`scripts/docs-catalog-builder.mjs` and verified for drift. It should remain a
reviewable Markdown artifact until the visual docs runtime exists.

## M58 Final Result

M58 added source preview metadata to the docs catalog builder and rendered a
reviewable source preview manifest:

```text
docs/components/source-preview.md
```

The source preview manifest renderer lives at:

```text
scripts/docs-source-preview-markdown.mjs
```

The drift gate lives at:

```text
scripts/docs-source-preview-verify.mjs
```

Validation ran:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
npm run verify:docs-catalog
npm run verify:docs-catalog-page
npm run verify:docs-routes
npm run verify:docs-source-preview
git diff --check
```

All commands passed. The manifest defines 64 source preview routes with template
path, target path, language, line count, and byte count. It does not embed full
template source or add rendered source preview routes.

## M59 Docs Metadata Gate Plan

The docs metadata surface now has four related verification commands:

```bash
npm run verify:docs-catalog
npm run verify:docs-catalog-page
npm run verify:docs-routes
npm run verify:docs-source-preview
```

M59 should add one aggregate command for local docs metadata checks while
keeping the individual commands available for focused debugging.

Aggregate command scope:

- catalog contract verification
- static catalog Markdown drift verification
- route manifest drift verification
- source preview manifest drift verification

Out of scope:

- browser automation
- screenshots
- `dx serve`
- rendered Dioxus docs runtime
- Rust workspace tests
- generated JSON artifacts

The aggregate gate should be a convenience alias, not a replacement for release
or browser smoke gates.

## M59 Aggregate Gate Usage

Use the aggregate docs metadata gate for normal local docs checks:

```bash
npm run verify:docs
```

## M125 First Publish Readiness Planning

M125 adds:

```text
docs/first-publish-readiness-plan.md
docs/blocker-resolution-evidence-checklist.md
docs/first-publish-local-implementation-map.md
```

The plan defines the repository-safe order for resolving remaining first
publish blockers:

- repository identity
- root license files
- pre-1.0 API stability
- release notes readiness
- registry availability
- workspace dependency publish readiness

It keeps maintainer decisions separate from local implementation follow-up and
does not authorize repository URL changes, license text generation, crates.io
contact, package archives, publish commands, version changes, tags, screenshots,
traces, or CI workflow activation.

The blocker resolution evidence checklist records the minimum maintainer
evidence required before each blocker can move to local implementation
follow-up. It keeps focused readiness gates attached to each blocker without
marking any current blocker as resolved.

The local implementation map records expected files, focused gates, common
follow-up checks, and rollback considerations for approved blocker decisions
without applying those changes.

## M126 API Stability Decision Preparation

M126 starts the decision preparation pass for the pre-`1.0` API stability
publish blocker:

```text
docs/api-stability-decision-preparation-plan.md
docs/api-stability-decision-record-template.md
docs/api-stability-local-follow-up-map.md
```

The plan kept API stability unresolved until maintainers decided whether the
current `0.1.x` crate-mode APIs are acceptable for first publish or whether a
focused API stabilization milestone is required. M133 records the accepted
`0.1.x` first-publish API policy. It does not rewrite APIs,
change versions, generate migration guides, package crates, publish crates,
create tags, or approve stability.

The decision record template captures accepted, blocked, and deferred outcomes,
surface review notes, breaking-change policy, migration note expectations, and
local follow-up owners without resolving the blocker by itself.

The local follow-up map separates source-copy compatibility follow-up from
crate-mode stability follow-up and maps approved, blocked, and deferred
decisions to local files and gates without applying API changes.

## M127 Workspace Dependency Publish Readiness Preparation

M127 starts the decision preparation pass for workspace dependency publish
readiness:

```text
docs/workspace-dependency-publish-readiness-preparation-plan.md
docs/workspace-dependency-evidence-checklist.md
docs/workspace-dependency-local-follow-up-map.md
```

The plan separates local development path dependencies from publish readiness
requirements, keeps the planned publish order linked to dependency metadata,
and does not change dependency versions, rewrite manifests, run `cargo package`,
run `cargo publish`, contact crates.io, inspect credentials, create package
archives, create tags, or authorize publishing.

The evidence checklist records required maintainer input for publishable crate
set, publish order, internal dependency graph, version policy, local development
behavior, and release-owner boundaries before local manifest follow-up begins.

The local follow-up map separates local workspace path dependencies from
publishable dependency version metadata and maps approved, blocked, and
deferred strategies to local files and gates without applying manifest changes.

## M128 Release Notes Readiness Preparation

M128 starts the decision preparation pass for first-publish release notes:

```text
docs/release-notes-readiness-preparation-plan.md
docs/release-notes-evidence-checklist.md
docs/release-notes-local-follow-up-map.md
```

The plan keeps project-owned changelog structure separate from publish-ready
release note completeness. It records the safe order for deciding included
changes, excluded changes, known warnings, changelog owner, and release owner
without generating release notes, deriving changes from Git history, running
git-cliff, creating tags, creating GitHub releases, creating package archives,
or publishing crates. M133 records the first publish release note scope in
`CHANGELOG.md`.

The evidence checklist records release owner, changelog owner, included scope,
excluded scope, known warnings, migration note expectation, and release
boundary evidence before local changelog follow-up begins.

The local follow-up map separates changelog structure ownership from
publish-ready release note completeness and maps approved, blocked, and
deferred decisions to local files and gates without writing final notes.

## Internal Trial Developer Guide

Internal source-copy trials are documented in:

```text
docs/internal-trial-developer-guide.md
```

The guide explains how to run the local CLI with `cargo run -p dioxus-ui-cli`,
initialize a trial app, add generated components, validate Tailwind CSS v4 input
styling, collect API/accessibility/runtime feedback, and keep the six publish
blockers as coordinated release-owner work rather than accidental trial scope.

Use the individual commands only when isolating a specific failure:

```bash
npm run verify:docs-catalog
npm run verify:docs-catalog-page
npm run verify:docs-routes
npm run verify:docs-source-preview
```

The aggregate command is intentionally limited to deterministic metadata and
Markdown drift checks. It does not run browser automation, screenshots, `dx
serve`, or Rust workspace tests.

## M59 Final Result

M59 added a single aggregate docs metadata gate:

```bash
npm run verify:docs
```

The command runs:

```bash
npm run verify:docs-catalog
npm run verify:docs-catalog-page
npm run verify:docs-routes
npm run verify:docs-source-preview
```

Validation ran:

```bash
npm run verify:smoke
cargo test --workspace --all-features -q
npm run verify:docs
git diff --check
```

All commands passed. The aggregate gate remains a convenience alias for
deterministic docs metadata and Markdown drift checks; browser, screenshot,
`dx serve`, and Rust workspace gates remain separate.

## M60 Local Verification Gate Plan

The project now has two stable npm aggregate gates:

```bash
npm run verify:smoke
npm run verify:docs
```

M60 should add one local default verification command that runs both. The goal is
to give contributors a single deterministic npm entry point before opening a PR
or making docs/catalog changes.

Aggregate command scope:

- rendered preview structural checks
- example smoke output
- docs metadata contract checks
- Markdown drift checks for catalog, routes, and source preview manifests

Out of scope:

- full Rust workspace tests
- browser automation that requires installed Chromium
- screenshots
- `dx serve`
- CI workflow activation
- release-only gates

The command should be a local convenience alias. Focused commands and full
workspace Rust tests should remain available and documented separately.

## M60 Local Gate Usage

Use the default local deterministic gate before handing off routine docs,
catalog, preview, or example-smoke changes:

```bash
npm run verify
```

The command runs:

```bash
npm run verify:smoke
npm run verify:docs
```

Use `npm run verify:smoke` for preview/example-only changes and
`npm run verify:docs` for catalog or generated Markdown drift changes.
Full Rust workspace tests, browser automation, screenshots, `dx serve`, and
release-only gates remain separate.

## M60 Final Result

M60 added a single default local npm verification command:

```bash
npm run verify
```

The command runs:

```bash
npm run verify:smoke
npm run verify:docs
```

Validation ran:

```bash
npm run verify
cargo test --workspace --all-features -q
git diff --check
```

All commands passed. The local aggregate gate remains deterministic and does not
include browser automation, screenshots, `dx serve`, full Rust workspace tests,
or release-only gates.

## M61 Release Verification Alignment Plan

The project now has a stable local npm aggregate command, but release guidance
must keep stronger gates explicit. M61 aligns the release documentation around
three tiers:

- `npm run verify` is the default deterministic local alias for preview smoke,
  example smoke, docs metadata, and Markdown drift checks.
- Rust workspace tests, source-copy fixture smoke, and feature checks remain
  explicit release gates because they cover compile-time and packaging surfaces
  that the npm aggregate command intentionally does not own.
- Playwright browser smoke and screenshots remain opt-in because they require
  browser installation or a local browser executable.

The release docs should make the aggregate alias useful without making it look
like a substitute for full pre-publish verification. The consistency check added
in this milestone should be read-only and should fail only when release docs no
longer mention the local aggregate gate, required Rust/source-copy/feature
gates, or the opt-in browser boundary.

Out of scope:

- installing browser binaries
- activating CI browser workflows
- adding new release automation
- changing component APIs, registry entries, or templates

## M61 Release Docs Consistency Gate

M61 adds a focused read-only release documentation check:

```bash
npm run verify:release-docs
```

The command verifies that `docs/release.md` still mentions:

- the local `npm run verify` aggregate alias
- required Rust workspace check and test commands
- source-copy fixture smoke
- feature checks
- opt-in mobile browser smoke and its release-gate boundary

This check is intentionally separate from `npm run verify`. The local aggregate
gate remains optimized for deterministic preview, example, docs metadata, and
Markdown drift checks, while release documentation consistency is a
release-hardening concern.

## M61 Final Result

M61 aligned release verification documentation around the local aggregate gate
and explicit release-only checks. The milestone added:

```bash
npm run verify:release-docs
```

The command reads `docs/release.md` and fails if release guidance stops
mentioning the local `npm run verify` alias, Rust workspace check/test gates,
source-copy fixture smoke, feature checks, or the opt-in browser smoke boundary.

Validation ran:

```bash
npm run verify
npm run verify:release-docs
cargo test --workspace --all-features -q
git diff --check
```

All commands passed. No generated release artifacts were committed.

## M62 Release Gate Aggregator Plan

M62 should add a single explicit npm alias for the existing local release gate
set:

```bash
npm run verify:release
```

The command should aggregate already-documented release checks rather than
introduce new release behavior. It should run:

- Rust workspace check and tests
- CLI registry test and `dxui list` smoke
- default deterministic local verification through `npm run verify`
- feature compilation checks
- generated source-copy fixture smoke
- release documentation consistency check

Command ordering should fail on broad deterministic checks before the more
expensive feature and generated-fixture gates where practical. Focused aliases
must remain available for isolation, and opt-in browser smoke should stay
outside the aggregate command because it requires browser installation or a
local browser executable.

Out of scope:

- installing Playwright browser binaries
- enabling screenshots by default
- activating CI workflows
- claiming native Mobile or native Desktop runtime automation

## M62 Release Aggregate Usage

M62 adds the release aggregate command:

```bash
npm run verify:release
```

The aggregate command keeps the release checklist executable while preserving
focused commands for debugging. It runs Rust workspace checks, CLI registry/list
smoke, `npm run verify`, feature checks, generated fixture smoke, and release
documentation consistency checks.

Browser-rendered smoke remains separate:

```bash
npm run verify:mobile-browser
```

That command still requires Playwright Chromium or an explicit browser
executable, so it is not part of the release aggregate.

## M62 Final Result

M62 added the release aggregate command:

```bash
npm run verify:release
```

The command runs:

- `cargo check --workspace --all-features`
- `cargo test --workspace --all-features`
- `cargo test -p dioxus-ui-cli --test registry`
- `cargo run -p dioxus-ui-cli -- list`
- `npm run verify`
- `scripts/feature-check.sh`
- `scripts/generated-fixture-smoke.sh`
- `npm run verify:release-docs`
- `npm run verify:package-scripts`

Validation ran:

```bash
npm run verify:release
git diff --check
```

All commands passed. Browser installation, screenshots, and native runtime
automation remain opt-in and outside the release aggregate.

## M63 Package Script Consistency Gate Plan

M63 should add a read-only package script consistency check for `package.json`.
The check should validate that the verification aliases remain wired to the
expected focused commands:

- docs metadata aliases and `npm run verify:docs`
- preview/example aliases and `npm run verify:smoke`
- default local `npm run verify`
- release documentation and package script consistency aliases
- release aggregate `npm run verify:release`
- opt-in mobile browser smoke alias

The consistency check should inspect command strings only. It should not execute
Cargo, Node, browser automation, screenshots, generated fixture smoke, or release
commands. The goal is to catch accidental alias drift before a contributor runs
an expensive gate.

If the check remains deterministic and read-only, it can be included in the
release aggregate command. Focused commands should remain available for failure
isolation.

## M63 Package Script Consistency Gate Usage

M63 adds the package script consistency command:

```bash
npm run verify:package-scripts
```

The command parses `package.json` and verifies that required focused and
aggregate verification aliases are present. It also checks that aggregate
commands still reference their expected focused checks, including the release
aggregate command.

This is a wiring check only. It does not run Cargo, browser automation,
generated fixture smoke, or any release command recursively. Because it is
read-only and deterministic, it is included at the end of `npm run
verify:release`.

## M63 Final Result

M63 added the package script consistency command:

```bash
npm run verify:package-scripts
```

The command validates required `package.json` verification aliases and aggregate
command relationships without executing the commands. It is included at the end
of:

```bash
npm run verify:release
```

Validation ran:

```bash
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
npm run verify:release
git diff --check
```

All commands passed. No generated artifacts were committed.

## M64 CI Browser Docs Alignment Plan

M64 should align the CI browser smoke documentation with the current verification
aliases. CI browser smoke remains opt-in and should not become an active
workflow in this milestone.

Expected documentation shape:

- local deterministic checks before browser smoke should use `npm run verify`
  plus Rust workspace tests where appropriate.
- release-ready checks should point to `npm run verify:release`.
- browser smoke should remain `npm run verify:mobile-browser` with explicit
  Playwright Chromium or external browser setup.
- the workflow template should stay documentation-only, manually triggered, and
  non-blocking.
- no `.github/workflows` file should be added.

The consistency check should read `docs/ci-browser-smoke.md` and
`docs/ci-browser-workflow-template.md` only. It should verify command references
and opt-in language without running browser automation or creating workflow
files.

## M64 CI Browser Docs Alignment Usage

M64 adds the CI browser documentation check:

```bash
npm run verify:ci-docs
```

The command verifies that the opt-in CI browser smoke guide and workflow
template reference the current local gates:

- `npm run verify`
- `cargo test --workspace --all-features -q`
- `npm run verify:release`
- `npm run verify:mobile-browser`

It also checks that the workflow template remains manual and non-blocking and
that `.github/workflows/browser-smoke.yml` is not committed.

Because the check is read-only and deterministic, it is included at the end of:

```bash
npm run verify:release
```

## M64 Final Result

M64 aligned CI browser smoke documentation with the current verification aliases
and added:

```bash
npm run verify:ci-docs
```

The command checks `docs/ci-browser-smoke.md` and
`docs/ci-browser-workflow-template.md` for the expected local gate references,
manual/non-blocking workflow language, and absence of
`.github/workflows/browser-smoke.yml`.

Validation ran:

```bash
npm run verify:ci-docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
npm run verify:release
git diff --check
test ! -e .github/workflows/browser-smoke.yml
```

All commands passed. No workflow files or generated artifacts were committed.

## M65 CI Plan Documentation Gate Plan

M65 should align the CI Plan documentation with the verification aliases added
through M59-M64. The CI Plan remains documentation only; this milestone should
not create or activate workflows.

Expected CI Plan shape:

- default pull request CI should run Rust workspace checks, CLI registry checks,
  source-copy fixture smoke, and `npm run verify`.
- scheduled or release CI should run `npm run verify:release`.
- feature checks remain part of the release aggregate and can still be called
  out as the expensive gate.
- browser smoke remains opt-in through `npm run verify:mobile-browser`.
- CI docs checks should verify the plan text but not execute CI commands.

The read-only check should inspect `docs/quality-gates.md` and ensure the CI
Plan mentions the current local, release, and browser verification boundaries.
It should also keep workflow creation out of scope.

## M65 CI Plan Documentation Gate Usage

M65 adds the CI Plan documentation check:

```bash
npm run verify:ci-plan
```

The command checks the `docs/quality-gates.md` CI Plan section for current PR,
scheduled/release, and opt-in browser verification boundaries:

- Rust workspace checks
- CLI registry checks
- source-copy fixture smoke
- `npm run verify`
- `npm run verify:release`
- `scripts/feature-check.sh`
- `docs/ci-browser-smoke.md`

The check is documentation-only. It does not execute CI commands, create
workflow files, install browsers, or promote browser smoke to a required gate.
Because it is read-only and deterministic, it is included at the end of:

```bash
npm run verify:release
```

## M65 Final Result

M65 aligned the CI Plan section in `docs/quality-gates.md` with the current
verification aliases and added:

```bash
npm run verify:ci-plan
```

The command checks that the CI Plan documents default pull request gates,
scheduled/release gates, feature-check placement, and the opt-in browser smoke
boundary without creating workflow files or running CI commands.

Validation ran:

```bash
npm run verify:ci-plan
npm run verify:package-scripts
npm run verify:ci-docs
npm run verify:release-docs
npm run verify:docs
npm run verify:release
git diff --check
test ! -e .github/workflows/browser-smoke.yml
```

All commands passed. No workflow files or generated artifacts were committed.

## M66 Documentation Index Consistency Gate Plan

M66 should keep project entry-point documentation discoverable as verification
commands and planning docs accumulate. This milestone is an index consistency
gate only; it should not build the docs site, crawl arbitrary links, generate
navigation files, or create CI workflows.

The required top-level `README.md` index should expose:

- architecture and roadmap docs
- component API, catalog, and docs-site planning docs
- quality gates and release docs
- CI browser smoke docs and workflow template docs
- TODO plan and current RFC entry points

The required `docs/README.md` index should expose:

- core design, roadmap, workspace, component API, release, and quality docs
- component catalog and docs-site planning docs
- runtime verification planning docs
- all currently active RFC documents, including CI browser workflow activation
- the root TODO plan

The read-only check should inspect only `README.md` and `docs/README.md`.
It should fail on missing required link tokens, but it should not validate
Markdown rendering, external URLs, generated route manifests, or filesystem-wide
link integrity. Those concerns belong to separate docs-site or link-check
milestones.

## M66 Documentation Index Consistency Gate Usage

M66 adds the documentation index check:

```bash
npm run verify:docs-index
```

The command checks the root README and docs README for required links to
architecture, roadmap, component API, release, quality, CI, docs-site, RFC, and
TODO entry points. It is read-only and does not build the docs site, crawl
external URLs, create navigation files, or create workflow files.

Because the check is deterministic and cheap, it is included in:

```bash
npm run verify:docs
```

## M66 Final Result

M66 added the documentation index consistency check:

```bash
npm run verify:docs-index
```

The command verifies required root README and docs README links for quality,
release, CI, docs-site, RFC, and TODO entry points. It is included in
`npm run verify:docs`, and package script consistency checks now require the
alias and aggregate wiring.

Validation ran:

```bash
npm run verify:docs-index
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:release
git diff --check
test ! -e .github/workflows/browser-smoke.yml
```

All commands passed. No workflow files or generated artifacts were committed.

## M67 Local Markdown Link Target Gate Plan

M67 should verify that local relative Markdown links keep pointing at real
repository files as documentation grows. This complements the M66 index check:
M66 checks that required entry-point links are present, while M67 checks that
local links in Markdown documents do not drift to missing files.

The check should scan tracked Markdown documentation sources, including:

- root README and TODO plan files
- `docs/**/*.md`
- RFC documents
- component documentation pages

The check should include local relative links to Markdown and repository assets.
It should strip query strings, fragments, and angle-bracket wrappers before
resolving the target relative to the source file.

Out of scope for this milestone:

- external URL validation
- fragment or heading anchor validation
- generated docs runtime route validation
- crawling HTML, Rust docs, registry JSON, or generated fixture output
- network access

Ignored build output, Cargo target directories, node modules, git metadata, and
temporary generated fixtures should remain outside the scan.

## M67 Local Markdown Link Target Gate Usage

M67 adds the local Markdown link target check:

```bash
npm run verify:docs-links
```

The command scans tracked Markdown files and verifies that local relative file
targets exist. It strips query strings and fragments before resolving targets.
It does not validate external URLs, heading fragments, generated docs routes,
or runtime navigation.

Because the check is deterministic and read-only, it is included in:

```bash
npm run verify:docs
```

## M90 Gitignore Metadata Gate Usage

M90 adds a focused gitignore metadata command:

```bash
npm run verify:gitignore
```

The command checks `.gitignore` patterns for generated directories, browser
automation state, and preview screenshot artifacts. It also verifies that the
repository hygiene policy still rejects tracked generated artifacts matching
those local artifact classes.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not delete files, inspect ignored artifact
contents, validate global Git excludes, or replace repository hygiene tracking
checks.

## M91 Preview State Inventory Metadata Gate Usage

M91 adds a focused preview state metadata command:

```bash
npm run verify:preview-state-metadata
```

The command checks that the shared `examples/preview-states` inventory,
Web/Desktop preview binaries, structural preview verifiers, Tailwind CSS v4
preview source inputs, and release wiring stay aligned. It is intended to keep
future docs runtime and screenshot work from duplicating preview state metadata.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not launch browsers, run `dx serve`, compile
Tailwind output, inspect screenshots, or assert visual parity.

## M92 Mobile Browser Smoke Metadata Gate Usage

M92 adds a focused mobile browser smoke metadata command:

```bash
npm run verify:mobile-browser-metadata
```

The command checks the opt-in mobile browser smoke script, package alias,
localhost target, `390x844` viewport, selector contract, screenshot artifact
pattern, browser environment variables, CI browser guide, and workflow template
references. It keeps browser smoke documentation aligned while the actual
browser run remains opt-in.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not launch Playwright, start `dx serve`, install
browsers, write screenshots, or validate rendered output.

## M93 CI Browser Workflow Template Metadata Gate Usage

M93 adds a focused CI workflow template metadata command:

```bash
npm run verify:ci-workflow-template
```

The command checks the documented browser smoke workflow template, RFC 0009
activation policy, CI browser guide, package alias, screenshot artifact
pattern, and absence of `.github/workflows/browser-smoke.yml`.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not create workflow files, run GitHub Actions,
install browsers, upload artifacts, or change browser smoke rollout policy.

## M93 Final Result

M93 added `npm run verify:ci-workflow-template` and wired it into
`npm run verify:release`. The gate validates the documented browser smoke
workflow template, RFC 0009 activation policy, CI browser guide, package alias,
screenshot artifact pattern, and absence of `.github/workflows/browser-smoke.yml`.

Validation completed:

```bash
npm run verify:ci-workflow-template
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The browser workflow remains documentation-only and
non-blocking until maintainers explicitly copy and activate it.

## M94 Browser Artifact Policy Metadata Gate Usage

M94 adds a focused browser artifact policy metadata command:

```bash
npm run verify:browser-artifact-policy
```

The command checks CI browser artifact guidance, workflow template upload
fields, RFC 0009 artifact policy, `.gitignore` screenshot patterns, repository
hygiene boundaries, and release wiring. It keeps screenshot-only upload policy
aligned while browser workflow execution remains opt-in.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not launch browser automation, upload artifacts,
delete local files, enforce remote retention, or validate screenshot pixels.

## M94 Final Result

M94 added `npm run verify:browser-artifact-policy` and wired it into
`npm run verify:release`. The gate validates CI browser artifact guidance,
workflow template upload fields, RFC 0009 artifact policy, `.gitignore`
screenshot patterns, repository hygiene boundaries, and release wiring.

Validation completed:

```bash
npm run verify:browser-artifact-policy
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The full release run still reports the existing Rust
future-incompatibility warning for `block v0.1.6`.

## M95 Release Warning Inventory Metadata Gate Usage

M95 adds a focused release warning inventory metadata command:

```bash
npm run verify:release-warning-inventory
```

The command checks the documented `block` `0.1.6` Rust
future-incompatibility warning inventory, Cargo lock evidence, release docs,
quality gate notes, docs-site notes, and release wiring.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not run Cargo, parse live compiler output, execute
`cargo report`, upgrade dependencies, or suppress warnings.

The inventory deliberately records the current warning instead of suppressing it
or upgrading dependencies as a side effect of verification.

## M95 Final Result

M95 added `npm run verify:release-warning-inventory` and wired it into
`npm run verify:release`. The gate validates the known `block` `0.1.6` Rust
future-incompatibility warning inventory, Cargo lock evidence, release docs,
quality gate notes, docs-site notes, and release wiring.

Validation completed:

```bash
npm run verify:release-warning-inventory
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The full release run still reports the documented
`block` `0.1.6` future-incompatibility warning.

## M96 Cargo Publish Metadata Gate Usage

M96 adds a focused Cargo publish metadata command:

```bash
npm run verify:cargo-publish-metadata
```

The command checks planned published crate descriptions, shared
README/keywords/categories metadata, example `publish = false` boundaries, and
release wiring.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not run `cargo publish`, run `cargo package`,
contact crates.io, replace repository URLs, or create package archives.

## M96 Final Result

M96 added `npm run verify:cargo-publish-metadata` and wired it into
`npm run verify:release`. The gate validates planned published crate
descriptions, shared README/keywords/categories metadata, example `publish =
false` boundaries, and release wiring.

Validation completed:

```bash
npm run verify:cargo-publish-metadata
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not package or publish crates, and the
placeholder repository URL still requires a later publish-readiness review.

## M97 Publish Readiness Blocker Metadata Gate Usage

M97 adds a focused publish readiness blocker metadata command:

```bash
npm run verify:publish-readiness-blockers
```

The command checks the documented placeholder repository URL, pre-1.0 API
stability, release notes readiness, root license file readiness, crates.io
review, and workspace dependency publish readiness blockers.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not replace repository URLs, check registries, run
`cargo package`, run `cargo publish`, stabilize APIs, generate changelogs, or
change embedded CLI template delivery.

## M97 Final Result

M97 added `npm run verify:publish-readiness-blockers` and wired it into
`npm run verify:release`. The gate validates the known placeholder repository
URL, pre-1.0 API stability, release notes readiness, root license file
readiness, crates.io review, and workspace dependency publish readiness
blockers.

Validation completed:

```bash
npm run verify:publish-readiness-blockers
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The blockers remain intentionally unresolved until a
maintainer performs a dedicated publish-readiness review.

## M99 Release Notes Readiness Metadata Gate Usage

M99 adds a focused release notes readiness metadata command:

```bash
npm run verify:release-notes-readiness
```

The command checks that project-owned changelog structure exists and its
Unreleased section records first publish included scope, excluded scope, and
known warnings.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not generate release notes, run git-cliff, derive
changes from Git history, create tags, or publish releases.

## M99 Final Result

M99 added `npm run verify:release-notes-readiness` and wired it into
`npm run verify:release`. The gate validates project-owned changelog structure
and, since M133, the first publish release note scope recorded in
`CHANGELOG.md`.

Validation completed:

```bash
npm run verify:release-notes-readiness
npm run verify:publish-readiness-blockers
npm run verify:changelog
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:readme
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not generate release notes, run git-cliff,
derive changes from Git history, create tags, publish releases, or decide
release contents.

## M100 License Readiness Metadata Gate Usage

M100 adds a focused license readiness metadata command:

```bash
npm run verify:license-readiness
```

The command checks workspace MIT license metadata and committed root `LICENSE`
file.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not choose different license terms, generate
replacement license text, change copyright holders, run `cargo package`, run
`cargo publish`, or contact crates.io.

## M100 Final Result

M100 added `npm run verify:license-readiness` and wired it into
`npm run verify:release`. The gate validates workspace MIT license metadata and
the committed root `LICENSE` file.

Validation completed:

```bash
npm run verify:license-readiness
npm run verify:publish-readiness-blockers
npm run verify:cargo-publish-metadata
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:readme
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not choose different license terms,
generate replacement license text, change copyright holders, run
`cargo package`, run `cargo publish`, or contact crates.io.

## M130 License Decision Preparation

M130 adds the decision preparation pass for first-publish root license files:

- docs/license-decision-preparation-plan.md
- docs/license-decision-record-template.md
- docs/license-local-follow-up-map.md

The plan records reviewed MIT license text, copyright holder text, license
expression confirmation, and file commit approval. The local follow-up map
separates workspace license expression follow-up from root license file
follow-up without generating replacement license text by itself.

Validation remains read-only:

```bash
npm run verify:license-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:cargo-publish-metadata
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
```

The preparation pass does not choose license terms, generate license text,
commit root license files, change copyright holders, change workspace license
metadata, contact crates.io, run `cargo package`, run `cargo publish`, create
package archives, generate release notes, create tags, or activate CI
workflows.

## M101 Repository Identity Readiness Metadata Gate Usage

M101 adds a focused repository identity readiness metadata command:

```bash
npm run verify:repository-identity-readiness
```

The command checks that the approved repository URL remains in workspace
metadata.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not choose a different repository owner, change repository
metadata, check remote repository existence, check crates.io availability, run
`cargo package`, or run `cargo publish`.

## M101 Final Result

M101 added `npm run verify:repository-identity-readiness` and wired it into
`npm run verify:release`. The gate validates that the placeholder repository
URL remains in workspace metadata and is still tracked as a publish-readiness
blocker.

Validation completed:

```bash
npm run verify:repository-identity-readiness
npm run verify:publish-readiness-blockers
npm run verify:cargo-publish-metadata
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:readme
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not choose a repository owner, replace
repository metadata, check remote repository existence, check crates.io
availability, run `cargo package`, or run `cargo publish`.

## M129 Repository Identity Decision Preparation

M129 adds the decision preparation pass for first-publish repository identity:

- docs/repository-identity-decision-preparation-plan.md
- docs/repository-identity-decision-record-template.md
- docs/repository-identity-local-follow-up-map.md

The plan keeps the placeholder repository URL unresolved until maintainers
record the canonical repository owner and URL. The decision template captures
owner, URL, remote availability, metadata approval, and rollback evidence. The
local follow-up map separates placeholder metadata follow-up from Cargo
workspace metadata follow-up without applying URL changes by itself.

Validation remains read-only:

```bash
npm run verify:repository-identity-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:cargo-publish-metadata
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
```

The preparation pass does not choose repository ownership, replace repository
metadata, check remote repository existence, contact crates.io, run
`cargo package`, run `cargo publish`, create package archives, change versions,
generate release notes, create tags, or activate CI workflows.

## M102 API Stability Readiness Metadata Gate Usage

M102 adds a focused API stability readiness metadata command:

```bash
npm run verify:api-stability-readiness
```

The command checks workspace version `0.1.0` and the accepted `0.1.x`
first-publish API policy.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not stabilize component APIs, change crate
versions, change the pre-`1.0` breaking-change policy, generate migration
guides, run `cargo package`, or run `cargo publish`.

## M102 Final Result

M102 added `npm run verify:api-stability-readiness` and wired it into
`npm run verify:release`. The gate validates workspace version `0.1.0` and the
accepted `0.1.x` first-publish API policy recorded in M133.

Validation completed:

```bash
npm run verify:api-stability-readiness
npm run verify:publish-readiness-blockers
npm run verify:cargo-publish-metadata
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:readme
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not stabilize component APIs, change crate
versions, change the pre-`1.0` breaking-change policy, generate migration
guides, run `cargo package`, or run `cargo publish`.

## M103 CLI Template Packaging Readiness Metadata Gate Usage

M103 adds a focused CLI template packaging readiness metadata command:

```bash
npm run verify:cli-template-packaging-readiness
```

The command checks that CLI registry and template assets are embedded at
compile time.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not run `cargo package`, run `cargo publish`,
install the CLI, contact crates.io, create package archives, or change embedded
template contents.

## M103 Final Result

M103 added `npm run verify:cli-template-packaging-readiness` and wired it into
`npm run verify:release`. The gate validates that CLI registry and template
assets are embedded at compile time.

Validation completed:

```bash
npm run verify:cli-template-packaging-readiness
npm run verify:publish-readiness-blockers
npm run verify:cargo-publish-metadata
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:readme
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not run `cargo package`, run
`cargo publish`, install the CLI, contact crates.io, create package archives,
or change embedded template contents.

## M104 Registry Availability Readiness Metadata Gate Usage

M104 adds a focused registry availability readiness metadata command:

```bash
npm run verify:registry-availability-readiness
```

The command checks that planned publishable crate names remain documented and
that the deferred crates.io name and ownership review blocker remains
documented. Since M133 it also checks the per-crate evidence table and that
deferral blocks crates.io publishing but not local release readiness.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not contact crates.io, check crate name
availability, check ownership, inspect credentials, run `cargo package`, run
`cargo publish`, or create package archives.

## M104 Final Result

M104 added `npm run verify:registry-availability-readiness` and wired it into
`npm run verify:release`. The gate validates planned publishable crate names
and preserves the unresolved crates.io name and ownership review blocker.

Validation completed:

```bash
npm run verify:registry-availability-readiness
npm run verify:publish-readiness-blockers
npm run verify:cargo-publish-metadata
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:readme
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not contact crates.io, check crate name
availability, check ownership, inspect credentials, run `cargo package`, run
`cargo publish`, or create package archives.

## M105 Publish Readiness Coverage Metadata Gate Usage

M105 adds a focused publish readiness coverage metadata command:

```bash
npm run verify:publish-readiness-coverage
```

The command checks that every current publish blocker has a focused readiness
gate, metadata doc, README mention, package script, and release wiring.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not resolve blockers, replace repository URLs,
stabilize APIs, generate release notes, generate license text, embed or package
CLI templates, contact registries, inspect credentials, run `cargo package`,
run `cargo publish`, or create package archives.

## M105 Final Result

M105 added `npm run verify:publish-readiness-coverage` and wired it into
`npm run verify:release`. The gate validates that every current publish blocker
has a focused readiness gate, metadata doc, README mention, package script, and
release wiring.

Validation completed:

```bash
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-blockers
npm run verify:release-notes-readiness
npm run verify:license-readiness
npm run verify:repository-identity-readiness
npm run verify:api-stability-readiness
npm run verify:cli-template-packaging-readiness
npm run verify:registry-availability-readiness
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:readme
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not resolve blockers, replace repository
URLs, stabilize APIs, generate release notes, generate license text, change
embedded CLI template delivery, contact registries, inspect credentials, run
`cargo package`, run `cargo publish`, or create package archives.

## M106 Publish Readiness Resolution Runbook Metadata Gate Usage

M106 adds a focused publish readiness runbook metadata command:

```bash
npm run verify:publish-readiness-runbook
```

The command checks manual resolution evidence for every current publish blocker
and verifies required follow-up update targets remain documented.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not resolve blockers, replace repository URLs,
stabilize APIs, generate release notes, generate license text, change embedded
CLI template delivery, contact registries, inspect credentials, run
`cargo package`, run `cargo publish`, or create package archives.

## M106 Final Result

M106 added `npm run verify:publish-readiness-runbook` and wired it into
`npm run verify:release`. The gate validates manual resolution evidence and
follow-up update targets for every current publish blocker.

Validation completed:

```bash
npm run verify:publish-readiness-runbook
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-blockers
npm run verify:release-notes-readiness
npm run verify:license-readiness
npm run verify:repository-identity-readiness
npm run verify:api-stability-readiness
npm run verify:cli-template-packaging-readiness
npm run verify:registry-availability-readiness
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not resolve blockers, replace repository
URLs, stabilize APIs, generate release notes, generate license text, change
embedded CLI template delivery, contact registries, inspect credentials, run
`cargo package`, run `cargo publish`, or create package archives.

## M122 Publish Readiness Decision Matrix Usage

M122 adds a maintainer-facing decision matrix:

```text
docs/publish-readiness-decision-matrix.md
```

The matrix records required decisions, evidence, local follow-up files, and
safe validation commands for each current publish readiness blocker. It is a
handoff aid and does not authorize publishing.

Use it with:

- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [Release Candidate Handoff Checklist](release-candidate-handoff-checklist.md)
- [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)

It remains repository-safe. It does not replace repository URLs, generate
license text, decide API stability, generate release notes, embed or package
CLI templates, contact crates.io, inspect credentials, run `cargo package`, run
`cargo publish`, create package archives, create Git tags, or publish
artifacts.

## M122 First Publish Handoff Template Usage

M122 adds a copyable maintainer handoff template:

```text
docs/first-publish-maintainer-handoff-template.md
```

The template records release-candidate identity, repository identity approval,
license file approval, API stability policy, release notes scope, CLI template
packaging strategy, crates.io ownership, workspace dependency publish
readiness, and final verification commands.

It remains a planning artifact. It does not run `cargo package`, run
`cargo publish`, contact crates.io, inspect credentials, create package
archives, create Git tags, publish GitHub releases, or authorize publishing.

## M123 Public API Surface Inventory Usage

M123 adds a public API surface inventory:

```text
docs/public-api-surface-inventory.md
```

The inventory records current styled component modules, component features,
source-copy templates, registry entries, primitive module groups, and core
exports that need maintainer review before the API stability blocker can be
resolved.

Use it with:

- [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
- [API Stability Surface Audit Plan](api-stability-surface-audit-plan.md)
- [API Stability Review Checklist](api-stability-review-checklist.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)

It remains descriptive. It does not freeze APIs, rename components, rewrite
props, change feature names, change crate versions, generate migration guides,
run `cargo package`, run `cargo publish`, contact crates.io, create tags, or
publish artifacts.

## M123 API Stability Review Checklist Usage

M123 adds a maintainer-facing API stability review checklist:

```text
docs/api-stability-review-checklist.md
```

The checklist covers component naming, props and variants, class helpers,
primitive helpers, source-copy compatibility, documentation examples, decision
outcomes, and safe validation commands.

It remains separate from approval. It does not approve API stability, change
versions, rename APIs, rewrite props, rewrite templates, generate migration
guides, run `cargo package`, run `cargo publish`, contact crates.io, create
tags, or publish artifacts.

## M107 Publish Order Metadata Gate Usage

M107 adds a focused publish order metadata command:

```bash
npm run verify:publish-order
```

The command checks the planned crate publish order across release docs, Cargo
publish metadata, registry availability metadata, and the publish readiness
runbook.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not create package archives, run `cargo package`,
run `cargo publish`, contact crates.io, check registry ownership, inspect
credentials, change dependency versions, or authorize a release.

## M107 Final Result

M107 added `npm run verify:publish-order` and wired it into
`npm run verify:release`. The gate validates the planned crate publish order
across release docs, Cargo publish metadata, registry availability metadata,
and the publish readiness runbook.

Validation completed:

```bash
npm run verify:publish-order
npm run verify:publish-readiness-runbook
npm run verify:registry-availability-readiness
npm run verify:cargo-publish-metadata
npm run verify:package-scripts
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not create package archives, run
`cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, change dependency versions, or authorize a
release.

## M108 Workspace Dependency Publish Readiness Metadata Gate Usage

M108 adds a focused workspace dependency publish readiness metadata command:

```bash
npm run verify:workspace-dependency-publish-readiness
```

The command checks that internal workspace dependencies declare versions
matching the workspace version alongside local paths, and that blocker docs,
coverage metadata, the runbook, Cargo publish metadata, publish order metadata,
release docs, quality gates, and docs-site notes describe that shape.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not change dependency versions, run
`cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, create package archives, or authorize a
release.

## M108 Final Result

M108 added `npm run verify:workspace-dependency-publish-readiness` and wired it
into `npm run verify:release`. Since M133 the gate validates that internal
workspace dependencies declare versions matching the workspace version
alongside local paths, consistently across blocker docs, coverage metadata, the
runbook, Cargo publish metadata, publish order metadata, release docs, quality
gates, README, package scripts, and docs-site notes.

Validation completed:

```bash
npm run verify:workspace-dependency-publish-readiness
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:publish-order
npm run verify:cargo-publish-metadata
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not change dependency versions, run
`cargo package`, run `cargo publish`, contact crates.io, check registry
ownership, inspect credentials, create package archives, or authorize a
release.

## M109 Rendered Component Verification Usage

M109 adds a focused rendered component coverage metadata command:

```bash
npm run verify:rendered-component-coverage
```

The command checks stable rendered preview target metadata for public
components against the docs catalog. It validates component slug, label,
category, panel, test id, coverage level, and notes, then confirms the shared
preview state source contains the matching `data-component-preview` targets
without running a renderer.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not start a server, launch a browser, write
screenshots, update generated docs, change component APIs, edit templates, or
claim visual parity.

## M110 Browser DOM Component Verification Usage

M110 adds an opt-in browser DOM command:

```bash
npm run verify:rendered-component-dom
```

The command starts the Web preview, opens it in Playwright Chromium or an
explicit `DIOXUS_UI_BROWSER_EXECUTABLE`, and checks every public
`data-component-preview` target from the rendered coverage manifest. It
validates DOM existence, visibility, text content, component metadata
attributes, and non-empty layout boxes.

It is not part of `npm run verify` or `npm run verify:release`. It does not
write screenshots or traces, assert runtime interactions, update generated
docs, change component APIs, edit templates, or claim visual parity.

## M111 Runtime Interaction Verification Usage

M111 adds an opt-in browser interaction command:

```bash
npm run verify:runtime-interactions
```

The command starts the Web preview, opens it in Playwright Chromium or an
explicit `DIOXUS_UI_BROWSER_EXECUTABLE`, and checks focused
`data-interaction-*` fixture targets for disclosure, overlay, selection,
keyboard-visible state, and scroll-status behavior.

It is not part of `npm run verify` or `npm run verify:release`. It does not
write screenshots or traces, certify full accessibility, verify native Desktop
or Mobile behavior, update generated docs, change component APIs, edit
templates, or claim visual parity.

## M112 Web Preview Screenshot Smoke Usage

M112 adds an opt-in Web screenshot smoke command:

```bash
npm run verify:web-screenshot-smoke
```

The command starts the Web preview, checks representative screenshot targets at
`1280x900` and `390x844`, and validates required panels, chart SVG output,
fallback rows, overlay content, interaction fixtures, and viewport dimensions.

It writes no screenshots by default. When `DIOXUS_UI_WEB_SCREENSHOT=1` is set,
it captures ignored `dioxus-ui-web-preview-*.png` files and validates PNG
signature, byte size, and dimensions before reporting success.

It is not part of `npm run verify` or `npm run verify:release`. It does not
compare pixels, certify shadcn/ui visual parity, activate CI workflows, verify
Desktop WebView or native Mobile screenshots, update generated docs, change
component APIs, or edit templates.

## M113 Browser Smoke Aggregate Usage

M113 adds an opt-in local browser smoke aggregate:

```bash
npm run verify:browser-local
```

The command runs mobile browser smoke, rendered component DOM verification, Web
screenshot smoke, and runtime interaction verification in sequence. It exists
because those browser commands each start `dx serve`; parallel execution can
produce duplicate preview roots or duplicate component targets.

It accepts the same browser and screenshot environment variables as the
individual commands, including `DIOXUS_UI_BROWSER_EXECUTABLE`,
`DIOXUS_UI_MOBILE_BROWSER_SCREENSHOT`, and `DIOXUS_UI_WEB_SCREENSHOT`.
Screenshots stay disabled unless the screenshot variables are explicitly set.

It is not part of `npm run verify` or `npm run verify:release`. It does not
activate CI workflows, enable screenshots by default, compare pixels, certify
visual parity, verify Desktop WebView or native Mobile screenshots, update
generated docs, change component APIs, or edit templates.

## M114 Release Screenshot Review Checklist Usage

M114 adds a manual release-candidate screenshot review checklist:

```text
docs/components/release-screenshot-review-checklist.md
```

The checklist uses the existing opt-in browser commands to capture Web desktop,
Web mobile, and mobile browser screenshots, records PNG metadata from command
output, guides human review across the preview panels, and ends with repository
hygiene checks.

It is not an automated visual regression system. It does not compare pixels,
create visual baselines, activate CI workflows, enable screenshots by default,
promote browser smoke into release gates, verify Desktop WebView or native
Mobile screenshots, update generated docs, change component APIs, or edit
templates.

## M115 Screenshot Artifact Retention Usage

M115 adds the screenshot retention decision for release-candidate review:

```text
docs/components/screenshot-artifact-retention.md
```

The policy keeps screenshots local-first. Screenshot PNG files are temporary
review aids by default; maintainers should copy command output and PNG metadata
into review notes, avoid committing screenshots, and delete them before final
handoff unless intentionally keeping ignored local files.

The policy is referenced by release docs, quality gates, CI browser smoke docs,
browser artifact policy metadata, and the release screenshot review checklist.
It does not upload artifacts, activate CI workflows, add GitHub release
attachments, enable screenshots by default, create visual baselines, update
generated docs, change component APIs, or edit templates.

## M116 Release Screenshot Review Notes Template Usage

M116 adds a copyable release-candidate screenshot review notes template:

```text
docs/components/release-screenshot-review-notes-template.md
```

The template records release candidate metadata, browser environment, commands,
Web desktop screenshot metadata, Web mobile screenshot metadata, mobile browser
screenshot metadata, observed issues, decision, retention outcome, cleanup
evidence, and follow-up tasks.

It is a Markdown review aid, not an artifact store. It does not commit
screenshots, upload artifacts, activate CI workflows, generate docs output,
create visual baselines, update component APIs, or edit templates.

## M117 Release Candidate Browser Review Runbook Usage

M117 adds a manual release-candidate browser review runbook:

```text
docs/components/release-candidate-browser-review-runbook.md
```

The runbook orders deterministic checks, the serial opt-in browser aggregate,
optional screenshot capture, manual screenshot review, review notes, retention
cleanup, and failure triage.

It remains local-first and repository-safe. It does not promote browser smoke
into default or release gates, enable screenshots by default, upload artifacts,
activate CI workflows, generate docs output, change component APIs, or edit
source-copy templates.

## M118 Release Candidate Handoff Checklist Usage

M118 adds the final release-candidate maintainer handoff checklist:

```text
docs/release-candidate-handoff-checklist.md
```

The checklist records release gate evidence, optional browser review evidence,
publish readiness blocker status, release warning inventory status, artifact
hygiene, unresolved follow-ups, and final repository state.

It is a manual evidence record. It does not publish packages, create Git tags,
create release artifacts, commit screenshots, upload artifacts, activate CI
workflows, generate docs output, change component APIs, or rewrite templates.

## M119 Release Candidate Handoff Metadata Gate Usage

M119 adds a focused release candidate handoff metadata command:

```bash
npm run verify:release-candidate-handoff
```

The command checks that the final handoff checklist keeps required sections,
release gate evidence, optional browser review evidence, publish readiness
blockers, warning inventory, artifact hygiene boundaries, and discoverability
links aligned.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not run release gates, launch browser automation,
capture screenshots, create artifacts, create Git tags, publish packages,
activate CI workflows, generate docs output, change component APIs, or rewrite
templates.

## M120 Release Gate Failure Triage Runbook Usage

M120 adds a manual triage runbook for release aggregate failures:

```text
docs/release-gate-failure-triage-runbook.md
```

The runbook groups `npm run verify:release` failures by Rust workspace checks,
CLI smoke, metadata gates, docs gates, generated fixture smoke, feature checks,
browser artifact policy, release warning inventory, handoff metadata, and
repository hygiene. It tells maintainers which focused command to rerun and
what evidence to copy into handoff notes.

It is not an automatic repair system. It does not delete files, launch
browsers, capture screenshots, activate workflows, publish packages, create
Git tags, create release artifacts, generate docs output, change component
APIs, or rewrite templates.

## M98 Changelog Metadata Gate Usage

M98 adds a focused changelog metadata command:

```bash
npm run verify:changelog
```

The command checks project-owned changelog structure, Unreleased section, Keep
a Changelog and Conventional Commits references, and stale template-link bans.

The check is included in:

```bash
npm run verify:release
```

It remains read-only. It does not generate release notes, run git-cliff, derive
changes from Git history, create tags, publish releases, or rewrite commit
history.

## M98 Final Result

M98 added `npm run verify:changelog` and wired it into
`npm run verify:release`. The gate validates project-owned changelog structure,
the Unreleased section, Keep a Changelog and Conventional Commits references,
and stale template-link bans.

Validation completed:

```bash
npm run verify:changelog
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The check does not generate release notes, run git-cliff,
derive changes from Git history, create tags, publish releases, or rewrite
commit history.

## M92 Final Result

M92 added `npm run verify:mobile-browser-metadata` and wired it into
`npm run verify:release`. The gate validates the opt-in mobile browser smoke
script, package alias, localhost target, `390x844` viewport, selector contract,
screenshot artifact pattern, browser environment variables, CI browser guide,
and workflow template references without launching browser automation.

Validation completed:

```bash
npm run verify:mobile-browser-metadata
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The actual browser smoke remains opt-in through
`npm run verify:mobile-browser`; M92 only validates the committed metadata and
documentation contract for that opt-in workflow.

## M91 Final Result

M91 added `npm run verify:preview-state-metadata` and wired it into
`npm run verify:release`. The gate validates shared preview state inventory
labels, rendered preview panels, Web/Desktop preview binaries, structural
preview verifier state lists, Tailwind CSS v4 preview source inputs, and release
wiring without launching browser automation.

Validation completed:

```bash
npm run verify:preview-state-metadata
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now includes the preview state
metadata gate before examples metadata, CSS input metadata, registry metadata,
Tailwind static token checks, preview structural gates, example smoke, docs
metadata, feature checks, generated fixture smoke, release documentation checks,
package metadata checks, CI documentation checks, and repository hygiene.

## M90 Final Result

M90 added `npm run verify:gitignore` and wired it into
`npm run verify:release`. The gate validates `.gitignore` patterns for
generated directories, browser automation state, and preview screenshot
artifacts, and checks that repository hygiene policy still rejects tracked
generated artifacts.

Validation completed:

```bash
npm run verify:gitignore
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, pre-commit metadata, script metadata, gitignore metadata,
examples metadata, CSS input metadata, registry metadata, Tailwind static
tokens, preview structural gates, example smoke, docs metadata including RFC
and README verification, feature checks, generated fixture smoke, release
documentation checks, package script checks, package lock checks, CI
documentation checks, CI plan checks, repository hygiene, and ignored artifact
policy.

## M75 Package Script Target Gate Plan

M75 should extend package script verification so local script file references in
`package.json` cannot drift silently. The check should catch aliases that point
to removed or renamed files before release verification reaches that command.

The check should inspect every npm script command and validate:

- `node scripts/*.mjs` targets exist
- direct `scripts/*` executable targets exist
- aggregate commands such as `verify:release` are scanned segment by segment
- non-local commands such as `cargo`, `npm run`, and environment variables are ignored

The check should remain deterministic and read-only. It should not execute npm
scripts, interpret arbitrary shell syntax, validate external commands, check
file executable bits, or replace focused command behavior tests.

The existing command remains:

```bash
npm run verify:package-scripts
```

If accepted, no new alias is required; the package script verifier should own
the additional target-existence invariant.

## M75 Package Script Target Gate Usage

M75 extends the package script verifier:

```bash
npm run verify:package-scripts
```

The command now checks required aliases, aggregate command references, and local
script targets referenced as `node scripts/*.mjs` or direct `scripts/*`
commands. It remains read-only and does not execute the referenced scripts,
validate external commands, or interpret arbitrary shell syntax.

## M75 Final Result

M75 extended package script verification with local script target existence
checks for `node scripts/*.mjs` and direct `scripts/*` references.

Validation completed:

```bash
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate also covered workspace checks and
tests, CLI registry tests, component listing, registry metadata, Tailwind static
tokens, preview structural gates, example smoke, feature checks, generated
fixture smoke, CI documentation checks, CI plan checks, and repository hygiene.

## M76 Quality Gate Alias Coverage Plan

M76 should add a deterministic read-only check that keeps the quality gate
documentation aligned with `package.json` verification aliases. Every npm script
whose name starts with `verify` should be discoverable from
`docs/quality-gates.md` so new local gates are not hidden behind package metadata
only.

The check should validate:

- every `verify` and `verify:*` alias in `package.json` appears in
  `docs/quality-gates.md`
- aggregate aliases such as `verify`, `verify:docs`, `verify:smoke`, and
  `verify:release` are documented explicitly
- focused preview/example aliases are documented even when they are normally
  reached through an aggregate alias
- opt-in browser smoke aliases are documented without promoting them to default
  release gates

The check should remain documentation-coverage focused. It should not execute
npm scripts, validate command behavior, score prose quality, inspect rendered
HTML, or change the release aggregate command.

If accepted, the existing docs index gate can own this invariant because it
already checks required documentation entry points:

```bash
npm run verify:docs-index
```

## M76 Quality Gate Alias Coverage Usage

M76 extends the docs index verifier:

```bash
npm run verify:docs-index
```

The command now checks both required documentation entry links and verification
alias coverage. Every `verify` and `verify:*` script in `package.json` must be
mentioned in `docs/quality-gates.md` as an `npm run ...` command.

This keeps focused aliases such as `verify:web-preview`, `verify:examples`, and
opt-in aliases such as `verify:mobile-browser` discoverable without changing
which commands are part of the default release aggregate.

## M76 Final Result

M76 extended `npm run verify:docs-index` so quality gate documentation must
mention every `verify` and `verify:*` npm alias from `package.json`.

Validation completed:

```bash
npm run verify:docs-index
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate also covered workspace checks and
tests, CLI registry tests, component listing, registry metadata, Tailwind static
tokens, preview structural gates, example smoke, feature checks, generated
fixture smoke, CI documentation checks, CI plan checks, and repository hygiene.

## M77 Release Aggregate Docs Coverage Plan

M77 should add a deterministic read-only check that keeps `docs/release.md`
aligned with the actual `package.json` `verify:release` command chain. The
release documentation should list direct aggregate command segments so manual
review and failure isolation follow the same order as the npm script.

The check should validate:

- the `verify:release` script exists in `package.json`
- every command segment split by `&&` appears in `docs/release.md`
- nested aggregate aliases such as `npm run verify` are documented as direct
  release segments rather than expanded inline
- existing opt-in browser smoke boundary snippets remain present

The check should remain release-documentation focused. It should not execute
release commands, parse arbitrary shell syntax, rewrite `package.json`, expand
nested aliases, or promote browser smoke into the default release aggregate.

The existing release docs verifier should own this invariant:

```bash
npm run verify:release-docs
```

## M77 Release Aggregate Docs Coverage Usage

M77 extends the release documentation verifier:

```bash
npm run verify:release-docs
```

The command now reads `package.json`, splits `verify:release` by direct `&&`
segments, and requires each segment to appear in `docs/release.md`. This keeps
manual release review aligned with the actual npm command chain while preserving
the existing opt-in browser smoke boundary checks.

## M77 Final Result

M77 extended `npm run verify:release-docs` so release documentation must cover
every direct command segment from `package.json` `verify:release`.

Validation completed:

```bash
npm run verify:release-docs
npm run verify:docs
npm run verify:package-scripts
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate also covered workspace checks and
tests, CLI registry tests, component listing, registry metadata, Tailwind static
tokens, preview structural gates, example smoke, feature checks, generated
fixture smoke, CI documentation checks, CI plan checks, and repository hygiene.

## M78 Release Gate Order Verification Plan

M78 should make the release documentation check stricter by verifying that the
expanded release gate block in `docs/release.md` exactly matches the
`package.json` `verify:release` command-chain order.

The check should:

- read `package.json` and split `verify:release` by direct `&&` command segments
- locate the first `bash` code block after the `expanded gate set is:` marker in
  `docs/release.md`
- compare the block line-for-line with the release command segments
- fail on missing, extra, or reordered commands

The check should remain documentation-only. It should not execute release
commands, expand nested npm aliases, parse arbitrary shell syntax, or generate
the release documentation block.

The existing release docs verifier should own this invariant:

```bash
npm run verify:release-docs
```

## M78 Release Gate Order Verification Usage

M78 extends the release documentation verifier:

```bash
npm run verify:release-docs
```

The command now requires the expanded release gate bash block in
`docs/release.md` to match the direct `verify:release` command chain exactly:
same commands, same order, and no extra commands. It still does not execute the
release commands or expand nested aliases.

## M78 Final Result

M78 extended `npm run verify:release-docs` so the expanded release gate block in
`docs/release.md` must exactly match the direct `package.json`
`verify:release` command chain.

Validation completed:

```bash
npm run verify:release-docs
npm run verify:docs
npm run verify:package-scripts
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate also covered workspace checks and
tests, CLI registry tests, component listing, registry metadata, Tailwind static
tokens, preview structural gates, example smoke, feature checks, generated
fixture smoke, CI documentation checks, CI plan checks, and repository hygiene.

## M79 Package Lock Metadata Gate Plan

M79 should add a deterministic read-only check that prevents `package.json` and
`package-lock.json` root metadata from drifting. The repository relies on npm
for local Node verification scripts, so lockfile drift should fail before
release verification reaches dependency-sensitive commands.

The check should validate:

- `package-lock.json` exists and has the expected npm lockfile shape
- lockfile root `name` and `version` match `package.json`
- lockfile root `devDependencies` match `package.json` `devDependencies`
- top-level lockfile `name` and `version` match `package.json`
- lockfile version is supported by the repository's current npm contract

The check should remain metadata-focused. It should not run `npm install`,
resolve dependency freshness, contact the npm registry, inspect transitive
package metadata, install Playwright browsers, or rewrite the lockfile.

If accepted, it should have a focused alias and become part of the release
aggregate:

```bash
npm run verify:package-lock
```

## M79 Package Lock Metadata Gate Usage

M79 adds a package lock metadata verifier:

```bash
npm run verify:package-lock
```

The command checks committed `package-lock.json` root metadata against
`package.json`, including package name, version, lockfile version, and root
devDependencies. It is included in `npm run verify:release`.

The check is read-only and does not resolve dependencies, run `npm install`,
contact the npm registry, install Playwright browsers, or rewrite the lockfile.

## M79 Final Result

M79 added `npm run verify:package-lock` and included it in
`npm run verify:release`. The check validates committed `package-lock.json` root
metadata against `package.json` without network access or lockfile rewrites.

Validation completed:

```bash
npm run verify:package-lock
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate also covered workspace checks and
tests, CLI registry tests, component listing, registry metadata, Tailwind static
tokens, preview structural gates, example smoke, feature checks, generated
fixture smoke, release documentation checks, package script checks, CI
documentation checks, CI plan checks, and repository hygiene.

## M80 Generated Directory Hygiene Gate Plan

M80 should strengthen repository hygiene by preventing generated dependency and
build directories from being tracked. Local directories such as `node_modules/`
and `target/` are expected during development, but they should never become
committed repository content.

The check should validate:

- tracked files do not live under `node_modules/`
- tracked files do not live under `target/`
- tracked files do not live under generated preview or fixture output roots
- project `.gitignore` explicitly documents common local generated directories

The check should remain read-only. It should not delete local directories,
clean Cargo or npm caches, validate global git ignore files, or remove generated
screenshots from the working tree.

The existing repository hygiene verifier should own this invariant:

```bash
npm run verify:repo-hygiene
```

## M80 Generated Directory Hygiene Gate Usage

M80 extends the repository hygiene verifier:

```bash
npm run verify:repo-hygiene
```

The command now fails if tracked files appear under generated directories such
as `node_modules/`, `target/`, `dist/`, or `build/`. The project `.gitignore`
also explicitly lists those local output directories.

The check remains read-only. It reports tracked generated artifacts but does not
delete local dependencies, build outputs, screenshots, or fixture directories.

## M80 Final Result

M80 extended `npm run verify:repo-hygiene` so tracked generated directories
such as `node_modules/`, `target/`, `dist/`, and `build/` fail repository
hygiene verification. The project `.gitignore` now explicitly lists those local
output directories.

Validation completed:

```bash
npm run verify:repo-hygiene
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate also covered workspace checks and
tests, CLI registry tests, component listing, registry metadata, Tailwind static
tokens, preview structural gates, example smoke, feature checks, generated
fixture smoke, release documentation checks, package script checks, package lock
checks, CI documentation checks, CI plan checks, and repository hygiene.

## M81 Cargo Workspace Metadata Gate Plan

M81 should add a deterministic read-only gate for Rust workspace package
metadata. The project already centralizes shared package fields in the root
`[workspace.package]` table, so member crates should keep inheriting those
fields instead of drifting locally.

The check should validate:

- the root workspace defines required package metadata such as `version`,
  `edition`, `license`, and `repository`
- publishable crates under `crates/` inherit those fields with
  `*.workspace = true`
- Cargo metadata resolves the same package field values for workspace members
- the check runs without network access, publishing, packaging, or dependency
  freshness checks

The gate should remain static and read-only. It should not run `cargo publish`,
rewrite manifests, regenerate lockfiles, inspect crates.io, or enforce
application example package metadata.

The intended command is:

```bash
npm run verify:cargo-workspace
```

## M81 Cargo Workspace Metadata Gate Usage

M81 adds a focused Cargo workspace metadata verifier:

```bash
npm run verify:cargo-workspace
```

The command checks that root `[workspace.package]` metadata defines `version`,
`edition`, `license`, and `repository`, that crate manifests under `crates/`
inherit those fields with `*.workspace = true`, and that `cargo metadata`
resolves matching values for those workspace crate packages.

The check is included in `npm run verify:release`. It remains read-only and
does not run `cargo publish`, package crates, contact crates.io, rewrite
manifests, update dependency versions, or enforce example application package
metadata.

## M81 Final Result

M81 added `npm run verify:cargo-workspace` and wired it into
`npm run verify:release`. The gate validates root workspace package metadata,
crate manifest inheritance under `crates/`, and resolved `cargo metadata`
values for workspace crate packages.

Validation completed:

```bash
npm run verify:cargo-workspace
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata,
registry metadata, Tailwind static tokens, preview structural gates, example
smoke, feature checks, generated fixture smoke, release documentation checks,
package script checks, package lock checks, CI documentation checks, CI plan
checks, and repository hygiene.

## M82 Quality Gate Release Block Sync Plan

M82 should add a deterministic read-only check that keeps the expanded release
gate block in `docs/quality-gates.md` aligned with the `package.json`
`verify:release` command chain. `docs/release.md` already has strict direct
segment coverage, and the quality gate reference should not drift from that
same release contract.

The check should validate:

- `package.json` exposes `verify:release`
- `docs/quality-gates.md` contains the intended release gate marker and bash
  block
- the quality gate block lists every direct `verify:release` command segment
  exactly once and in order
- the check runs as part of docs verification without executing release gates

The gate should remain static and read-only. It should not expand nested npm
aliases, execute Cargo or browser commands, rewrite Markdown, or replace the
dedicated `docs/release.md` release documentation verifier.

The intended owner is:

```bash
npm run verify:docs-index
```

## M82 Quality Gate Release Block Sync Usage

M82 extends the documentation index verifier:

```bash
npm run verify:docs-index
```

The command now checks that the release gate block in `docs/quality-gates.md`
matches the direct `verify:release` command segments from `package.json` in
order. The check is included in `npm run verify:docs`.

The check remains read-only. It does not execute release commands, expand
nested aliases such as `npm run verify`, rewrite Markdown, or replace the
dedicated release documentation verifier for `docs/release.md`.

## M82 Final Result

M82 extended `npm run verify:docs-index` so the release gate block in
`docs/quality-gates.md` must match the direct `verify:release` command segments
from `package.json` in order. The quality gate reference now uses the same
direct release contract as the release aggregate.

Validation completed:

```bash
npm run verify:docs-index
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate covered workspace checks and tests,
CLI registry tests, component listing, Cargo workspace metadata, registry
metadata, Tailwind static tokens, preview structural gates, example smoke,
feature checks, generated fixture smoke, release documentation checks, package
script checks, package lock checks, CI documentation checks, CI plan checks,
and repository hygiene.

## M83 Cargo Lock Metadata Gate Plan

M83 should add a deterministic read-only gate for committed Cargo lockfile
metadata. The Rust workspace already validates manifests through Cargo and
workspace package metadata, but `Cargo.lock` can still drift from workspace
package names or versions in a way that is useful to catch before release.

The check should validate:

- `Cargo.lock` exists and uses the expected lockfile format
- every Cargo workspace member from `cargo metadata --no-deps` has a matching
  lockfile package entry
- lockfile package versions for workspace members match Cargo metadata
- duplicate lockfile entries do not hide ambiguous workspace package metadata

The gate should remain static and read-only. It should not run `cargo update`,
rewrite `Cargo.lock`, resolve dependency freshness, inspect crates.io, or
replace normal `cargo check` and `cargo test` release gates.

The intended command is:

```bash
npm run verify:cargo-lock
```

## M83 Cargo Lock Metadata Gate Usage

M83 adds a focused Cargo lock metadata verifier:

```bash
npm run verify:cargo-lock
```

The command checks that `Cargo.lock` exists, uses the expected lockfile format,
and includes local workspace package entries whose names and versions match
`cargo metadata --locked --no-deps` workspace members.

The check is included in `npm run verify:release`. It remains read-only and
does not run `cargo update`, rewrite `Cargo.lock`, resolve dependency
freshness, contact crates.io, or replace normal Rust compile and test gates.

## M83 Final Result

M83 added `npm run verify:cargo-lock` and wired it into
`npm run verify:release`. The gate validates committed `Cargo.lock` metadata
against `cargo metadata --locked --no-deps` workspace members, including
lockfile format, workspace package presence, and workspace package versions.

Validation completed:

```bash
npm run verify:cargo-lock
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, registry metadata, Tailwind static tokens, preview structural
gates, example smoke, feature checks, generated fixture smoke, release
documentation checks, package script checks, package lock checks, CI
documentation checks, CI plan checks, and repository hygiene.

## M84 Pre-commit Config Metadata Gate Plan

M84 should add a deterministic read-only gate for committed pre-commit
configuration metadata. The repository uses `.pre-commit-config.yaml` for local
developer checks, and release verification should catch obvious drift in local
hook wiring without installing or executing pre-commit hooks.

The check should validate:

- `.pre-commit-config.yaml` exists and defines the local repository hook block
- required local hook ids are present: `cargo-fmt`, `cargo-deny`, `typos`,
  `cargo-check`, `cargo-clippy`, and `cargo-test`
- required local hook entry command snippets remain aligned with the project
  conventions
- supporting config files such as `deny.toml`, `rustfmt.toml`, and
  `_typos.toml` are present when local hooks reference those tool families

The gate should remain static and read-only. It should not run `pre-commit`,
install hook environments, execute Cargo commands, contact remote hook
repositories, check remote hook freshness, or rewrite YAML.

The intended command is:

```bash
npm run verify:pre-commit
```

## M84 Pre-commit Config Metadata Gate Usage

M84 adds a focused pre-commit metadata verifier:

```bash
npm run verify:pre-commit
```

The command checks that `.pre-commit-config.yaml` has the expected local hook
ids and entry command snippets, and that supporting config files such as
`deny.toml`, `rustfmt.toml`, and `_typos.toml` exist.

The check is included in `npm run verify:release`. It remains read-only and
does not run `pre-commit`, install hook environments, execute Cargo or typos
commands, contact remote hook repositories, check remote hook freshness, or
rewrite YAML.

## M84 Final Result

M84 added `npm run verify:pre-commit` and wired it into
`npm run verify:release`. The gate validates committed `.pre-commit-config.yaml`
local hook metadata, expected local hook command snippets, and supporting config
files without executing pre-commit hooks.

Validation completed:

```bash
npm run verify:pre-commit
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, pre-commit metadata, registry metadata, Tailwind static tokens,
preview structural gates, example smoke, feature checks, generated fixture
smoke, release documentation checks, package script checks, package lock
checks, CI documentation checks, CI plan checks, and repository hygiene.

## M85 Script Metadata Gate Plan

M85 should add a deterministic read-only gate for repository script metadata.
Package and release verification already check script wiring, but direct shell
script targets also rely on committed executable bits and shebangs. Runnable
Node scripts should keep a stable Node shebang, while helper-only modules can
remain import-only.

The check should validate:

- shell scripts under `scripts/` use `#!/usr/bin/env bash`
- shell scripts under `scripts/` are executable
- direct package script targets such as `scripts/example-smoke.sh` are
  executable when invoked without `node`
- runnable `.mjs` scripts under `scripts/` use `#!/usr/bin/env node`
- known helper-only modules such as `scripts/docs-catalog-builder.mjs` are
  allowed to omit a shebang

The gate should remain static and read-only. It should not execute scripts,
lint shell syntax, install dependencies, rewrite file modes, inspect generated
outputs, or replace package script wiring checks.

The intended command is:

```bash
npm run verify:scripts
```

## M85 Script Metadata Gate Usage

M85 adds a focused script metadata verifier:

```bash
npm run verify:scripts
```

The command checks shell script shebangs and executable bits, direct package
script targets, and Node shebangs for runnable `.mjs` scripts. Helper-only
modules such as `scripts/docs-catalog-builder.mjs` may remain import-only.

The check is included in `npm run verify:release`. It remains read-only and
does not execute scripts, lint shell syntax, install dependencies, inspect
generated outputs, or rewrite file modes.

## M85 Final Result

M85 added `npm run verify:scripts` and wired it into
`npm run verify:release`. The gate validates committed script metadata,
including shell shebangs, executable bits, direct package script targets, and
Node shebangs for runnable `.mjs` scripts.

Validation completed:

```bash
npm run verify:scripts
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, pre-commit metadata, script metadata, registry metadata,
Tailwind static tokens, preview structural gates, example smoke, feature
checks, generated fixture smoke, release documentation checks, package script
checks, package lock checks, CI documentation checks, CI plan checks, and
repository hygiene.

## M86 README Verification Summary Gate Plan

M86 should add a deterministic read-only check that keeps the README
verification shortcut section aligned with package script metadata and quality
gate documentation. The README is the first handoff surface for contributors,
so it should continue to expose the default local gate, docs aggregate, smoke
aggregate, release aggregate, and focused metadata gates as the verification
surface grows.

The check should validate:

- README keeps the primary aliases discoverable, including `npm run verify`,
  `npm run verify:docs`, `npm run verify:smoke`, and
  `npm run verify:release`
- README mentions focused metadata gates that are easy to forget, including
  package scripts, release docs, Cargo workspace metadata, Cargo lock metadata,
  pre-commit metadata, script metadata, and repository hygiene
- README release shortcut prose acknowledges the same high-level gate groups
  documented in `docs/quality-gates.md`
- README docs aggregate expansion stays aligned with the package script
  requirements for `verify:docs`

The gate should not execute referenced commands, crawl external links, render
the docs site, rewrite README prose, or replace the stricter release docs and
package script checks.

The intended command is:

```bash
npm run verify:readme
```

## M86 README Verification Summary Gate Usage

M86 adds a focused README verification summary command:

```bash
npm run verify:readme
```

The command checks that the root `README.md` keeps verification shortcuts
discoverable for the default local gate, docs aggregate, smoke aggregate,
release aggregate, focused metadata gates, and direct `verify:docs` command
segments from package metadata.

The check is included in:

```bash
npm run verify:docs
```

It remains read-only and does not execute referenced commands, crawl external
links, render the docs site, install dependencies, or rewrite README prose.

## M86 Final Result

M86 added `npm run verify:readme` and wired it into `npm run verify:docs`. The
gate validates README verification shortcut discoverability, focused metadata
alias coverage, release summary coverage, and direct `verify:docs` command
segments from package metadata.

Validation completed:

```bash
npm run verify:readme
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, pre-commit metadata, script metadata, registry metadata,
Tailwind static tokens, preview structural gates, example smoke, docs metadata
including README verification, feature checks, generated fixture smoke, release
documentation checks, package script checks, package lock checks, CI
documentation checks, CI plan checks, and repository hygiene.

## M87 Examples Metadata Gate Plan

M87 should add a deterministic read-only check that keeps example metadata
aligned across workspace configuration, package scripts, smoke scripts, and
documentation. The repository now has command-line Web and Desktop demos,
shared preview-state fixtures, runtime Web verification, and runtime Desktop
verification. Those entry points should remain discoverable without relying on
release-time command execution to reveal documentation drift.

The check should validate:

- every `examples/*/Cargo.toml` package is a Cargo workspace member
- `examples/README.md` documents the Web demo, Desktop demo, runtime Web
  verification fixture, runtime Desktop verification fixture, CLI init/add
  smoke commands, generated fixture smoke, and example smoke
- package metadata exposes `npm run verify:examples`
- `scripts/example-smoke.sh` still runs the Web and Desktop demo packages
- rendered preview checks continue to point at the documented Web and Desktop
  preview packages

The gate should not run examples, launch rendered previews, install browser
dependencies, compile generated fixtures, rewrite docs, or replace runtime and
browser verification checks.

The intended command is:

```bash
npm run verify:examples-metadata
```

## M87 Examples Metadata Gate Usage

M87 adds a focused examples metadata command:

```bash
npm run verify:examples-metadata
```

The command checks example Cargo manifests, Cargo workspace membership,
`examples/README.md`, `verify:examples` package script wiring,
`scripts/example-smoke.sh`, and rendered preview verifier references.

The check is included in:

```bash
npm run verify:release
```

It remains read-only and does not run examples, launch rendered previews,
install browser dependencies, compile generated fixtures, or rewrite
documentation.

## M87 Final Result

M87 added `npm run verify:examples-metadata` and wired it into
`npm run verify:release`. The gate validates example Cargo manifests, Cargo
workspace membership, examples documentation, package script wiring, example
smoke script references, and rendered preview verifier references without
running examples.

Validation completed:

```bash
npm run verify:examples-metadata
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, pre-commit metadata, script metadata, examples metadata,
registry metadata, Tailwind static tokens, preview structural gates, example
smoke, docs metadata including README verification, feature checks, generated
fixture smoke, release documentation checks, package script checks, package
lock checks, CI documentation checks, CI plan checks, and repository hygiene.

## M88 RFC Metadata Gate Plan

M88 should add a deterministic read-only check that keeps RFC metadata
discoverable and internally consistent. RFC files are stable planning anchors,
but their numbering, first headings, root README links, and docs README index
can drift as new RFCs are added.

The check should validate:

- every `docs/rfcs/*.md` file uses a `NNNN-kebab-title.md` filename
- each RFC has a first heading shaped as `# RFC NNNN: Title`
- RFC numbers are unique and contiguous from `0001`
- root `README.md` links every current RFC document
- `docs/README.md` links every current RFC document with matching titles

The gate should not review RFC prose, decide whether RFCs are accepted,
validate implementation status, crawl external links, render the docs site, or
rewrite index files.

The intended command is:

```bash
npm run verify:rfcs
```

## M88 RFC Metadata Gate Usage

M88 adds a focused RFC metadata command:

```bash
npm run verify:rfcs
```

The command checks RFC filenames, first headings, contiguous numbering, root
README links, and docs README links for every `docs/rfcs/*.md` file.

The check is included in:

```bash
npm run verify:docs
```

It remains read-only and does not review RFC prose, decide acceptance status,
validate implementation status, crawl external links, render the docs site, or
rewrite index files.

## M88 Final Result

M88 added `npm run verify:rfcs` and wired it into `npm run verify:docs`. The
gate validates RFC filename shape, first-heading numbering, contiguous RFC
sequence, root README links, and docs README links for every current RFC.

Validation completed:

```bash
npm run verify:rfcs
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, pre-commit metadata, script metadata, examples metadata,
registry metadata, Tailwind static tokens, preview structural gates, example
smoke, docs metadata including RFC and README verification, feature checks,
generated fixture smoke, release documentation checks, package script checks,
package lock checks, CI documentation checks, CI plan checks, and repository
hygiene.

## M89 CSS Input Metadata Gate Plan

M89 should add a deterministic read-only check that keeps Tailwind CSS v4 input
metadata aligned across generated CLI defaults, rendered preview CSS inputs, and
documentation. The project intentionally treats `assets/dioxus-ui.css` and
preview `preview.css` files as Tailwind input stylesheets, not committed full
Tailwind output.

The check should validate:

- CLI default generated CSS includes `@import "tailwindcss";`
- CLI default generated CSS keeps the documented `@theme` bridge tokens
- preview CSS inputs use Tailwind CSS v4 `@import "tailwindcss";`
- preview CSS inputs include required `@source` roots for crates, templates,
  local preview source, and shared preview states
- committed CSS inputs do not reintroduce Tailwind CSS v3 directives such as
  `@tailwind base`, `@tailwind components`, or `@tailwind utilities`

The gate should not compile Tailwind CSS, inspect generated CSS output, launch
preview binaries, perform browser automation, scan Rust class tokens, or assert
visual parity.

The intended command is:

```bash
npm run verify:css-inputs
```

## M89 CSS Input Metadata Gate Usage

M89 adds a focused CSS input metadata command:

```bash
npm run verify:css-inputs
```

The command checks the CLI default generated `assets/dioxus-ui.css` content in
`crates/dioxus-ui-cli/src/main.rs` and the rendered preview CSS inputs:

```text
examples/web-demo/assets/preview.css
examples/desktop-demo/assets/preview.css
```

The check is included in:

```bash
npm run verify:release
```

It remains read-only and does not compile Tailwind CSS, inspect generated CSS
output, launch previews, run browser automation, scan Rust class tokens, or
assert visual parity.

## M89 Final Result

M89 added `npm run verify:css-inputs` and wired it into
`npm run verify:release`. The gate validates Tailwind CSS v4 input metadata for
CLI default CSS and rendered preview CSS inputs, including required `@source`
roots and the absence of Tailwind CSS v3 directives.

Validation completed:

```bash
npm run verify:css-inputs
npm run verify:package-scripts
npm run verify:docs
npm run verify:release-docs
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate now covers workspace checks and
tests, CLI registry tests, component listing, Cargo workspace metadata, Cargo
lock metadata, pre-commit metadata, script metadata, examples metadata, CSS
input metadata, registry metadata, Tailwind static tokens, preview structural
gates, example smoke, docs metadata including RFC and README verification,
feature checks, generated fixture smoke, release documentation checks, package
script checks, package lock checks, CI documentation checks, CI plan checks,
and repository hygiene.

## M90 Gitignore Metadata Gate Plan

M90 should add a deterministic read-only check that keeps `.gitignore`,
repository hygiene policy, and documented artifact patterns aligned. Local
development can create `node_modules/`, Cargo target outputs, Playwright state,
preview screenshots, and generated fixture directories; those should stay
ignored and should continue to be rejected if accidentally tracked.

The check should validate:

- `.gitignore` lists common local generated directories such as `node_modules/`,
  `target/`, `dist/`, and `build/`
- `.gitignore` lists browser automation state such as `.playwright-mcp/`
- `.gitignore` lists generated preview screenshot patterns for Web, Desktop
  WebView, and Mobile browser smoke
- `scripts/repo-hygiene-verify.mjs` rejects tracked generated directories and
  known generated screenshot artifacts
- package metadata exposes the focused gitignore metadata gate

The gate should not delete files, inspect ignored artifact contents, validate
global Git excludes, crawl temporary directories, or replace repository hygiene
tracking checks.

The intended command is:

```bash
npm run verify:gitignore
```

## M74 Tailwind Static Token Gate Plan

M74 should add a deterministic read-only check that prevents dynamic Tailwind
class token interpolation in shipped component source. Tailwind scans source as
text, so tokens such as `bg-{color}-600` or `text-{tone}-foreground` are not
safe defaults for a source-copy component library.

The check should scan:

- `crates/dioxus-ui/src/**/*.rs`
- `crates/dioxus-ui-cli/templates/**/*.rs`
- `crates/dioxus-ui-core/src/**/*.rs`

The check should fail on common dynamic utility prefixes followed by
interpolation, including:

- color tokens such as `bg-{...}`, `text-{...}`, `border-{...}`, `ring-{...}`
- gradient tokens such as `from-{...}`, `via-{...}`, `to-{...}`
- SVG color tokens such as `fill-{...}` and `stroke-{...}`
- spacing and sizing tokens such as `p-{...}`, `px-{...}`, `m-{...}`, `h-{...}`, and `w-{...}`
- grid tokens such as `grid-cols-{...}`, `col-span-{...}`, and `row-span-{...}`

The check should remain static and conservative. It should not compile
Tailwind, generate CSS, validate user-provided `class` props, inspect rendered
HTML, or assert visual parity.

The expected command should be:

```bash
npm run verify:tailwind-static
```

If accepted, the command should be included in package script consistency and
release verification documentation.

## M74 Tailwind Static Token Gate Usage

M74 adds the Tailwind static token check:

```bash
npm run verify:tailwind-static
```

The check scans shipped Rust source and source-copy templates for dynamic
Tailwind utility interpolation such as `bg-{...}`, `text-{...}`, spacing
interpolation, gradient interpolation, and common grid/sizing interpolation.

It is included in:

```bash
npm run verify:release
```

The check remains source-shape focused. It does not compile Tailwind CSS,
validate user-provided `class` props, inspect rendered HTML, or assert visual
parity.

## M74 Final Result

M74 added a read-only Tailwind static token gate and included it in release
verification. The gate scans 132 shipped Rust source and source-copy template
files for dynamic Tailwind utility interpolation such as `bg-{...}`,
`text-{...}`, spacing interpolation, gradient interpolation, and common
grid/sizing interpolation.

Validation completed:

```bash
npm run verify:tailwind-static
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. The release aggregate also covered registry metadata,
workspace checks and tests, CLI registry tests, component listing, preview
structural gates, example smoke, feature checks, generated fixture smoke, CI
documentation checks, and repository hygiene.

## M73 Registry Metadata Gate Plan

M73 should add a fast npm-side registry metadata check for `registry/*.json`.
The goal is to catch source-copy metadata drift before slower Rust tests or
generated fixture smoke run.

The check should validate:

- registry filenames match the `name` field
- names use component slug format
- descriptions are non-empty strings
- `files` is a non-empty array
- every file source exists in the repository
- every file target stays under `src/components/ui/`
- dependency names point to existing registry entries
- asset mappings, when present, have existing sources and non-empty targets
- no duplicate registry names or source-copy targets exist

The check should remain deterministic and read-only. It should not execute CLI
commands, compile Rust crates, replace the existing CLI registry tests, validate
against a full JSON Schema implementation, or run generated fixture smoke.

The expected command should be:

```bash
npm run verify:registry
```

If accepted, the command should be included in package script consistency and
release verification documentation.

## M73 Registry Metadata Gate Usage

M73 adds the registry metadata check:

```bash
npm run verify:registry
```

The check scans `registry/*.json` entries and verifies names, descriptions,
file mappings, dependency references, asset mappings, existing sources, and
source-copy targets under `src/components/ui/`.

It is included in:

```bash
npm run verify:release
```

The check remains metadata-only. It does not execute CLI commands, compile Rust
crates, replace the CLI registry tests, or run generated fixture smoke.

## M73 Final Result

M73 added a read-only registry metadata gate and included it in release
verification. The gate validates 65 registry entries for stable names,
descriptions, file mappings, source-copy targets, dependency references, asset
mappings, existing sources, and duplicate targets.

Validation completed:

```bash
npm run verify:registry
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
npm run verify:repo-hygiene
CARGO_NET_OFFLINE=true npm run verify:release
git diff --check
```

All commands passed. A normal online `npm run verify:release` reached generated
fixture smoke but failed while Cargo was updating the crates.io index due an SSL
connection error, so the final release verification was rerun in Cargo offline
mode with cached dependencies.

## M72 Component Docs Structure Gate Plan

M72 should add a deterministic gate for public component documentation shape.
The goal is to catch incomplete component docs when registry entries, templates,
or crate features are added without the expected user-facing sections.

The check should derive its public component list from:

- `scripts/docs-catalog-builder.mjs`
- `docs/components/*.md`
- `registry/*.json`
- `templates/*.rs`
- `crates/dioxus-ui/Cargo.toml`

Each public component docs page should include:

- a level-one title matching the catalog title
- a `Source Copy` section with the expected `dxui add <component>` command
- a `Crate Feature` section with the expected `dioxus-ui` feature snippet
- an `API Surface` section with at least one bullet
- an `Accessibility Notes` section with non-empty prose

The check should remain read-only and deterministic. It should not judge prose
quality, validate rendered HTML, refresh live upstream shadcn/ui parity, inspect
screenshots, or assert runtime visual parity.

The expected command should be:

```bash
npm run verify:docs-structure
```

If the gate is accepted, it should be included in `npm run verify:docs` and
documented with the other release-quality checks.

## M72 Component Docs Structure Gate Usage

M72 adds the component docs structure check:

```bash
npm run verify:docs-structure
```

The check scans public component docs pages from the shared docs catalog and
verifies required sections plus generated command and feature snippets. It is
included in:

```bash
npm run verify:docs
```

The check remains documentation-shape focused. It does not score prose quality,
refresh upstream shadcn/ui parity, validate rendered HTML, inspect screenshots,
or assert runtime visual parity.

## M72 Final Result

M72 added a read-only component docs structure gate and included it in the docs
aggregate. The gate validates 64 public component docs pages for required
headings, generated source-copy commands, generated crate feature snippets, API
Surface bullets, and Accessibility Notes prose.

Validation completed:

```bash
npm run verify:docs-structure
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:repo-hygiene
npm run verify:release
git diff --check
```

All commands passed. `verify:release` also covered workspace checks and tests,
CLI registry tests, component listing, preview structural gates, example smoke,
feature checks, generated fixture smoke, CI documentation checks, and repository
hygiene.

## M70 Final Result

M70 added the component status snapshot:

```text
docs/components/status.md
```

The page is generated by:

```bash
node scripts/docs-component-status.mjs
```

The drift check is:

```bash
npm run verify:docs-status
```

It reports the local implemented component surface: 64 public components, 65
registry entries including `utils`, 64 crate features, 64 styled crate modules,
and 64 component docs pages. The check is included in `npm run verify:docs`.

Validation ran:

```bash
npm run verify:docs-status
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:repo-hygiene
npm run verify:release
git diff --check
```

All commands passed. No workflow files or generated artifacts were committed.

## M71 Component Coverage Matrix Plan

M71 should make the component status snapshot more useful by showing coverage
across the local implementation surfaces each component needs:

- component docs page
- source-copy template
- generated source-copy target
- crate feature
- styled crate module
- source preview route and source file metadata

The matrix should be derived from `scripts/docs-catalog-builder.mjs` and should
summarize how many public components have complete local wiring. A component is
complete for this matrix when all listed local surfaces are present in the
catalog metadata.

Out of scope for this milestone:

- upstream shadcn/ui parity refresh
- runtime visual preview status
- screenshot coverage status
- adding or changing components
- generated JSON artifacts

## M71 Component Coverage Matrix Usage

M71 expands the component status snapshot:

```text
docs/components/status.md
```

The page now includes a coverage matrix for each public component:

- docs page
- source-copy template
- generated source-copy target
- crate feature
- styled crate module
- source preview metadata
- complete local wiring

The drift check remains:

```bash
npm run verify:docs-status
```

The command verifies local wiring coverage only. It does not refresh upstream
shadcn/ui parity, run visual previews, or assert screenshot coverage.

## M71 Final Result

M71 expanded the generated component status snapshot with a coverage matrix for
local component wiring. The matrix records whether each public component has a
docs page, source-copy template, generated target, crate feature, styled crate
module, source preview metadata, and complete local wiring.

Validation completed:

```bash
npm run verify:docs-status
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:repo-hygiene
npm run verify:release
git diff --check
```

All commands passed. `verify:release` also covered workspace checks and tests,
CLI registry tests, component listing, preview structural gates, example smoke,
feature checks, generated fixture smoke, CI documentation checks, and repository
hygiene.

## M67 Final Result

M67 added the local Markdown link target check:

```bash
npm run verify:docs-links
```

The command scans tracked Markdown files and verifies local relative link
targets exist after stripping query strings and fragments. It is included in
`npm run verify:docs`, and package script consistency checks now require the
alias and aggregate wiring.

Validation ran:

```bash
npm run verify:docs-links
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:release
git diff --check
test ! -e .github/workflows/browser-smoke.yml
```

All commands passed. No workflow files or generated artifacts were committed.

## M68 Local Markdown Anchor Gate Plan

M68 should verify local Markdown fragment links after M67 has proven that local
file targets exist. This milestone remains documentation-only and read-only. It
should not render Markdown, crawl external URLs, validate generated docs routes,
or check anchors on remote sites.

The check should scan tracked Markdown files and validate fragment links for:

- same-file links such as `#usage`
- relative Markdown links such as `docs/site.md#m68-local-markdown-anchor-gate-plan`
- explicit HTML anchors such as `<a id="usage"></a>` or `<a name="usage"></a>`

Heading slugs should follow GitHub-style Markdown behavior closely enough for
repository documentation:

- lowercase heading text
- strip inline Markdown formatting and HTML tags
- remove punctuation that is not a word, CJK character, space, or hyphen
- collapse whitespace to `-`
- append `-1`, `-2`, and later suffixes for duplicate headings

Out of scope for this milestone:

- external URL fragment validation
- generated docs runtime route anchors such as `/components#category-layout`
- rendered HTML heading ids
- non-Markdown source scanning
- network access

## M68 Local Markdown Anchor Gate Usage

M68 adds the local Markdown anchor check:

```bash
npm run verify:docs-anchors
```

The command scans tracked Markdown files and verifies same-file and relative
Markdown fragments against target headings or explicit anchors. It does not
validate external URL fragments, generated docs runtime routes, rendered HTML,
or remote content.

Because the check is deterministic and read-only, it is included in:

```bash
npm run verify:docs
```

## M68 Final Result

M68 added the local Markdown anchor check:

```bash
npm run verify:docs-anchors
```

The command scans tracked Markdown files and verifies same-file and relative
Markdown fragments against headings or explicit anchors. It is included in
`npm run verify:docs`, and package script consistency checks now require the
alias and aggregate wiring.

Validation ran:

```bash
npm run verify:docs-anchors
npm run verify:docs
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:release
git diff --check
test ! -e .github/workflows/browser-smoke.yml
```

All commands passed. No workflow files or generated artifacts were committed.

## M69 Repository Hygiene Gate Plan

M69 should turn repeated manual repository hygiene checks into a deterministic
read-only gate. This milestone should report forbidden committed artifacts; it
should not delete files, rewrite the working tree, prune dependencies, or run
formatters.

The check should verify:

- `.github/workflows/browser-smoke.yml` is still absent until the browser smoke
  workflow is explicitly reviewed and activated.
- mobile browser screenshot artifacts matching
  `dioxus-ui-mobile-browser-preview-*.png` are not tracked.
- generated fixture directories or archives that match repository-local smoke
  artifact naming are not tracked.

Out of scope for this milestone:

- deleting untracked files
- checking ignored temporary directories outside the repository
- enforcing `.gitignore` contents
- validating CI provider configuration
- promoting browser smoke to a required merge gate

## M69 Repository Hygiene Gate Usage

M69 adds the repository hygiene check:

```bash
npm run verify:repo-hygiene
```

The command checks tracked files and the repository workspace for known
forbidden artifacts, including the inactive browser smoke workflow and mobile
browser screenshot artifacts. It is read-only and reports drift without
removing files.

Because the check is deterministic and read-only, it is included in:

```bash
npm run verify:release
```

## M69 Final Result

M69 added the repository hygiene check:

```bash
npm run verify:repo-hygiene
```

The command checks tracked files and repository-local paths for forbidden
artifacts such as the inactive browser smoke workflow and generated mobile
browser screenshots. It is included in `npm run verify:release`, and package
script consistency checks now require the alias and aggregate wiring.

Validation ran:

```bash
npm run verify:repo-hygiene
npm run verify:package-scripts
npm run verify:release-docs
npm run verify:docs
npm run verify:release
git diff --check
```

All commands passed. No workflow files or generated artifacts were committed.

## M70 Component Status Snapshot Plan

M70 should make the current implemented component surface easier to audit by
adding a deterministic Markdown status page derived from the docs catalog
builder. The page should answer which public components currently exist and
which source-copy, crate feature, template, and docs entry points are present.

The status snapshot should derive from:

- `registry/*.json`
- `templates/*.rs`
- `crates/dioxus-ui/Cargo.toml`
- `crates/dioxus-ui/src/*.rs`
- `docs/components/*.md`
- `scripts/docs-catalog-builder.mjs`

The page should summarize:

- public component count
- source-copy helper count
- category counts
- per-component docs, CLI command, feature, template path, and generated target

Out of scope for this milestone:

- refreshing live upstream shadcn/ui parity
- adding new components
- visual parity claims
- rendered preview screenshots
- generated JSON artifacts

## M70 Component Status Snapshot Usage

M70 adds the component status snapshot:

```text
docs/components/status.md
```

The snapshot is generated by:

```bash
node scripts/docs-component-status.mjs
```

The drift check is:

```bash
npm run verify:docs-status
```

The command verifies that the status page still matches local registry,
template, docs, crate feature, and crate module metadata. It does not refresh
live upstream shadcn/ui parity or assert visual parity.

Because the check is deterministic and read-only, it is included in:

```bash
npm run verify:docs
```
