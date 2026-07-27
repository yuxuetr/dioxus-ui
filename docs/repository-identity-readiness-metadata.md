# Repository Identity Readiness Metadata

M101 defines the read-only contract for repository identity readiness. The
workspace package metadata still uses the placeholder repository URL
`https://github.com/your-org/dioxus-ui`, so the project is not ready to publish
until maintainers choose the final repository identity.

Use [Repository Identity Decision Preparation Plan](repository-identity-decision-preparation-plan.md)
for the repository-safe decision pass before replacing the placeholder URL.
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
- The repository value is intentionally still a placeholder.
- Published crate review remains blocked until maintainers replace the
  placeholder with the final repository URL.

This gate must not replace the URL automatically.

## Scope

In scope:

- placeholder repository URL discoverability
- publish blocker alignment
- workspace and Cargo publish metadata references
- release and quality gate references
- package script and release aggregate wiring for the read-only check

Out of scope:

- choosing the final repository owner or URL
- replacing repository metadata
- checking remote repository existence
- checking crates.io name availability
- running `cargo package`
- running `cargo publish`

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- workspace repository metadata stops being the documented placeholder before
  the blocker is resolved
- publish readiness blockers omit the placeholder repository blocker
- release docs imply repository identity readiness is resolved
- package scripts stop running the repository identity readiness gate

This gate should keep repository identity readiness explicit without resolving it.
