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

  pub const fn from_policy(
    strategy: FocusStrategy,
    return_policy: FocusReturn,
    modal: bool,
  ) -> Self {
    Self::new(strategy, return_policy, modal)
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

  pub fn from_policy(target: PortalTarget, modal: bool) -> Self {
    Self::new(target, modal)
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

/// Runtime timer use case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TimerReason {
  ToastDismiss,
  SonnerDismiss,
  TooltipDelay,
  HoverCardDelay,
  CarouselAutoplay,
}

/// Request metadata for a one-shot runtime timer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimerRuntimeRequest {
  pub delay_ms: u64,
  pub reason: TimerReason,
}

impl TimerRuntimeRequest {
  pub const fn new(delay_ms: u64, reason: TimerReason) -> Self {
    Self { delay_ms, reason }
  }

  pub const fn toast_dismiss(delay_ms: u64) -> Self {
    Self::new(delay_ms, TimerReason::ToastDismiss)
  }

  pub const fn sonner_dismiss(delay_ms: u64) -> Self {
    Self::new(delay_ms, TimerReason::SonnerDismiss)
  }

  pub const fn tooltip_delay(delay_ms: u64) -> Self {
    Self::new(delay_ms, TimerReason::TooltipDelay)
  }

  pub const fn hover_card_delay(delay_ms: u64) -> Self {
    Self::new(delay_ms, TimerReason::HoverCardDelay)
  }

  pub const fn carousel_autoplay(delay_ms: u64) -> Self {
    Self::new(delay_ms, TimerReason::CarouselAutoplay)
  }

  pub const fn is_enabled(self) -> bool {
    self.delay_ms > 0
  }
}

/// Result of a runtime timer command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimerRuntimeResult<TimerId> {
  Scheduled(TimerId),
  Cancelled,
  Missing,
  Disabled,
  Unsupported,
}

/// Runtime timer command surface.
pub trait TimerRuntime {
  type TimerId;

  fn schedule_once(&self, request: &TimerRuntimeRequest) -> TimerRuntimeResult<Self::TimerId>;
  fn cancel(&self, id: &Self::TimerId) -> TimerRuntimeResult<Self::TimerId>;
}

/// Timer runtime used when the current target has no adapter installed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimerRuntimeUnsupported;

impl TimerRuntime for TimerRuntimeUnsupported {
  type TimerId = ();

  fn schedule_once(&self, request: &TimerRuntimeRequest) -> TimerRuntimeResult<Self::TimerId> {
    if request.is_enabled() {
      TimerRuntimeResult::Unsupported
    } else {
      TimerRuntimeResult::Disabled
    }
  }

  fn cancel(&self, _id: &Self::TimerId) -> TimerRuntimeResult<Self::TimerId> {
    TimerRuntimeResult::Unsupported
  }
}

/// Live-region announcement urgency.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnnouncementPriority {
  Polite,
  Assertive,
}

/// Duplicate handling policy for consecutive announcements.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DuplicateAnnouncementPolicy {
  Allow,
  SuppressConsecutive,
}

/// Request metadata for a runtime live-region announcement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveRegionRuntimeRequest {
  pub message: String,
  pub priority: AnnouncementPriority,
  pub duplicate_policy: DuplicateAnnouncementPolicy,
}

impl LiveRegionRuntimeRequest {
  pub fn new(
    message: impl Into<String>,
    priority: AnnouncementPriority,
    duplicate_policy: DuplicateAnnouncementPolicy,
  ) -> Self {
    Self {
      message: message.into(),
      priority,
      duplicate_policy,
    }
  }

  pub fn polite(message: impl Into<String>) -> Self {
    Self::new(
      message,
      AnnouncementPriority::Polite,
      DuplicateAnnouncementPolicy::SuppressConsecutive,
    )
  }

  pub fn assertive(message: impl Into<String>) -> Self {
    Self::new(
      message,
      AnnouncementPriority::Assertive,
      DuplicateAnnouncementPolicy::SuppressConsecutive,
    )
  }

  pub fn allow_duplicates(mut self) -> Self {
    self.duplicate_policy = DuplicateAnnouncementPolicy::Allow;
    self
  }

  pub fn suppress_consecutive_duplicates(mut self) -> Self {
    self.duplicate_policy = DuplicateAnnouncementPolicy::SuppressConsecutive;
    self
  }

  pub fn is_empty(&self) -> bool {
    self.message.trim().is_empty()
  }

  pub fn should_suppress_duplicate(&self, previous_message: Option<&str>) -> bool {
    matches!(
      self.duplicate_policy,
      DuplicateAnnouncementPolicy::SuppressConsecutive
    ) && previous_message == Some(self.message.as_str())
  }
}

/// Result of a runtime live-region announcement command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveRegionRuntimeResult {
  Queued,
  SuppressedDuplicate,
  EmptyMessage,
  Unsupported,
}

/// Runtime live-region command surface.
pub trait LiveRegionRuntime {
  fn announce(&self, request: &LiveRegionRuntimeRequest) -> LiveRegionRuntimeResult;
}

/// Live-region runtime used when the current target has no adapter installed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LiveRegionRuntimeUnsupported;

