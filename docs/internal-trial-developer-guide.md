# Internal Trial Developer Guide

This guide explains how to try `dioxus-shadcn` inside an internal Dioxus project
before the crates are published. It is for controlled evaluation, not a stable
commercial release.

Use this guide with:

- [Component Catalog](components/catalog.md)
- [Component Status](components/status.md)
- [Quality Gates](quality-gates.md)
- [Publish Readiness Blockers](publish-readiness-blockers.md)

## Trial Scope

Internal trial is appropriate for:

- evaluating component APIs in one or two real internal apps
- validating source-copy ergonomics through `dxui init` and `dxui add`
- checking Tailwind CSS v4 integration in a real Dioxus build
- collecting API, accessibility, runtime, and docs feedback before public
  publish

Internal trial should not claim:

- stable `1.0` APIs
- publish-ready crates
- complete commercial accessibility guarantees
- native Mobile or Desktop behavior beyond the documented verification gates
- support for `cargo install dioxus-shadcn-cli` from crates.io

## Recommended Mode

Use source-copy mode first. It matches the shadcn-style workflow and lets trial
apps edit generated component source locally.

From the `dioxus-shadcn` repository, list available components:

```bash
cargo run -q -p dioxus-shadcn-cli -- list
```

Initialize a trial app:

```bash
cargo run -q -p dioxus-shadcn-cli -- init --root /path/to/trial-app
```

Add components:

```bash
cargo run -q -p dioxus-shadcn-cli -- add button --root /path/to/trial-app
cargo run -q -p dioxus-shadcn-cli -- add input --root /path/to/trial-app
cargo run -q -p dioxus-shadcn-cli -- add dialog --root /path/to/trial-app
```

Use `--overwrite` only when intentionally replacing locally generated files:

```bash
cargo run -q -p dioxus-shadcn-cli -- add button --root /path/to/trial-app --overwrite
```

Generated files are written under:

```text
assets/dioxus-shadcn.css
src/components/ui/
```

`assets/dioxus-shadcn.css` is a Tailwind CSS v4 input stylesheet. It is not a full
compiled Tailwind output.

## Trial App Wiring

In the trial app, import generated modules from `src/components/ui/mod.rs`.
Example:

```rust
mod components;

use components::ui::button::Button;
```

Keep generated source in the trial app repository so reviewers can inspect API
shape, class names, variants, and local customization needs.

## Optional Crate Path Trial

Crate mode can be evaluated with local path dependencies, but it should be
treated as less stable than source-copy mode until API stability is approved.

Example trial app dependency:

```toml
[dependencies]
dioxus-shadcn = { path = "/path/to/dioxus-shadcn/crates/dioxus-shadcn", default-features = false, features = ["button", "input", "dialog"] }
```

Use this only for API feedback. Do not treat path dependency behavior as a
published crate compatibility guarantee.

## Suggested Trial Component Set

Start with lower-risk composition and form components:

```text
button
input
textarea
label
card
badge
alert
avatar
separator
tabs
accordion
checkbox
switch
radio-group
select
```

Then evaluate overlays separately:

```text
dialog
popover
dropdown
tooltip
sheet
alert-dialog
combobox
```

Overlay feedback should explicitly cover focus behavior, keyboard navigation,
dismissal behavior, ARIA expectations, positioning, and runtime differences
between Web, Desktop, and Mobile targets.

## Local Verification

Run these in the `dioxus-shadcn` repository before giving a commit to trial users:

```bash
npm run verify:docs
npm run verify:registry
npm run verify:cli-template-packaging-readiness
scripts/generated-fixture-smoke.sh
cargo test -p dioxus-shadcn-cli
cargo check -p dioxus-shadcn-cli
git diff --check
```

For a broader local confidence check:

```bash
npm run verify:release
```

The release gate is local and deterministic by default. It still does not
publish crates, create package archives, contact crates.io, or authorize a
commercial release.

## Feedback Checklist

Collect feedback in these categories:

- component names and feature names
- prop naming consistency
- enum variants and size/variant coverage
- Tailwind class override ergonomics
- source-copy file organization
- docs clarity and missing examples
- accessibility gaps
- keyboard navigation gaps
- Web/Desktop/Mobile runtime differences
- components that need app-owned state or integration hooks

Feedback that changes public API should feed into the API stability decision
preparation documents before public publish.

## Publish Blockers And Parallel Work

Internal trial can proceed with every locally resolvable publish readiness item
in place:

- repository identity: `https://github.com/yuxuetr/dioxus-ui`
- license: MIT with committed root `LICENSE`
- API stability: current `0.1.x` API surface accepted for first publish;
  breaking changes before `1.0` require a minor bump and a `CHANGELOG.md`
  migration note
- release notes: first publish scope, excluded scope, and known warnings in the
  `CHANGELOG.md` Unreleased section
- workspace dependencies: internal crates declare `version = "0.1.0"`
  alongside local paths

The only remaining blocker is crates.io registry availability. It stays
deferred until a release owner confirms crate names, owners, credentials, and
publish order using the evidence table in
[Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md#deferral).
Deferral blocks crates.io publishing; it does not block local release readiness
or internal trial.

When bumping the workspace version, update the internal dependency versions in
the same change; `npm run verify:workspace-dependency-publish-readiness` fails
if they drift.

Do not run `cargo package`, `cargo publish`, create Git tags, create GitHub
releases, or contact registries as part of internal trial.

## crates.io Evidence Boundary

Internal trial does not require crates.io access. Formal crates.io publishing
still requires release-owner evidence for:

- `dioxus-shadcn-core`
- `dioxus-shadcn-primitives`
- `dioxus-shadcn`
- `dioxus-shadcn-cli`

For each crate, record whether the name is available or already owned, who the
crate owner or team is, whether credentials are ready, and whether the publish
order is confirmed as core, primitives, styled crate, then CLI.
