# Release Gate Failure Triage Runbook

This runbook helps maintainers isolate failures from `npm run verify:release`.
It is a manual triage guide, not an automatic repair process.

## Related Documents

- [Release and Package Strategy](release.md)
- [Quality Gates](quality-gates.md)
- [Release Candidate Browser Review Runbook](components/release-candidate-browser-review-runbook.md)

## First Response

When `npm run verify:release` fails:

1. Identify the first failed command segment.
2. Rerun only that focused command.
3. Copy the command, exit status, and key error lines into handoff notes.
4. Inspect the owning source or documentation before editing.
5. Keep fixes scoped to the failing gate.
6. Rerun the focused command before rerunning the full release aggregate.

Do not start by deleting generated directories, changing APIs, rewriting
templates, installing browsers, or publishing packages unless the focused
failure proves that work is required.

## Failure Groups

| Failure Group | First Focused Command | Inspect First | Evidence To Record |
| --- | --- | --- | --- |
| Rust workspace check | `cargo check --workspace --all-features` | `Cargo.toml`, crate source, feature flags | first compiler error and affected crate |
| Rust workspace tests | `cargo test --workspace --all-features` | failing test module and crate feature set | failing test name and assertion |
| CLI registry test | `cargo test -p dioxus-shadcn-cli --test registry` | `registry/`, `templates/`, CLI registry loader | missing registry/template mapping |
| CLI list smoke | `cargo run -p dioxus-shadcn-cli -- list` | CLI command output and registry names | command output and missing component |
| Cargo metadata | `npm run verify:cargo-workspace` | workspace manifests and crate metadata docs | mismatched package field |
| Cargo publish metadata | `npm run verify:cargo-publish-metadata` | crate manifests and workspace dependency versions | stale description or internal dependency version |
| Package contents | `npm run verify:package-contents` | `cargo package --list` output, CLI `build.rs` assets | file missing from a package |
| Cargo lock | `npm run verify:cargo-lock` | `Cargo.lock`, workspace manifests | package metadata drift |
| Pre-commit metadata | `npm run verify:pre-commit` | `.pre-commit-config.yaml` and local hook config | missing hook or target |
| Script metadata | `npm run verify:scripts` | `scripts/`, `package.json` | missing executable bit or script target |
| Gitignore metadata | `npm run verify:gitignore` | `.gitignore`, repo hygiene policy | missing ignore pattern |
| Preview metadata | `npm run verify:preview-state-metadata` | preview state source and preview docs | missing preview selector or route |
| Mobile browser metadata | `npm run verify:mobile-browser-metadata` | mobile browser smoke script and CI guide | missing viewport, selector, or env var |
| Rendered coverage | `npm run verify:rendered-component-coverage` | component docs catalog and preview inventory | missing public component preview id |
| Examples metadata | `npm run verify:examples-metadata` | example manifests and examples README | missing example wiring |
| CSS input metadata | `npm run verify:css-inputs` | Tailwind CSS v4 input files | stale v3 directive or missing source root |
| Registry metadata | `npm run verify:registry` | `crates/dioxus-shadcn-cli/registry/` and `crates/dioxus-shadcn-cli/templates/` | invalid source or target path |
| Tailwind static tokens | `npm run verify:tailwind-static` | Rust source and templates | dynamic utility construction, bare data variants, palette colors outside the RFC 0051 tokens |
| Tailwind utility conflicts | `npm run verify:tailwind-conflicts` | Rust source and templates | a base class and a state class setting the same property |
| Preview stylesheet drift | `npm run verify:preview-css` | `examples/preview-states/assets/preview.generated.css` | a class changed without `npm run css:preview` |
| Default local gate | `npm run verify` | smoke and docs output | nested failing command |
| Changelog metadata | `npm run verify:changelog` | `CHANGELOG.md` and changelog metadata docs | missing Unreleased or stale link |
| Source-copy fixture | `scripts/generated-fixture-smoke.sh` | CLI template output and generated fixture logs | generated file path and compiler error |
| Component feature checks | `scripts/feature-check.sh` | crate feature list and public modules | failing feature name |
| Release docs | `npm run verify:release-docs` | `docs/release.md`, release docs verifier | missing release command or boundary text |
| Package scripts | `npm run verify:package-scripts` | `package.json`, `scripts/` | missing alias or local target |
| Package lock | `npm run verify:package-lock` | `package.json`, `package-lock.json` | root metadata mismatch |
| CI docs | `npm run verify:ci-docs` | CI browser guide and workflow template | missing opt-in boundary |
| CI plan | `npm run verify:ci-plan` | CI plan docs and RFC references | missing activation policy |
| CI workflow template | `npm run verify:ci-workflow-template` | workflow template and RFC 0009 | unsafe workflow activation or broad upload |
| Browser artifact policy | `npm run verify:browser-artifact-policy` | artifact policy docs, `.gitignore`, repo hygiene | upload or retention policy drift |
| Repository hygiene | `npm run verify:repo-hygiene` | tracked files and hygiene script | forbidden tracked artifact path |

## Evidence Template

Copy this into the release candidate handoff notes when a release gate fails:

```text
Failed command:
Exit status:
First failing line:
Focused rerun command:
Focused rerun result:
Likely owner:
Files inspected:
Decision:
Follow-up:
```

## Triage Order

Prefer this order:

1. Fix the first failing command, not every suspicious warning.
2. Rerun the focused command.
3. Rerun dependent metadata checks if the fix touched docs or scripts.
4. Rerun `npm run verify:release` only after focused checks pass.
5. Update handoff notes with the final command evidence.

## Unsafe Shortcuts

Avoid these unless explicitly required and reviewed:

- deleting tracked files to satisfy repository hygiene
- committing generated screenshots or traces
- committing generated docs output
- activating `.github/workflows/browser-smoke.yml`
- changing public component APIs to satisfy docs metadata
- rewriting source-copy templates to silence unrelated failures
- creating Git tags
- creating release artifacts
- running `cargo package`
- running `cargo publish`

## Non-goals

- no automatic repair command
- no destructive cleanup by default
- no browser launch
- no screenshot capture
- no CI workflow activation
- no package publishing
- no release tagging
- no generated docs output
- no component API changes unless a focused failure requires them
- no source-copy template rewrites unless a focused failure requires them
