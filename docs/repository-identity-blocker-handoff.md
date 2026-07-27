# Repository Identity Blocker Handoff

This handoff consolidates the placeholder repository URL blocker for the first
publish preparation cycle. It does not choose the repository owner, replace the
placeholder URL, check remote availability, or authorize publishing.

Use it with:

- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md)
- [Repository Identity Decision Record Template](repository-identity-decision-record-template.md)
- [Repository Identity Local Follow-up Map](repository-identity-local-follow-up-map.md)
- [Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md)

## Current Blocker

| Field | Current Value |
| --- | --- |
| Blocker | Placeholder repository URL |
| Current metadata | `https://github.com/your-org/dioxus-ui` |
| Publish state | Unresolved |
| Resolution owner | Maintainer or release owner |
| Focused gate | `npm run verify:repository-identity-readiness` |

## Required Evidence

Local follow-up can start only after a maintainer or release owner records:

- final repository owner
- canonical repository URL
- remote availability evidence
- approval to replace workspace package metadata
- confirmation that crate manifests should keep inheriting workspace repository
  metadata, or a crate-specific metadata worklist
- accepted release documentation wording after the blocker is resolved
- rollback expectation if repository identity changes before first publish

Record the decision with
[Repository Identity Decision Record Template](repository-identity-decision-record-template.md).

## Local Follow-up After Approval

If the decision is `approved`, update these surfaces together:

| Surface | Follow-up |
| --- | --- |
| `Cargo.toml` | Replace the placeholder repository URL with the approved canonical URL |
| Crate manifests | Keep `repository.workspace = true` unless maintainers approve crate-specific URLs |
| `docs/repository-identity-readiness-metadata.md` | Move from placeholder metadata to approved identity metadata |
| `docs/publish-readiness-blockers.md` | Remove the placeholder URL from current blockers only after focused gates pass |
| `docs/publish-readiness-coverage-metadata.md` | Update coverage from unresolved blocker to resolved readiness item if useful |
| `docs/publish-readiness-resolution-runbook.md` | Update manual evidence and follow-up wording |
| `docs/cargo-publish-metadata.md` | Reflect repository metadata as approved for publish preparation |
| `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md` | Replace unresolved-placeholder wording with approved-identity wording |
| `TODOs.md` | Mark the approved local follow-up task done only after validation and commit |

If the decision is `blocked` or `deferred`, keep the placeholder URL and record
the missing ownership or remote evidence in TODO planning.

## Validation

Run these checks after any repository identity handoff or approved local
follow-up change:

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
git status --short
```

Run `npm run verify:publish-readiness-runbook` when the runbook wording changes.

## Rollback

If approved repository identity changes before first publish:

- revert metadata and docs to the last approved repository identity
- keep the blocker unresolved until replacement evidence is recorded
- rerun the focused gate and shared publish readiness checks
- update TODO status only after the rollback commit and validation

## Non-goals

This handoff does not:

- choose repository ownership
- replace repository metadata
- check remote repository existence
- check crates.io availability
- change package versions
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts
