# License Decision Preparation Plan

This plan prepares the maintainer decision for the missing root license file
publish blocker. It does not generate license text, commit root license files,
change license terms, or authorize publishing.

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

Maintainers need to decide whether the declared `MIT OR Apache-2.0` workspace
license expression should be backed by reviewed root license files before first
publish.

The decision must cover:

- approved `LICENSE-MIT` text
- approved `LICENSE-APACHE` text
- copyright holder text
- confirmation that `MIT OR Apache-2.0` remains the intended workspace license
  expression
- confirmation that root license files may be committed
- whether crate manifests keep inheriting workspace license metadata
- release documentation wording after the blocker is resolved
- rollback expectation if license ownership changes before publish

## Preparation Pass

Prepare the decision in this order:

1. Confirm workspace metadata still declares `MIT OR Apache-2.0`.
2. Confirm root `LICENSE-MIT` and `LICENSE-APACHE` files are still absent.
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
| `approved` | Maintainers approve root license files, copyright holder text, and the current workspace license expression for first publish | Commit approved license files and update license readiness metadata, publish blockers, Cargo publish metadata, release docs, quality gates, README, docs-site notes, and TODO planning |
| `blocked` | License file readiness cannot be decided yet | Keep root license files absent and unresolved blocker metadata, then record the missing evidence |
| `deferred` | First publish needs a separate legal or ownership milestone first | Create focused TODO planning without generating or committing license files |

## Non-goals

M130 must not:

- choose license terms
- generate license text
- commit `LICENSE-MIT`
- commit `LICENSE-APACHE`
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
