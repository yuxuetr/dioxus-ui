# Repository Identity Decision Preparation Plan

This plan prepares the maintainer decision for the placeholder repository URL
publish blocker. It does not choose the repository owner, replace metadata,
check remote repository existence, or authorize publishing.

Use this plan with:

- [Repository Identity Readiness Metadata](repository-identity-readiness-metadata.md)
- [Repository Identity Decision Record Template](repository-identity-decision-record-template.md)
- [Repository Identity Local Follow-up Map](repository-identity-local-follow-up-map.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)

## Decision Question

Maintainers need to decide which canonical repository owner and URL should be
committed to workspace package metadata before first publish.

The decision must cover:

- final repository owner
- canonical repository URL
- confirmation that the URL should replace
  `https://github.com/your-org/dioxus-ui`
- remote availability evidence supplied by the maintainer or release owner
- whether crate manifests keep inheriting workspace repository metadata
- release documentation wording after the blocker is resolved
- rollback expectation if the repository identity changes before publish

## Preparation Pass

Prepare the decision in this order:

1. Confirm workspace metadata still contains the documented placeholder URL.
2. Confirm publish blockers, Cargo publish metadata, release docs, quality
   gates, README, and docs-site notes still describe repository identity as
   unresolved.
3. Record the maintainer decision as `approved`, `blocked`, or `deferred` with
   the repository identity decision record template.
4. Only after approval, update Cargo metadata and blocker docs using the
   repository identity local follow-up map.

## Decision Outcomes

| Outcome | Meaning | Local Follow-up |
| --- | --- | --- |
| `approved` | Maintainers approve a canonical repository owner and URL for first publish | Replace the placeholder URL and update repository identity metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, docs-site notes, and TODO planning |
| `blocked` | Repository identity cannot be decided yet | Keep the placeholder and unresolved blocker metadata, then record the missing evidence |
| `deferred` | First publish needs a separate repository ownership milestone first | Create focused TODO planning without replacing the placeholder URL |

## Non-goals

M129 must not:

- choose the final repository owner
- replace the placeholder repository URL
- check remote repository existence
- check crates.io availability
- run `cargo package`
- run `cargo publish`
- create package archives
- change workspace or crate versions
- generate release notes
- create tags or release artifacts
- activate CI workflows

## Safe Validation

Use read-only checks while preparing decision artifacts:

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

Run `npm run verify:release` only for a final local release-candidate audit.
It still must not package, publish, contact registries, create tags, or create
release artifacts.

## Exit Criteria

This preparation pass is complete when:

- the repository identity decision question is explicit
- the decision record has fields for owner, URL, remote availability, and
  metadata update approval
- possible decision outcomes and local follow-up are documented
- repository identity remains unresolved until a maintainer decision is
  recorded
- no repository URL changes, remote lookups, crates.io contact, package
  archives, publish commands, version changes, generated release notes, tags,
  screenshots, traces, or CI workflow files are committed
