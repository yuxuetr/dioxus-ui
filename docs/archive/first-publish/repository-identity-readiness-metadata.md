# Repository Identity Readiness Metadata

M101 defines the read-only contract for repository identity readiness. The
workspace package metadata now uses the approved repository URL
`https://github.com/yuxuetr/dioxus-ui` for first publish preparation.

Use [Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md)
for the repository-safe decision pass that approved the canonical URL.
Use [Repository Identity Decision Record Template](repository-identity-decision-record-template.md)
to record approved, blocked, or deferred maintainer outcomes.
Use [Repository Identity Local Follow-up Map](repository-identity-local-follow-up-map.md)
to map approved decisions to local metadata and documentation updates.
Use [Repository Identity Blocker Handoff](repository-identity-blocker-handoff.md)
for the consolidated first-publish owner, evidence, rollback, and validation
view.

## Expected Shape

The release metadata should distinguish these states:

- Cargo workspace repository metadata exists.
- The repository value is the approved canonical URL.
- Published crate review must keep the approved URL aligned across Cargo
  metadata and release docs.

This gate must not change the URL automatically.

## Scope

In scope:

- approved repository URL discoverability
- publish blocker alignment
- workspace and Cargo publish metadata references
- release and quality gate references
- package script and release aggregate wiring for the read-only check

Out of scope:

- choosing a different repository owner or URL
- changing repository metadata
- checking remote repository existence
- checking crates.io name availability
- running `cargo package`
- running `cargo publish`

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- workspace repository metadata stops being the approved URL
- publish readiness blockers omit the resolved repository identity item
- release docs imply repository identity can change without maintainer review
- package scripts stop running the repository identity readiness gate

This gate should keep approved repository identity explicit after resolution.
