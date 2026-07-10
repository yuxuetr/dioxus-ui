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

- `registry/*.json`
- `templates/*.rs`
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
| `name` | `registry/*.json` | Stable CLI/source-copy component id. |
| `description` | `registry/*.json` | Short catalog summary. |
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

## M74 Tailwind Static Token Gate Plan

M74 should add a deterministic read-only check that prevents dynamic Tailwind
class token interpolation in shipped component source. Tailwind scans source as
text, so tokens such as `bg-{color}-600` or `text-{tone}-foreground` are not
safe defaults for a source-copy component library.

The check should scan:

- `crates/dioxus-ui/src/**/*.rs`
- `templates/**/*.rs`
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
