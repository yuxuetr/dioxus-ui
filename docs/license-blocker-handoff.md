# License Blocker Handoff

This handoff consolidates the missing root license files blocker for the first
publish preparation cycle. It does not choose license terms, generate license
text, commit root license files, or authorize publishing.

Use it with:

- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [License Decision Preparation Plan](license-decision-preparation-plan.md)
- [License Decision Record Template](license-decision-record-template.md)
- [License Local Follow-up Map](license-local-follow-up-map.md)
- [License Readiness Metadata](license-readiness-metadata.md)

## Current Blocker

| Field | Current Value |
| --- | --- |
| Blocker | Root license files not committed |
| Workspace license expression | `MIT OR Apache-2.0` |
| Missing root files | `LICENSE-MIT`, `LICENSE-APACHE` |
| Publish state | Unresolved |
| Resolution owner | Maintainer or release owner |
| Focused gate | `npm run verify:license-readiness` |

## Required Evidence

Local follow-up can start only after a maintainer or release owner records:

- approved `LICENSE-MIT` text and evidence location
- approved `LICENSE-APACHE` text and evidence location
- copyright holder text
- confirmation that `MIT OR Apache-2.0` remains the intended workspace license
  expression
- approval to commit the root license files
- confirmation that crate manifests should keep inheriting workspace license
  metadata, or a crate-specific metadata worklist
- accepted release documentation wording after the blocker is resolved
- rollback expectation if license ownership changes before first publish

Record the decision with
[License Decision Record Template](license-decision-record-template.md).

## Local Follow-up After Approval

If the decision is `approved`, update these surfaces together:

| Surface | Follow-up |
| --- | --- |
| `LICENSE-MIT` | Commit the maintainer-approved MIT license text exactly as reviewed |
| `LICENSE-APACHE` | Commit the maintainer-approved Apache-2.0 license text exactly as reviewed |
| `Cargo.toml` | Keep `MIT OR Apache-2.0` unless maintainers explicitly approve a metadata change |
| Crate manifests | Keep `license.workspace = true` unless maintainers approve crate-specific metadata |
| `docs/license-readiness-metadata.md` | Move from missing-file metadata to approved license file metadata |
| `docs/publish-readiness-blockers.md` | Remove missing root license files from current blockers only after focused gates pass |
| `docs/publish-readiness-coverage-metadata.md` | Update coverage from unresolved blocker to resolved readiness item if useful |
| `docs/publish-readiness-resolution-runbook.md` | Update manual evidence and follow-up wording |
| `docs/cargo-publish-metadata.md` | Reflect root license files as approved for publish preparation |
| `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md` | Replace unresolved-license wording with approved-license-file wording |
| `TODOs.md` | Mark the approved local follow-up task done only after validation and commit |

If the decision is `blocked` or `deferred`, keep root license files absent and
record the missing legal, ownership, or review evidence in TODO planning.

## Validation

Run these checks after any license handoff or approved local follow-up change:

```bash
npm run verify:license-readiness
npm run verify:cargo-publish-metadata
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
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

If approved license ownership or terms change before first publish:

- revert root license files and metadata to the last approved license decision
- keep the blocker unresolved until replacement evidence is recorded
- rerun the focused gate and shared publish readiness checks
- update TODO status only after the rollback commit and validation

## Non-goals

This handoff does not:

- choose license terms
- generate license text
- commit root license files without approval
- change copyright holders
- change workspace license metadata
- contact crates.io
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts
