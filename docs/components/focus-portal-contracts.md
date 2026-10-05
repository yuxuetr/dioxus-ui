# Focus And Portal Contract Implementation Plan

This document defines the M22.1 implementation plan for focus and portal
adapter contracts. It turns the M21 planning documents into a narrow code
surface before any renderer-specific runtime is added.

Status: Implemented in M22.

## Decision

Focus and portal adapter contracts should live in `dioxus-shadcn-primitives`.

Rationale:

- existing focus policy types already live in the primitive layer
- existing portal policy types already live in the primitive layer
- contracts need to reference `FocusStrategy`, `FocusReturn`, and
  `PortalTarget`
- `dioxus-shadcn-core` should remain styling and shared type infrastructure, not
  interaction runtime policy
- a new crate would add workspace complexity before there is renderer-specific
  implementation to isolate

The first implementation should add contracts only. It should not call DOM APIs,
WebView APIs, timers, or renderer hooks.

## Module Location

Planned files:

```text
crates/dioxus-shadcn-primitives/src/runtime.rs
```

Initial public exports:

```rust
FocusCommandResult
FocusRuntime
FocusRuntimeRequest
FocusRuntimeUnsupported
PortalMountResult
PortalRuntime
PortalRuntimeRequest
PortalRuntimeUnsupported
```

The names may be adjusted during implementation, but the code should keep these
concepts:

- explicit request structs for deterministic tests
- explicit unsupported result
- no panic path for missing runtime support
- no renderer-specific node handle type

## Feature Strategy

Add a feature to `dioxus-shadcn-primitives`:

```toml
runtime = []
```

Do not add the feature to `dioxus-shadcn` component features automatically in M22.
Styled components should continue to compile and behave as controlled
composition parts without runtime contracts.

Future `dioxus-shadcn` feature wiring can be explicit, for example:

```toml
dioxus-shadcn = { features = ["dialog", "runtime"] }
```

That should wait until contracts have tests and examples.

## Contract Shape

Focus contract:

```rust
pub enum FocusCommandResult {
  Applied,
  MissingTarget,
  Unsupported,
}

pub struct FocusRuntimeRequest {
  pub strategy: FocusStrategy,
  pub return_policy: FocusReturn,
  pub modal: bool,
}

pub trait FocusRuntime {
  type NodeId;

  fn focus_initial(&self, scope: Self::NodeId, request: FocusRuntimeRequest) -> FocusCommandResult;
  fn trap_focus(&self, scope: Self::NodeId, request: FocusRuntimeRequest) -> FocusCommandResult;
  fn restore_focus(&self, target: Self::NodeId, request: FocusRuntimeRequest) -> FocusCommandResult;
}
```

Portal contract:

```rust
pub enum PortalMountResult<MountId> {
  Mounted(MountId),
  Inline,
  MissingTarget,
  Unsupported,
}

pub struct PortalRuntimeRequest {
  pub target: PortalTarget,
  pub modal: bool,
}

pub trait PortalRuntime {
  type MountId;

  fn mount_target(&self, request: PortalRuntimeRequest) -> PortalMountResult<Self::MountId>;
}
```

The implementation may use simpler helper functions for M22.2 and M22.3, but it
must preserve explicit fallback results.

## Dialog And Popover Mapping

The first examples should be pure mappings from existing primitive config to
runtime requests:

```rust
let focus = FocusRuntimeRequest::from_policy(
  dialog.focus_strategy,
  dialog.focus_return,
  true,
);
let portal = PortalRuntimeRequest::from_policy(dialog.portal_target, true);
```

Dialog and Alert Dialog pass `modal: true`; Popover passes `modal: false`.
Runtime implementations can then decide whether they support the requested
focus or portal command.

## Source-Copy Policy

M22 should not change default generated component output.

Runtime contracts should not be copied when a user runs:

```bash
dxui add dialog
dxui add popover
```

Future opt-in commands can be planned separately:

```bash
dxui add runtime-focus
dxui add runtime-portal
```

Until then, source-copy components remain controlled styled parts with app-owned
runtime behavior.

## Platform Fallbacks

| Target | Focus Fallback | Portal Fallback |
| --- | --- | --- |
| Web | return `Unsupported` until a Web runtime is installed | inline or `Unsupported` until a Web runtime is installed |
| Desktop | return `Unsupported` until WebView behavior is verified | inline by default until body/named targets are verified |
| Mobile | prefer explicit app-owned sheet/full-screen flows | inline or app shell target |

Fallback behavior must be testable without a renderer.

## M22 Implementation Order

1. Add `runtime` feature and primitive module scaffold.
2. Implement focus contract result/request types and tests.
3. Implement portal contract result/request types and tests.
4. Add Dialog and Popover contract examples in docs or pure tests.
5. Update runtime planning docs after the contract surface is proven.

M22 shipped the primitive-layer `runtime` feature with focus and portal request,
result, trait, and unsupported-runtime types. Dialog and Popover mappings are
covered by pure tests without renderer-specific commands.

## Quality Gates

Before marking M22 implementation tasks done:

```bash
cargo test -p dioxus-shadcn-primitives --features runtime
cargo test --workspace --all-features --quiet
scripts/feature-check.sh
```

Generated fixture smoke is only required when registry/templates change.
