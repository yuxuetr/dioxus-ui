# Low-risk Composition Gap API Plan

This document defines the M30.2 API plan for Button Group, Input Group,
Collapsible, and Direction.

Status: Planned in M30.2. Implemented in M31.

## Decision

Implement the low-risk composition gaps before Input OTP, message components,
Message Scroller, or Chart.

These components should stay source-copy friendly and avoid renderer-specific
runtime behavior:

- Button Group and Input Group are pure styled composition parts.
- Collapsible is a controlled disclosure surface with ARIA attributes, not an
  uncontrolled animation runtime.
- Direction is accepted as a small direction attribute/style helper only if it
  stays useful without context providers or runtime dependencies.

## Component Classification

| Component | Public In M31 | Implementation Shape | Runtime Needed |
| --- | --- | --- | --- |
| Button Group | Yes | styled root and item/slot helpers around existing Button patterns | No |
| Input Group | Yes | styled root, addon, control, and action parts around existing Input patterns | No |
| Collapsible | Yes | controlled trigger/content composition with `open` state passed by app | No renderer runtime |
| Direction | Conditional yes | `dir` attribute wrapper/helper for LTR/RTL composition | No |

Direction should be implemented only if the API is clearer than asking apps to
set `dir` directly. If implementation reveals that it adds no value, M31.4
should document the deferral rather than forcing a weak component.

## Button Group API

Crate feature:

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["button-group"] }
```

Source-copy command:

```bash
dxui add button-group
```

Planned crate API:

```rust
ButtonGroup {
  orientation: ButtonGroupOrientation,
  attached: bool,
  class: String,
  children: Element,
}

ButtonGroupItem {
  class: String,
  children: Element,
}

ButtonGroupOrientation::{Horizontal, Vertical}
button_group_class(...)
button_group_item_class(...)
```

Behavior:

- `ButtonGroup` owns layout classes only.
- `ButtonGroupItem` should be compatible with `Button` children or direct
  button-like content.
- Selection, pressed state, roving focus, and command behavior remain app-owned
  or owned by Toggle Group when semantic selection is required.

Accessibility:

- Use `role="group"` when a label is provided by the consuming app.
- Do not invent toolbar semantics by default.
- Icon-only children still need accessible labels from the app.

Tailwind constraints:

- Use static full class tokens for orientation, spacing, borders, and attached
  radius behavior.
- Do not build dynamic radius or border class fragments.

## Input Group API

Crate feature:

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["input-group"] }
```

Source-copy command:

```bash
dxui add input-group
```

Planned crate API:

```rust
InputGroup {
  invalid: bool,
  disabled: bool,
  class: String,
  children: Element,
}

InputGroupAddon {
  position: InputGroupAddonPosition,
  class: String,
  children: Element,
}

InputGroupControl {
  class: String,
  children: Element,
}

InputGroupAction {
  class: String,
  children: Element,
}

InputGroupAddonPosition::{Start, End}
input_group_class(...)
input_group_addon_class(...)
input_group_control_class(...)
input_group_action_class(...)
```

Behavior:

- `InputGroup` owns visual grouping, density, invalid, and disabled styling.
- The actual `Input`, label, value, validation, and form submission remain
  app-owned.
- Addons are visual or descriptive slots; they must not replace labels.

Accessibility:

- Pair grouped inputs with `Label`.
- Preserve native `input` semantics.
- Decorative addons should be `aria-hidden` in app markup when appropriate.
- Action slots should use native buttons with accessible labels.

Tailwind constraints:

- Use static class maps for invalid and disabled states.
- Avoid runtime-generated positional class fragments.

## Collapsible API

Crate feature:

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["collapsible"] }
```

Source-copy command:

```bash
dxui add collapsible
```

Planned crate API:

```rust
Collapsible {
  open: bool,
  disabled: bool,
  class: String,
  children: Element,
}

CollapsibleTrigger {
  open: bool,
  disabled: bool,
  controls: Option<String>,
  class: String,
  children: Element,
}

CollapsibleContent {
  open: bool,
  id: Option<String>,
  force_mount: bool,
  class: String,
  children: Element,
}

collapsible_class(...)
collapsible_trigger_class(...)
collapsible_content_class(...)
```

Behavior:

- Open state is controlled by the consuming app.
- Trigger click handlers are app-owned in source-copy and crate mode.
- Content can be hidden when closed unless `force_mount` is enabled.
- Animation timing and height measurement remain app-owned.

Accessibility:

- Trigger should render button semantics or wrap app-provided button content.
- Trigger maps `open` to `aria-expanded`.
- `controls` and `id` should be passed through when the app wants explicit
  trigger/content association.

Tailwind constraints:

- Use static open/closed class maps.
- Do not depend on measured height classes or runtime-generated animation names.

## Direction API

Crate feature:

```toml
dioxus-ui = { version = "0.1", default-features = false, features = ["direction"] }
```

Source-copy command, if accepted:

```bash
dxui add direction
```

Planned crate API:

```rust
Direction {
  dir: TextDirection,
  class: String,
  children: Element,
}

TextDirection::{Ltr, Rtl}
direction_class(...)
```

Behavior:

- `Direction` should set `dir="ltr"` or `dir="rtl"` on a wrapper.
- It should not introduce context providers, global document mutation, or
  runtime direction detection.
- Apps own locale, text shaping, mixed-direction content, and route-level
  direction policy.

Accessibility:

- Prefer native `dir` semantics.
- Do not encode direction only through CSS classes.
- Nested direction wrappers are acceptable when app content requires them.

Tailwind constraints:

- Direction classes must be static.
- Avoid requiring Tailwind plugins or generated directional variants in the
  first implementation.

## Source-copy Policy

Generated templates for M31 must remain self-contained:

- no imports from `dioxus-ui-core`
- no imports from `dioxus-ui-primitives`
- no renderer runtime dependencies
- no Tailwind dynamic class generation

Registry entries must clearly list any component dependencies. Button Group and
Input Group may depend conceptually on Button/Input patterns, but their source
templates should still be usable independently.

## Implementation Order

1. Button Group
2. Input Group
3. Collapsible
4. Direction or explicit Direction deferral
5. Documentation and quality gates

This order starts with pure composition, then adds controlled disclosure, and
leaves Direction last because it may be a documentation-only recommendation if
the wrapper is not useful enough.

## Implementation Result

M31 accepted and implemented all four low-risk composition gaps:

- Button Group
- Input Group
- Collapsible
- Direction

All four components have crate features, source-copy templates, registry
entries, docs pages, catalog entries, accessibility notes, parity updates, and
demo smoke coverage.

## Quality Gates

Before each M31 task is marked done:

```bash
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
scripts/generated-fixture-smoke.sh
git diff --check
```

M31.5 should also update the parity matrix and component catalog after all
accepted components are implemented.
