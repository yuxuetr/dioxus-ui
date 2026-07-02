# Documentation Site Plan

## Current Site Shape

The documentation site is currently Markdown-first:

```text
docs/
├─ README.md
├─ components/
│  └─ README.md
├─ component-api.md
├─ design.md
├─ release.md
├─ roadmap.md
├─ workspace.md
└─ rfcs/
```

This keeps the project easy to review before choosing a docs runtime.

## Future Site Runtime

The site should eventually be a Dioxus Web app or static site that uses the same
component crate and registry metadata as the CLI.

Required views:

- component catalog
- component detail page
- source-copy command
- crate feature usage
- accessibility notes
- Web/Desktop preview tabs
- generated source preview

## Preview Contract

Each component preview should show:

- default state
- variant or role-specific states
- disabled state when applicable
- invalid state when applicable
- density differences when applicable
- keyboard and ARIA notes for interactive components

Overlay previews should also show:

- open state
- closed state
- primitive config defaults
- mobile/desktop behavior notes

## Data Sources

The docs site should derive its catalog from:

- `registry/*.json`
- `templates/*.rs`
- public crate features
- component docs in `docs/components`

Duplicating component metadata manually should be avoided once the docs site
becomes executable.

## M53 Catalog Data Contract

Before building a visual docs runtime, the project should expose a deterministic
catalog contract that can be derived from local source files.

Required catalog fields:

| Field | Source | Notes |
| --- | --- | --- |
| `name` | `registry/*.json` | Stable CLI/source-copy component id. |
| `description` | `registry/*.json` | Short catalog summary. |
| `registry_path` | filesystem | Path to the registry entry. |
| `template_path` | registry file list | Primary source-copy template path. |
| `docs_path` | `docs/components/{name}.md` | Component detail markdown page. |
| `crate_feature` | `crates/dioxus-ui/Cargo.toml` | Feature users enable for crate mode. |
| `crate_module` | `crates/dioxus-ui/src/{name}.rs` | Styled crate module path. |
| `source_copy_target` | registry file list | Generated target path for `dxui add`. |

Derived catalog fields:

- `slug`: same as `name`
- `title`: title-cased `name`
- `crate_import`: module name with dashes converted to underscores
- `source_copy_command`: `dxui add {name}`
- `crate_feature_toml`: `dioxus-ui = { features = ["{name}"] }`

Intentional exceptions:

- `utils` remains a source-copy helper, not a catalog component page.
- Planning and strategy markdown files under `docs/components` are not component
  detail pages unless their basename matches a registry component.

Deferred visual-runtime fields:

- preview image paths
- rendered Web/Desktop route ids
- screenshot artifact paths
- visual state matrix ids
- interactive examples

Those fields should be added only after the catalog metadata check is stable.

## Catalog Verification

The current catalog contract is verified by:

```bash
npm run verify:docs-catalog
```

The command builds the catalog in memory and fails on missing required fields or
surface drift. It verifies:

- public component registry entries
- source-copy template paths and generated targets
- component docs pages
- crate feature names
- styled crate module paths
- `lib.rs` public module exports

The command intentionally does not write a generated catalog artifact. A future
docs runtime should either call the same source-reading logic or introduce a
generated artifact only after the contract is stable and reviewed.
