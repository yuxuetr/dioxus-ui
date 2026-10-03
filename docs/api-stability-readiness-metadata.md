# API Stability Readiness Metadata

M102 defines the read-only contract for API stability readiness. The workspace
is at `0.1.0`, and M133 records the maintainer decision that the current
`0.1.x` API surface is accepted for first publish under a pre-`1.0`
breaking-change policy.
The current audit inventory is tracked in
[Public API Surface Inventory](public-api-surface-inventory.md).
The maintainer review checklist is tracked in
[API Stability Review Checklist](api-stability-review-checklist.md).
The maintainer decision preparation plan is tracked in
[API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md).
The copyable decision record template is tracked in
[API Stability Decision Record Template](api-stability-decision-record-template.md).
The local follow-up map is tracked in
[API Stability Local Follow-up Map](api-stability-local-follow-up-map.md).
The consolidated first-publish evidence and rollback view is tracked in
[API Stability Blocker Handoff](api-stability-blocker-handoff.md).

## Accepted Policy

| Field | Value |
| --- | --- |
| Workspace version | `0.1.0` |
| Decision | Current `0.1.x` API surface is accepted for first publish |
| Decision source | [Approved Publish Blocker Resolution Plan](approved-publish-blocker-resolution-plan.md) |
| Patch releases (`0.1.x`) | Must not break public crate-mode APIs |
| Breaking changes | Allowed before `1.0` only in a minor bump (`0.1` to `0.2`) |
| Changelog | Every breaking change is documented in `CHANGELOG.md` with a migration note |

Cargo already treats `0.x` minor bumps as incompatible, so `^0.1` consumers do
not receive `0.2` automatically. Source-copy templates are owned by the
consuming app after generation and are not covered by this compatibility
promise.

## Expected Shape

The release metadata should distinguish these states:

- Crate versions exist and are intentionally pre-`1.0`.
- Breaking API changes remain allowed before `1.0`.
- The current `0.1.x` API surface is accepted for first publish.

This gate must not freeze APIs or change versions automatically.

## Scope

In scope:

- pre-`1.0` version metadata
- accepted first-publish API policy discoverability
- publish blocker alignment
- release and quality gate references
- Cargo publish metadata references
- package script and release aggregate wiring for the read-only check

Out of scope:

- stabilizing component APIs
- changing crate versions
- generating migration guides
- running `cargo package`
- running `cargo publish`

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- workspace version stops matching the documented `0.1.x` policy without
  updating release readiness docs
- publish readiness blockers stop recording the accepted API policy as resolved
- release docs stop saying breaking changes before `1.0` must be documented
- package scripts stop running the API stability readiness gate

This gate should keep the accepted `0.1.x` API policy explicit after
resolution.
