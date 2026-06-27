use crate::{FocusReturn, FocusStrategy, PortalTarget};

/// Result of a runtime focus command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FocusCommandResult {
  Applied,
  MissingTarget,
  Unsupported,
}

/// Request metadata for runtime focus commands.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FocusRuntimeRequest {
  pub strategy: FocusStrategy,
  pub return_policy: FocusReturn,
  pub modal: bool,
}

impl FocusRuntimeRequest {
  pub const fn new(strategy: FocusStrategy, return_policy: FocusReturn, modal: bool) -> Self {
    Self {
      strategy,
      return_policy,
      modal,
    }
  }

  pub const fn modal(strategy: FocusStrategy, return_policy: FocusReturn) -> Self {
    Self::new(strategy, return_policy, true)
  }

  pub const fn non_modal(strategy: FocusStrategy, return_policy: FocusReturn) -> Self {
    Self::new(strategy, return_policy, false)
  }

  pub const fn dialog_default() -> Self {
    Self::modal(FocusStrategy::FirstFocusable, FocusReturn::Trigger)
  }

  pub const fn popover_default() -> Self {
    Self::non_modal(FocusStrategy::None, FocusReturn::None)
  }

  pub const fn should_focus_initial(self) -> bool {
    !matches!(self.strategy, FocusStrategy::None)
  }

  pub const fn should_restore_focus(self) -> bool {
    !matches!(self.return_policy, FocusReturn::None)
  }

  pub const fn should_trap_focus(self) -> bool {
    self.modal
  }
}

/// Runtime focus command surface.
pub trait FocusRuntime {
  type NodeId;

  fn focus_initial(
    &self,
    scope: &Self::NodeId,
    request: FocusRuntimeRequest,
  ) -> FocusCommandResult;

  fn trap_focus(
    &self,
    scope: &Self::NodeId,
    request: FocusRuntimeRequest,
  ) -> FocusCommandResult;

  fn restore_focus(
    &self,
    target: &Self::NodeId,
    request: FocusRuntimeRequest,
  ) -> FocusCommandResult;
}

/// Focus runtime used when the current target has no adapter installed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FocusRuntimeUnsupported;

impl FocusRuntime for FocusRuntimeUnsupported {
  type NodeId = ();

  fn focus_initial(
    &self,
    _scope: &Self::NodeId,
    _request: FocusRuntimeRequest,
  ) -> FocusCommandResult {
    FocusCommandResult::Unsupported
  }

  fn trap_focus(
    &self,
    _scope: &Self::NodeId,
    _request: FocusRuntimeRequest,
  ) -> FocusCommandResult {
    FocusCommandResult::Unsupported
  }

  fn restore_focus(
    &self,
    _target: &Self::NodeId,
    _request: FocusRuntimeRequest,
  ) -> FocusCommandResult {
    FocusCommandResult::Unsupported
  }
}

/// Result of resolving a runtime portal target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PortalMountResult<MountId> {
  Mounted(MountId),
  Inline,
  MissingTarget,
  Unsupported,
}

/// Request metadata for runtime portal mounting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PortalRuntimeRequest {
  pub target: PortalTarget,
  pub modal: bool,
}

impl PortalRuntimeRequest {
  pub fn new(target: PortalTarget, modal: bool) -> Self {
    Self { target, modal }
  }

  pub fn inline(modal: bool) -> Self {
    Self::new(PortalTarget::Inline, modal)
  }

  pub fn body(modal: bool) -> Self {
    Self::new(PortalTarget::Body, modal)
  }

  pub fn selector(selector: impl Into<String>, modal: bool) -> Self {
    Self::new(PortalTarget::Selector(selector.into()), modal)
  }

  pub fn should_mount(&self) -> bool {
    !matches!(self.target, PortalTarget::Inline)
  }

  pub fn is_body_target(&self) -> bool {
    matches!(self.target, PortalTarget::Body)
  }
}

/// Runtime portal command surface.
pub trait PortalRuntime {
  type MountId;

  fn mount_target(&self, request: &PortalRuntimeRequest) -> PortalMountResult<Self::MountId>;
}

/// Portal runtime used when the current target has no adapter installed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PortalRuntimeUnsupported;

impl PortalRuntime for PortalRuntimeUnsupported {
  type MountId = ();

  fn mount_target(&self, request: &PortalRuntimeRequest) -> PortalMountResult<Self::MountId> {
    if matches!(request.target, PortalTarget::Inline) {
      PortalMountResult::Inline
    } else {
      PortalMountResult::Unsupported
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dialog_request_maps_to_modal_focus_policy() {
    let request = FocusRuntimeRequest::dialog_default();

    assert_eq!(request.strategy, FocusStrategy::FirstFocusable);
    assert_eq!(request.return_policy, FocusReturn::Trigger);
    assert!(request.should_focus_initial());
    assert!(request.should_restore_focus());
    assert!(request.should_trap_focus());
  }

  #[test]
  fn popover_request_maps_to_non_modal_focus_policy() {
    let request = FocusRuntimeRequest::popover_default();

    assert_eq!(request.strategy, FocusStrategy::None);
    assert_eq!(request.return_policy, FocusReturn::None);
    assert!(!request.should_focus_initial());
    assert!(!request.should_restore_focus());
    assert!(!request.should_trap_focus());
  }

  #[test]
  fn unsupported_runtime_returns_explicit_fallback() {
    let runtime = FocusRuntimeUnsupported;
    let node = ();
    let request = FocusRuntimeRequest::dialog_default();

    assert_eq!(
      runtime.focus_initial(&node, request),
      FocusCommandResult::Unsupported
    );
    assert_eq!(
      runtime.trap_focus(&node, request),
      FocusCommandResult::Unsupported
    );
    assert_eq!(
      runtime.restore_focus(&node, request),
      FocusCommandResult::Unsupported
    );
  }

  #[test]
  fn inline_portal_request_does_not_require_mounting() {
    let request = PortalRuntimeRequest::inline(false);

    assert_eq!(request.target, PortalTarget::Inline);
    assert!(!request.modal);
    assert!(!request.should_mount());
    assert!(!request.is_body_target());
  }

  #[test]
  fn body_portal_request_requires_mounting() {
    let request = PortalRuntimeRequest::body(true);

    assert_eq!(request.target, PortalTarget::Body);
    assert!(request.modal);
    assert!(request.should_mount());
    assert!(request.is_body_target());
  }

  #[test]
  fn selector_portal_request_preserves_target() {
    let request = PortalRuntimeRequest::selector("#overlays", false);

    assert_eq!(request.target, PortalTarget::Selector("#overlays".to_string()));
    assert!(!request.modal);
    assert!(request.should_mount());
    assert!(!request.is_body_target());
  }

  #[test]
  fn unsupported_portal_runtime_keeps_inline_fallback() {
    let runtime = PortalRuntimeUnsupported;
    let request = PortalRuntimeRequest::inline(false);

    assert_eq!(runtime.mount_target(&request), PortalMountResult::Inline);
  }

  #[test]
  fn unsupported_portal_runtime_reports_unsupported_external_targets() {
    let runtime = PortalRuntimeUnsupported;

    assert_eq!(
      runtime.mount_target(&PortalRuntimeRequest::body(true)),
      PortalMountResult::Unsupported
    );
    assert_eq!(
      runtime.mount_target(&PortalRuntimeRequest::selector("#overlays", false)),
      PortalMountResult::Unsupported
    );
  }
}
