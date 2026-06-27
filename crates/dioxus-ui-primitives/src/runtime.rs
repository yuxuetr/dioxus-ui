use crate::{FocusReturn, FocusStrategy};

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
}
