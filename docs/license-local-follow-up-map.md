# License Local Follow-up Map

This map describes local follow-up after maintainers record a root license file
decision. It is hypothetical until a decision record is committed.

Use this map with:

- [License Decision Preparation Plan](license-decision-preparation-plan.md)
- [License Decision Record Template](license-decision-record-template.md)
- [License Readiness Metadata](license-readiness-metadata.md)
- [License Blocker Handoff](license-blocker-handoff.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)

## Outcome Map

| Decision | Local Files | Validation | Notes |
| --- | --- | --- | --- |
| `approved` | `LICENSE-MIT`, `LICENSE-APACHE`, `Cargo.toml` only if the license expression changes by explicit approval, crate manifests if they stop inheriting workspace metadata, `docs/license-readiness-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/publish-readiness-coverage-metadata.md`, `docs/publish-readiness-resolution-runbook.md`, `docs/cargo-publish-metadata.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:license-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:publish-readiness-coverage`; `npm run verify:cargo-publish-metadata`; `npm run verify:release-docs`; `npm run verify:docs` | Move license readiness out of current blockers only after approved files, Cargo metadata, and docs agree |
| `blocked` | Decision notes or TODO planning only | `npm run verify:license-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:docs` | Keep root license files absent and record missing legal or ownership evidence |
| `deferred` | `TODOs.md`, legal or ownership planning docs, optional release ownership notes | `npm run verify:license-readiness`; `npm run verify:docs`; `git diff --check` | Create a focused license ownership milestone before resolving publish readiness |

## Surface-specific Follow-up

| Surface | Possible Approved Follow-up | Possible Deferred Follow-up |
| --- | --- | --- |
| Root license files | Commit approved `LICENSE-MIT` and `LICENSE-APACHE` exactly as reviewed | Keep files absent and plan evidence collection |
| Workspace license metadata | Keep `MIT OR Apache-2.0` unless maintainers explicitly approve a metadata change | Keep metadata unchanged and unresolved |
| Crate manifests | Keep `license.workspace = true` unless maintainers approve crate-specific metadata | Plan crate-specific license review before publish |
| Cargo publish metadata | Move license readiness from unresolved blocker to verified publish metadata | Keep publish metadata marked not ready |
| Publish blocker docs | Remove missing root license files from current blockers only after focused gates pass | Keep blocker unresolved and document missing evidence |
| Release docs and quality gates | Document approved license files and ongoing validation command | Keep read-only missing-license-file validation wording |
| README and docs-site notes | Update command descriptions so they no longer imply unresolved license files | Link the deferred license ownership milestone |

## Common Validation

After any local follow-up, run:

```bash
npm run verify:license-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:cargo-publish-metadata
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
git status --short
```

## Non-goals

This map does not:

- generate license text
- commit license files
- approve license terms
- change copyright holders
- change workspace license metadata
- contact crates.io
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts

In short, it does not commit license files without maintainer approval.
