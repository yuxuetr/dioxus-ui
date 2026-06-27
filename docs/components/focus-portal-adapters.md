# Focus And Portal Adapter Plan

This document defines the M21.2 focus and portal adapter contracts. It builds
on the [runtime adapter plan](runtime-adapters.md), RFC 0006, and RFC 0008.

Status: Planned in M21.

## Goals

- Define focus trap, focus return, initial focus, and outside-focus contracts.
- Define portal target behavior without hard-coding one renderer.
- Map existing overlay, menu, choice, and date components to adapter needs.
- Keep pure primitives separate from platform commands.

## Non-Goals

- Implement a DOM focus trap.
- Pick a final Dioxus portal API.
- Change existing styled component APIs in this milestone.
- Make generated components depend on runtime adapters by default.

## Focus Concepts

| Concept | Meaning | Default Owner |
| --- | --- | --- |
| Initial focus | Where focus moves when content opens. | adapter command with primitive policy |
| Focus trap | Whether tab navigation stays inside a modal scope. | adapter command |
| Focus return | Where focus moves when content closes. | adapter command with app-provided trigger |
| Outside focus | Whether focus leaving content dismisses it. | primitive policy plus adapter event wiring |
| Focus scope | The mounted content subtree used by focus commands. | adapter runtime |

Focus policies should stay typed and deterministic; focus movement itself is a
runtime command.

## Portal Concepts

| Concept | Meaning | Default Owner |
| --- | --- | --- |
| Portal target | Inline, body, or named target. | primitive policy plus adapter runtime |
| Target availability | Whether a target exists on the current platform. | adapter runtime |
| Relationship preservation | ARIA IDs and trigger/content relationships across targets. | consuming app and component attrs |
| Stacking context | z-index and app shell layering. | consuming app |

The conservative default remains inline until Web and Desktop behavior is
verified. Body portals can become a Web default only after focus, stacking, and
measurement checks are covered.

## Component Mapping

| Component | Initial Focus | Trap | Return | Outside Focus | Portal Need |
| --- | --- | --- | --- | --- | --- |
| Dialog | first focusable or container | yes | trigger | ignored for modal close | body on Web, inline fallback |
| Alert Dialog | first destructive-safe action or cancel action | yes | trigger | ignored for modal close | body on Web, inline fallback |
| Sheet | first focusable or container | yes | trigger | ignored for modal close | body on Web, inline fallback |
| Drawer | first focusable or container | yes | trigger | ignored for modal close | body on Web, inline fallback |
| Popover | optional content focus | no by default | optional trigger | may dismiss | body or inline |
| Tooltip | none | no | none | no dismiss-by-focus | inline or body after hover/focus verification |
| Select | selected item or content | no modal trap by default | trigger | dismisses | body or mobile sheet |
| Combobox | input keeps focus | no | input or trigger | dismisses list | body or inline |
| Date Picker | calendar grid or content | no modal trap by default | trigger | dismisses | body or inline |
| Dropdown | first item | no | trigger | dismisses | body or inline |
| Context Menu | first item | no | invocation target if known | dismisses | body |
| Menubar | current item or first item | no | top-level trigger | dismisses nested content | body or inline |
| Navigation Menu | active trigger/content policy | no | trigger | dismisses content | inline by default |
| Hover Card | none or content when keyboard-opened | no | trigger | dismisses | body or inline |

## Primitive Boundary

Belongs in primitives:

- `FocusStrategy`
- `FocusReturn`
- `DismissBehavior`
- `PortalTarget`
- modal versus non-modal policy
- outside-focus dismissal policy

Belongs in runtime adapters:

- resolving focusable elements
- moving focus
- trapping tab order
- remembering and restoring trigger focus
- mounting into body or named targets
- detecting focus leaving a scope

Belongs in styled components:

- role and ARIA attributes
- data attributes for state and placement
- static Tailwind classes
- content slots and composition parts

Belongs in consuming apps:

- trigger node identity when the adapter cannot infer it
- route transitions that close overlays
- product-specific focus exceptions
- app shell z-index and portal target provisioning

## Adapter Shape

The exact Rust API should wait for implementation, but M21 should keep the
contract narrow:

```rust
pub enum FocusCommandResult {
  Applied,
  MissingTarget,
  Unsupported,
}

pub trait FocusRuntime {
  type NodeId;

  fn focus_initial(&self, scope: Self::NodeId, strategy: FocusStrategy) -> FocusCommandResult;
  fn trap_focus(&self, scope: Self::NodeId) -> FocusCommandResult;
  fn restore_focus(&self, target: Self::NodeId) -> FocusCommandResult;
}

pub trait PortalRuntime {
  type MountId;

  fn is_supported(&self, target: &PortalTarget) -> bool;
  fn mount_target(&self, target: &PortalTarget) -> Option<Self::MountId>;
}
```

`Unsupported` must be a valid result so Desktop and Mobile can use conservative
fallbacks without panics.

## Platform Defaults

| Target | Focus Default | Portal Default |
| --- | --- | --- |
| Web | enable modal focus trap after browser verification | body for modal overlays, inline fallback |
| Desktop | enable explicit focus return first, defer body portals | inline until WebView verification |
| Mobile | prefer sheet/full-screen modal flows for complex choices | inline or app shell target |

## Verification Strategy

Before enabling runtime defaults:

- Web example verifies focus entry, tab loop, escape close, outside focus, and
  focus return.
- Desktop example verifies focus return and stacking behavior in the WebView.
- Mobile examples avoid hover-only access and verify touch-first open/close
  paths.
- Components still work as controlled styled parts when adapters return
  `Unsupported`.

## Implementation Order

1. Keep existing primitive policies stable.
2. Add adapter traits behind an explicit module or feature.
3. Verify Dialog or Alert Dialog as the first modal adapter slice.
4. Verify Popover as the first non-modal adapter slice.
5. Reuse the same runtime contract for menu and choice components.

M22 turns this plan into a concrete primitive-layer contract. See the
[focus and portal contract implementation plan](focus-portal-contracts.md).
