# Rendered Preview Plan

This document defines the M37 plan for moving from command-line smoke examples
to rendered component previews and screenshot verification.

Status: Planned in M37.1.

## Problem

The component catalog now has registry, template, feature, docs, and command-line
smoke coverage. That proves API availability, but it does not prove visual
quality, responsive layout, or renderer behavior.

The next milestone should add rendered previews without turning the examples
into a marketing page or coupling source-copy templates to optional runtime
adapters.

## Preview Surfaces

M37 should keep the existing command-line examples as stable smoke fixtures and
add rendered preview surfaces beside them.

Recommended split:

- `examples/web-demo` remains the Web preview package and gains a rendered
  Dioxus Web entry after its current command-line smoke output is preserved
- `examples/desktop-demo` remains the Desktop preview package and gains a
  Desktop WebView smoke path after the Web surface stabilizes
- command-line smoke output stays available through scripts so CI can verify
  component state fragments without launching a browser

If one crate cannot cleanly expose both command-line and rendered entry points,
add explicit binaries instead of replacing the smoke path:

- `cargo run -p dioxus-ui-web-demo --bin smoke`
- `dx serve --package dioxus-ui-web-demo --bin preview`

## State Inventory

Rendered previews should be generated from a shared state inventory so Web,
Desktop, command-line smoke output, and future docs screenshots stay aligned.

The first inventory should cover representative states, not every prop
permutation:

- normal, disabled, invalid, selected, loading, empty, open, and overflow where
  applicable
- low-risk composition: Button Group, Input Group, Collapsible, Direction
- form-specific: Input OTP
- message composition: Attachment, Bubble, Message, Marker, Message Scroller
- chart: title, legend, line, bar, area, and fallback table
- runtime-sensitive components: Dialog, Popover, Tooltip, Select, Toast, Sonner,
  Carousel, Scroll Area, Resizable, and Sidebar

The state inventory can start as Rust helper functions that produce stable
labels, variants, and class strings. It should not require network data, upload
state, markdown parsing, chart backend adapters, or model/provider integration.

## Tailwind CSS v4 Contract

The preview app should use Tailwind CSS v4 input syntax. Do not reintroduce the
Tailwind CSS v3 `@tailwind base/components/utilities` entry.

The preview CSS input should be a source file consumed by the app build, not a
committed complete Tailwind output artifact.

Suggested source shape:

```css
@import "tailwindcss";

@source "../../crates";
@source "../../templates";
@source "./src";
```

Keep dynamic class assembly constrained to complete class strings already
present in Rust source or templates.

## Screenshot Gate

The first screenshot gate should be explicit and small:

- Web desktop viewport
- Web mobile viewport
- one chart panel with visible title, legend, SVG plot, and fallback table
- one message panel with user, assistant, attachment, marker, and scroller states
- one form panel with Input Group and Input OTP states
- one overlay panel with at least one open state rendered in-place or through a
  documented portal strategy

Desktop WebView screenshots can remain a later task until the Web screenshot
path is stable.

## Non-goals

- no landing page or hero page before usable component previews
- no Tailwind CSS v3 entry syntax
- no generated, committed full Tailwind output CSS
- no external chart backend adapter in the default preview
- no upload transport, markdown parser, stream provider, or virtualization layer
  inside the preview inventory
