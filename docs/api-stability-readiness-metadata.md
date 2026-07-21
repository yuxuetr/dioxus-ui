# API Stability Readiness Metadata

M102 defines the read-only contract for API stability readiness. The workspace is still at `0.1.0`, and release docs explicitly allow breaking API changes before `1.0`, so crate-mode APIs are not yet publish-ready stable.
The current audit inventory is tracked in
[Public API Surface Inventory](public-api-surface-inventory.md).
The maintainer review checklist is tracked in
[API Stability Review Checklist](api-stability-review-checklist.md).
The maintainer decision preparation plan is tracked in
[API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md).

## Expected Shape

The release metadata should distinguish these states:

- Crate versions exist and are intentionally pre-`1.0`.
- Breaking API changes remain allowed before `1.0`.
- Publish readiness remains blocked until maintainers decide crate-mode
  stability and versioning policy.

This gate must not freeze APIs or change versions automatically.

## Scope

In scope:

- pre-`1.0` version metadata
- publish blocker alignment
- release and quality gate references
- Cargo publish metadata references
- package script and release aggregate wiring for the read-only check

Out of scope:

- stabilizing component APIs
- changing crate versions
- deciding semantic versioning policy
- generating migration guides
- running `cargo package`
- running `cargo publish`

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- workspace version stops matching the documented pre-`1.0` blocker without
  updating release readiness docs
- publish readiness blockers omit pre-`1.0` API stability
- release docs stop saying breaking changes before `1.0` must be documented
- package scripts stop running the API stability readiness gate

This gate should keep API stability readiness explicit without resolving it.
