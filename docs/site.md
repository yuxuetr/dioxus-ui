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
