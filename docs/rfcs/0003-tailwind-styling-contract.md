# RFC 0003: Tailwind Styling Contract

- Status: Draft
- Created: 2026-06-21

## Summary

Use Tailwind CSS as the default styled layer while requiring every class token
to appear as a complete static string in source files.

## Motivation

Tailwind scans source files for class-like tokens. It does not evaluate Rust
string construction. Components that dynamically build class names can silently
miss generated CSS.

## Decision

Variants and sizes should select among complete class strings:

```rust
pub enum ButtonVariant {
  Primary,
  Secondary,
  Destructive,
  Ghost,
}

impl ButtonVariant {
  pub fn class(self) -> &'static str {
    match self {
      Self::Primary => "bg-blue-600 text-white hover:bg-blue-700",
      Self::Secondary => "bg-zinc-100 text-zinc-900 hover:bg-zinc-200",
      Self::Destructive => "bg-red-600 text-white hover:bg-red-700",
      Self::Ghost => "bg-transparent hover:bg-zinc-100",
    }
  }
}
```

Do not build Tailwind tokens dynamically:

```rust
format!("bg-{}-500", color)
```

## Class Composition

Components should compose classes in this order:

1. base structural classes
2. variant classes
3. size classes
4. state classes
5. user-provided `class`

User-provided classes come last so copied-source users can override defaults
when Tailwind conflict behavior allows it.

## Theming

Early versions should prefer direct Tailwind classes over a large token system.
Theme tokens can be introduced after component APIs stabilize.

Potential future token names:

- `primary`
- `secondary`
- `destructive`
- `muted`
- `accent`
- `background`
- `foreground`
- `border`
- `ring`

## Accessibility-Related Styling

Every interactive component should include visible focus styles. Disabled and
invalid states should be represented visually and behaviorally.

Common class groups should cover:

- `focus-visible:*`
- `disabled:*`
- `aria-invalid:*` or equivalent state mapping
- dark mode only after the default light theme is stable

## Open Questions

- Should the project use `tailwind-merge`-like behavior implemented in Rust, or
  keep class composition simple at first?
- Should dark mode be included in initial component classes?
- Should generated CSS define design tokens, or should everything stay inline in
  component templates for the first milestone?
