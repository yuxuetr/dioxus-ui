# Product Surface Audit

This document tracks the M52 local product surface audit.

## M52.1 Scope

The audit checks local consistency across the implemented product surface. It
does not refresh upstream shadcn/ui parity and does not add new components.

In scope:

- `registry/*.json` component entries
- `templates/*.rs` source-copy files
- `crates/dioxus-ui/src/*.rs` styled crate modules
- `docs/components/*.md` component documentation pages
- `crates/dioxus-ui/src/lib.rs` public module and re-export surface
- `crates/dioxus-ui/Cargo.toml` feature declarations

Out of scope:

- browsing upstream shadcn/ui for new components
- changing component APIs
- changing Tailwind tokens or theme behavior
- adding a docs site renderer
- activating CI browser workflows

Audit categories:

- complete local component: registry entry, template, crate module, docs page,
  feature, and public export all exist
- source-copy utility: registry/template item that is intentionally not a
  public styled component, such as `utils`
- docs-only planning page: documentation without a registry/template/module
  component
- drift: an item missing one or more expected local surfaces without an
  documented reason

Evidence required for M52.2:

- counts for registry entries, templates, crate modules, component docs, and
  features
- exact missing/extra item lists after normalizing dash and underscore naming
- notes for intentional exceptions

Decision required for M52.3:

- whether the next milestone should focus on component parity, theme tokens,
  docs-site rendering, or release hardening
- the smallest next milestone that improves user-facing quality without adding
  unrelated churn

## M52.2 Local Catalog Consistency Result

The local audit compared normalized component names across:

- `registry/*.json`
- `templates/*.rs`
- `crates/dioxus-ui/src/*.rs`
- `crates/dioxus-ui/Cargo.toml` features
- `crates/dioxus-ui/src/lib.rs` modules and re-exports
- `docs/components/*.md`

Counts:

| Surface | Count |
| --- | ---: |
| Registry entries, excluding `schema.json` | 65 |
| Template files | 65 |
| Styled crate modules, excluding `lib.rs` | 64 |
| Styled crate features, excluding `default` | 64 |
| `lib.rs` public modules | 64 |
| `lib.rs` component re-export modules | 64 |
| Component docs matching registry names | 64 |
| Planning and strategy docs | 48 |

Intentional exceptions:

- `utils` is a source-copy helper with `registry/utils.json` and
  `templates/utils.rs`; it is not a styled crate component, feature, or docs
  page.
- `dioxus-ui-core` and `dioxus-ui-primitives` are external crate re-exports in
  `lib.rs`; they are not component modules and are excluded from component
  re-export counts.

Drift result:

| Check | Result |
| --- | --- |
| Registry entries missing templates | none |
| Registry components missing crate modules | none |
| Registry components missing features | none |
| Registry components missing docs pages | none |
| Templates missing registry entries | none |
| Crate modules missing registry entries | none |
| Features missing registry entries | none |
| `lib.rs` modules missing crate files | none |
| Component docs missing registry entries | none |

Conclusion: the local product surface is internally consistent. There is no
immediate release-hardening task for registry/template/docs/crate drift.

The remaining 48 docs under `docs/components` are planning, verification,
runtime strategy, or audit documents rather than public component pages.

## M52.3 Recommendation

Recommended next milestone: docs-site rendering preparation.

Reasoning:

- Component parity does not need an immediate local implementation milestone.
  The current parity matrix says no new component-fill milestone is required by
  the current tracked catalog.
- Release hardening for registry/template/docs/crate drift is not the bottleneck.
  M52.2 found no local product surface drift.
- Theme tokens are useful later, but RFC 0003 still recommends direct Tailwind
  classes before introducing a large token system.
- The project already has rendered Web and Desktop preview shells plus a shared
  preview state inventory, but the docs site remains Markdown-first.

The next milestone should therefore make the product easier to inspect without
changing component APIs:

1. define a docs-site catalog data contract derived from registry entries,
   component docs, crate features, and preview states
2. add a small script or static metadata check that proves catalog pages can be
   generated without hand-maintained duplication
3. keep actual visual site runtime work separate unless the data contract is
   stable

Suggested next milestone name:

```text
M53 Docs Site Catalog Data Contract
```

Suggested M53 tasks:

- plan catalog fields and source ownership
- generate or verify a local catalog index from registry/docs/features
- document how the future docs site should consume the catalog
- run deterministic gates without adding new component APIs

Not recommended as the next milestone:

- component parity refresh: requires browsing upstream and should be scheduled
  separately when desired
- theme token implementation: premature until catalog and preview consumption
  are easier to inspect
- active CI workflow: intentionally deferred by RFC 0009
