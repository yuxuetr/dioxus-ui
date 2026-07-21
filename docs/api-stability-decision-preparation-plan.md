# API Stability Decision Preparation Plan

This plan prepares the maintainer decision for the pre-`1.0` API stability
publish blocker. It does not approve stability, rewrite APIs, change versions,
or authorize publishing.

Use this plan with:

- [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
- [API Stability Surface Audit Plan](api-stability-surface-audit-plan.md)
- [Public API Surface Inventory](public-api-surface-inventory.md)
- [API Stability Review Checklist](api-stability-review-checklist.md)
- [API Stability Decision Record Template](api-stability-decision-record-template.md)
- [API Stability Local Follow-up Map](api-stability-local-follow-up-map.md)
- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)

## Decision Question

Maintainers need to decide whether the current `0.1.x` crate-mode APIs are
acceptable for first publish, or whether an API stabilization milestone must
run before publishing.

The decision must cover:

- public component names and props
- public component feature names
- registry slugs and generated source-copy target paths
- source-copy template APIs
- primitive helper names, state structs, config structs, and runtime adapter
  traits
- core crate exports and registry metadata types
- breaking-change policy before `1.0`
- migration note expectations for any required changes

## Preparation Pass

Prepare the decision in this order:

1. Confirm the public API surface inventory still matches registry, template,
   crate module, and feature metadata.
2. Review component names, prop names, variants, sizes, class helpers, and
   constants against the API stability review checklist.
3. Review feature names, registry slugs, source-copy filenames, generated
   target paths, and docs examples as compatibility surfaces.
4. Review primitive and core exports for helper names that should remain
   provisional versus acceptable for first publish.
5. Record the maintainer decision as `approved`, `blocked`, or `deferred` with
   the API stability decision record template.
6. Only after approval, update blocker metadata and local follow-up files using
   the API stability local follow-up map.

## Decision Outcomes

| Outcome | Meaning | Local Follow-up |
| --- | --- | --- |
| `approved` | Current `0.1.x` APIs are acceptable for first publish with documented pre-`1.0` breaking-change policy | Update API stability metadata, publish blockers, release docs, quality gates, README, docs-site notes, and TODO planning |
| `blocked` | API stability cannot be decided yet | Keep blocker unresolved and record missing review evidence |
| `deferred` | First publish needs a focused API stabilization milestone first | Create a new TODO milestone for required API changes without changing versions or publishing |

## Non-goals

M126 must not:

- approve API stability by itself
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

## Safe Validation

Use read-only checks while preparing decision artifacts:

```bash
npm run verify:api-stability-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
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

- the API stability decision question is explicit
- every reviewed surface has an expected decision record location
- possible decision outcomes and local follow-up are documented
- API stability remains unresolved until a maintainer decision is recorded
- no API rewrites, version changes, migration guides, package archives, publish
  commands, generated release notes, tags, screenshots, traces, or CI workflow
  files are committed
