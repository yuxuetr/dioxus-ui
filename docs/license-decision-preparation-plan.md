# License Decision Preparation Plan

This plan records the maintainer decision for the resolved MIT license
readiness item. It does not generate replacement license text, change license
terms, or authorize publishing.

Use this plan with:

- [License Readiness Metadata](license-readiness-metadata.md)
- [License Decision Record Template](license-decision-record-template.md)
- [License Local Follow-up Map](license-local-follow-up-map.md)
- [License Blocker Handoff](license-blocker-handoff.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)

## Decision Question

Maintainers approved the `MIT` workspace license expression and reviewed root
`LICENSE` text before first publish.

The decision must cover:

- approved `LICENSE` text
- copyright holder text
- confirmation that `MIT` remains the intended workspace license
  expression
- confirmation that root license files may be committed
- whether crate manifests keep inheriting workspace license metadata
- release documentation wording after the blocker is resolved
- rollback expectation if license ownership changes before publish

## Preparation Pass

Prepare the decision in this order:

1. Confirm workspace metadata still declares `MIT`.
2. Confirm root `LICENSE` is committed.
3. Confirm publish blockers, Cargo publish metadata, release docs, quality
   gates, README, and docs-site notes still describe license readiness as
   unresolved.
4. Record the maintainer decision as `approved`, `blocked`, or `deferred` with
   the license decision record template.
5. Only after approval, commit reviewed license files and update blocker docs
   using the license local follow-up map.

## Decision Outcomes

| Outcome | Meaning | Local Follow-up |
| --- | --- | --- |
| `approved` | Maintainers approve root `LICENSE`, copyright holder text, and the current workspace license expression for first publish | Update license readiness metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, docs-site notes, and TODO planning |
| `blocked` | License file readiness cannot be decided yet | Keep root license files absent and unresolved blocker metadata, then record the missing evidence |
| `deferred` | First publish needs a separate legal or ownership milestone first | Create focused TODO planning without generating or committing license files |

## Non-goals

M130 must not:

- choose license terms
- generate replacement license text
- change copyright holders
- change workspace or crate license metadata
- run `cargo package`
- run `cargo publish`
- contact crates.io
- create package archives
- generate release notes
- create tags or release artifacts
- activate CI workflows

## Safe Validation

Use read-only checks while preparing decision artifacts:

```bash
npm run verify:license-readiness
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

- the license file decision question is explicit
- the decision record has fields for license file text, copyright holder,
  expression confirmation, and file commit approval
- possible decision outcomes and local follow-up are documented
- license file readiness remains unresolved until a maintainer decision is
  recorded
- no license files, license text generation, license expression changes,
  copyright holder changes, package archives, publish commands, crates.io
  contact, generated release notes, tags, screenshots, traces, or CI workflow
  files are committed
