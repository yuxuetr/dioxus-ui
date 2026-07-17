# License Readiness Metadata

M100 defines the read-only contract for license file readiness. The workspace
already declares `MIT OR Apache-2.0`, but the repository does not yet commit the
corresponding root license text files.

## Expected Shape

The release metadata should distinguish these states:

- Cargo workspace license metadata exists.
- Published crate metadata inherits the workspace license expression.
- Root license text files are still a publish-readiness blocker until
  maintainers commit the intended license files.

The expected root files are:

```text
LICENSE-MIT
LICENSE-APACHE
```

## Scope

In scope:

- license expression discoverability
- missing root license file blocker tracking
- publish blocker alignment
- release and quality gate references
- package script and release aggregate wiring for the read-only check

Out of scope:

- choosing or changing project license terms
- generating license text
- changing copyright holders
- running `cargo package`
- publishing crates or GitHub releases
- contacting crates.io

## Expected Check

The verifier should fail when committed metadata drifts. Examples include:

- workspace license stops being documented as `MIT OR Apache-2.0`
- publish readiness blockers omit missing root license files
- release docs imply license file readiness is resolved
- package scripts stop running the license readiness metadata gate

This gate should keep license file readiness explicit without resolving it.
