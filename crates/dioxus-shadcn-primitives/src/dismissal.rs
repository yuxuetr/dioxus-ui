use crate::DismissBehavior;

/// User interaction that can request closing an overlay or composite widget.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DismissalEvent {
  EscapeKey,
  PointerOutside,
  FocusOutside,
  PointerInside,
  FocusInside,
}

impl DismissalEvent {
  pub const fn is_outside_interaction(self) -> bool {
    matches!(self, Self::PointerOutside | Self::FocusOutside)
  }
}

/// Result of evaluating one dismissal interaction against a behavior config.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DismissalDecision {
  pub event: DismissalEvent,
  pub should_dismiss: bool,
}

impl DismissalDecision {
  pub const fn ignored(event: DismissalEvent) -> Self {
    Self { event, should_dismiss: false }
  }

  pub const fn dismissed(event: DismissalEvent) -> Self {
    Self { event, should_dismiss: true }
  }
}

impl DismissBehavior {
  pub const fn should_dismiss(&self, event: DismissalEvent) -> bool {
    match event {
      DismissalEvent::EscapeKey => self.escape_key,
      DismissalEvent::PointerOutside => self.outside_pointer,
      DismissalEvent::FocusOutside => self.focus_outside,
      DismissalEvent::PointerInside | DismissalEvent::FocusInside => false,
    }
  }

  pub const fn dismissal_decision(&self, event: DismissalEvent) -> DismissalDecision {
    if self.should_dismiss(event) {
      DismissalDecision::dismissed(event)
    } else {
      DismissalDecision::ignored(event)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn dialog_default_dismisses_only_on_escape() {
    let behavior = DismissBehavior::dialog_default();

    assert!(behavior.should_dismiss(DismissalEvent::EscapeKey));
    assert!(!behavior.should_dismiss(DismissalEvent::PointerOutside));
    assert!(!behavior.should_dismiss(DismissalEvent::FocusOutside));
  }

  #[test]
  fn popover_default_dismisses_on_escape_and_outside_interactions() {
    let behavior = DismissBehavior::popover_default();

    assert!(behavior.should_dismiss(DismissalEvent::EscapeKey));
    assert!(behavior.should_dismiss(DismissalEvent::PointerOutside));
    assert!(behavior.should_dismiss(DismissalEvent::FocusOutside));
  }

  #[test]
  fn inside_interactions_never_dismiss() {
    let behavior = DismissBehavior::popover_default();

    assert!(!behavior.should_dismiss(DismissalEvent::PointerInside));
    assert!(!behavior.should_dismiss(DismissalEvent::FocusInside));
  }

  #[test]
  fn disabled_paths_return_ignored_decisions() {
    let behavior =
      DismissBehavior { escape_key: false, outside_pointer: false, focus_outside: false };

    assert_eq!(
      behavior.dismissal_decision(DismissalEvent::EscapeKey),
      DismissalDecision::ignored(DismissalEvent::EscapeKey)
    );
    assert_eq!(
      behavior.dismissal_decision(DismissalEvent::PointerOutside),
      DismissalDecision::ignored(DismissalEvent::PointerOutside)
    );
  }

  #[test]
  fn enabled_paths_return_dismissed_decisions() {
    let behavior = DismissBehavior::popover_default();

    assert_eq!(
      behavior.dismissal_decision(DismissalEvent::FocusOutside),
      DismissalDecision::dismissed(DismissalEvent::FocusOutside)
    );
  }

  #[test]
  fn outside_interaction_helper_identifies_outside_events() {
    assert!(DismissalEvent::PointerOutside.is_outside_interaction());
    assert!(DismissalEvent::FocusOutside.is_outside_interaction());
    assert!(!DismissalEvent::EscapeKey.is_outside_interaction());
    assert!(!DismissalEvent::PointerInside.is_outside_interaction());
  }
}
