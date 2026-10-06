# Release Warning Inventory Metadata

M95 defines the read-only contract for known release warnings. The goal is to
make recurring release output discoverable without changing dependency state or
teaching release verification to parse Cargo output.

Current known warning:

| Package | Version | Warning Type | Observed During |
| --- | --- | --- | --- |
| `block` | `0.1.6` | Rust future-incompatibility warning | `cargo check --workspace --all-features` and `cargo test --workspace --all-features` inside `npm run verify:release` |

The release aggregate still passes with this warning. The inventory exists so
maintainers can distinguish a known upstream dependency warning from a new
project warning.

## Scope

The metadata gate should verify committed source and documentation only.

In scope:

- `Cargo.lock` still records `block` `0.1.6`
- release docs mention the known future-incompatibility warning
- quality gates explain the read-only warning inventory check
- README exposes the focused verification alias
- docs-site results record the current warning boundary
- package scripts wire the check into `npm run verify:release`

Out of scope:

- running Cargo commands
- parsing live compiler output
- executing `cargo report future-incompatibilities`
- upgrading, patching, or removing dependencies
- suppressing warnings
- contacting crates.io or upstream repositories

## Expected Check

The verifier should fail when committed warning inventory metadata drifts.
Examples include:

- `Cargo.lock` no longer contains the documented package and version
- release docs omit the warning type or observed release commands
- quality gates imply the check proves Cargo is warning-free
- package scripts stop running the inventory metadata gate during release

If the warning disappears because dependencies are upgraded, update this
inventory and the verifier in the same change. The gate should make that cleanup
explicit instead of silently leaving stale release notes behind.