impl LiveRegionRuntime for LiveRegionRuntimeUnsupported {
  fn announce(&self, request: &LiveRegionRuntimeRequest) -> LiveRegionRuntimeResult {
    if request.is_empty() {
      LiveRegionRuntimeResult::EmptyMessage
    } else {
      LiveRegionRuntimeResult::Unsupported
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

  #[test]
  fn timer_request_preserves_reason_and_delay() {
    let request = TimerRuntimeRequest::toast_dismiss(5000);

    assert_eq!(request.delay_ms, 5000);
    assert_eq!(request.reason, TimerReason::ToastDismiss);
    assert!(request.is_enabled());
  }

  #[test]
  fn timer_request_can_be_disabled_by_zero_delay() {
    let request = TimerRuntimeRequest::sonner_dismiss(0);

    assert_eq!(request.reason, TimerReason::SonnerDismiss);
    assert!(!request.is_enabled());
  }

  #[test]
  fn timer_request_covers_deferred_runtime_reasons() {
    assert_eq!(
      TimerRuntimeRequest::tooltip_delay(150).reason,
      TimerReason::TooltipDelay
    );
    assert_eq!(
      TimerRuntimeRequest::hover_card_delay(200).reason,
      TimerReason::HoverCardDelay
    );
    assert_eq!(
      TimerRuntimeRequest::carousel_autoplay(3000).reason,
      TimerReason::CarouselAutoplay
    );
  }

  #[test]
  fn unsupported_timer_runtime_reports_unsupported_for_enabled_timer() {
    let runtime = TimerRuntimeUnsupported;
    let request = TimerRuntimeRequest::toast_dismiss(5000);

    assert_eq!(
      runtime.schedule_once(&request),
      TimerRuntimeResult::Unsupported
    );
  }

  #[test]
  fn unsupported_timer_runtime_reports_disabled_for_zero_delay() {
    let runtime = TimerRuntimeUnsupported;
    let request = TimerRuntimeRequest::toast_dismiss(0);

    assert_eq!(runtime.schedule_once(&request), TimerRuntimeResult::Disabled);
  }

  #[test]
  fn unsupported_timer_runtime_reports_unsupported_cancel() {
    let runtime = TimerRuntimeUnsupported;
    let timer_id = ();

    assert_eq!(runtime.cancel(&timer_id), TimerRuntimeResult::Unsupported);
  }

  #[test]
  fn polite_live_region_request_uses_duplicate_suppression() {
    let request = LiveRegionRuntimeRequest::polite("Saved");

    assert_eq!(request.message, "Saved");
    assert_eq!(request.priority, AnnouncementPriority::Polite);
    assert_eq!(
      request.duplicate_policy,
      DuplicateAnnouncementPolicy::SuppressConsecutive
    );
    assert!(!request.is_empty());
  }

  #[test]
  fn assertive_live_region_request_uses_assertive_priority() {
    let request = LiveRegionRuntimeRequest::assertive("Failed");

    assert_eq!(request.priority, AnnouncementPriority::Assertive);
    assert_eq!(
      request.duplicate_policy,
      DuplicateAnnouncementPolicy::SuppressConsecutive
    );
  }

  #[test]
  fn live_region_request_can_allow_duplicates() {
    let request = LiveRegionRuntimeRequest::polite("Saved").allow_duplicates();

    assert_eq!(request.duplicate_policy, DuplicateAnnouncementPolicy::Allow);
    assert!(!request.should_suppress_duplicate(Some("Saved")));
  }

  #[test]
  fn live_region_request_detects_empty_message() {
    let request = LiveRegionRuntimeRequest::polite("  ");

    assert!(request.is_empty());
  }

  #[test]
  fn live_region_request_suppresses_consecutive_duplicate() {
    let request = LiveRegionRuntimeRequest::polite("Saved");

    assert!(request.should_suppress_duplicate(Some("Saved")));
    assert!(!request.should_suppress_duplicate(Some("Loaded")));
    assert!(!request.should_suppress_duplicate(None));
  }

  #[test]
  fn unsupported_live_region_runtime_reports_unsupported_for_message() {
    let runtime = LiveRegionRuntimeUnsupported;
    let request = LiveRegionRuntimeRequest::polite("Saved");

    assert_eq!(runtime.announce(&request), LiveRegionRuntimeResult::Unsupported);
  }

  #[test]
  fn unsupported_live_region_runtime_reports_empty_message() {
    let runtime = LiveRegionRuntimeUnsupported;
    let request = LiveRegionRuntimeRequest::polite(" ");

    assert_eq!(runtime.announce(&request), LiveRegionRuntimeResult::EmptyMessage);
  }

  #[cfg(feature = "dialog")]
  #[test]
  fn dialog_config_maps_to_modal_runtime_requests() {
    let mut config = crate::DialogPrimitiveConfig::controlled(true);
    config.portal_target = PortalTarget::Body;

    let focus_request = FocusRuntimeRequest::from_policy(
      config.focus_strategy,
      config.focus_return,
      true,
    );
    let portal_request = PortalRuntimeRequest::from_policy(config.portal_target, true);

    assert!(focus_request.should_focus_initial());
    assert!(focus_request.should_restore_focus());
    assert!(focus_request.should_trap_focus());
    assert_eq!(portal_request.target, PortalTarget::Body);
    assert!(portal_request.modal);
    assert!(portal_request.should_mount());
  }

  #[cfg(feature = "popover")]
  #[test]
  fn popover_config_maps_to_non_modal_runtime_requests() {
    let config = crate::PopoverPrimitiveConfig::controlled(true);

    let focus_request = FocusRuntimeRequest::from_policy(
      config.focus_strategy,
      config.focus_return,
      false,
    );
    let portal_request = PortalRuntimeRequest::from_policy(config.portal_target, false);

    assert!(!focus_request.should_focus_initial());
    assert!(focus_request.should_restore_focus());
    assert!(!focus_request.should_trap_focus());
    assert_eq!(portal_request.target, PortalTarget::Inline);
    assert!(!portal_request.modal);
    assert!(!portal_request.should_mount());
  }
}
