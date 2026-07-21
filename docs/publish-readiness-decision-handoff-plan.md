# Publish Readiness Decision Handoff Plan

M122 turns the known publish readiness blockers into a maintainer decision
handoff. It is a planning artifact, not a publish authorization.

The repository has already passed the local release gate in
[Full Release Gate Audit](full-release-gate-audit.md). Passing that gate proves
the current workspace is internally consistent; it does not resolve the
publish blockers tracked in
[Publish Readiness Blockers](publish-readiness-blockers.md).
The detailed maintainer evidence table is tracked in
[Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md).

## Decision Boundary

This milestone may document, classify, and validate blocker metadata. It must
not:

- replace the placeholder repository URL
- choose a repository owner
- generate license text
- choose copyright holders
- decide API stability policy
- generate final release notes
- embed or package CLI templates
- contact crates.io
- inspect credentials
- run `cargo package`
- run `cargo publish`
- create package archives
- create Git tags
- publish GitHub releases
- add CI workflow activation

## Blocker Classification

| Blocker | Decision Type | Maintainer Input Required | Local Follow-up After Decision |
| --- | --- | --- | --- |
| Placeholder repository URL | Identity decision | Final repository owner and URL | Update workspace metadata, repository identity metadata, publish blockers, release docs, quality gates, README, docs-site notes, and TODO planning |
| Root license files not committed | Legal/project ownership decision | Approved `LICENSE-MIT` and `LICENSE-APACHE` text and copyright holder | Commit license files and update license readiness metadata, blocker inventory, release docs, quality gates, README, docs-site notes, and TODO planning |
| Pre-1.0 API stability | Product/API decision | Whether `0.1.x` APIs are acceptable for first publish or require a stabilization pass | Update API stability metadata, release docs, changelog guidance, quality gates, README, docs-site notes, and TODO planning |
| Release notes not publish-ready | Release management decision | First publish release notes and historical scope | Update changelog, release notes readiness metadata, release docs, quality gates, README, docs-site notes, and TODO planning |
| CLI template packaging strategy | Implementation decision | Compile-time embedding or stable install-location package strategy | Implement selected template delivery path and update CLI packaging metadata, blocker inventory, release docs, quality gates, README, docs-site notes, and TODO planning |
| Registry availability not checked | Registry ownership decision | crates.io names, ownership, credentials, and publish order confirmation | Update registry availability metadata, Cargo publish metadata, publish blockers, release docs, quality gates, README, docs-site notes, and TODO planning |
| Workspace dependency publish readiness | Packaging/dependency decision | Internal crate dependency version metadata policy for crates.io-resolvable packages | Update crate manifests, workspace dependency metadata, publish order metadata, Cargo publish metadata, release docs, quality gates, README, docs-site notes, and TODO planning |

## Safe Local Validation

Before and after any handoff documentation change, run the focused metadata
checks that do not publish, package, contact registries, or launch browsers:

```bash
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:publish-readiness-runbook
npm run verify:cargo-publish-metadata
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs-links
npm run verify:docs-anchors
git diff --check
```

For a full local release-candidate check, use:

```bash
npm run verify:release
```

The release aggregate remains local and repository-safe. It still does not run
`cargo package`, run `cargo publish`, contact crates.io, create Git tags, or
publish artifacts.

## M122 Output

M122 should produce:

1. A decision matrix that maintainers can use before resolving blockers.
2. A copyable first-publish handoff checklist.
3. Documentation links from the existing publish readiness, release, quality
   gate, README, and docs-site surfaces.
4. Final evidence that the handoff documents are discoverable and that no
   blocker was accidentally marked as resolved.

## Exit Criteria

M122 is complete when:

- every current publish blocker is represented in the decision matrix
- every blocker has a required maintainer input
- every blocker has local follow-up files listed
- every blocker has safe validation commands listed
- the handoff template is linked from the publish readiness runbook
- final checks pass
- the worktree is clean
