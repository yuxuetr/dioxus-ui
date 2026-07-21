# Blocker Resolution Evidence Checklist

This checklist records the minimum evidence required before a first-publish
blocker can move from planning to local implementation follow-up. It is not a
publish authorization and does not resolve blockers by itself.

Use it with:

- [First Publish Readiness Plan](first-publish-readiness-plan.md)
- [Publish Readiness Blockers](publish-readiness-blockers.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- [First Publish Local Implementation Map](first-publish-local-implementation-map.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [First Publish Maintainer Handoff Template](first-publish-maintainer-handoff-template.md)

## Evidence Rules

- Evidence must come from a maintainer or release owner, not inference.
- Local follow-up starts only after the relevant evidence is recorded.
- Blocker docs stay unresolved until implementation, focused gates, release
  docs, publish readiness coverage, and TODO status are all updated.
- A resolved blocker must be removed from the current blocker list and, when
  useful, added to resolved readiness items with an ongoing verification gate.

## Checklist

| Blocker | Required Evidence Before Local Follow-up | Focused Gate |
| --- | --- | --- |
| Placeholder repository URL | Final canonical repository URL, repository owner, and confirmation that workspace metadata should use that URL | `npm run verify:repository-identity-readiness` |
| Root license files not committed | Approved `LICENSE-MIT`, approved `LICENSE-APACHE`, copyright holder text, and confirmation that files may be committed | `npm run verify:license-readiness` |
| Pre-1.0 API stability | Decision to publish `0.1.x` APIs as-is or stabilization worklist, breaking-change policy, migration note expectation, and API stability decision record | `npm run verify:api-stability-readiness` |
| Release notes not publish-ready | Approved first-publish release notes scope, included changes, excluded changes, known warnings, changelog owner, and release notes evidence checklist | `npm run verify:release-notes-readiness` |
| Registry availability not checked | crates.io crate names, owner list, credential readiness, publish order confirmation, and release owner | `npm run verify:registry-availability-readiness` |
| Workspace dependency publish readiness | Internal crate version policy, crates.io-resolvable dependency plan, publish order dependency review, and package owner | `npm run verify:workspace-dependency-publish-readiness` |

Resolved item to keep verified:

| Item | Evidence | Ongoing Gate |
| --- | --- | --- |
| CLI template packaging strategy | Compile-time embedded registry and template assets | `npm run verify:cli-template-packaging-readiness` |

## Validation Commands

After a blocker has approved evidence and local follow-up is implemented, run
the focused gate plus the common publish readiness checks:

```bash
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

Run `npm run verify:release` before release-candidate handoff, not as proof
that publishing is authorized.

## Non-goals

This checklist does not:

- choose repository identity
- write license text
- decide API stability policy
- generate release notes
- contact crates.io
- inspect credentials
- change dependency versions
- run `cargo package`
- run `cargo publish`
- create package archives
- create tags
- activate CI workflows
