# Roadmap

## Stage 0: Documentation

Goal: align the product direction before implementation.

Deliverables:

- README
- design overview
- RFCs
- TODO plan

Exit criteria:

- repository shape is specified
- component sequence is defined
- styling and CLI direction are documented

## Stage 1: Workspace Skeleton

Goal: create the Rust project structure without shipping components yet.

Deliverables:

- Cargo workspace
- `dioxus-ui-core`
- `dioxus-ui-primitives`
- `dioxus-ui`
- `dioxus-ui-cli`
- empty registry and template directories
- web and desktop example skeletons

Exit criteria:

- `cargo check` succeeds
- crate dependency direction is enforced
- project docs match the actual layout

## Stage 2: Static Components and CLI Prototype

Goal: prove the copied-source workflow with low-risk components.

Deliverables:

- registry schema
- `dxui list`
- `dxui add button`
- Button template
- Input, Textarea, Label templates
- web demo

Exit criteria:

- generated files compile in a Dioxus example app
- Tailwind class strings are statically discoverable
- component API is documented

## Stage 3: Stateful Components

Goal: add components that need Dioxus state but not overlay primitives.

Deliverables:

- Checkbox
- Switch
- Tabs
- Accordion
- component interaction tests where practical

Exit criteria:

- keyboard behavior is documented
- controlled and uncontrolled APIs are clear
- examples cover common usage

## Stage 4: Primitive Layer

Goal: solve the hard accessibility and interaction problems once.

Deliverables:

- focus management utilities
- Dialog primitive
- Popover primitive
- Tooltip primitive
- Select primitive design

Exit criteria:

- primitives are unstyled
- styled wrappers and templates reuse primitive behavior
- accessibility behavior is tested or manually verified

## Stage 5: Packaged Components

Goal: make crate-mode usage viable after APIs settle.

Deliverables:

- feature-gated `dioxus-ui` exports
- crate-mode examples
- versioning policy
- release checklist

Exit criteria:

- users can choose copied-source or dependency mode
- feature flags avoid unnecessary component code
- docs clearly explain both installation paths

## Stage 6: Documentation Site and Previews

Goal: make component evaluation and adoption easy.

Deliverables:

- component docs site
- live examples
- theme documentation
- registry browsing

Exit criteria:

- each component has API docs, examples, and accessibility notes
- generated source and crate-mode examples stay in sync

## Stage 7: Runtime Adapters

Goal: add optional renderer-aware behavior without making controlled styled
components or source-copy templates depend on runtime commands by default.

Deliverables:

- focus and portal adapter contracts
- timer and live-region adapter contracts
- measurement, pointer, and gesture adapter contracts
- renderer verification matrix
- Web/Desktop verification examples
- documented fallback behavior for unsupported targets

Exit criteria:

- components still work without adapters
- adapters can report unsupported behavior without panics
- Web, Desktop, and Mobile checks are planned before defaults change
- Dialog or Alert Dialog verifies modal focus behavior
- Popover verifies non-modal portal and measurement behavior
- Toast or Sonner verifies timer and live-region behavior

Status: M135 meets the Dialog and Alert Dialog modal focus criterion and the
Popover measurement criterion in the styled components, verified in the Web
browser smoke. Popover uses fixed positioning instead of a DOM portal by
decision in [RFC 0010](rfcs/0010-overlay-interaction-behavior.md). M136 meets
the Toast and Sonner criterion with a paused countdown and persistent viewport
live regions ([RFC 0011](rfcs/0011-toast-timer-and-live-region.md)), verified
in the same browser smoke.
