# Component API Specification

## Goal

This document defines the public component API conventions for `dioxus-ui`.
Every component template and crate module should follow these rules unless a
component-specific RFC documents an exception.

## API Shape

Components should use Dioxus component conventions and feel natural in RSX:

```rust
rsx! {
  Button {
    variant: ButtonVariant::Primary,
    size: ButtonSize::Md,
    density: UiDensity::Comfortable,
    class: "w-full",
    "Save changes"
  }
}
```

The default usage should stay concise:

```rust
rsx! {
  Button { "Save changes" }
}
```

## Common Props

### `class`

Every styled component should accept a user class string.

Rules:

- default is empty
- user classes are appended last
- component classes must be complete Tailwind tokens
- dynamic class token construction is not allowed
- a component never joins two utilities that set the same property under the
  same variant, such as a base `border-input` with a state
  `border-destructive`; the base value moves into the default branch instead
  (see [RFC 0044](rfcs/0044-tailwind-utility-conflicts.md))

Appending does not make a user class win. Tailwind orders utilities in the
stylesheet, not by their position in the class list, so `bg-blue-100` passed
to a component whose base class has `bg-muted` may lose. Add classes for
properties the component leaves unset, or override one it sets with
Tailwind's important modifier, such as `bg-blue-100!`.

### `children`

Container and content components should accept `children: Element`.

Examples:

- Button
- Card
- Alert
- Dialog
- Tabs

Void-like controls such as Input may omit children.

### `variant`

`variant` describes visual intent.

Common variant names:

```rust
pub enum ButtonVariant {
  Primary,
  Secondary,
  Destructive,
  Outline,
  Ghost,
  Link,
}
```

Variant names should be semantic, not color-specific. Use `Destructive` instead
of `Red`, and `Primary` instead of `Blue`.

### `size`

`size` describes component scale within the current density.

Common size names:

```rust
pub enum ButtonSize {
  Sm,
  Md,
  Lg,
  Icon,
}
```

Size should not replace density. Size is component-local scale; density is
application or platform profile.

### `density`

Interactive components should eventually support `UiDensity`.

```rust
pub enum UiDensity {
  Compact,
  Comfortable,
  Touch,
}
```

Initial components may use a default density internally before global context is
implemented. The public API should avoid design choices that make density hard
to add later.

### State Props

Use boolean props for simple state:

```rust
disabled: bool
invalid: bool
loading: bool
```

Naming rules:

- use `disabled`, not `is_disabled`
- use `invalid`, not `error`, when the prop maps to ARIA invalid state
- use `loading` only when the component changes interaction behavior

### Event Props

Event props should follow Dioxus naming and types where possible.

Examples:

```rust
onclick: EventHandler<MouseEvent>
oninput: EventHandler<FormEvent>
onchange: EventHandler<FormEvent>
```

Component-specific events should use clear names such as:

```rust
on_open_change
on_value_change
on_checked_change
```

## Class Composition

Class order should be deterministic:

1. base structural classes
2. variant classes
3. size classes
4. density classes
5. state classes
6. user `class`

Example:

```rust
let class = classes([
  Some("inline-flex items-center justify-center"),
  Some(variant.class()),
  Some(size.class()),
  Some(density.class()),
  disabled.then_some("disabled:pointer-events-none disabled:opacity-50"),
  Some(class.as_str()),
]);
```

`classes` joins complete class fragments in order, skips empty fragments, and
preserves duplicate tokens so later user classes can override earlier defaults
where Tailwind ordering allows it.

## Attribute Forwarding

A component or part that renders a native interactive element (a button,
link, label, or form control) the app acts on directly takes:

- the element's event callback as an explicit prop, such as
  `onclick: Option<EventHandler<MouseEvent>>`, because Dioxus 0.7.9
  `#[props(extends = ...)]` forwards attributes but not event listeners;
- `attributes: Vec<Attribute>` extending `GlobalAttributes` and the element,
  spread after the component's explicit attributes so its own state and ARIA
  attributes keep their values.

A part whose click already reports through a component callback, such as
`on_open_change` or `on_value_change`, and a static wrapper add these only
when a consumer needs them. See RFC 0028 to RFC 0036 and
[RFC 0053](rfcs/0053-interactive-part-callbacks.md).

For source-copy templates, users can edit the generated component if they need
additional attributes.

## Accessibility Baseline

Every component specification should document:

- semantic element or role
- keyboard behavior
- focus behavior
- disabled behavior
- ARIA attributes, if any
- screen reader label strategy

## Component Category Requirements

### Static Components

Examples: Badge, Card, Alert, Avatar, Separator.

Requirements:

- semantic HTML where possible
- no unnecessary ARIA
- no hidden interaction behavior

### Form Components

Examples: Input, Textarea, Label, Checkbox, Switch.

Requirements:

- label association must be documented
- disabled state must affect behavior and styling
- invalid state should map to `aria-invalid` when applicable
- focus-visible styling is required

### Stateful Layout Components

Examples: Tabs, Accordion.

Requirements:

- keyboard navigation must be specified before implementation
- controlled and uncontrolled APIs must be explicit
- selected/open state must be reflected in attributes

### Overlay Components

Examples: Dialog, Dropdown, Popover, Tooltip, Select.

Requirements:

- primitive-first design
- Escape key behavior documented where applicable
- outside interaction behavior documented
- focus entry and return behavior documented
- mobile/touch presentation considered before implementation

## Naming Rules

Type names:

```rust
Button
ButtonProps
ButtonVariant
ButtonSize
Dialog
DialogRoot
DialogTrigger
DialogContent
```

Module names:

```rust
button
dialog
input
tabs
```

Feature names and registry names should match module names:

```text
button
dialog
input
tabs
```

## Initial Component API Targets

### Button

Minimum props:

- `variant`
- `size`
- `density`
- `class`
- `disabled`
- `children`
- `onclick`

### Input

Minimum props:

- `value`
- `placeholder`
- `class`
- `disabled`
- `invalid`
- `oninput`

### Label

Minimum props:

- `r#for`
- `class`
- `children`

### Dialog

Dialog requires a separate primitive design before implementation.

Minimum conceptual pieces:

- `DialogRoot`
- `DialogTrigger`
- `DialogContent`
- `DialogTitle`
- `DialogDescription`
- `DialogClose`

## Compatibility Notes

The copied-source API and crate API should stay as close as practical. If they
diverge, the source-copy template takes priority until the first stable release.
