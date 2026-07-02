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
