# RFC 0004: Benchmark and CSS Output Strategy

- Status: Draft
- Created: 2026-06-21

## Summary

Use shadcn/ui as the workflow benchmark, HeroUI/NextUI as the visual and API
experience reference, and Tailwind CSS v4 as the default CSS integration target.

## Benchmark Decision

The first product target is not a direct clone of HeroUI/NextUI. The target is:

```text
Workflow benchmark: shadcn/ui
Visual/API reference: HeroUI / NextUI
Primitive behavior reference: Radix UI and Dioxus components
```

This matches the project architecture:

- copied source first
- package mode later
- primitive-first implementation for complex interactions
- Tailwind styled defaults that users can edit

## CSS Output Decision

`dioxus-shadcn` should not publish a universal precompiled Tailwind output as its
main artifact. Tailwind must scan the user's app source and generated component
files to produce the final CSS.

The CLI should create or update an input stylesheet, for example:

```css
@import "tailwindcss";
```

Optional project-level theme variables may be included:

```css
@theme {
  --color-background: var(--dxui-background);
  --color-foreground: var(--dxui-foreground);
}
```

The final compiled CSS is produced by the user's Dioxus app build.

## Non-Goals

- Do not generate a full static `tailwind.css` bundle as the primary output.
- Do not use Tailwind v3 `@tailwind base/components/utilities` directives in
  default generated files.
- Do not require a JavaScript runtime in Dioxus components.

## Consequences

Benefits:

- generated components stay compatible with Tailwind's source scanning model
- users control the final CSS build
- v4 syntax is the default from the beginning

Costs:

- `dxui init` must understand existing project CSS setup well enough to avoid
  destructive edits
- documentation needs separate notes if Tailwind v3 compatibility is added
