# License Readiness Metadata

M100 defines the read-only contract for license file readiness. The workspace
declares `MIT`, and the repository commits reviewed root MIT license text in
`LICENSE`.

Use [License Decision Preparation Plan](license-decision-preparation-plan.md)
for the repository-safe decision pass that approved MIT.
Use [License Decision Record Template](license-decision-record-template.md)
to record approved, blocked, or deferred maintainer outcomes.
Use [License Local Follow-up Map](license-local-follow-up-map.md)
to map approved decisions to local license file, metadata, and documentation
updates.
Use [License Blocker Handoff](license-blocker-handoff.md) for the consolidated
first-publish license evidence, rollback, and validation view.

## Expected Shape

The release metadata should distinguish these states:

- Cargo workspace license metadata exists.
- Published crate metadata inherits the workspace license expression.
- Root MIT license text is committed in `LICENSE`.

The expected root file is:

```text
LICENSE
```

## Scope

In scope:

- license expression discoverability
- committed root MIT license file tracking
- publish blocker alignment
- release and quality gate references
- package script and release aggregate wiring for the read-only check

Out of scope:

- choosing or changing project license terms
- generating replacement license text
- changing copyright holders
- running `cargo package`
- publishing crates or GitHub releases
- contacting crates.io

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- workspace license stops being documented as `MIT`
- root `LICENSE` is missing
- release docs imply license metadata can change without maintainer review
- package scripts stop running the license readiness metadata gate

This gate should keep approved MIT license readiness explicit after resolution.
