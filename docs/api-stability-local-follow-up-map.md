# API Stability Local Follow-up Map

This map describes local follow-up after maintainers record an API stability
decision. It is hypothetical until a decision record is committed.

Use this map with:

- [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md)
- [API Stability Decision Record Template](api-stability-decision-record-template.md)
- [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
- [Public API Surface Inventory](public-api-surface-inventory.md)
- [API Stability Review Checklist](api-stability-review-checklist.md)

## Outcome Map

| Decision | Local Files | Validation | Notes |
| --- | --- | --- | --- |
| `approved` | `docs/api-stability-readiness-metadata.md`, `docs/publish-readiness-blockers.md`, `docs/publish-readiness-coverage-metadata.md`, `docs/publish-readiness-resolution-runbook.md`, `docs/cargo-publish-metadata.md`, `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md`, `TODOs.md` | `npm run verify:api-stability-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:publish-readiness-coverage`; `npm run verify:publish-readiness-runbook`; `npm run verify:release-docs`; `npm run verify:docs` | Move API stability out of current blockers only after focused gates and docs agree |
| `blocked` | Decision notes or TODO planning only | `npm run verify:api-stability-readiness`; `npm run verify:publish-readiness-blockers`; `npm run verify:docs` | Keep blocker unresolved and record missing evidence |
| `deferred` | `TODOs.md`, API stabilization planning docs, optional component/primitive audit docs | `npm run verify:api-stability-readiness`; `npm run verify:docs`; `git diff --check` | Create a focused stabilization milestone before resolving publish readiness |

## Surface-specific Follow-up

| Surface | Possible Approved Follow-up | Possible Deferred Follow-up |
| --- | --- | --- |
| Component names and props | Document accepted names and keep examples aligned | Create component API rename or prop consistency milestone |
| Feature names | Keep current feature names as first-publish dependency API | Plan feature rename before publish because post-publish rename is breaking |
| Registry slugs and source-copy paths | Keep slugs and generated paths as source-copy compatibility surface | Plan registry slug or target path changes before publish |
| Source-copy templates | Document intentional differences from crate-mode APIs | Plan template API alignment without changing published crate versions |
| Primitive helpers | Document provisional helper groups and accepted runtime adapter boundaries | Plan primitive API cleanup before first publish |
| Core exports | Keep class and registry metadata helpers as tooling API | Plan registry/core helper rename before first publish |
| Documentation examples | Keep examples as current `0.1.x` usage guidance | Update examples after API changes, before release notes readiness |

## Common Validation

After any local follow-up, run:

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

## Non-goals

This map does not:

- apply API changes
- approve stability
- change versions
- generate migration guides
- generate release notes
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags or release artifacts
