# Internal Trial Developer Guide

This guide explains how to try `dioxus-ui` inside an internal Dioxus project
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
- support for `cargo install dioxus-ui-cli` from crates.io

## Recommended Mode

Use source-copy mode first. It matches the shadcn-style workflow and lets trial
apps edit generated component source locally.

From the `dioxus-ui` repository, list available components:

```bash
cargo run -q -p dioxus-ui-cli -- list
```

Initialize a trial app:

```bash
cargo run -q -p dioxus-ui-cli -- init --root /path/to/trial-app
```

Add components:

```bash
cargo run -q -p dioxus-ui-cli -- add button --root /path/to/trial-app
cargo run -q -p dioxus-ui-cli -- add input --root /path/to/trial-app
cargo run -q -p dioxus-ui-cli -- add dialog --root /path/to/trial-app
```

Use `--overwrite` only when intentionally replacing locally generated files:

```bash
cargo run -q -p dioxus-ui-cli -- add button --root /path/to/trial-app --overwrite
```

Generated files are written under:

```text
assets/dioxus-ui.css
src/components/ui/
```

`assets/dioxus-ui.css` is a Tailwind CSS v4 input stylesheet. It is not a full
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
dioxus-ui = { path = "/path/to/dioxus-ui/crates/dioxus-ui", default-features = false, features = ["button", "input", "dialog"] }
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

Run these in the `dioxus-ui` repository before giving a commit to trial users:

```bash
npm run verify:docs
npm run verify:registry
npm run verify:cli-template-packaging-readiness
scripts/generated-fixture-smoke.sh
cargo test -p dioxus-ui-cli
cargo check -p dioxus-ui-cli
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

The six current publish blockers can be prepared in parallel, but they should
not all be resolved independently without coordination.

Safe to work in parallel:

- repository identity evidence
- license file approval
- API stability decision record
- release notes evidence
- registry availability review
- workspace dependency strategy review

Must be coordinated before removal from blockers:

- repository URL must be final before publish metadata claims readiness
- license files must be reviewed before committing root license text
- API stability decision affects release notes and possible migration notes
- registry availability and workspace dependency strategy affect publish order
- release notes should reflect final blocker outcomes and known warnings

Recommended sequencing:

1. Collect evidence for all six blockers in parallel.
2. Resolve repository identity and license files first.
3. Resolve API stability before final release notes.
4. Resolve registry availability and workspace dependency strategy together.
5. Update publish blockers only after each focused gate and common publish
   readiness checks pass.

Do not run `cargo package`, `cargo publish`, create Git tags, create GitHub
releases, or contact registries as part of internal trial.
