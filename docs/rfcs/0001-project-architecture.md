# RFC 0001: Project Architecture

- Status: Draft
- Created: 2026-06-21

## Summary

Build `dioxus-ui` as a multi-crate workspace with a source-copy CLI and an
optional packaged component crate.

## Motivation

Dioxus applications benefit from reusable UI components, but early ecosystem
users often need to customize component source directly. A source-copy workflow
keeps adoption flexible while APIs mature. A packaged crate can be added once
the component surface is stable enough.

## Decision

Use this planned layout:

```text
crates/
├─ dioxus-ui-core
├─ dioxus-ui-primitives
├─ dioxus-ui
└─ dioxus-ui-cli
registry/
templates/
examples/
docs/
```

The dependency direction is:

```text
dioxus-ui-cli ──► dioxus-ui-core
dioxus-ui ──────► dioxus-ui-core
dioxus-ui ──────► dioxus-ui-primitives
dioxus-ui-primitives ──► dioxus-ui-core
```

Templates may duplicate or adapt crate code when that produces better generated
source for users. Shared behavior for complex components should come from
primitives where possible.

## Alternatives Considered

### Single Component Crate

This is simpler to publish but makes customization harder and encourages
premature API stability.

### Pure Source Generator

This maximizes user control but can duplicate complex accessibility behavior.
The primitive crate reduces that risk for overlays and keyboard-heavy widgets.

### Pure Tailwind Components

This works for static components but fails for Dialog, Popover, Select, and
Tooltip because behavior and accessibility are the hard parts.

## Consequences

Benefits:

- source-copy mode supports early customization
- primitives make complex behavior reusable
- crate mode can arrive after the API stabilizes

Costs:

- more workspace structure
- templates and crate exports can drift without tests
- CLI registry needs validation

## Open Questions

- Should templates depend on `dioxus-ui-primitives`, or should they copy all
  primitive behavior into the user project?
- How much should `dxui init` modify an existing Dioxus project?
- What minimum Dioxus version should be supported for the first release?
