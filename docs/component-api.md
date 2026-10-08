# Component API Specification

## Goal

This document defines the public component API conventions for `dioxus-shadcn`.
Every component template and crate module should follow these rules unless a
component-specific RFC documents an exception.

## API Shape

Components should use Dioxus component conventions and feel natural in RSX:

```rust
rsx! {
  Button {
    variant: ButtonVariant::Primary,
    size: ButtonSize::Md,
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
- the user class is merged last with `merge_classes`, so it wins (see
  [RFC 0076](rfcs/0076-user-class-overrides.md))
- component classes must be complete Tailwind tokens
- dynamic class token construction is not allowed
- a component never joins two utilities that set the same property under the
  same variant, such as a base `border-input` with a state
  `border-destructive`; the base value moves into the default branch instead
  (see [RFC 0044](rfcs/0044-tailwind-utility-conflicts.md))

A user utility replaces each component utility whose properties it sets in
full, under the same variants and importance: `bg-blue-100` passed to a
component with `bg-muted` renders `bg-blue-100` alone, and `hover:` or
`data-[state=open]:` component styles stay. A user utility that sets only
some of a component utility's properties, such as `px-2` over `p-4`, keeps
both; it applies wherever Tailwind orders it later, which holds for that
pair. Where it does not, as with a logical side over a physical one (`ms-2`
over `ml-4`), add Tailwind's important modifier, such as `ms-2!`.

A class the merge does not know, such as an app's own class, a theme name
the app added (`bg-brand`), or an arbitrary value of a type the merge cannot
tell on a utility with several meanings (`bg-[13px]`), replaces nothing. In
debug builds the merge logs each class it could not classify and each
component class it replaced, once, at the debug level.

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

Density is the platform profile, so it comes from one root,
`DensityProvider { density }`, and interactive components read it with
`use_density()` ([RFC 0078](rfcs/0078-touch-density.md)); without a provider
they render at `Comfortable`.

```rust
pub enum UiDensity {
  Compact,
  Comfortable,
  Touch,
}
```

Under `Touch` every interactive control offers a 44 by 44 CSS pixel target:
a new control that holds text appends `density_control_class(use_density())`
before the app's class, and one drawn smaller appends
`density_hit_area_class(use_density())`. A component never takes a
`density` prop.

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

## Public Surface

A component module makes public only what an app uses
([RFC 0079](rfcs/0079-public-surface.md)):

- the components, their props, and the enums the props take
- a class function, when the component page lists it under API Surface, for
  styling an element the component does not render
- a helper, when the page's text or an example uses it

Class constants, the class functions of parts no page lists, and helpers for
the component's own work (geometry, style strings, key handling, parsing)
stay private, or `pub(crate)` when another module needs them. Every public
item has a doc comment: the crate denies `missing_docs`.

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
DialogContent
DialogTitle
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

## Minimum Component APIs

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
- `on_value_change`

### Label

Minimum props:

- `r#for`
- `class`
- `children`

### Dialog

Dialog is controlled: the app owns `open`, renders its own trigger, and passes
`on_open_change` to the content. Its parts are:

- `DialogOverlay`
- `DialogContent`
- `DialogTitle`
- `DialogDescription`
- `DialogClose`

## Page Scripts

A component that needs the browser beyond Dioxus events, such as focus
trapping, anchored placement, or pointer capture, runs a page script through
the `script` helper's `component_script!`: a wasm-bindgen snippet on the web
and `document::eval` on Desktop and Mobile
([RFC 0080](rfcs/0080-page-scripts-without-eval.md)). Components never call
`document::eval` directly, so they work under a Content Security Policy
without `'unsafe-eval'`; `npm run verify:csp` fails when one does.

## Security Rules

- A prop that takes a link's URL passes it through the `safe-url` helper's
  `safe_href`, which keeps relative URLs, fragments, and `http`, `https`,
  `mailto`, and `tel`, and renders no `href` otherwise. A disabled link drops
  its `href` and keeps `role="link"`.
- No component sets `dangerous_inner_html`; text and children go through
  Dioxus, which escapes them.
- A value a component writes into script source is a JavaScript string with
  `\u` escapes (`theme_init_script`), and a value read back from storage is
  checked against the form it should have before it is applied.
- Escape closes the overlay opened last: Dialog, Sheet, Drawer, and Alert
  Dialog consult the `layer` helper before closing.
- Shortcuts a component registers on the page leave inputs, selects, and
  editable content alone.

See the README's [Security](../README.md#security) section for what apps
are told.

## Compatibility Notes

The copied-source API and crate API should stay as close as practical. If they
diverge, the source-copy template takes priority until the first stable release.
