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

Validation ran:

```bash
npm run verify:release
git diff --check
```

All commands passed. Browser installation, screenshots, and native runtime
automation remain opt-in and outside the release aggregate.
