# API Stability Blocker Handoff

This handoff consolidates the pre-`1.0` API stability blocker for the first
publish preparation cycle. It does not approve API stability, rewrite public
APIs, change versions, generate migration guides, or authorize publishing.

Use it with:

- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md)
- [API Stability Decision Record Template](api-stability-decision-record-template.md)
- [API Stability Local Follow-up Map](api-stability-local-follow-up-map.md)
- [API Stability Review Checklist](api-stability-review-checklist.md)
- [Public API Surface Inventory](../../public-api-surface-inventory.md)
- [API Stability Readiness Metadata](api-stability-readiness-metadata.md)

## Current Blocker

| Field | Current Value |
| --- | --- |
| Blocker | Pre-1.0 API stability |
| Workspace version | `0.1.0` |
| Current policy | Breaking API changes remain allowed before `1.0` |
| Publish state | Resolved locally: current `0.1.x` API surface accepted for first publish |
| Resolution owner | Maintainer or release owner |
| Focused gate | `npm run verify:api-stability-readiness` |

## Required Evidence

Local follow-up can start only after a maintainer or release owner records:

- accepted public component names and props, or a required-change worklist
- accepted variant enums, size enums, class helpers, and public constants
- accepted crate feature names
- accepted registry slugs, source-copy filenames, and generated target paths
- accepted source-copy template APIs and any intentional crate-mode differences
- accepted primitive helper names, state structs, config structs, and runtime
  adapter traits
- accepted core exports and registry metadata helper APIs
- breaking-change policy before `1.0`
- migration note and changelog expectations
- known provisional APIs and any deferred stabilization work

Record the decision with
[API Stability Decision Record Template](api-stability-decision-record-template.md).

## Review Inputs

Use these local artifacts during maintainer review:

| Surface | Review Artifact |
| --- | --- |
| Public component API | [Public API Surface Inventory](../../public-api-surface-inventory.md) |
| Naming and prop consistency | [API Stability Review Checklist](api-stability-review-checklist.md) |
| Decision boundary | [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md) |
| Decision output | [API Stability Decision Record Template](api-stability-decision-record-template.md) |
| Local follow-up | [API Stability Local Follow-up Map](api-stability-local-follow-up-map.md) |
| Current readiness gate | [API Stability Readiness Metadata](api-stability-readiness-metadata.md) |

## Local Follow-up After Approval

If the decision is `approved`, update these surfaces together:

| Surface | Follow-up |
| --- | --- |
| `docs/api-stability-readiness-metadata.md` | Move from unresolved pre-`1.0` metadata to approved first-publish API policy metadata |
| `docs/publish-readiness-blockers.md` | Remove pre-`1.0` API stability from current blockers only after focused gates pass |
| `docs/publish-readiness-coverage-metadata.md` | Update coverage from unresolved blocker to resolved readiness item if useful |
| `docs/publish-readiness-resolution-runbook.md` | Update manual evidence and follow-up wording |
| `docs/cargo-publish-metadata.md` | Reflect approved API policy in publish preparation notes |
| `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md` | Replace unresolved API stability wording with approved first-publish policy wording |
| `CHANGELOG.md` or `docs/changelog-metadata.md` | Update only if the approved decision changes release note expectations |
| `TODOs.md` | Mark the approved local follow-up task done only after validation and commit |

If the decision is `blocked` or `deferred`, keep the blocker unresolved and
record the missing review evidence or required API stabilization milestone.

## Validation

Run these checks after any API stability handoff or approved local follow-up
change:

```bash
npm run verify:api-stability-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
npm run verify:repo-hygiene
npm run verify:package-lock
git diff --check
git status --short
```

Run `npm run verify:release` before release-candidate handoff.

## Rollback

If an approved API stability decision changes before first publish:

- revert API policy docs to the last approved decision
- keep the blocker unresolved until replacement evidence is recorded
- create a focused stabilization milestone if public API changes are required
- rerun the focused gate and shared publish readiness checks
- update TODO status only after the rollback commit and validation

## Non-goals

This handoff does not:

- approve API stability without maintainer input
- rename public APIs
- rewrite component props
- rewrite source-copy templates
- change feature names
- change registry slugs or generated paths
- change workspace or crate versions
- generate migration guides
- generate release notes
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts
