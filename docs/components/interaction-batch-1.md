# Interaction Batch 1 Specification

This document defines the first post-primitive interaction component batch.
The batch was implemented in M11.

Batch 1 should stay below overlay complexity. These components exercise
controlled state, ARIA mapping, keyboard behavior, and source-copy templates
without requiring portals or runtime DOM measurement.

## Components

| Component | Primitive dependency | Implementation mode | Rationale |
| --- | --- | --- | --- |
| Spinner | None | Styled-only | Static feedback primitive with ARIA status semantics. |
| Toggle | None | Styled controlled component | Single pressed state maps directly to button semantics. |
| Radio Group | Roving focus | Primitive-backed styled component | Needs grouped single selection and arrow-key navigation. |
| Toggle Group | Roving focus | Primitive-backed styled component | Needs grouped pressed state and arrow-key navigation. |
| Slider | First-party slider state | Primitive-backed styled component | Needs value math, keyboard increments, and ARIA value mapping. |

## Implementation Status

| Component | Status |
| --- | --- |
| Spinner | Implemented |
| Toggle | Implemented |
| Radio Group | Implemented |
| Toggle Group | Implemented |
| Slider primitive state | Implemented |
| Slider styled component | Implemented |

Remaining hardening:

- Wire real keyboard events in examples instead of only exposing pure helpers.
- Add browser-level focus movement tests when the docs/demo app has a rendered
  component preview surface.
- Revisit mobile touch drag behavior for Slider before declaring it stable.

## Shared API Rules

- Components must support crate-mode feature flags and `dxui add` source-copy
  templates.
- Generated templates must remain self-contained and must not import
  `dioxus-ui-core` or `dioxus-ui-primitives`.
- Styling must use static Tailwind class tokens.
- Controlled props come first; later uncontrolled helpers can be added only
  after the controlled API is stable.
- Components that expose ARIA roles must provide helper functions for class and
  attribute decisions where that keeps tests pure.

## Spinner

Scope:

- Styled `Spinner` component.
- Size variants: `Sm`, `Md`, `Lg`.
- Optional accessible label.
- `role="status"` wrapper semantics in generated template.

No primitive crate API is required.

Tests:

- Class includes size and user class.
- Accessible label path is rendered or documented for generated source.

## Toggle

Scope:

- Styled `Toggle` component.
- Variants: `Default`, `Outline`.
- Sizes: `Sm`, `Md`, `Lg`.
- Controlled `pressed` state.
- `aria-pressed` semantics.

No primitive crate API is required for the first version.

Tests:

- Class reflects pressed, variant, size, disabled, and user class.
- Generated template compiles independently.

## Radio Group

Scope:

- `RadioGroup` container.
- `RadioGroupItem` item.
- Single selected value.
- Horizontal and vertical orientation.
- Disabled item styling.

Primitive needs:

- Reuse `RovingFocusState`, `RovingFocusItem`, `FocusMove`, and
  `NavigationOrientation`.
- Add a small radio selection state helper only if class/ARIA tests become
  repetitive.

Keyboard contract:

- Arrow keys move focus according to orientation.
- Moving focus may select the focused item in the first controlled API.
- Disabled items are skipped.

Tests:

- Roving focus integration over enabled items.
- Checked state class and `aria-checked` mapping.
- Generated template compiles independently.

## Toggle Group

Scope:

- `ToggleGroup` container.
- `ToggleGroupItem` item.
- Single and multiple selection modes.
- Horizontal and vertical orientation.

Primitive needs:

- Reuse roving focus primitives.
- Add `ToggleGroupSelection` helper if single/multiple state logic becomes
  shared across crate-mode and templates.

Keyboard contract:

- Arrow keys move focus.
- Space/Enter toggles the focused item.
- Disabled items are skipped.

Tests:

- Single selection replaces the active value.
- Multiple selection toggles membership.
- Roving focus skips disabled items.
- Generated template compiles independently.

## Slider

Scope:

- Styled `Slider` root, track, range, and thumb.
- Controlled numeric value.
- Min, max, step, and disabled props.
- Horizontal orientation first.

Primitive needs:

- Add pure slider state helpers for clamping, stepping, percentage conversion,
  and keyboard delta decisions.

Keyboard contract:

- Arrow keys step value.
- Home moves to min.
- End moves to max.
- Page Up and Page Down use larger step increments.

Tests:

- Values clamp to min and max.
- Step increments round consistently.
- Percent calculation is stable for min/max ranges.
- ARIA value attributes map to the controlled value.
- Generated template compiles independently.

## Recommended M11 Order

1. Spinner
2. Toggle
3. Radio Group
4. Toggle Group
5. Slider primitive state
6. Slider styled component
7. Batch documentation and examples

This order starts with the lowest-risk components, then applies M10 roving
focus primitives, and leaves slider value math isolated before rendering.
