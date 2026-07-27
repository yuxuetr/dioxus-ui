# Registry Availability Blocker Handoff

This handoff consolidates the crates.io registry availability blocker for the
first publish preparation cycle. It does not contact crates.io, check crate
name availability, inspect credentials, create package archives, or authorize
publishing.

Use it with:

- [Publish Blocker Resolution Tracker](publish-blocker-resolution-tracker.md)
- [Registry Availability Readiness Metadata](registry-availability-readiness-metadata.md)
- [Publish Order Metadata](publish-order-metadata.md)
- [Cargo Publish Metadata](cargo-publish-metadata.md)
- [Publish Readiness Resolution Runbook](publish-readiness-resolution-runbook.md)
- [Blocker Resolution Evidence Checklist](blocker-resolution-evidence-checklist.md)

## Current Blocker

| Field | Current Value |
| --- | --- |
| Blocker | Registry availability not checked |
| Registry | crates.io |
| Publish state | Unresolved |
| Resolution owner | Release owner |
| Focused gates | `npm run verify:registry-availability-readiness`, `npm run verify:publish-order` |

## Planned Crates

The planned publishable crates are:

```text
dioxus-ui-core
dioxus-ui-primitives
dioxus-ui
dioxus-ui-cli
```

The planned publish order is:

```text
dioxus-ui-core
dioxus-ui-primitives
dioxus-ui
dioxus-ui-cli
```

## Required Evidence

Local follow-up can start only after a release owner records:

- crates.io availability or ownership evidence for each planned crate name
- owner list or team responsible for every planned crate
- credential readiness confirmation
- publish order confirmation
- package owner responsible for the publish sequence
- confirmation that registry checks were performed outside repository-safe
  local metadata gates
- rollback expectation if a crate name or ownership changes before first
  publish

Record the evidence in the publish readiness decision matrix, release-candidate
handoff notes, or another maintainer-approved release-owner artifact.

## Local Follow-up After Approval

If registry availability and ownership are approved, update these surfaces
together:

| Surface | Follow-up |
| --- | --- |
| `docs/registry-availability-readiness-metadata.md` | Move from unchecked registry metadata to release-owner-confirmed registry readiness metadata |
| `docs/publish-order-metadata.md` | Keep planned order aligned with approved crate names and dependency order |
| `docs/cargo-publish-metadata.md` | Reflect registry availability and ownership as approved for publish preparation |
| `docs/publish-readiness-blockers.md` | Remove registry availability from current blockers only after focused gates pass |
| `docs/publish-readiness-coverage-metadata.md` | Update coverage from unresolved blocker to resolved readiness item if useful |
| `docs/publish-readiness-resolution-runbook.md` | Update manual registry evidence and follow-up wording |
| `docs/release.md`, `docs/quality-gates.md`, `README.md`, `docs/site.md` | Replace unresolved-registry wording with approved registry review wording |
| `TODOs.md` | Mark the approved local follow-up task done only after validation and commit |

If the decision is blocked or deferred, keep the blocker unresolved and record
the missing availability, ownership, credential, or publish-order evidence in
TODO planning.

## Validation

Run these checks after any registry availability handoff or approved local
follow-up change:

```bash
npm run verify:registry-availability-readiness
npm run verify:publish-order
npm run verify:cargo-publish-metadata
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

If registry ownership or crate names change before first publish:

- revert registry metadata and publish order docs to the last approved decision
- keep the blocker unresolved until replacement evidence is recorded
- rerun registry availability, publish order, Cargo publish metadata, and shared
  publish readiness checks
- update TODO status only after the rollback commit and validation

## Non-goals

This handoff does not:

- contact crates.io
- check crate name availability
- check registry ownership
- inspect credentials
- run `cargo package`
- run `cargo publish`
- create package archives
- change crate names
- change dependency versions
- authorize a release
