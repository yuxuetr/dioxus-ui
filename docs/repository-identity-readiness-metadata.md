# Repository Identity Readiness Metadata

M101 defines the read-only contract for repository identity readiness. The
workspace package metadata still uses the placeholder repository URL
`https://github.com/your-org/dioxus-ui`, so the project is not ready to publish
until maintainers choose the final repository identity.

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
