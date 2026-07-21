# API Stability Surface Audit Plan

M123 prepares the API stability discussion for a first publish decision. It
does not freeze APIs, change versions, or approve publishing.

The current publish blocker remains documented in
[API Stability Readiness Metadata](api-stability-readiness-metadata.md). The
workspace is still `0.1.0`, and maintainers still need to decide whether the
crate-mode APIs are acceptable for a first publish.

## Audit Scope

The audit covers these public surfaces:

| Surface | Current Shape | Stability Concern |
| --- | --- | --- |
| Styled component crate | `dioxus-ui` exports feature-gated component modules and re-exports component types, class helpers, constants, and selected primitive config types. | Component names, prop names, variants, feature names, and class helper names become user-facing in crate mode. |
| Primitive crate | `dioxus-ui-primitives` exports pure helper functions, state structs, config structs, runtime adapter traits, and interaction enums. | Helper naming and state shape become reusable app logic contracts. |
| Core crate | `dioxus-ui-core` exports class merging and registry data types. | Registry schema types and helper names affect CLI and integrator tooling. |
| CLI source-copy API | `dxui add`, registry slugs, template filenames, and generated source targets. | Slugs and file paths become source-copy workflow contracts. |
| Feature flags | Component feature names in `crates/dioxus-ui/Cargo.toml`. | Feature names are dependency API and should not drift accidentally. |
| Documentation examples | README, component docs, release docs, and source preview routes. | Examples shape user expectations before the crate API is stable. |

## Baseline Counts

Use existing deterministic gates as the source of truth:

- 64 public component docs and crate modules
- 64 public component crate features
- 65 source-copy templates, including the shared `utils` helper
- 65 registry component/helper entries
- `registry/schema.json` as registry metadata schema, not a public component

If these counts drift, update the audit inventory and the existing docs catalog
gates in the same change.

## Risk Classification

Classify each API surface with one of these labels:

| Risk | Meaning |
| --- | --- |
| `low` | Mostly visual composition or helper naming; source-copy users can edit locally. |
| `medium` | Controlled state, enum variants, feature names, or source-copy target paths likely affect user code. |
| `high` | Primitive behavior, runtime adapter traits, overlay/focus/dismissal contracts, or generated CLI behavior likely affect broad integration code. |
| `deferred` | Stability decision requires maintainer approval before local implementation. |

## Audit Questions

For each public component or helper group, record:

- Is the name aligned with shadcn-style expectations and Rust naming?
- Is the feature flag the same as the registry slug?
- Is the source-copy target stable enough for users to import?
- Are prop names consistent across related components?
- Are variants and sizes complete enough for first publish?
- Are class helper functions intended public API or implementation detail?
- Does the primitive helper expose reusable behavior without runtime coupling?
- Does the component depend on a primitive config whose name should be stable?
- Does documentation present the API as stable or pre-`1.0` provisional?

## Non-goals

This audit must not:

- change workspace or crate versions
- approve `0.1.x` API stability
- rename public APIs
- rewrite component props
- rewrite templates
- generate migration guides
- run `cargo package`
- run `cargo publish`
- contact crates.io
- create tags or release artifacts

## Safe Validation

Run these focused checks while preparing the inventory and checklist:

```bash
npm run verify:api-stability-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
git diff --check
```

Use the full release gate only for final local confidence:

```bash
npm run verify:release
```

The full release gate remains local and does not package, publish, tag, or
contact registries.

## Exit Criteria

M123 is complete when:

- the public API surface inventory is documented
- source-copy and crate-mode surfaces are clearly separated
- high-risk primitive/runtime surfaces are called out
- maintainers have a checklist for deciding whether to stabilize or defer APIs
- existing publish readiness blockers still show API stability as unresolved
- final checks pass with a clean worktree
