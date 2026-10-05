# Static Composition API Plan

This document defines the M20 static composition component scope. The goal is
to close low-risk shadcn-style gaps with styled, semantic composition parts
that do not introduce runtime behavior.

Status: Implemented in M20.

## Scope

M20 covers:

- Aspect Ratio
- Kbd
- Typography
- Breadcrumb
- Empty
- Field
- Item

These components should ship in crate mode and source-copy mode. Crate mode can
reuse `dioxus-shadcn-core`; generated templates must remain self-contained and must
not import internal crates.

## Shared Rules

- class tokens must be static and Tailwind-detectable
- user `class` is appended through existing class composition helpers
- no routing ownership
- no validation ownership
- no media loading ownership
- no icon dependency
- no layout measurement

## Aspect Ratio

Aspect Ratio provides a fixed-ratio slot for media or custom content.

Crate API:

```rust
AspectRatio { ratio, class, children }
aspect_ratio_style(ratio) -> String
```

Behavior defaults:

- ratio defaults to `16.0 / 9.0`
- invalid, zero, or non-finite ratios fall back to 16:9
- implementation uses deterministic inline `aspect-ratio`
- media loading, object-fit, and captions are app-owned

## Kbd

Kbd provides styled keyboard shortcut hints.

Crate API:

```rust
Kbd { size, class, children }
KbdSize::{Sm, Md, Lg}
```

Behavior defaults:

- semantic output remains inline
- keyboard shortcut meaning is supplied by text content
- app owns platform-specific shortcut wording

## Typography

Typography provides styled text composition parts for prose-like content.

Crate API:

```rust
TypographyProse { class, children }
TypographyH1 { class, children }
TypographyH2 { class, children }
TypographyH3 { class, children }
TypographyP { class, children }
TypographyLead { class, children }
TypographyMuted { class, children }
TypographyBlockquote { class, children }
TypographyInlineCode { class, children }
```

Behavior defaults:

- parts map to native semantic text elements where practical
- markdown parsing and rich text rendering are app-owned
- heading hierarchy remains app-owned

## Breadcrumb

Breadcrumb provides semantic navigation composition parts.

Crate API:

```rust
Breadcrumb { class, children }
BreadcrumbList { class, children }
BreadcrumbItem { class, children }
BreadcrumbLink { current, class, children }
BreadcrumbPage { class, children }
BreadcrumbSeparator { class, children }
BreadcrumbEllipsis { class }
```

Behavior defaults:

- root uses navigation semantics
- list uses ordered list semantics
- app owns routing and link elements
- current page state maps to `aria-current`

## Empty

Empty provides empty-state layout parts.

Crate API:

```rust
Empty { class, children }
EmptyHeader { class, children }
EmptyTitle { class, children }
EmptyDescription { class, children }
EmptyContent { class, children }
EmptyActions { class, children }
```

Behavior defaults:

- no implicit alert or status role
- app owns actions, icons, and illustrations
- usable for zero results, first-run, or filtered-empty states

## Field

Field provides form layout composition around existing controls.

Crate API:

```rust
Field { invalid, disabled, class, children }
FieldLabel { class, children }
FieldDescription { class, children }
FieldError { class, children }
FieldGroup { class, children }
```

Behavior defaults:

- invalid and disabled map to data attributes
- app owns control IDs and label association
- validation state and messages are app-owned

## Item

Item provides generic list or command result composition parts.

Crate API:

```rust
Item { selected, disabled, class, children }
ItemMedia { class, children }
ItemContent { class, children }
ItemTitle { class, children }
ItemDescription { class, children }
ItemActions { class, children }
```

Behavior defaults:

- selected and disabled map to data attributes
- app owns click handlers and navigation
- app owns collection semantics when a listbox, menu, or table is required

## Implementation Order

1. Static composition API plan
2. Aspect Ratio, Kbd, and Typography
3. Breadcrumb and Empty
4. Field and Item
5. documentation, parity, and batch updates

This order starts with the lowest-risk styled wrappers, then moves into small
semantic composition surfaces.

M20 shipped all seven components in crate mode and source-copy mode with docs
pages, registry entries, demo usage, per-feature compile coverage, and generated
fixture smoke coverage.

## Quality Gates

Each M20 component should include:

- crate-mode class composition tests
- registry entry and self-contained template
- component docs page
- web and desktop demo usage
- generated fixture smoke coverage
- per-feature compile coverage

Before marking implementation tasks done, run:

```bash
cargo test --workspace --all-features
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
```
