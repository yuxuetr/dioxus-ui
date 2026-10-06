# API Stability Review Checklist

Use this checklist before resolving the pre-`1.0` API stability publish
blocker. It is a maintainer review aid, not an API stabilization approval.

Related documents:

- [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
- [API Stability Surface Audit Plan](api-stability-surface-audit-plan.md)
- [API Stability Decision Preparation Plan](api-stability-decision-preparation-plan.md)
- [API Stability Decision Record Template](api-stability-decision-record-template.md)
- [API Stability Local Follow-up Map](api-stability-local-follow-up-map.md)
- [API Stability Blocker Handoff](api-stability-blocker-handoff.md)
- [Public API Surface Inventory](../../public-api-surface-inventory.md)
- [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)

## Review Metadata

- Review date:
- Reviewer:
- Candidate commit:
- Decision: `blocked` / `approved` / `deferred`
- Follow-up owner:

## Component Naming

- Component type names are consistent with docs titles.
- Component module names match Rust snake_case conventions.
- Registry slugs match shadcn-style kebab-case names.
- Feature names match registry slugs.
- Source-copy filenames match Rust module names.
- Component docs use the same names as crate exports.

Notes:

## Props And Variants

- Common props use consistent names across components.
- Boolean state props use predictable names such as `disabled`, `selected`,
  `open`, `active`, `invalid`, or `pressed`.
- Size enums are consistent across related components.
- Variant enums are complete enough for first publish.
- Controlled state props are documented as app-owned.
- `children` usage is consistent for composition parts.

Notes:

## Class Helpers And Constants

- Public `*_class` helpers are intentionally public.
- Base class constants are intentionally public.
- Tailwind class tokens remain static and searchable.
- Helper names match component and part names.
- Public class helpers do not hide runtime-only behavior.

Notes:

## Primitive Helpers

- Primitive helper names describe behavior, not styling.
- Pure helper functions stay runtime-independent.
- Overlay, focus, dismissal, placement, and portal config names are acceptable.
- Runtime adapter traits and request/result types are acceptable as provisional
  `0.1.x` APIs or have a planned stabilization pass.
- State structs expose fields intentionally.
- Accessibility-related helpers remain documented as app integration contracts.

Notes:

## Source-copy Compatibility

- `dxui add <slug>` commands match registry entries.
- Generated paths remain under `src/components/ui/`.
- Source-copy templates avoid internal crate imports.
- Shared `utils` behavior is acceptable for copied projects.
- Template APIs and crate-mode APIs are intentionally aligned or documented as
  different.

Notes:

## Documentation And Examples

- README examples do not imply post-`1.0` stability.
- Component docs show current names and feature flags.
- Release docs continue to call API stability unresolved until approved.
- Changelog guidance covers breaking changes before `1.0`.
- Source preview routes match source-copy templates.

Notes:

## Decision Outcomes

Choose exactly one outcome:

- `blocked`: keep the API stability publish blocker unresolved.
- `approved`: accept the current `0.1.x` API surface for first publish.
- `deferred`: postpone first publish or require an API stabilization milestone.

Required follow-up for `approved`:

- update [API Stability Readiness Metadata](api-stability-readiness-metadata.md)
- update [Publish Readiness Blockers](publish-readiness-blockers.md)
- update [Publish Readiness Decision Matrix](publish-readiness-decision-matrix.md)
- update [Release and Package Strategy](../../release.md)
- update [Quality Gates](../../quality-gates.md)
- update [README](../../../README.md)
- update [Changelog Metadata](../../changelog-metadata.md) or `CHANGELOG.md` if
  release notes change
- update `TODOs.md`

Required follow-up for `blocked` or `deferred`:

- keep the publish blocker unresolved
- create a focused TODO milestone for the required API changes
- avoid changing versions or publishing

## Safe Validation Commands

Run after checklist updates:

```bash
npm run verify:api-stability-readiness
npm run verify:publish-readiness-blockers
npm run verify:publish-readiness-coverage
npm run verify:release-docs
npm run verify:package-scripts
npm run verify:docs
git diff --check
git status --short
```

Run before a release-candidate handoff:

```bash
npm run verify:release
```

## Non-goals

This checklist does not:

- approve API stability by itself
- change versions
- rename APIs
- rewrite props
- rewrite templates
- generate migration guides
- run `cargo package`
- run `cargo publish`
- contact crates.io
- create tags or release artifacts
